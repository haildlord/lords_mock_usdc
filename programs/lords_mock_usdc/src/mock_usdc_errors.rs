use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount must be > 0. Non-admins are limited to ≤ 10,000 USDC per transaction.")]
    InvalidMintAmount,
}