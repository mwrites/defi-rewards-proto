use {
    crate::state::*,
    solana_program::{
        self,
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        program::{invoke, invoke_signed},
        pubkey::Pubkey,
    },
};

pub fn create_mint_dest_ata<'a>(
    // sys_program: &AccountInfo<'a>,
    // token_program: &AccountInfo<'a>,
    payer: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    dest_ata_tbc: &AccountInfo<'a>,
) -> ProgramResult {
    // TODO: /Users/matanwrites/.cargo/registry/src/github.com-1ecc6299db9ec823/anchor-attribute-account-0.25.0/src/lib.rs

    let create_ata_ix =
        spl_associated_token_account::instruction::create_associated_token_account_idempotent(
            payer.key,
            payer.key,
            mint.key,
            &spl_token::ID,
        );
    // TODO: seems like we don't need to send all of the accounts as long as the client do it for us, bug? lol
    let create_ata_ix_accounts = [payer.clone(), dest_ata_tbc.clone(), mint.clone()];
    // [writable,signer] Funding account (must be a system account)
    // [writeable] Associated token account address to be created
    //     [] Wallet address for the new associated token account
    //     [] The token mint for the new associated token account
    //     [] System program
    //     [] SPL Token program
    // AccountMeta::new(*funding_address, true),
    // AccountMeta::new(associated_account_address, false),
    // AccountMeta::new_readonly(*wallet_address, false),
    // AccountMeta::new_readonly(*token_mint_address, false),
    // AccountMeta::new_readonly(solana_program::system_program::id(), false),
    // AccountMeta::new_readonly(*token_program_id, false),
    invoke(&create_ata_ix, &create_ata_ix_accounts)
}

pub fn mint<'a>(
    program_id: &Pubkey,
    mint_pda: &AccountInfo<'a>,
    dest_ata: &AccountInfo<'a>,
) -> ProgramResult {
    // TODO add the bump in the account
    let (_unused, mint_bump) = Pubkey::find_program_address(&[MINT_SEED], program_id);
    let seeds = &[MINT_SEED, &[mint_bump]];
    let signers_seed = [&seeds[..]];

    let signer_pubkeys = [mint_pda.key];

    // Maybe use MintTo directly since we control the decimals config?
    let mint_ix = spl_token::instruction::mint_to_checked(
        &spl_token::ID,
        mint_pda.key,
        dest_ata.key,
        mint_pda.key,
        &signer_pubkeys,
        MINT_AMOUNT,
        MINT_DECIMALS,
    )?;

    // TODO: seems like we don't need to send all of the accounts as long as the client do it for us, bug? lol
    let mint_ix_accounts = [
        mint_pda.clone(), // mint_info
        dest_ata.clone(),
        mint_pda.clone(), // mint_auth
    ];

    // /Users/matanwrites/.cargo/registry/src/github.com-1ecc6299db9ec823/spl-token-3.5.0/src/instruction.rs

    // let account_info_iter = &mut accounts.iter();
    // let mint_info = next_account_info(account_info_iter)?;
    // let destination_account_info = next_account_info(account_info_iter)?;
    // let owner_info = next_account_info(account_info_iter)?;

    invoke_signed(&mint_ix, &mint_ix_accounts, &signers_seed)
}
