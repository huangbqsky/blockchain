use crate::transaction::Transaction;
use utils::serializer::{serialize, hash_str};
use serde::Serialize;

// 交易账户： 要包含地址、余额，其次还可以包含姓名、哈希标志等信息
#[derive(Serialize, Debug, Eq, PartialEq, Clone)]
pub struct Account {
    pub nonce: u64, // 随机数，每次交易+1
    pub name: String, // 姓名
    pub balance: u64, // 余额
    pub address: String, // 地址
    pub hash: String, // 哈希标志
}

impl Account {
    // 创建一个交易账号
    pub fn new(address: String, name: String) -> Self {
        let mut account = Account {
            nonce: 0,
            name: name,
            balance: 100,
            address: address,
            hash: "".to_string(),
        };
        account.set_hash();

        account
    }

    // 计算哈希值
    fn set_hash(&mut self) {
        let data = serialize(&self);
        self.hash = hash_str(&data);
    }

    // 交易转移比特币
    pub fn transfer_to(&mut self, to: &mut Self, amount: u64, fee: u64)
        -> Result<Transaction, String>
    {
        if amount + fee > self.balance {
            return Err("Error: not enough amount!".to_string());
        }

        self.balance -= amount;
        self.balance -= fee;
        self.nonce += 1;
        self.set_hash();

        to.balance += amount;
        to.nonce += 1;
        to.set_hash();

        // 标记一些具体信息，比如从交易账号A到交易账号B交易了amount btc
        let sign = format!("{} -> {}: {} btc",
                           self.address.clone(),
                           to.address.clone(),
                           amount);
        let tx = Transaction::new(self.address.clone(),
                                  to.address.clone(),
                                  amount, fee, self.nonce, sign);
        Ok(tx)
    }

    // 打印交易账号信息
    pub fn account_info(&self) {
        println!("{:#?}", &self);
    }
}
