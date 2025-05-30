use std::io;

#[derive(Debug)]
struct Account {
    name: String,
    balance: f64,
}

fn main() {
    let mut accounts: Vec<Account> = Vec::new();
    let mut current_active:Option<usize>=None;
    loop {
        println!("\n--- Bank Menu ---");
        println!("0. Access account");
        println!("1. Create Account");
        println!("2. View Balance");
        println!("3. Deposit");
        println!("4. Withdraw");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed input");

        match choice.trim() {
            "0" => current_active = access_account(&mut accounts),
            "1" => create_account(&mut accounts),
            "2" => view_balance(&accounts,&current_active),
            "3" => deposit(&mut accounts,&current_active),
            "4" => withdraw(&mut accounts,&current_active),
            "5" => break,
            _ => println!("Invalid option."),
        }
    }
}
fn get_account_index(accounts: &Vec<Account>,target_name: &str) -> Option<usize>{
    accounts.iter().position(|account| account.name==target_name)
}
fn access_account(accounts: &mut Vec<Account>) -> Option<usize>{
    let mut name = String::new();
    println!("Write your name:");
    io::stdin().read_line(&mut name).expect("invalid inp");
    match get_account_index(&accounts,&name) {
        Some(index) => {println!("login successful"); Some(index)}
        None =>{println!("login failed");None}
    }
}
fn create_account(accounts: &mut Vec<Account>) {
    // TODO: implement account creation
    let mut name = String::new();
    let mut balance = String::new();

    println!("Enter account name:");
    io::stdin().read_line(&mut name).expect("Failed name input");

    let name = name.trim().to_string();
    println!("Enter initial balance:");

    io::stdin().read_line(&mut balance)
        .expect("failed bakance input");
    let balance: f64 = balance.trim().parse()
        .expect("Invalid balance input");
    
    let account = Account {
        name,
        balance,
    };
    accounts.push(account);

    println!("Account created successfully!");
}

fn view_balance(accounts: &Vec<Account>,index:&Option<usize>) {
    if let Some(i) =*index{
        println!("your balance is :{}",(accounts[i].balance));
    }else{
        println!("not connected");
    }
}

fn deposit(accounts: &mut Vec<Account>,index:&Option<usize>) {
    // TODO: implement deposit logic
    println!("how much do you want to deposit?");
    let mut amount:String = String::new();
    io::stdin().read_line(&mut amount).expect("failed balance input");
    let amount:f64 = amount.trim().parse().expect("invalid balance input");
    if let Some(i) =*index{
        println!("you added {}",amount);
        accounts[i].balance+=amount;
    }else{
        println!("not connected");
    }
}

fn withdraw(accounts: &mut Vec<Account>,index:&Option<usize>) {
    // TODO: implement withdrawal logic
    println!("how much do you want to withdraw?");
    let mut amount:String = String::new();
    io::stdin().read_line(&mut amount).expect("failed balance input");
    let amount:f64 = amount.trim().parse().expect("invalid balance input");
    if let Some(i) =*index{
        if amount <= accounts[i].balance{
            println!("you added {}",amount);
            accounts[i].balance-=amount;
        }else {
            println!("you are poor :D");
        }

    }else{
        println!("not connected");
    }
}
