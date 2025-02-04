mod claim;
mod close;
mod initialize;
mod mine;
mod open;
mod reset;
mod stake;
mod update;
mod upgrade;

use claim::*;
use close::*;
use initialize::*;
use mine::*;
use open::*;
use reset::*;
use stake::*;
use update::*;
use upgrade::*;

use ze_api::instruction::*;
// use solana_include_idl::{include_idl, parse::IdlType};
use steel::*;

// include_idl!(IdlType::Codama, concat!(env!("OUT_DIR"), "/codama.idl.zip"));

#[allow(deprecated)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ze_api::ID, program_id, data)?;

    match ix {
        ZeInstruction::Claim => process_claim(accounts, data)?,
        ZeInstruction::Close => process_close(accounts, data)?,
        ZeInstruction::Mine => process_mine(accounts, data)?,
        ZeInstruction::Open => process_open(accounts, data)?,
        ZeInstruction::Reset => process_reset(accounts, data)?,
        ZeInstruction::Stake => process_stake(accounts, data)?,
        ZeInstruction::Update => process_update(accounts, data)?,
        ZeInstruction::Upgrade => process_upgrade(accounts, data)?,
        ZeInstruction::Initialize => process_initialize(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
