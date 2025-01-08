import {Keypair} from "@solana/web3.js";

//creating a new keypair (Solana Wallet)
const keypair = Keypair.generate();

//Displaying the public key
console.log(`Solana Wallet: ${keypair.publicKey.toBase58()}`)


//Solana Wallet: Private Key
console.log(`Wallet Private Key: [${keypair.secretKey}]`)

