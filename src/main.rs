use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let path = "./sample.md";
    // let path = "./README.md";

    let lines = match read_lines(path) {
        Ok(lines) => lines,
        Err(why) => panic!("Couldn't open {}: {}", path, why),
    };

    print!("\n{}", parse_document(lines));
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone, Debug)]
enum BlockSyntax {
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
    syntax: BlockSyntax,
    content: String,
}

impl Block {
    fn push_str(&mut self, s: &str) {
        self.content += s;
    }
}

fn parse_document(lines: io::Lines<io::BufReader<File>>) -> String {
    let mut document = String::from("");
    for ln in lines {   // TODO prev_line
        let reader = LineReader::new();
        let parsed_line = match ln {
            Ok(ln) => {document += &ln; reader.parse(&ln)},
            Err(why) => panic!("Couldn't read the file: {}", why),
        };
        println!("{:?}", parsed_line);
        document += "\n";
    }
    return document;
}

enum BufferState {
    Word,
    Space,
}

struct Buffer(String, BufferState);

type B = BufferState;

impl Buffer {
    pub fn new() -> Buffer {
        return Buffer(String::new(), B::Space);
    }

    pub fn switch(&mut self) -> &mut Buffer {
        self.1 = match self.1 {
            B::Word => B::Space,
            B::Space => B::Word,
        };
        return self;
    }

    pub fn clear(&mut self) -> &mut Buffer {
        self.1 = B::Space;
        return self.flush();
    }

    pub fn flush(&mut self) -> &mut Buffer {
        self.0.clear();
        return self;
    }

    pub fn push(&mut self, c: char) -> &mut Buffer {
        self.0.push(c);
        return self;
    }

    pub fn get(&self) -> &str {
        return &self.0;
    }

    pub fn state(&self) -> &BufferState {
        return &self.1;
    }
}

struct LineReader {
    syntax: Option<BlockSyntax>,
    content: String,
    buf: Buffer
}

impl LineReader {
    pub fn new() -> LineReader {
        return LineReader {
            syntax: None,
            content: String::new(),
            buf: Buffer::new(),
        }
    }

    pub fn parse(mut self, ln: &str) -> Block {
        for c in ln.chars() {
            if let Some(_) = &self.syntax {
                match (self.buf.state(), c) {
                    (B::Word, ' ') => {
                        self.write();
                        self.buf.flush().switch();
                    },
                    (B::Word, _) | (B::Space, ' ') => { self.buf.push(c); },
                    (B::Space, _) => {
                        self.write();
                        self.buf.flush().switch().push(c);
                    },
                }
            } else {
                match (self.buf.state(), c) {
                    (B::Word, ' ') => {
                        self.syntax = self.parse_syntax(self.buf.get());
                        if let Some(BlockSyntax::P) = &self.syntax {
                            self.write();
                        }
                        self.buf.flush().switch();
                    },
                    (B::Word, _) | (B::Space, ' ') => { self.buf.push(c); },
                    (B::Space, _) => { self.buf.flush().switch().push(c); },
                }
            }
        }

        if let B::Word = self.buf.state() {
            self.write();
        }

        if let None = &self.syntax {
            self.syntax = Some(BlockSyntax::P);
        }

        let block = Block {
            syntax: self.syntax.unwrap(),
            content: self.content,
        };

        return block;
    }

    fn parse_syntax(&self, buf: &str) -> Option<BlockSyntax> {
        return if let Some(c) = buf.chars().nth(0) {
            match c {
                '#' => Some(self.check_header(buf)),
                _ =>  Some(BlockSyntax::P),
            }
        } else {
            Some(BlockSyntax::P)
        };
    }

    fn check_header(&self, buf: &str) -> BlockSyntax {
        let mut level = 0;
        for c in buf.chars() {
            match (level, c) {
                (0..=5, '#') => level += 1,
                _ => return BlockSyntax::P,
            }
        }
        return BlockSyntax::H(level);
    }

    fn flush(&mut self) {
        self.syntax = None;
        self.content.clear();
        self.buf.clear();
    }

    fn write(&mut self) {
        match self.buf.state() {
            B::Word => self.content += self.buf.get(),
            B::Space => if !self.content.is_empty() { self.content += " "; },
        };
    }
}