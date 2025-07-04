use anchor_lang::{prelude::*, system_program};

use crate::{
    constant::*,
    state::{OmniFarmingInfo},
    helper::preview_deposit,
    transfer_helper::{token_transfer_from_user, mint_shares_to_user},
};

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub omnifarming_info: Account<'info, OmniFarmingInfo>,

    /// CHECK
    #[account(
        seeds = [OMNIFARMING_VAULT_SEED],
        bump,
    )]
    pub omnifarming_vault: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        seeds = [
            OMNIFARMING_USER_SEED, 
            user.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        payer = user,
        space = 8 + std::mem::size_of::<OmniFarmingUser>(),
    )]
    pub user_reserve: Account<'info, OmniFarmingUser>,

    #[account(mut)]
    pub token_mint: Account<'info, TokenMint>,

    #[account(
        seeds = [SHARE_TOKEN],
        bump,
    )]
    pub share_mint: Account<'info, TokenMint>,

    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == token_mint.key(),
    )]
    pub user_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = share_mint,
        associated_token::authority = user,
    )]
    pub user_share_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [VAULT_TOKEN, token_mint.key().as_ref()],
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = fee_receiver.key() == omnifarming_info.fee_receiver
    )]
    pub fee_receiver: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Deposit<'info> {
    pub fn process(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // Update management fee
        let ctx_withdraw_fee = Context::new(
            ctx.program_id,
            WithdrawFee {
                omnifarming_info: ctx.accounts.omnifarming_info.to_account_info(),
                omnifarming_vault: ctx.accounts.omnifarming_vault.to_account_info(),
                share_mint: ctx.accounts.share_mint.to_account_info(),
                user: ctx.accounts.user.to_account_info(),
                fee_receiver: ctx.accounts.fee_receiver.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );
        WithdrawFee::process(ctx_withdraw_fee)?;

        // Check amount > min_deposit
        let omnifarming_info = &mut ctx.accounts.omnifarming_info;
        require!(
            amount >= omnifarming_info.min_deposit, // > or >= ?
            ErrorCode::DepositAmountTooLow
        );

        // Calculate LP amount
        let shares = preview_deposit(omnifarming_info, amount)?;
        require!(
            shares > 0,
            ErrorCode::DepositSharesTooLow
        );
        
        // Transfer tokens from user to vault
        omnifarming_info.total_assets = omnifarming_info
            .total_assets
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;
            
        token_transfer_from_user(
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.user.to_account_info(),
            ctx.accounts.vault_token_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            amount,
        )?;

        // Mint LP tokens to user
        let pda_seeds = &[&[
            OMNIFARMING_VAULT_SEED,
            &[ctx.bumps["omnifarming_vault"]],
        ]];

        mint_shares_to_user(
            ctx.accounts.share_mint.to_account_info(),
            ctx.accounts.omnifarming_vault.to_account_info(),
            ctx.accounts.user_share_account.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            pda_seeds,
            shares,
        )?;
        Ok(())
    }
}


