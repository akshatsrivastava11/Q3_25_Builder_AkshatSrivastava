import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorAmm } from "../target/types/anchor_amm";
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import {createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID} from '@solana/spl-token'
import { BN } from "bn.js";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
describe("anchor-amm",()=>{
  let provider=anchor.AnchorProvider.env()
  let program=anchor.workspace.AnchorAmm as Program<AnchorAmm>
  let connection=provider.connection;
  let depositer=Keypair.generate()
  
  let mintX:PublicKey
  let mintY:PublicKey
  let vaultX:PublicKey
  let vaultY:PublicKey
  let config:PublicKey
  let mintLp:PublicKey
  let userX:PublicKey
  let userY:PublicKey
  let depositerLp:PublicKey;
  let amount_lp=1e5
  let maxX=1e6
  let maxY=1e6
  let amountIn=0.5e6
  let minOut=0.2e6
  let isXIn=true
  before(async()=>{
    //airdropping sol
await provider.connection.confirmTransaction(
  await provider.connection.requestAirdrop(depositer.publicKey, 2 * LAMPORTS_PER_SOL),
  "confirmed"
);

    //minting x and y
    mintX=await createMint(connection,depositer,depositer.publicKey,null,6);
    mintY=await createMint(connection,depositer,depositer.publicKey,null,6);

    //transfering tokens to vault_x and vault_y
    vaultX=(await getOrCreateAssociatedTokenAccount(provider.connection,depositer,mintX,depositer.publicKey,true)).address;
    vaultY=(await getOrCreateAssociatedTokenAccount(provider.connection,depositer,mintY,depositer.publicKey,true)).address;
  //user x and y
  userX=(await getOrCreateAssociatedTokenAccount(provider.connection,depositer,mintX,depositer.publicKey,true)).address;
  userY=(await getOrCreateAssociatedTokenAccount(provider.connection,depositer,mintY,depositer.publicKey,true)).address;



    //mintto vault_x and y
    await mintTo(provider.connection,depositer,mintX,userX,depositer.publicKey,2e6);
    await mintTo(provider.connection,depositer,mintY,userY,depositer.publicKey,2e6);
    //seeds=[b"config",seed.to_le_bytes().as_ref(),depositer.key().as_ref()]
    config=PublicKey.findProgramAddressSync(
      [
        Buffer.from("config"),
        new BN(1).toArrayLike(Buffer,"le",8),
        depositer.publicKey.toBuffer()
      ],
      program.programId
    )[0]
    
    //seeds=[b"lp",config.key().as_ref()],
    mintLp=PublicKey.findProgramAddressSync(
      [
        Buffer.from("lp"),config.toBuffer()
      ],
      program.programId
    )[0];
    console.log("Minted",userX,userY)
depositerLp = getAssociatedTokenAddressSync(mintLp, mintLp, true); // `true` for allowOwnerOffCurve  })
  })
  //creating depositer lp

  it("initilize",async()=>{
    await program.methods.initialize(new BN(1),2)
    .accounts({
      depositer:depositer.publicKey,
      mintX,
      mintY,
      config,
      mintLp,
      vaultX,
      vaultY,
      systemProgram:SystemProgram.programId,
      tokenProgram:TOKEN_PROGRAM_ID,
      associatedTokenProgram:ASSOCIATED_PROGRAM_ID
    });
  })
  it("deposit",async()=>{
    await program.methods.deposit(new BN(amount_lp),new BN(maxX),new  BN(maxY))
    .accounts(
      {
         depositer:depositer.publicKey,
      mintX,
      mintY,
      config,
      mintLp,
      depositerLp,
      vaultX,
      vaultY,
      userX,
      userY,
      systemProgram:SystemProgram.programId,
      tokenProgram:TOKEN_PROGRAM_ID,
      associatedTokenProgram:ASSOCIATED_PROGRAM_ID 
      }
    )
  })
  it("withdraw",async()=>{
    await program.methods.withdraw(new BN(amount_lp),new BN(maxX),new  BN(maxY))
    .accounts(
      {
          depositer:depositer.publicKey,
      mintX,
      mintY,
      config,
      mintLp,
      depositerLp,
      vaultX,
      vaultY,
      userX,
      userY,
      systemProgram:SystemProgram.programId,
      tokenProgram:TOKEN_PROGRAM_ID,
      associatedTokenProgram:ASSOCIATED_PROGRAM_ID 
      }
    )
  })
  it("swap",async()=>{
    await program.methods.swap(amountIn,minOut,isXIn)
    .accounts(
      {
                  depositer:depositer.publicKey,
      mintX,
      mintY,
      config,
      mintLp,
      depositerLp,
      vaultX,
      vaultY,
      userX,
      userY,
      systemProgram:SystemProgram.programId,
      tokenProgram:TOKEN_PROGRAM_ID,
      associatedTokenProgram:ASSOCIATED_PROGRAM_ID 
      }
    )
  })
})