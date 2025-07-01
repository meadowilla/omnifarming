use anchor_lang::prelude::*;

declare_id!("B61XPDSuobij3c4aBvyeVmKfBJVdTzkV2Xn4TpM1AtGS");

#[program]
pub mod omnifarming {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
