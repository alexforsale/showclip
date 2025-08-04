use std::env;
use std::process::{Command, Stdio};

fn get_clip() -> String {
    let xdg_session_type = match env::var("XDG_SESSION_TYPE") {
        Ok(value) => value,
        Err(_) => "".to_string(),
    }
    .to_lowercase();

    let clipboard_command = match xdg_session_type.as_str() {
        "x11" => Command::new("xsel")
            .arg("-b")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .expect("Error getting clipboard"),
        "wayland" => Command::new("wl-paste")
            .arg("-n")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .expect("Error getting clipboard"),
        &_ => panic!("Clipboard not found"),
    };

    let output_string = String::from_utf8_lossy(&clipboard_command.stdout);
    return output_string.trim().to_string();
}

fn notify(clip: String) {
    let notify_text = "󰅇 Clipboard";
    Command::new("notify-send")
        .arg("-t")
        .arg("3000")
        .arg("-i")
        .arg("bookmark")
        .arg(notify_text)
        .arg(clip)
        .arg("-a")
        .arg("showclip")
        .spawn()
        .unwrap();
}

fn main() {
    notify(get_clip().to_string())
}
