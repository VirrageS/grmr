use super::regex::Regex;
use super::*;

pub struct Dups {}

impl Dups {
    fn annotate_words(line: &str) -> Vec<(usize, &str)> {
        line.split(" ")
            .scan(0, |acc, word| {
                let res = Some((*acc, word));
                *acc = *acc + word.len() + 1;
                res
            })
            .filter(|(_, word)| !word.is_empty())
            .collect::<Vec<(usize, &str)>>()
    }
}

impl Checker for Dups {
    fn check(file_name: &str, file_content: Vec<String>) {
        let re = Regex::new(r"(\W+)").unwrap();
        print_header("DUPS", file_name);

        for (line_idx, line) in file_content.iter().enumerate() {
            let mut last_word = (0, "");

            let words = Dups::annotate_words(line);
            for (cursor_pos, word) in words {
                let mut lm = LineMatches::new(line_idx + 1, line);
                if last_word.1 == word && !re.is_match(word) {
                    lm.add_match((last_word.0, cursor_pos));
                }
                lm.print();
                last_word = (cursor_pos, word);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_word_annotation() {
        let line = "large fox jumping over";
        let words = Dups::annotate_words(line);
        assert_eq!(words, vec![(0, "large"), (6, "fox"), (10, "jumping"), (18, "over")]);
    }

    #[test]
    fn word_annotation_empty_words() {
        let line = "large    fox";
        let words = Dups::annotate_words(line);
        assert_eq!(words, vec![(0, "large"), (9, "fox")]);
    }
}
