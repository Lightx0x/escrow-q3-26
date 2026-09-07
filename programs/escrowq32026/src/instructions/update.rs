use crate::{error::ErrorCode, Escrow, ESCROW_SEED};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [ESCROW_SEED, maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        has_one = maker,
        bump = escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
}

impl<'info> Update<'info> {
    pub fn update(&mut self, expiration: i64, receive: u64) -> Result<()> {
        if receive != 0 {
            self.escrow.receive = receive;
        }

        if expiration != 0 {
            require!(
                expiration > Clock::get()?.unix_timestamp,
                ErrorCode::InvalidExpiry
            );
            self.escrow.expiration = expiration;
        }

        Ok(())
    }
}
