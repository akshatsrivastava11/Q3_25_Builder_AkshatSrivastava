import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorDice } from "../target/types/anchor_dice";
import { Ed25519Program, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SYSVAR_INSTRUCTIONS_PUBKEY, Transaction } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("anchor-dice", () => {
  // Configure the client to use the local cluster.
  const provider=anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  const program = anchor.workspace.anchorDice as Program<AnchorDice>;
  let house=Keypair.generate()
  let gambler=Keypair.generate()
  let vault:PublicKey;
  let bet:PublicKey
  let amountToBeDeposited=3*LAMPORTS_PER_SOL;
  let bet_amonut=1
  let roll=50
  let seed=1
  before(async()=>{
    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(house.publicKey,5*LAMPORTS_PER_SOL),"confirmed");

    await provider.connection.confirmTransaction(await provider.connection.requestAirdrop(gambler.publicKey,5*LAMPORTS_PER_SOL),"confirmed");

    vault=PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    )[0]

    bet=PublicKey.findProgramAddressSync(
      [Buffer.from("bet"),gambler.publicKey.toBuffer()],
      program.programId
    )[0]

  })
  it("Initialize",async()=>{
    // const amount=await provider.connection.getAccountInfo(house.publicKey);
    // console.log(amount.lamports/LAMPORTS_PER_SOL)
    await program.methods.initialize(new anchor.BN(amountToBeDeposited)).accounts({
      house:house.publicKey,
      vault:vault,
      systemProgram:SYSTEM_PROGRAM_ID
    }).signers([house]).rpc()
  })
  it("place_bet",async()=>{
    await program.methods.placeBet(new anchor.BN(bet_amonut),new anchor.BN(roll),seed).accounts({
      gambler:gambler.publicKey,
      vault:vault,
      bet:bet,
      systemProgram:SYSTEM_PROGRAM_ID
    }).signers([gambler]).rpc()
  })
  // it("refund_bet",async()=>{
  //   await program.methods.refundBet().accounts({
  //           gambler:gambler.publicKey,
  //     vault:vault,
  //     bet:bet,
  //     systemProgram:SYSTEM_PROGRAM_ID
  //   }).signers([gambler]).rpc()    
  // })
  it("resolve_bet",async()=>{
    let account=await anchor.getProvider().connection.getAccountInfo(bet,"confirmed");
    let sig_ix=Ed25519Program.createInstructionWithPrivateKey({
      privateKey:house.secretKey,
      message:account.data.subarray(8),
    })
    const resolve_ix=await program.methods.resolveBet(Buffer.from(sig_ix.data.buffer.slice(16+32,16+32+64))).accounts({
      house:house.publicKey,
      gambler:gambler.publicKey,
      bet:bet,
      vault:vault,
      systemProgram:SYSTEM_PROGRAM_ID,
      instructionSysvar:SYSVAR_INSTRUCTIONS_PUBKEY
    }).signers([house]).instruction();
    const tx=new Transaction().add(sig_ix).add(resolve_ix)
    await sendAndConfirmTransaction(program.provider.connection,tx,[house])
  })
});

  //  #[account(mut)]
  //   pub house:Signer<'info>,

  //   #[account(mut)]
  //   ///CHECK:This is safe
  //   pub gambler:UncheckedAccount<'info>,
  //   #[account(
  //       seeds=[b"bet",gambler.key().as_ref()],
  //       bump        
  //   )]
  //   pub bet:Account<'info,Bet>,
  //   #[account(
  //       mut,
  //       seeds=[b"vault"],
  //       bump
  //   )]
  //   pub vault:SystemAccount<'info>,
  //   pub system_program:Program<'info,System>,
  //   ///CHECK:This is safe
  //   pub instruction_sysvar:AccountInfo<'info>