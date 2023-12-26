use std::fmt;

use colored::Colorize;
use console::{style, Style};

use similar::{ChangeTag, TextDiff};
struct Line(Option<usize>);

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(idx) => write!(f, "{:<4}", idx + 1),
        }
    }
}

pub struct Difference;

impl Difference {
    #[allow(dead_code)]
    pub fn is_same(expect: &str, result: &str) -> bool {
        let diff = TextDiff::from_lines(expect, result);
        if diff.ratio() != 1.0 {
            println!(
                "Expected Answer:\n{}\n\nGot Answer:\n{}\n",
                expect.dimmed(),
                result.dimmed()
            );
            println!("Difference(Expected & Got):");
        }
        for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
            if idx > 0 {
                println!("{:-^1$}", "-", 80);
            }
            for op in group {
                for change in diff.iter_inline_changes(op) {
                    let (sign, s) = match change.tag() {
                        ChangeTag::Delete => ("-", Style::new().red()),
                        ChangeTag::Insert => ("+", Style::new().green()),
                        ChangeTag::Equal => (" ", Style::new().dim()),
                    };
                    print!(
                        "{}{} |{}",
                        style(Line(change.old_index())).dim(),
                        style(Line(change.new_index())).dim(),
                        s.apply_to(sign).bold(),
                    );
                    for (emphasized, value) in change.iter_strings_lossy() {
                        if emphasized {
                            print!("{}", s.apply_to(value).underlined().bold());
                        } else {
                            print!("{}", s.apply_to(value));
                        }
                    }
                    if change.missing_newline() {
                        println!();
                    }
                }
            }
        }
        return diff.ratio() == 1.0;
    }
}

#[test]
fn test_get_diff() {
    let output = "Hello World\nThis is the second line.\nThis is the third.";
    let expect = "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more";
    let same = Difference::is_same(output, expect);
    assert_ne!(same, true);
}
