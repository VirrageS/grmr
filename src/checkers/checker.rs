use super::colored::*;

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

pub trait Checker {
    fn check(file_name: &str, file_content: Vec<String>);
}

pub fn print_header(check_id: &str, file_name: &str) {
    println!(
        "Checking {} for file: {}\n",
        check_id.bold(),
        file_name.green(),
    );
}
