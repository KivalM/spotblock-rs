# a little program to mute ads in spotify via pulseaudio written in rust

## Information

- works with pipewire
- checks playerctl to check if an advertisment is playing and will mute its pulseaudio sink

## Installation

It requires [playerctl](https://github.com/altdesktop/playerctl) and pactl command line tools.

## Running

download the files from github

run

```
cargo run --release
```

copy the executable from /target/ to a folder of your choice and run it

```
./spotblock-rs
```

## todo

- add systemctl service
- optimise (although it should be barely noticable as is)
