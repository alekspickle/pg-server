use crate::data::{self, Account, AccountLogin};
use axum::{debug_handler, extract::State, http::StatusCode, Json};
use sqlx::postgres::PgPool;
use tracing::{debug, info};

/// Create user.
///
/// Extract json data, hash the password and insert the new user's data
#[debug_handler]
pub(crate) async fn create_account(
    State(pool): State<PgPool>,
    Json(account): Json<Account>,
) -> Result<String, (StatusCode, String)> {
    let hashed =
        data::hash_password(&account.pass).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    debug!(hashed=%hashed, "Hashed pass");

    let s = sqlx::query(&data::queries::create_user())
        .bind(account.car)
        .bind(&account.email)
        .bind(account.bank_details)
        .bind(hashed)
        .execute(&pool)
        .await
        .map_err(internal_error)?;

    let s = s.rows_affected();

    info!(s=%s, "Rows affected");

    Ok(format!(
        "Account for {} successfully created!",
        account.email
    ))
}

/// Login
///
/// Extract account info, get account by the hashed password, return 200 if found
pub(crate) async fn login(
    State(pool): State<PgPool>,
    Json(account): Json<AccountLogin>,
) -> Result<String, (StatusCode, String)> {
    info!(login=%account, "Trying to login");

    let hashed =
        data::hash_password(&account.pass).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let s = sqlx::query_scalar(&data::queries::login())
        .bind(hashed)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()));
    s
}

/// Get details
///
/// Query account by the email. Return account details
/// Note: In production you'd probably want to check users permissions to do that
/// but for brevity we omit it
pub(crate) async fn get_details(
    State(pool): State<PgPool>,
    email: String,
) -> Result<String, (StatusCode, String)> {
    let s = sqlx::query_scalar(&data::queries::get_details())
        .bind(email)
        .fetch_one(&pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()));
    s
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
