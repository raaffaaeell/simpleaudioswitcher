use std::process::Command;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
};


pub fn set_default_sink_pactl() {
    let output_pactl = Command::new("pactl")
        .arg("list")
        .arg("short")
        .arg("sinks")
        .output()
        .expect("Invalid pactl command");

    let text = String::from_utf8(output_pactl.stdout).unwrap();

    let items: Vec<&str> = text.split('\n').collect();

    let selected = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact().unwrap();

    let selected_split: Vec<&str> = items[selected].split_whitespace().collect();
    let sink_id = selected_split[0];

    Command::new("pactl")
        .arg("set-default-sink")
        .arg(sink_id)
        .output()
        .expect("Failed to set default sink with pactl");
}