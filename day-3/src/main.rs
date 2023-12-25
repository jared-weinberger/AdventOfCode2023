use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let stream = BufReader::new(File::open(".input-data/input.txt").unwrap());
    let table = parse_input(stream.lines().flatten().take(3));
    for row in table {
        for val in row {
            print!("{}", val);
        }
        println!();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Find {
    Number { val: u64, run: Run },
    Symbol,
    Empty,
}
impl Display for Find {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rep = match &self {
            &Self::Empty => "_".to_string(),
            &Self::Symbol => "#".to_string(),
            &Self::Number { val, run: _ } => format!("{}", val),
        };
        write!(f, "{}", rep)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Run {
    start: usize,
    length: usize,
}

impl Run {
    pub fn new(start: usize, length: usize) -> Self {
        Self {
            start,
            length: length,
        }
    }
    pub fn start(start: usize) -> Self {
        Self::new(start, 1)
    }
    pub fn get_slice<'my, 'other, T, X>(&'my self, sliceable: &'other T) -> &'other X
    where
        X: ?Sized,
        T: std::ops::Index<std::ops::Range<usize>, Output = X>,
    {
        &sliceable[self.start..self.start + self.length]
    }
    pub fn extend_one(&mut self) {
        self.length += 1;
    }
}

fn parse_input<L>(lines: L) -> Vec<Vec<Find>>
where
    L: Iterator<Item = String>,
{
    let mut table: Vec<Vec<Find>> = Vec::new();
    for line in lines {
        let mut row: Vec<Find> = Vec::new();
        let mut maybe_run: Option<Run> = None;
        for (index, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                if maybe_run.is_none() {
                    maybe_run = Some(Run::start(index));
                } else {
                    maybe_run.as_mut().unwrap().extend_one();
                }
            } else {
                if maybe_run.is_some() {
                    let run = maybe_run.unwrap();
                    let val = run.get_slice(&line).parse().unwrap();
                    row.push(Find::Number { val, run });
                    maybe_run = None;
                }
                if character == '.' {
                    row.push(Find::Empty);
                } else if character.is_ascii_punctuation() {
                    row.push(Find::Symbol);
                }
            }
        }
        if row.len() > 0 {
            table.push(row);
        }
    }
    table
}
