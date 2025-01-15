use std::io::{self, BufRead}; 
use bs58;  


#[test]
    fn base58_to_wallet() {
    println!("\n\n\nEnter Private Key(Base58):");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap(); 
    
    //gdtKSTXYULQNx87fdD3YgXkzVeyFeqwtxHm6WdEb5a9YJRnHse7GQr7t5pbepsyvUCk7VvksUGhPt4SZ8JHVSkt

    let wallet = bs58::decode(base58).into_vec().unwrap();

    println!("Wallet: {:?}\n\n\n", wallet);

    let back_to_base58 = bs58::encode(&wallet).into_string();
}


#[test]
fn wallet_to_base58() {
    let wallet: Vec<u8> = vec![34,46,55,124,141,190,24,204,134,91,70,184,161,181,44,122,15,172,63,62,153,150,99,255,202,89,105,77,41,89,253,130,27,195,134,14,66,75,242,7,132,234,160,203,109,195,116,251,144,44,28,56,231,114,50,131,185,168,138,61,35,98,78,53];
    let base58 = bs58::encode(wallet).into_string();

    println!("\n\n\nBase58 {:?}\n\n\n", base58);

}
