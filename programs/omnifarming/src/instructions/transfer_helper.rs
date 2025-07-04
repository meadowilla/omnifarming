use anchor_lang::prelude::*;

use anchor_spl::token::{self, MintTo, Transfer}

// transfer tokens from user
pub fn token_transfer_from_user<'info>(
    source: AccountInfo<'info>,
    authority: &Signer<'info>,
    destination: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new(
        token_program.to_account_info(),
        token::Transfer {
            from: source,
            to: destination,
            authority: authority.to_account_info(),
        },
    );
    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

// share mint to user
pub fn mint_shares_to_user<'info>(
    share_mint: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    destination: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    pda_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::MintTo {
            mint: share_mint.to_account_info(),
            to: destination,
            authority: authority.to_account_info(),
        },
        pda_seeds,
    );

    token::mint_to(cpi_ctx, amount)?;

    Ok(())
}

// burn shares from user
pub fn burn_shares_from_user<'info>(
    share_mint: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    from: AccountInfo<'info>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_ctx: CpiContext<_> = CpiContext::new_with_signer(
        token_program.to_account_info(),
        token::Burn {
            mint: share_mint.to_account_info(),
            from: from.to_account_info(),
            authority: authority.to_account_info(),
        },
    );

    token::burn(cpi_ctx, amount)?;

    Ok(())
}
