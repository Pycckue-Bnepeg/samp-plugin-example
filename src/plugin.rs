use samp_sdk::consts::*;
use samp_sdk::types;
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
        let natives = natives![
            { "callme", callme }
        ];

        match amx.register(&natives) {
            Ok(_) => log("Natives was successful loaded!"),
            Err(err) => log(&format!("Error {:?}", err)),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(_amx: AMX) -> u32 {
        AMX_ERR_NONE
    }

    pub fn supports() -> u32 {
        SUPPORTS_VERSION | SUPPORTS_AMX_NATIVES 
    }

    // callme(&val)
    pub fn callme(amx: AMX, params: *mut types::Cell) -> types::Cell {
        0
    }
}

extern "C" fn callme(amx: *mut types::AMX, params: *mut types::Cell) -> types::Cell {
    Plugin::callme(AMX::new(amx), params)
}

pub fn log(text: &str) {
    let printf = data::logprintf.lock().unwrap();
    let c_text = CString::new(text).unwrap();
    printf(c_text.as_ptr());
}