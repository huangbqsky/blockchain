use serde::Serialize;
use utils::serializer::{serialize, hash_str};

// 交易体
#[derive(Serialize, Debug, Clone)]
pub struct Transaction {
    pub nonce: u64, // 交易记录值
    pub amount: u64, // 交易金额
    pub fee: u64, // 交易手续费
    pub from: String, // 交易双方from来源方地址
    pub to: String, // 交易双方的to接收方地址
    pub sign: String, // 标记一些具体信息
    pub hash: String, //整个交易的哈希值
}

impl Transaction {
    pub fn new(from: String, to: String,
               amount: u64, fee: u64,
               nonce: u64, sign: String) -> Self
    {
        let mut tx = Transaction {
            nonce,
            amount,
            fee,
            from,
            to,
            sign,
            hash: "".to_string(),
        };
        tx.set_hash();

        tx
    }

    pub fn set_hash(&mut self) {
        let txs_ser = serialize(&self);
        self.hash = hash_str(&txs_ser);
    }
}
