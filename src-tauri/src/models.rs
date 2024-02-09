use std::collections::HashMap;
use std::fs;

use log::error;
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::types::{Json, JsonValue};
use sqlx::{Pool, Sqlite};
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

fn default_true() -> bool {
    true
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Settings {
    pub id: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub theme: String,
    pub appearance: String,
    pub update_channel: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Workspace {
    pub id: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub description: String,
    pub variables: Json<Vec<EnvironmentVariable>>,

    // Settings
    #[serde(default = "default_true")]
    pub setting_validate_certificates: bool,
    #[serde(default = "default_true")]
    pub setting_follow_redirects: bool,
    pub setting_request_timeout: i64,
}

// Implement default for Workspace
impl Workspace {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            model: "workspace".to_string(),
            setting_validate_certificates: true,
            setting_follow_redirects: true,
            ..Default::default()
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
pub struct CookieX {}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct CookieJar {
    pub id: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub workspace_id: String,
    pub name: String,
    pub cookies: Json<Vec<JsonValue>>,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Environment {
    pub id: String,
    pub workspace_id: String,
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub variables: Json<Vec<EnvironmentVariable>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct EnvironmentVariable {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct Folder {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: String,
    pub workspace_id: String,
    pub folder_id: Option<String>,
    pub model: String,
    pub name: String,
    pub sort_priority: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct HttpRequestHeader {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct HttpUrlParameter {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub name: String,
    pub value: String,
}

fn default_http_request_method() -> String {
    "GET".to_string()
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct HttpRequest {
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub id: String,
    pub workspace_id: String,
    pub folder_id: Option<String>,
    pub model: String,
    pub sort_priority: f64,
    pub name: String,
    pub url: String,
    pub url_parameters: Json<Vec<HttpUrlParameter>>,
    #[serde(default = "default_http_request_method")]
    pub method: String,
    pub body: Json<HashMap<String, JsonValue>>,
    pub body_type: Option<String>,
    pub authentication: Json<HashMap<String, JsonValue>>,
    pub authentication_type: Option<String>,
    pub headers: Json<Vec<HttpRequestHeader>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct HttpResponseHeader {
    pub name: String,
    pub value: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct HttpResponse {
    pub id: String,
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub error: Option<String>,
    pub url: String,
    pub content_length: Option<i64>,
    pub version: Option<String>,
    pub elapsed: i64,
    pub elapsed_headers: i64,
    pub remote_addr: Option<String>,
    pub status: i64,
    pub status_reason: Option<String>,
    pub body_path: Option<String>,
    pub headers: Json<Vec<HttpResponseHeader>>,
}

impl HttpResponse {
    pub(crate) fn new() -> Self {
        Self {
            model: "http_response".to_string(),
            ..Default::default()
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct GrpcRequest {
    pub id: String,
    pub model: String,
    pub workspace_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub folder_id: Option<String>,
    pub name: String,
    pub sort_priority: f64,
    pub url: String,
    pub service: Option<String>,
    pub method: Option<String>,
    pub message: String,
    pub proto_files: Json<Vec<String>>,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct GrpcConnection {
    pub id: String,
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub service: String,
    pub method: String,
    pub elapsed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct GrpcMessage {
    pub id: String,
    pub model: String,
    pub workspace_id: String,
    pub request_id: String,
    pub connection_id: String,
    pub created_at: NaiveDateTime,
    pub message: String,
    pub is_server: bool,
    pub is_info: bool,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct GrpcResponse {
    pub id: String,
    pub model: String,
    pub workspace_id: String,
    pub grpc_endpoint_id: String,
    pub grpc_connection_id: String,
    pub request_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
pub struct KeyValue {
    pub model: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub namespace: String,
    pub key: String,
    pub value: String,
}

pub async fn set_key_value_string(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
    value: &str,
) -> (KeyValue, bool) {
    let encoded = serde_json::to_string(value);
    set_key_value_raw(app_handle, namespace, key, &encoded.unwrap()).await
}

pub async fn set_key_value_int(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
    value: i32,
) -> (KeyValue, bool) {
    let encoded = serde_json::to_string(&value);
    set_key_value_raw(app_handle, namespace, key, &encoded.unwrap()).await
}

pub async fn get_key_value_string(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
    default: &str,
) -> String {
    match get_key_value_raw(app_handle, namespace, key).await {
        None => default.to_string(),
        Some(v) => {
            let result = serde_json::from_str(&v.value);
            match result {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed to parse string key value: {}", e);
                    default.to_string()
                }
            }
        }
    }
}

pub async fn get_key_value_int(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
    default: i32,
) -> i32 {
    match get_key_value_raw(app_handle, namespace, key).await {
        None => default.clone(),
        Some(v) => {
            let result = serde_json::from_str(&v.value);
            match result {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed to parse int key value: {}", e);
                    default.clone()
                }
            }
        }
    }
}

pub async fn set_key_value_raw(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
    value: &str,
) -> (KeyValue, bool) {
    let db = get_db(app_handle).await;
    let existing = get_key_value_raw(app_handle, namespace, key).await;
    sqlx::query!(
        r#"
            INSERT INTO key_values (namespace, key, value)
            VALUES (?, ?, ?) ON CONFLICT DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               value = excluded.value
        "#,
        namespace,
        key,
        value,
    )
    .execute(&db)
    .await
    .expect("Failed to insert key value");

    let kv = get_key_value_raw(app_handle, namespace, key)
        .await
        .expect("Failed to get key value");
    (kv, existing.is_none())
}

pub async fn get_key_value_raw(
    app_handle: &AppHandle,
    namespace: &str,
    key: &str,
) -> Option<KeyValue> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        KeyValue,
        r#"
            SELECT model, created_at, updated_at, namespace, key, value
            FROM key_values
            WHERE namespace = ? AND key = ?
        "#,
        namespace,
        key,
    )
    .fetch_one(&db)
    .await
    .ok()
}

pub async fn list_workspaces(app_handle: &AppHandle) -> Result<Vec<Workspace>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Workspace,
        r#"
            SELECT
                id, model, created_at, updated_at, name, description, setting_request_timeout,
                setting_follow_redirects, setting_validate_certificates,
                variables AS "variables!: sqlx::types::Json<Vec<EnvironmentVariable>>"
            FROM workspaces
        "#,
    )
    .fetch_all(&db)
    .await
}

pub async fn get_workspace(app_handle: &AppHandle, id: &str) -> Result<Workspace, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Workspace,
        r#"
            SELECT
                id, model, created_at, updated_at, name, description, setting_request_timeout,
                setting_follow_redirects, setting_validate_certificates,
                variables AS "variables!: sqlx::types::Json<Vec<EnvironmentVariable>>"
            FROM workspaces WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn delete_workspace(app_handle: &AppHandle, id: &str) -> Result<Workspace, sqlx::Error> {
    let db = get_db(app_handle).await;
    let workspace = get_workspace(app_handle, id).await?;
    let _ = sqlx::query!(
        r#"
            DELETE FROM workspaces
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    for r in list_responses_by_workspace_id(app_handle, id).await? {
        delete_http_response(app_handle, &r.id).await?;
    }

    emit_deleted_model(app_handle, workspace)
}

pub async fn get_cookie_jar(app_handle: &AppHandle, id: &str) -> Result<CookieJar, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        CookieJar,
        r#"
            SELECT
                id, model, created_at, updated_at, workspace_id, name,
                cookies AS "cookies!: sqlx::types::Json<Vec<JsonValue>>"
            FROM cookie_jars WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_cookie_jars(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<CookieJar>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        CookieJar,
        r#"
            SELECT
                id, model, created_at, updated_at, workspace_id, name,
                cookies AS "cookies!: sqlx::types::Json<Vec<JsonValue>>"
            FROM cookie_jars WHERE workspace_id = ?
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn delete_cookie_jar(app_handle: &AppHandle, id: &str) -> Result<CookieJar, sqlx::Error> {
    let cookie_jar = get_cookie_jar(app_handle, id).await?;
    let db = get_db(app_handle).await;

    let _ = sqlx::query!(
        r#"
            DELETE FROM cookie_jars
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, cookie_jar)
}

pub async fn duplicate_grpc_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcRequest, sqlx::Error> {
    let mut request = get_grpc_request(app_handle, id).await?.clone();
    request.id = "".to_string();
    upsert_grpc_request(app_handle, &request).await
}

pub async fn upsert_grpc_request(
    app_handle: &AppHandle,
    request: &GrpcRequest,
) -> Result<GrpcRequest, sqlx::Error> {
    let db = get_db(app_handle).await;
    let id = match request.id.as_str() {
        "" => generate_id(Some("gr")),
        _ => request.id.to_string(),
    };
    let trimmed_name = request.name.trim();
    sqlx::query!(
        r#"
            INSERT INTO grpc_requests (
                id, name, workspace_id, folder_id, sort_priority, url, service, method, message,
                proto_files
             )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
                updated_at = CURRENT_TIMESTAMP,
                name = excluded.name,
                folder_id = excluded.folder_id,
                sort_priority = excluded.sort_priority,
                url = excluded.url,
                service = excluded.service,
                method = excluded.method,
                message = excluded.message,
                proto_files = excluded.proto_files
        "#,
        id,
        trimmed_name,
        request.workspace_id,
        request.folder_id,
        request.sort_priority,
        request.url,
        request.service,
        request.method,
        request.message,
        request.proto_files,
    )
    .execute(&db)
    .await?;

    match get_grpc_request(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn get_grpc_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcRequest, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        GrpcRequest,
        r#"
            SELECT
                id, model, workspace_id, folder_id, created_at, updated_at, name, sort_priority,
                url, service, method, message,
                proto_files AS "proto_files!: sqlx::types::Json<Vec<String>>"
            FROM grpc_requests
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_grpc_requests(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<GrpcRequest>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        GrpcRequest,
        r#"
            SELECT
                id, model, workspace_id, folder_id, created_at, updated_at, name, sort_priority,
                url, service, method, message,
                proto_files AS "proto_files!: sqlx::types::Json<Vec<String>>"
            FROM grpc_requests
            WHERE workspace_id = ?
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn upsert_grpc_connection(
    app_handle: &AppHandle,
    connection: &GrpcConnection,
) -> Result<GrpcConnection, sqlx::Error> {
    let db = get_db(&app_handle).await;
    let id = match connection.id.as_str() {
        "" => generate_id(Some("gc")),
        _ => connection.id.to_string(),
    };
    sqlx::query!(
        r#"
            INSERT INTO grpc_connections (
                id, workspace_id, request_id, service, method, elapsed
            )
            VALUES (?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
                updated_at = CURRENT_TIMESTAMP,
                service = excluded.service,
                method = excluded.method,
                elapsed = excluded.elapsed
        "#,
        id,
        connection.workspace_id,
        connection.request_id,
        connection.service,
        connection.method,
        connection.elapsed,
    )
    .execute(&db)
    .await?;

    match get_grpc_connection(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn get_grpc_connection(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcConnection, sqlx::Error> {
    let db = get_db(&app_handle).await;
    sqlx::query_as!(
        GrpcConnection,
        r#"
            SELECT
                id, model, workspace_id, request_id, created_at, updated_at, service,
                method, elapsed
            FROM grpc_connections
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_grpc_connections(
    app_handle: &AppHandle,
    request_id: &str,
) -> Result<Vec<GrpcConnection>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        GrpcConnection,
        r#"
            SELECT
                id, model, workspace_id, request_id, created_at, updated_at, service,
                method, elapsed
            FROM grpc_connections
            WHERE request_id = ?
            ORDER BY created_at DESC
        "#,
        request_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn upsert_grpc_message(
    app_handle: &AppHandle,
    message: &GrpcMessage,
) -> Result<GrpcMessage, sqlx::Error> {
    let db = get_db(app_handle).await;
    let id = match message.id.as_str() {
        "" => generate_id(Some("gm")),
        _ => message.id.to_string(),
    };
    sqlx::query!(
        r#"
            INSERT INTO grpc_messages (
                id, workspace_id, request_id, connection_id, message, is_server, is_info
            )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
                updated_at = CURRENT_TIMESTAMP,
                message = excluded.message,
                is_server = excluded.is_server,
                is_info = excluded.is_info
        "#,
        id,
        message.workspace_id,
        message.request_id,
        message.connection_id,
        message.message,
        message.is_server,
        message.is_info,
    )
    .execute(&db)
    .await?;

    match get_grpc_message(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn get_grpc_message(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcMessage, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        GrpcMessage,
        r#"
            SELECT
                id, model, workspace_id, request_id, connection_id, created_at, message,
                is_server, is_info
            FROM grpc_messages
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_grpc_messages(
    app_handle: &AppHandle,
    connection_id: &str,
) -> Result<Vec<GrpcMessage>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        GrpcMessage,
        r#"
            SELECT
                id, model, workspace_id, request_id, connection_id, created_at, message,
                is_server, is_info
            FROM grpc_messages
            WHERE connection_id = ?
        "#,
        connection_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn upsert_cookie_jar(
    app_handle: &AppHandle,
    cookie_jar: &CookieJar,
) -> Result<CookieJar, sqlx::Error> {
    let id = match cookie_jar.id.as_str() {
        "" => generate_id(Some("cj")),
        _ => cookie_jar.id.to_string(),
    };
    let trimmed_name = cookie_jar.name.trim();

    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            INSERT INTO cookie_jars (
                id, workspace_id, name, cookies
             )
            VALUES (?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               name = excluded.name,
               cookies = excluded.cookies
        "#,
        id,
        cookie_jar.workspace_id,
        trimmed_name,
        cookie_jar.cookies,
    )
    .execute(&db)
    .await?;

    match get_cookie_jar(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn list_environments(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<Environment>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Environment,
        r#"
            SELECT id, workspace_id, model, created_at, updated_at, name,
                variables AS "variables!: sqlx::types::Json<Vec<EnvironmentVariable>>"
            FROM environments
            WHERE workspace_id = ?
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn delete_environment(
    app_handle: &AppHandle,
    id: &str,
) -> Result<Environment, sqlx::Error> {
    let db = get_db(app_handle).await;
    let env = get_environment(app_handle, id).await?;
    let _ = sqlx::query!(
        r#"
            DELETE FROM environments
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, env)
}

async fn get_settings(app_handle: &AppHandle) -> Result<Settings, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Settings,
        r#"
            SELECT
                id, model, created_at, updated_at, theme, appearance, update_channel
            FROM settings
            WHERE id = 'default'
        "#,
    )
    .fetch_one(&db)
    .await
}

pub async fn get_or_create_settings(app_handle: &AppHandle) -> Settings {
    if let Ok(settings) = get_settings(app_handle).await {
        return settings;
    }

    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
                INSERT INTO settings (id)
                VALUES ('default')
            "#,
    )
    .execute(&db)
    .await
    .expect("Failed to insert settings");

    get_settings(&app_handle)
        .await
        .expect("Failed to get settings")
}

pub async fn update_settings(
    app_handle: &AppHandle,
    settings: Settings,
) -> Result<Settings, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            UPDATE settings SET (
                theme, appearance, update_channel
            ) = (?, ?, ?) WHERE id = 'default';
        "#,
        settings.theme,
        settings.appearance,
        settings.update_channel
    )
    .execute(&db)
    .await?;

    match get_settings(app_handle).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn upsert_environment(
    app_handle: &AppHandle,
    environment: Environment,
) -> Result<Environment, sqlx::Error> {
    let id = match environment.id.as_str() {
        "" => generate_id(Some("ev")),
        _ => environment.id.to_string(),
    };
    let trimmed_name = environment.name.trim();
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            INSERT INTO environments (
                id, workspace_id, name, variables
            )
            VALUES (?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               name = excluded.name,
               variables = excluded.variables
        "#,
        id,
        environment.workspace_id,
        trimmed_name,
        environment.variables,
    )
    .execute(&db)
    .await?;

    match get_environment(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn get_environment(app_handle: &AppHandle, id: &str) -> Result<Environment, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Environment,
        r#"
            SELECT
                id, model, workspace_id, created_at, updated_at, name,
                variables AS "variables!: sqlx::types::Json<Vec<EnvironmentVariable>>"
            FROM environments
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn get_folder(app_handle: &AppHandle, id: &str) -> Result<Folder, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Folder,
        r#"
            SELECT
                id, model, workspace_id, created_at, updated_at, folder_id, name, sort_priority
            FROM folders
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_folders(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<Folder>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        Folder,
        r#"
            SELECT
                id, model, workspace_id, created_at, updated_at, folder_id, name, sort_priority
            FROM folders
            WHERE workspace_id = ?
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn delete_folder(app_handle: &AppHandle, id: &str) -> Result<Folder, sqlx::Error> {
    let folder = get_folder(app_handle, id).await?;
    let db = get_db(app_handle).await;
    let _ = sqlx::query!(
        r#"
            DELETE FROM folders
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, folder)
}

pub async fn upsert_folder(app_handle: &AppHandle, r: Folder) -> Result<Folder, sqlx::Error> {
    let id = match r.id.as_str() {
        "" => generate_id(Some("fl")),
        _ => r.id.to_string(),
    };
    let trimmed_name = r.name.trim();

    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            INSERT INTO folders (
                id, workspace_id, folder_id, name, sort_priority
            )
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               name = excluded.name,
               folder_id = excluded.folder_id,
               sort_priority = excluded.sort_priority
        "#,
        id,
        r.workspace_id,
        r.folder_id,
        trimmed_name,
        r.sort_priority,
    )
    .execute(&db)
    .await?;

    match get_folder(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn duplicate_http_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<HttpRequest, sqlx::Error> {
    let mut request = get_http_request(app_handle, id).await?.clone();
    request.id = "".to_string();
    upsert_http_request(app_handle, request).await
}

pub async fn upsert_http_request(
    app_handle: &AppHandle,
    r: HttpRequest,
) -> Result<HttpRequest, sqlx::Error> {
    let id = match r.id.as_str() {
        "" => generate_id(Some("rq")),
        _ => r.id.to_string(),
    };
    let headers_json = Json(r.headers);
    let auth_json = Json(r.authentication);
    let trimmed_name = r.name.trim();

    let db = get_db(app_handle).await;

    sqlx::query!(
        r#"
            INSERT INTO http_requests (
                id, workspace_id, folder_id, name, url, url_parameters, method, body, body_type,
                authentication, authentication_type, headers, sort_priority
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               name = excluded.name,
               folder_id = excluded.folder_id,
               method = excluded.method,
               headers = excluded.headers,
               body = excluded.body,
               body_type = excluded.body_type,
               authentication = excluded.authentication,
               authentication_type = excluded.authentication_type,
               url = excluded.url,
               url_parameters = excluded.url_parameters,
               sort_priority = excluded.sort_priority
        "#,
        id,
        r.workspace_id,
        r.folder_id,
        trimmed_name,
        r.url,
        r.url_parameters,
        r.method,
        r.body,
        r.body_type,
        auth_json,
        r.authentication_type,
        headers_json,
        r.sort_priority,
    )
    .execute(&db)
    .await?;

    match get_http_request(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn list_requests(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<HttpRequest>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        HttpRequest,
        r#"
            SELECT
                id, model, workspace_id, folder_id, created_at, updated_at, name, url,
                url_parameters AS "url_parameters!: sqlx::types::Json<Vec<HttpUrlParameter>>",
                method, body_type, authentication_type, sort_priority,
                body AS "body!: Json<HashMap<String, JsonValue>>",
                authentication AS "authentication!: Json<HashMap<String, JsonValue>>",
                headers AS "headers!: sqlx::types::Json<Vec<HttpRequestHeader>>"
            FROM http_requests
            WHERE workspace_id = ?
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn get_http_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<HttpRequest, sqlx::Error> {
    let db = get_db(app_handle).await;

    sqlx::query_as!(
        HttpRequest,
        r#"
            SELECT
                id, model, workspace_id, folder_id, created_at, updated_at, name, url, method,
                body_type, authentication_type, sort_priority,
                url_parameters AS "url_parameters!: sqlx::types::Json<Vec<HttpUrlParameter>>",
                body AS "body!: Json<HashMap<String, JsonValue>>",
                authentication AS "authentication!: Json<HashMap<String, JsonValue>>",
                headers AS "headers!: sqlx::types::Json<Vec<HttpRequestHeader>>"
            FROM http_requests
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn delete_http_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<HttpRequest, sqlx::Error> {
    let req = get_http_request(app_handle, id).await?;

    // DB deletes will cascade but this will delete the files
    delete_all_http_responses(app_handle, id).await?;

    let db = get_db(app_handle).await;
    let _ = sqlx::query!(
        r#"
            DELETE FROM http_requests
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, req)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_response(
    app_handle: &AppHandle,
    request_id: &str,
    elapsed: i64,
    elapsed_headers: i64,
    url: &str,
    status: i64,
    status_reason: Option<&str>,
    content_length: Option<i64>,
    body_path: Option<&str>,
    headers: Vec<HttpResponseHeader>,
    version: Option<&str>,
    remote_addr: Option<&str>,
) -> Result<HttpResponse, sqlx::Error> {
    let req = get_http_request(app_handle, request_id).await?;
    let id = generate_id(Some("rp"));
    let headers_json = Json(headers);
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            INSERT INTO http_responses (
                id, request_id, workspace_id, elapsed, elapsed_headers, url, status, status_reason,
                content_length, body_path, headers, version, remote_addr
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);
        "#,
        id,
        request_id,
        req.workspace_id,
        elapsed,
        elapsed_headers,
        url,
        status,
        status_reason,
        content_length,
        body_path,
        headers_json,
        version,
        remote_addr,
    )
    .execute(&db)
    .await?;

    get_http_response(app_handle, &id).await
}

pub async fn cancel_pending_grpc_connections(app_handle: &AppHandle) -> Result<(), sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            UPDATE grpc_connections
            SET (elapsed) = (-1)
            WHERE elapsed = 0;
        "#,
    )
    .execute(&db)
    .await?;
    Ok(())
}

pub async fn cancel_pending_responses(app_handle: &AppHandle) -> Result<(), sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            UPDATE http_responses
            SET (elapsed, status_reason) = (-1, 'Cancelled')
            WHERE elapsed = 0;
        "#,
    )
    .execute(&db)
    .await?;
    Ok(())
}

pub async fn update_response_if_id(
    app_handle: &AppHandle,
    response: &HttpResponse,
) -> Result<HttpResponse, sqlx::Error> {
    if response.id.is_empty() {
        Ok(response.clone())
    } else {
        update_response(app_handle, response).await
    }
}

pub async fn upsert_workspace(
    app_handle: &AppHandle,
    workspace: Workspace,
) -> Result<Workspace, sqlx::Error> {
    let id = match workspace.id.as_str() {
        "" => generate_id(Some("wk")),
        _ => workspace.id.to_string(),
    };
    let trimmed_name = workspace.name.trim();

    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            INSERT INTO workspaces (
                id, name, description, variables, setting_request_timeout,
                setting_follow_redirects, setting_validate_certificates
             )
            VALUES (?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT (id) DO UPDATE SET
               updated_at = CURRENT_TIMESTAMP,
               name = excluded.name,
               description = excluded.description,
               variables = excluded.variables,
               setting_request_timeout = excluded.setting_request_timeout,
               setting_follow_redirects = excluded.setting_follow_redirects,
               setting_validate_certificates = excluded.setting_validate_certificates
        "#,
        id,
        trimmed_name,
        workspace.description,
        workspace.variables,
        workspace.setting_request_timeout,
        workspace.setting_follow_redirects,
        workspace.setting_validate_certificates,
    )
    .execute(&db)
    .await?;

    match get_workspace(app_handle, &id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn update_response(
    app_handle: &AppHandle,
    response: &HttpResponse,
) -> Result<HttpResponse, sqlx::Error> {
    let headers_json = Json(&response.headers);
    let db = get_db(app_handle).await;
    sqlx::query!(
        r#"
            UPDATE http_responses SET (
                elapsed, elapsed_headers, url, status, status_reason, content_length, body_path,
                error, headers, version, remote_addr, updated_at
            ) = (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURRENT_TIMESTAMP) WHERE id = ?;
        "#,
        response.elapsed,
        response.elapsed_headers,
        response.url,
        response.status,
        response.status_reason,
        response.content_length,
        response.body_path,
        response.error,
        headers_json,
        response.version,
        response.remote_addr,
        response.id,
    )
    .execute(&db)
    .await?;

    match get_http_response(app_handle, &response.id).await {
        Ok(m) => Ok(emit_upserted_model(app_handle, m)),
        Err(e) => Err(e),
    }
}

pub async fn get_http_response(
    app_handle: &AppHandle,
    id: &str,
) -> Result<HttpResponse, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        HttpResponse,
        r#"
            SELECT
                id, model, workspace_id, request_id, updated_at, created_at, url, status,
                status_reason, content_length, body_path, elapsed, elapsed_headers, error,
                version, remote_addr,
                headers AS "headers!: sqlx::types::Json<Vec<HttpResponseHeader>>"
            FROM http_responses
            WHERE id = ?
        "#,
        id,
    )
    .fetch_one(&db)
    .await
}

pub async fn list_responses(
    app_handle: &AppHandle,
    request_id: &str,
    limit: Option<i64>,
) -> Result<Vec<HttpResponse>, sqlx::Error> {
    let limit_unwrapped = limit.unwrap_or_else(|| i64::MAX);
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        HttpResponse,
        r#"
            SELECT
                id, model, workspace_id, request_id, updated_at, created_at, url, status,
                status_reason, content_length, body_path, elapsed, elapsed_headers, error,
                version, remote_addr,
                headers AS "headers!: sqlx::types::Json<Vec<HttpResponseHeader>>"
            FROM http_responses
            WHERE request_id = ?
            ORDER BY created_at DESC
            LIMIT ?
        "#,
        request_id,
        limit_unwrapped,
    )
    .fetch_all(&db)
    .await
}

pub async fn list_responses_by_workspace_id(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> Result<Vec<HttpResponse>, sqlx::Error> {
    let db = get_db(app_handle).await;
    sqlx::query_as!(
        HttpResponse,
        r#"
            SELECT
                id, model, workspace_id, request_id, updated_at, created_at, url, status,
                status_reason, content_length, body_path, elapsed, elapsed_headers, error,
                version, remote_addr,
                headers AS "headers!: sqlx::types::Json<Vec<HttpResponseHeader>>"
            FROM http_responses
            WHERE workspace_id = ?
            ORDER BY created_at DESC
        "#,
        workspace_id,
    )
    .fetch_all(&db)
    .await
}

pub async fn delete_grpc_request(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcRequest, sqlx::Error> {
    let req = get_grpc_request(app_handle, id).await?;

    let db = get_db(app_handle).await;
    let _ = sqlx::query!(
        r#"
            DELETE FROM grpc_requests
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, req)
}

pub async fn delete_grpc_connection(
    app_handle: &AppHandle,
    id: &str,
) -> Result<GrpcConnection, sqlx::Error> {
    let resp = get_grpc_connection(app_handle, id).await?;

    let db = get_db(app_handle).await;
    let _ = sqlx::query!(
        r#"
            DELETE FROM grpc_connections
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, resp)
}

pub async fn delete_http_response(
    app_handle: &AppHandle,
    id: &str,
) -> Result<HttpResponse, sqlx::Error> {
    let resp = get_http_response(app_handle, id).await?;

    // Delete the body file if it exists
    if let Some(p) = resp.body_path.clone() {
        if let Err(e) = fs::remove_file(p) {
            error!("Failed to delete body file: {}", e);
        };
    }

    let db = get_db(app_handle).await;
    let _ = sqlx::query!(
        r#"
            DELETE FROM http_responses
            WHERE id = ?
        "#,
        id,
    )
    .execute(&db)
    .await;

    emit_deleted_model(app_handle, resp)
}

pub async fn delete_all_grpc_connections(
    app_handle: &AppHandle,
    request_id: &str,
) -> Result<(), sqlx::Error> {
    for r in list_grpc_connections(app_handle, request_id).await? {
        delete_grpc_connection(app_handle, &r.id).await?;
    }
    Ok(())
}

pub async fn delete_all_http_responses(
    app_handle: &AppHandle,
    request_id: &str,
) -> Result<(), sqlx::Error> {
    for r in list_responses(app_handle, request_id, None).await? {
        delete_http_response(app_handle, &r.id).await?;
    }
    Ok(())
}

pub fn generate_id(prefix: Option<&str>) -> String {
    let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
    match prefix {
        None => id,
        Some(p) => format!("{p}_{id}"),
    }
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WorkspaceExport {
    yaak_version: String,
    yaak_schema: i64,
    timestamp: NaiveDateTime,
    resources: WorkspaceExportResources,
}

#[derive(Default, Debug, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct WorkspaceExportResources {
    workspaces: Vec<Workspace>,
    environments: Vec<Environment>,
    folders: Vec<Folder>,
    requests: Vec<HttpRequest>,
}

pub async fn get_workspace_export_resources(
    app_handle: &AppHandle,
    workspace_id: &str,
) -> WorkspaceExport {
    let workspace = get_workspace(app_handle, workspace_id)
        .await
        .expect("Failed to get workspace");
    return WorkspaceExport {
        yaak_version: app_handle.package_info().version.clone().to_string(),
        yaak_schema: 1,
        timestamp: chrono::Utc::now().naive_utc(),
        resources: WorkspaceExportResources {
            workspaces: vec![workspace],
            environments: list_environments(app_handle, workspace_id)
                .await
                .expect("Failed to get environments"),
            folders: list_folders(app_handle, workspace_id)
                .await
                .expect("Failed to get folders"),
            requests: list_requests(app_handle, workspace_id)
                .await
                .expect("Failed to get requests"),
        },
    };
}

fn emit_upserted_model<S: Serialize + Clone>(app_handle: &AppHandle, model: S) -> S {
    app_handle
        .emit_all("upserted_model", model.clone())
        .unwrap();
    model
}

fn emit_deleted_model<S: Serialize + Clone, E>(app_handle: &AppHandle, model: S) -> Result<S, E> {
    app_handle.emit_all("deleted_model", model.clone()).unwrap();
    Ok(model)
}

async fn get_db(app_handle: &AppHandle) -> Pool<Sqlite> {
    let db_state = app_handle.state::<Mutex<Pool<Sqlite>>>();
    let db = &*db_state.lock().await;
    db.clone()
}
