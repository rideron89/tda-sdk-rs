//! Structs and utilities for building API request parameters.

/// Parameters for the `get_account()` method.
///
/// [API Documentation](https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D-0)
#[derive(Debug)]
pub struct GetAccountParams {
    /// Balances displayed by default, additional fields can be added here by adding `positions` or `orders`
    ///
    /// Choices: `positions` or `orders`
    pub fields: Option<String>,
}

impl Default for GetAccountParams {
    fn default() -> Self {
        Self {
            fields: None,
        }
    }
}

/// Parameters for the `get_accounts()` method.
///
/// [API Documentation](https://developer.tdameritrade.com/account-access/apis/get/accounts-0)
#[derive(Debug)]
pub struct GetAccountsParams {
    /// Balances displayed by default, additional fields can be added here by adding `positions` or `orders`
    ///
    /// Choices: `positions` or `order`s
    pub fields: Option<String>,
}

impl Default for GetAccountsParams {
    fn default() -> Self {
        Self {
            fields: None,
        }
    }
}

/// Parameters for the `get_movers()` method.
///
/// [API Documentation](https://developer.tdameritrade.com/movers/apis/get/marketdata/%7Bindex%7D/movers)
#[derive(Debug)]
pub struct GetMoversParams {
    /// To return movers with the specified directions of up or down
    ///
    /// Choices: `up` or `down`
    pub change: Option<String>,

    /// To return movers with the specified change types of percent or value
    ///
    /// Choices: `value` or `percent`
    pub direction: Option<String>,
}

impl Default for GetMoversParams {
    fn default() -> Self {
        Self {
            change: None,
            direction: None,
        }
    }
}

/// Parameters for the `get_price_history()` method.
///
/// [API Documentation](https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory)
#[derive(Debug)]
pub struct GetPriceHistoryParams {
    /// End date as milliseconds since epoch. If startDate and endDate are
    /// provided, period should not be provided. Default is previous trading
    /// day.
    pub end_date: Option<String>,

    /// The type of frequency with which a new candle is formed.
    ///
    /// Valid frequencyTypes by periodType (defaults marked with an asterisk):
    ///
    /// `day`: minute*
    ///
    /// `month`: daily, weekly*
    ///
    /// `year`: daily, weekly, monthly*
    ///
    /// `ytd`: daily, weekly*
    pub frequency_type: Option<String>,

    /// The number of the frequencyType to be included in each candle.
    ///
    /// Valid frequencies by frequencyType (defaults marked with an asterisk):
    ///
    /// `minute`: 1*, 5, 10, 15, 30
    ///
    /// `daily`: 1*
    ///
    /// `weekly`: 1*
    ///
    /// `monthly`: 1*
    pub frequency: Option<String>,

    /// `true` to return extended hours data, `false` for regular market hours
    /// only. Default is `true`
    pub need_extended_hours_data: Option<bool>,

    /// The type of period to show. Valid values are `day`, `month`, `year`, or
    /// `ytd` (year to date). Default is `day`.
    pub period_type: Option<String>,

    /// The number of periods to show.
    ///
    /// Example: For a 2 day / 1 min chart, the values would be:
    ///
    /// `period`: 2
    ///
    /// `periodType`: day
    ///
    /// `frequency`: 1
    ///
    /// `frequencyType`: min
    ///
    /// Valid periods by periodType (defaults marked with an asterisk):
    ///
    /// `day`: 1, 2, 3, 4, 5, 10*
    ///
    /// `month`: 1*, 2, 3, 6
    ///
    /// `year`: 1*, 2, 3, 5, 10, 15, 20
    ///
    /// `ytd`: 1*
    pub period: Option<String>,

    /// Start date as milliseconds since epoch. If startDate and endDate are
    /// provided, period should not be provided.
    pub start_date: Option<String>,
}

impl Default for GetPriceHistoryParams {
    fn default() -> Self {
        Self {
            end_date: None,
            frequency_type: None,
            frequency: None,
            need_extended_hours_data: None,
            period_type: None,
            period: None,
            start_date: None,
        }
    }
}
