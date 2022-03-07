use std::process::Command;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
};

pub fn get_outputs_wpctl() {
    let output = Command::new("wpctl")
        .arg("status")
        .output()
        .expect("Failed to execute wireplumber ctl");


    let output_text = String::from_utf8(output.stdout).unwrap();
    const EMPTY_LINE_LEN: usize = 7;

    let lines: Vec<&str> = output_text.split('\n').collect();
    let mut prompt_lines = vec![];

    let mut is_sinks = false;
    let mut is_audio = false;

    for x in lines {
        if x.contains("Sink endpoints") {
            is_audio = false;
            is_sinks = false;
        }

        if is_sinks && x.len() > EMPTY_LINE_LEN {
            prompt_lines.push(String::from(x));
        }

        if x.contains("Audio") {
            is_audio = true;
        }

        if x.contains("Sinks") && is_audio {
            is_sinks = true;
        }
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&prompt_lines)
        .default(0)
        .interact().unwrap();

    let selected_split: Vec<&str> = prompt_lines[selection].split('.').collect();
    let sink_id: String = selected_split[0].chars().filter(|a| a.is_digit(10)).collect();

    Command::new("wpctl").
        arg("set-default")
        .arg(sink_id.as_str())
        .output()
        .expect("Failed to set default sink with wpctl");
}