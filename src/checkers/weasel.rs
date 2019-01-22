use super::regex::Regex;
use super::*;

const WEASELS: &str = r"\b(many|various|very|fairly|several|extremely|exceedingly|quite|remarkably|few|surprisingly|mostly|largely|huge|tiny|((are|is) a number)|excellent|interestingly|significantly|substantially|clearly|vast|relatively|completely)\b";

pub struct Weasel {}

impl Checker for Weasel {
    fn check(file_name: &str, file_content: Vec<String>) {
        print_header("WEASEL", file_name);

        let re = Regex::new(WEASELS).unwrap();
        for (idx, line) in file_content.iter().enumerate() {
            let mut lm = LineMatches::new(idx + 1, line);
            for mat in re.find_iter(&line) {
                lm.add_match((mat.start(), mat.end()));
            }
            lm.print()
        }
    }
}
