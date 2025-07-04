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

    #[account(
        init_if_needed,
        seeds = [
            OMNIFARMING_USER_SEED, 
            payer.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + std::mem::size_of::<OmniFarmingUser>(),
    )]
    pub payer_reserve: Account<'info, OmniFarmingUser>,

    #[account(
        mut,
        constraint = payer_token_account.owner == payer.key(),
        constraint = payer_token_account.mint == token_mint.key(),
    )]
    pub payer_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = share_mint,
        associated_token::authority = payer,
    )]
    pub payer_share_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = payer,
        seeds = [VAULT_TOKEN, token_mint.key().as_ref()],
        token::mint = token_mint,
        token::authority = omnifarming_vault,
    )]
    pub vault_token_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub token_mint: Account<'info, TokenMint>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 6,
        mint::authority = omnifarming_vault,
        seeds = [SHARE_TOKEN],
        bump,
    )]
    pub share_mint: Account<'info, TokenMint>,

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
        min_deposit: u64,
        min_withdraw: u64,
        fee_bps: u64,
        init_deposit_amount: u64,
    ) -> Result<()> {
        let omnifarming_info = &mut ctx.accounts.omnifarming_info;

        omnifarming_info.bump = ctx.bumps.omnifarming_vault;
        omnifarming_info.governance = ctx.accounts.governance.key();
        omnifarming_info.agent = ctx.accounts.agent.key();
        omnifarming_info.fee_receiver = ctx.accounts.fee_receiver.key();

        omnifarming_info.total_assets = 0;
        omnifarming_info.total_shares = 0;
        omnifarming_info.min_deposit = min_deposit;
        omnifarming_info.min_withdraw = min_withdraw;
        omnifarming_info.management_fee = fee_bps;
        
        // Initial deposit here
        let ctx_deposit = Context::new(
            ctx.program_id,
            Deposit {
                omnifarming_info: ctx.accounts.omnifarming_info.to_account_info(),
                omnifarming_vault: ctx.accounts.omnifarming_vault.to_account_info(),
                user_reserve: ctx.accounts.payer_reserve.to_account_info(),
                user_token_account: ctx.accounts.payer_token_account.to_account_info(),
                user_share_account: ctx.accounts.payer_share_account.to_account_info(),
                vault_token_account: ctx.accounts.vault_token_account.to_account_info(),
                token_mint: ctx.accounts.token_mint.to_account_info(),
                share_mint: ctx.accounts.share_mint.to_account_info(),
                user: ctx.accounts.payer.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        );
        Deposit::process(
            ctx_deposit,
            init_deposit_amount,
        )?;
        
        Ok(())
    }
}