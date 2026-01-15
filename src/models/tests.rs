use super::*;
use rust_decimal_macros::dec;

#[test]
fn test_deposit_try_new_valid() {
    let input = TransactionInput {
        r#type: TransactionType::Deposit,
        client: 1,
        tx: 10,
        amount: Some(dec!(100.0)),
    };

    let new_deposit = Deposit::try_new(&input);
    assert!(new_deposit.is_some());

    let deposit = new_deposit.unwrap();
    assert_eq!(deposit.client_id, 1);
    assert_eq!(deposit.tx, 10);
    assert_eq!(deposit.amount, dec!(100.0));
    assert_eq!(deposit.under_dispute, false);
}

#[test]
fn test_deposit_try_new_missing_amount() {
    let input = TransactionInput {
        r#type: TransactionType::Deposit,
        client: 1,
        tx: 10,
        amount: None,
    };

    assert!(Deposit::try_new(&input).is_none());
}

#[test]
fn test_valid_transaction_as_deposit_mut_when_deposit() {
    let mut tx_deposit = Transaction::Deposit(Deposit {
        tx: 1,
        client_id: 1,
        amount: dec!(10.0),
        under_dispute: false,
    });

    assert!(tx_deposit.as_deposit_mut().is_some());
}

#[test]
fn test_invalid_transaction_as_deposit_mut_when_not_deposit() {
    let mut tx_withdrawal = Transaction::Withdrawal;

    assert!(tx_withdrawal.as_deposit_mut().is_none());
}

#[test]
fn test_account_total() {
    let account = Account {
        available: dec!(100.0),
        held: dec!(50.0),
        locked: false,
    };
    assert_eq!(account.total(), dec!(150.0));
}

#[test]
fn test_account_output_rounding() {
    let account = Account {
        available: dec!(10.123456),
        held: dec!(20.123444),
        locked: true,
    };

    let output = account.to_output(99);

    assert_eq!(output.client, 99);
    assert!(output.locked);
    assert_eq!(output.available, dec!(10.1235));
    assert_eq!(output.held, dec!(20.1234));
    assert_eq!(output.total, dec!(30.2469));
}
