use anchor_lang::prelude::*;

#[constant]
pub const BPS_BASE: u64 = 10_000;

pub const OMNIFARMING_INFO_SEED: &[u8] = b"omnifarming_info";
pub const OMNIFARMING_VAULT_SEED: &[u8] = b"omnifarming_vault";
pub const OMNIFARMING_USER_SEED: &[u8] = b"omnifarming_user";