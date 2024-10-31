use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum AccountType {
    Message = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Message {
    pub message_len: u32,
    pub message: [u8; 1024], // Max size, actual used space will be determined during allocation
}

impl Message {
    pub fn required_space(message_len: usize) -> usize {
        DISCRIMINATOR_SIZE + // discriminator
            4 + // message length
            message_len // actual message bytes
    }

    pub fn find_pda(program_id: &Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[MESSAGE_SEED], program_id)
    }
}

account!(AccountType, Message);
