use crate::block::Block;
use crate::pow::ProofOfWork;
use crate::transaction::Transaction;

const MINER_NAME: &str = "anonymous";

// 矿工： 一个矿工要包含自己的地址，用于接受比特币，一个账户余额，当然也可以加上名字。
// 矿工第一笔交易是coinbase易，如果最后打包成功则获得挖矿奖励 50 比特币（随时 间半衰）。
// 其他的交易则陆续加入。然后进行工作量证明，开始挖矿。
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Miner {
    name: String,
    pub balance: u64,
    address: String,
}

impl Miner {
    // 生成一个矿工
    pub fn new(address: String) -> Self {
        Miner {
            name: MINER_NAME.to_string(),
            balance: 100,
            address: address,
        }
    }

    // 挖矿比特币奖励： 将矿工挖矿该得的收益支付给矿工，如果最后打包成功则获得挖矿奖励 50 比特币(随时间半衰)
    pub fn mine_block(&mut self, txs: &mut Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let mut fee = 0; // 挖矿手续费
        for tx in txs.iter() {
            fee += tx.fee.clone();
        }

        let from = "0x0000".to_string();
        let to = self.address.clone();
        let sign = format!("{} -> {}: 50 btc", from, to);
        let coinbase = Transaction::new(from, to, 0, 0, 0, sign);

        // 加入coinbase交易和普通交易
        let mut txs_all: Vec<Transaction> = Vec::new();
        txs_all.push(coinbase);
        txs_all.append(txs);
        let block = Self::mine_job(txs_all, pre_hash, bits);

        self.balance += 50; // 挖矿奖励，实际中会半衰 50、25、12.5
        self.balance += fee;

        block
    }

    // 挖矿任务 - 工作量证明
    fn mine_job(txs: Vec<Transaction>, pre_hash: String, bits: u32) -> Block {
        let mut block = Block::new(txs, pre_hash, bits);
        let pow = ProofOfWork::new(bits);
        pow.run(&mut block);

        block
    }

    // 打印矿工信息
    pub fn miner_info(&self) {
        println!("{:#?}", &self);
    }
}
