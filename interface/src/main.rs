extern crate libra_wallet;

use libra_wallet::{wallet_library::WalletLibrary};
use std::path::PathBuf;
use std::env;

const WALLET_DIRECTORY: &str = "/wallets/";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Return if no operation
    if(args.len() < 1)
    {
        return;
    }

    let func = &args[1];
    if(func == "create")
    {
        createAccount();
    }
    else
    {
        // More functions?
    }
}

// Generates new account and prints public key with mnemonic.
fn createAccount() {
    let mut wallet = WalletLibrary::new();

    let (address,_) = wallet.new_address().unwrap();
    println!("{}:{}",address,wallet.mnemonic());    
}

// Save wallet to a file with given name.
fn writeWallet(wallet:WalletLibrary,name: &str) {
    let mut file_path = std::env::current_dir().unwrap();
    file_path.push(WALLET_DIRECTORY);
    file_path.push(name);
    wallet.write_recovery(&file_path);
}