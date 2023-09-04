use crate::utils::get_lines;

pub fn answer_part_1() -> u64 {
    let mut result = 0;
    let mut stack = Vec::new();

    for line in get_lines("input/day_07.txt").map(Result::unwrap) {
        let tokens = tokenize(&line);
        let p = Line::parse(&tokens);
        match p {
            Line::Command(cmd) => match cmd {
                Command::CdInto { target: _ } => stack.push(0),
                Command::CdOut => {
                    let current = stack.pop().unwrap();
                    if current <= 100_000 {
                        result += current;
                    }
                    if let Some(parent) = stack.last_mut() {
                        *parent += current;
                    }
                }
                Command::Ls => (),
            },
            Line::Node(node) => match node {
                Node::Dir { name: _ } => (),
                Node::File { name: _, size } => *stack.last_mut().unwrap() += size,
            },
        }
    }

    while let Some(n) = stack.pop() {
        if n <= 100_000 {
            result += n;
        }
    }

    result
}

#[derive(Debug)]
enum Token {
    Dollar,
    Cd,
    Ls,
    Root,
    Up,
    Dir,
    Value(String),
}

fn tokenize(s: &str) -> Vec<Token> {
    s.split_whitespace()
        .map(|s| match s {
            "$" => Token::Dollar,
            "cd" => Token::Cd,
            "ls" => Token::Ls,
            "/" => Token::Root,
            ".." => Token::Up,
            "dir" => Token::Dir,
            val => Token::Value(val.to_owned()),
        })
        .collect()
}

enum Line {
    Command(Command),
    Node(Node),
}

impl Line {
    fn parse(tokens: &[Token]) -> Self {
        if let Ok(cmd) = Command::parse(tokens) {
            Self::Command(cmd)
        } else if let Ok(node) = Node::parse(tokens) {
            Self::Node(node)
        } else {
            panic!("unparseable line: {tokens:?}")
        }
    }
}

enum Command {
    CdInto { target: String },
    CdOut,
    Ls,
}

impl Command {
    fn parse(tokens: &[Token]) -> Result<Self, CommandParseError> {
        if let Some(Token::Dollar) = tokens.first() {
            match (tokens.get(1), tokens.get(2)) {
                (Some(Token::Cd), Some(Token::Value(t))) => Ok(Self::CdInto {
                    target: t.to_owned(),
                }),
                (Some(Token::Cd), Some(Token::Up)) => Ok(Self::CdOut),
                (Some(Token::Cd), Some(Token::Root)) => Ok(Self::CdInto {
                    target: "ROOT".to_owned(),
                }),
                (Some(Token::Ls), None) => Ok(Self::Ls),
                _ => Err(CommandParseError::InvalidCommand),
            }
        } else {
            Err(CommandParseError::NotCommand)
        }
    }
}

enum CommandParseError {
    /// A non-command was encountered
    NotCommand,
    /// An invalid command was encountered
    InvalidCommand,
}

enum Node {
    Dir { name: String },
    File { name: String, size: u64 },
}

impl Node {
    fn parse(tokens: &[Token]) -> Result<Self, NodeParseError> {
        match (tokens.get(0), tokens.get(1)) {
            (Some(Token::Dir), Some(Token::Value(n))) => Ok(Node::Dir { name: n.to_owned() }),
            (Some(Token::Value(s)), Some(Token::Value(n))) => s
                .parse::<u64>()
                .map(|size| Node::File {
                    name: n.to_owned(),
                    size,
                })
                .map_err(|_| NodeParseError::InvalidSize),
            _ => Err(NodeParseError::NotNode),
        }
    }
}

enum NodeParseError {
    /// A non-node was encountered
    NotNode,
    /// Size not parseable as u64
    InvalidSize,
}
