use realloc_api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{compute_budget::ComputeBudgetInstruction, signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "realloc_program",
        realloc_api::ID,
        processor!(realloc_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}


#[tokio::test]
async fn test_invalid_update() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Try to update non-existent account
    let (message_account, _bump) = Pubkey::find_program_address(&[b"message"], &realloc_api::ID);
    let test_message = "This should fail".to_string();
    let compute_budget_ix = ComputeBudgetInstruction::set_compute_unit_limit(400_000);  // Increased from default 200k
    let set_price_ix = ComputeBudgetInstruction::set_compute_unit_price(1);
    
    let ix = update(
        payer.pubkey(),
        message_account,
        test_message,
    );
    let tx = Transaction::new_signed_with_payer(
        &[        
            compute_budget_ix,
            set_price_ix,
            ix
        ],
        Some(&payer.pubkey()),
        &[&payer],
        blockhash,
    );
    let res = banks.process_transaction(tx).await;
    assert!(res.is_err());
}