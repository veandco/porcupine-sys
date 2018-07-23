# porcupine-sys

This is a Rust binding to [Porcupine](https://github.com/Picovoice/Porcupine).

## Setup

Before you can use this, the Porcupine libraries will need to be copied to a place where the linker can find it (e.g. `/usr/local/lib` on Linux).

For example on Linux x86_64 you would do something like this:

```
git clone https://github.com/Picovoice/Porcupine
cd Porcupine
sudo cp lib/linux/x86_64 /usr/local/lib
```

## Example

To run the examples, you can first clone this project:

```
git clone --recursive https://github.com/veandco/porcupine-sys
```

Then run the example via:

```
cargo run --example single
```

or for the multiple keywords version:

```
cargo run --example multiple
```