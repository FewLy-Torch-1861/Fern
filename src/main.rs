use std::env;
use rand::prelude::IndexedRandom;

fn main() {
    let mut rng = rand::rng();

    // ignore these exit code
    let ignore_list = vec![
        130,
    ];

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
    let sweetie_list = vec![
        "love",
        "cutie",
        "girl",
        "nut",
    ];

    // Random sweetie from the list
    let mut sweetie: &str = "you";
    if let Some(x) = sweetie_list.choose(&mut rng) {
        sweetie = x;
    };

    // success messages
    let success_messages = vec![
        // sweet praise
        format!("i'm so pround of you, {}~", sweetie),
        format!("you did so well, {}!", sweetie),
        format!("that's it, my {}!", sweetie),
        format!("look at my clever girl go~"),

        // teasing love
        format!("my gorgeous {} always delivers", sweetie),
        format!("you’re so cute when you focus~"),
        format!("mm~ look at you showing off"),

        // rewardy
        format!("you deserve a big kiss for that"),
        format!("come here, i owe you a cuddle"),
        format!("*pet your head*"),
    ];

    // fail messages
    let fail_messages = vec![
        format!("don't worry, {}. you can do it next time!", sweetie),
        format!("it's okay, {}. keep trying!", sweetie),
        format!("i'm always here for you~"),
    ];

    // Choose message category from exit code
    let messages_list = if exit_code == 0 {
        &success_messages
    } else {
        &fail_messages
    };

    // random message
    if let Some(msg) = messages_list.choose(&mut rng) {
        println!("{}", msg);
    }
}
