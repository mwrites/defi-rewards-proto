# Example of Rewards Protocol For Educational Purpose

- ☢️ all the validations stuff
- CreateMint idempotent (shouldnt happen in prod but annyoing for tests ..or delete mint before running tests)
- update errors.rs
- bubble error up, currently rust errors appears in log, but front-end get ({"err":{"InstructionError":[0,{"Custom":0}]}})
- https://solanacookbook.com/references/programs.html#how-to-verify-accounts


check cluster version, this tutorial has been made for 1.14.4
`solana cluster-version -u devnet`