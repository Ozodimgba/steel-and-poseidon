use steel::*;
use favorites_api::prelude::*;

pub fn process_set_favorites(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [signer_info, favorites_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate accounts
    signer_info.is_signer()?;
    system_program.is_program(&system_program::ID)?;

    // Parse instruction data
    let args = SetFavoritesArgs::try_from_bytes(data)?;

    // Calculate PDA
    let seeds = &[b"favorites", signer_info.key.as_ref()];
    let (_, bump_seed) = Pubkey::find_program_address(seeds, &favorites_api::ID);

    // If account doesn't exist, create it
    if favorites_info.data_is_empty() {
        create_account::<Favorites>(
            favorites_info,
            &favorites_api::ID,
            seeds,
            system_program,
            signer_info,
        )?;
    }

    // Verify PDA
    favorites_info.has_seeds(seeds, bump_seed, &favorites_api::ID)?;

    // Get mutable reference to account data
    let favorites = favorites_info.to_account_mut::<Favorites>(&favorites_api::ID)?;

    // Update account data
    favorites.number = args.number;
    favorites.color_len = args.color_len;
    favorites.color = args.color;
    favorites.hobbies_count = args.hobbies_count;
    favorites.hobbies_len = args.hobbies_len;
    favorites.hobbies = args.hobbies;

    Ok(())
}
