use anchor_lang::prelude::*;

use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::BaseCollectionV1,
    instructions::CreateV2CpiBuilder,
    types::{
        Plugin, FreezeDelegate, PluginAuthority, PluginAuthorityPair,
        ExternalPluginAdapterInitInfo, AppDataInitInfo,
        ExternalPluginAdapterSchema
    }
};

declare_id!("G8sAEnDs3GjHqQEQyLzDCrdbE4nY4LAXHh6ibRfFzmHx");

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
}

#[program]
pub mod create_core_asset_example {
    use super::*;

    pub fn create_core_asset(ctx: Context<CreateAsset>, args: CreateAssetArgs) -> Result<()> {
        // オプションアカウントは Some / None を確認してから AccountInfo に変換
        let collection = ctx.accounts.collection.as_ref().map(|c| c.to_account_info());
        let authority = ctx.accounts.authority.as_ref().map(|a| a.to_account_info());
        let owner = ctx.accounts.owner.as_ref().map(|o| o.to_account_info());
        let update_authority = ctx.accounts.update_authority.as_ref().map(|u| u.to_account_info());

        // プラグインを利用する場合の例（必要に応じてプラグインの設定を変更）
        let mut plugins: Vec<PluginAuthorityPair> = vec![];
        plugins.push(
            PluginAuthorityPair { 
                plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }), 
                authority: Some(PluginAuthority::UpdateAuthority) 
            }
        );

        // 外部プラグインアダプタの例（必要な場合のみ追加。data_authority の値は適宜設定してください）
        let mut external_plugin_adapters: Vec<ExternalPluginAdapterInitInfo> = vec![];
        // external_plugin_adapters.push(
        //   ExternalPluginAdapterInitInfo::AppData(
        //     AppDataInitInfo {
        //       init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
        //       data_authority: PluginAuthority::Address { address: /* data_authority の PublicKey を指定 */ },
        //       schema: Some(ExternalPluginAdapterSchema::Binary),
        //     }
        //   )
        // );

        // CreateV2CpiBuilder を利用してコアアセットの作成を実行
        CreateV2CpiBuilder::new(&ctx.accounts.mpl_core_program.to_account_info())
            .asset(&ctx.accounts.asset.to_account_info())
            .collection(collection.as_ref())
            .authority(authority.as_ref())
            .payer(&ctx.accounts.payer.to_account_info())
            .owner(owner.as_ref())
            .update_authority(update_authority.as_ref())
            .system_program(&ctx.accounts.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(plugins)
            .external_plugin_adapters(external_plugin_adapters)
            .invoke()?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(mut)]
    pub asset: Signer<'info>,
    // collection を UncheckedAccount に変更
    #[account(mut)]
    pub collection: Option<UncheckedAccount<'info>>,
    pub authority: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: このアカウントは mpl_core プログラム側で検証されます
    pub owner: Option<UncheckedAccount<'info>>,
    /// CHECK: このアカウントは mpl_core プログラム側で検証されます
    pub update_authority: Option<UncheckedAccount<'info>>,
    pub system_program: Program<'info, System>,
    #[account(address = MPL_CORE_ID)]
    /// CHECK: アドレス制約により検証されます
    pub mpl_core_program: UncheckedAccount<'info>,
}

