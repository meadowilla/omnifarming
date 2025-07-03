use anchor_lang::prelude::*;

declare_id!("B61XPDSuobij3c4aBvyeVmKfBJVdTzkV2Xn4TpM1AtGS");

use instructions::*;

#[program]
pub mod omnifarming {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>, 
        governance: Pubkey, 
        agent: Pubkey,
        beneficiary: Pubkey,
        min_deposit: u64,
        min_withdraw: u64,
        fee_bps: u64,
        init_deposit_amount: u64,
    ) -> Result<()> {
        return Initialize::process(ctx, governance, agent, fee_receiver, min_deposit, min_withdraw, fee_bps, init_deposit_amount);
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        return Deposit::process(ctx, amount);
    }

    pub fn request_withdraw(ctx: Context<RequestWithdraw>, amount: u64) -> Result<()> {
        return RequestWithdraw::process(ctx, amount);
    }

    pub fn withdraw(ctx: Context<Withdraw>, user: Pubkey) -> Result<()> {
        return Withdraw::process(ctx, user);
    }

    pub fn update_fee(ctx: Context<UpdateFee>) -> Result<()> {
        return UpdateFee::process(ctx);
    }

    pub fn update_config(ctx: Context<UpdateConfig>, new_config: OmniFarmingConfig) -> Result<()> {
        return UpdateConfig::process(ctx, new_config);
    }
}
