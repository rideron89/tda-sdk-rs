//! SDK for interacting with the TD Ameritrade API.
//!
//! [Developer Documentation](https://developer.tdameritrade.com/)
//!
//! **Important**: Before starting, you will need to make sure you have a
//! developer application created (thus a client ID), and a valid refresh token.
//! If you need help with either of these steps, you should refer to the
//! following [API Guide Pages](https://developer.tdameritrade.com/guides):
//!
//! - [Getting Start](https://developer.tdameritrade.com/content/getting-started)
//! - [Simple Auth for Local Apps](https://developer.tdameritrade.com/content/simple-auth-local-apps)
//! - [Authentication FAQ](https://developer.tdameritrade.com/content/authentication-faq)
//!
//! ### Create a Client and Fetch a New Token
//!
//! After creating a [`Client`](struct.Client.html), you will need to give it an
//! access token. You can either use an existing one from your database or
//! filesystem, or fetch a new one from the API.
//!
//! ```no_run
//! # use tda_sdk::Client;
//! let mut client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);
//!
//! let access_token = client.get_access_token().unwrap();
//!
//! // We must convert the token response into a token usable by the client.
//! client.set_access_token(&Some(access_token.into()));
//! ```
//!
//! ### Create a Client and Use an Old Token
//!
//! ```no_run
//! use tda_sdk::{AccessToken, Client};
//!
//! let access_token = AccessToken {
//!     expires_at: 0,
//!     token: "YOUR_TOKEN_STRING".to_string(),
//!     scope: Vec::new(),
//! };
//!
//! let client = Client::new("CLIENT_ID", "REFRESH_TOKEN", Some(access_token));
//! ```
//!
//! ### Full Example for Fetching All Accounts
//!
//! After a token has been set, you may call any of the API methods. You can
//! view all request parameters in the [params](params/index.html) module.
//!
//! ```no_run
//! use tda_sdk::{
//!     Client,
//!     params::GetAccountsParams,
//!     responses::SecuritiesAccount,
//! };
//!
//! let mut client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);
//!
//! let access_token = client.get_access_token().unwrap();
//! client.set_access_token(&Some(access_token.into()));
//!
//! let accounts = client.get_accounts(GetAccountsParams::default()).unwrap();
//!
//! for account in accounts {
//!     match account.securities_account {
//!         SecuritiesAccount::MarginAccount { r#type, account_id, .. } => {
//!             println!("Account ID: {}", account_id);
//!             println!("Account Type: {}", r#type);
//!         }
//!     }
//! }
//! ```
//!
//! ### Token Structure and Expiration
//!
//! This library does not handle token expirations, that is up to the user.
//! However, the [`AccessToken`](struct.AccessToken.html) struct has a handy
//! method for detecting its expiration status.
//!
//! **Note**: The `get_access_token()` response has a different structure than
//! the token expected by the client. You will need to parse the response.
//!
//! ```no_run
//! # use tda_sdk::{AccessToken, Client};
//! # let client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);
//! let access_token: AccessToken = client.get_access_token().unwrap().into();
//!
//! if access_token.has_expired() {
//!     panic!("Token has expired!");
//! }
//! ```

#[macro_use] extern crate serde;

pub mod params;
pub mod responses;

use chrono::Utc;
use params::{
    GetAccountParams,
    GetAccountsParams,
    GetMoversParams,
    GetPriceHistoryParams,
};
use thiserror::Error;

use std::io;

/// Base path for the TDA API.
pub const TDA_API_BASE: &str = "https://api.tdameritrade.com/v1";

/// Client for interacting with the TDA API.
///
/// Most API methods will panic if an access token is not set.
#[derive(Debug)]
pub struct Client {
    pub access_token: Option<AccessToken>,
    client_id: String,
    refresh_token: String,
}

