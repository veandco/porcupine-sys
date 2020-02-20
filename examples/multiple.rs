extern crate porcupine_sys;

// Std
use std::cmp::min;
use std::fs::File;
use std::io::Read;

use porcupine_sys as pv;

#[cfg(target_os = "linux")]
const KEYWORD_FILES: [&str; 3] = [
    "assets/play music_linux.ppn",
    "assets/next music_linux.ppn",
    "assets/stop music_linux.ppn",
];
#[cfg(target_os = "macos")]
const KEYWORD_FILES: [&str; 3] = [
    "assets/play music_mac.ppn",
    "assets/next music_mac.ppn",
    "assets/stop music_mac.ppn",
];
#[cfg(target_os = "windows")]
const KEYWORD_FILES: [&str; 3] = [
    "assets/play music_windows.ppn",
    "assets/next music_windows.ppn",
    "assets/stop music_windows.ppn",
];

fn read_audio_file() -> Vec<u8> {
    let mut file = File::open("assets/multiple.raw").unwrap();
    let mut audio_u8 = Vec::new();

    // Read file to memory
    file.read_to_end(&mut audio_u8).unwrap();

    audio_u8
}

fn vec_u8_to_vec_i16(input: &[u8]) -> Vec<i16> {
    let mut output = Vec::new();
    let mut i = 0;

    // Padding at the start
    for _ in 0..2000 {
        output.push(0);
    }

    while i < input.len() {
        let h = (input[i + 1] as u16) << 8;
        let l = input[i] as u16;
        let val = (h | l) as i16;
        output.push(val);
        i += 2;
    }

    // Padding at the end
    for _ in 0..2000 {
        output.push(0);
    }

    output
}

fn main() {
    let audio_u8 = read_audio_file();
    let audio_i16 = vec_u8_to_vec_i16(&audio_u8);
    let frame_length = unsafe { pv::frame_length() };
    let mut object = unsafe {
        pv::Object::new_multiple_keywords(
            "assets/porcupine_params.pv",
            &KEYWORD_FILES,
            &[0.5, 0.5, 0.5],
        ).unwrap()
    };
    let mut index = 0;

    // Detect keyword
    while index < audio_i16.len() {
        let start = index;
        let end = min(audio_i16.len(), index + frame_length);
        let audio = &audio_i16[start..end];
        let detected = unsafe { object.process_multiple_keywords(audio).unwrap() };

        if detected >= 0 {
            println!("Detected keyword at {}!", detected);
        }

        index += frame_length;
    }

    unsafe {
        object.delete();
    }
}
