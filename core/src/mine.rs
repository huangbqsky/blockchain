use crate::miner::Miner;
use crate::blockchain::BlockChain;
use crate::transaction::Transaction;

const MINER_ADDRESS: &str = "0x1b2d";

// 挖矿任务 : 将区块链和矿工放到一起组成挖矿任务，每当矿工挖到一个区块就立马添加到区块链中
pub struct Mine {
    pub miner: Miner, // 矿工
    pub blockchain: BlockChain, // 区块链
}

impl Mine {
    // 生成一个挖矿任务
    pub fn new() -> Self {
        Mine {
            blockchain: BlockChain::new(),
            miner: Miner::new(MINER_ADDRESS.to_string()),
        }
    }
    // 将矿工挖到的矿(区块)加入区块链中
    pub fn mining(&mut self, txs: &mut Vec<Transaction>) {
        // 准备pre_hash和难度值
        let pre_hash = self.blockchain.curr_hash.clone();
        let bits = self.blockchain.curr_bits.clone();
        // 核心代码点 ： 开始挖矿
        let block = self.miner.mine_block(txs, pre_hash, bits);
        // 区块保存
        self.blockchain.add_block(block);
    }
}
