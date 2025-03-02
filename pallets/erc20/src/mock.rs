#![cfg(test)]

use chainbridge as bridge;
use frame_support::{ord_parameter_types, derive_impl, parameter_types, weights::Weight};
use frame_system::{self as system};
pub use pallet_balances as balances;
use sp_core::{hashing::blake2_128, H256};
use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, Block as BlockT, IdentityLookup},
	Perbill,
};

use super::*;
use crate::{self as example, Config};

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = u64;
	type AccountId = u64;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type AccountData = balances::AccountData<u64>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

ord_parameter_types! {
	pub const One: u64 = 1;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const TestChainId: u8 = 5;
	pub const ProposalLifetime: u64 = 100;
}

impl bridge::Config for Test {
	type Event = Event;
	type AdminOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type Proposal = Call;
	type ChainIdentity = TestChainId;
	type ProposalLifetime = ProposalLifetime;
}

parameter_types! {
	pub HashId: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"hash"));
	pub NativeTokenId: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"DAV"));
	pub Erc721Id: bridge::ResourceId = bridge::derive_resource_id(1, &blake2_128(b"NFT"));
}

impl erc721::Config for Test {
	type Event = Event;
	type Identifier = Erc721Id;
}

impl Config for Test {
	type Event = Event;
	type BridgeOrigin = bridge::EnsureBridge<Test>;
	type Currency = Balances;
	type HashId = HashId;
	type NativeTokenId = NativeTokenId;
	type Erc721Id = Erc721Id;
	type WeightInfo = ();
}

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, u64, Call, ()>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: system::{Pallet, Call, Event<T>},
		Balances: balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Bridge: bridge::{Pallet, Call, Storage, Event<T>},
		Erc721: erc721::{Pallet, Call, Storage, Event<T>},
		Example: example::{Pallet, Call, Event<T>}
	}
);

pub const RELAYER_A: u64 = 0x2;
pub const RELAYER_B: u64 = 0x3;
pub const RELAYER_C: u64 = 0x4;
pub const ENDOWED_BALANCE: u64 = 100_000_000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let bridge_id = PalletId(*b"cb/bridg").into_account();
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(bridge_id, ENDOWED_BALANCE), (RELAYER_A, ENDOWED_BALANCE)],
	}
		.assimilate_storage(&mut t)
		.unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

fn last_event() -> Event {
	system::Module::<Test>::events().pop().map(|e| e.event).expect("Event expected")
}

pub fn expect_event<E: Into<Event>>(e: E) {
	assert_eq!(last_event(), e.into());
}

// Asserts that the event was emitted at some point.
pub fn event_exists<E: Into<Event>>(e: E) {
	let actual: Vec<Event> =
		system::Module::<Test>::events().iter().map(|e| e.event.clone()).collect();
	let e: Event = e.into();
	let mut exists = false;
	for evt in actual {
		if evt == e {
			exists = true;
			break
		}
	}
	assert!(exists);
}

// Checks events against the latest. A contiguous set of events must be provided. They must
// include the most recent event, but do not have to include every past event.
pub fn assert_events(mut expected: Vec<Event>) {
	let mut actual: Vec<Event> =
		system::Module::<Test>::events().iter().map(|e| e.event.clone()).collect();

	expected.reverse();

	for evt in expected {
		let next = actual.pop().expect("event expected");
		assert_eq!(next, evt.into(), "Events don't match");
	}
}
