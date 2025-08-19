use std::{env, fs};
use rand::prelude::IndexedRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ignore_list: Option<Vec<i32>>,
    sweetie_list: Option<Vec<String>>,
    success_messages: Option<Vec<String>>,
    fail_messages: Option<Vec<String>>,
}

fn load_config() -> Option<Config> {
    // Get home directory and build config path
    let home = std::env::var("HOME").ok()?;
    let config_path = format!("{}/.config/fern/config.toml", home);
    let contents = fs::read_to_string(config_path).ok()?;
    toml::from_str(&contents).ok()
}

fn main() {
    let mut rng = rand::rng();

    // Load config
    let config = load_config();

    // ignore these exit code
    let ignore_list = config
        .as_ref()
        .and_then(|c| c.ignore_list.clone())
        .unwrap_or_else(|| vec![130]);

    // Get exit code from argument
    let args: Vec<String> = env::args().collect();
    let exit_code = if args.len() > 1 {
        let parsed = args[1].parse::<i32>().unwrap_or(0);
        if ignore_list.contains(&parsed) {
            0
        } else {
            parsed
        }
    } else {
        0
    };

    // pronouns
    let sweetie_list = config
        .as_ref()
        .and_then(|c| c.sweetie_list.clone())
        .unwrap_or_else(|| vec!["cutie".into(), "sweetie".into()]);

    // Random sweetie from the list
    let mut sweetie: &str = "you";
    if let Some(x) = sweetie_list.choose(&mut rng) {
        sweetie = x;
    };

    // success messages
    let success_messages = config
        .as_ref()
        .and_then(|c| c.success_messages.clone())
        .unwrap_or_else(|| vec![
            "amazing work as always".to_string(),
            "you are doing amazing".to_string(),
            "that's a good {sweetie}".to_string(),
            "good job, {sweetie}".to_string(),
        ]);

    // fail messages
    let fail_messages = config
        .as_ref()
        .and_then(|c| c.fail_messages.clone())
        .unwrap_or_else(|| vec![
            "don't worry, {sweetie}. you can do it next time!".to_string(),
            "it's okay, {sweetie}. keep trying!".to_string(),
        ]);

    // Choose message category from exit code
    let messages_list = if exit_code == 0 {
        &success_messages
    } else {
        &fail_messages
    };

    // random message
    if let Some(msg) = messages_list.choose(&mut rng) {
        // Replace "{sweetie}" with the selected sweetie
        let output = msg.replace("{sweetie}", sweetie);
        println!("{}", output);
    }
}
