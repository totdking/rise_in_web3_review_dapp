pub mod errors;
pub mod instructions;
pub mod processor;
pub mod state;
use processor::process_instruction;

use solana_program::entrypoint;

entrypoint!(process_instruction);
