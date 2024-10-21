use api::prelude::*;
use solana_program::hash::Hash;
use solana_program_test::{processor, BanksClient, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use steel::*;
use api::prelude::TokenInstruction::CreateToken;

async fn setup() -> (BanksClient, Keypair, Hash) {
    let mut program_test = ProgramTest::new(
        "pda-mint-authority",
        api::ID,
        processor!(pda_mint_authority_program::process_instruction),
    );
    program_test.prefer_bpf(true);
    program_test.start().await
}

#[tokio::test]
async fn run_test() {
    // Setup test
    let (mut banks, payer, blockhash) = setup().await;

    // Submit initialize transaction.
    let ix = create_token(
        payer.pubkey(), "Test_Token".to_string(),
        "$TEST".to_string(),
        "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQqGK3diR3Zi-mnOXEaj-3ewmFyRYVxGzVzZw&s".to_string()
    );
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was initialized.
    let mint_address = mint_pda().0;
    let mint_account = banks.get_account(mint_address).await.unwrap().unwrap();
    // let counter = CreateToken::try_from_bytes(&mint_account.data).unwrap();
    assert_eq!(mint_account.owner, api::ID);
    // assert_eq!(counter.value, 0);

    // Submit add transaction.
    let ix = mint_token(payer.pubkey(), 42);
    let tx = Transaction::new_signed_with_payer(&[ix], Some(&payer.pubkey()), &[&payer], blockhash);
    let res = banks.process_transaction(tx).await;
    assert!(res.is_ok());

    // Verify counter was incremented.
    let counter_account = banks.get_account(counter_address).await.unwrap().unwrap();
    let counter = Counter::try_from_bytes(&counter_account.data).unwrap();
    assert_eq!(counter.value, 42);
}

