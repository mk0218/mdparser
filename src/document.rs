#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Syntax {
    H(u8),
    P,
    // Ul,
    // Ol(u32),
    // Quote,
    // Code,
    // Hr,
}

#[derive(Debug)]
struct Block {
    syntax: Syntax,
    content: String,
}

#[derive(Debug)]
pub struct Document(Vec<Block>);

impl Document {
    pub fn new() -> Document {
        Document(vec![])
    }

    pub fn add(&mut self, syntax: Syntax, content: String) {
        self.0.push(Block {
            syntax: syntax,
            content: content,
        });
    }

    pub fn print(&self) {
        for block in &self.0 {
            println!("{:?}", block);
        }
    }
}
