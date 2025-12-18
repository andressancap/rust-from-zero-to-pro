use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Amount(u64);

impl Amount {
    fn checked_add(self, other: Amount) -> Result<Amount, TxError> {
        self.0.checked_add(other.0).map(Amount).ok_or(TxError::Overflow)
    }
    fn checked_sub(self, other: Amount) -> Result<Amount, TxError> {
        if self.0 < other.0 {
            return Err(TxError::InsufficientFunds);
        }
        self.0.checked_sub(other.0).map(Amount).ok_or(TxError::Overflow)
    }
    fn zero() -> Amount {
        Amount(0)
    }
}

#[derive(Debug)]
enum TxError {
    Overflow,
    InsufficientFunds,
    InvalidAmount,
    CannotTransferToSelf,
    UserNotFound,
    TargetNotFound,
}

#[derive(Debug, Clone)]
struct Transaction {
    from: Option<u64>,
    to: Option<u64>,
    amount: Amount,
}

#[derive(Debug)]
struct UserAccount {
    id: u64,
    balance: Amount,
    ledger: Vec<Transaction>,
}

impl UserAccount {
    fn new(id: u64) -> Self {
        UserAccount {
            id,
            balance: Amount::zero(),
            ledger: Vec::new(),
        }
    }

    fn deposit(&mut self, amount: Amount) -> Result<(), TxError> {
        if amount == Amount::zero() {
            return Err(TxError::InvalidAmount);
        }
        self.balance = self.balance.checked_add(amount)?;
        self.ledger.push(Transaction {
            from: None,
            to: Some(self.id),
            amount,
        });
        Ok(())
    }

    fn withdraw(&mut self, amount: Amount) -> Result<(), TxError> {
        if amount == Amount::zero() {
            return Err(TxError::InvalidAmount);
        }
        self.balance = self.balance.checked_sub(amount)?;
        self.ledger.push(Transaction {
            from: Some(self.id),
            to: None,
            amount,
        });
        Ok(())
    }

    fn transfer(&mut self, target: &mut UserAccount, amount: Amount) -> Result<(), TxError> {
        if self.id == target.id {
            return Err(TxError::CannotTransferToSelf);
        }
        if amount == Amount::zero() {
            return Err(TxError::InvalidAmount);
        }

        self.balance.checked_sub(amount)?;
        target.balance.checked_add(amount)?;

        self.balance = self.balance.checked_sub(amount)?;
        target.balance = target.balance.checked_add(amount)?;

        let tx = Transaction {
            from: Some(self.id),
            to: Some(target.id),
            amount,
        };
        self.ledger.push(tx.clone());
        target.ledger.push(tx);
        Ok(())
    }

    fn get_balance(&self) -> Amount {
        self.balance
    }
}

fn handle_transaction(
    accounts: &mut HashMap<u64, UserAccount>,
    user_id: u64,
    tx: TransactionType,
) -> Result<(), TxError> {
    match tx {
        TransactionType::Deposit(amount) => {
            let user = accounts.get_mut(&user_id).ok_or(TxError::UserNotFound)?;
            user.deposit(Amount(amount))
        },
        TransactionType::Withdraw(amount) => {
            let user = accounts.get_mut(&user_id).ok_or(TxError::UserNotFound)?;
            user.withdraw(Amount(amount))
        },
        TransactionType::Transfer { target_id, amount } => {
            if user_id == target_id {
                return Err(TxError::CannotTransferToSelf);
            }

            let mut from = accounts.remove(&user_id).ok_or(TxError::UserNotFound)?;
            let mut to = accounts.remove(&target_id).ok_or(TxError::TargetNotFound)?;

            let result = from.transfer(&mut to, Amount(amount));

            accounts.insert(user_id, from);
            accounts.insert(target_id, to);
            result
        }
    }
}
enum TransactionType {
    Deposit(u64),
    Withdraw(u64),
    Transfer { target_id: u64, amount: u64 },
}

fn main() -> Result<(), TxError> {
    let mut accounts: HashMap<u64, UserAccount> = HashMap::new();
    accounts.insert(1, UserAccount::new(1));
    accounts.insert(2, UserAccount::new(2));

    handle_transaction(&mut accounts, 1, TransactionType::Deposit(100))?;
    println!("User 1 Balance: {:?}", accounts.get(&1).unwrap().get_balance());

    handle_transaction(&mut accounts, 1, TransactionType::Transfer { target_id: 2, amount: 50 })?;
    println!("User 1 Balance: {:?}", accounts.get(&1).unwrap().get_balance());
    println!("User 2 Balance: {:?}", accounts.get(&2).unwrap().get_balance());

    handle_transaction(&mut accounts, 2, TransactionType::Withdraw(30))?;
    println!("User 2 Balance: {:?}", accounts.get(&2).unwrap().get_balance());

    for (id, account) in &accounts {
        println!("User {}: Balance: {:?}, Ledger: {:?}", id, account.get_balance(), account.ledger);
    }

    Ok(())
}