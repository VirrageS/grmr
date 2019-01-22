extern crate colored;
extern crate regex;

pub use self::checker::*;
pub use self::dups::Dups;
pub use self::passive::Passive;
pub use self::weasel::Weasel;

mod checker;
mod dups;
mod passive;
mod weasel;
