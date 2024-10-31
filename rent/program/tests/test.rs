use rent_api::prelude::*;
use rent_program::process_instruction;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, signature::{read_keypair_file, Keypair}, signer::Signer, transaction::Transaction
};
use std::{env, error::Error};
use steel::*;

fn load_keypair() -> Result<Keypair, Box<dyn Error>> {
    let keypair_path = env::var("KEYPAIR_PATH")
        .unwrap_or_else(|_| format!("{}/.config/solana/id.json", env::var("HOME").unwrap()));
    println!("🔑 Loading keypair from: {}", keypair_path);
    Ok(read_keypair_file(&keypair_path)
        .map_err(|e| format!("Failed to load keypair: {}", e))?)
}

async fn setup() -> (BanksClient, Keypair, Hash) {
    println!("🚀 Setting up test environment...");
    
    let mut program_test = ProgramTest::new(
        "rent_program",
        rent_api::ID,
        processor!(process_instruction),
    );
    
    let payer = load_keypair().expect("Failed to load keypair");
    println!("✅ Loaded payer wallet: {}", payer.pubkey());
    
    // Add initial balance for payer
    program_test.add_account(
        payer.pubkey(),
        solana_sdk::account::Account {
            lamports: 1000 * LAMPORTS_PER_SOL,
            owner: solana_sdk::system_program::ID,
            executable: false,
            rent_epoch: 0,
            data: vec![],
        },
    );
    println!("💰 Added initial balance to payer");
    
    println!("🔄 Starting program test...");
    let (banks, payer, hash) = program_test.start().await;
    println!("✅ Test environment setup complete");
    
    (banks, payer, hash)
}

#[tokio::test]
async fn test_create_system_account_string_too_long() {
    println!("\n🧪 Starting string length validation test");
    
    let (mut _banks, payer, _blockhash) = setup().await;
    let new_account = Keypair::new();
    
    // Create string longer than STRING_MAX_SIZE
    let long_string = "a".repeat(STRING_MAX_SIZE + 1);
    println!("📝 Attempting to create account with too long name");
    
    // This should return an error
    let result = create_system_account(
        payer.pubkey(),
        new_account.pubkey(),
        long_string.clone(),
        "Valid Address".to_string(),
    );
    
    assert!(result.is_err(), "Expected error for too long name");
    assert!(matches!(result.unwrap_err(), ProgramError::Custom(_)));
    println!("✅ Successfully caught string too long error");
}

#[tokio::test]
async fn test_create_system_account_insufficient_funds() {
    println!("\n🧪 Starting insufficient funds test");
    
    let (mut banks, _payer, blockhash) = setup().await;
    let new_account = Keypair::new();
    let poor_payer = Keypair::new(); // New keypair with no funds
    
    let ix = create_system_account(
        poor_payer.pubkey(),
        new_account.pubkey(),
        "Test Name".to_string(),
        "Test Address".to_string(),
    ).unwrap();
    
    println!("🔑 Creating and signing transaction with unfunded account...");
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&poor_payer.pubkey()),
        &[&poor_payer, &new_account],
        blockhash,
    );
    
    println!("📡 Sending transaction...");
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err(), "Expected transaction to fail due to insufficient funds");
    println!("✅ Successfully caught insufficient funds error");
}