extern crate porcupine_sys;

// Std
use std::cmp::min;
use std::fs::File;
use std::io::Read;

use porcupine_sys as pv;

fn read_audio_file() -> Vec<u8> {
    let mut file = File::open("assets/single.raw").unwrap();
    let mut audio_u8 = Vec::new();

    // Read file to memory
    file.read_to_end(&mut audio_u8).unwrap();

    audio_u8
}

fn vec_u8_to_vec_i16(input: &[u8]) -> Vec<i16> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let h = (input[i + 1] as u16) << 8;
        let l = input[i] as u16;
        let val = (h | l) as i16;
        output.push(val);
        i += 2;
    }

    output
}

fn main() {
    let audio_u8 = read_audio_file();
    let audio_i16 = vec_u8_to_vec_i16(&audio_u8);
    let frame_length = unsafe { pv::frame_length() };
    let mut object = unsafe {
        pv::Object::new(
            "assets/porcupine_params.pv",
            "assets/hi robot_linux.ppn",
            0.5,
        ).unwrap()
    };
    let mut index = 0;

    // Detect keyword
    while index < audio_i16.len() {
        let start = index;
        let end = min(audio_i16.len(), index + frame_length);
        let audio = &audio_i16[start..end];
        let detected = unsafe { object.process(audio).unwrap() };

        if detected {
            println!("Detected keyword!");
            break;
        }

        index += frame_length;
    }

    unsafe {
        object.delete();
    }
}
