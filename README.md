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
ignore_list = [
  130,
]
sweetie_list = [
  "love",
  "lovely",
  "cutie",
  "girl",
  "sweetie",
]
success_messages = [
    "amazing work as always",
    "you are doing amazing",
    "that's a good {sweetie}",
    "good job, {sweetie}",
    "you did so well, {sweetie}",
]
fail_messages = [
    "don't worry, {sweetie}. you can do it next time!",
    "it's okay, {sweetie}. keep trying!",
    "i believe in you~",
    "i believe in you, my {sweetie}~",
    "i know you'll get there",
]
```

- `{sweetie}` will be replaced with a random entry from `sweetie_list`.

## Usage

Run Fern after a command and pass the exit code as an argument:

```sh
fern 0    # Success message
fern 1    # Failure message
```

## Fact

Fern is heavyly inspired from [MOMMY](https://github.com/fwdekker/mommy)