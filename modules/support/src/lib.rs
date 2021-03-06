#![cfg_attr(not(feature = "std"), no_std)]

use codec::FullCodec;
use orml_utilities::FixedU128;
use rstd::{
	cmp::{Eq, PartialEq},
	fmt::Debug,
};
use sp_runtime::{DispatchError, DispatchResult};

pub type Price = FixedU128;
pub type ExchangeRate = FixedU128;
pub type Ratio = FixedU128;
pub type Rate = FixedU128;

pub trait RiskManager<AccountId, CurrencyId, Balance, DebitBalance> {
	fn check_position_valid(
		currency_id: CurrencyId,
		collateral_balance: Balance,
		debit_balance: DebitBalance,
	) -> DispatchResult;

	fn check_debit_cap(currency_id: CurrencyId, total_debit_balance: DebitBalance) -> DispatchResult;
}

pub trait AuctionManager<AccountId> {
	type CurrencyId;
	type Balance;
	type AuctionId: FullCodec + Debug + Clone + Eq + PartialEq;

	fn new_collateral_auction(
		who: &AccountId,
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
	);
	fn new_debit_auction(amount: Self::Balance, fix: Self::Balance);
	fn new_surplus_auction(amount: Self::Balance);
	fn cancel_auction(id: Self::AuctionId) -> DispatchResult;

	fn get_total_collateral_in_auction(id: Self::CurrencyId) -> Self::Balance;
	fn get_total_surplus_in_auction() -> Self::Balance;
	fn get_total_debit_in_auction() -> Self::Balance;
	fn get_total_target_in_auction() -> Self::Balance;
}

pub trait DEXManager<AccountId, CurrencyId, Balance> {
	fn get_target_amount(
		supply_currency_id: CurrencyId,
		target_currency_id: CurrencyId,
		supply_currency_amount: Balance,
	) -> Balance;

	fn get_supply_amount(
		supply_currency_id: CurrencyId,
		target_currency_id: CurrencyId,
		target_currency_amount: Balance,
	) -> Balance;

	fn exchange_currency(
		who: AccountId,
		supply_currency_id: CurrencyId,
		supply_amount: Balance,
		target_currency_id: CurrencyId,
		acceptable_target_amount: Balance,
	) -> rstd::result::Result<Balance, DispatchError>;

	fn get_exchange_slippage(
		supply_currency_id: CurrencyId,
		target_currency_id: CurrencyId,
		supply_amount: Balance,
	) -> Option<Ratio>;
}

impl<AccountId, CurrencyId, Balance> DEXManager<AccountId, CurrencyId, Balance> for ()
where
	Balance: Default,
{
	fn get_target_amount(
		_supply_currency_id: CurrencyId,
		_target_currency_id: CurrencyId,
		_supply_currency_amount: Balance,
	) -> Balance {
		Default::default()
	}

	fn get_supply_amount(
		_supply_currency_id: CurrencyId,
		_target_currency_id: CurrencyId,
		_target_currency_amount: Balance,
	) -> Balance {
		Default::default()
	}

	fn exchange_currency(
		_who: AccountId,
		_supply_currency_id: CurrencyId,
		_supply_amount: Balance,
		_target_currency_id: CurrencyId,
		_acceptable_target_amount: Balance,
	) -> rstd::result::Result<Balance, DispatchError> {
		Ok(Default::default())
	}

	fn get_exchange_slippage(
		_supply_currency_id: CurrencyId,
		_target_currency_id: CurrencyId,
		_supply_amount: Balance,
	) -> Option<Ratio> {
		None
	}
}

pub trait CDPTreasury<AccountId> {
	type Balance;
	type CurrencyId;

	fn get_surplus_pool() -> Self::Balance;
	fn get_debit_pool() -> Self::Balance;
	fn get_total_collaterals(id: Self::CurrencyId) -> Self::Balance;

	fn on_system_debit(amount: Self::Balance) -> DispatchResult;
	fn on_system_surplus(amount: Self::Balance) -> DispatchResult;
	fn deposit_backed_debit(who: &AccountId, amount: Self::Balance) -> DispatchResult;
	fn withdraw_backed_debit(who: &AccountId, amount: Self::Balance) -> DispatchResult;
	fn transfer_system_surplus(to: &AccountId, amount: Self::Balance) -> DispatchResult;
	fn transfer_surplus_from(from: &AccountId, amount: Self::Balance) -> DispatchResult;
	fn transfer_system_collateral(
		currency_id: Self::CurrencyId,
		to: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;
	fn transfer_collateral_from(
		currency_id: Self::CurrencyId,
		from: &AccountId,
		amount: Self::Balance,
	) -> DispatchResult;
}

pub trait CDPTreasuryExtended<AccountId>: CDPTreasury<AccountId> {
	fn get_stable_currency_ratio(amount: Self::Balance) -> Ratio;
	fn swap_collateral_to_stable(
		currency_id: Self::CurrencyId,
		supply_amount: Self::Balance,
		target_amount: Self::Balance,
	) -> DispatchResult;
	fn create_collateral_auctions(
		currency_id: Self::CurrencyId,
		amount: Self::Balance,
		target: Self::Balance,
		refund_receiver: AccountId,
	);
}

pub trait PriceProvider<CurrencyId, Price> {
	fn get_price(base: CurrencyId, quote: CurrencyId) -> Option<Price>;
	fn lock_price(currency_id: CurrencyId);
	fn unlock_price(currency_id: CurrencyId);
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
pub trait EmergencyShutdown {
	fn on_emergency_shutdown();
}
