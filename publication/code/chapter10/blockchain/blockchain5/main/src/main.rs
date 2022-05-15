use core::account::Account;
use core::blockchain::BlockChain as BC;

fn main() {
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    let mut bc = BC::new();

    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => bc.add_block(vec![tx]),
        Err(e) => panic!("{}", e),
    }

    let res = user2.transfer_to(&mut user3, 6, 1);
    match res {
        Ok(tx) => bc.add_block(vec![tx]),
        Err(e) => panic!("{}", e),
    }

    println!("-------------------------Account Info----------------------------");
    let users = vec![&user1, &user2, &user3];
    for u in users {
        u.account_info();
    }

    println!("-------------------------Block Info------------------------------");
    bc.block_info();
}