impl<'a> Client {
    /// Create a new client with a client ID and refresh token.
    pub fn new(client_id: &'a str, refresh_token: &'a str, access_token: Option<AccessToken>) -> Self {
        Self {
            access_token,
            client_id: client_id.to_string(),
            refresh_token: refresh_token.to_string(),
        }
    }

    /// Set the internal access token of the client.
    pub fn set_access_token(&mut self, access_token: &Option<AccessToken>) -> &mut Self {
        self.access_token = access_token.clone();

        self
    }

    /// Get a new access token from the API.
    pub fn get_access_token(&self) -> Result<responses::AccessTokenResponse, ClientError> {
        let url = format!("{}/oauth2/token", TDA_API_BASE);

        let response = ureq::post(&url)
            .send_form(&[
                ("grant_type", "refresh_token"),
                ("refresh_token", &self.refresh_token),
                ("client_id", &self.client_id),
           ]);
        let status = response.status();
        let body = response.into_string().map_err(ClientError::ReadResponse)?;

        if status != 200 {
            return Err(ClientError::NotHttpOk(status, body))
        }

        serde_json::from_str(&body).map_err(ClientError::ParseResponse)
    }

    /// Account balances, positions, and orders for a specific account.
    ///
    /// [API documentation](https://developer.tdameritrade.com/account-access/apis/get/accounts/%7BaccountId%7D-0)
    pub fn get_account(&self, account_id: &'a str, params: GetAccountParams) -> Result<responses::Account, ClientError> {
        if self.access_token.is_none() {
            panic!("Client does not have a token set!");
        }

        let access_token = self.access_token.as_ref().unwrap();
        let url = format!("{}/accounts/{}", TDA_API_BASE, account_id);

        let mut request = ureq::get(&url);
        request.set("Authorization", &format!("Bearer {}", access_token.token));

        if let Some(fields) = params.fields {
            request.query("fields", &fields);
        }

        let response = request.call();
        let status = response.status();
        let body = response.into_string().map_err(ClientError::ReadResponse)?;

        if status != 200 {
            return Err(ClientError::NotHttpOk(status, body));
        }

        serde_json::from_str(&body).map_err(ClientError::ParseResponse)
    }

    /// Account balances, positions, and orders for all linked accounts.
    ///
    /// [Api Documentation](https://developer.tdameritrade.com/account-access/apis/get/accounts-0)
    pub fn get_accounts(&self, params: GetAccountsParams) -> Result<Vec<responses::Account>, ClientError> {
        if self.access_token.is_none() {
            panic!("Client does not have a token set!");
        }

        let access_token = self.access_token.as_ref().unwrap();
        let url = format!("{}/accounts", TDA_API_BASE);

        let mut request = ureq::get(&url);
        request.set("Authorization", &format!("Bearer {}", access_token.token));

        if let Some(fields) = params.fields {
            request.query("fields", &fields);
        }

        let response = request.call();
        let status = response.status();
        let body = response.into_string().map_err(ClientError::ReadResponse)?;

        if status != 200 {
            return Err(ClientError::NotHttpOk(status, body));
        }

        serde_json::from_str(&body).map_err(ClientError::ParseResponse)
    }

    /// Top 10 (up or down) movers by value or percent for a particular market
    ///
    /// [API Documentation](https://developer.tdameritrade.com/movers/apis/get/marketdata/%7Bindex%7D/movers)
    pub fn get_movers(&self, index: &'a str, params: GetMoversParams) -> Result<Vec<responses::Mover>, ClientError> {
        if self.access_token.is_none() {
            panic!("Client does not have a token set!");
        }

        let access_token = self.access_token.as_ref().unwrap();
        let url = format!("{}/marketdata/{}/movers", TDA_API_BASE, index);

        let mut request = ureq::get(&url);
        request.set("Authorization", &format!("Bearer {}", access_token.token));

        if let Some(direction) = params.direction {
            request.query("direction", &direction);
        }

        if let Some(change) = params.change {
            request.query("change", &change);
        }

        let response = request.call();
        let status = response.status();
        let body = response.into_string().map_err(ClientError::ReadResponse)?;

        if status != 200 {
            return Err(ClientError::NotHttpOk(status, body));
        }

        serde_json::from_str(&body).map_err(ClientError::ParseResponse)
    }

    /// Get price history for a symbol
    ///
    /// [API Documentation](https://developer.tdameritrade.com/price-history/apis/get/marketdata/%7Bsymbol%7D/pricehistory)
    pub fn get_price_history(&self, symbol: &str, params: GetPriceHistoryParams) -> Result<responses::GetPriceHistoryResponse, ClientError> {
        if self.access_token.is_none() {
            panic!("Client does not have a token set!");
        }

        let access_token = self.access_token.as_ref().unwrap();
        let url = format!("{}/marketdata/{}/pricehistory", TDA_API_BASE, symbol);

        let mut request = ureq::get(&url);
        request.set("Authorization", &format!("Bearer {}", access_token.token));

        if let Some(period_type) = params.period_type {
            request.query("periodType", &period_type);
        }

        if let Some(period) = params.period {
            request.query("period", &period);
        }

        if let Some(frequency_type) = params.frequency_type {
            request.query("frequencyType", &frequency_type);
        }

        if let Some(frequency) = params.frequency {
            request.query("frequency", &frequency);
        }

        if let Some(end_date) = params.end_date {
            request.query("endDate", &end_date);
        }

        if let Some(start_date) = params.start_date {
            request.query("startDate", &start_date);
        }

        if let Some(need_extended_hours_data) = params.need_extended_hours_data {
            request.query("needExtendedHoursData", &need_extended_hours_data.to_string());
        }

        let response = request.call();
        let status = response.status();
        let body = response.into_string().map_err(ClientError::ReadResponse)?;

        if status != 200 {
            return Err(ClientError::NotHttpOk(status, body));
        }

        serde_json::from_str(&body).map_err(ClientError::ParseResponse)
    }
}

