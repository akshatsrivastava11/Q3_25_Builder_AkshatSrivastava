import { createSolanaClient, lamports, LAMPORTS_PER_SOL, SolanaClusterMoniker } from 'gill'
import {loadKeypairSignerFromFile} from 'gill/node'


try {    
    const keypair=await loadKeypairSignerFromFile()
    const cluster:SolanaClusterMoniker="devnet"
    const {rpc}=createSolanaClient({urlOrMoniker:cluster})
    const initialBalance=await rpc.getBalance(keypair.address,{commitment:"finalized"}).send()
    console.log(`The initial balance of account is ${(initialBalance.value)/lamports(BigInt(LAMPORTS_PER_SOL))}`)
    const tx=await rpc.requestAirdrop(keypair.address,lamports(BigInt(LAMPORTS_PER_SOL*3)),{commitment:"finalized"}).send()
    console.log(tx)
    const finalBalance=await rpc.getBalance(keypair.address,{commitment:"finalized"}).send()
    console.log(`The final balance of account is ${(finalBalance.value)/lamports(BigInt(LAMPORTS_PER_SOL))}`)
    
} catch (error) {
    console.log("Oops an error occured!!",error)
}