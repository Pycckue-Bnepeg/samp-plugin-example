use samp_sdk::consts::*;
use samp_sdk::types;
use samp_sdk::amx::AMX;

pub struct Plugin;

define_native!(Plugin, callme, arg1: ref i32, arg2: f32);
define_native!(Plugin, raw_function as raw);
define_native!(Plugin, no_args);

impl Plugin {
    pub fn load() -> bool {
        log!("Plugin was successful loaded! Version 0.1");
        return true;
    }

    pub fn unload() {

    }

    pub fn amx_load(amx: AMX) -> u32 {
        let natives = natives![
            { "CallMe", callme },
            { "RawFunction", raw_function },
            { "NoArgs", no_args }
        ];

        match amx.register(&natives) {
            Ok(_) => log!("Natives was successful loaded!"),
            Err(err) => log!("Error {:?}", err),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(_amx: AMX) -> u32 {
        AMX_ERR_NONE
    }

    pub fn supports() -> u32 {
        SUPPORTS_VERSION | SUPPORTS_AMX_NATIVES 
    }

    pub fn callme(_amx: AMX, arg1: &mut i32, arg2: f32) -> types::Cell {
        *arg1 = 10;
        log!("float value is {}", arg2);
        *arg1
    }

    pub fn raw_function(_amx: AMX, _params: *mut types::Cell) -> types::Cell {
        0
    }

    pub fn no_args(_amx: AMX) -> types::Cell {
        0
    }
}