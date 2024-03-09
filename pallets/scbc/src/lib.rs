#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	use frame_support::{pallet_prelude::*, traits::Hash};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	type PhoneNumber = Vec<u8>;
	type DomainType = Vec<u8>;
	type TransactionHash = Vec<u8>;
	type Timestamp = Vec<u8>;

	// Data Structures
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
	pub struct PhoneRecord {
		trust_rating: [u8; 16],
		spam_transactions: Vec<TransactionHash>,
		domain: DomainType,
		unique_id: [u8; 16],
	}

	// Storage Items
	#[pallet::storage]
	#[pallet::getter(fn phone_record)]
	pub type Ledger<T: Config> =
		StorageMap<_, Blake2_128Concat, PhoneNumber, PhoneRecord, OptionQuery>;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		#[pallet::constant]
		type MaximumOwned: Get<u32>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	// Events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]

	pub enum Event<T: Config> {
		RegisterPhoneNumber { phone_number: PhoneNumber },
		RegiterDomain { domain: DomainType },
		GetPhoneNumberDetails { unique_id: [u8; 16] },
		GetDomainRating { domain: DomainType },
		ReportSPAM { caller: T::AccountId, phone_number: PhoneNumber, timestamp: Timestamp },
		// Consider more events: TrustRatingChanged, SpamThresholdReached, etc.
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The phone number is already registered
		PhoneNumberAlreadyRegistered,
		/// The domain is already registered
		DomainAlreadyRegistered,
		/// The phone number is not registered
		PhoneNumberNotRegistered,
		/// The domain is not registered
		DomainNotRegistered,
		/// The phone number is not spam
		PhoneNumberNotSpam,
		/// The domain is not spam
		DomainNotSpam,
		/// The phone number is spam
		PhoneNumberSpam,
		/// The domain is spam
		DomainSpam,
	}

	// // Dispatchable Calls (Extrinsics)
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn register_phone_number(
			origin: OriginFor<T>,
			phone_number: PhoneNumber,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				!Ledger::<T>::contains_key(phone_number.clone()),
				Error::<T>::PhoneNumberAlreadyRegistered
			);
			let new_phone_record = PhoneRecord {
				trust_rating: [0; 16],
				spam_transactions: vec![],
				domain: "normal".as_bytes().to_vec(),
				unique_id: Default::default(),
			};
			Ledger::<T>::insert(phone_number.clone(), new_phone_record);
			Self::deposit_event(Event::RegisterPhoneNumber { phone_number });
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn get_phone_number_details(
			origin: OriginFor<T>,
			phone_number: PhoneNumber,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let phone_record = Ledger::<T>::get(phone_number.clone())
				.ok_or(Error::<T>::PhoneNumberNotRegistered)?;
			Self::deposit_event(Event::GetPhoneNumberDetails { unique_id: phone_record.unique_id });
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn report_spam(
			origin: OriginFor<T>,
			phone_number: PhoneNumber,
			timestamp: Timestamp, // Assuming Timestamp type is defined
		) -> DispatchResult {
			// Ensure the caller is signed
			let who = ensure_signed(origin)?;

			// Get spam threshold (consider storing this as configurable parameter)
			let spam_threshold: u8 = 50; // Example threshold

			// Check if the phone number exists, otherwise register it automatically
			if !Ledger::<T>::contains_key(&phone_number) {
				let new_phone_record = PhoneRecord {
					trust_rating: [0; 16],
					spam_transactions: vec![],
					domain: "normal".as_bytes().to_vec(),

					unique_id: Default::default(),
				};
				Ledger::<T>::insert(&phone_number, new_phone_record);
			}

			// Fetch the existing phone record (guaranteed that it exists at this point)
			let mut phone_record = Ledger::<T>::get(&phone_number).unwrap();

			// Update the trust rating of the phone number
			phone_record.trust_rating = Self::update_trust_rating(&phone_record.trust_rating, 10);

			// get the current transaction hash
			let transaction_hash = <frame_system::Pallet<T>>::extrinsic_index();
			let transaction_hash = transaction_hash.encode();
			// Add the spam transaction to the record
			phone_record.spam_transactions.push(transaction_hash);

			// Check if the trust rating has fallen below the spam threshold
			if phone_record.trust_rating.iter().sum::<u8>() > spam_threshold {
				// Change domain type to spam
				phone_record.domain = Self::update_domain_type(&phone_record.domain, "spam");
			}

			// Update the ledger with the modified phone record information
			Ledger::<T>::insert(&phone_number, phone_record);

			// Report spam event
			Self::deposit_event(Event::ReportSPAM { caller: who, phone_number, timestamp });

			Ok(())
		}
		// Helper function to update the trust rating (replace with your own logic)
	}

	impl<T: Config> Pallet<T> {
		fn update_trust_rating(current_rating: &[u8; 16], adjustment: u8) -> [u8; 16] {
			// Placeholder logic; replace with your own
			let mut new_rating = current_rating.clone();
			for i in 0..16 {
				new_rating[i] = new_rating[i].saturating_add(adjustment);
			}

			new_rating
		}
		fn update_domain_type(current_domain: &DomainType, new_type: &str) -> DomainType {
			// Placeholder logic; replace with your own
			let mut new_domain = current_domain.clone();
			new_domain.clear();
			new_domain.extend(new_type.as_bytes());
			new_domain
		}
	}

	// #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
	// pub fn register_domain(origin: OriginFor<T>, domain: DomainType) -> DispatchResult {
	// 	let who = ensure_signed(origin)?;
	// 	ensure!(
	// 		!Ledger::<T>::contains_key(domain.clone()),
	// 		Error::<T>::DomainAlreadyRegistered
	// 	);
	// 	let new_phone_record = PhoneRecord {
	// 		trust_rating: [0; 16],
	// 		spam_transactions: vec![],
	// 		// domain: vec![],
	// 		unique_id: Default::default(),
	// 	};
	// 	Ledger::<T>::insert(domain.clone(), new_phone_record);
	// 	Self::deposit_event(Event::RegiterDomain { domain });
	// 	Ok(())
	// }

	// 	fn is_spam_threshold_reached(record: &PhoneRecord) -> bool {
	// 		let spam_threshold = 5; // Placeholder value; adjust as needed
	// 		record.spam_transactions.len() >= spam_threshold
	// 	}

	// 	fn get_transaction_hash() -> TransactionHash {
	// 		<frame_system::Pallet<T>>::extrinsic().hash()
	// 	}

	// 	fn decrease_trust_rating(trust_rating: u32) -> u32 {
	// 		let decrement_amount = 10; // Placeholder; you'll likely adjust or make this dynamic
	// 		trust_rating.saturating_sub(decrement_amount)
	// 	}

	// 	#[pallet::call_index(1)]
	// 	#[pallet::weight(T::WeightInfo::report_spam())]
	// 	pub fn report_spam(
	// 		origin: OriginFor<T>,
	// 		phone_number: PhoneNumber,
	// 		kind_of_spam: Vec<u8>,
	// 	) -> DispatchResult {
	// 		let reporter = ensure_signed(origin)?;
	// 		let kind_of_spam = kind_of_spam.clone();
	// 		let phone_number_clone = phone_number.clone();

	// 		Ledger::<T>::mutate(phone_number_clone, |record| match record {
	// 			Some(rec) => {
	// 				// Update existing record:
	// 				rec.trust_rating = Self::decrease_trust_rating(rec.trust_rating); // You'll implement this
	// function 				rec.spam_transactions.push(Self::get_transaction_hash()); // Assuming this fetches
	// the current transaction's hash

	// 				if Self::is_spam_threshold_reached(rec) { // You'll define this
	// 					 // Mark as spam (update PhoneRecord)
	// 					 // Potentially emit an event
	// 				}
	// 			},
	// 			None => todo!(),
	// 		});

	// 		Self::deposit_event(Event::SpamReported { phone_number, reporter });
	// 		Ok(())
	// 	}

	// 	#[pallet::call_index(2)]
	// 	#[pallet::weight(T::WeightInfo::report_spam())]
	// 	pub fn register(origin: OriginFor<T>, phone_number: PhoneNumber) -> DispatchResult {
	// 		let new_phone_record = PhoneRecord {
	// 			trust_rating: 0,
	// 			spam_transactions: vec![],
	// 			domain: vec![],
	// 			unique_id: Default::default(),
	// 		};

	// 		Ledger::<T>::insert(phone_number.clone(), new_phone_record);

	// 		Self::deposit_event(Event::PhoneRecorded { phone_number });
	// 		Ok(())
	// 	}
	// }
}
