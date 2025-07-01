import { Keypair,Connection,LAMPORTS_PER_SOL } from "@solana/web3.js";
// import wallet from "./dev-wallet.json"
import * as fs from 'fs'
const wallet=JSON.parse(fs.readFileSync("./dev-wallet.json",{encoding:'utf-8'}))
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection=new Connection("https://api.devnet.solana.com ","confirmed");

(async()=>{
    try {

        const txhash=await connection.requestAirdrop(keypair.publicKey,LAMPORTS_PER_SOL*2);
        console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        
    } catch (error) {
        console.log(`Oops,something went wrong:${error}`)
    }
})()