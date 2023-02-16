use core::account::Account;
use core::transaction::Transaction;
use core::mine::Mine;

fn main() {
    // 生成3个交易账户
    let mut user1 = Account::new("0xabcd".to_string(), "Kim".to_string());
    let mut user2 = Account::new("0xabce".to_string(), "Tom".to_string());
    let mut user3 = Account::new("0xabcf".to_string(), "Jim".to_string());

    println!("-------------------------Mine Info----------------------------");
    // 生成挖矿任务：包含矿工和区块链
    let mut mine = Mine::new();
    // 生成一个交易：
    let mut txs: Vec<Transaction> = Vec::new();
     // 交易转移比特币：user1账号转移一笔比特币给user2，金额是9，手续费是1
    let res = user1.transfer_to(&mut user2, 9, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
     // 交易转移比特币：user1账号转移一笔比特币给user2，金额是5，手续费是1
    let res = user1.transfer_to(&mut user2, 5, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    // 矿工挖矿，挖到矿加入区块链
    mine.mining(&mut txs);

    // 生成一个交易：
    let mut txs: Vec<Transaction> = Vec::new();
    // 交易转移比特币：user2账号转移一笔比特币给user1，金额是6，手续费是1
    let res = user2.transfer_to(&mut user3, 6, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    // 交易转移比特币：user2账号转移一笔比特币给user1，金额是3，手续费是1
    let res = user2.transfer_to(&mut user3, 3, 1);
    match res {
        Ok(tx) => txs.push(tx),
        Err(e) => panic!("{}", e),
    }
    // 矿工挖矿，挖到矿加入区块链中
    mine.mining(&mut txs);

    println!("-------------------------Miner Info------------------------------");
    // 打印矿工信息
    mine.miner.miner_info();

    println!("-------------------------Account Info----------------------------");
    // 打印交易账号信息
    let users = vec![&user1, &user2, &user3];
    for u in users {
        u.account_info();
    }

    println!("-------------------------Block Info------------------------------");
    // 打印交易区块链信息
    mine.blockchain.block_info();
}
