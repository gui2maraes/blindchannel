use crate::domain::key::{PublicJwk, KeyName};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use sqlx::PgPool;
use axum::extract::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    alias: KeyName,
}
#[tracing::instrument(skip(pool), name = "get public_key by name")]
pub async fn fetch_alias(
    State(pool): State<PgPool>,
    Path(params): Path<Params>,
) -> Result<Json<PublicJwk>, StatusCode> {
    let key = match get_key_by_name(&pool, params.alias.name()).await {
        Ok(k) => k,
        Err(sqlx::Error::RowNotFound) => return Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(key))
}

#[tracing::instrument(skip(pool), name = "name fuzzy search")]
pub async fn search_alias(
    State(pool): State<PgPool>,
    Path(params): Path<Params>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let names = match name_fuzzy_search(&pool, params.alias.name()).await {
        Ok(names) => names,
        Err(e) => {
            tracing::error!("database error: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    Ok(Json(names))
}

async fn get_key_by_name(pool: &PgPool, name: &str) -> sqlx::Result<PublicJwk> {
    let key_str = sqlx::query!(
        r#"SELECT public_key as "key: sqlx::types::Json<PublicJwk>"
        FROM keymap WHERE name = $1"#,
        name
    )
    .fetch_one(pool)
    .await?;
    Ok(key_str.key.0)
}

async fn name_fuzzy_search(pool: &PgPool, name: &str) -> sqlx::Result<Vec<String>> {
    let names = sqlx::query!(
        r#"SELECT name
        FROM keymap
        WHERE name % $1
        ORDER BY similarity(name, $1) DESC, name
        LIMIT 10"#,
        name
    )
    .fetch_all(pool)
    .await?;
    Ok(names.into_iter().map(|r| r.name).collect())
}
