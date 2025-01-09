use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file}, 
    pubkey::Pubkey,
};

use bs58;
use std::io::{self, BufRead};
use solana_client::rpc_client::RpcClient; 



//SOLANA DEVNET URL
const RPC_URL: &str = "https://api.devnet.solana.com";

//LAMPORTS_PER_SOL
const LAMPORTS_PER_SOL: u64 = 2_000_000_000; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("\nYour wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:"); let stdin = io::stdin(); let
        wallet = stdin.lock()
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

        match client.request_airdrop(&keypair.pubkey(), LAMPORTS_PER_SOL) {

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
    }
}

