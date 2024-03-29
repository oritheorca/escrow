/// Defines state objects that the processor can use
/// Serializes and deserializes state objects from / into arrays of u8

use solana_program::{
  program_pack::{IsInitialized, Pack, Sealed},
  program_error::ProgramError,
  pubkey::Pubkey
};

// It's the program's responsibility to check that received accounts == expected accounts
pub struct Escrow {
  pub is_initialized: bool,
  pub initializer_pubkey: Pubkey,
  pub temp_token_account_pubkey: Pubkey,
  pub initializer_token_to_receive_account_pubkey: Pubkey,
  pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}
