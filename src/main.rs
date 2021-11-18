use std::process::Command;
use std::{thread, time};

// function that runs a command
fn run_cmd(cmd: &str) -> String {
    String::from_utf8(
        Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
            .stdout,
    )
    .unwrap()
}

// function that checks the pactl output and
// returns all the pulseaudio sinks associated to spotify
fn get_sinks(pactl: &str) -> Vec<String> {
    let mut sinks: Vec<String> = Vec::new();
    let mut sink = "";
    for line in pactl.split('\n') {
        if line.contains("Sink Input #") {
            sink = &line[12..];
        } else if line.contains("application.name = \"spotify\"")
            || line.contains("application.name = \"Spotify\"")
            || line.contains("application.process.binary = \"spotify\"")
            || line.contains("application.process.binary = \"Spotify\"")
        {
            sinks.push(sink.to_string());
        }
    }
    sinks
}

// mutes a vector of sinks
fn mute(sinks: Vec<String>) {
    for sink in sinks {
        run_cmd(format!("pactl set-sink-input-mute {} 1", &sink).as_str());
    }
}

// unmutes a vector of sinks
fn unmute(sinks: Vec<String>) {
    for sink in sinks {
        run_cmd(format!("pactl set-sink-input-mute {} 0", &sink).as_str());
    }
}
fn main() {
    // set wait duration to 1 second
    // can reduce this to make it so that it is more accurate
    let wait = time::Duration::from_secs(1);

    // wait until spotify is open and unmute it
    // in case the blocker was closed while muted previously
    println!("Waiting for Spotify to start playing");
    loop {
        let response = run_cmd("pactl list sink-inputs");
        let sink = get_sinks(&response);

        // wait until spotify sink is detected
        if sink.is_empty() {
            thread::sleep(wait);
            continue;
        }

        // unmute
        unmute(sink);
        println!("Unmuted Spotify");
        break;
    }

    // set default state muted to false
    let mut muted = false;
    println!("Now Muting Advertisements");

    // start infinite loop
    loop {
        // query playerctl for the track-id
        let mut response =
            run_cmd("playerctl --player=spotify metadata --format '{{mpris:trackid}}'");

        // remove trailing newline
        response.pop();

        // get spotify sinks
        // if this is empty it means spotify isnt open
        // probably
        let pactl = run_cmd("pactl list sink-inputs");
        let sink = get_sinks(&pactl);

        if sink.is_empty() {
            thread::sleep(wait);
            continue;
        }
        // check conditions for an ad playing
        let ad = response.starts_with("spotify:ad:") || response.is_empty();

        if ad && !muted {
            // if not muted and is an ad then mute
            mute(sink.to_owned());
            println!("Muted Sinks {:?}", sink);
            muted = true;
        } else if muted && !ad {
            // if muted and is a song thats not an ad then unmute
            unmute(sink.to_owned());
            println!("Unmuted Sinks {:?}", sink);
            muted = false;
        }

        thread::sleep(wait);
    }
}
