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

fn main() {
    let mut user1 = UserAccount::new(1, String::from("user1"));
    let mut user2 = UserAccount::new(2, String::from("user2"));

    user1.deposit(1000);
    println!("{}", user1.info());

    match user1.withdraw(500) {
        Ok(_) => println!("Withdrawal successful"),
        Err(e) => println!("Error during withdrawal: {:?}", e),
    }
    println!("{}", user1.info());

    match user1.transfer(&mut user2, 300) {
        Ok(_) => println!("Transfer successful"),
        Err(e) => println!("Error during transfer: {:?}", e),
    }
    println!("{}", user1.info());
    println!("{}", user2.info());
}
