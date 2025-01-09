mod programs;

use crate::programs::turbin3_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs}; 

use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file}, 
    pubkey::Pubkey,
    message::Message,
    transaction::Transaction

};

use bs58;
use std::io::{self, BufRead};
use solana_client::rpc_client::RpcClient; 

use std::str::FromStr;

use solana_program::{
    system_instruction::transfer,
    system_program
};


//SOLANA DEVNET URL
const RPC_URL: &str = "https://api.devnet.solana.com";

//LAMPORTS_PER_SOL
const LAMPORTS_PER_SOL: u64 = 1_000_000_000; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
        println!("\nYour wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); 
        let stdin = io::stdin(); 
        let wallet = stdin.lock()
                .lines()
                .next()
                .unwrap()
                .unwrap()
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',') 
                .map(|s| s.trim().parse::<u8>().unwrap())
                .collect::<Vec<u8>>();

        println!("\nYour private key is:");
        let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);
    
    }

    #[test]
    fn keygen() {
        // Test implementation for keygen
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string()); 
        println!("\n");
        println!("To save your wallet, copy and paste the following into a JSON file:");

        println!("{:?}", kp.to_bytes());

    }

    #[test]
    fn airdrop() {
        // Test implementation for airdrop
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file"); 
        
        // Connected to Solana Devnet RPC Client
        let client = RpcClient::new(RPC_URL); 

        match client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL*2) {

            Ok(s) => {
                println!("Success! Check out your TX here:\n");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            },

            Err(e) => println!("Oops, something went wrong: {}", e.to_string())
        };


    }

    #[test]
    fn transfer_sol() {
        // Test implementation for transfer_sol
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        
        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("AsAduBWNpjJXvW2mN1PXKM1CuHeNYapQQ2VCjZpq9Hbq").unwrap(); 
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
                                .get_latest_blockhash() 
                                .expect("Failed to get recent blockhash");

        
        //Transferring 0.1 sol from dev wallet to turbin3-wallet-address
        let transaction = Transaction::new_signed_with_payer(
                            &[transfer(
                                &keypair.pubkey(), &to_pubkey, LAMPORTS_PER_SOL/10
                            )],
                            Some(&keypair.pubkey()), &vec![&keypair], 
                            recent_blockhash
                          );

        // Send the transaction
        let signature = rpc_client
                        .send_and_confirm_transaction(&transaction)
                        .expect("Failed to send transaction");

        
        // Print our transaction out 
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
                            
    }


    #[test]
    fn transfer_all_sol() {
        // Test implementation for transfer_sol
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");


        // Define our Turbin3 public key
        let to_pubkey = Pubkey::from_str("AsAduBWNpjJXvW2mN1PXKM1CuHeNYapQQ2VCjZpq9Hbq").unwrap(); 
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);


        // Get recent blockhash
        let recent_blockhash = rpc_client
                                .get_latest_blockhash() 
                                .expect("Failed to get recent blockhash");

        
        // Get balance of dev wallet
        let balance = rpc_client
                        .get_balance(&keypair.pubkey())
                        .expect("Failed to get balance");


         //Mock transaction for calculating fees
         let message = Message::new_with_blockhash(
                        &[transfer( &keypair.pubkey(), &to_pubkey, balance,)],
                        Some(&keypair.pubkey()), 
                        &recent_blockhash
                       );

        //getting fees
        let fee = rpc_client
                    .get_fee_for_message(&message) 
                    .expect("Failed to get fee calculator");
        
        //Transferring 0.1 sol from dev wallet to turbin3-wallet-address
        let transaction = Transaction::new_signed_with_payer(
                            &[transfer(
                                &keypair.pubkey(), &to_pubkey, 
                                balance-fee
                            )],
                            Some(&keypair.pubkey()), &vec![&keypair], 
                            recent_blockhash
                          );

        // Send the transaction
        let signature = rpc_client
                        .send_and_confirm_transaction(&transaction)
                        .expect("Failed to send transaction");

        
        // Print our transaction out 
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
                            
    }

    #[test]
    fn enroll() {
        
        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Let's define our accounts
        let signer = read_keypair_file("Turbin3-wallet.json").expect("Couldn't find wallet file");

        let prereq = WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

        // Define our instruction data 
        let args = CompleteArgs{github: b"spellsaif".to_vec() };

        // Get recent blockhash
        let blockhash = rpc_client 
                            .get_latest_blockhash() 
                            .expect("Failed to get recentblockhash");

        
        // Now we can invoke the "complete" function let transaction =
        let transaction = WbaPrereqProgram::complete(
                            &[&signer.pubkey(), &prereq, &system_program::id()], &args, Some(&signer.pubkey()),
                            &[&signer],
                            blockhash 
                          );


        // Send the transaction
        let signature = rpc_client 
                            .send_and_confirm_transaction(&transaction) 
                            .expect("Failed to send transaction");


        // Print our transaction out 
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
    

        // https://explorer.solana.com/tx/3E61aKKBAQCewpJgEDxRtqHLh8EzSwNFUiPyqhuw76cWUds9L26NzNPP21kDPpAfNfzCPswbwLbw7LTFPD33p4Dk?cluster=devnet                  
        
     }

   

}

