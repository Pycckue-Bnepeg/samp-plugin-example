use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};

pub struct Plugin {
    version: &'static str,
}

// native: CallMe(&arg1, Float:arg2);
define_native!(callme, arg1: ref i32, arg2: f32);
// native: RawFunction(const str[]);
define_native!(raw_function as raw);
// native: NoArgs();
define_native!(no_args);

impl Plugin {
    pub fn load(&self) -> bool {
        log!("Plugin was successful loaded! Version {}", self.version);
        return true;
    }

    pub fn unload(&self) {
        
    }

    pub fn amx_load(&mut self, amx: AMX) -> u32 {
        let natives = natives! {
            "CallMe" => callme,
            "RawFunction" => raw_function,
            "NoArgs" => no_args
        };

        match amx.register(&natives) {
            Ok(_) => log!("Natives was successful loaded!"),
            Err(err) => log!("Error {:?}", err),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(&self, _amx: AMX) -> u32 {
        AMX_ERR_NONE
    }

    pub fn process_tick(&self, ) {

    }

    pub fn callme(&self, _amx: &mut AMX, arg1: &mut i32, arg2: f32) -> AmxResult<Cell> {
        *arg1 = 10;
        log!("float value is {}", arg2);
        Ok(*arg1)
    }

    pub fn raw_function(&self, _amx: &mut AMX, params: *mut Cell) -> AmxResult<Cell> {
        let count = args_count!(params);
        log!("raw_function. args count {}", count);
        Ok(0)
    }

    pub fn no_args(&self, _amx: &mut AMX) -> AmxResult<Cell> {
        Ok(0)
    }
}

impl Default for Plugin {
    fn default() -> Self {
        Plugin {
            version: "0.1",
        }
    }
}