/// API access token.
#[derive(Clone, Debug, Serialize)]
pub struct AccessToken {
    /// Timestamp in milliseconds when the token expires.
    pub expires_at: i64,
    pub scope: Vec<String>,
    pub token: String,
}

impl From<responses::AccessTokenResponse> for AccessToken {
    fn from(response: responses::AccessTokenResponse) -> Self {
        let now = Utc::now().naive_utc().timestamp_millis();

        Self {
            token: response.access_token,
            expires_at: now + response.expires_in,
            scope: response.scope.split(' ').map(|v| v.to_string()).collect(),
        }
    }
}

impl AccessToken {
    /// Return true if the access token has expired.
    #[allow(dead_code)]
    pub fn has_expired(&self) -> bool {
        self.expires_at >= Utc::now().naive_utc().timestamp_millis()
    }
}

/// Represents all possible errors the `Client` might encounter.
#[derive(Debug, Error)]
pub enum ClientError {
    /// Received a non-200 HTTP status code from the server.
    #[error("Received a {0} HTTP code: {1}")]
    NotHttpOk(u16, String),

    /// Was unable to parse the response into a usable struct.
    #[error("Failed to parse response: {0}")]
    ParseResponse(#[from] serde_json::error::Error),

    /// Was unable to read the response string.
    #[error("Failed to read response string: {0}")]
    ReadResponse(#[from] io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, OpenOptions};

    /// Configuration file path.
    const CONFIG_FILE: &'static str = "./.test.env";

    /// Local token file path.
    const TOKEN_FILE_PATH: &'static str = "./.token.json";

    /// Configuration settings found in `.test.env`.
    #[derive(Debug)]
    struct Config {
        tda_client_id: String,
        tda_refresh_token: String,
    }

    /// Get a client with a working access token.
    ///
    /// Handles loading/saving the local file token, as well as fetching a new
    /// one if necessary.
    fn get_working_client() -> Client {
        let config = load_config();
        let mut client = Client::new(&config.tda_client_id, &config.tda_refresh_token, None);

        let mut token: AccessToken = match OpenOptions::new().open(TOKEN_FILE_PATH) {
            Ok(_) => load_token().into(),
            Err(_) => {
                let token: AccessToken = client.get_access_token().unwrap().into();
                save_token(&token);

                token
            },
        };

        if token.has_expired() {
            token = client.get_access_token().unwrap().into();
            save_token(&token);
        }

        client.set_access_token(&Some(token));

        client
    }

    /// Load config settings from `.test.env`.
    fn load_config() -> Config {
        dotenv::from_path(CONFIG_FILE).ok();

        Config {
            tda_client_id: dotenv::var("TDA_CLIENT_ID").unwrap(),
            tda_refresh_token: dotenv::var("TDA_REFRESH_TOKEN").unwrap(),
        }
    }

    /// Load the token from the local `.token.json` file.
    ///
    /// Panics if the file could not be found or accessed.
    fn load_token() -> responses::AccessTokenResponse {
        let token = fs::read_to_string(TOKEN_FILE_PATH).unwrap();

        serde_json::from_str(&token).unwrap()
    }

    /// Save a token to the local `.token.json` file.
    ///
    /// Panics if the file could not be written to.
    fn save_token(token: &AccessToken) {
        fs::write(TOKEN_FILE_PATH, serde_json::to_string(&token).unwrap()).unwrap();
    }

    #[test]
    fn get_access_token() {
        let config = load_config();
        let client = Client::new(&config.tda_client_id, &config.tda_refresh_token, None);

        let token = client.get_access_token().unwrap();

        assert_ne!(token.access_token.len(), 0);
    }

    #[test]
    fn set_access_token() {
        let config = load_config();
        let mut client = Client::new(&config.tda_client_id, &config.tda_refresh_token, None);

        let response = client.get_access_token().unwrap();
        let new_access_token = response.access_token.clone();

        client.set_access_token(&Some(response.into()));

        assert_eq!(new_access_token, client.access_token.unwrap().token);
    }

    #[test]
    fn get_account() {
        let client = get_working_client();

        let accounts = client.get_accounts(GetAccountsParams::default()).unwrap();

        match &accounts.get(0).unwrap().securities_account {
            responses::SecuritiesAccount::MarginAccount { account_id, .. } => {
                client.get_account(account_id, GetAccountParams::default()).unwrap();
            }
        }
    }

    #[test]
    fn get_accounts() {
        let client = get_working_client();

        let accounts = client.get_accounts(GetAccountsParams::default()).unwrap();

        assert_ne!(accounts.len(), 0);
    }

    #[test]
    fn get_movers() {
        let client = get_working_client();

        let _movers = client.get_movers("$DJI", GetMoversParams::default()).unwrap();

        // TODO: Make sure test the response is parsing, when we get data again.
    }

    #[test]
    fn get_price_history() {
        let client = get_working_client();

        let response = client.get_price_history("AAPL", GetPriceHistoryParams::default()).unwrap();

        assert_ne!(response.candles.len(), 0);
    }
}
