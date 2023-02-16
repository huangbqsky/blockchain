use std::thread;
use std::time::Duration;
use bigint::U256;
use utils::serializer::{serialize, hash_str, hash_u8};
use crate::block::Block;

// nonce随机数的最大值
const MAX_NONCE: u32 = 0x7FFFFFFF;

// 工作量证明任务
pub struct ProofOfWork {
    target: U256,
}

impl ProofOfWork {
    // 计算当前任务难度值
    pub fn new(bits: u32) -> Self {
        let (mant, expt) = {
            let unshifted_expt = bits >> 24;
            if unshifted_expt <= 3 {
                ((bits & 0xFFFFFF) >> (8 * (3 - unshifted_expt as usize)), 0)
            } else {
                (bits & 0xFFFFFF, 8 * ((bits >> 24) - 3))
            }
        };

        if mant > 0x7FFFFF {
            Self {
                target: Default::default(),
            }
        } else {
            Self {
                target: U256::from(mant as u64) << (expt as usize),
            }
        }
    }

    // 开启工作量证明任务，即挖矿
    pub fn run(&self, mut block: &mut Block) {
        println!("Start mining .... ");
        // 休眠3秒，模拟挖矿
        thread::sleep(Duration::from_secs(3));

        // 计算 nonce 值的过程就是对区块 header 不断的运算哈希，直至找到能使区块哈希小于 target 的 nonce。
        let mut nonce: u32 = 0;
        while nonce <= MAX_NONCE {
            // 计算值
            let header_ser = Self::prepare_data(&mut block, nonce);
            let mut hash_u: [u8; 32] = [0; 32];
            hash_u8(&header_ser, &mut hash_u);

            // 判断值是否满足要求，满足则计算并设置区块哈希值
            let hash_int = U256::from(hash_u);
            if hash_int <= self.target {
                block.hash = hash_str(&header_ser);
                println!("Produced a new block!");
                return;
            }

            nonce += 1;
        }
    }

    // 准备区块头数据
    pub fn prepare_data(block: &mut Block, nonce: u32) -> Vec<u8> {
        block.header.nonce = nonce;
        serialize(&(block.header))
    }
}
