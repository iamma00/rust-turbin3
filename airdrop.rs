use solana_client::rpc_client::RpcClient;
use solana_program::{pubkey::Pubkey, system_instruction::transfer};
use solana_sdk::{
    signature::{read_keypair_file, Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

// Define the RPC URL constant
const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();

        // Print the public key (Solana address)
        println!(
            "You've generated a new Solana wallet address: {}",
            kp.pubkey().to_string()
        );

        // Print the private key (in bytes)
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
    fn airdrop() {
        // 1. Read keypair (private key) from the saved JSON wallet file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // 2. Create a connection to the Solana Devnet
        let client = RpcClient::new(RPC_URL);

        // 3. Request an airdrop of 2 SOL (2,000,000,000 lamports)
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(signature) => {
                println!("Airdrop successful! Check your transaction here:");
                println!(
                    "https://explorer.solana.com/tx/{}?cluster=devnet",
                    signature.to_string()
                );
            }
            Err(e) => println!("Airdrop failed: {}", e.to_string()),
        }
    }

    #[test]
    fn transfer_sol() {
        // 1. Read keypair from the saved JSON wallet file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // 2. Create a connection to the Solana Devnet
        let client = RpcClient::new(RPC_URL);

        // 3. Define your Turbin3 public key
        let to_pubkey = Pubkey::from_str("FqaiW9B3EPtwXjqZbWjoESf3SfooJgRWtgodmn3Dg7Mv").unwrap();

        // 4. Get the recent blockhash
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // 5. Create a transaction to transfer 0.001 SOL (1,000,000 lamports)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)], // 0.001 SOL
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // 6. Send the transaction
        match client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => println!("Transfer successful! Check your transaction here: https://explorer.solana.com/tx/{}?cluster=devnet", signature.to_string()),
            Err(e) => println!("Transfer failed: {}", e.to_string())
        }
    }

    #[test]
    fn empty_wallet() {
        // 1. Read keypair from the saved JSON wallet file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // 2. Create a connection to the Solana Devnet
        let client = RpcClient::new(RPC_URL);

        // 3. Define your Turbin3 public key
        let to_pubkey = Pubkey::from_str("FqaiW9B3EPtwXjqZbWjoESf3SfooJgRWtgodmn3Dg7Mv").unwrap();

        // 4. Get the balance of the dev wallet
        let balance = client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // 5. Get the recent blockhash
        let recent_blockhash = client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // 6. Create a mock transaction to calculate the fee
        let message = solana_sdk::message::Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        // 7. Get the fee for the transaction
        let fee = client
            .get_fee_for_message(&message)
            .expect("Failed to get fee");

        // 8. Create a transaction to transfer all remaining SOL (balance - fee)
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        // 9. Send the transaction
        match client.send_and_confirm_transaction(&transaction) {
            Ok(signature) => println!("Wallet emptied! Check your transaction here: https://explorer.solana.com/tx/{}?cluster=devnet", signature.to_string()),
            Err(e) => println!("Transaction failed: {}", e.to_string())
        }
    }
}
