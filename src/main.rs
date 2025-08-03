use std::process::{Command, Stdio};

fn get_clip() -> String {
    let xclip = Command::new("xsel")
        .arg("-b")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .expect("Error getting clipboard");

    let output_string = String::from_utf8_lossy(&xclip.stdout);
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
