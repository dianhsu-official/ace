use lazy_static::lazy_static;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct CodeTemplate {
    pub alias: String,
    pub lang: String,
    pub path: String,
    pub suffix: String,
    pub before_script: String,
    pub script: String,
    pub after_script: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Platform {
    Codeforces,
    Atcoder,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Platform::Codeforces => "codeforces",
            Platform::Atcoder => "atcoder",
        })
    }
}
lazy_static! {
    pub static ref PLATFORM_MAP: HashMap<&'static str, Platform> = HashMap::from([
        ("cf", Platform::Codeforces),
        ("codeforces", Platform::Codeforces),
        ("at", Platform::Atcoder),
        ("atc", Platform::Atcoder),
        ("ac", Platform::Atcoder),
        ("atcoder", Platform::Atcoder),
    ]);
}
pub enum ProgramLanguage {
    C,
    Cpp,
    Java,
    Python,
    Rust,
    Kotlin,
    Go,
    CSharp,
    D,
    Ruby,
    Scala,
    Haskell,
    Perl,
    Php,
    OCaml,
    Delphi,
    Pascal,
    Lua,
    JavaScript,
    TypeScript,
    FSharp,
    QSharp,
    Swift,
    R,
    Julia,
    Tcl,
    Io,
    Factor,
    Text,
    Zsh,
    Bash,
    PowerShell,
    Cobol,
    Awk,
    Nim,
    Vim,
    V,
    Zig,
    Dart,
    VisualBasic,
    Fortran,
    Lisp,
    Erlang,
}
