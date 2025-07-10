import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { createAssociatedTokenAccount, createMint, getAccount, getAssociatedTokenAddress, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("anchor_escrow", () => {
  describe("anchor-escrow", () => {

    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);
    const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;

    //type  defn for the states
    let mintA: PublicKey;
    let mintB: PublicKey;
    let maker_ata_a: PublicKey;
    let maker_ata_b: PublicKey;
    let taker_ata_b: PublicKey;
    let taker_ata_a: PublicKey;
    let vault: PublicKey;
    let escrow: PublicKey;
    let seed = new anchor.BN(1);

    let maker = anchor.web3.Keypair.generate();
    let taker = anchor.web3.Keypair.generate();

    const depositAmount = new anchor.BN(50);
    const reciveamount = new anchor.BN(100);

    before(async () => {
      //airdropping maker and taker
      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(maker.publicKey, LAMPORTS_PER_SOL * 1),
        "confirmed"
      )

      await provider.connection.confirmTransaction(
        await provider.connection.requestAirdrop(taker.publicKey, LAMPORTS_PER_SOL * 1),
        "confirmed"
      )

      //creating tokenMints
      mintA = await createMint(provider.connection, maker, maker.publicKey, null, 6)
      mintB = await createMint(provider.connection, maker, taker.publicKey, null, 6)


      //creating associatedTokenAccounts
      maker_ata_a = await createAssociatedTokenAccount(provider.connection, maker, mintA, maker.publicKey)
      maker_ata_b = await createAssociatedTokenAccount(provider.connection, maker, mintB, maker.publicKey);
      taker_ata_a = await createAssociatedTokenAccount(provider.connection, taker, mintA, taker.publicKey);
      taker_ata_b = await createAssociatedTokenAccount(provider.connection, taker, mintB, taker.publicKey)

      //mint initial tokens
      await mintTo(provider.connection, maker, mintA, maker_ata_a, maker.publicKey, depositAmount.toNumber())
      await mintTo(provider.connection, taker, mintB, taker_ata_b, taker.publicKey, reciveamount.toNumber())

      //derive PDA
      const seedBuffer = seed.toArrayLike(Buffer, "le", 8);
      console.log("Seed as BN:", seed.toString());
      console.log("Seed buffer (hex):", seedBuffer.toString('hex'));
      console.log("Maker pubkey:", maker.publicKey.toString());

      const [derivedEscrow, bump] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("escrow"),
          maker.publicKey.toBuffer(),
        ],
        program.programId
      );

      escrow = derivedEscrow;
      console.log("Derived escrow PDA:", escrow.toString());
      console.log("Bump:", bump);

      //get Vault
      vault = await getAssociatedTokenAddress(
        mintA, escrow, true
      )


    })

    it("Make", async () => {
      const initialMakerAtaBalance = (await getAccount(provider.connection, maker_ata_a, "confirmed")).amount
      await program.methods.make(depositAmount, reciveamount)
        .accounts({
          maker: maker.publicKey,
          mintA: mintA,
          mintB: mintB,
          makerAtaMintA: maker_ata_a,
          escrow: escrow,
          vault: vault,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
        })
        .signers([maker])
        .rpc();

    })


  })
});