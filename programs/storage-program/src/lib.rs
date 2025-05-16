use anchor_lang::prelude::*;
use std::mem::size_of;
declare_id!("6ytMmvJR2YYsuPR7FSQUQnb7UGi1rf36BrXzZUNvKsnj");

#[program]
pub mod mappings {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, domain: u64, key: u64) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Set>, domain: u64, key: u64, value: u64) -> Result<()> {
        ctx.accounts.val.value = value;
        Ok(())
    }

    pub fn get(ctx: Context<Get>, domain: u64, key: u64) -> Result<u64> {
        Ok(ctx.accounts.val.value)
    }
}

#[derive(Accounts)]
#[instruction(domain: u64, key: u64)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Val>() + 8,
              seeds=[&domain.to_le_bytes().as_ref(), &key.to_le_bytes().as_ref()],
              bump)]
    val: Account<'info, Val>,
    
    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}

#[derive(Accounts)]
#[instruction(domain: u64, key: u64)]
pub struct Set<'info> {
    #[account(mut)]
    val: Account<'info, Val>,
}

#[derive(Accounts)]
#[instruction(domain: u64, key: u64)]
pub struct Get<'info> {
    val: Account<'info, Val>,
}