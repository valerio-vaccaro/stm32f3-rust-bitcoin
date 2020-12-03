#![feature(alloc_error_handler)]
#![no_main]
#![no_std]
use panic_halt as _;
#[macro_use]
extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use core::alloc::Layout;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

use bitcoin::secp256k1::{ffi, All, Context, Secp256k1};
use bitcoin::util::address::Address;
use bitcoin::util::key::PrivateKey;
use bitcoin::Network;

// this is the allocator the application will use
#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 1024; // in bytes

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }

    // Load a private key
    let raw = "L1HKVVLHXiUhecWnwFYF6L3shkf1E12HUmuZTESvBXUdx3yqVP1D";
    let pk = PrivateKey::from_wif(raw).unwrap();
    hprintln!("Seed WIF: {}", pk).unwrap();

    // Create secp256k1 context
    let buf = vec![0u8; Secp256k1::<All>::preallocate_size_gen()].into_boxed_slice();
    let ptr = Box::into_raw(buf);
    let ctx = unsafe {
        ffi::secp256k1_context_preallocated_create(ptr as *mut ffi::types::c_void, All::FLAGS)
    };
    let secp = Secp256k1::new();

    // Derive address
    let address = Address::p2wpkh(&pk.public_key(&secp), Network::Bitcoin).unwrap();
    hprintln!("Address: {}", address).unwrap();

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();

    loop {}
}
