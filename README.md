# Spotify advertisement muter for \*nix

For arch users AUR link [spotblock-rs](https://aur.archlinux.org/packages/spotblock-rs/)

## Information

- works with pipewire
- checks playerctl to check if an advertisment is playing and will mute its pulseaudio sink

## Installation

- install [rust](https://www.rust-lang.org/tools/install)
- Install [playerctl](https://github.com/altdesktop/playerctl)
- Ensure you have `pactl` installed (run `pactl` in a terminal to check)
- download the files from github
  `git clone https://github.com/egggod/spotblock-rs`
- enter the programs directory
  `cd spotblock-rs`
- Install
  `./install.sh`

## Running

- Now spotblock should be available from your application menu, this will run spotify and start spotblock, and then kill spotblock after you close it
- alternatively run `spotblock-run` from a terminal to run spotify with ad muting enabled
- and run `spotblock-rs` from a terminal to run just the adblocker
## Disclaimer

This set of code is purely for educational purposes. What you do with the code and any repurcussions is entirely your responsibility.
