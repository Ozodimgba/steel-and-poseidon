use steel::*;
use crate::consts::*;

// First, define an enum for the account discriminator
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountType {
    MintAuthorityPda = 0,
    // Add other account types here as needed
}

// Now define the actual account state
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct MintAuthorityPda {
    pub bump: u8,
    // pub _padding: [u8; 7], // To ensure 8-byte alignment
}

impl MintAuthorityPda {
    pub const SEED_PREFIX: &'static str = "mint_authority";
    pub const SIZE: usize = 8; // 8 bytes for the struct (1 for bump, 7 for padding)

    // pub fn new(bump: u8) -> Self {
    //     Self {
    //         bump,
    //         // _padding: [0; 7],
    //     }
    // }
}

// // Implement the Discriminator trait for MintAuthorityPdaState
// impl Discriminator for MintAuthorityPda {
//     fn discriminator() -> u8 {
//         AccountType::MintAuthorityPda as u8
//     }
// }

// Use the account! macro to implement necessary traits
account!(AccountType, MintAuthorityPda);

// Helper function to get the total size of the account, including discriminator
pub fn get_mint_authority_pda_size() -> usize {
    8 + MintAuthorityPda::SIZE // 8 bytes for discriminator + size of state
}

pub fn mint_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT], &crate::id())
}