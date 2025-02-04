use steel::*;

use super::ZeAccount;

/// Treasury is a singleton account which is the mint authority for the ZE token and the authority of
/// the program's global token account.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct Treasury {}

account!(ZeAccount, Treasury);
