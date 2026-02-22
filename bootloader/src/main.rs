#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi::{cstr16, system};

#[entry]
fn main() -> Status {
    system::with_stdout(|stdout| -> uefi::Result<()> {
        stdout.reset(false)?;
        stdout.output_string(cstr16!("Hello\r\n"))?;
        Ok(())
    })
    .unwrap();

    loop {
        core::hint::spin_loop();
    }
}
