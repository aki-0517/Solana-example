import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateCoreAssetExample } from "../target/types/create_core_asset_example";
import { Keypair, SystemProgram } from "@solana/web3.js";
import { MPL_CORE_PROGRAM_ID } from "@metaplex-foundation/mpl-core";
import * as dotenv from "dotenv";

dotenv.config();

describe("create-core-asset-example", () => {
  // Anchor のプロバイダーを環境変数から設定
  anchor.setProvider(anchor.AnchorProvider.env());
  const wallet = anchor.Wallet.local();
  const program = anchor.workspace.CreateCoreAssetExample as Program<CreateCoreAssetExample>;

  // アセット用の新規 Keypair を生成
  let asset = Keypair.generate();

  it("Create Asset", async () => {
    // CreateAssetArgs の引数を定義
    const createAssetArgs = {
      name: "My Asset",
      uri: "https://example.com/my-asset.json",
    };

    // createCoreAsset メソッド呼び出し
    const tx = await program.methods.createCoreAsset(createAssetArgs)
      .accountsPartial({
        asset: asset.publicKey,
        collection: null,
        authority: null,
        payer: wallet.publicKey,
        owner: null,
        updateAuthority: null,
        systemProgram: SystemProgram.programId,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
      })
      .signers([asset, wallet.payer])
      .rpc();

    console.log("Transaction signature:", tx);
  });
});
