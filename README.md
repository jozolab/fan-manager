# fan-manager

## Description

a simple fan manager to control the fan speed based on cpu temperature

## Usage

```bash
#edit fan-manager.yaml to fit your value
cargo run
```

### Install as a systemd service

```bash
sh install.sh
```

## Configuration

```yaml
fan:
  # name of the hwmon device
  device: oxpec

  # min fan speed value (e.g. 100)
  min: 80

  # max fan speed value (e.g. 255)
  max: 180

  # step on every increase/decrease (e.g. 5)
  step: 5

  # sleep interval in seconds
  interval: 1

cpu:
  # target min cpu temperature (celsius)
  min: 40

  # target max cpu temperature (celsius)
  max: 60
```

## License

MIT