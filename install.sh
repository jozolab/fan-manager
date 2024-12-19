#!/bin/sh

sudo systemctl disable --now fan-manager;
cargo build --release;
sudo cp target/release/fan-manager /usr/local/bin/fan-manager;
sudo cp fan-manager.service /etc/systemd/system/fan-manager.service;
sudo cp fan-manager.yaml /etc/fan-manager.yaml;
sudo systemctl daemon-reload;
sudo systemctl enable --now fan-manager;
