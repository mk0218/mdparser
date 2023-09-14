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

    pub fn append(&mut self, syntax: Syntax, content: String) {
        type S = Syntax;
        let block = self.0.last_mut().filter(|b| b.syntax == S::P && syntax == S::P);

        if let Some(b) = block {
            b.content.push(' ');
            b.content.push_str(&content);
        } else {
            self.0.push(Block {
                syntax: syntax,
                content: content,
            });
        }
    }

    pub fn print(&self) {
        for block in &self.0 {
            println!("{:?}", block);
        }
    }
}
