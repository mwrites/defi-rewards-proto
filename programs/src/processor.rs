use {
    crate::instructions::*,
    solana_program::{
        self,
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_pack::Pack,
        pubkey::Pubkey,
        rent::Rent,
        sysvar::Sysvar,
    },
    std::slice::Iter,
};

pub struct Processor {}

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = RewardInstruction::unpack(input)?;
        // let instruction = RewardInstruction::try_from_slice(input)
        //     .map_err(|_| ProgramError::InvalidInstructionData)?;
        // msg!("processor.process - Instruction: {:#?}", instruction);

        let accounts_iter = &mut accounts.iter();
        match instruction {
            RewardInstruction::InitializeMint => {
                Self::process_initialize_mint(program_id, accounts_iter)
            }
            RewardInstruction::Mint => Self::process_mint(program_id, accounts_iter),
        }
    }

    pub fn process_initialize_mint(
        program_id: &Pubkey,
        accounts_iter: &mut Iter<AccountInfo>,
    ) -> ProgramResult {
        // 1. Payer account for the state account creation
        let payer = next_account_info(accounts_iter)?;
        // 2. Token account we hold
        let to_be_created = next_account_info(accounts_iter)?;
        // 3. Sys Program for creation
        // let system_program = next_account_info(accounts_iter)?;
        // 4. Space for mint account
        let space = spl_token::state::Mint::LEN as u64;
        // 6. Rent
        let lamports_required = (Rent::get()?).minimum_balance(space as usize);

        create_mint_account(program_id, payer, to_be_created, space, lamports_required)?;
        msg!(
            "Processor.process.process_initialize_mint - create_mint_account success: {:#?}",
            to_be_created
        );

        initialize_mint(to_be_created, program_id)?;
        msg!(
            "Processor.process.process_initialize_mint - initialize_mint success: {:#?}",
            to_be_created
        );

        Ok(())
    }

    // TODO: move this shit into a macro instead
    // pub fn mint_pda_seeds(program_id: &Pubkey, mut seeds: &[&[u8]]) {
    //     let (mint_pda, mint_bump) = Pubkey::find_program_address(&[MINT_SEED], program_id);
    // }

    pub fn process_mint(
        program_id: &Pubkey,
        accounts_iter: &mut Iter<AccountInfo>,
    ) -> ProgramResult {
        let mint_pda = next_account_info(accounts_iter)?;
        msg!("mint_pda {:#?}", mint_pda.key.to_string());
        let wallet = next_account_info(accounts_iter)?;
        msg!("wallet: {:#?}", wallet.key.to_string());
        let mint_dest_ata_tbc = next_account_info(accounts_iter)?;
        msg!(
            "mint_dest_ata_tbc: {:#?}",
            mint_dest_ata_tbc.key.to_string()
        );

        let sys_program = next_account_info(accounts_iter)?;
        msg!("sys_program{:#?}", sys_program.key.to_string());
        let token_program = next_account_info(accounts_iter)?;
        msg!("token_program {:#?}", token_program.key.to_string());

        // 1️⃣ Ask Associated Token (CPI) Program to create ATA
        create_mint_dest_ata(
            // sys_program,
            // token_program,
            wallet,
            mint_pda,
            mint_dest_ata_tbc,
        )?;
        // eating too much CUs
        // msg!("Processor.process.process_mint - create_dest_ata success: {:#?}", mint_dest_ata_tbc);
        msg!("Processor.process.process_mint - create_dest_ata success");

        // 2️⃣ Ask Token Program (CPI) to mint the token to that ATA
        let mint_dest_ata = mint_dest_ata_tbc;
        mint(program_id, mint_pda, mint_dest_ata)?;
        // eating too much CUs
        // msg!("Processor.process.process_mint - create_dest_ata success: {:#?}", mint_dest_ata_tbc);
        // msg!("Processor.process.process_mint - mint success: {:#?}", mint_dest_ata_tbc);
        msg!("Processor.process.process_mint - mint success");

        Ok(())
    }
}
