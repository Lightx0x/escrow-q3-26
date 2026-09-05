use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The escrow has expired")]
    EscrowExpired,
    #[msg("Invalid expiry time set")]
    InvalidExpiry,
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Mint A must be different from Mint B")]
    SameMint,
}
