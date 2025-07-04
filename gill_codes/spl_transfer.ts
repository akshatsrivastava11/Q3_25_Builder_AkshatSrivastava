import { address, createSolanaClient, getSignatureFromTransaction, signTransactionMessageWithSigners } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { buildTransferTokensTransaction, TOKEN_2022_PROGRAM_ADDRESS } from "gill/programs";

const keypair=await loadKeypairSignerFromFile()
const mintAddress=address("E1vW2rvsPV3A58hcy9zdXXgbv4sSdBp7JKCPmne5wvkx")
const destinationAddress=address("HGeqJmtaZDUButfbMpG2DHSaPetMnCR7izEeQMsQ6Jrq")
const {rpc,sendAndConfirmTransaction}=createSolanaClient({urlOrMoniker:"devnet"});
try {
    const {value:latestBlockhash}=await rpc.getLatestBlockhash({commitment:"finalized"}).send()
    const  tx=await buildTransferTokensTransaction({
        amount:100,
        authority:keypair.address,
        destination:destinationAddress,
        feePayer:keypair,
        mint:mintAddress,
        latestBlockhash:latestBlockhash,
        tokenProgram:TOKEN_2022_PROGRAM_ADDRESS
    })
    const signedTx=await signTransactionMessageWithSigners(tx)
    const signature=await getSignatureFromTransaction(signedTx)
    console.log(`The transactions signature ${signature}`)
    await sendAndConfirmTransaction(signedTx);
    console.log("Transaction completed")
} catch (error) {
    console.log(`An error occured ${error}`)
}