use anchor_lang::prelude::*;
use anchor_spl::token::{
    self, Mint, Token, TokenAccount,
    Transfer, MintTo, Burn,
};

declare_id!("VaUlt4626pHkYwWjSfERn6y6oAYcg4UH8zQcAXXXXXXXXX"); // replace after deploy

#[account]
pub struct VaultState {
    pub share_mint: Pubkey,
    pub pda_bump:   u8,
}
const VAULT_STATE_SIZE: usize = 8 + 32 + 1;

#[program]
pub mod spl_4626_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state        = &mut ctx.accounts.vault_state;
        state.share_mint = ctx.accounts.share_mint.key();
        state.pda_bump   = *ctx.bumps.get("token_account_owner_pda").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, assets: u64) -> Result<()> {
        require!(assets > 0, CustomError::ZeroAmount);
        token::transfer(ctx.accounts.transfer_into_vault_ctx(), assets)?;

        let supply        = ctx.accounts.share_mint.supply;
        let total_assets  = ctx.accounts.vault_token_account.amount;
        let shares = if supply == 0 {
            assets
        } else {
            (assets as u128 * supply as u128 / total_assets as u128) as u64
        };

        token::mint_to(ctx.accounts.mint_shares_ctx(), shares)?;

        emit!(DepositEvt {
            caller:   ctx.accounts.signer.key(),
            receiver: ctx.accounts.signer.key(),
            assets,
            shares,
        });
        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>, shares: u64) -> Result<()> {
        require!(shares > 0, CustomError::ZeroAmount);

        let supply        = ctx.accounts.share_mint.supply;
        let total_assets  = ctx.accounts.vault_token_account.amount;
        let assets        = (shares as u128 * total_assets as u128 / supply as u128) as u64;

        token::burn(ctx.accounts.burn_shares_ctx(), shares)?;
        token::transfer(ctx.accounts.transfer_out_ctx(), assets)?;

        emit!(WithdrawEvt {
            caller:   ctx.accounts.signer.key(),
            receiver: ctx.accounts.signer.key(),
            owner:    ctx.accounts.signer.key(),
            assets,
            shares,
        });
        Ok(())
    }
}

#[event]
pub struct DepositEvt {
    pub caller:   Pubkey,
    pub receiver: Pubkey,
    pub assets:   u64,
    pub shares:   u64,
}
#[event]
pub struct WithdrawEvt {
    pub caller:   Pubkey,
    pub receiver: Pubkey,
    pub owner:    Pubkey,
    pub assets:   u64,
    pub shares:   u64,
}

#[error_code]
pub enum CustomError {
    #[msg("amount must be > 0")]
    ZeroAmount,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer  = signer,
        seeds  = [b"vault_state"],
        bump,
        space  = VAULT_STATE_SIZE,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(seeds = [b"token_account_owner_pda"], bump)]
    /// CHECK: program‑derived signer
    pub token_account_owner_pda: AccountInfo<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"share_mint"],
        bump,
        mint::decimals   = mint_of_token_being_sent.decimals,
        mint::authority  = token_account_owner_pda,
    )]
    pub share_mint: Account<'info, Mint>, // Share token

    #[account(
        init,
        payer = signer,
        seeds = [b"token_vault", mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint      = mint_of_token_being_sent,
        token::authority = token_account_owner_pda,
    )]
    pub vault_token_account: Account<'info, TokenAccount>, // ATA for vault of USDC

    pub mint_of_token_being_sent: Account<'info, Mint>, // Token being deposited (USDC)

    #[account(mut)] pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program:  Program<'info, Token>,
    pub rent:           Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds=[b"vault_state"], bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds=[b"token_account_owner_pda"], bump)]
    /// CHECK:
    pub token_account_owner_pda: AccountInfo<'info>,

    #[account(mut,
        seeds=[b"token_vault", mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint      = mint_of_token_being_sent,
        token::authority = token_account_owner_pda
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)] pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub sender_share_account: Account<'info, TokenAccount>,

    #[account(seeds=[b"share_mint"], bump)]
    pub share_mint: Account<'info, Mint>,
    pub mint_of_token_being_sent: Account<'info, Mint>,

    #[account(mut)] pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut, seeds=[b"vault_state"], bump)]
    pub vault_state: Account<'info, VaultState>,
    #[account(seeds=[b"token_account_owner_pda"], bump)]
    /// CHECK:
    pub token_account_owner_pda: AccountInfo<'info>,

    #[account(mut,
        seeds=[b"token_vault", mint_of_token_being_sent.key().as_ref()],
        bump,
        token::mint      = mint_of_token_being_sent,
        token::authority = token_account_owner_pda
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    #[account(mut)] pub sender_token_account: Account<'info, TokenAccount>,
    #[account(mut)] pub sender_share_account: Account<'info, TokenAccount>,

    #[account(seeds=[b"share_mint"], bump)]
    pub share_mint: Account<'info, Mint>,
    pub mint_of_token_being_sent: Account<'info, Mint>,

    #[account(mut)] pub signer: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Deposit<'info> {
    fn transfer_into_vault_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from:      self.sender_token_account.to_account_info(),
                to:        self.vault_token_account.to_account_info(),
                authority: self.signer.to_account_info(),
            },
        )
    }
    fn mint_shares_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let seeds = &[b"token_account_owner_pda", &[self.vault_state.pda_bump]];
        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint:      self.share_mint.to_account_info(),
                to:        self.sender_share_account.to_account_info(),
                authority: self.token_account_owner_pda.to_account_info(),
            },
            &[seeds],
        )
    }
}

impl<'info> Redeem<'info> {
    fn burn_shares_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Burn<'info>> {
        let seeds = &[b"token_account_owner_pda", &[self.vault_state.pda_bump]];
        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Burn {
                mint:      self.share_mint.to_account_info(),
                from:      self.sender_share_account.to_account_info(),
                authority: self.token_account_owner_pda.to_account_info(),
            },
            &[seeds],
        )
    }
    fn transfer_out_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let seeds = &[b"token_account_owner_pda", &[self.vault_state.pda_bump]];
        CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                from:      self.vault_token_account.to_account_info(),
                to:        self.sender_token_account.to_account_info(),
                authority: self.token_account_owner_pda.to_account_info(),
            },
            &[seeds],
        )
    }
}
