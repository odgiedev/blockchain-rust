mod blockchain;
use blockchain::Block;

fn main() {
    let gen_block = Block::new(0, String::from("Genesis Block"), String::from("0"));
    println!("Genesis Block: {:?}", gen_block);
    
    let block1 = Block::new(1, String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit.xs"), gen_block.hash.clone());
    println!("Block 1: {:?}", block1);
    
    let block2 = Block::new(2, String::from("Proin vehicula tincidunt pretium."), block1.hash.clone());
    println!("Block 2: {:?}", block2);
}
