fn main() {
    let mut account1 = BankAccount {
        account_number: 10,
        holder_name: String::from("Patika"),
        balance: 500.0,
    };

    let mut account2 = BankAccount {
        account_number: 11,
        holder_name: String::from("Risein"),
        balance: 300.0,
    };

    account1.deposit(200.0);

    account2.withdraw(100.0);

    println!(
        "Account {} ({}) balance: ${:.2}",
        account1.account_number,
        account1.holder_name,
        account1.balance()
    );
    println!(
        "Account {} ({}) balance: ${:.2}",
        account2.account_number,
        account2.holder_name,
        account2.balance()
    );
}
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}
struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: f64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!(
                "Deposited ${:.2} into account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!("Invalid deposit amount: ${:.2}", amount);
        }
    }
    fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            println!(
                "Withdrew ${:.2} from account {}. New balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else if amount > self.balance {
            println!(
                "Insufficient funds! Cannot withdraw ${:.2} from account {}. Current balance: ${:.2}",
                amount, self.account_number, self.balance
            );
        } else {
            println!("Invalid withdraw amount: ${:.2}", amount);
        }
    }
    fn balance(&self) -> f64 {
        self.balance
    }
}
