use solana_program::program_option::COption;
use solana_program::sysvar::rent;
use steel::*;
use api::prelude::*;
use spl_token::state::Mint;


use spl_token::solana_program::program_pack::Pack;


pub fn process_create_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Derserialize your data bytes
    let args = CreateToken::try_from_bytes(data)?;

    // Load the accounts
    let [
    payer,
    mint_account,
    mint_authority,
    metadata_account,
    token_program,
    // update_authority_info,
    token_metadata_program,
    system_program,
    rent
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validation and checks start here
    payer.is_signer()?;

    mint_account
        .is_empty()?
        .has_seeds(&[b"mint"], args.bump, &api::ID)?
        .to_mint()?
        .check(|m| m.mint_authority == COption::from(*mint_authority.key))?
        .check(|m| m.freeze_authority == COption::from(*mint_authority.key))?;

    metadata_account
        // .to_account_mut::<CreateToken>(&api::ID)?
        .has_seeds(&[b"metadata", token_metadata_program.key.as_ref(), mint_account.key.as_ref()], args.bump, token_metadata_program.key)?;

    token_program.is_program(&spl_token::ID)?;
    token_metadata_program.is_program(&mpl_token_metadata::ID)?;
    system_program.is_program(&system_program::ID)?;
    rent.is_program(&rent::ID)?;
    // and ends here

    // Validate mint PDA
    let (mint_pda, bump) = Pubkey::find_program_address(&[MintAuthorityPda::SEED_PREFIX.as_bytes()], &api::ID);
    assert!(&mint_pda.eq(mint_authority.key));

    if mint_pda != *mint_account.key {
        return Err(ProgramError::InvalidAccountData);
    }


    // Initialize the account as a Mint (standard Mint)
    solana_program::msg!("Initializing mint account...");
    solana_program::msg!("Mint: {}", mint_account.key);

    // Initialize mint.
    allocate_account(
        mint_account,
        &spl_token::ID,
        Mint::LEN,
        &[MINT, MintAuthorityPda::SEED_PREFIX.as_bytes(), &[args.bump]],
        system_program,
        payer,
    )?;

    solana_program::program::invoke_signed(
        &spl_token::instruction::initialize_mint(
            &spl_token::ID,
            mint_account.key,
            mint_authority.key,
            None,
            TOKEN_DECIMALS,
        )?,
        &[
            token_program.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            rent.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[args.bump]]],
    )?;

    // Create metadata account
    solana_program::msg!("Creating metadata account...");
    solana_program::msg!("Metadata account address: {}", metadata_account.key);

    let token_name = std::str::from_utf8(&args.token_name).unwrap().trim_matches(char::from(0));
    let token_symbol = std::str::from_utf8(&args.token_symbol).unwrap().trim_matches(char::from(0));
    let token_uri = std::str::from_utf8(&args.token_uri).unwrap().trim_matches(char::from(0));

    let data = mpl_token_metadata::types::DataV2 {
        name: token_name.to_string(),
        symbol: token_symbol.to_string(),
        uri: token_uri.to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let create_metadata_account_v3 = mpl_token_metadata::instructions::CreateMetadataAccountV3 {
        metadata: *metadata_account.key,
        mint: *mint_account.key,
        mint_authority: *mint_authority.key,
        payer: *payer.key,
        update_authority: (*mint_authority.key, false),
        system_program: *system_program.key,
        rent: Some(*rent.key),
    };

    let ix = create_metadata_account_v3.instruction(mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs {
        data,
        is_mutable: true,
        collection_details: None,
    });

    solana_program::program::invoke_signed(
        &ix,
        &[
            metadata_account.clone(),
            mint_account.clone(),
            mint_authority.clone(),
            payer.clone(),
            system_program.clone(),
            rent.clone(),
        ],
        &[&[MintAuthorityPda::SEED_PREFIX.as_bytes(), &[bump]]],
    )?;

    Ok(())
}