use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

// Import only what we need from hand_api and lever_api
use hand_api::sdk::pull_lever;
use lever_api::ID as LEVER_PROGRAM_ID;
use hand_api::ID as HAND_PROGRAM_ID;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "hand",
        HAND_PROGRAM_ID,
        processor!(hand_program::process_instruction),
    );
    
    // Add the lever program to the test environment
    program_test.add_program(
        "lever",
        LEVER_PROGRAM_ID,
        processor!(lever_program::process_instruction),
    );
    
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_pull_lever_success() {
    // Setup test environment
    let (mut banks, payer, recent_blockhash) = setup().await;
    
    // Create a power account for testing
    let power = Keypair::new();
    
    // Test data
    let test_name = "test_power".to_string();
    
    // Create the pull lever instruction
    let ix = pull_lever(
        power.pubkey(),
        test_name.clone(),
    );
    
    // Create and sign transaction
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    // Process transaction
    let result = banks.process_transaction(tx).await;
    assert!(result.is_ok(), "Failed to process pull_lever transaction: {:?}", result);
}

#[tokio::test]
async fn test_pull_lever_invalid_name() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    let power = Keypair::new();
    
    // Create a string that's too long (> 32 bytes)
    let invalid_name = "this_name_is_definitely_too_long_for_the_program".to_string();
    
    let ix = pull_lever(
        power.pubkey(),
        invalid_name,
    );
    
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err(), "Expected transaction to fail with invalid name");
}

#[tokio::test]
async fn test_pull_lever_wrong_program() {
    let (mut banks, payer, recent_blockhash) = setup().await;
    let power = Keypair::new();
    
    // Create instruction with wrong program ID
    let wrong_program = Keypair::new();
    let mut ix = pull_lever(
        power.pubkey(),
        "test_power".to_string(),
    );
    ix.accounts[1].pubkey = wrong_program.pubkey();
    
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    
    let result = banks.process_transaction(tx).await;
    assert!(result.is_err(), "Expected transaction to fail with invalid program ID");
}