pub mod initialize_mint;
pub mod mint;

/// RewardInstruction
use solana_program::program_error::ProgramError;
use {
    crate::instructions::RewardInstruction::{InitializeMint, Mint},
    spl_token::error::TokenError,
};
pub use {initialize_mint::*, mint::*};

#[derive(Clone, Debug, PartialEq)]
pub enum RewardInstruction {
    InitializeMint,
    Mint,
}

impl RewardInstruction {
    /// Unpacks a byte buffer into a [TokenInstruction](enum.TokenInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;

        let (&tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => InitializeMint,
            1 => Mint,
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
