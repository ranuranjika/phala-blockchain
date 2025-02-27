pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{ValueQuery, *};
    use frame_support::traits::{
        Currency,
        ExistenceRequirement::{AllowDeath, KeepAlive},
    };
    use pallet_contracts::AddressGenerator;
    use phala_crypto::sr25519::Sr25519SecretKey;
    use scale::{Decode, Encode};
    use scale_info::TypeInfo;
    use sp_core::crypto::UncheckedFrom;
    use sp_runtime::{
        traits::{Convert, Hash as _},
        SaturatedConversion, Saturating,
    };

    use crate::types::Hash;

    type CodeHash<T> = <T as frame_system::Config>::Hash;
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Eq, PartialEq, Encode, Decode, TypeInfo)]
    pub struct WasmCode<AccountId> {
        pub owner: AccountId,
        pub code: Vec<u8>,
    }

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn cluster_id)]
    pub(crate) type ClusterId<T: Config> = StorageValue<_, Hash, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn gas_price)]
    pub(crate) type GasPrice<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn deposit_per_byte)]
    pub(crate) type DepositPerByte<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn deposit_per_item)]
    pub(crate) type DepositPerItem<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    pub(crate) type TreasuryAccount<T: Config> = StorageValue<_, T::AccountId>;

    /// The priviate key of the cluster
    #[pallet::storage]
    #[pallet::getter(fn key)]
    pub(crate) type Key<T: Config> = StorageValue<_, Sr25519SecretKey>;

    /// Uploaded sidevm codes
    #[pallet::storage]
    #[pallet::getter(fn sidevm_codes)]
    pub(crate) type SidevmCodes<T: Config> =
        StorageMap<_, Twox64Concat, T::Hash, WasmCode<T::AccountId>>;

    /// The system contract address
    #[pallet::storage]
    #[pallet::getter(fn system_contract)]
    pub(crate) type SystemContract<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    impl<T: Config + pallet_contracts::Config> AddressGenerator<T> for Pallet<T>
    where
        T::AccountId: UncheckedFrom<T::Hash> + AsRef<[u8]>,
    {
        fn contract_address(
            deploying_address: &T::AccountId,
            code_hash: &CodeHash<T>,
            _input_data: &[u8],
            salt: &[u8],
        ) -> T::AccountId {
            let cluster_id = <ClusterId<T>>::get();
            let buf = phala_types::contract::contract_id_preimage(
                deploying_address.as_ref(),
                code_hash.as_ref(),
                cluster_id.as_ref(),
                salt,
            );
            UncheckedFrom::unchecked_from(<T as frame_system::Config>::Hashing::hash(&buf))
        }

        fn deposit_address(contract_addr: &T::AccountId) -> T::AccountId {
            let entropy = (b"contract_depo_v1", contract_addr).using_encoded(T::Hashing::hash);
            UncheckedFrom::unchecked_from(entropy)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn set_cluster_id(cluster_id: Hash) {
            <ClusterId<T>>::put(cluster_id);
        }

        pub fn set_key(key: Sr25519SecretKey) {
            <Key<T>>::put(key);
        }

        pub fn put_sidevm_code(
            owner: T::AccountId,
            code: Vec<u8>,
        ) -> Result<T::Hash, DispatchError> {
            let hash = T::Hashing::hash(&code);
            let bytes = code.len() + hash.as_ref().len();
            let fee = Self::deposit_per_byte()
                .saturating_mul(BalanceOf::<T>::saturated_from(bytes))
                .saturating_add(Self::deposit_per_item());
            Self::pay(&owner, fee)?;
            <SidevmCodes<T>>::insert(hash, WasmCode { owner, code });
            Ok(hash)
        }

        pub fn sidevm_code_exists(code_hash: &T::Hash) -> bool {
            <SidevmCodes<T>>::contains_key(code_hash)
        }

        pub fn set_system_contract(address: &T::AccountId) {
            <SystemContract<T>>::put(address);
        }

        pub fn pay_for_gas(user: &T::AccountId, gas: Weight) -> DispatchResult {
            Self::pay(user, Self::convert(gas))
        }

        pub fn refund_gas(user: &T::AccountId, gas: Weight) -> DispatchResult {
            Self::refund(user, Self::convert(gas))
        }

        fn pay(user: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let Some(treasury) = TreasuryAccount::<T>::get() else {
                return Ok(());
            };
            <T as Config>::Currency::transfer(user, &treasury, amount, KeepAlive)
        }

        fn refund(user: &T::AccountId, amount: BalanceOf<T>) -> DispatchResult {
            let Some(treasury) = TreasuryAccount::<T>::get() else {
                return Ok(());
            };
            <T as Config>::Currency::transfer(&treasury, user, amount, AllowDeath)
        }

        pub fn set_gas_price(price: BalanceOf<T>) {
            <GasPrice<T>>::put(price);
        }

        pub fn set_deposit_per_item(value: BalanceOf<T>) {
            <DepositPerItem<T>>::put(value);
        }

        pub fn set_deposit_per_byte(value: BalanceOf<T>) {
            <DepositPerByte<T>>::put(value);
        }

        pub fn set_treasury_account(account: &T::AccountId) {
            <TreasuryAccount<T>>::put(account);
        }
    }

    impl<T: Config> Convert<Weight, BalanceOf<T>> for Pallet<T> {
        fn convert(w: Weight) -> BalanceOf<T> {
            let weight = BalanceOf::<T>::saturated_from(w.ref_time());
            weight.saturating_mul(GasPrice::<T>::get())
        }
    }
}
