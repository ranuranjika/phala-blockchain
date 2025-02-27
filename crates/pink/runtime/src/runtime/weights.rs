#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use core::marker::PhantomData;
use frame_support::weights::constants::RocksDbWeight;

use frame_support::weights::Weight;
use pallet_contracts::weights::WeightInfo;
use sp_core::Get;

pub struct PinkWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PinkWeights<T> {
    /// Storage: Contracts DeletionQueueCounter (r:1 w:0)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    fn on_process_deletion_queue_batch() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `1594`
        // Minimum execution time: 2_565_000 picoseconds.
        Weight::from_parts(2_759_000, 1594)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `k` is `[0, 1024]`.
    fn on_initialize_per_trie_key(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `488 + k * (69 ±0)`
        //  Estimated: `478 + k * (70 ±0)`
        // Minimum execution time: 13_480_000 picoseconds.
        Weight::from_parts(10_153_869, 478)
            // Standard Error: 427
            .saturating_add(Weight::from_parts(958_726, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(k.into())))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_parts(0, 70).saturating_mul(k.into()))
    }
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn reinstrument(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `238 + c * (1 ±0)`
        //  Estimated: `3708 + c * (1 ±0)`
        // Minimum execution time: 30_406_000 picoseconds.
        Weight::from_parts(28_467_370, 3708)
            // Standard Error: 46
            .saturating_add(Weight::from_parts(53_724, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn call_with_code_per_byte(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `707`
        //  Estimated: `6656 + c * (1 ±0)`
        // Minimum execution time: 263_198_000 picoseconds.
        Weight::from_parts(276_162_279, 6656)
            // Standard Error: 18
            .saturating_add(Weight::from_parts(37_378, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate_with_code(c: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `270`
        //  Estimated: `8659`
        // Minimum execution time: 3_132_259_000 picoseconds.
        Weight::from_parts(513_284_834, 8659)
            // Standard Error: 280
            .saturating_add(Weight::from_parts(106_723, 0).saturating_mul(c.into()))
            // Standard Error: 16
            .saturating_add(Weight::from_parts(1_166, 0).saturating_mul(i.into()))
            // Standard Error: 16
            .saturating_add(Weight::from_parts(1_436, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(10_u64))
    }
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate(i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `482`
        //  Estimated: `6408`
        // Minimum execution time: 1_646_604_000 picoseconds.
        Weight::from_parts(271_369_256, 6408)
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_426, 0).saturating_mul(i.into()))
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_438, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(7_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `759`
        //  Estimated: `6699`
        // Minimum execution time: 191_360_000 picoseconds.
        Weight::from_parts(192_625_000, 6699)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 61717]`.
    fn upload_code(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `3574`
        // Minimum execution time: 245_207_000 picoseconds.
        Weight::from_parts(244_703_457, 3574)
            // Standard Error: 61
            .saturating_add(Weight::from_parts(106_850, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts CodeStorage (r:0 w:1)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    fn remove_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `255`
        //  Estimated: `3720`
        // Minimum execution time: 33_560_000 picoseconds.
        Weight::from_parts(33_833_000, 3720)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:2 w:2)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn set_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `570`
        //  Estimated: `8985`
        // Minimum execution time: 33_288_000 picoseconds.
        Weight::from_parts(33_775_000, 8985)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `781 + r * (6 ±0)`
        //  Estimated: `6722 + r * (6 ±0)`
        // Minimum execution time: 234_292_000 picoseconds.
        Weight::from_parts(235_941_911, 6722)
            // Standard Error: 413
            .saturating_add(Weight::from_parts(339_913, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_is_contract(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `839 + r * (240 ±0)`
        //  Estimated: `6743 + r * (2715 ±0)`
        // Minimum execution time: 236_273_000 picoseconds.
        Weight::from_parts(74_380_906, 6743)
            // Standard Error: 5_745
            .saturating_add(Weight::from_parts(3_331_781, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2715).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `831 + r * (244 ±0)`
        //  Estimated: `6747 + r * (2719 ±0)`
        // Minimum execution time: 236_573_000 picoseconds.
        Weight::from_parts(82_473_906, 6747)
            // Standard Error: 5_510
            .saturating_add(Weight::from_parts(4_131_820, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2719).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_own_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `788 + r * (6 ±0)`
        //  Estimated: `6730 + r * (6 ±0)`
        // Minimum execution time: 235_878_000 picoseconds.
        Weight::from_parts(238_387_359, 6730)
            // Standard Error: 318
            .saturating_add(Weight::from_parts(409_923, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller_is_origin(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 233_476_000 picoseconds.
        Weight::from_parts(238_014_452, 6723)
            // Standard Error: 145
            .saturating_add(Weight::from_parts(165_823, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `782 + r * (6 ±0)`
        //  Estimated: `6724 + r * (6 ±0)`
        // Minimum execution time: 235_490_000 picoseconds.
        Weight::from_parts(240_039_685, 6724)
            // Standard Error: 330
            .saturating_add(Weight::from_parts(332_291, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_gas_left(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `783 + r * (6 ±0)`
        //  Estimated: `6721 + r * (6 ±0)`
        // Minimum execution time: 236_093_000 picoseconds.
        Weight::from_parts(238_513_328, 6721)
            // Standard Error: 206
            .saturating_add(Weight::from_parts(328_899, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:2 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `922 + r * (6 ±0)`
        //  Estimated: `6846 + r * (6 ±0)`
        // Minimum execution time: 234_801_000 picoseconds.
        Weight::from_parts(243_519_159, 6846)
            // Standard Error: 1_367
            .saturating_add(Weight::from_parts(1_449_599, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_value_transferred(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `792 + r * (6 ±0)`
        //  Estimated: `6741 + r * (6 ±0)`
        // Minimum execution time: 236_765_000 picoseconds.
        Weight::from_parts(237_843_244, 6741)
            // Standard Error: 308
            .saturating_add(Weight::from_parts(329_911, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_minimum_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `790 + r * (6 ±0)`
        //  Estimated: `6739 + r * (6 ±0)`
        // Minimum execution time: 236_690_000 picoseconds.
        Weight::from_parts(241_743_164, 6739)
            // Standard Error: 333
            .saturating_add(Weight::from_parts(324_693, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_block_number(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787 + r * (6 ±0)`
        //  Estimated: `6737 + r * (6 ±0)`
        // Minimum execution time: 236_149_000 picoseconds.
        Weight::from_parts(239_090_707, 6737)
            // Standard Error: 246
            .saturating_add(Weight::from_parts(344_488, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_now(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (6 ±0)`
        //  Estimated: `6723 + r * (6 ±0)`
        // Minimum execution time: 235_057_000 picoseconds.
        Weight::from_parts(237_752_870, 6723)
            // Standard Error: 236
            .saturating_add(Weight::from_parts(328_235, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
    /// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_weight_to_fee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `856 + r * (10 ±0)`
        //  Estimated: `6796 + r * (10 ±0)`
        // Minimum execution time: 234_995_000 picoseconds.
        Weight::from_parts(246_473_554, 6796)
            // Standard Error: 1_015
            .saturating_add(Weight::from_parts(1_337_653, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_gas(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `745 + r * (4 ±0)`
        //  Estimated: `6687 + r * (4 ±0)`
        // Minimum execution time: 160_445_000 picoseconds.
        Weight::from_parts(165_558_135, 6687)
            // Standard Error: 234
            .saturating_add(Weight::from_parts(133_607, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 4).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_input(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `780 + r * (6 ±0)`
        //  Estimated: `6724 + r * (6 ±0)`
        // Minimum execution time: 235_065_000 picoseconds.
        Weight::from_parts(237_797_177, 6724)
            // Standard Error: 336
            .saturating_add(Weight::from_parts(267_302, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_input_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `784`
        //  Estimated: `6724`
        // Minimum execution time: 236_215_000 picoseconds.
        Weight::from_parts(239_347_313, 6724)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(587, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_return(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `768 + r * (45 ±0)`
        //  Estimated: `6708 + r * (45 ±0)`
        // Minimum execution time: 231_571_000 picoseconds.
        Weight::from_parts(233_477_918, 6708)
            // Standard Error: 95_776
            .saturating_add(Weight::from_parts(1_733_181, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 45).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_return_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778`
        //  Estimated: `6731`
        // Minimum execution time: 234_956_000 picoseconds.
        Weight::from_parts(236_785_051, 6731)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(177, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts DeletionQueueCounter (r:1 w:1)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts DeletionQueue (r:0 w:1)
    /// Proof: Contracts DeletionQueue (max_values: None, max_size: Some(142), added: 2617, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_terminate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `810 + r * (356 ±0)`
        //  Estimated: `6750 + r * (7781 ±0)`
        // Minimum execution time: 234_275_000 picoseconds.
        Weight::from_parts(236_776_769, 6750)
            // Standard Error: 137_203
            .saturating_add(Weight::from_parts(110_758_930, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 7781).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_random(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `825 + r * (10 ±0)`
        //  Estimated: `6769 + r * (10 ±0)`
        // Minimum execution time: 235_593_000 picoseconds.
        Weight::from_parts(253_731_242, 6769)
            // Standard Error: 2_129
            .saturating_add(Weight::from_parts(1_771_297, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_deposit_event(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `778 + r * (10 ±0)`
        //  Estimated: `6723 + r * (10 ±0)`
        // Minimum execution time: 232_124_000 picoseconds.
        Weight::from_parts(245_904_447, 6723)
            // Standard Error: 2_185
            .saturating_add(Weight::from_parts(3_470_410, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:6 w:6)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 4]`.
    /// The range of component `n` is `[0, 16384]`.
    fn seal_deposit_event_per_topic_and_byte(t: u32, n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `797 + t * (32 ±0)`
        //  Estimated: `6744 + t * (2508 ±0)`
        // Minimum execution time: 250_301_000 picoseconds.
        Weight::from_parts(245_292_258, 6744)
            // Standard Error: 29_864
            .saturating_add(Weight::from_parts(2_163_531, 0).saturating_mul(t.into()))
            // Standard Error: 8
            .saturating_add(Weight::from_parts(583, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2508).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_debug_message(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `777 + r * (7 ±0)`
        //  Estimated: `6721 + r * (7 ±0)`
        // Minimum execution time: 165_711_000 picoseconds.
        Weight::from_parts(168_792_571, 6721)
            // Standard Error: 216
            .saturating_add(Weight::from_parts(230_285, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 7).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    fn seal_debug_message_per_byte(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `125728`
        //  Estimated: `131670`
        // Minimum execution time: 348_928_000 picoseconds.
        Weight::from_parts(352_224_793, 131670)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(731, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_set_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `845 + r * (292 ±0)`
        //  Estimated: `843 + r * (293 ±0)`
        // Minimum execution time: 236_418_000 picoseconds.
        Weight::from_parts(129_862_840, 843)
            // Standard Error: 9_733
            .saturating_add(Weight::from_parts(6_005_187, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 293).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_new_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1304`
        //  Estimated: `1280`
        // Minimum execution time: 251_599_000 picoseconds.
        Weight::from_parts(285_284_665, 1280)
            // Standard Error: 46
            .saturating_add(Weight::from_parts(410, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_old_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1167 + n * (1 ±0)`
        //  Estimated: `1167 + n * (1 ±0)`
        // Minimum execution time: 251_309_000 picoseconds.
        Weight::from_parts(253_555_552, 1167)
            // Standard Error: 9
            .saturating_add(Weight::from_parts(27, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_clear_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `841 + r * (288 ±0)`
        //  Estimated: `845 + r * (289 ±0)`
        // Minimum execution time: 235_441_000 picoseconds.
        Weight::from_parts(132_980_942, 845)
            // Standard Error: 9_421
            .saturating_add(Weight::from_parts(5_854_896, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_clear_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1163 + n * (1 ±0)`
        //  Estimated: `1163 + n * (1 ±0)`
        // Minimum execution time: 249_967_000 picoseconds.
        Weight::from_parts(252_122_186, 1163)
            // Standard Error: 10
            .saturating_add(Weight::from_parts(74, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_get_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `835 + r * (296 ±0)`
        //  Estimated: `840 + r * (297 ±0)`
        // Minimum execution time: 235_647_000 picoseconds.
        Weight::from_parts(145_525_169, 840)
            // Standard Error: 8_553
            .saturating_add(Weight::from_parts(4_948_021, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_get_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1179 + n * (1 ±0)`
        //  Estimated: `1179 + n * (1 ±0)`
        // Minimum execution time: 249_576_000 picoseconds.
        Weight::from_parts(250_747_191, 1179)
            // Standard Error: 8
            .saturating_add(Weight::from_parts(717, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_contains_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `856 + r * (288 ±0)`
        //  Estimated: `857 + r * (289 ±0)`
        // Minimum execution time: 236_110_000 picoseconds.
        Weight::from_parts(148_420_625, 857)
            // Standard Error: 8_175
            .saturating_add(Weight::from_parts(4_684_126, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_contains_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1166 + n * (1 ±0)`
        //  Estimated: `1166 + n * (1 ±0)`
        // Minimum execution time: 247_800_000 picoseconds.
        Weight::from_parts(249_410_575, 1166)
            // Standard Error: 6
            .saturating_add(Weight::from_parts(99, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_take_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `829 + r * (296 ±0)`
        //  Estimated: `836 + r * (297 ±0)`
        // Minimum execution time: 235_251_000 picoseconds.
        Weight::from_parts(128_816_707, 836)
            // Standard Error: 9_887
            .saturating_add(Weight::from_parts(6_167_176, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_take_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1180 + n * (1 ±0)`
        //  Estimated: `1180 + n * (1 ±0)`
        // Minimum execution time: 250_401_000 picoseconds.
        Weight::from_parts(253_298_243, 1180)
            // Standard Error: 9
            .saturating_add(Weight::from_parts(667, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1602 w:1601)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_transfer(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1373 + r * (45 ±0)`
        //  Estimated: `7270 + r * (2520 ±0)`
        // Minimum execution time: 236_470_000 picoseconds.
        Weight::from_parts(98_898_727, 7270)
            // Standard Error: 33_316
            .saturating_add(Weight::from_parts(35_149_946, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2520).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:802 w:802)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1237 + r * (256 ±0)`
        //  Estimated: `7125 + r * (2732 ±0)`
        // Minimum execution time: 238_303_000 picoseconds.
        Weight::from_parts(239_024_000, 7125)
            // Standard Error: 65_907
            .saturating_add(Weight::from_parts(209_419_071, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2732).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:736 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:737 w:737)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_delegate_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (502 ±0)`
        //  Estimated: `6727 + r * (2572 ±10)`
        // Minimum execution time: 235_961_000 picoseconds.
        Weight::from_parts(236_939_000, 6727)
            // Standard Error: 83_087
            .saturating_add(Weight::from_parts(205_646_517, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2572).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:3 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `c` is `[0, 1048576]`.
    fn seal_call_per_transfer_clone_byte(t: u32, c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1154 + t * (204 ±0)`
        //  Estimated: `9569 + t * (5154 ±0)`
        // Minimum execution time: 410_156_000 picoseconds.
        Weight::from_parts(378_378_143, 9569)
            // Standard Error: 285_172
            .saturating_add(Weight::from_parts(34_736_740, 0).saturating_mul(t.into()))
            // Standard Error: 0
            .saturating_add(Weight::from_parts(591, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(5_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 5154).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1602 w:1602)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:801 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:800 w:800)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:802 w:802)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_instantiate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1301 + r * (254 ±0)`
        //  Estimated: `7131 + r * (5205 ±0)`
        // Minimum execution time: 236_748_000 picoseconds.
        Weight::from_parts(237_129_000, 7131)
            // Standard Error: 280_059
            .saturating_add(Weight::from_parts(341_428_013, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(6_u64))
            .saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 5205).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:2 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1 w:1)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `i` is `[0, 983040]`.
    /// The range of component `s` is `[0, 983040]`.
    fn seal_instantiate_per_transfer_input_salt_byte(t: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1071 + t * (187 ±0)`
        //  Estimated: `9492 + t * (2634 ±2)`
        // Minimum execution time: 1_613_796_000 picoseconds.
        Weight::from_parts(340_002_206, 9492)
            // Standard Error: 4_296_381
            .saturating_add(Weight::from_parts(115_239_834, 0).saturating_mul(t.into()))
            // Standard Error: 6
            .saturating_add(Weight::from_parts(1_145, 0).saturating_mul(i.into()))
            // Standard Error: 6
            .saturating_add(Weight::from_parts(1_315, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(13_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(10_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2634).saturating_mul(t.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_sha2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `777 + r * (8 ±0)`
        //  Estimated: `6718 + r * (8 ±0)`
        // Minimum execution time: 233_111_000 picoseconds.
        Weight::from_parts(238_643_933, 6718)
            // Standard Error: 184
            .saturating_add(Weight::from_parts(572_296, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_sha2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `785`
        //  Estimated: `6725`
        // Minimum execution time: 234_746_000 picoseconds.
        Weight::from_parts(229_815_552, 6725)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(3_892, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_keccak_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6721 + r * (8 ±0)`
        // Minimum execution time: 232_732_000 picoseconds.
        Weight::from_parts(239_007_209, 6721)
            // Standard Error: 256
            .saturating_add(Weight::from_parts(733_879, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_keccak_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6729`
        // Minimum execution time: 234_184_000 picoseconds.
        Weight::from_parts(227_603_375, 6729)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(3_127, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6724 + r * (8 ±0)`
        // Minimum execution time: 233_038_000 picoseconds.
        Weight::from_parts(238_515_817, 6724)
            // Standard Error: 255
            .saturating_add(Weight::from_parts(413_343, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6733`
        // Minimum execution time: 232_996_000 picoseconds.
        Weight::from_parts(226_706_997, 6733)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(908, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_128(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `779 + r * (8 ±0)`
        //  Estimated: `6725 + r * (8 ±0)`
        // Minimum execution time: 232_292_000 picoseconds.
        Weight::from_parts(237_997_001, 6725)
            // Standard Error: 219
            .saturating_add(Weight::from_parts(410_177, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_128_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `787`
        //  Estimated: `6727`
        // Minimum execution time: 234_815_000 picoseconds.
        Weight::from_parts(226_317_150, 6727)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(911, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 125697]`.
    fn seal_sr25519_verify_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `912 + n * (1 ±0)`
        //  Estimated: `6848 + n * (1 ±0)`
        // Minimum execution time: 286_323_000 picoseconds.
        Weight::from_parts(290_287_955, 6848)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(4_693, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_sr25519_verify(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `727 + r * (112 ±0)`
        //  Estimated: `6666 + r * (112 ±0)`
        // Minimum execution time: 235_938_000 picoseconds.
        Weight::from_parts(242_728_358, 6666)
            // Standard Error: 9_725
            .saturating_add(Weight::from_parts(47_527_740, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 112).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_recover(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `822 + r * (76 ±0)`
        //  Estimated: `6716 + r * (77 ±0)`
        // Minimum execution time: 236_108_000 picoseconds.
        Weight::from_parts(248_577_226, 6716)
            // Standard Error: 9_565
            .saturating_add(Weight::from_parts(36_733_552, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 77).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_to_eth_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `792 + r * (42 ±0)`
        //  Estimated: `6731 + r * (42 ±0)`
        // Minimum execution time: 236_440_000 picoseconds.
        Weight::from_parts(240_771_418, 6731)
            // Standard Error: 1_849
            .saturating_add(Weight::from_parts(9_185_896, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 42).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1536 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts OwnerInfoOf (r:1536 w:1536)
    /// Proof: Contracts OwnerInfoOf (max_values: None, max_size: Some(88), added: 2563, mode: Measured)
    /// Storage: System EventTopics (r:1538 w:1538)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_set_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (964 ±0)`
        //  Estimated: `8190 + r * (3090 ±10)`
        // Minimum execution time: 235_056_000 picoseconds.
        Weight::from_parts(235_743_000, 8190)
            // Standard Error: 46_122
            .saturating_add(Weight::from_parts(21_447_984, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 3090).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `773 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 235_213_000 picoseconds.
        Weight::from_parts(239_456_464, 6723)
            // Standard Error: 130
            .saturating_add(Weight::from_parts(159_851, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_account_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1975 + r * (39 ±0)`
        //  Estimated: `7805 + r * (40 ±0)`
        // Minimum execution time: 237_886_000 picoseconds.
        Weight::from_parts(262_430_157, 7805)
            // Standard Error: 939
            .saturating_add(Weight::from_parts(260_005, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(r.into()))
    }
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeStorage (r:1 w:0)
    /// Proof: Contracts CodeStorage (max_values: None, max_size: Some(126001), added: 128476, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_instantiation_nonce(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `776 + r * (3 ±0)`
        //  Estimated: `6723 + r * (3 ±0)`
        // Minimum execution time: 234_014_000 picoseconds.
        Weight::from_parts(240_042_671, 6723)
            // Standard Error: 152
            .saturating_add(Weight::from_parts(138_382, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64const(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_533_000 picoseconds.
        Weight::from_parts(1_846_015, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(2_935, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64load(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_671_000 picoseconds.
        Weight::from_parts(2_197_197, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(6_335, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64store(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_665_000 picoseconds.
        Weight::from_parts(2_200_545, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(6_011, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_select(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_545_000 picoseconds.
        Weight::from_parts(1_977_462, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(7_901, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_515_000 picoseconds.
        Weight::from_parts(1_866_184, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(10_514, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_618_000 picoseconds.
        Weight::from_parts(1_895_104, 0)
            // Standard Error: 12
            .saturating_add(Weight::from_parts(4_523, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br_if(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_510_000 picoseconds.
        Weight::from_parts(1_779_998, 0)
            // Standard Error: 8
            .saturating_add(Weight::from_parts(6_832, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_br_table(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_529_000 picoseconds.
        Weight::from_parts(1_726_996, 0)
            // Standard Error: 26
            .saturating_add(Weight::from_parts(9_199, 0).saturating_mul(r.into()))
    }
    /// The range of component `e` is `[1, 256]`.
    fn instr_br_table_per_entry(e: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_682_000 picoseconds.
        Weight::from_parts(1_789_910, 0)
            // Standard Error: 16
            .saturating_add(Weight::from_parts(42, 0).saturating_mul(e.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_539_000 picoseconds.
        Weight::from_parts(2_093_056, 0)
            // Standard Error: 27
            .saturating_add(Weight::from_parts(18_917, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_call_indirect(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_851_000 picoseconds.
        Weight::from_parts(3_134_610, 0)
            // Standard Error: 34
            .saturating_add(Weight::from_parts(24_714, 0).saturating_mul(r.into()))
    }
    /// The range of component `l` is `[0, 1024]`.
    fn instr_call_per_local(l: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_654_000 picoseconds.
        Weight::from_parts(1_885_921, 0)
            // Standard Error: 14
            .saturating_add(Weight::from_parts(1_243, 0).saturating_mul(l.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_744_000 picoseconds.
        Weight::from_parts(3_014_725, 0)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(2_447, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_881_000 picoseconds.
        Weight::from_parts(3_137_711, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(3_608, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_local_tee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 2_809_000 picoseconds.
        Weight::from_parts(3_142_066, 0)
            // Standard Error: 3
            .saturating_add(Weight::from_parts(3_841, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_global_get(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_704_000 picoseconds.
        Weight::from_parts(2_083_619, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(8_366, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_global_set(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_652_000 picoseconds.
        Weight::from_parts(2_048_256, 0)
            // Standard Error: 5
            .saturating_add(Weight::from_parts(8_826, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_memory_current(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_671_000 picoseconds.
        Weight::from_parts(1_924_626, 0)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(3_746, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 16]`.
    fn instr_memory_grow(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_585_000 picoseconds.
        Weight::from_parts(490_856, 0)
            // Standard Error: 133_673
            .saturating_add(Weight::from_parts(13_182_582, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64clz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_533_000 picoseconds.
        Weight::from_parts(1_851_563, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_820, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ctz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_564_000 picoseconds.
        Weight::from_parts(1_914_178, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_732, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64popcnt(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_559_000 picoseconds.
        Weight::from_parts(1_886_992, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_731, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64eqz(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_553_000 picoseconds.
        Weight::from_parts(1_886_545, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_658, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64extendsi32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_507_000 picoseconds.
        Weight::from_parts(1_853_647, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_852, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64extendui32(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_554_000 picoseconds.
        Weight::from_parts(1_868_877, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_806, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i32wrapi64(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_514_000 picoseconds.
        Weight::from_parts(1_882_233, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(3_700, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64eq(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_529_000 picoseconds.
        Weight::from_parts(1_897_247, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_955, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ne(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_513_000 picoseconds.
        Weight::from_parts(1_922_333, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_933, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64lts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_512_000 picoseconds.
        Weight::from_parts(1_848_668, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_966, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ltu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_522_000 picoseconds.
        Weight::from_parts(1_875_257, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_965, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64gts(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_546_000 picoseconds.
        Weight::from_parts(1_836_691, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(5_842, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64gtu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_505_000 picoseconds.
        Weight::from_parts(1_907_551, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_075, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64les(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_527_000 picoseconds.
        Weight::from_parts(1_891_008, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_971, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64leu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_556_000 picoseconds.
        Weight::from_parts(1_910_864, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_059, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64ges(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_544_000 picoseconds.
        Weight::from_parts(1_912_650, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_943, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64geu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_513_000 picoseconds.
        Weight::from_parts(1_855_260, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_975, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64add(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_521_000 picoseconds.
        Weight::from_parts(1_867_259, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_846, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64sub(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_509_000 picoseconds.
        Weight::from_parts(1_893_018, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_096, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64mul(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_496_000 picoseconds.
        Weight::from_parts(1_886_659, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_754, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64divs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_527_000 picoseconds.
        Weight::from_parts(1_890_548, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(11_842, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64divu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_518_000 picoseconds.
        Weight::from_parts(1_891_903, 0)
            // Standard Error: 4
            .saturating_add(Weight::from_parts(10_613, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rems(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_504_000 picoseconds.
        Weight::from_parts(1_632_694, 0)
            // Standard Error: 7
            .saturating_add(Weight::from_parts(12_281, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64remu(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_507_000 picoseconds.
        Weight::from_parts(1_878_413, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(10_737, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64and(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_534_000 picoseconds.
        Weight::from_parts(1_898_519, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_645, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64or(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_503_000 picoseconds.
        Weight::from_parts(1_895_532, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_745, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64xor(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_507_000 picoseconds.
        Weight::from_parts(1_868_720, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_873, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_513_000 picoseconds.
        Weight::from_parts(1_894_207, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_843, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shrs(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_473_000 picoseconds.
        Weight::from_parts(1_880_224, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(6_107, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64shru(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_447_000 picoseconds.
        Weight::from_parts(1_884_551, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_849, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rotl(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_538_000 picoseconds.
        Weight::from_parts(1_908_813, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_987, 0).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64rotr(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_538_000 picoseconds.
        Weight::from_parts(1_878_015, 0)
            // Standard Error: 2
            .saturating_add(Weight::from_parts(5_848, 0).saturating_mul(r.into()))
    }
}
