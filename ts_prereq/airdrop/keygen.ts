import { Keypair } from "@solana/web3.js";
const keypair=Keypair.generate()
console.log("keypair is ",keypair.publicKey.toBase58());
console.log(`${keypair.secretKey}`)