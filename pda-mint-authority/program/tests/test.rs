// USING RPC DEVNET
use api::prelude::*;
use solana_program::program_pack::Pack;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
    signer::keypair::read_keypair_file
};
use spl_token::state::Mint;
use solana_program::program_option::COption;
use mpl_token_metadata::accounts::Metadata;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::error::Error;
use steel::*;
use std::env;

// Helper function to get PDA addresses (unchanged)
fn get_addresses(_mint_seed: &[u8]) -> (Pubkey, Pubkey, u8) {
    let (mint_pda, bump) = Pubkey::find_program_address(
        &[MintAuthorityPda::SEED_PREFIX.as_bytes()],  
        &api::ID
    );
    
    let (metadata_pda, _) = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            mint_pda.as_ref()
        ],
        &mpl_token_metadata::ID
    );
    
    (mint_pda, metadata_pda, bump)
}

// REQUEST AIRDROP OR VISIT SOLANA FAUCET
// async fn request_airdrop(client: &RpcClient, pubkey: &Pubkey, amount: u64) -> Result<(), Box<dyn Error>> {
//     let signature = client.request_airdrop(pubkey, amount).await?;
//     let commitment = CommitmentConfig::confirmed();
//     client.confirm_transaction_with_commitment(&signature, commitment).await?;
//     Ok(())
// }

async fn verify_program_deployment(client: &RpcClient, program_id: Pubkey) -> Result<(), Box<dyn Error>> {
    match client.get_account(&program_id).await {
        Ok(account) => {
            println!("Program found: {:?}", program_id);
            println!("Program owner: {:?}", account.owner);
            println!("Program executable: {}", account.executable);
            Ok(())
        }
        Err(err) => Err(format!("Program not found: {}", err).into())
    }
}

async fn debug_transaction(client: &RpcClient, tx: &Transaction) {
    println!("\n=== Transaction Debug Info ===");
    println!("Number of signatures: {}", tx.signatures.len());
    println!("Signing pubkeys: {:?}", tx.message.header.num_required_signatures);
    println!("Readonly signers: {:?}", tx.message.header.num_readonly_signed_accounts);
    println!("Readonly non-signers: {:?}", tx.message.header.num_readonly_unsigned_accounts);
    
    for (i, acc) in tx.message.account_keys.iter().enumerate() {
        if let Ok(account) = client.get_account(acc).await {
            println!("\nAccount #{}: {:?}", i, acc);
            println!("  Owner: {:?}", account.owner);
            println!("  Lamports: {}", account.lamports);
            println!("  Executable: {}", account.executable);
            println!("  Data len: {}", account.data.len());
            println!("  Is signer: {}", tx.message.is_signer(i));
            println!("  Is writable: {}", tx.message.is_writable(i));
        } else {
            println!("\nAccount #{}: {:?} (not found - will be created)", i, acc);
        }
    }
}

// Helper function to load keypair from file
fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    // Try to get keypair path from environment variable
    let keypair_path = env::var("KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", env::var("HOME").unwrap()));
    
    println!("Loading keypair from: {}", keypair_path);
    Ok(read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to load keypair: {}", e))?)
}

#[tokio::test]
async fn test_create_token() -> Result<(), Box<dyn Error>> {
    // Set up RpcClient with devnet URL using the nonblocking client
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    // Load wallet keypair
    let payer = load_keypair()?;
    println!("Using wallet address: {}", payer.pubkey());
    
    // Check wallet balance
    let balance = client.get_balance(&payer.pubkey()).await?;
    println!("Wallet balance: {} SOL", balance as f64 / 1_000_000_000.0);
    
    if balance < 1_000_000_000 {  // 1 SOL
        println!("Warning: Wallet balance is low. You may need more SOL for transactions.");
    }
    
    // Verify program deployments
    println!("Verifying program deployments...");
    println!("Verifying mpl_token_metadata::ID");
    verify_program_deployment(&client, mpl_token_metadata::ID).await?;
    println!("Verifying spl_token::ID");
    verify_program_deployment(&client, spl_token::id()).await?;
    println!("Verifying api::ID");
    verify_program_deployment(&client, api::ID).await?;
    println!("Verified all deployments");

    // Set up token info
    let token_name = "Test".to_string();
    let token_symbol = "TST".to_string();
    let token_uri = "https://test.json".to_string();
    
    let (mint_pda, metadata_pda, bump) = get_addresses(MintAuthorityPda::SEED_PREFIX.as_bytes());

    println!("\n=== PDA Info ===");
    println!("Mint PDA: {}", mint_pda);
    println!("Metadata PDA: {}", metadata_pda);
    println!("Bump: {}", bump);

    // Create token instruction
    let create_token_ix = create_token(
        payer.pubkey(),
        token_name.clone(),
        token_symbol.clone(),
        token_uri.clone()
    );

    // Set compute budget
    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);
    let set_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1);

    let recent_blockhash = client.get_latest_blockhash().await?;
    
    let tx = Transaction::new_signed_with_payer(
        &[
            compute_budget_ix,
            set_price_ix,
            create_token_ix
        ],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    // Debug transaction before sending
    debug_transaction(&client, &tx).await;

    // Send and confirm transaction
    let signature = client.send_and_confirm_transaction(&tx).await?;
    println!("Transaction signature: {}", signature);

    Ok(())
}

