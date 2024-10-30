use steel::*;

use super::CheckingAccountsAccount;

// Use the account! macro to link account structs with a discriminator and implement basic serialization logic.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Counter {
    pub value: u64 
}

account!(CheckingAccountsAccount, Counter);
// We have successfully linked our Counter account to the discriminator CheckingAccountsAccount
