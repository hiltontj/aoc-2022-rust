use crate::utils::get_lines;

const TOTAL_FS_SPACE: u64 = 70_000_000;
const DESIRED_SPACE: u64 = 30_000_000;

pub fn answer_part_1() -> (u64, u64) {
    let mut result = 0;
    let mut used_space = 0;
    let mut stack = Vec::new();

    for line in get_lines("input/day_07.txt").map(Result::unwrap) {
        let tokens = tokenize(&line);
        let p = Line::parse(&tokens);
        match p {
            Line::Command(cmd) => match cmd {
                Command::CdInto => stack.push(0),
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
                Node::Dir => (),
                Node::File { size } => *stack.last_mut().unwrap() += size,
            },
        }
    }

    while let Some(n) = stack.pop() {
        if n <= 100_000 {
            result += n;
        }
        used_space += n;
    }

    (result, used_space)
}

pub fn answer_part_2() -> u64 {
    let (_, used_space) = answer_part_1();
    let unused_space = TOTAL_FS_SPACE - used_space;
    let min_size = DESIRED_SPACE - unused_space;
    let mut result = u64::MAX;
    let mut stack = Vec::new();
    for line in get_lines("input/day_07.txt").map(Result::unwrap) {
        let tokens = tokenize(&line);
        let p = Line::parse(&tokens);
        match p {
            Line::Command(cmd) => match cmd {
                Command::CdInto => stack.push(0),
                Command::CdOut => {
                    let current = stack.pop().unwrap();
                    if current >= min_size {
                        result = result.min(current);
                    }
                    if let Some(parent) = stack.last_mut() {
                        *parent += current;
                    }
                }
                Command::Ls => (),
            },
            Line::Node(node) => match node {
                Node::Dir => (),
                Node::File { size } => *stack.last_mut().unwrap() += size,
            },
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
    CdInto,
    CdOut,
    Ls,
}

impl Command {
    fn parse(tokens: &[Token]) -> Result<Self, CommandParseError> {
        if let Some(Token::Dollar) = tokens.first() {
            match (tokens.get(1), tokens.get(2)) {
                (Some(Token::Cd), Some(Token::Value(_))) => Ok(Self::CdInto),
                (Some(Token::Cd), Some(Token::Up)) => Ok(Self::CdOut),
                (Some(Token::Cd), Some(Token::Root)) => Ok(Self::CdInto),
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
    Dir,
    File { size: u64 },
}

impl Node {
    fn parse(tokens: &[Token]) -> Result<Self, NodeParseError> {
        match (tokens.get(0), tokens.get(1)) {
            (Some(Token::Dir), Some(Token::Value(_))) => Ok(Node::Dir),
            (Some(Token::Value(s)), Some(Token::Value(_))) => s
                .parse::<u64>()
                .map(|size| Node::File { size })
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
