extern crate samp_sdk;
extern crate libc;

mod plugin;
use plugin::Plugin;

use samp_sdk::*;

#[no_mangle]
pub extern "C" fn Supports() -> u32 {
    Plugin::supports()
}

#[no_mangle]
pub unsafe extern "C" fn Load(data: *const libc::c_void) -> bool {
    let mut log = data::logprintf.lock().unwrap();

    *log = *(data as *const types::Logprintf_t);
    data::amx_functions = std::ptr::read((data as u32 + samp_sdk::consts::PLUGIN_DATA_AMX_EXPORTS) as *const u32);

    drop(log);
    Plugin::load()
}

#[no_mangle]
pub extern "C" fn Unload() {
    Plugin::unload();
}

#[no_mangle]
pub extern "C" fn AmxLoad(amx: *mut types::AMX) -> u32 {
    let amx = amx::AMX::new(amx);
    Plugin::amx_load(amx)
}

#[no_mangle]
pub extern "C" fn AmxUnload(amx: *mut types::AMX) -> u32 {
    let amx = amx::AMX::new(amx);
    Plugin::amx_unload(amx)
}