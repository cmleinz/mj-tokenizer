# MidJourney Tokenizer
This project is aimed at creating a fast and simple way to transform images created with the MidJourney AI, tokens for use with your favorite VTT.

## Installation
To install, first ensure you have the rust compiler installed. If you do not already have it installed, it is available for download [here](https://www.rust-lang.org/).

In the future I plan to release binary versions of the tool, but for now, while it is still being developed, this is the best way to access it.

With Rust installed, you'll need to clone this repository, then compile the program.

```bash
git clone https://github.com/cmleinz/mj-tokenizer
cd mj-tokenizer
cargo build --release
```

## Usage
Now that you have the tokenizer compiled you can check the available options with `target/release/tokenizer --help`. This will display the following:

```
tokenizer 0.1.0

USAGE:
    midjourney-tokenizer [OPTIONS] <FILE>

ARGS:
    <FILE>    Location of midjourney tile image

OPTIONS:
    -f, --frame <FRAME>    The frame that the final token will use (see assets folder) [default: 1]
    -h, --help             Print help information
    -s, --size <SIZE>      The size of the final token, both width and height (pixels) [default:
                           256]
    -t, --tile <TILE>      The tile number you would like to tokenize
    -V, --version          Print version information
```

## Examples
Take the below example which was output by MidJourney.

![scifi_example](https://github.com/cmleinz/mj-tokenizer/blob/main/examples/scifi.png?raw=true)

Very cool! Now lets quickly create a token of #4 with the below code:

```bash
target/release/tokenizer examples/scifi.png -t 4
```

Here `-t 4` indicates that we would like a token made of the 4th tile. And now we have a token!

![scifi_token](https://github.com/cmleinz/mj-tokenizer/blob/main/examples/scifi_token.png?raw=true)

We could also optionally set the token size with `-s SIZE_IN_PIXELS`. This value defaults to 256 which is the default resolution that MidJourney outputs for each tile. Bare in mind that increasing the size beyond 256 will result in some distortion of the image, since we are scaling beyond the native resolution. Scaling down however, will result in a high-quality down-scaled image. For example:

```bash
target/release/tokenizer examples/cat.png -t 1 -s 128
```

![cat_token](https://github.com/cmleinz/mj-tokenizer/blob/main/examples/cat_token.png?raw=true)

Finally, if there is an up-scaled image that you would like to create a token of, simply omit the tile specification in the command, like so:

```bash
target/release/tokenizer examples/ratfolk.png 
```

![ratfolk_token](https://github.com/cmleinz/mj-tokenizer/blob/main/examples/ratfolk_token.png?raw=true)

## Upcoming
I plan to add more features, and certainly more frames from which to choose from (currently there is only one available).
