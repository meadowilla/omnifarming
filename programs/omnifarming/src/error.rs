use anchor_lang::prelude::*;

#[error_code]
pub enum OmniFarmingError {
    #[msg("Amount below minimum deposit.")]
    DepositAmountTooLow,

    #[msg("Shares amount too small.")]
    DepositSharesTooLow,

    #[msg("Amount below minimum withdrawal.")]
    WithdrawAmountTooLow,

    #[msg("Processing withdrawal, please wait until the current withdrawal is processed.")]
    ProcessingWithdrawal,

    #[msg("Insufficient shares for the requested amount.")]
    InefficientShares,

    #[msg("Arithmetic overflow.")]
    Overflow,
}