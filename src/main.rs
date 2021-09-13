mod step1;

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
}