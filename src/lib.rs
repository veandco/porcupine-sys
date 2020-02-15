//! # porcupine-sys
//!
//! A Rust binding for Porcupine

// libc
extern crate libc;
use libc::{c_char, c_int};

mod c;

// std
use std::ffi::{CStr, CString};
use std::fmt::{self, Display};
use std::ptr;

/// The status code returned by various porcupine functions.
#[derive(Debug)]
pub enum Status {
    Success = 0,
    OutOfMemory = 1,
    IOError = 2,
    InvalidArgument = 3,
    Unknown = 4,
}

/// Audio sample rate accepted by Picovoice.
pub unsafe fn sample_rate() -> usize {
    c::pv_sample_rate() as usize
}

/// Version of Porcupine.
pub unsafe fn version() -> &'static str {
    CStr::from_ptr(c::pv_porcupine_version()).to_str().unwrap()
}

/// Length (number of audio samples) per frame.
pub unsafe fn frame_length() -> usize {
    c::pv_porcupine_frame_length() as usize
}

pub struct Object {
    _object: *mut c::pv_porcupine_t,
}

unsafe impl Send for Object {}

impl Object {
    /// Creates a new Porcupine object.
    pub unsafe fn new(
        model_file_path: &str,
        keyword_file_path: &str,
        sensitivity: f32,
    ) -> Result<Self, Status> {
        Self::new_multiple_keywords(model_file_path, &[keyword_file_path], &[sensitivity])
    }

    /// Creates a new Porcupine object that is capable of detecting multiple keywords.
    pub unsafe fn new_multiple_keywords(
        model_file_path: &str,
        keyword_file_paths: &[&str],
        sensitivities: &[f32],
    ) -> Result<Self, Status> {
        let mut _object: *mut c::pv_porcupine_t = ptr::null_mut();
        let _model_file_path = CString::new(model_file_path).unwrap().into_raw();
        let _number_keywords = keyword_file_paths.len() as c_int;
        let _keyword_file_paths: Vec<CString> = keyword_file_paths
            .iter()
            .map(|p| CString::new(*p).unwrap())
            .collect();
        let _keyword_file_paths: Vec<_> = _keyword_file_paths.iter().map(|p| p.as_ptr()).collect();

        let status = c::pv_porcupine_init(
            _model_file_path,
            _number_keywords,
            _keyword_file_paths.as_ptr() as *const *const c_char,
            sensitivities.as_ptr(),
            &mut _object,
        );
        if status != 0 {
            return Err(status.into());
        }

        Ok(Object { _object })
    }

    /// Delete the Porcupine object.
    pub unsafe fn delete(&mut self) {
        c::pv_porcupine_delete(self._object);
    }

    /// Detect keyword within the provided audio data. The data must be 16-bit linearly-encoded and single-channel with sample rate equal to `sample_rate()`.
    pub unsafe fn process(&self, pcm: &[i16]) -> Result<bool, Status> {
        self.process_multiple_keywords(pcm).map(|x| x != -1)
    }

    /// Detect one of the keywords within the provided audio data. The data must be 16-bit linearly-encoded and single-channel with sample rate equal to `sample_rate()`. It returns the index of the detected keyword if successful.
    pub unsafe fn process_multiple_keywords(&self, pcm: &[i16]) -> Result<isize, Status> {
        let mut keyword_index: c_int = -1;

        let status = c::pv_porcupine_process(
            self._object,
            pcm.as_ptr(),
            &mut keyword_index,
        );
        if status != 0 {
            return Err(status.into());
        }

        Ok(keyword_index as isize)
    }
}

impl From<c::pv_status_t> for Status {
    fn from(status: c::pv_status_t) -> Self {
        match status {
            0 => Status::Success,
            1 => Status::OutOfMemory,
            2 => Status::IOError,
            3 => Status::InvalidArgument,
            _ => Status::Unknown,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::OutOfMemory => write!(f, "Out Of Memory"),
            Status::IOError => write!(f, "I/O Error"),
            Status::InvalidArgument => write!(f, "Invalid Argument"),
            _ => write!(f, "Unknown"),
        }
    }
}
