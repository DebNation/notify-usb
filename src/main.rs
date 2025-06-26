use difference::Difference;
use rodio::Source;
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, ErrorKind};
use std::process::Command;
use std::{thread, time};

const CONNECT_MP3: &[u8] = include_bytes!("../assets/connect.mp3");

fn main() {
    println!("Running notify-usb");
    let home = env::var("HOME").expect("HOME not set");
    let dir = format!("{}/.local/share/notify-usb", home);
    let _ = fs::create_dir_all(&dir).expect("failed to create directory");
    let audio_path = format!("{}/connect.mp3", dir);
    std::fs::write(&audio_path, CONNECT_MP3).expect("Failed to write connect.mp3");
    let file_path = format!("{}/usbstate.txt", &dir);

    loop {
        let out = Command::new("ls")
            .arg("/dev")
            .output()
            .expect("ls command failed to start");
        let str_out = str::from_utf8(&out.stdout).unwrap();
        let contents = match fs::read_to_string(&file_path) {
            Ok(contents) => contents,

            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    fs::write(&file_path, str_out).unwrap();
                    str_out.to_string()
                } else {
                    eprintln!("Failed to read file: {}", e);
                    std::process::exit(1);
                }
            }
        };
        if str_out != contents {
            fs::write(&file_path, str_out).unwrap();
            play_audio(&audio_path);
            let changeset = difference::Changeset::new(contents.trim(), str_out, "\n");

            for diff in changeset.diffs {
                match diff {
                    Difference::Rem(removed) => {
                        if removed == "bsg" {
                            let _ = Command::new("dunstify")
                                .arg("Notify-USB")
                                .arg("Block Device Removed'")
                                .output()
                                .expect("dunst command failed to start");
                        }
                        // println!("{:?}", removed);
                    }
                    Difference::Add(added) => {
                        if added == "bsg" {
                            let _ = Command::new("dunstify")
                                .arg("Notify-USB")
                                .arg("Block Device Connected")
                                .output()
                                .expect("dunst command failed to start");
                        }
                        // println!("{:?}", added);
                    }
                    _ => (),
                }
            }
        }
        let one_second = time::Duration::from_millis(1000);
        thread::sleep(one_second);
    }
}

fn play_audio(audio_path: &str) {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let audio = BufReader::new(File::open(audio_path).unwrap());
    let source = rodio::Decoder::new(audio).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}

fn find_the_diff(new_string: String, old_string: String) {
    let mut diff: Vec<char> = Vec::new();

    for (new_char, old_char) in new_string.chars().zip(old_string.chars()) {
        if new_char != old_char {
            diff.push(new_char);
        }
    }
    println!("{:?}", diff);
}
