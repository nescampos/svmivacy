// unit_tests.rs
// Unit tests for smart contract modules

use anchor_lang::prelude::*;
use anchor_lang::{InstructionData, ToAccountMetas};
use solana_program::instruction::Instruction;
use solana_program::system_instruction;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

// Import from contracts crate
use contracts::smart_contracts::{Order, PlaceOrder, PlaceOrderParams};
use contracts::ID;

#[tokio::test]
async fn test_place_order() {
    // Initialize the test context
    let program_test = ProgramTest::new(
        "contracts",
        ID,
        None,
    );

    // Start the test context
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Create test accounts
    let user = Keypair::new();
    let order_account = Keypair::new();

    // Fund the user account
    let transfer_ix = system_instruction::transfer(
        &payer.pubkey(),
        &user.pubkey(),
        1_000_000_000, // 1 SOL
    );

    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Create the place order instruction
    let place_order_ix = Instruction {
        program_id: ID,
        accounts: PlaceOrder {
            order: order_account.pubkey(),
            user: user.pubkey(),
            system_program: solana_program::system_program::ID,
        }
        .to_account_metas(None),
        data: anchor_lang::InstructionData::data(
            &PlaceOrderParams {
                amount: 100,
                price: 50,
            }
        ),
    };

    // Create and sign transaction
    let mut transaction = Transaction::new_with_payer(&[place_order_ix], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &user, &order_account], recent_blockhash);

    // Process the transaction
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify the order account data
    let order_account_data = banks_client
        .get_account(order_account.pubkey())
        .await
        .unwrap()
        .expect("Order account not found");

    // Skip Anchor discriminator
    let order_data = &order_account_data.data[8..];

    // Deserialize the order account data
    let order: Order = Order::try_from_slice(order_data).unwrap();

    // Validate the order
    assert_eq!(order.amount, 100);
    assert_eq!(order.price, 50);
    assert_eq!(order.user, user.pubkey());
}