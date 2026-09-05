use anchor_lang::prelude::*;

use crate::{Escrow, ESCROW_SEED};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    pub taker: Signer<'info>,
}

impl<'info> Take<'info> {}
