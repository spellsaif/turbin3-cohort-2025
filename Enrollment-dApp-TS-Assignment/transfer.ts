
import {Transaction, Connection, SystemProgram, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction} from "@solana/web3.js";

import wallet from "./private-key.json";

const from = Keypair.fromSecretKey(new Uint8Array(wallet));

//Our Turbin3 Wallet Address
const to = new PublicKey("AsAduBWNpjJXvW2mN1PXKM1CuHeNYapQQ2VCjZpq9Hbq");

//creating devnet connection
const connection = new Connection("https://api.devnet.solana.com/");

//new transaction
//Transferring 0.1 Sol

(async() => {

    try {

        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL / 10
            })
        );
    
        transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    
        transaction.feePayer = from.publicKey;

    
        // Sign transaction, broadcast, and confirm
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )

        console.log(`Success! Check out your TX here:
            https://explorer.solana.com/tx/${signature}?cluster=devnet`);
        
    } catch (error) {
        console.error("Oops! Something went wrong. \n", error);
    }

    

})();


