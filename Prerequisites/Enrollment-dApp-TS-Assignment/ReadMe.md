# Enrollment dApp (TypeScript Assignment) 
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
    "version": "0.1.0",
    "name": "wba_prereq",
    "metadata": {
        
        "address": "HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1" 
    },
    "instructions": [
      {
        "name": "complete",
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
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
        "accounts": [
          {
            "name": "signer",
            "isMut": true,
            "isSigner": true
          },
          {
            "name": "prereq",
            "isMut": true,
            "isSigner": false
          },
          {
            "name": "systemProgram",
            "isMut": false,
            "isSigner": false
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
        "name": "PrereqAccount",
        "type": {
          "kind": "struct",
          "fields": [
            {
              "name": "github",
              "type": "bytes"
            },
            {
              "name": "key",
              "type": "publicKey"
            }
          ]
        }
      }
    ],
    "errors": [
      {
        "code": 6000,
        "name": "InvalidGithubAccount",
        "msg": "Invalid Github account"
      }
    ]
  }
  
  ```
- You can get IDL from this address: `HC2oqz2p6DEWfrahenqdq2moUcga9c9biqRBcdK3XKU1`
- It is needed for interacting programs on `Solana` Blockchain

# Submitting Our Github Username 
- We will interact with `Turbin3 Program(IDL)` and submit our `Github Username` to this contract.

