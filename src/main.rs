mod args;
mod config;
mod fan;
mod cpu;

use clap::Parser;
use std::{fs, io, thread};
use std::error::Error;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use signal_hook::consts::{SIGTERM};
use signal_hook::iterator::Signals;
use console::Term;

use crate::args::Args;
use crate::config::Config;


fn trace(term: &Term, msg: String) {
    if cfg!(debug_assertions) {
        term.clear_last_lines(1).expect("cannot clear last lines");
        term.write_line(format!("{}", msg).as_str()).expect("cannot write line");
    }
}

fn write_file(path: &str, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())
}

fn average_temp() -> Result<f64, std::io::Error> {
    let cpu_temp_dir = Path::new("/sys/class/thermal");
    let mut sum = 0.0;
    let mut count = 0;

    for entry in fs::read_dir(cpu_temp_dir)? {
        let path = entry?.path();
        if path.is_dir() && path.file_name().unwrap().to_str().unwrap().starts_with("thermal_zone") {
            let temp_path = path.join("temp");
            if temp_path.exists() {
                let temp_contents = fs::read_to_string(temp_path)?;
                let temp: f64 = temp_contents.trim().parse().unwrap();
                sum += temp;
                count += 1;
            }
        }
    }

    if count == 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No temperature files found"));
    }

    let average_temp = sum / count as f64;
    Ok(average_temp / 1000.0)
}

fn find_device_path(config: &Config) -> Result<Option<PathBuf>, std::io::Error> {
    let hwmon_dir = Path::new("/sys/class/hwmon");

    for entry in fs::read_dir(hwmon_dir)? {
        let device_path = entry?.path();
        if device_path.is_dir() {
            let name_path = device_path.join("name");
            if name_path.exists() {
                let name_contents = fs::read_to_string(name_path)?;
                if name_contents.trim() == config.fan.device {
                    return Ok(Some(device_path));
                }
            }
        }
    }

    Ok(None)
}

fn main_loop(pwm_path: &str, config: &Config) {
    let term = Term::stdout();

    let mut fan_speed = config.fan.min + (config.fan.max - config.fan.min) / 2;

    let current_temp = average_temp().unwrap();
    if current_temp > config.cpu.max as f64 {
        fan_speed = config.fan.max;
    }

    println!("current temperature is {}, starting at fan speed {}", current_temp, fan_speed);
    write_file(pwm_path, fan_speed.to_string().as_str()).expect("Could not write pwm1.");

    thread::sleep(std::time::Duration::from_secs(config.fan.interval as u64));

    loop {
        let avg_temp = average_temp().unwrap();

        if fan_speed >= config.fan.max {
            trace(&term, format!("temperature is now {}, reached max fan speed, not updating.", avg_temp));
        } else if avg_temp > config.cpu.min as f64 && fan_speed < config.fan.max {
            trace(&term, format!("temperature is now {}, increasing fan speed {}", avg_temp, fan_speed));
            write_file(pwm_path, fan_speed.to_string().as_str()).expect("Could not write pwm1.");
            fan_speed += config.fan.step;
        } else if fan_speed > config.fan.min {
            trace(&term, format!("temperature is now {}, decreasing fan speed {}", avg_temp, fan_speed));
            write_file(pwm_path, fan_speed.to_string().as_str()).expect("Could not write pwm1.");
            fan_speed -= config.fan.step;
        } else {
            trace(&term, format!("temperature is now {}, keeping fan speed {}", avg_temp, fan_speed));
        }

        thread::sleep(std::time::Duration::from_secs(config.fan.interval as u64));
    }
}

fn main() -> Result<(), Box<dyn Error>>{
    sudo::escalate_if_needed()?;

    let args = Args::parse();
    let f = std::fs::File::open(args.config).expect("config file not found.");
    let config: Config = serde_yaml::from_reader(f).expect("Could not read values.");

    let device_path = find_device_path(&config).expect("device name not found.").unwrap().into_os_string().into_string().unwrap();

    let pwm_path = format!("{}/pwm1", device_path);
    let pwm_enable_path = format!("{}/pwm1_enable", device_path);
    // println!("found device {}", device_path);

    // Enable fan
    write_file(&pwm_enable_path, "1").expect("Could not write pwm1_enable.");
    println!("fan enabled");

    let mut signals = Signals::new([SIGTERM])?;
    thread::spawn(move || {
        for _sig in signals.forever() {
            println!("shutting down");

            // Disable fan
            write_file(&pwm_enable_path, "0").expect("Could not write pwm1_enable.");
            println!("fan disabled");

            exit(0);
        }
    });

    thread::sleep(std::time::Duration::from_secs(config.fan.interval as u64));

    main_loop(&pwm_path, &config);

    Ok(())
}