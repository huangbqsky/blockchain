use std::sync::Mutex;
use std::collections::HashMap;
use bigint::U256;
use leveldb::database::Database;
use utils::bkey::BKey;
use utils::serializer::{serialize, hash_str, hash_u8};
use crate::block::Block;
use crate::bcdb::BlockChainDb;
use crate::transaction::Transaction;
use crate::pow::ProofOfWork;

const INIT_BITS: u32 = 0x2100FFFF;
// 使用 bcdb.rs 中的存储函数来存储区块链。
const SAVE_DIR: &str = "bc_db";
// 创世区块pre_hash：第一个区块没有pre_hash，所以需要手动设置一个值作为创世区块的 pre_hash。
const PRE_HASH: &str = "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7";

// 区块链：一条链需要保存多个区块。
pub struct BlockChain {
    blocks_db: Box<Database<BKey>>, // LevelDB数据库
    blocks_index: Mutex<HashMap<String, Block>>, // 区块链
    pub gnes_hash: String, // 创世区块hash
    pub curr_hash: String, // 当前区块hash
    pub curr_bits: u32,  // // 当前区块hash
}

impl BlockChain {
    pub fn new() -> Self {
        // 创建LevelDB数据库
        let mut db = BlockChainDb::new(SAVE_DIR);
        // 生成创世区块
        let genesis = Self::genesis_block();
        // 将创世区块写入
        BlockChain::write_block(&mut db, &genesis);
        // 将创世区块哈希值作为尾巴写入
        BlockChain::write_tail(&mut db, &genesis);
        println!("New Produced genesis block saved!\n");
        let gene_block = genesis.clone();
        let mut block_index = Mutex::new(HashMap::new());
        // 创世区块入链
        Self::update_hmap(&mut block_index, gene_block);

        let gnes_hash = genesis.hash.clone();
        let curr_hash = genesis.hash.clone();
        BlockChain {
            blocks_db: Box::new(db),
            blocks_index: block_index,
            gnes_hash: gnes_hash,
            curr_hash: curr_hash,
            curr_bits: INIT_BITS,
        }
    }

    // 生成创世区块 ：产生第一个区块（创世区块）以及添加新区块
    fn genesis_block() -> Block {
        println!("Start genesis mining .... ");
        let from = "0x0000".to_string();
        let to   = "0x0000".to_string();
        let sign = "创世区块".to_string();
        // 生成一个创世交易
        let tx = Transaction::new(from, to, 0, 0, 0, sign);
        // 生成一个创世区块
        let mut block  = Block::new(vec![tx], PRE_HASH.to_string(), INIT_BITS);
        // 为创世区块准备区块头数据
        let header_ser = ProofOfWork::prepare_data(&mut block, 0);
         // 为创世区块等区块头数据计算哈希
        block.hash = hash_str(&header_ser);
        println!("New produced a genesis block!");

        block
    }

    // 添加区块，形成区块链
    pub fn add_block(&mut self, block: Block) {
        // 数据写入库
        Self::write_block(&mut (self.blocks_db), &block);
        // 将区块哈希值作为尾巴写入
        Self::write_tail(&mut (self.blocks_db), &block);
        println!("New produced block saved!\n");
        self.curr_hash = block.hash.clone();
        self.curr_bits = block.header.bits.clone();
        // 区块入链
        Self::update_hmap(&mut self.blocks_index, block);
    }

    // 区块入链，更新区块链
    fn update_hmap(hmap: &mut Mutex<HashMap<String, Block>>, block: Block) {
        let mut hmap = hmap.lock().unwrap();
        let hash = block.hash.clone();
        hmap.insert(hash, block);
    }

    // 区块数据写入库
    fn write_block(db: &mut Database<BKey>, block: &Block) {
        // 基于区块链的header生成 key
        let header_ser = serialize(&(block.header));
        let mut hash_u: [u8; 32] = [0; 32];
        hash_u8(&header_ser, &mut hash_u);

        let key = BKey{ val: U256::from(hash_u) };
        let val = serialize(&block);
        BlockChainDb::write_db(db, key, &val);
    }

    // 将区块哈希值作为尾巴写入
    fn write_tail(mut db: &mut Database<BKey>, block: &Block) {
        let key = BKey{ val: U256::from("tail".as_bytes()) };
        let val = serialize(&(block.hash));
        BlockChainDb::write_db(&mut db, key, &val);
    }

    // 打印区块信息
    pub fn block_info(&self) {
        let mut hash = self.curr_hash.clone();
        let hmap = self.blocks_index.lock().unwrap();
        let mut blocks: Vec<Block> = Vec::new();

        loop {
            if let Some(b) = hmap.get(&hash) {
                blocks.push(b.clone());
                hash = b.header.pre_hash.clone();
            } else {
                panic!("Error getting block");
            }

            if hash == self.gnes_hash {
                if let Some(b) = hmap.get(&hash) {
                    blocks.push(b.clone());
                }
                break;
            }
        }
        blocks.reverse();

        for b in blocks {
            println!("{:#?}", b);
        }
    }
}
