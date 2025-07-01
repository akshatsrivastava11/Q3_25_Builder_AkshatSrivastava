import { Transaction, SystemProgram, Connection, Keypair,
LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
"@solana/web3.js"
import * as fs from 'fs'
const wallet=JSON.parse(fs.readFileSync("./dev-wallet.json",{encoding:'utf-8'}))
const to= new PublicKey("DLkK7oCoDct33byQ3uiLfSVTZTM988EJDksjfte7UN6m");
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection=new Connection("https://api.devnet.solana.com ","confirmed");

(async ()=>{
    try {
        const balance=await connection.getBalance(keypair.publicKey)

        const txs=new Transaction().add(
            SystemProgram.transfer({
                fromPubkey:keypair.publicKey,
                toPubkey:to,
                lamports:balance
            })
        )
        txs.recentBlockhash=(await connection.getLatestBlockhash()).blockhash
        txs.feePayer=keypair.publicKey
        const fee = (await  
        connection.getFeeForMessage(txs.compileMessage(),
        'confirmed')).value 
        if(!fee) return
        txs.instructions.pop()
        txs.add(SystemProgram.transfer({
            fromPubkey:keypair.publicKey,
            toPubkey:to,
            lamports:balance-fee
        }))
        
        const signature=await sendAndConfirmTransaction(connection,txs,[keypair])
        console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (error) {
        console.log(`Oops Something went wrong ${error}`,error);
    }
})()