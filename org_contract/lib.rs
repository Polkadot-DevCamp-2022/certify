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
    NoPermission,
    NotAMember,
    AlreadyAMember,
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

    use ink_lang::utils::initialize_contract;
    use ink_storage::{
            traits::{
                SpreadAllocate,
            },
            Mapping,
        };

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct OrgContract {
        name: Hash,
        owner: AccountId,
        members: Mapping<AccountId, bool>
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

    #[ink(event)]
    pub struct AddMember {
        #[ink(topic)]
        account_id:AccountId
    }

    #[ink(event)]
    pub struct RemoveMember {
        #[ink(topic)]
        account_id:AccountId
    }


    impl OrgContract {
        #[ink(constructor)]
        pub fn new(name: Hash) -> Self {

            initialize_contract(|contract: &mut Self| {
                let caller = Self::env().caller();
                contract.name = name;
                contract.owner = caller;
                contract.members.insert(&caller, &true)
            })
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
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn is_member(&self, account_id: AccountId) -> bool {
            self.members.get(&account_id).unwrap_or_default()
        }

        #[ink(message)]
        pub fn add_member(& mut self, account_id: AccountId) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let owner = self.owner;
            if caller != owner {
                return Err(ContractError::NoPermission)
            }
            if self.members.contains(&account_id) {
                return Err(ContractError::AlreadyAMember)
            }
            
            self.members.insert(&account_id, &true);
            self.env().emit_event(AddMember { account_id});

            Ok(())
        }


        #[ink(message)]
        pub fn remove_member(& mut self, account_id: AccountId) -> Result<(), ContractError> {
            let caller = self.env().caller();
            let owner = self.owner;
            if caller != owner {
                return Err(ContractError::NoPermission)
            }
            if !self.members.contains(&account_id) {
                return Err(ContractError::NotAMember)
            }
            
            self.members.remove(&account_id);
            self.env().emit_event(RemoveMember { account_id});

            Ok(())
        }

		#[ink(message)]
		pub fn issue(&mut self, value: Hash) -> Result<(), ContractError> {
            let caller = self.env().caller();

            if !self.members.get(&caller).unwrap_or_default() {
                return Err(ContractError::NoPermission)
            }

			self.env().extension().issue_in_runtime(value)?;
            self.env().emit_event(IssuedByContract { value: value, account_id: caller });

			Ok(())
		}

        #[ink(message)]
		pub fn revoke(&mut self, value: Hash) -> Result<(), ContractError> {
            let caller = self.env().caller();

          if !self.members.get(&caller).unwrap_or_default() {
                return Err(ContractError::NoPermission)
            }

			self.env().extension().revoke_in_runtime(value)?;
            self.env().emit_event(RevokedByContract { value: value, account_id: caller });

			Ok(())
		}
        

    }

}
