use anyhow::Context;

use crate::cpu::{CPU, MINIMUM_MEMORY_SIZE};
use crate::loader::load_and_verify;

mod loader;
mod cpu;

fn main() -> anyhow::Result<()> {
    let args = std::env::args();
    let mut args = args.skip(1); // Skip program name
    let file_name = args.next()
        .context("Required argument file name not missing")
        .unwrap();
    let code = load_and_verify(file_name)?;
    let cpu = CPU::new(MINIMUM_MEMORY_SIZE, code);
    cpu.run();
    Ok(())
}
