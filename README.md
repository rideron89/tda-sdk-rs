# tda-sdk-rs

> SDK for interacting with the TD Ameritrade API.

[Developer Documentation](https://developer.tdameritrade.com/)

**Important**: Before starting, you will need to make sure you have a
developer application created (thus a client ID), and a valid refresh token.
If you need help with either of these steps, you should refer to the
following [API Guide Pages](https://developer.tdameritrade.com/guides):

- [Getting Start](https://developer.tdameritrade.com/content/getting-started)
- [Simple Auth for Local Apps](https://developer.tdameritrade.com/content/simple-auth-local-apps)
- [Authentication FAQ](https://developer.tdameritrade.com/content/authentication-faq)

### Create a Client and Fetch a New Token

After creating a `Client`, you will need to give it an
access token. You can either use an existing one from your database or
filesystem, or fetch a new one from the API.

```rust
use tda_sdk::Client;

let mut client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);

let access_token = client.get_access_token().unwrap();

// We must convert the token response into a token usable by the client.
client.set_access_token(&Some(access_token.into()));
```

### Create a Client and Use an Old Token

```rust
use tda_sdk::{AccessToken, Client};

let access_token = AccessToken {
    expires_at: 0,
    token: "YOUR_TOKEN_STRING".to_string(),
    scope: Vec::new(),
};

let client = Client::new("CLIENT_ID", "REFRESH_TOKEN", Some(access_token));
```

### Full Example for Fetching All Accounts

After a token has been set, you may call any of the API methods. You can
view all request parameters in the params module.

```rust
use tda_sdk::{
    Client,
    params::GetAccountsParams,
    responses::SecuritiesAccount,
};

let mut client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);

let access_token = client.get_access_token().unwrap();
client.set_access_token(&Some(access_token.into()));

let accounts = client.get_accounts(GetAccountsParams::default()).unwrap();

for account in accounts {
    match account.securities_account {
        SecuritiesAccount::MarginAccount { r#type, account_id, .. } => {
            println!("Account ID: {}", account_id);
            println!("Account Type: {}", r#type);
        }
    }
}
```

### Token Structure and Expiration

This library does not handle token expirations, that is up to the user.
However, the `AccessToken` struct has a handy
method for detecting its expiration status.

**Note**: The `get_access_token()` response has a different structure than
the token expected by the client. You will need to parse the response.

```rust
use tda_sdk::{AccessToken, Client};

let client = Client::new("CLIENT_ID", "REFRESH_TOKEN", None);
let access_token: AccessToken = client.get_access_token().unwrap().into();

if access_token.has_expired() {
    panic!("Token has expired!");
}
```
