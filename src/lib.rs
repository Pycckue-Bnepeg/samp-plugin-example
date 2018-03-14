#[macro_use] extern crate samp_sdk;
extern crate memcache;

mod plugin;
use plugin::Memcached;

new_plugin!(Memcached);