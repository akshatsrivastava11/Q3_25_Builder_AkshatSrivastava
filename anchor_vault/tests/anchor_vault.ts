import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { assert } from "chai";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider=anchor.AnchorProvider.env()
  const program = anchor.workspace.anchorVault as Program<AnchorVault>;
  const user=anchor.AnchorProvider.env().wallet;

  let vaultStatePda: anchor.web3.PublicKey;
  let vaultPda: anchor.web3.PublicKey;
  let bumpState: number;
  let bumpVault: number;

  const LAMPORTS = 1_000_000; // 0.001 SOL

  before(async () => {
    // Derive PDAs
    [vaultStatePda, bumpState] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("state"), user.publicKey.toBuffer()],
      program.programId
    );

    [vaultPda, bumpVault] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vault"), user.publicKey.toBuffer()],
      program.programId
    );
  });
 it("Initialize vault", async () => {
    await program.methods.initialize().accounts({
      user: user.publicKey,
      vaultState: vaultStatePda,
      vault: vaultPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    const state = await program.account.vaultState.fetch(vaultStatePda);
    assert.equal(state.bumpState, bumpState);
    assert.equal(state.bumpVault, bumpVault);
  });

  it("Deposit lamports", async () => {
    const tx = await program.methods.deposit(new anchor.BN(LAMPORTS)).accounts({
      user: user.publicKey,
      vaultState: vaultStatePda,
      vault: vaultPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(vaultBalance, LAMPORTS);
  });

  it("Withdraw lamports", async () => {
    const preUserBalance = await provider.connection.getBalance(user.publicKey);

    const tx = await program.methods.withdraw(new anchor.BN(LAMPORTS)).accounts({
      user: user.publicKey,
      vaultState: vaultStatePda,
      vault: vaultPda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    const vaultBalance = await provider.connection.getBalance(vaultPda);
    assert.equal(vaultBalance, 0);

    const postUserBalance = await provider.connection.getBalance(user.publicKey);
    assert.ok(postUserBalance > preUserBalance - 10000); // subtract fees
  });
});

