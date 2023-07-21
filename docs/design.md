# Project design for ACE

## Overview
The ACE has serval components:
- Platform interfaces.
- Accounts management.
- Command-line arguments parser.
- Http request client and other tools.

## Platform interfaces
Each account has same behavior, for example: login account, grab test cases from website and submit code.

## Accounts management

Data Structure:
```json5
{
    "platforms": [
        {
            "name": "Codeforces", 
            "credentials": [
                "<the serialization string of combination of username, password and cookies>", 
                "", 
                ""
            ],
            "current_credential_index": 0,
            "host": "https://codeforces.com" // no suffix dash 
        },
        {
            "name": "Atcoder", 
            "credentials": ["", "", ""],
            "current_credential_index": 0,
            "host": "https://atcoder.jp"
        }
    ],
    // other configurations 
}
```

## Command-line arguments parser

## Http request client and other tools