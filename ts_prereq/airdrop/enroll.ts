import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import * as fs from 'fs'

const wallet = JSON.parse(fs.readFileSync("./Turbin3-wallet.json", {encoding:'utf-8'}))
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
});

// Create our program
const program: Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()];
const [account_key, account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintTs = Keypair.generate();

// Create the authority PDA (this was missing!)
const authority_seeds = [Buffer.from("collection"), mintCollection.toBuffer()];
const [authority_key, authority_bump] = PublicKey.findProgramAddressSync(authority_seeds, program.programId);

// Execute the initialize transaction
// (async () => {
//     try {
//         const txhash = await program.methods
//             .initialize("akshatsrivastava11")
//             .accountsPartial({
//                 user: keypair.publicKey,
//                 account: account_key,
//                 system_program: SystemProgram.programId,
//             })
//             .signers([keypair])
//             .rpc();
//         console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
//     } catch (e) {
//         console.error(`Oops, something went wrong: ${e}`);
//     }
// })();

// Execute the submitTs transaction
(async () => {
    try {
        const txhash = await program.methods
            .submitTs()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: authority_key,  // This was missing!
                mpl_core_program: MPL_CORE_PROGRAM_ID,
                system_program: SystemProgram.programId,
            })
            .signers([keypair, mintTs])
            .rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();