#![no_main]
//#![no_std]
//#[macro_use]
//extern crate alloc;

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
use risc0_zkvm_guest::env;
use::risc0_zkvm_platform::{memory};

pub const DRAM_SIZE: u64 = 26214401;
use std::alloc::System;
//use alloc::vec::Vec;

#[global_allocator]
static GLOBAL: System = System;

#[derive(Debug)]
pub struct Dram {
    pub dram: Vec<u8>,
    code_size: u64,
}

impl Dram {
    /// Create a new memory object with default memory size.
    pub fn new() -> Self {
        Self {
            dram: vec![1; DRAM_SIZE.try_into().unwrap()],
            code_size: 0,
        }
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    let _ = Dram::new();
    let start = memory::HEAP.start();
    let end = memory::HEAP.end();
    println!("{:?} and {:?}", start, end);
}
#[cfg(test)]
risc0_zkvm_guest::entry!(test_main);

#[cfg(not(test))]
risc0_zkvm_guest::entry!(main);

pub fn main() {
    // Load the first number from the host
    let a: u64 = env::read();
    // Load the second number from the host
    let b: u64 = env::read();
    // Verify that neither of them are 1 (i.e. nontrivial factors)
    if a == 1 || b == 1 {
        panic!("Trivial factors")
    }
    let _ = Dram::new();
    let start = memory::HEAP.start();
    let end = memory::HEAP.end();
    // Compute the product while being careful with integer overflow
    let product = a.checked_mul(b).expect("Integer overflow");
    env::commit(&product);
}
