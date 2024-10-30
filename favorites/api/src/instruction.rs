use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum FavoritesInstruction {
    SetFavoritesArgs = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFavoritesArgs {
    pub number: u64,
    pub color_len: u32,
    pub color: [u8; 32],
    pub hobbies_count: u32,
    pub hobbies_len: [u32; 32],
    pub hobbies: [[u8; 32]; 32],
}


instruction!(FavoritesInstruction, SetFavoritesArgs);

