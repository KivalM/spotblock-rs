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
fn get_sink(pactl: String) -> String {
    let mut sink = "";
    for line in pactl.split("\n") {
        // println!("1{}", line);
        if line.contains("Sink Input #") {
            sink = &line[12..];
        } else if line.contains("application.name = \"spotify\"")
            || line.contains("application.name = \"Spotify\"")
        {
            println!("found");
            break;
        }
    }
    return sink.to_string();
}
fn mute(sink: String) {
    run_cmd(format!("pactl set-sink-input-mute {} 1", &sink).as_str());
}
fn unmute(sink: String) {
    run_cmd(format!("pactl set-sink-input-mute {} 0", &sink).as_str());
}
fn main() {
    let mut muted = false;
    loop {
        let mut response =
            run_cmd("playerctl --player=spotify metadata --format '{{ title }}-{{ artist }}'");
        response.pop();
        if response == "Advertisement-" || response == "advertisement-" {
            if muted == false {
                // if not muted and is an ad
                // mute
                response = run_cmd("pactl list sink-inputs");
                let sink = get_sink(response);
                mute(sink.to_owned());
                println!("Muted Sink {}", sink);
                muted = true;
            }
        } else if muted == true {
            // if muted and is a song thats not an ad
            // unmute
            response = run_cmd("pactl list sink-inputs");
            let sink = get_sink(response);
            unmute(sink.to_owned());
            println!("Unmuted Sink {}", sink);
            muted = false;
        }

        // wait 1 second
        // can reduce this to make it so that it is more accurate
        let wait = time::Duration::from_secs(1);
        thread::sleep(wait);
    }
}
