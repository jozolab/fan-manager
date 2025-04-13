#!/bin/sh

sudo systemctl disable --now fan-manager;
cargo build --release;
sudo rm /usr/local/bin/fan-manager;
sudo cp target/release/fan-manager /usr/local/bin/fan-manager;
sudo cp fan-manager.service /etc/systemd/system/fan-manager.service;

# Only copy yaml if it doesn't exist
if [ ! -f "/etc/fan-manager.yaml" ]; then
    sudo cp fan-manager.yaml /etc/fan-manager.yaml;
fi

sudo systemctl daemon-reload;
sudo systemctl enable --now fan-manager;
