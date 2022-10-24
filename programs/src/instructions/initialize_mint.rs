use {
    crate::state::*,
    solana_program::{
        self, account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
        pubkey::Pubkey,
    },
};

pub fn create_mint_account<'a>(
    program_id: &Pubkey,
    payer: &AccountInfo<'a>,
    mint_pda_tbc: &AccountInfo<'a>,
    space: u64,
    lamports_required: u64,
) -> ProgramResult {
    // TODO: validate that pda given and pda found match
    let (_mint_pda, mint_bump) = Pubkey::find_program_address(&[MINT_SEED], program_id);
    let seeds = &[MINT_SEED, &[mint_bump]];
    let signers_seed = [&seeds[..]];

    if mint_pda_tbc.lamports() == 0 {
        msg!("create_mint_account THIS ACCOUNT HAS NO LAMPORTS, CONTINUING AS USUAL");
        let create_ix = solana_program::system_instruction::create_account(
            payer.key,
            mint_pda_tbc.key,
            lamports_required,
            space,
            &spl_token::ID,
        );
        let create_ix_accounts = [payer.clone(), mint_pda_tbc.clone()];
        invoke_signed(&create_ix, &create_ix_accounts, &signers_seed)
    } else {
        msg!("create_mint_account THIS ACCOUNT ALREADY GOT LAMPORTS, REFUNDING THE PAYER FIRST");
        // If the account being initialized already has lamports, then
        // return them all back to the payer so that the account has
        // zero lamports when the system program's create instruction
        // is eventually called.

        // the account already has lamports, so it must be already initialized, we need to:
        // 1. refund the payer
        // 2. erase the space
        let allocate_ix = solana_program::system_instruction::allocate(mint_pda_tbc.key, space);
        let allocate_ix_accounts = [mint_pda_tbc.clone()];
        invoke_signed(&allocate_ix, &allocate_ix_accounts, &signers_seed)?;

        // 3. assign the account's owner to the Token Program (we are not sure what the owner was)
        let assign_ix =
            solana_program::system_instruction::assign(mint_pda_tbc.key, &spl_token::ID);
        let assign_ix_accounts = [mint_pda_tbc.clone()];
        invoke_signed(&assign_ix, &assign_ix_accounts, &signers_seed)
    }
}

pub fn initialize_mint<'a>(mint: &AccountInfo<'a>, program_id: &Pubkey) -> ProgramResult {
    let mint_seed = b"beef_mint".as_ref(); // to_bytes?
    let (_mint_pda, mint_bump) = Pubkey::find_program_address(&[mint_seed], program_id);
    // TODO: replace mint with mint_pda?
    let seeds = &[mint_seed, &[mint_bump]];
    let signers_seed = [&seeds[..]];

    let initialize_mint2_ix = spl_token::instruction::initialize_mint2(
        &spl_token::ID,
        mint.key,
        mint.key, // TODO: SHOULD WE ASSIGN ANOTHER PDA INSTEAD?
        None,
        MINT_DECIMALS,
    )?;

    let initialize_mint2_accounts = [mint.clone()];
    invoke_signed(
        &initialize_mint2_ix,
        &initialize_mint2_accounts,
        &signers_seed,
    )
}
