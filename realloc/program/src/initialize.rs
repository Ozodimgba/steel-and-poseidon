use realloc_api::ID;
use realloc_api::prelude::*;
use steel::*;
use solana_program::{
    system_instruction,
    program::invoke_signed,
    msg,
};
use steel::sysvar::rent::Rent;

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    msg!("Starting process_initialize");
    msg!("Received data length: {}", data.len());
    msg!("Raw data: {:?}", data);

    // Parse accounts
    let [payer_info, message_account_info, system_program] = accounts else {
        msg!("❌ Error: Account parsing failed - incorrect number of accounts");
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    msg!("✓ Accounts parsed successfully");

    // Validate accounts
    if let Err(e) = payer_info.is_signer() {
        msg!("❌ Error: Payer is not a signer: {:?}", e);
        return Err(e);
    }
    msg!("✓ Payer signature verified");

    if let Err(e) = system_program.is_program(&system_program::ID) {
        msg!("❌ Error: Invalid system program: {:?}", e);
        return Err(e);
    }
    msg!("✓ System program validated");

    // Parse instruction data
    let args = match Initialize::try_from_bytes(data) {
        Ok(args) => {
            msg!("✓ Instruction data parsed successfully");
            args
        },
        Err(e) => {
            msg!("❌ Error: Failed to parse instruction data: {:?}", e);
            return Err(e.into());
        }
    };

    // Calculate required space
    let required_space = Message::required_space(args.message_len as usize);
    msg!("Required space calculated: {} bytes", required_space);

    // Calculate minimum rent
    let rent = match Rent::get() {
        Ok(rent) => {
            msg!("✓ Rent sysvar retrieved successfully");
            rent
        },
        Err(e) => {
            msg!("❌ Error: Failed to get rent sysvar: {:?}", e);
            return Err(e);
        }
    };
    let lamports = rent.minimum_balance(required_space);
    msg!("Minimum balance calculated: {} lamports", lamports);

    // Generate PDA seeds
    let seeds = &[&b"message"[..]];
    let (_pda, bump) = Pubkey::find_program_address(seeds, &ID);
    msg!("PDA generated with bump: {}", bump);

    // Create the account
    let ix = system_instruction::create_account(
        &payer_info.key,
        &message_account_info.key,
        lamports,
        required_space as u64,
        &ID,
    );
    msg!("System instruction created");

    // Sign and send instruction
    if let Err(e) = invoke_signed(
        &ix,
        &[
            payer_info.clone(),
            message_account_info.clone(),
            system_program.clone(),
        ],
        &[&[b"message", &[bump]]],
    ) {
        msg!("❌ Error: Failed to create account: {:?}", e);
        return Err(e);
    }
    msg!("✓ Account created successfully");

    // Initialize the account data
    let message = match message_account_info.to_account_mut::<Message>(&ID) {
        Ok(msg) => {
            msg!("✓ Message account loaded successfully");
            msg
        },
        Err(e) => {
            msg!("❌ Error: Failed to load message account: {:?}", e);
            return Err(e);
        }
    };

    // Initialize data
    message.message_len = args.message_len;
    message.message[..args.message_len as usize].copy_from_slice(&args.message[..args.message_len as usize]);
    msg!("✓ Message data initialized successfully");

    msg!("✓ Process complete - initialization successful");
    Ok(())
}