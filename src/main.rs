mod step1;
mod step2;


fn main() {
    println!("Hello, world!");
    println!("Test func");
}


#[test]
fn test_step_1() {
    let mut balances = step1::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);

    assert!(balances.transfer(1, 2, 50).is_ok());

    assert!(balances.balance(1) == 50);
    assert!(balances.balance(2) == 250);
}

#[test]
fn test_step_2() {
    let mut balances = step2::BalancesModule::new();

    balances.set_balance(1, 100);
    balances.set_balance(2, 200);

    assert!(balances.balance(1) == 100);
    assert!(balances.balance(2) == 200);

    assert!(balances.transfer(1, 2, 60).is_ok());
}