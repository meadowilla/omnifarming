use anchor_lang::prelude::*;

#[error_code]
pub enum OmniFarmingError {
    #[msg("Amount below minimum deposit.")]
    DepositAmountTooLow,

    #[msg("Shares amount too small.")]
    DepositSharesTooLow,

    #[msg("Arithmetic overflow.")]
    Overflow,
}