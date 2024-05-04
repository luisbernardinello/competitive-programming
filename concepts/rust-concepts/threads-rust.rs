use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

struct BankAccount {
    account_number: i32,
    balance: f64,
    lock: Mutex<()>,
}

impl BankAccount {
    fn new(account_number: i32, initial_balance: f64) -> Self {
        BankAccount {
            account_number,
            balance: initial_balance,
            lock: Mutex::new(()),
        }
    }

    fn deposit(&self, amount: f64) {
        let _guard = self.lock.lock().unwrap();
        self.balance += amount;
    }

    fn withdraw(&self, amount: f64) -> Result<(), &'static str> {
        let _guard = self.lock.lock().unwrap();
        if self.balance >= amount {
            self.balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    fn transfer_to(&self, destination: &BankAccount, amount: f64) -> Result<(), &'static str> {
        let _guard1 = self.lock.lock().unwrap();
        let _guard2 = destination.lock.lock().unwrap();
        if self.balance >= amount {
            self.balance -= amount;
            destination.balance += amount;
            Ok(())
        } else {
            Err("Insufficient funds")
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }
}

struct Bank {
    accounts: Mutex<Vec<BankAccount>>,
}

impl Bank {
    fn new() -> Self {
        Bank {
            accounts: Mutex::new(Vec::new()),
        }
    }

    fn open_account(&self, initial_balance: f64) -> i32 {
        let mut accounts = self.accounts.lock().unwrap();
        let account_number = accounts.len() as i32 + 1;
        let account = BankAccount::new(account_number, initial_balance);
        accounts.push(account);
        account_number
    }

    fn close_account(&self, account_number: i32) {
        let mut accounts = self.accounts.lock().unwrap();
        accounts.retain(|account| account.account_number != account_number);
    }

    fn get_total_balance(&self) -> f64 {
        let accounts = self.accounts.lock().unwrap();
        accounts.iter().map(|account| account.get_balance()).sum()
    }

    fn get_account_balance(&self, account_number: i32) -> Option<f64> {
        let accounts = self.accounts.lock().unwrap();
        accounts.iter().find(|account| account.account_number == account_number).map(|account| account.get_balance())
    }

    fn find_account(&self, account_number: i32) -> Option<&BankAccount> {
        let accounts = self.accounts.lock().unwrap();
        accounts.iter().find(|account| account.account_number == account_number)
    }
}

fn main() {
    let bank = Arc::new(Bank::new());

    for _ in 0..5 {
        bank.open_account(1000.0);
    }

    let deposit_handle = {
        let bank = Arc::clone(&bank);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let account_number = rng.gen_range(1, 6);
                let deposit_amount = rng.gen_range(1.0, 101.0);
                let account = bank.find_account(account_number).unwrap();
                account.deposit(deposit_amount);
                println!("Deposited ${:.2} into Account {}. Updated Balance: {:.2}", deposit_amount, account_number, account.get_balance());
                thread::sleep(Duration::from_millis(rng.gen_range(500, 1001)));
            }
        })
    };

    let withdrawal_handle = {
        let bank = Arc::clone(&bank);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let account_number = rng.gen_range(1, 6);
                let withdrawal_amount = rng.gen_range(1.0, 201.0);
                let account = bank.find_account(account_number).unwrap();
                match account.withdraw(withdrawal_amount) {
                    Ok(_) => println!("Withdrawn ${:.2} from Account {}. Updated Balance: {:.2}", withdrawal_amount, account_number, account.get_balance()),
                    Err(err) => println!("Withdrawal from Account {} failed: {}. Updated Balance: {:.2}", account_number, err, account.get_balance()),
                }
                thread::sleep(Duration::from_millis(rng.gen_range(500, 1001)));
            }
        })
    };

    let transfer_handle = {
        let bank = Arc::clone(&bank);
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            loop {
                let source_account_number = rng.gen_range(1, 6);
                let destination_account_number = rng.gen_range(1, 6);
                let transfer_amount = rng.gen_range(1.0, 101.0);
                let source_account = bank.find_account(source_account_number).unwrap();
                let destination_account = bank.find_account(destination_account_number).unwrap();
                match source_account.transfer_to(destination_account, transfer_amount) {
                    Ok(_) => println!("Transferred ${:.2} from Account {} to Account {}. Updated Balance in Source Account: {:.2}. Updated Balance in Destination Account: {:.2}", transfer_amount, source_account_number, destination_account_number, source_account.get_balance(), destination_account.get_balance()),
                    Err(err) => println!("Transfer from Account {} failed: {}.", source_account_number, err),
                }
                thread::sleep(Duration::from_millis(rng.gen_range(500, 1001)));
            }
        })
    };

    let monitoring_handle = thread::spawn(move || {
        loop {
            println!("Total Balance in the Bank: ${:.2}", bank.get_total_balance());
            thread::sleep(Duration::from_secs(5));
        }
    });

    deposit_handle.join().unwrap();
    withdrawal_handle.join().unwrap();
    transfer_handle.join().unwrap();
    monitoring_handle.join().unwrap();
}
