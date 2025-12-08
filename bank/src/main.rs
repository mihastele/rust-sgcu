#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Self { id, holder, balance: 0 }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Self { accounts: Vec::new() }
    }
}

fn print_account(account: &Account) {
    println!("{:#?}", account);
}
fn print_akavnts(accounts: Vec<Account>) {
    println!("{:#?}", accounts);
}
fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn print_account_ret(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn add_cash_with_mutable_ref(account: &mut Account, cash: i32) {
    account.balance += cash;
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Alice"));

    // println!("{:#?}", bank);
    // print_account(&account);

    // let mut bank = Bank::new();
    // bank.accounts.push(account);
    // print_account(bank.accounts.get(0).unwrap());
    // print_akavnts(bank.accounts);
    // below won't work because accounts is moved and partial of bank is missing.
    // print_bank(bank);

    let account = print_account_ret(account);
    let account = print_account_ret(account);



    let mut accountmut = Account::new(2, String::from("Bob"));
    add_cash_with_mutable_ref(&mut accountmut, 100);
    print_account(&accountmut);
}
