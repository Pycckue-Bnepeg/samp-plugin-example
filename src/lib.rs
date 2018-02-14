#![feature(trace_macros)]

#[macro_use] extern crate samp_sdk;
extern crate libc;

mod plugin;
use plugin::Plugin;

new_plugin!(Plugin);