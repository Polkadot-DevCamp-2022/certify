#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::{Hash, Environment};
use ink_lang as ink;

#[ink::chain_extension]
pub trait CertifyExtension {
    type ErrorCode = ContractError;

    #[ink(extension = 1)]
	fn issue_in_runtime(key: Hash) -> Result<Hash, ContractError>;

    #[ink(extension = 2)]
	fn revoke_in_runtime(key: Hash) -> Result<Hash, ContractError>;

}

#[derive(Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    UnknownStatusCode,
    InvalidScaleEncoding,
}

impl From<scale::Error> for ContractError {
	fn from(_: scale::Error) -> Self {
		ContractError::InvalidScaleEncoding
	}
}

impl ink_env::chain_extension::FromStatusCode for ContractError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
			0 => Ok(()),
			_ => Err(Self::UnknownStatusCode),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = CertifyExtension;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod org_contract {
    use super::ContractError;

    #[ink(storage)]
    pub struct OrgContract {
        name: Hash,
    }

    #[ink(event)]
    pub struct IssuedByContract {
        #[ink(topic)]
        value: Hash,
        #[ink(topic)]
        account_id:AccountId
    }

    #[ink(event)]
    pub struct RevokedByContract {
        #[ink(topic)]
        value: Hash,
        #[ink(topic)]
        account_id:AccountId
    }

    #[ink(event)]
    pub struct NameChanged {
        #[ink(topic)]
        name: Hash,
    }

    impl OrgContract {
        #[ink(constructor)]
        pub fn new(name: Hash) -> Self {
            Self { name: name }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn get(&self) -> Hash {
            self.name
        }

        #[ink(message)]
        pub fn set(&mut self, value: Hash) -> Result<(), ContractError> {
            self.name = value;
            self.env().emit_event(NameChanged { name: value });

            Ok(())
        }

		#[ink(message)]
		pub fn issue(&mut self, value: Hash) -> Result<(), ContractError> {
            let caller = self.env().caller();

			self.env().extension().issue_in_runtime(value)?;
            self.env().emit_event(IssuedByContract { value: value, account_id: caller });

			Ok(())
		}

        #[ink(message)]
		pub fn revoke(&mut self, value: Hash) -> Result<(), ContractError> {
            let caller = self.env().caller();

			self.env().extension().revoke_in_runtime(value)?;
            self.env().emit_event(RevokedByContract { value: value, account_id: caller });

			Ok(())
		}
        

    }

}
