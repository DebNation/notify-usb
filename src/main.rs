use clap::Parser;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

const CONNECT_MP3: &[u8] = include_bytes!("../assets/connect.mp3");
const DISCONNECT_MP3: &[u8] = include_bytes!("../assets/disconnect.mp3");

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Do not play audio when notifying
    #[arg(long)]
    no_sound: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut state = String::new();
    let home = env::var("HOME").expect("HOME not set");
    let dir = format!("{}/.local/share/notify-usb", home);
    let connect_path = format!("{}/connect.mp3", dir);
    let disconnect_path = format!("{}/disconnect.mp3", dir);
    if !Path::new(&dir).exists() {
        fs::create_dir_all(&dir).expect("failed to create directory");
        std::fs::write(&connect_path, CONNECT_MP3).expect("Failed to write connect.mp3");
        std::fs::write(&disconnect_path, DISCONNECT_MP3).expect("Failed to write connect.mp3");
    }

    let mut child = Command::new("udevadm")
        .args(["monitor", "--udev", "--property"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start udevadm");

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    let mut action: Option<String> = None;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("ACTION=") {
            let act = line.trim_start_matches("ACTION=").to_string();
            action = match act.as_str() {
                "add" => Some("connected".to_string()),

                "remove" => Some("disconnected".to_string()),
                _ => None,
            };
        } else if line.starts_with("ID_MODEL=") {
            let model = line.trim_start_matches("ID_MODEL=").to_string();
            if let Some(ref act) = action {
                let new_state = format!("{}{}", model, act);
                if state != new_state {
                    notify(std::format!("Device: {} {}", model, act).trim());
                    if !args.no_sound {
                        if act == "connected" {
                            play_audio(&connect_path)
                        } else {
                            play_audio(&disconnect_path)
                        }
                    }
                    state = new_state;
                }
                action = None;
            }
        }
    }

    Ok(())
}
fn play_audio(audio_path: &str) {
    if let Err(err) = Command::new("mpv")
        .arg("--ao=pulse")
        .arg(audio_path)
        .output()
    {
        eprintln!("Failed to play audio: {err}");
        eprintln!("💡 Tip: Make sure `mpv` is installed and in your PATH.");
    }
}

fn notify(text: &str) {
    Command::new("dunstify")
        .arg(text)
        .output()
        .expect("dunst is not found");
}
