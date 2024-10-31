use favorites_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "favorites_program",
        favorites_api::ID,
        processor!(favorites_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn test_validation_limits() {
    // Setup test
    let (mut _banks, payer, _blockhash) = setup().await;

    // Test string too long
    let long_color = "x".repeat(STRING_MAX_SIZE + 1);
    let ix = set_favorites(
        payer.pubkey(),
        42,
        long_color,
        vec!["hobby".to_string()],
    );
    assert!(ix.is_err());

    // Test too many hobbies
    let too_many_hobbies = (0..MAX_HOBBIES + 1)
        .map(|i| format!("hobby{}", i))
        .collect();
    let ix = set_favorites(
        payer.pubkey(),
        42,
        "blue".to_string(),
        too_many_hobbies,
    );
    assert!(ix.is_err());

    // Test hobby string too long
    let long_hobby = "x".repeat(STRING_MAX_SIZE + 1);
    let ix = set_favorites(
        payer.pubkey(),
        42,
        "blue".to_string(),
        vec![long_hobby],
    );
    assert!(ix.is_err());
}

