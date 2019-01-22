use super::regex::Regex;
use super::*;

const WEASELS: &str = r"\b(many|various|very|fairly|several|extremely|exceedingly|quite|remarkably|few|surprisingly|mostly|largely|huge|tiny|((are|is) a number)|excellent|interestingly|significantly|substantially|clearly|vast|relatively|completely)\b";

pub struct Weasel {}

impl Weasel {
    pub fn new() -> Weasel {
        Weasel {}
    }
}

impl Checker for Weasel {
    fn name(&self) -> String {
        "WEASELS".to_owned()
    }

    fn matches<'a>(&self, file_content: &'a Vec<String>) -> Vec<LineMatches<'a>> {
        let re = Regex::new(WEASELS).unwrap();
        let mut matches: Vec<LineMatches> = vec![];
        for (idx, line) in file_content.iter().enumerate() {
            let mut lm = LineMatches::new(idx + 1, line);
            for mat in re.find_iter(&line) {
                lm.add_match((mat.start(), mat.end()));
            }
            matches.push(lm)
        }
        matches
    }
}
