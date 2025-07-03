use anchor_lang::{prelude::*, system_program};

use crate::{
    constant::*,
    state::{OmniFarmingInfo},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [OMNIFARMING_INFO_SEED],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<OmniFarmingInfo>(),
    )]
    pub omnifarming_info: Account<'info, OmniFarmingInfo>,

    /// CHECK
    #[account(
        init,
        seeds = [OMNIFARMING_VAULT_SEED],
        bump,
        payer = payer,
        space = 0,
        owner = system_program::ID,
    )]
    pub omnifarming_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub governance: Signer<'info>,
    pub agent: Signer<'info>,
    pub fee_receiver: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(
        ctx: Context<Initialize>,
        governance: Pubkey,
        agent: Pubkey,
        fee_receiver: Pubkey,
        min_deposit: u64,
        min_withdraw: u64,
        fee_bps: u64,
        init_deposit_amount: u64,
    ) -> Result<()> {
        let omnifarming_info = &mut ctx.accounts.omnifarming_info;

        omnifarming_info.bump = ctx.bumps.omnifarming_vault;
        omnifarming_info.governance = governance;
        omnifarming_info.agent = agent;
        omnifarming_info.fee_receiver = fee_receiver;
        omnifarming_info.total_supply = 0;
        omnifarming_info.total_supply_locked = 0;
        omnifarming_info.min_deposit_amount = min_deposit;
        omnifarming_info.min_shares_withdrawal = min_withdraw;
        omnifarming_info.management_fee = fee_bps;
        // Initial deposit here
        // ...
        Ok(())
    }
}