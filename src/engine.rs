use crate::models::{Account, Deposit, Transaction, TransactionInput, TransactionType};
use std::collections::HashMap;

pub struct Engine {
    accounts: HashMap<u16, Account>,
    transactions: HashMap<u32, Transaction>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn get_accounts(&self) -> &HashMap<u16, Account> {
        &self.accounts
    }

    fn is_duplicated(&self, transaction: &TransactionInput) -> bool {
        let is_credit_or_debit = matches!(
            transaction.r#type,
            TransactionType::Deposit | TransactionType::Withdrawal
        );
        is_credit_or_debit && self.transactions.contains_key(&transaction.tx)
    }

    pub fn process(&mut self, transaction: TransactionInput) {
        if self.is_duplicated(&transaction) {
            return;
        }

        match transaction.r#type {
            TransactionType::Deposit => self.handle_deposit(transaction),
            TransactionType::Withdrawal => self.handle_withdrawal(transaction),
            TransactionType::Dispute => self.handle_dispute(transaction),
            TransactionType::Resolve => self.handle_resolve(transaction),
            TransactionType::Chargeback => self.handle_chargeback(transaction),
        }
    }

    fn find_deposit_mut(
        transactions: &mut HashMap<u32, Transaction>,
        tx_id: u32,
    ) -> Option<&mut Deposit> {
        transactions.get_mut(&tx_id)?.as_deposit_mut()
    }

    fn handle_deposit(&mut self, transaction: TransactionInput) {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(Account::new);

        if account.locked {
            return;
        }

        let Some(deposit) = Deposit::try_new(&transaction) else {
            return;
        };

        account.available += deposit.amount;
        self.transactions
            .insert(deposit.tx, Transaction::Deposit(deposit));
    }

    fn handle_withdrawal(&mut self, transaction: TransactionInput) {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(Account::new);

        if account.locked {
            return;
        }

        let Some(amount) = transaction.amount else {
            return;
        };

        if account.available >= amount {
            account.available -= amount;
            self.transactions
                .insert(transaction.tx, Transaction::Withdrawal);
        }
    }

    fn handle_dispute(&mut self, transaction: TransactionInput) {
        let Some(deposit) = Self::find_deposit_mut(&mut self.transactions, transaction.tx) else {
            return;
        };

        if deposit.under_dispute || deposit.client_id != transaction.client {
            return;
        }

        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(Account::new);
        account.available -= deposit.amount;
        account.held += deposit.amount;
        deposit.under_dispute = true;
    }

    fn handle_resolve(&mut self, transaction: TransactionInput) {
        let Some(deposit) = Self::find_deposit_mut(&mut self.transactions, transaction.tx) else {
            return;
        };

        if !deposit.under_dispute || deposit.client_id != transaction.client {
            return;
        }

        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(Account::new);
        account.held -= deposit.amount;
        account.available += deposit.amount;
        deposit.under_dispute = false;
    }

    fn handle_chargeback(&mut self, transaction: TransactionInput) {
        let Some(deposit) = Self::find_deposit_mut(&mut self.transactions, transaction.tx) else {
            return;
        };

        if !deposit.under_dispute || deposit.client_id != transaction.client {
            return;
        }

        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(Account::new);
        account.held -= deposit.amount;
        account.locked = true;
        deposit.under_dispute = false;
    }
}

#[cfg(test)]
mod tests;
