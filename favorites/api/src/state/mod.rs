mod favorites;

pub use favorites::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum FavoritesAccount {
    Favorites = 0
}
