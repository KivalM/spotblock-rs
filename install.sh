#!/bin/bash

cargo build --release
sudo cp ./target/release/spotblock-rs /usr/bin/spotblock-rs
sudo cp run.sh /usr/bin/spotblock-run
sudo cp Spotblock.desktop /usr/share/applications/