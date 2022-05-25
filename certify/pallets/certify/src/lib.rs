#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
	pub struct Certificate<AccountId> {
		issuer: AccountId,
		is_revoked: bool,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn certificate_struct)]
    pub(super) type CertificateMap<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, Certificate<T::AccountId>>;

	#[pallet::storage]
	#[pallet::getter(fn certificate_revocation)]
	pub(super) type CertificateRevocationStatusMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool>;

	#[pallet::storage]
	#[pallet::getter(fn certificate_issuer)]
	pub(super) type CertificateIssuerMap<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Issued(T::AccountId, T::Hash),
		Revoked(T::AccountId, T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NoOwner,
		IncorrectOwner,
		AlreadyExists,
		DoesNotExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(10_000)]
		pub fn issue(origin: OriginFor<T>, certificate_id: T::Hash) -> DispatchResultWithPostInfo {
			// A user can only set their own entry
			let user = ensure_signed(origin)?;

			ensure!(
				!(<CertificateIssuerMap<T>>::contains_key(&certificate_id) || <CertificateRevocationStatusMap<T>>::contains_key(&user)),
				Error::<T>::AlreadyExists
			);

			<CertificateIssuerMap<T>>::insert(certificate_id, &user);

			<CertificateRevocationStatusMap<T>>::insert(&user, false);

			Self::deposit_event(Event::Issued(user, certificate_id));

			Ok(().into())
		}


		#[pallet::weight(10_000)]
		pub fn revoke(origin: OriginFor<T>, certificate_id: T::Hash) -> DispatchResultWithPostInfo {
			// A user can only take (delete) their own entry
			let user = ensure_signed(origin)?;

			ensure!(
				<CertificateIssuerMap<T>>::contains_key(&certificate_id) && <CertificateRevocationStatusMap<T>>::contains_key(&user),
				Error::<T>::DoesNotExist
			);

			let owner = <CertificateIssuerMap<T>>::get(&certificate_id).ok_or(Error::<T>::NoOwner)?;
			ensure!(user == owner, <Error<T>>::IncorrectOwner);

			<CertificateRevocationStatusMap<T>>::insert(&user, true);
			Self::deposit_event(Event::Revoked(user, certificate_id));
			Ok(().into())
		}
	}
}
