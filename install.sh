#!/bin/bash

cargo build --release
sudo cp ./target/release/spotblock-rs /usr/local/bin/
sudo cp run.sh /usr/local/bin/spotblock-run
sudo cp Spotblock.desktop /usr/share/applications/