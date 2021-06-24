#![no_std]
#![cfg_attr(test, no_main)]

use hello_f446 as _; // memory layout + panic handler

#[defmt_test::tests]
mod tests {}
