//@ needs-asm-support
//@ run-pass

use std::arch::global_asm;

unsafe global_asm!("");

global_asm!("");

fn main() {}
