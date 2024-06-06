//! A shell pallet built with [`frame`].

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	type Balance = u128;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::storage]
	pub type TotalIssuance<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub type Balances<T: Config> = StorageMap<_, _, T::AccountId, Balance>;


	#[pallet::error]
	pub enum Error<T> {
		NotEnoughBalance,
		KeyNotPresent,
	}

	// #[pallet::event]
	// pub enum Event<T> {
	// 	Minted {owner: T::Account},
	// 	Transfered {from: T::Account, to: T::Account},
	// }


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T:Config> Pallet<T> {
		pub fn mint_token(origin: OriginFor<T>, to: T::AccountId, amount: Balance) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			Balances::<T>::mutate(to, |value| *value = Some(value.unwrap_or(0) + amount));

			// Self::deposit_event(Event::Minted {owner: to.clone()});

			TotalIssuance::<T>::mutate(|value| *value = Some(value.unwrap_or(0) + amount));

			Ok(())
		}

		// Check if sender has that much amount to send or not?
		// Check the sender should be signed.
		pub fn transfer(origin: OriginFor<T>, from: T::AccountId, to: T::AccountId, amount: Balance) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// check if sender has enough balance or not
			let current_balance = Balances::<T>::get(&who).ok_or(Error::<T>::KeyNotPresent)?;

			ensure!(current_balance >= amount, Error::<T>::NotEnoughBalance);

			Balances::<T>::mutate(to.clone(), |value| *value = Some(value.unwrap_or(0) - amount));

			Balances::<T>::insert(who, amount);
			// Self::deposit_event(Event::Transfered {from: from.clone(), to});

			// we are not taking total issuance into consideration at this time because no new token is generated or burn
			Ok(())
		}
	}
}


