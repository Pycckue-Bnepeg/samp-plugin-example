use samp_sdk::consts::*;
use samp_sdk::data;
use samp_sdk::amx::AMX;
use std::ffi::CString;

pub struct Plugin;

impl Plugin {
    pub fn load() -> bool {
        log("Plugin was successful loaded! Version 0.1");
        return true;
    }

    pub fn unload() {

    }

    pub fn amx_load(amx: AMX) -> u32 {
        let (result, index) = amx.find_public("OnPlayerConnect");
        log(&format!("amx.find_public ({}, {})", result, index));
        AMX_ERR_NONE
    }

    pub fn amx_unload(_amx: AMX) -> u32 {
        AMX_ERR_NONE
    }

    pub fn supports() -> u32 {
        SUPPORTS_VERSION | SUPPORTS_AMX_NATIVES 
    }
}

pub fn log(text: &str) {
    let printf = data::logprintf.lock().unwrap();
    let c_text = CString::new(text).unwrap();
    printf(c_text.as_ptr());
}