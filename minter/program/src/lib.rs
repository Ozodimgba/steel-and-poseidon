mod add;
mod initialize;

use add::*;
use initialize::*;
        
use minter_api::prelude::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&minter_api::ID, program_id, data)?;

    match ix {
        MinterInstruction::Initialize => process_initialize(accounts, data)?,
        MinterInstruction::Add => process_add(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
