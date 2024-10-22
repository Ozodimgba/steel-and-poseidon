use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum MinterError {
    #[error("This is a dummy error")]
    Dummy = 0,
}

error!(MinterError);
