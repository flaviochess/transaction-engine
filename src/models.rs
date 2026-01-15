use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, Deserialize)]
pub struct TransactionInput {
    #[serde(rename = "type")]
    pub r#type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

#[derive(Debug, Serialize)]
pub struct AccountOutput {
    client: u16,
    available: Decimal,
    held: Decimal,
    total: Decimal,
    locked: bool,
}

pub struct Deposit {
    pub(crate) tx: u32,
    pub(crate) client_id: u16,
    pub(crate) amount: Decimal,
    pub(crate) under_dispute: bool,
}

impl Deposit {
    pub fn try_new(tx: &TransactionInput) -> Option<Self> {
        let amount = tx.amount?;

        Some(Self {
            tx: tx.tx,
            client_id: tx.client,
            amount,
            under_dispute: false,
        })
    }
}

pub enum Transaction {
    Deposit(Deposit),
    Withdrawal,
}

impl Transaction {
    pub fn as_deposit_mut(&mut self) -> Option<&mut Deposit> {
        match self {
            Transaction::Deposit(data) => Some(data),
            _ => None,
        }
    }
}

pub struct Account {
    pub(crate) available: Decimal,
    pub(crate) held: Decimal,
    pub(crate) locked: bool,
}

impl Account {
    pub fn new() -> Self {
        Self {
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            locked: false,
        }
    }

    pub fn total(&self) -> Decimal {
        self.available + self.held
    }

    pub fn to_output(&self, client_id: u16) -> AccountOutput {
        AccountOutput {
            client: client_id,
            available: self.available.round_dp(4),
            held: self.held.round_dp(4),
            total: self.total().round_dp(4),
            locked: self.locked,
        }
    }
}

#[cfg(test)]
mod tests;
