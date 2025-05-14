use anchor_lang::prelude::*;

declare_id!("AxvHNpUZDvuuPXaAzm6oUqx4zb68PVWCaMVxqZdbbgn5");

#[program]
pub mod storage_program {
    use std::ops::DerefMut;

    use super::*;

    pub fn initialize(ctx: Context<Initialise>) -> Result<()> {
        let storage = ctx.accounts.storage.deref_mut();
        let entries: Vec<StorageEntry> = Vec::new();

        *storage = Storage { storage: entries };

        Ok(())
    }

    pub fn set_value(ctx: Context<SetKey>, key: Vec<u8>, value: Vec<u8>) -> Result<()> {
        let storage = ctx.accounts.storage.deref_mut();
        if let Some(entry) = storage
            .storage
            .iter_mut()
            .find(|storage_item| storage_item.key == key)
        {
            entry.value = value;
        } else {
            storage.storage.push(StorageEntry { key, value });
        }

        return Ok(());
    }
}

#[derive(Accounts)]
pub struct Initialise<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(init, payer = authority, space = 64, seeds = [b"storage".as_ref()], bump)]
    pub storage: Account<'info, Storage>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct StorageEntry {
    key: Vec<u8>,
    value: Vec<u8>,
}

#[account]
pub struct Storage {
    storage: Vec<StorageEntry>,
}

#[derive(Accounts)]
pub struct SetKey<'info> {
    #[account(mut, seeds = [b"storage".as_ref()], bump)]
    pub storage: Account<'info, Storage>,
}
