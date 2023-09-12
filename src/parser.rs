use std::fs::File;
use std::io::{self};

use crate::document::Document;
use line_parser::Parser as LineParser;


pub struct MDParser;

type Lines = io::Lines<io::BufReader<File>>;

impl MDParser {
    pub fn parse(lines: Lines) -> Result<Document, String> {
        let mut document = Document::new();
    
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(why) => panic!("Couldn't read the file: {}", why),
            };
            let (syntax, content) = LineParser::parse(&line);
            document.add(syntax, content);
        }
    
        Ok(document)
    }
}

mod line_parser {
    use crate::document::Syntax;

    pub struct Parser {
        syntax: Option<Syntax>,
        content: String,
        buffer: Buffer,
    }
    
    impl Parser {
        pub fn parse(line: &str) -> (Syntax, String) {
            let mut parser = Parser {
                syntax: None,
                content: String::new(),
                buffer: Buffer::new(),
            };

            for c in line.chars() {
                match (parser.buffer.state(), c) {
                    (B::Word, ' ') => {
                        if let Some(_) = &parser.syntax {
                            parser.write();
                            parser.buffer.switch();
                        } else {
                            let syntax = parser.get_syntax(parser.buffer.content());
                            if syntax == Syntax::P { parser.write(); }
                            parser.syntax = Some(syntax);
                            parser.buffer.switch();
                        }
                    },
                    (B::Word, _) | (B::Space, ' ') => parser.buffer.push(c),
                    (B::Space, _ ) => {
                        parser.write();
                        parser.buffer.switch();
                        parser.buffer.push(c);
                    },
                };
            }

            if let B::Word = parser.buffer.state() {
                parser.write();
            }

            if let Some(syntax) = parser.syntax {
                (syntax, parser.content)
            } else {
                (Syntax::P, parser.content)
            }
        }

        fn write(&mut self) {
            match self.buffer.state() {
                B::Word => self.content += self.buffer.content(),
                B::Space => if self.content.len() > 0 {
                    self.content += " ";
                },
            };
            self.buffer.flush();
        }

        fn get_syntax(&self, s: &str) -> Syntax {
            match s.chars().nth(0).unwrap() {
                '#' => self.check_header(s),
                _ => Syntax::P,
            }
        }

        fn check_header(&self, s: &str) -> Syntax {
            let mut level = 0;
            for c in s.chars() {
                match (level, c) {
                    (0..=5, '#') => level += 1,
                    _ => return Syntax::P,
                }
            }
            Syntax::H(level)
        }
    }

    struct Buffer(String, BufferState);

    enum BufferState {
        Word,
        Space,
    }

    type B = BufferState;
    
    impl Buffer {
        fn new() -> Buffer {
            Buffer(String::new(), B::Space)
        }

        fn content(&self) -> &str {
            &self.0
        }

        fn state(&self) -> &BufferState {
            &self.1
        }

        fn push(&mut self, c: char) {
            self.0.push(c);
        }
    
        fn switch(&mut self) -> &mut Self {
            self.1 = match self.1 {
                B::Word => B::Space,
                B::Space => B::Word,
            }; self
        }
    
        fn flush(&mut self) -> &mut Self {
            self.0.clear(); self
        }
    }
}
