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
        // generic
        format!("amazing work as always"),
        format!("you are doing amazing"),

        // good sweetie
        format!("that's a good {}", sweetie),
        format!("who's my good {}", sweetie),
        format!("that's it, my {}", sweetie),
        format!("good job, {}", sweetie),
        format!("good {}", sweetie),

        // sweet praise
        format!("you did so well, {}", sweetie),
        format!("look at my clever girl go~"),
        format!("you're shining today, {}", sweetie),
        format!("my {} is unstoppable!", sweetie),

        // teasing love
        format!("my gorgeous {} always delivers", sweetie),
        format!("you’re so cute when you focus~"),
        format!("mm~ look at you showing off~"),
        format!("you make me proud every time, {}", sweetie),

        // proud
        format!("i'm very proud of you"),
        format!("i'm very proud of you, my {}", sweetie),
        format!("i'm so proud of you, {}~", sweetie),
        format!("i knew you could do it"),
        format!("i knew you could do it, {}~", sweetie),
        format!("look at you go, {}!", sweetie), 

        // reward
        format!("*pet your head~*"),
        format!("*kiss your cheek~*"),
        format!("you deserve a big kiss for that~"),
        format!("i think you deserve a special treat for that"),
        format!("come here, i owe you a cuddle~"),
        format!("a little reward for my {}!", sweetie),
        ];

    // fail messages
    let fail_messages = vec![
        // trust
        format!("don't worry, {}. you can do it next time!", sweetie),
        format!("it's okay, {}. keep trying!", sweetie),
        format!("i believe in you~"),
        format!("i believe in you, my {}~", sweetie),
        format!("i know you'll get there"),
        format!("i know my little {} can do better", sweetie),
        format!("you can do better, {}", sweetie),

        // fallback
        format!("just know that i still love you~"),
        format!("i'm always here for you~"),
        format!("i'm always here for you if you need me"),
        format!("come here, sit on my lap while we figure this out together"),

        // consolation
        format!("don't worry, it'll be alright"),
        format!("it's okay to make mistakes"),
        format!("it's okay to make mistakes, my {}", sweetie),
        format!("i know it's hard, but it will be okay"),
        format!("even mistakes are part of learning, {}", sweetie),

        // encouragement
        format!("never give up, my love"),
        format!("just a little further, i know you can do it"),
        format!("i know you'll get there, don't worry about it"),
        format!("keep pushing, you're stronger than you think, {}", sweetie),

        // clean up
        format!("did my {} make a big mess", sweetie),
        format!("we'll clean this up together, {}", sweetie),
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
