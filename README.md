# porcupine-sys

This is a Rust binding to [Porcupine](https://github.com/Picovoice/Porcupine).

## Setup

Before you can use this, the Porcupine libraries will need to be copied
to a place where the linker can find it (e.g. `/usr/local/lib` on Linux).

For example on Linux x86_64 you would do something like this:

```
git clone https://github.com/Picovoice/Porcupine
cd Porcupine
sudo cp lib/linux/x86_64 /usr/local/lib
```

## Example

To run the examples, you must first generate the wake word model by
going to the website at https://console.picovoice.ai/ppn. It will take
a couple hours for it to finish. Once finished, you can download it and
replace, for example, the `assets/hi robot_linux.ppn` which is used by
the `single` example. If you want to run the `multiple` example, you
must generate three word models and replace `assets/play music_linux.ppn`,
`assets/next music_linux.ppn`, `assets/stop music_linux.ppn`.

Then you can clone this project by running:

```
git clone --recursive https://github.com/veandco/porcupine-sys
```

After that, run the example via:

```
cargo run --example single
```

or for the multiple keywords version:

```
cargo run --example multiple
```
