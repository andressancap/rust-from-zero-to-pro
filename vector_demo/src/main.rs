use std::collections::HashMap;


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Amount(u64);

impl TryFrom<u64> for Amount {
    type Error = TxError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(TxError::InvalidAmount)
        } else {
            Ok(Amount(value))
        }
    }
}

impl Amount {
    fn checked_add(self, other: Amount) -> Result<Amount, TxError> {
        self.0.checked_add(other.0).map(Amount).ok_or(TxError::Overflow)
    }

    fn checked_sub(self, other: Amount) -> Result<Amount, TxError> {
        if self.0 < other.0 {
            return Err(TxError::InsufficientFunds);
        }
        Ok(Amount(self.0 - other.0))
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
enum Transaction {
    Deposit { to: u64, amount: Amount },
    Withdraw { from: u64, amount: Amount },
    Transfer { from: u64, to: u64, amount: Amount },
}


#[derive(Debug)]
struct UserAccount {
    id: u64,
    balance: Amount,
    ledger: Vec<Transaction>,
}

impl UserAccount {
    fn new(id: u64) -> Self {
        Self {
            id,
            balance: Amount::zero(),
            ledger: Vec::new(),
        }
    }

    fn deposit(&mut self, amount: Amount) -> Result<(), TxError> {
        if amount.0 == 0 {
            return Err(TxError::InvalidAmount);
        } 
        self.balance = self.balance.checked_add(amount)?;
        self.ledger.push(Transaction::Deposit {
            to: self.id,
            amount,
        });
        Ok(())
    }

    fn withdraw(&mut self, amount: Amount) -> Result<(), TxError> {
        self.balance = self.balance.checked_sub(amount)?;
        self.ledger.push(Transaction::Withdraw {
            from: self.id,
            amount,
        });
        Ok(())
    }

    fn transfer(&mut self, target: &mut UserAccount, amount: Amount) -> Result<(), TxError> {
        if self.id == target.id {
            return Err(TxError::CannotTransferToSelf);
        }

        let new_from_balance = self.balance.checked_sub(amount)?;
        let new_to_balance = target.balance.checked_add(amount)?;

        self.balance = new_from_balance;
        target.balance = new_to_balance;

        let tx = Transaction::Transfer {
            from: self.id,
            to: target.id,
            amount,
        };

        self.ledger.push(tx.clone());
        target.ledger.push(tx);

        Ok(())
    }

    fn balance(&self) -> Amount {
        self.balance
    }
}


enum TransactionType {
    Deposit(u64),
    Withdraw(u64),
    Transfer { target_id: u64, amount: u64 },
}

fn handle_transaction(
    accounts: &mut HashMap<u64, UserAccount>,
    user_id: u64,
    tx: TransactionType,
) -> Result<(), TxError> {
    match tx {
        TransactionType::Deposit(raw) => {
            let amount = raw.try_into()?;
            let user = accounts.get_mut(&user_id).ok_or(TxError::UserNotFound)?;
            user.deposit(amount)
        }

        TransactionType::Withdraw(raw) => {
            let amount = raw.try_into()?;
            let user = accounts.get_mut(&user_id).ok_or(TxError::UserNotFound)?;
            user.withdraw(amount)
        }

        TransactionType::Transfer { target_id, amount: raw } => {
            if user_id == target_id {
                return Err(TxError::CannotTransferToSelf);
            }

            let amount = raw.try_into()?;

            let mut from = accounts.remove(&user_id).ok_or(TxError::UserNotFound)?;
            let mut to = accounts.remove(&target_id).ok_or(TxError::TargetNotFound)?;

            let result = from.transfer(&mut to, amount);

            accounts.insert(user_id, from);
            accounts.insert(target_id, to);

            result
        }
    }
}


fn main() -> Result<(), TxError> {
    let mut accounts: HashMap<u64, UserAccount> = HashMap::new();
    accounts.insert(1, UserAccount::new(1));
    accounts.insert(2, UserAccount::new(2));

    handle_transaction(&mut accounts, 1, TransactionType::Deposit(100))?;
    println!("User 1 Balance: {:?}", accounts.get(&1).unwrap().balance());

    handle_transaction(&mut accounts, 1, TransactionType::Transfer { target_id: 2, amount: 50 })?;
    println!("User 1 Balance: {:?}", accounts.get(&1).unwrap().balance());
    println!("User 2 Balance: {:?}", accounts.get(&2).unwrap().balance());

    handle_transaction(&mut accounts, 2, TransactionType::Withdraw(30))?;
    println!("User 2 Balance: {:?}", accounts.get(&2).unwrap().balance());

    for (id, account) in &accounts {
        println!("User {}: Balance: {:?}, Ledger: {:?}", id, account.balance(), account.ledger);
    }

    Ok(())
}