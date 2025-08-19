# Fern

Fern is a simple, customizable command-line compliment and encouragement tool for Linux.  
It prints a random positive message after a command finishes, using your own config file for personalization.

## Features

- Customizable messages and pronouns via config file
- Success and failure messages based on exit code
- Random selection for variety
- Easy to extend and configure

## Installation

1. **Clone the repository:**
   ```sh
   git clone https://github.com/FewLy-Torch-1861/Fern.git
   cd fern
   ```

2. **Build the project:**
   ```sh
   cargo build --release
   ```

3. **(Optional) Move the binary to your PATH:**
   ```sh
   cp target/release/fern ~/.local/bin/
   ```

## Configuration

Fern loads its config from `~/.config/fern/config.toml`.  
Here’s an example config:

```toml
ignore_list = [130]
sweetie_list = ["love", "lovely", "cutie", "girl", "sweetie"]
success_messages = [
    "amazing work as always",
    "you are doing amazing",
    "that's a good {sweetie}",
    "good job, {sweetie}",
    "you did so well, {sweetie}",
    "look at my clever girl go~",
    "you're shining today, {sweetie}",
    "my gorgeous {sweetie} always delivers",
    "you're so cute when you focus~",
    "mm~ look at you showing off~",
    "you make me proud every time, {sweetie}",
    "i'm very proud of you",
    "i'm very proud of you, my {sweetie}",
    "i'm so proud of you, {sweetie}~",
    "i knew you could do it",
    "i knew you could do it, {sweetie}~",
    "look at you go, {sweetie}!",
    "*pet your head~*",
    "*kiss your cheek~*",
    "you deserve a big kiss for that~",
    "i think you deserve a special treat for that",
    "come here, i owe you a cuddle~",
    "a little reward for my {sweetie}!",
]
fail_messages = [
    "don't worry, {sweetie}. you can do it next time!",
    "it's okay, {sweetie}. keep trying!",
    "i believe in you~",
    "i believe in you, my {sweetie}~",
    "i know you'll get there",
    "i know my little {sweetie} can do better",
    "you can do better, {sweetie}",
    "just know that i still love you~",
    "i'm always here for you~",
    "i'm always here for you if you need me",
    "come here, sit on my lap while we figure this out together",
    "don't worry, it'll be alright",
    "it's okay to make mistakes",
    "it's okay to make mistakes, my {sweetie}",
    "i know it's hard, but it will be okay",
    "even mistakes are part of learning, {sweetie}",
    "never give up, my love",
    "just a little further, i know you can do it",
    "i know you'll get there, don't worry about it",
    "keep pushing, you're stronger than you think, {sweetie}",
    "did my {sweetie} make a big mess",
    "we'll clean this up together, {sweetie}",
]
```

- `{sweetie}` will be replaced with a random entry from `sweetie_list`.

## Usage

Run Fern after a command and pass the exit code as an argument:

```sh
fern 0    # Success message
fern 1    # Failure message
```

You can use it in shell scripts or as part of your workflow.