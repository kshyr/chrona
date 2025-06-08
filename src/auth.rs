use anyhow::{Result, anyhow};
use chrono::{DateTime, Duration, Utc};
use oauth2::{
    AccessToken, AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyExtraTokenFields, RedirectUrl, StandardTokenResponse, TokenResponse as _, TokenUrl,
    basic::{BasicClient, BasicTokenType},
    ureq,
};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};
use tracing::{debug, error};

const SERVICE_NAME: &str = "chrona-ticktick";

type TokenResponse = StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

#[derive(Serialize, Deserialize, Debug)]
struct CachedToken {
    pub access_token: String, // We only store the string, not the whole AccessToken struct
    pub expires_at: DateTime<Utc>,
    // Refresh token would go here if TickTick provided one
}

// 1. =================================================================
//    Functions for SECURELY SAVING and LOADING the token
// =================================================================

/// Saves the token securely.
/// The access token secret is stored in the OS keyring.
/// The expiration time is stored in a plain config file.
pub fn save_token(token_response: TokenResponse) -> Result<()> {
    let expires_in = token_response
        .expires_in()
        .unwrap_or(std::time::Duration::from_secs(0));
    let expires_at = Utc::now() + Duration::from_std(expires_in)?;

    // Save the secret access token to the keyring
    let entry = keyring::Entry::new(SERVICE_NAME, "user")?;
    entry.set_password(token_response.access_token().secret())?;

    // Save the non-secret metadata to a file
    let metadata = CachedToken {
        access_token: "stored_in_keyring".to_string(), // Placeholder
        expires_at,
    };
    let config_path = crate::store::get_config_dir()?.join("ticktick_token.json");
    let file = std::fs::File::create(config_path)?;
    serde_json::to_writer_pretty(file, &metadata)?;

    debug!("Token saved successfully.");
    Ok(())
}

/// Loads the token.
/// Retrieves the secret from the keyring and metadata from the config file.
pub fn load_token() -> Result<AccessToken> {
    // Load metadata from the file
    let config_path = crate::store::get_config_dir()?.join("ticktick_token.json");
    if !config_path.exists() {
        return Err(anyhow!("Token file not found."));
    }
    let file = std::fs::File::open(config_path)?;
    let metadata: CachedToken = serde_json::from_reader(file)?;

    // Check for expiration
    if Utc::now() >= metadata.expires_at {
        return Err(anyhow!("Token has expired."));
    }

    // Load secret from keyring
    let entry = keyring::Entry::new(SERVICE_NAME, "user")?;
    let secret = entry.get_password()?;

    debug!("Token loaded successfully.");
    Ok(AccessToken::new(secret))
}

pub fn run_interactive_auth() -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>>
{
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let redirect_url = listener.local_addr()?.to_string();

    let client_id = std::env::var("TICKTICK_CLIENT_ID").expect("TICKTICK_CLIENT_ID must be set.");
    let client_secret =
        std::env::var("TICKTICK_CLIENT_SECRET").expect("TICKTICK_CLIENT_SECRET must be set.");

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret))
        .set_auth_uri(AuthUrl::new(
            "https://ticktick.com/oauth/authorize".to_string(),
        )?)
        .set_token_uri(TokenUrl::new(
            "https://ticktick.com/oauth/token".to_string(),
        )?)
        .set_redirect_uri(RedirectUrl::new(format!("http://{}", redirect_url))?)
        .set_auth_type(AuthType::RequestBody);

    let scopes = ["tasks:read".to_string(), "tasks:write".to_string()];
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(oauth2::Scope::new(scopes[0].clone()))
        .add_scope(oauth2::Scope::new(scopes[1].clone()))
        .url();

    webbrowser::open(authorize_url.as_str())?;

    if let Some(Ok(mut stream)) = listener.incoming().next() {
        let code;
        let state;
        {
            let mut reader = BufReader::new(&stream);
            let mut request_line = String::new();
            reader.read_line(&mut request_line)?;

            let redirect_path = request_line.split_whitespace().nth(1).unwrap_or("/");
            let parsed_url =
                oauth2::url::Url::parse(&format!("http://localhost{}", redirect_path))?;
            let mut query_pairs = parsed_url.query_pairs();

            code = AuthorizationCode::new(
                query_pairs
                    .clone()
                    .find(|(key, _)| key == "code")
                    .map(|(_, value)| value.into_owned())
                    .ok_or_else(|| anyhow!("Missing 'code'"))?,
            );
            state = CsrfToken::new(
                query_pairs
                    .find(|(key, _)| key == "state")
                    .map(|(_, value)| value.into_owned())
                    .ok_or_else(|| anyhow!("Missing 'state'"))?,
            );
        }

        if state.secret() != csrf_state.secret() {
            return Err(anyhow!("CSRF token mismatch"));
        }

        let http_client = ureq::agent();
        let token_result = client
            .exchange_code(code)
            .add_extra_param("scope", scopes.join(" "))
            .request(&http_client);

        let message = "Authorization complete! You can close this window and return to Chrona.";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes())?;

        match token_result {
            Ok(token_response) => Ok(token_response),
            Err(e) => {
                let request_error = anyhow!("Failed to obtain token: {:?}", e);
                error!("{request_error:?}");
                Err(request_error)
            }
        }
    } else {
        Err(anyhow!("Failed to receive authorization callback."))
    }
}
