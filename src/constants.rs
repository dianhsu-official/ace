use crate::model::Platform;
use clap::ValueEnum;
use lazy_static::lazy_static;
use std::collections::HashMap;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;
lazy_static! {
    pub static ref PLATFORMS: Vec<Platform> =
        Vec::from(vec![Platform::Codeforces, Platform::Atcoder]);
    pub static ref PLATFORM_MAP: HashMap<&'static str, Platform> = HashMap::from([
        ("cf", Platform::Codeforces),
        ("codeforces", Platform::Codeforces),
        ("at", Platform::Atcoder),
        ("atc", Platform::Atcoder),
        ("ac", Platform::Atcoder),
        ("atcoder", Platform::Atcoder),
    ]);
}
#[allow(dead_code)]
#[derive(Debug, EnumIter, Display, Copy, Clone, ValueEnum, EnumString)]
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
