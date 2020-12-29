#![no_std]
#![no_main]
#![feature(global_asm)]

global_asm!(include_str!("start.s"));

mod panic;
mod periferals;

/// This is the main which is called from the assembly code using core 0
/// All other cores are blocked in WPE. This is a single core solution!
#[no_mangle]
pub extern "C" fn rmain() {

// Place your code here


}