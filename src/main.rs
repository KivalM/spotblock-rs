use std::process::Command;
use std::{thread, time};
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
fn get_sinks(pactl: String) -> Vec<String> {
    let mut sinks: Vec<String> = Vec::new();
    let mut sink = "";
    for line in pactl.split("\n") {
        // println!("1{}", line);
        if line.contains("Sink Input #") {
            sink = &line[12..];
        } else if line.contains("application.name = \"spotify\"")
            || line.contains("application.name = \"Spotify\"")
            || line.contains("application.process.binary = \"spotify\"")
            || line.contains("application.process.binary = \"Spotify\"")
        {
            // println!("found");
            sinks.push(sink.to_string());
        }
    }
    return sinks;
}
fn mute(sinks: Vec<String>) {
    for sink in sinks {
        run_cmd(format!("pactl set-sink-input-mute {} 1", &sink).as_str());
    }
}
fn unmute(sinks: Vec<String>) {
    for sink in sinks {
        run_cmd(format!("pactl set-sink-input-mute {} 0", &sink).as_str());
    }
}
fn main() {
    let mut muted = false;
    println!("Now Muting ADs");
    loop {
        let mut response =
            run_cmd("playerctl --player=spotify metadata --format '{{ title }}-{{ artist }}'");
        response.pop();
        if response == "Advertisement-"
            || response == "advertisement-"
            || response == "Spotify-"
            || response == "spotify-"
        {
            if !muted {
                // if not muted and is an ad
                // mute
                response = run_cmd("pactl list sink-inputs");
                let sink = get_sinks(response);
                if sink.is_empty() {
                    continue;
                }
                mute(sink.to_owned());
                println!("Muted Sinks {:?}", sink);
                muted = true;
            }
        } else if muted {
            // if muted and is a song thats not an ad
            // unmute
            response = run_cmd("pactl list sink-inputs");
            let sink = get_sinks(response);
            if sink.is_empty() {
                continue;
            }
            unmute(sink.to_owned());
            println!("Unmuted Sinks {:?}", sink);
            muted = false;
        }

        // wait 1 second
        // can reduce this to make it so that it is more accurate
        let wait = time::Duration::from_secs(1);
        thread::sleep(wait);
    }
}
