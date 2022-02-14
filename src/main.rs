use serde::{Serialize, Deserialize};
use chrono::Utc;
use sha256::digest;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BlockData {
    sender: String,
    recipient: String,
    amount: i64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: usize,
    timestamp: i64,
    data: Vec<BlockData>,
    hash: String,
    previous_hash: String,
    proof: i64
}

impl Block {
    fn hash(&self) -> String {
        let hashable_data = format!("{:?}{}{}{}{}{}", self.data, self.hash, self.index, self.previous_hash, self.proof, self.timestamp);
        return digest(hashable_data);
    }

    fn mine(&self, previous_proof: i64) -> i64 {
        let mut hashed_string = "".to_string();
        let mut proof = 0;
        while !hashed_string.starts_with("00000") {
            let work_string = format!("f{}{}", previous_proof, proof);
            hashed_string = digest(work_string);
            println!("{}", &hashed_string);
            proof += 1;
        }
        return proof;
    }
}

#[derive(Debug, Serialize)]
struct BlockChain {
    chain: Vec<Block>
}

impl BlockChain {
    fn genesis(&mut self) {
        let now = Utc::now();
        let mut new_block = Block {
            index: 0, 
            timestamp: now.timestamp(),
            data: [].to_vec(),
            hash: "".to_string(),
            previous_hash: "".to_string(),
            proof: 0
        };

        new_block.hash = new_block.hash();
        self.chain.push(new_block);
    }

    fn add_block(&mut self, sender: String, recipient: String, amount: i64) {
        let previous_block = self.chain.last().unwrap();

        let mut chain_data = previous_block.data.to_vec();
        let new_data = BlockData {
            sender: sender.to_string(),
            recipient: recipient.to_string(),
            amount: amount
        };
        chain_data.push(new_data);

        let now = Utc::now();
        let mut new_block = Block {
            index: previous_block.index + 1, 
            timestamp: now.timestamp(),
            data: chain_data,
            hash: "".to_string(),
            previous_hash: previous_block.hash.to_string(),
            proof: 0
        };

        new_block.proof = new_block.mine(previous_block.proof);
        new_block.hash = new_block.hash();
        self.chain.push(new_block);
    }
}

fn main() {
    let mut chain = BlockChain {
        chain: [].to_vec()
    };

    chain.genesis();
    chain.add_block("mr.magoo".to_string(), "mr_magee".to_string(), 1000);

    println!("{:#?}", chain);
}
