//! Structs and utilities for handling API response data.

/// Response returned by the `get_access_token()` method.
#[derive(Debug, Deserialize, Serialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub scope: String,
    pub expires_in: i64,
}

/// Response returned by the `get_price_history()` method.
#[derive(Debug, Deserialize)]
pub struct GetPriceHistoryResponse {
    pub candles: Vec<Candle>,
    pub empty: bool,
    pub symbol: String,
}

/// Individual candle item in [`GetPriceHistoryResponse`](struct.GetPriceHistoryResponse.html).
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Candle {
    pub close: f64,
    pub datetime: usize,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub volume: i64,
}

/// Individual response item returned by the `get_movers()` method.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mover {
    pub change: f64,
    pub description: String,
    pub direction: String,
    pub last: f64,
    pub symbol: String,
    pub total_volume: i64,
}

/// Individual response item returned by the `get_account()` and
/// `get_accounts()` methods.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub securities_account: SecuritiesAccount,
}

/// Securities Account item in [`Account`](struct.Account.html)
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SecuritiesAccount {
    #[serde(rename_all = "camelCase")]
    MarginAccount {
        r#type: String,
        account_id: String,
        round_trips: usize,
        is_day_trader: bool,
        is_closing_only_restricted: bool,
        initial_balances: InitialBalances,
        current_balances: CurrentBalances,
        projected_balances: ProjectedBalances,
    },
}

/// Initial Balances item in [`SecuritiesAccount`](enum.SecuritiesAccount.html)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialBalances {
    pub account_value: f64,
    pub accrued_interest: f64,
    pub bond_value: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub cash_debit_call_value: f64,
    pub cash_receipts: f64,
    pub is_in_call: bool,
    pub liquidation_value: f64,
    pub long_option_market_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub pending_deposits: f64,
    pub short_option_market_value: f64,
    pub short_stock_value: f64,
    pub unsettled_cash: f64,
}

/// Current Balances item in [`SecuritiesAccount`](enum.SecuritiesAccount.html)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentBalances {
    pub accrued_interest: f64,
    pub bond_value: f64,
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
    pub cash_balance: f64,
    pub cash_call: f64,
    pub cash_debit_call_value: f64,
    pub cash_receipts: f64,
    pub liquidation_value: f64,
    pub long_market_value: f64,
    pub long_option_market_value: f64,
    pub money_market_fund: f64,
    pub mutual_fund_value: f64,
    pub pending_deposits: f64,
    pub savings: f64,
    pub short_market_value: f64,
    pub short_option_market_value: f64,
    pub total_cash: f64,
    pub unsettled_cash: f64,
}

/// Projected Balances item in [`SecuritiesAccount`](enum.SecuritiesAccount.html)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectedBalances {
    pub cash_available_for_trading: f64,
    pub cash_available_for_withdrawal: f64,
}
