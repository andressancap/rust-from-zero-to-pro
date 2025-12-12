use std::collections::HashMap;

#[derive(Debug)]
enum TransactionType {
    Deposit(u64),
    Withdraw(u64),
    Transfer { amount: u64, to: u64 },
}

#[derive(Debug)]
struct UserAccount {
    id: u64,
    username: String,
    balance: u64,
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
            Err(String::from("InsufficientFunds"))
        } else {
            self.balance -= amount;
            Ok(())
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
            Err(String::from("InsufficientFunds"))
        } else {
            self.balance -= amount;
            to.balance += amount;
            Ok(())
        }
    }
}

fn handle_transaction(accounts: &mut HashMap<u64, UserAccount>, user_id: u64,tx: TransactionType) -> Result<(), String> {
    match tx {
        TransactionType::Deposit(amount) => {
            let user = accounts.get_mut(&user_id).ok_or("UserNotFound")?;
            user.deposit(amount);
            Ok(())
        }
        TransactionType::Withdraw(amount) => {
            let user = accounts.get_mut(&user_id).ok_or("UserNotFound")?;
            user.withdraw(amount)
        }
        TransactionType::Transfer { amount, to } => {
            if user_id == to {
                return Err(String::from("CannotTransferToSelf"));
            }
            let mut from = accounts.remove(&user_id).ok_or("UserNotFound")?;
            let mut to_acc = accounts.remove(&to).ok_or("TargetNotFound")?;

            let result = from.transfer(&mut to_acc, amount);

            accounts.insert(user_id, from);
            accounts.insert(to, to_acc);

            result
        }
    }
}


fn main() {

    let mut accounts = HashMap::new();
    accounts.insert(1, UserAccount::new(1, "user1".into()));
    accounts.insert(2, UserAccount::new(2, "user2".into()));

    let tx1 = TransactionType::Deposit(1000);
    let tx2 = TransactionType::Withdraw(500);
    let tx3 = TransactionType::Transfer {
        amount: 300,
        to: 2,
    };

    handle_transaction(&mut accounts, 1, tx1).unwrap();
    println!("{:?}", &accounts.get_mut(&1).unwrap().info());

    handle_transaction(&mut accounts, 1, tx2).unwrap();
    println!("{:?}", &accounts.get_mut(&1).unwrap().info());

    match handle_transaction(&mut accounts, 1, tx3) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Transfer failed: {:?}", e),
    }
    println!("{:?}", &accounts.get_mut(&1).unwrap().info());
    println!("{:?}", accounts.get_mut(&2).unwrap().info());
}