# Enrollment dApp (Rust Assignment) 
This is a Turbin3 Cohort 2025 Prequisite Assignemnt. Task is to follow all the instruction carefully to achieve:
 - Creation of Solana Wallet using `@solana/web3` library.
 - Since wallets like `Phantom` uses `Base58` as `Private Key` to import wallets. We have to convert our `Private Key ` from `Bytes Array` to `Base58`.
 - Creating tests using  `Rust` in order to test whether conversion between `Base58` to `Bytes Array` and vice versa.
 - Airdropping some `Sol` to our `Wallet` on `Devnet`.
 - Transferring 0.1 `Sol` to `Turbin3 Wallet` address.
 - Transferring All `Sol` to `Turbin3 Wallet` address.


# Getting Turbin3Program(IDL)
```json
{
    "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa",
    "metadata": {
      "name": "wba_prereq",
      "version": "0.1.0",
      "spec": "0.1.0",
      "description": "Created with Anchor"
    },
    "instructions": [
      {
        "name": "complete",
        "discriminator": [
          0,
          77,
          224,
          147,
          136,
          25,
          88,
          76
        ],
        "accounts": [
          {
            "name": "signer",
            "writable": true,
            "signer": true
          },
          {
            "name": "prereq",
            "writable": true,
            "pda": {
              "seeds": [
                {
                  "kind": "const",
                  "value": [
                    112,
                    114,
                    101,
                    114,
                    101,
                    113
                  ]
                },
                {
                  "kind": "account",
                  "path": "signer"
                }
              ]
            }
          },
          {
            "name": "system_program",
            "address": "11111111111111111111111111111111"
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      },
      {
        "name": "update",
        "discriminator": [
          219,
          200,
          88,
          176,
          158,
          63,
          253,
          127
        ],
        "accounts": [
          {
            "name": "signer",
            "writable": true,
            "signer": true
          },
          {
            "name": "prereq",
            "writable": true
          },
          {
            "name": "system_program",
            "address": "11111111111111111111111111111111"
          }
        ],
        "args": [
          {
            "name": "github",
            "type": "bytes"
          }
        ]
      }
    ],
    "accounts": [
      {
        "name": "Q2Prereq2024",
        "discriminator": [
          210,
          203,
          168,
          103,
          251,
          233,
          204,
          6
        ]
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "InvalidGithubAccount",
        "msg": "Invalid Github account"
      }
    ],
    "types": [
      {
        "name": "Q2Prereq2024",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "pubkey"
            }
          ]
        }
      }
    ]
  }
  
  ```
- You can get IDL from this address: `ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa`
- It is needed for interacting programs on `Solana` Blockchain

# Submitting Our Github Username 
- We will interact with `Turbin3 Program(IDL)` and submit our `Github Username` to this contract.

- [TX - ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa](https://explorer.solana.com/tx/39zNX7jzT5XHFxAJoTaqSH3DkJAna6H3z3XYUUCpYQ3aYtVBVTH7pBn8Vi3UwY3Fw4hmKQfWprEGSfBNEokSWi6r?cluster=devnet)
