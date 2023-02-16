### Description

#Rust-in-Blockchain

A very simple blockchain in Rust.

➜ caogo run 

    -------------------------Mine Info----------------------------
    Start genesis mining .... 
    New produced a genesis block!
    New Produced genesis block saved!
    
    Start mining .... 
    Produced a new block!
    New produced block saved!
    
    Start mining .... 
    Produced a new block!
    New produced block saved!
    
    -------------------------Miner Info------------------------------
    Miner {
        name: "anonymous",
        balance: 204,
        address: "0x1b2d",
    }
    -------------------------Account Info----------------------------
    Account {
        nonce: 2,
        name: "Kim",
        balance: 84,
        address: "0xabcd",
        hash: "19a1b86d52dc298fb8ec67080b3cfe4672cb640b79e9b4bfd43fcaacce40e618",
    }
    Account {
        nonce: 4,
        name: "Tom",
        balance: 103,
        address: "0xabce",
        hash: "bcc52eda8d4b4ef78e2b7c9e5ed720141fe4a4e05646ccf5418ed3fbd2ea7541",
    }
    Account {
        nonce: 2,
        name: "Jim",
        balance: 109,
        address: "0xabcf",
        hash: "3d6cb5ed0f318bf9c57bd5912937eb8561def6702f84835424de6bc2c37e24c8",
    }
    -------------------------Block Info------------------------------
    Block {
        header: BlockHeader {
            nonce: 0,
            bits: 553713663,
            time: 1676540903,
            txs_hash: "fb8cde2d50d0d48bce64c96c507194823a5b6dcb46c90034fda8f0f6b0dc6656",
            pre_hash: "22caaf24ef0aea3522c13d133912d2b722caaf24ef0aea3522c13d133912d2b7",
        },
        tranxs: [
            Transaction {
                nonce: 0,
                amount: 0,
                fee: 0,
                from: "0x0000",
                to: "0x0000",
                sign: "创世区块",
                hash: "fb8cde2d50d0d48bce64c96c507194823a5b6dcb46c90034fda8f0f6b0dc6656",
            },
        ],
        hash: "f8b2c59e9fc7b17282326ff74a344f9c9d258c94f4ed7a2a0f8cad3378f3c9a0",
    }
    Block {
        header: BlockHeader {
            nonce: 0,
            bits: 553713663,
            time: 1676540903,
            txs_hash: "20c4a3b94c760d6a9f0bd9b2cd0dcb683cd6ad57d8c8fd3287df2df2eb5c1d3c",
            pre_hash: "f8b2c59e9fc7b17282326ff74a344f9c9d258c94f4ed7a2a0f8cad3378f3c9a0",
        },
        tranxs: [
            Transaction {
                nonce: 0,
                amount: 0,
                fee: 0,
                from: "0x0000",
                to: "0x1b2d",
                sign: "0x0000 -> 0x1b2d: 50 btc",
                hash: "9a1cd23a5b0ecb6326621ae93c218e8db4499283386ce6b6008d496d469364c2",
            },
            Transaction {
                nonce: 1,
                amount: 9,
                fee: 1,
                from: "0xabcd",
                to: "0xabce",
                sign: "0xabcd -> 0xabce: 9 btc",
                hash: "53d5cc1ac2b19555233eb2a9e8fdb62fb4a19387127f5f3b45b195edaa568305",
            },
            Transaction {
                nonce: 2,
                amount: 5,
                fee: 1,
                from: "0xabcd",
                to: "0xabce",
                sign: "0xabcd -> 0xabce: 5 btc",
                hash: "84fa895cb4f0d21d555e66366af23648987fd81da539d99cb4f6810558863c2d",
            },
        ],
        hash: "e5038ecfe990c594c37ed75209afe063169ba4e75fd49814c6c32aa5d658c7d3",
    }
    Block {
        header: BlockHeader {
            nonce: 0,
            bits: 553713663,
            time: 1676540906,
            txs_hash: "dcb88666af447af1833b20e871f0a8f024b369b88111a2b315758c5e841539f7",
            pre_hash: "e5038ecfe990c594c37ed75209afe063169ba4e75fd49814c6c32aa5d658c7d3",
        },
        tranxs: [
            Transaction {
                nonce: 0,
                amount: 0,
                fee: 0,
                from: "0x0000",
                to: "0x1b2d",
                sign: "0x0000 -> 0x1b2d: 50 btc",
                hash: "9a1cd23a5b0ecb6326621ae93c218e8db4499283386ce6b6008d496d469364c2",
            },
            Transaction {
                nonce: 3,
                amount: 6,
                fee: 1,
                from: "0xabce",
                to: "0xabcf",
                sign: "0xabce -> 0xabcf: 6 btc",
                hash: "edb391ee92448903f7b1f26b8e23e1cc4b7a77d78e9f5e70fec7ebedb7670052",
            },
            Transaction {
                nonce: 4,
                amount: 3,
                fee: 1,
                from: "0xabce",
                to: "0xabcf",
                sign: "0xabce -> 0xabcf: 3 btc",
                hash: "cde1e971fab799a9b3ca6adc150c4c3c3b472ab8f2185f3d1985179b4d30f3b2",
            },
        ],
        hash: "ea215e23cc2bf4133a09db7b0aeb0598f02579b4f31854db06f4a579ceafa4de",
    }


