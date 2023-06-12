use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<Block>,
}

#[allow(dead_code)]
impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![Block::genesis()] }
    }

    pub fn mine_and_add_block(&mut self, data: String) -> &Block {
        let last_block = self.blocks.last().unwrap();
        let new_block = Block::mine_block(last_block, data);

        self.blocks.push(new_block);
        self.blocks.last().unwrap()
    }

    pub fn add_block(&mut self, block: Block) -> &Block {
        self.blocks.push(block);
        self.blocks.last().unwrap()
    }

    pub fn verify(&self) -> bool {
        let genesis = Block::genesis();

        if !self.blocks.first().unwrap().eq(&genesis) {
            return false;
        }

        for i in 1..self.blocks.len() {
            let block = &self.blocks[i];
            let last_block = &self.blocks[i - 1];

            if block.last_hash != last_block.hash {
                return false;
            }

            if block.hash != block.compute_hash() {
                return false;
            }

            if block.difficulty.abs_diff(last_block.difficulty) > 1 {
                return false;
            }
        }

        true
    }
}

impl std::fmt::Debug for Blockchain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut blocks = String::from("");

        for block in &self.blocks {
            blocks.push_str("\n");
            blocks.push_str(&format!("{:?}", block));
        }

        write!(f, "Blockchain {{ {} }}", blocks)
    }
}
