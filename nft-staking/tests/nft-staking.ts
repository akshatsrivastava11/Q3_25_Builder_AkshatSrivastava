import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
// import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction } from "@solana/web3.js";
import { BN, min } from "bn.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { ASSOCIATED_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createSignerFromKeypair, generateSigner, keypairIdentity, percentAmount, publicKey } from "@metaplex-foundation/umi";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from "@metaplex-foundation/mpl-token-metadata";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
let metadataProgram = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
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
  let freezePeriod=0;
  let userConfig:PublicKey
  // const umi=createUmi(provider.connection.rpcEndpoint)
  // const keypair=createSignerFromKeypair(user.publicKey)

  const umi=createUmi(provider.connection)
  const mint=generateSigner(umi);
  const collectionMint=generateSigner(umi)
  const creatorWallet=umi.eddsa.createKeypairFromSecretKey(provider.wallet.payer.secretKey);
  const creator=createSignerFromKeypair(umi,creatorWallet)
  let rewardsAtaUser:PublicKey;


  let stakeAccount:PublicKey;
  let userMintAta: PublicKey;
  // let metadata: PublicKey;          // seeds=[b"marketplace",name.as_bytes()],
//        seeds=[b"treasury",marketplace.key().as_ref()],
//        seeds=[b"rewards",marketplace.key().as_ref()],
  // let masterEdition: PublicKey;
  before(async()=>{
        umi.use(keypairIdentity(creator));
    umi.use(mplTokenMetadata());
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

    // create collection
    await createNft(umi,{
      mint:collectionMint,
      name:"Ns",
      symbol:"NS",
      uri:"https://arweave.net/123",
      sellerFeeBasisPoints:percentAmount(0.5),
      collectionDetails:{__kind:"V1",size:10}
    }).sendAndConfirm(umi)
    console.log("created nft collection")

    //mint nft
    await createNft(umi,{
      mint:mint,
      name:"Ns",
      symbol:"NS",
      uri:"https://arweave.net/123",
      sellerFeeBasisPoints:percentAmount(0.5),
      collection:{verified:false,key:collectionMint.publicKey},
      tokenOwner:publicKey(user.publicKey)

    }).sendAndConfirm(umi)
    console.log("created nft mint")
    //verify collection

    const collectionMetadata=findMetadataPda(umi,{mint:collectionMint.publicKey});
    const collectionMasterEdition=findMasterEditionPda(umi,{mint:collectionMint.publicKey})
    const nftmetadata=findMetadataPda(umi,{mint:mint.publicKey})
    await verifySizedCollectionItem(
      umi,
      {
        metadata:nftmetadata,
        collection:collectionMetadata,
        collectionAuthority:creator,
        collectionMint:collectionMint.publicKey,
        collectionMasterEditionAccount:collectionMasterEdition,

      }
    ).sendAndConfirm(umi)
    console.log("collection nft verified")
    //user's nft ata
    userMintAta=(await getOrCreateAssociatedTokenAccount(provider.connection,user,new anchor.web3.PublicKey(mint.publicKey),user.publicKey)).address;

       stakeAccount=PublicKey.findProgramAddressSync(
      [Buffer.from("stake"),userConfig.toBuffer()],
      program.programId
    )[0];
    rewardsAtaUser=(await getOrCreateAssociatedTokenAccount(provider.connection,user,new anchor.web3.PublicKey(mint.publicKey),user.publicKey)).address;

    })


    //rewards ata


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
      const metadata=findMetadataPda(umi,{mint:mint.publicKey})
      const edition=findMasterEditionPda(umi,{mint:mint.publicKey})
      await program.methods.stake().accounts({
        user:user.publicKey,
        mint:mint.publicKey,
        userMintAta:userMintAta,
        collection:collectionMint.publicKey,
        metadata:new anchor.web3.PublicKey(metadata[0]),
        masterEdition:new anchor.web3.PublicKey(edition[0]),
        userConfig:userConfig,
        stakeConfig:stakeConfig,
        stakeAccount:stakeAccount,
        systemProgram:SYSTEM_PROGRAM_ID,
        tokenProgram:TOKEN_PROGRAM_ID,
        metadataProgram:metadataProgram
      }).signers([user]).rpc()
      const userConfigAccount = await program.account.userConfig.fetch(userConfig);
console.log("amounts_staked after stake:", userConfigAccount.amountsStaked.toString());
console.log("amounts_staked_points after stake: ",userConfigAccount.points.toString());

    })
    it("unstake",async()=>{

    //  const metadata=findMetadataPda(umi,{mint:mint.publicKey})
    // console.log('bump:');

      const edition=findMasterEditionPda(umi,{mint:mint.publicKey})
    //   await program.methods.stake().accounts({
    //     user:user.publicKey,
    //     mint:mint.publicKey,
    //     userMintAta:userMintAta,
    //     collection:collectionMint.publicKey,
    //     metadata:new anchor.web3.PublicKey(metadata[0]),
    //     masterEdition:new anchor.web3.PublicKey(edition[0]),
    //     userConfig:userConfig,
    //     stakeConfig:stakeConfig,
    //     stakeAccount:stakeAccount,
    //     systemProgram:SYSTEM_PROGRAM_ID,
    //     tokenProgram:TOKEN_PROGRAM_ID,
    //     metadataProgram:metadataProgram
    //   }).signers([user]).rpc()

      await program.methods.unstake().accounts({
          user:user.publicKey,
        mint:mint.publicKey,
        userMintAta:userMintAta,
        masterEdition:new anchor.web3.PublicKey(edition[0]),
        userConfig:userConfig,
        stakeConfig:stakeConfig,
        stakeAccount:stakeAccount,
        systemProgram:SYSTEM_PROGRAM_ID,
        tokenProgram:TOKEN_PROGRAM_ID,
        metadataProgram:metadataProgram
      }).signers([user]).rpc()
    })
    it("claim",async()=>{
      await program.methods.claim().accounts({
        user:user.publicKey,
        stakeConfig:stakeConfig,
        userConfig:userConfig,
        rewards:rewardsMint,
        rewardsAtaUser:rewardsAtaUser,
        systemProgram:SYSTEM_PROGRAM_ID,
        tokenProgram:TOKEN_PROGRAM_ID,
        associatedTokenProgram:ASSOCIATED_PROGRAM_ID
      })
    })
});

