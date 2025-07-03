use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct OmniFarmingInfo {
    pub bump: u8,

    pub governance: Pubkey,
    pub agent: Pubkey,
    pub fee_receiver: Pubkey,

    pub total_supply: u64,
    pub total_supply_locked: u64,
    pub min_deposit_amount: u64,
    pub min_shares_withdrawal: u64,

    pub management_fee: u64,
    // pub last_fee_collection_time: i64,
}

#[account]
#[derive(Default)]
pub struct OmniFarmingUser {
    pub balance_locked: u64,
    pub balance: u64,
}

#[account]
#[derive(Default)]
pub struct OmniFarmingConfig {
    pub new_exit_fee: u64,
    pub new_management_fee: u64,
}