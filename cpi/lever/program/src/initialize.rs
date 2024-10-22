use lever_api::prelude::*;
use steel::*;

pub fn process_initialize(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [power_info, user_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    user_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    create_account::<PowerStatus>(
        power_info,
        system_program,
        user_info,
        &ID,
        &[b"power"],
    )?;

    let power = power_info.as_account_mut::<PowerStatus>(&ID)?;
    power.is_on = 0;

    Ok(())
}
