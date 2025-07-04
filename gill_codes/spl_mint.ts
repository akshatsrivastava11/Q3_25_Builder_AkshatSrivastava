import { address, createSolanaClient, getSignatureFromTransaction, sendAndConfirmTransactionFactory, signTransactionMessageWithSigners } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { buildMintTokensTransaction, getAssociatedTokenAccountAddress, TOKEN_2022_PROGRAM_ADDRESS } from "gill/programs";


const mintAddress=address("E1vW2rvsPV3A58hcy9zdXXgbv4sSdBp7JKCPmne5wvkx")
const keypair=await loadKeypairSignerFromFile();
const {rpc,sendAndConfirmTransaction}=await createSolanaClient({urlOrMoniker:"devnet"})
try {


    const {value:latestBlockhash}=await rpc.getLatestBlockhash().send()
    const tx=await buildMintTokensTransaction({
        amount:100000,
        destination:keypair.address,
        feePayer:keypair,
        mint:mintAddress,
        mintAuthority:keypair.address,
        latestBlockhash,
        tokenProgram:TOKEN_2022_PROGRAM_ADDRESS,
    })
    const signedTx=await signTransactionMessageWithSigners(tx)
    const signature=await getSignatureFromTransaction(signedTx) 
    console.log(`The signature for the transaction is ${signature}`)
    await sendAndConfirmTransaction(signedTx)
    console.log(`Transactions done boiii`)
    
    
    const {value}=await rpc.getTokenAccountBalance(
        await getAssociatedTokenAccountAddress(mintAddress,keypair.address,TOKEN_2022_PROGRAM_ADDRESS)
    ).send()
    console.log(`final balance of the account for the mint is  !! ${value.amount}`)
} catch (error) {
    console.log(`An error occured ${error}`)
}