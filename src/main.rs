use clap::Parser;

use crate::cpu::{CPU, MINIMUM_MEMORY_SIZE};
use crate::loader::load_and_verify;

mod loader;
mod cpu;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = MINIMUM_MEMORY_SIZE)]
    memory_size: usize,

    file_name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let code = load_and_verify(&args.file_name)?;
    let cpu = CPU::new(args.memory_size, code);
    cpu.run();
    Ok(())
}
