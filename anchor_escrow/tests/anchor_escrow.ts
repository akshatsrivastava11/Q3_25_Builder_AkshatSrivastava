import * as anchor from "@coral-xyz/anchor";
import { Program,BN } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { clusterApiUrl, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { createAssociatedTokenAccount, createMint, decodeSetTransferFeeInstruction, getAccount, getAssociatedTokenAddress, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { config } from "chai";
// import { BN } from "bn.js";
import {randomBytes} from 'crypto'
describe("anchor-escrow",()=>{
  const provider=anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program=anchor.workspace.anchor_escrow as Program<AnchorEscrow>;
  //one maker one taker maker makes the offer for mint_a as deposit  taker takes the
  //offer give the maker mint_b and taker's mint_A from the vault is given to the maker 
  const maker=anchor.web3.Keypair.generate()
  const taker=anchor.web3.Keypair.generate()
  let mintX:PublicKey
  let mintY:PublicKey
  let makerX:PublicKey
  let makerY:PublicKey
  let takerX:PublicKey
  let takerY:PublicKey
  let escrow:PublicKey
  let escrowBump:number
  let vaultX:PublicKey
  const amountToBeRecieved=1e6;
  const amountToBeDeposited=1e6;
  // const seed=new BN(randomBytes(8));
  before(async()=>{
    //airdropping both the accounts
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(maker.publicKey,2*LAMPORTS_PER_SOL)
    )
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(taker.publicKey,2*LAMPORTS_PER_SOL)
    )


    //creating the mints
    mintX=await createMint(provider.connection,maker,maker.publicKey,null,6)
    mintY=await createMint(provider.connection,taker,taker.publicKey,null,6)

    //creating the escrow 
    let [escrowPda,bump]=PublicKey.findProgramAddressSync(
     [ Buffer.from("escrow"),
      new BN(1).toArrayLike(Buffer,"le",8),
      maker.publicKey.toBuffer()
    ],
    program.programId
  );
  escrow=escrowPda
  escrowBump=bump

  //creating the ata
  makerX=(await getOrCreateAssociatedTokenAccount(provider.connection,maker,mintX,maker.publicKey)).address;
  takerY=(await getOrCreateAssociatedTokenAccount(provider.connection,taker,mintY,taker.publicKey)).address;
  //ata for vault_x     TypeError: src.toArrayLike is not a function
  vaultX=(await getOrCreateAssociatedTokenAccount(provider.connection,maker,mintX,escrow,true)).address

  //minto makerX and takerY 
  await mintTo(provider.connection,maker,mintX,makerX,maker,2e6);
  await mintTo(provider.connection,taker,mintY,takerY,taker,2e6)

  })
  it("make",async()=>{
    await program.methods.make(
      new BN(amountToBeRecieved),new BN(amountToBeDeposited),new BN(1))
    .accounts({
      maker:maker.publicKey,
      mintX,
      mintY,
      escrow:escrow,
      vaultX,
      makerY,
      makerX,
      systemProgram:SystemProgram.programId,
      tokenProgram:TOKEN_PROGRAM_ID,
      associatedTokenProgram:ASSOCIATED_PROGRAM_ID
    }).signers([maker]).rpc()
  })


})