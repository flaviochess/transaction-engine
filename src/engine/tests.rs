use super::*;
use crate::models::TransactionType;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn new_tx(
    client: u16,
    tx: u32,
    transaction_type: TransactionType,
    amount: Option<Decimal>,
) -> TransactionInput {
    TransactionInput {
        r#type: transaction_type,
        client,
        tx,
        amount,
    }
}

#[test]
fn test_deposit_increases_balance_for_new_account() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_id = 1;
    let transaction = new_tx(client_id, tx_id, TransactionType::Deposit, Some(dec!(10.0)));

    engine.process(transaction);

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(10.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(10.0));
    assert!(!account.locked);
}

#[test]
fn test_deposit_increases_balance() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(10.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Deposit,
        Some(dec!(20.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(30.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(30.0));
    assert!(!account.locked);
}

#[test]
fn test_deposit_ignore_duplicates() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(10.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(10.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(10.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(10.0));
    assert!(!account.locked);
}

#[test]
fn test_deposit_ignored_for_locked_account() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_one, TransactionType::Dispute, None));
    engine.process(new_tx(client_id, tx_one, TransactionType::Chargeback, None));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Deposit,
        Some(dec!(50.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(0.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(0.0));
    assert!(account.locked);
}

#[test]
fn test_withdrawal_sufficient_funds() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(10.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(5.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(5.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(5.0));
    assert!(!account.locked);
}

#[test]
fn test_withdrawal_insufficient_funds() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_id = 1;

    engine.process(new_tx(
        client_id,
        tx_id,
        TransactionType::Deposit,
        Some(dec!(10.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_id,
        TransactionType::Withdrawal,
        Some(dec!(20.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(10.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(10.0));
    assert!(!account.locked);
}

#[test]
fn test_withdrawal_ignore_duplicates() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(10.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(10.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(90.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(90.0));
    assert!(!account.locked);
}

#[test]
fn test_withdrawal_ignored_for_locked_account() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;
    let tx_three = 3;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(50.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_two, TransactionType::Dispute, None));
    engine.process(new_tx(client_id, tx_two, TransactionType::Chargeback, None));
    engine.process(new_tx(
        client_id,
        tx_three,
        TransactionType::Withdrawal,
        Some(dec!(50.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(50.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(50.0));
    assert!(account.locked);
}

#[test]
fn test_withdrawal_ignored_for_held_amount() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_one, TransactionType::Dispute, None));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(100.0)),
    ));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(0.0));
    assert_eq!(account.held, dec!(100.0));
    assert_eq!(account.total(), dec!(100.0));
    assert!(!account.locked);
}

#[test]
fn test_dispute() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_id = 1;

    engine.process(new_tx(
        client_id,
        tx_id,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_id, TransactionType::Dispute, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(0.0));
    assert_eq!(account.held, dec!(100.0));
    assert_eq!(account.total(), dec!(100.0));
    assert!(!account.locked);
}

#[test]
fn test_dispute_ignored_when_origin_tx_is_not_deposit() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(50.0)),
    ));
    engine.process(new_tx(client_id, tx_two, TransactionType::Dispute, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(50.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(50.0));
    assert!(!account.locked);
}

#[test]
fn test_resolve() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_id = 1;

    engine.process(new_tx(
        client_id,
        tx_id,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_id, TransactionType::Dispute, None));
    engine.process(new_tx(client_id, tx_id, TransactionType::Resolve, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(100.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(100.0));
    assert!(!account.locked);
}

#[test]
fn test_resolve_ignored_when_tx_is_not_under_dispute() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_one, TransactionType::Resolve, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(100.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(100.0));
    assert!(!account.locked);
}

#[test]
fn test_resolve_ignored_when_origin_tx_is_not_deposit() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;
    let tx_two = 2;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(
        client_id,
        tx_two,
        TransactionType::Withdrawal,
        Some(dec!(50.0)),
    ));
    engine.process(new_tx(client_id, tx_two, TransactionType::Resolve, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(50.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(50.0));
    assert!(!account.locked);
}

#[test]
fn test_chargeback_locks_account() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_id = 1;

    engine.process(new_tx(
        client_id,
        tx_id,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_id, TransactionType::Dispute, None));
    engine.process(new_tx(client_id, tx_id, TransactionType::Chargeback, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(0.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(0.0));
    assert!(account.locked);
}

#[test]
fn test_chargeback_ignored_when_tx_is_not_under_dispute() {
    let mut engine = Engine::new();
    let client_id = 1;
    let tx_one = 1;

    engine.process(new_tx(
        client_id,
        tx_one,
        TransactionType::Deposit,
        Some(dec!(100.0)),
    ));
    engine.process(new_tx(client_id, tx_one, TransactionType::Chargeback, None));

    let account = engine.get_accounts().get(&1).unwrap();
    assert_eq!(account.available, dec!(100.0));
    assert_eq!(account.held, dec!(0.0));
    assert_eq!(account.total(), dec!(100.0));
    assert!(!account.locked);
}
