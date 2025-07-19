import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMarketplace } from "../target/types/anchor_marketplace";
import { PublicKey } from "@solana/web3.js";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
describe("anchor-marketplace", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider=anchor.AnchorProvider.env()
  const program = anchor.workspace.anchorMarketplace as Program<AnchorMarketplace>;
  let name="Akshat"
  let fees=1;
  //accounts

  let admin=anchor.web3.Keypair.generate();
  let treasury:PublicKey;
  let marketplace:PublicKey;
  let rewards:PublicKey
  before(async()=>{
    marketplace=PublicKey.findProgramAddressSync(
      [Buffer.from("marketplace"),Buffer.from(name,'utf-8')],
      program.programId
    )[0]
    treasury=PublicKey.findProgramAddressSync(
      [Buffer.from("treasury"),marketplace.toBuffer()],
      program.programId
    )[0];
    rewards=PublicKey.findProgramAddressSync(
      [Buffer.from("rewards"),marketplace.toBuffer()],
      program.programId
    )[0];


  })
  it("initialize",async()=>{
    await program.methods.initialize(fees,name).accounts({
      admin:admin.publicKey,
    treasury:treasury,
      marketplace,
      rewards,
      systemProgram:SYSTEM_PROGRAM_ID,
      tokenProgram:TOKEN_PROGRAM_ID
    }).signers([admin]).rpc()
  })
});
