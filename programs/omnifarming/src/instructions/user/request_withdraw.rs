use anchor_lang::prelude::*;

pub struct RequestWithdraw<'info> {
    
    

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