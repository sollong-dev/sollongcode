use anchor_lang::prelude::*;
use crate::errors::SollongError;
use crate::instructions::OwOpInRound;
use crate::state::{Metadata, RoundStock};

pub fn set_merkle_tree_hash(ctx: Context<OwOpInRound>, new_list: [u8; 32]) -> Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;
    let metadata = &ctx.accounts.metadata;

    require!(ctx.accounts.user.key() == metadata.owner, SollongError::OwError);

    round_stock.merkle_root_hash = new_list;
    round_stock.whitelist_enabled = true;

    Ok(())
}

pub fn set_white_list_status(ctx: Context<OwOpInRound>, is_enabled: bool) -> Result<()> {
    let round_stock = &mut ctx.accounts.round_stock;
    let metadata = &ctx.accounts.metadata;

    require!(ctx.accounts.user.key() == metadata.owner, SollongError::OwError);
    require!(!round_stock.merkle_root_hash.iter().all(|&x|x==0), SollongError::MerkleIsEmptyError);

    round_stock.whitelist_enabled = is_enabled;

    Ok(())
}
