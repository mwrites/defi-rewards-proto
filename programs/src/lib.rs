#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod errors;
pub mod state;

mod instructions;
mod processor;
