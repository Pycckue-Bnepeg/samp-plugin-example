use samp_sdk::consts::*;
use samp_sdk::types::Cell;
use samp_sdk::amx::{AmxResult, AMX};

use memcache::Client;

define_native!(connect, address: String);
define_native!(get, connection: usize, key: String, value: ref Cell);
define_native!(get_string, connection: usize, key: String, value: ref Cell, size: usize);
define_native!(set, connection: usize, key: String, value: Cell, expire: u32);
define_native!(set_string, connection: usize, key: String, value: String, expire: u32);
define_native!(delete, connection: usize, key: String);
define_native!(increment, connection: usize, key: String, value: Cell);

pub struct Memcached {
    clients: Vec<Client>,
}

impl Memcached {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {
        
    }

    pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
        let natives = natives!{
            "Memcached_Connect" => connect,
            "Memcached_Set" => set,
            "Memcached_SetString" => set_string,
            "Memcached_Get" => get,
            "Memcached_GetString" => get_string,
            "Memcached_Delete" => delete,
            "Memcached_Increment" => increment
        };

        match amx.register(&natives) {
            Ok(_) => log!("Natives are successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(&self, _: &mut AMX) -> Cell {
        AMX_ERR_NONE
    }

    pub fn connect(&mut self, _: &AMX, address: String) -> AmxResult<Cell> {
        match Client::new(address.as_str()) {
            Ok(client) => {
                self.clients.push(client);
                Ok(self.clients.len() as Cell - 1)
            },
            Err(_) => Ok(-1),
        }
    }

    pub fn get(&mut self, _: &AMX, con: usize, key: String, value: &mut Cell) -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].get::<Cell>(key.as_str()) {
                Ok(Some(data)) => {
                    *value = data;
                    Ok(1)
                },
                Ok(None) => Ok(0),
                Err(_) => Ok(-1)
            }
        } else {
            Ok(-2)
        }
    }

    pub fn get_string(&mut self, _: &AMX, con: usize, key: String, string: &mut Cell, size: usize)
            -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].get::<String>(key.as_str()) {
                Ok(Some(data)) => {
                    let encoded = samp_sdk::cp1251::encode(&data)?;
                    set_string!(encoded, string, size);
                    Ok(1)
                },
                Ok(None) => Ok(0),
                Err(_) => Ok(-1),
            }
        } else {
            Ok(-2)
        }
    }

    pub fn set(&mut self, _: &AMX, con: usize, key: String, value: Cell, expire: u32) 
            -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].set(key.as_str(), value, expire) {
                Ok(_) => Ok(1),
                Err(_) => Ok(-1)
            }
        } else {
            Ok(-2)
        }
    }

    pub fn set_string(&mut self, _: &AMX, con: usize, key: String, value: String, expire: u32)
            -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].set(key.as_str(), value.as_str(), expire) {
                Ok(_) => Ok(1),
                Err(_) => Ok(-1),
            }
        } else {
            Ok(-2)
        }
    }

    pub fn increment(&mut self, _: &AMX, con: usize, key: String, value: Cell)
            -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].increment(key.as_str(), value as u64) {
                Ok(_) => Ok(1),
                Err(_) => Ok(-1),
            }
        } else {
            Ok(-2)
        }
    }

    pub fn delete(&mut self, _: &AMX, con: usize, key: String) -> AmxResult<Cell> {
        if con < self.clients.len() {
            match self.clients[con].delete(key.as_str()) {
                Ok(true) => Ok(1),
                Ok(false) => Ok(0),
                Err(_) => Ok(-1),
            }
        } else {
            Ok(-2)
        }
    }
}

impl Default for Memcached {
    fn default() -> Self {
        Memcached {
            clients: Vec::new(),
        }
    }
}