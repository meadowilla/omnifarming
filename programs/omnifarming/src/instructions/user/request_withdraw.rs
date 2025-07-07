use anchor_lang::prelude::*;
use crate::{
    constants::{OMNIFARMING_INFO_SEED, OMNIFARMING_USER_SEED, SHARE_TOKEN},
}

pub struct RequestWithdraw<'info> {
    #[account(
        mut,
        seeds = [OMNIFARMING_INFO_SEED],
        bump,
    )]
    pub omnifarming_info: Account<'info, OmniFarmingInfo>,

    #[account(
        mut,
        seeds = [OMNIFARMING_USER_SEED, user.key().as_ref(), token_mint.key().as_ref()],
        bump,
    )]
    pub user_reserve: Account<'info, OmniFarmingUser>,

    pub token_mint: Account<'info, TokenMint>,
    #[account(
        seeds = [SHARE_TOKEN],
        bump,
    )]
    pub share_mint: Account<'info, TokenMint>,

    pub user: Signer<'info>,
    pub token_program: Program<'info, TokenProgram>,

    #[account(
        associated_token::mint = share_mint,
        associated_token::authority = user,
    )]
    pub user_share_account: Account<'info, TokenAccount>,

    // modify the state in helper functions ? is it ok?
    // NO! Due to modify state directly requires the accounts to be defined in the context, 
    // which cannot be done in helper functions!

}

impl<'info> RequestWithdraw<'info> {
    pub fn process(ctx: Context<RequestWithdraw>, amount: u64) -> Result<()> {
        require!(amount >= ctx.accounts.omnifarming_info.min_withdraw, 
            OmniFarmingError::WithdrawAmountTooLow);

        require!(ctx.accounts.user_reserve.balance_locked == 0, 
            OmniFarmingError::ProcessingWithdrawal);

        require!(ctx.accounts.user_share_account.amount >= amount,
            OmniFarmingError::InefficientShares);

        // Burn shares from user
        burn_shares_from_user(
            ctx.accounts.share_mint.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.user_share_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            amount,
        )?;

        // Update user reserve, and omnifarming info
        ctx.accounts.user_reserve.balance_locked = amount;
        ctx.accounts.omnifarming_info.total_locked_shares += amount;

        Ok(())
    }
}