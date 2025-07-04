use anchor_lang::{prelude::*, system_program};

use crate::{
    constant::*,
    state::{OmniFarmingInfo},
    transfer_helper::token_transfer_from_user,
};

pub struct WithdrawFee<'info> {
    #[account(mut)]
    pub omnifarming_info: Account<'info, OmniFarmingInfo>,

    /// CHECK
    #[account(
        seeds = [OMNIFARMING_VAULT_SEED],
        bump,
    )]
    pub omnifarming_vault: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [SHARE_TOKEN],
        bump,
    )]
    pub share_mint: Account<'info, TokenMint>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub fee_receiver: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> WithdrawFee<'info> {
    pub fn process(ctx: Context<WithdrawFee>) -> Result<()> {
        // Calculate the fee amount
        let now = Clock::get()?.unix_timestamp;
        let time = now - ctx.accounts.omnifarming_info.last_fee_collection_time;
        let total_supply = ctx.accounts.share_mint.supply + ctx.accounts.omnifarming_info.total_locked_shares;
        let fee_on_a_year = (total_supply * ctx.accounts.omnifarming_info.management_fee) / BPS_BASE;

        let fees = if time > 0 {
            (fee_on_a_year * time as u64) / 365
        } else {
            0
        };

        if fees > 0 {
            // Mint the fee to the fee receiver
            let pda_seeds = &[
                OMNIFARMING_VAULT_SEED,
                &[ctx.bumps.omnifarming_vault],
            ];
            mint_shares_to_user(
                ctx.accounts.share_mint.to_account_info(),
                ctx.accounts.omnifarming_vault.to_account_info(),
                ctx.accounts.fee_receiver.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                pda_seeds,
                fees,
            )?;
        }

        // Update the last fee collection time
        ctx.accounts.omnifarming_info.last_fee_collection_time = Clock::get()?.unix_timestamp;
        Ok(())
    }
}