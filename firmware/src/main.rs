#![no_std]
#![no_main]

use contract::SelfId;
use embassy_executor::Spawner;
use embedded_alloc::LlffHeap as Heap;
use panic_halt as _;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Initialize the heap so the contract's alloc-backed types link/run.
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 4096;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(core::ptr::addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
    }

    let _p = embassy_rp::init(Default::default());

    // Prove the firmware shares the single-source-of-truth contract (FR-6).
    let _id = SelfId {
        id: "mol-001".into(),
        name: "Mini-Molecule".into(),
        fw_version: "0.1.0".into(),
        capabilities: Default::default(),
    };

    loop {
        core::hint::spin_loop();
    }
}
