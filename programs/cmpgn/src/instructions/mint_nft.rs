use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginAuthorityPair},
    ID as CORE_PROGRAM_ID,
};

use crate::{error::ErrorCode, state::CollectionAuthority, Campaign};

#[derive(Accounts)]
#[instruction(bug_id: u8, name: String, nft_uri: String)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,

    #[account(
        mut,
        constraint = asset.data_is_empty() @ ErrorCode::AssetAlreadyInitialized
    )]
    pub asset: Signer<'info>,

    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @ ErrorCode::InvalidCollection,
        constraint = !collection.data_is_empty() @ ErrorCode::CollectionNotInitialized,
        constraint = collection.key() == collection_authority.collection @ ErrorCode::InvalidCollection
    )]
    /// CHECK: This will also be checked by core
    pub collection: UncheckedAccount<'info>,

    #[account(
        seeds = [b"collection", collection.key().as_ref()],
        bump,
    )]
    pub collection_authority: Box<Account<'info, CollectionAuthority>>,

    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: This will also be checked by core
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> MintNft<'info> {
    pub fn mint_nft(&mut self, bug_id: u8, name: String, nft_uri: String) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection",
            &self.collection.key().to_bytes(),
            &[self.collection_authority.bump],
        ]];

        let current_timestamp = Clock::get()?.unix_timestamp;

        CreateV2CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.collection_authority.to_account_info()))
            .payer(&self.minter.to_account_info())
            .owner(Some(&self.minter.to_account_info()))
            .update_authority(None)
            .system_program(&self.system_program.to_account_info())
            .name(name)
            .uri(nft_uri)
            .plugins(vec![PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute {
                            key: "Creator".to_string(),
                            value: self.collection_authority.creator.to_string(),
                        },
                        Attribute {
                            key: "Minter".to_string(),
                            value: self.minter.key().to_string(),
                        },
                        Attribute {
                            key: "Collection".to_string(),
                            value: self.collection.key().to_string(),
                        },
                        Attribute {
                            key: "Mint Timestamp".to_string(),
                            value: current_timestamp.to_string(),
                        },
                        Attribute {
                            key: "Bug ID".to_string(),
                            value: bug_id.to_string(),
                        },
                    ],
                }),
                authority: None,
            }])
            .external_plugin_adapters(vec![])
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
}

pub fn handler(ctx: Context<MintNft>, bug_id: u8, name: String, nft_uri: String) -> Result<()> {
    ctx.accounts.mint_nft(bug_id, name, nft_uri)?;

    Ok(())
}
