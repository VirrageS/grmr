use super::regex::Regex;
use super::*;

const PASSIVE: &str = r"\b(am|are|were|being|is|been|was|be)\b[ ]*(\w+ed|(
awoken|
been|born|beat|
been|born|beat|
become|begun|bent|
beset|bet|bid|
bidden|bound|bitten|
bled|blown|broken|
bred|brought|broadcast|
built|burnt|burst|
bought|cast|caught|
chosen|clung|come|
cost|crept|cut|
dealt|dug|dived|
done|drawn|dreamt|
driven|drunk|eaten|fallen|
fed|felt|fought|found|
fit|fled|flung|flown|
forbidden|forgotten|
foregone|forgiven|
forsaken|frozen|
gotten|given|gone|
ground|grown|hung|
heard|hidden|hit|
held|hurt|kept|knelt|
knit|known|laid|led|
leapt|learnt|left|
lent|let|lain|lighted|
lost|made|meant|met|
misspelt|mistaken|mown|
overcome|overdone|overtaken|
overthrown|paid|pled|proven|
put|quit|read|rid|ridden|
rung|risen|run|sawn|said|
seen|sought|sold|sent|
set|sewn|shaken|shaven|
shorn|shed|shone|shod|
shot|shown|shrunk|shut|
sung|sunk|sat|slept|
slain|slid|slung|slit|
smitten|sown|spoken|sped|
spent|spilt|spun|spit|
split|spread|sprung|stood|
stolen|stuck|stung|stunk|
stridden|struck|strung|
striven|sworn|swept|
swollen|swum|swung|taken|
taught|torn|told|thought|
thrived|thrown|thrust|
trodden|understood|upheld|
upset|woken|worn|woven|
wed|wept|wound|won|
withheld|withstood|wrung|
written
))\b";

pub struct Passive {}

impl Checker for Passive {
    fn check(file_name: &str, file_content: Vec<String>) {
        print_header("PASSIVE", file_name);

        let re = Regex::new(&PASSIVE).unwrap();
        for (idx, line) in file_content.iter().enumerate() {
            let mut lm = LineMatches::new(idx + 1, line);
            for mat in re.find_iter(&line) {
                lm.add_match((mat.start(), mat.end()));
            }
            lm.print()
        }
    }
}
