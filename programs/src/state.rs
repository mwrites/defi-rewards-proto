// use borsh::{BorshDeserialize, BorshSerialize};
//
// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
// pub struct AuthorizedBufferHeader {
//     // TODO
// }
//
// #[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
// pub struct VendingMachineBufferHeader {
//     // TODO
// }
//

// all of these should live in an account to be configurable?
pub const MINT_SEED: &[u8] = b"beef_mint";
pub const MINT_DECIMALS: u8 = 8;
pub const MINT_AMOUNT: u64 = 42;
