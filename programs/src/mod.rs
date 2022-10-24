use {
    crate::errors::RewardError,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum RewardInstruction {
    /// Initializes a new mint and optionally deposits all the newly minted
    /// tokens in an account.
    ///
    /// The `InitializeMint` instruction requires no signers and MUST be
    /// included within the same Transaction as the system program's
    /// `CreateAccount` instruction that creates the account being initialized.
    /// Otherwise another party can acquire ownership of the uninitialized
    /// account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The mint to initialize.
    ///   1. `[]` Rent sysvar
    ///
    InitializeMint,

    /// Mints new tokens to an account.  The native mint does not support
    /// minting.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   * Single authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[signer]` The mint's minting authority.
    ///
    ///   * Multisignature authority
    ///   0. `[writable]` The mint.
    ///   1. `[writable]` The account to mint tokens to.
    ///   2. `[]` The mint's multisignature mint-tokens authority.
    ///   3. ..3+M `[signer]` M signer accounts.2
    Mint, // TODO: amount should be hardcoded or configurable in an account with the right authority
          // amount: u64,
}

impl RewardInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use RewardError::InvalidInstruction;

        let (&tag, _rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                Self::InitializeMint
                //     let (&decimals, rest) = rest.split_first().ok_or(InvalidInstruction)?;
                //     let (mint_authority, rest) = Self::unpack_pubkey(rest)?;
                //     let (freeze_authority, _rest) = Self::unpack_pubkey_option(rest)?;
                //     Self::InitializeMint {
                //         mint_authority,
                //         freeze_authority,
                //         decimals,
                //     }
            }
            1 => {
                // let (token_program, rest) = Self::unpack_pubkey(rest)?;
                // let (mint_pda, rest) = Self::unpack_pubkey(rest)?;
                // let (mint_authority_pda, rest) = Self::unpack_pubkey(rest)?;
                // let (dest_ata, _rest) = Self::unpack_pubkey(rest)?;
                // let amount = rest
                //     .get(..8)
                //     .and_then(|slice| slice.try_into().ok())
                //     .map(u64::from_le_bytes)
                //     .ok_or(InvalidInstruction)?;
                Self::Mint
                // amount
                // token_program,
                // mint_pda,
                // mint_authority_pda,
                // dest_ata,
            }
            // 2 => {
            //     let &m = rest.get(0).ok_or(InvalidInstruction)?;
            //     Self::InitializeMultisig { m }
            // }
            _ => return Err(InvalidInstruction.into()),
        })
    }
    //
    // fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
    //     if input.len() >= 32 {
    //         let (key, rest) = input.split_at(32);
    //         let pk = Pubkey::new(key);
    //         msg!("{:#?}", pk);
    //         Ok((pk, rest))
    //     } else {
    //         Err(TokenError::InvalidInstruction.into())
    //     }
    // }
}
//
