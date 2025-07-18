import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN, min } from "bn.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {createMint, getOrCreateAssociatedTokenAccount} from '@solana/spl-token'
describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  let provider=anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  let program=anchor.workspace.nftStaking as Program<NftStaking>
  let admin=Keypair.generate()

  let user=Keypair.generate()
  let stakeConfig:PublicKey
  let rewardsMint:PublicKey
  let pointsPerStake=10;
  let maxAmountStaked=5;
  let fees=5;
  let freezePeriod=10;
  let userConfig:PublicKey

   let mint: PublicKey;
  let userMintAta: PublicKey;
  let metadata: PublicKey;
  let masterEdition: PublicKey;
  let metadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  let collectionMint:PublicKey;
  before(async()=>{
    //airdropping to the user and admin
      await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey,2*LAMPORTS_PER_SOL)
    )
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey,2*LAMPORTS_PER_SOL)
    )


    //stake config and rewards mint 
    stakeConfig=PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    )[0];
    rewardsMint=PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"),stakeConfig.toBuffer()],
      program.programId
    )[0]
    //user config
    userConfig=PublicKey.findProgramAddressSync(
      [Buffer.from("user"),stakeConfig.toBuffer(),user.publicKey.toBuffer()],
      program.programId
    )[0]


    //mint
        mint = await createMint(
      provider.connection,
      admin,
      user.publicKey,
      user.publicKey,
      0
    );
    //create's user's ata
    collectionMint=mint
    userMintAta=(await getOrCreateAssociatedTokenAccount(provider.connection,user,mint,user.publicKey)).address;

    metadata=await PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        mint.toBuffer()
      ],
      metadataProgram
    )[0];
    masterEdition=await PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        mint.toBuffer(),
        Buffer.from("master")
      ],
      metadataProgram
    )[0]

  })




  it("InitializeConfig",async()=>{
    await program.methods.initialize(new BN(pointsPerStake),maxAmountStaked,fees,freezePeriod).accounts(
     { 
      admin:admin.publicKey,
      stakeConfig:stakeConfig,
      rewardsMint:rewardsMint,
      systemProgram:SYSTEM_PROGRAM_ID,
      tokenProgram:TOKEN_PROGRAM_ID
    }
    ).signers([admin]).rpc()
      })
    it("initilializeUser",async()=>{
      await program.methods.initilializeUser().accounts(
        {
          user:user.publicKey,
          stakeConfig:stakeConfig,
          userConfig:userConfig,
          systemProgram:SYSTEM_PROGRAM_ID
        }
      ).signers([user]).rpc()
    })
    it("stake",async()=>{
      await program.methods.stake()
    })


});
