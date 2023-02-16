use chrono::prelude::*;
use serde::Serialize;
use utils::serializer::{serialize, hash_str};
use crate::transaction::Transaction;

// 区块头：主要包含前一个区块，当前区块，区块打包时间，随机数，target等
#[derive(Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockHeader {
    pub nonce: u32, // 随机数，含义查看 https://happypeter.github.io/binfo/nonce
    pub bits: u32, // 压缩格式的target值。这一项是跟nonce紧密相关的
    pub time: i64, // 区块打包时间：矿工开始运算 header 哈希的那个时间点
    pub txs_hash: String, // 当前区块交易哈希：梅根哈希值，包含的所有交易的哈希值运算得出
    pub pre_hash: String, // 前一个区块的哈希：之前一个区块的header哈希
}

// 区块 ：包含区块头、区块体、区块哈希
#[derive(Serialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,  // 区块头: 包含前一个区块，当前区块，区块打包时间等
    pub tranxs: Vec<Transaction>, // 区块体：包含所有交易
    pub hash: String, // 区块哈希：计算区块头和区块体得到的哈希值
}

impl Block {
    // 生成一个区块
    pub fn new(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Self {
        // 准备时间、计算交易哈希值
        let time = Utc::now().timestamp();
        let txs_hash = Self::merkle_hash_str(&txs);

        Block {
            header: BlockHeader {
                nonce: 0,
                time: time,
                bits: bits,
                txs_hash: txs_hash,
                pre_hash: pre_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        }
    }

    // 计算梅根哈希值
    fn merkle_hash_str(txs: &Vec<Transaction>) -> String {
        if txs.len() == 0 {
            return "00000000".to_string();
        }

        let mut merkle_tree: Vec<String> = Vec::new();
        for tx in txs {
            merkle_tree.push(tx.hash.clone());
        }

        let mut j: u64 = 0;
        let mut size = merkle_tree.len();
        while size > 1 {
            let mut i: u64 = 0;
            let temp = size as u64;

            while i < temp {
                let k = Self::min(i + 1, temp - 1);
                let idx1 = (j + i) as usize;
                let idx2 = (j + k) as usize;
                let hash1 = merkle_tree[idx1].clone();
                let hash2 = merkle_tree[idx2].clone();
                let merge: String = format!("{}-{}",hash1, hash2);
                let merge_ser = serialize(&merge);
                let merge_hash = hash_str(&merge_ser);
                merkle_tree.push(merge_hash);
                i += 2;
            }

            j += temp;
            size = (size + 1) >> 1;
        }

        if merkle_tree.len() != 0 {
            merkle_tree.pop().unwrap()
        } else {
            "00000000".to_string()
        }
    }

    fn min(num1: u64, num2: u64) -> u64 {
        if num1 >= num2 {
            num2
        } else {
            num1
        }
    }
}
