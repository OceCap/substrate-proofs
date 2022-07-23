/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @Date: 2022-06-28 22:05:13
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-07-24 02:45:23
 * @FilePath: \substrate-node-template\pallets\simple_pallet\src\lib.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
#![cfg_attr(not(feature = "std"), no_std)]
//1.Import and Dependencies
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	//2.Declaration of the Pallet type
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	//3.Runtime Configuration Trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	//4.Runtime Storage
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat, 
		u32, 
		u128, 
		OptionQuery
		>; 

	//5.Runtime Event
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(u32, u128),
		RemoveSuccess(u32,u128),
	}

	//6.Runtime Error
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyClaimed,
		NoSuchProof,
		NotOwnerOfProof,
	}
/* 	//6.Hooks
	//define some logic that should be executed 
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

	}
 */
	//7.Extrinsics
	// Functions that are callable from outside the runtime
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100)]
		pub fn create_proof(
			origin: OriginFor<T>,
			id: u32, 
			proof: u128) -> DispatchResult {
				let _sender = ensure_signed(origin)?;
				ensure!(!Proofs::<T>::contains_key(&id), Error::<T>::ProofAlreadyClaimed);
				Proofs::<T>::insert(&id, &proof);

				Self::deposit_event(Event::ClaimCreated(id, proof));
				Ok(())
			} 
	}


}