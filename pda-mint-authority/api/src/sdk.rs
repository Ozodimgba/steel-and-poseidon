use steel::*;
use crate::prelude::*;

pub fn create_token(
    payer: Pubkey,
    token_name: String,
    token_symbol: String,
    token_uri: String
) -> Instruction {
    let (mint_pda, bump) = Pubkey::find_program_address(&[b"mint"], &crate::ID);
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[b"metadata", &crate::ID.to_bytes(), &mint_pda.to_bytes()],
        &crate::ID
    );

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(mint_pda, false),
            AccountMeta::new(metadata_pda, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(mpl_token_metadata::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::rent::ID, false),
        ],
        data: CreateToken {
            token_name: token_name.as_bytes()[..32].try_into().unwrap(),
            token_symbol: token_symbol.as_bytes()[..10].try_into().unwrap(),
            token_uri: token_uri.as_bytes()[..200].try_into().unwrap(),
            bump: bump
        }.to_bytes(),
    }
}

pub fn mint_token(payer: Pubkey, amount: u64) -> Instruction {
    let (mint_pda, _) = Pubkey::find_program_address(&[b"mint"], &crate::ID);
    let associated_token_account = spl_associated_token_account::get_associated_token_address(&payer, &mint_pda);

    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(mint_pda, false),
            AccountMeta::new(associated_token_account, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: MintToken {
            amount: amount,
        }.to_bytes(),
    }
}
