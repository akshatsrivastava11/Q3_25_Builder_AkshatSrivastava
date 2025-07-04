import { containsBytes, createSolanaClient, generateKeyPairSigner, getSignatureFromTransaction, lamports, sendAndConfirmTransactionFactory, signAndSendTransactionMessageWithSigners, signTransactionMessageWithSigners, SolanaClusterMoniker } from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import { buildCreateTokenTransaction, TOKEN_2022_PROGRAM_ADDRESS } from "gill/programs";

//load keypair from file

//reading fromm the svm
//creating tx 
//signing txs

const keypair=await loadKeypairSignerFromFile()
console.log(`The signer's address is ${keypair.address}`)
const cluster:SolanaClusterMoniker="devnet";
const {rpc,sendAndConfirmTransaction}=createSolanaClient({urlOrMoniker:cluster})

try {
    const mint=await generateKeyPairSigner();
    console.log(`The mint address is ${mint.address}`)
    const {value:latestBlockhash}=await rpc.getLatestBlockhash().send()


    //the buildCreateToken is just  a high level abstraction over 
//     const tx = createTransaction({
//   feePayer: signer,
//   version: "legacy",
//   instructions: [
//     getCreateAccountInstruction({
//       space,
//       lamports: getMinimumBalanceForRentExemption(space),
//       newAccount: mint,
//       payer: signer,
//       programAddress: tokenProgram,
//     }),
//     getInitializeMintInstruction(
//       {
//         mint: mint.address,
//         mintAuthority: signer.address,
//         freezeAuthority: signer.address,
//         decimals: 9,
//       },
//       {
//         programAddress: tokenProgram,
//       },
//     ),
//     getCreateMetadataAccountV3Instruction({
//       collectionDetails: null,
//       isMutable: true,
//       updateAuthority: signer,
//       mint: mint.address,
//       metadata: metadataAddress,
//       mintAuthority: signer,
//       payer: signer,
//       data: {
//         sellerFeeBasisPoints: 0,
//         collection: null,
//         creators: null,
//         uses: null,
//         name: "super sweet token",
//         symbol: "SST",
//         uri: "https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/Climate/metadata.json",
//       },
//     }),
//   ],
//   latestBlockhash,
// });

    const tx=await buildCreateTokenTransaction({
        feePayer:keypair,
        metadata:{name:"RohitToken",symbol:"RS45",isMutable:true,uri:"https://raw.githubusercontent.com/solana-developers/opos-asset/main/assets/Climate/metadata.json"},
        mint:mint,
        decimals:6,
        tokenProgram:TOKEN_2022_PROGRAM_ADDRESS,
        latestBlockhash
    })
    const signedTx=await signTransactionMessageWithSigners(tx)
    const getSignature=await getSignatureFromTransaction(signedTx)
    await sendAndConfirmTransaction(signedTx,{commitment:"finalized"})
    console.log("The transaction will be as ",getSignature)
    

} catch (error) {
    console.log(`An error occured  ${error}`)
}