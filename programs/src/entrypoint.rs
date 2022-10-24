#[rustfmt::skip]
use {
    crate::{errors::RewardError, processor::Processor},
    solana_program::{
        account_info::AccountInfo,
        entrypoint,
        entrypoint::{ProgramResult},
        program_error::PrintProgramError,
        pubkey::Pubkey,
    },
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // TODO: FIX THIS IS TAKING TOO MUCH COMPUTE UNIT
    // msg!(
    //     "entrypoint.process_instruction: {}: {:#?} accounts, data={:#?}",
    //     program_id,
    //     accounts.len(),
    //     instruction_data
    // );

    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<RewardError>();
        return Err(error);
    }
    Ok(())
}
