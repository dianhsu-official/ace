# ACE

**Deprecated due to Codeforces protected by CloudFlare, I am trying to use Competitive Companion finish parse process.**
**Please take a look at [ccs](https://github.com/dianhsu-official/ccs) for current status.**

[![ACE](https://github.com/dianhsu/ace/actions/workflows/test.yml/badge.svg)](https://github.com/dianhsu/ace/actions/workflows/test.yml)


ACE is a command-line interface tool for Algorithm Contest, just like: Codeforces, Atcoder, etc.

[Installation](#installation) | [Usage](#usage) | [Current developing progress](#current-developing-progress) | [Snippets](#snippets)

## Features
- Support AtCoder and Codeforces.
- Support Multiple accounts for these platforms.
- Same experience with cf-tool (Partial right now).



## Installation

For now, download latest binary from `Github Actions`, find binary file from Artifacts which match your system.

Github Action Url: https://github.com/dianhsu/ace/actions/workflows/release.yml

## Usage

**Manage accounts for atcoder or codeforces**

`ace account` 

```
Manage account for ace, such as add, remove, list

Usage: ace.exe account [OPTIONS] <COMMAND>

Commands:
  add          Create a new account
  list         List all accounts
  set-default  Set default account
  update       Update account password
  delete       Remove account
  help         Print this message or the help of the given subcommand(s)

Options:
  -p, --platform <PLATFORM>
  -h, --help                 Print help
```

**Manage submit language, code template and execute scripts for atcoder or codeforces**

`ace lang`

```
Manage language for ace, such as set, list

Usage: ace.exe lang <COMMAND>

Commands:
  list         List all language config
  add          Add language config
  delete       Delete language config
  set-default  Set default language
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

**Parse contest from atcoder or codeforces**

`ace parse`

e.g. 

`ace parse cf 1888`: parse codeforces contest of 1880

`ace parse atc abc321`: parse atcoder contest of abc321


**Generate code file from template**

`ace gen`

This command should run in contest directory.

**Test my code locally**

`ace test`

This command should run in contest directory.

**Submit my code to atcoder or codeforces**

`ace submit`

This command should run in contest directory.

----------------
## Current developing progress

### Commands


| Command     | Description                                       | Progress   |
| ----------- | ------------------------------------------------- | ---------- |
| ace account | user management                                   | ✅          |
| ace config  | config management                                 | ✅          |
| ace lang    | config command and template for specific language | ✅          |
| ace parse   | get contest info                                  | ✅          |
| ace gen     | generate code from template                       | ✅          |
| ace submit  | submit code                                       | ✅          |
| ace test    | local run test                                    | ✅          |
| *ace race   | start race                                        | Pending    |
| *ace debug  | start debug file                                  | Scheduling |

### Snippets

You can insert some snippets into your template code or command. When generate a code from template or execute your command, ace will replace all snippets by following rules.

| snippet         | description                                     | e.g.                          | Capability (code/command) |
| --------------- | ----------------------------------------------- | ----------------------------- | ------------------------- |
| `%$platform$%`  | target platform                                 | `codeforces`                  | ✅/️✅                       |
| `%$pid$%`       | problem identifier                              | `1848_A`                      | ✅/️✅                       |
| `%$cid$%`       | contest identifier                              | `1848`                        | ✅/️✅                       |
| `%$workspace$%` | current directory                               | `/home/dianhsu/ace/cf/1848_A` | ✅/✅                       |
| `%$full$%`      | full name of source file                        | `main.cpp`                    | ✅/✅                       |
| `%$rand$%`      | ramdom string with 8 character(`^[a-z0-9]{8}$`) | `a1b2c3d4`                    | ✅/✅                       |
| `%$Y$%`         | Year                                            | `2023`                        | ✅/✅                       |
| `%$M$%`         | Month                                           | `01`                          | ✅/✅                       |
| `%$D$%`         | Day                                             | `01`                          | ✅/✅                       |
| `%$h$%`         | Hour                                            | `23`                          | ✅/✅                       |
| `%$m$%`         | Minute                                          | `04`                          | ✅/✅                       |
| `%$s$%`         | Second                                          | `00`                          | ✅/✅                       |
