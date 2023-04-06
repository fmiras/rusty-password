use std::process::{Command, Stdio};

fn get_wifi_password(ssid: &str) -> Option<String> {
  let output = Command::new("security")
      .args(&[
          "find-generic-password",
          "-D",
          "AirPort network password",
          "-ga",
          ssid,
      ])
      .output()
      .expect("Failed to get wifi password");

  let stdout = String::from_utf8_lossy(&output.stderr);
  let password_line = stdout.lines().find(|line| line.contains("password:"))?;
  let password = password_line.trim().split('"').nth(1)?;
  Some(password.to_string())
}

fn main() {
    let airport = "/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport";

    let ssid = Command::new(airport)
        .arg("-I")
        .output()
        .expect("Failed to execute command")
        .stdout;
    let ssid = String::from_utf8_lossy(&ssid);

    let ssid = ssid
        .lines()
        .find(|line| line.contains("SSID: "))
        .unwrap()
        .trim_start_matches("           SSID: ")
        .to_string();

    println!("📡 Found Wi-Fi SSID: {}", ssid);

    let password = get_wifi_password(&ssid).unwrap();

    println!("🔑 Password found: {}", password);
}
