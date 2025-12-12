#[derive(Debug)]
struct UserAccount {
    id: u64,
    username: String,
    balance: u64,
}

#[derive(Debug)]
enum TxError {
    InsufficientFunds,
}

impl UserAccount {
    fn new(id: u64, username: String) -> Self {
        UserAccount {
            id,
            username,
            balance: 0,
        }
    }
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: u64) -> Result<(), String> {
        if amount > self.balance {
            Err(TxError::InsufficientFunds)
        } else {
            self.balance -= amount;
            Ok(());
        }
    }
    fn info(&self) -> String {
        format!(
            "UserAccount {{ id: {}, username: {}, balance: {} }}",
            self.id, self.username, self.balance
        )
    }
    fn transfer(&mut self, to: &mut UserAccount, amount: u64) -> Result<(), String>
    {
        if amount > self.balance {
            Err(TxError::InsufficientFunds)
        } else {
            self.balance -= amount;
            to.balance += amount;
            Ok(())
        }
    }
}

