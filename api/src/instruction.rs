use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum ZeInstruction {
    // User
    Claim = 0,
    Close = 1,
    Mine = 2,
    Open = 3,
    Reset = 4,
    #[deprecated(since = "2.4.0", note = "Please stake with the boost program")]
    Stake = 5,
    Update = 6,
    #[deprecated(since = "2.6.0", note = "v1 tokens are no longer eligable to upgrade")]
    Upgrade = 7,

    // Admin
    Initialize = 100,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Claim {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mine {
    pub digest: [u8; 16],
    pub nonce: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Open {
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bump: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Reset {}

#[deprecated(since = "2.4.0", note = "Please stake with the boost program")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Stake {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Update {}

#[deprecated(since = "2.6.0", note = "v1 tokens are no longer eligable to upgrade")]
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Upgrade {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Initialize {
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_0_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_1_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_2_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_3_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_4_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_5_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_6_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub bus_7_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub config_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub metadata_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub mint_bump: u8,
    #[deprecated(since = "2.5.0", note = "Bump no longer used")]
    pub treasury_bump: u8,
}

instruction!(ZeInstruction, Claim);
instruction!(ZeInstruction, Close);
instruction!(ZeInstruction, Mine);
instruction!(ZeInstruction, Open);
instruction!(ZeInstruction, Reset);
instruction!(ZeInstruction, Stake);
instruction!(ZeInstruction, Update);
instruction!(ZeInstruction, Upgrade);
instruction!(ZeInstruction, Initialize);
