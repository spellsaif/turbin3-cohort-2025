import {Keypair, Connection, LAMPORTS_PER_SOL} from "@solana/web3.js";

import wallet from "./private-key.json";

//importing keypair using private key
const keypair = Keypair.fromSecretKey(Uint8Array.from(wallet));

//Connecting to Solana Devnet
const connection = new Connection("https://api.devnet.solana.com");

//Airdrop 2 SOl to the wallet
(async () => {

    try {

        const txHash = await connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL * 2);

        console.log(`Success! Transaction Hash:  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
        
    } catch (error) {
        console.error("Oops! Something went wrong", error);
    }
})();