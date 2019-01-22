use super::colored::*;
use super::*;

pub struct LineMatches<'a> {
    pub line: &'a String,
    pub line_idx: usize,
    pub matches: Vec<(usize, usize)>,
}

impl<'a> LineMatches<'a> {
    pub fn new(line_idx: usize, line: &'a String) -> Self {
        LineMatches {
            line_idx: line_idx,
            line: line,
            matches: Vec::new(),
        }
    }

    pub fn add_match(&mut self, mat: (usize, usize)) {
        self.matches.push(mat);
    }

    pub fn print(&self) {
        if self.matches.len() == 0 {
            return;
        }

        print!("{}: ", self.line_idx);
        let mut last_end: usize = 0;
        for (start, end) in self.matches.clone() {
            print!(
                "{}{}",
                self.line.get(last_end..start).unwrap(),
                self.line.get(start..end).unwrap().red(),
            );
            last_end = end;
        }
        println!("{}", self.line.get(last_end..self.line.len()).unwrap());
    }
}

fn print_header(check_id: String, file_name: &str) {
    println!(
        "\nChecking {} for file: {}\n",
        check_id.bold(),
        file_name.underline(),
    );
}

fn print_no_matches_found() {
    println!("{}", "No problems found!".green())
}

pub trait Checker {
    fn name(&self) -> String;
    fn matches<'a>(&self, file_content: &'a Vec<String>) -> Vec<LineMatches<'a>>;
}

pub struct Checkers {
    checkers: Vec<Box<Checker>>,
}

impl Checkers {
    pub fn new() -> Checkers {
        Checkers {
            checkers: vec![
                Box::new(Dups::new()),
                Box::new(Passive::new()),
                Box::new(Weasel::new()),
            ],
        }
    }

    pub fn check_all(&self, file_name: &str, file_content: Vec<String>) {
        for checker in &self.checkers {
            print_header(checker.name(), file_name);

            let matches = checker.matches(&file_content);
            if matches.len() == 0 {
                print_no_matches_found();
            } else {
                for mat in matches {
                    mat.print()
                }
            }
        }
    }
}
