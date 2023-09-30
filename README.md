# ACE
> Developing.

[![Rust](https://github.com/dianhsu/ace/actions/workflows/test.yml/badge.svg)](https://github.com/dianhsu/ace/actions/workflows/test.yml)

ACE is a command-line interface tool for Algorithm Contest, just like: Codeforces, Atcoder, etc.

## Current Progress

### Platform Spec

|                  | Atcoder | Codeforces |
| ---------------- | ------- | ---------- |
| Check login      | ✅       | ✅          |
| Login            | ✅       | ✅          |
| Import cookies   | ✅       | ✅          |
| Save cookies     | ✅       | ✅          |
| Submit code      | ✅       | ✅          |
| Retrieve result  | ✅       | ✅          |
| Get problem list | ✅       | ✅          |
| Get test cases   | ✅       | ✅          |
| Get contest      | ✅       | ✅          |


## Commands

###  Generic 

| Command     | Description                                       | Progress   |
| ----------- | ------------------------------------------------- | ---------- |
| ace account | user management                                   | ✅          |
| ace config  | config management                                 | ✅          |
| ace lang    | config command and template for specific language | ✅          |
| ace parse   | get contest info                                  | ✅          |
| ace gen     | generate code from template                       | ✅          |
| ace submit  | submit code                                       | Processing |
| ace test    | local run test                                    | Pending    |
| *ace race   | start race                                        | Pending    |
| *ace debug  | start debug file                                  | Scheduling |
## Snippets

You can insert some snippets into your template code or command. When generate a code from template or execute your command, ace will replace all snippets by following rules.

| snippet         | description                                     | e.g.                          | Capability (code/command) |
| --------------- | ----------------------------------------------- | ----------------------------- | ------------------------- |
| `%$platform$%`  | target platform                                 | `codeforces`                  | ✅/⚠️                       |
| `%$pid$%`       | problem identifier                              | `1848_A`                      | ✅/⚠️                       |
| `%$cid$%`       | contest identifier                              | `1848`                        | ✅/⚠️                       |
| `%$workspace$%` | current directory                               | `/home/dianhsu/ace/cf/1848_A` | ✅/✅                       |
| `%$full$%`      | full name of source file                        | `main.cpp`                    | ✅/✅                       |
| `%$rand$%`      | ramdom string with 8 character(`^[a-z0-9]{8}$`) | `a1b2c3d4`                    | ✅/✅                       |
| `%$Y$%`         | Year                                            | `2023`                        | ✅/✅                       |
| `%$M$%`         | Month                                           | `01`                          | ✅/✅                       |
| `%$D$%`         | Day                                             | `01`                          | ✅/✅                       |
| `%$h$%`         | Hour                                            | `23`                          | ✅/✅                       |
| `%$m$%`         | Minute                                          | `04`                          | ✅/✅                       |
| `%$s$%`         | Second                                          | `00`                          | ✅/✅                       |