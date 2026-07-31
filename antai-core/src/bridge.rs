// ANTAI Core — Dashboard Bridge REST API (porta 8091)
// Espone gli endpoint REST per collegare la Dashboard UI (index.html/app.js)
// al motore Rust in tempo reale. CORS abilitato SOLO per localhost.

use axum::{
    extract::State,
    http::{header, Method},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::config::AntaiConfig;
use crate::proxy::ProxyState;

/// Risposta dello stato dello shield.
#[derive(Serialize)]
pub struct StatusResponse {
    pub shield_active: bool,
    pub defense_mode: String,
    pub active_engine: String,
    pub threats_blocked: usize,
    pub proxy_port: u16,
    pub bridge_port: u16,
    pub ollama_host: String,
    pub openrouter_key: String, // mascherata
    pub groq_key: String,       // mascherata
}

/// Richiesta di test per il sanitizer.
#[derive(Deserialize)]
pub struct ScanRequest {
    pub payload: String,
}

/// Risposta del test di scansione.
#[derive(Serialize)]
pub struct ScanResponse {
    pub blocked: bool,
    pub reason: Option<String>,
    pub engine: String,
    pub latency: String,
}

/// Richiesta di salvataggio chiavi API.
#[derive(Deserialize)]
pub struct KeysRequest {
    #[serde(default)]
    pub openrouter_key: Option<String>,
    #[serde(default)]
    pub groq_key: Option<String>,
}

/// Risposta al salvataggio chiavi.
#[derive(Serialize)]
pub struct KeysResponse {
    pub saved: bool,
    pub message: String,
}

/// Crea il router del bridge REST (porta 8091).
pub fn create_bridge_router(state: Arc<ProxyState>) -> Router {
    // CORS: permetti solo localhost e file:// per sicurezza
    let cors = CorsLayer::new()
        .allow_origin(Any) // necessario per file:// protocol
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/api/status", get(handle_status))
        .route("/api/threats", get(handle_threats))
        .route("/api/scan", post(handle_scan))
        .route("/api/keys", post(handle_keys))
        .route("/api/config", get(handle_config))
        .route("/api/system/scan", get(handle_system_scan).post(handle_system_scan))
        .route("/api/system/processes", get(handle_system_processes))
        .layer(cors)
        .with_state(state)
}

/// GET /api/status — Stato dello shield.
async fn handle_status(State(state): State<Arc<ProxyState>>) -> Json<StatusResponse> {
    let config = state.config.read().await;
    let threats = state.threat_log.read().await;

    Json(StatusResponse {
        shield_active: true,
        defense_mode: config.defense_mode.clone(),
        active_engine: determine_active_engine(&config),
        threats_blocked: threats.len(),
        proxy_port: config.proxy_port,
        bridge_port: config.bridge_port,
        ollama_host: config.ollama_host.clone(),
        openrouter_key: AntaiConfig::masked_key(&config.openrouter_key),
        groq_key: AntaiConfig::masked_key(&config.groq_key),
    })
}

/// GET /api/threats — Lista delle minacce bloccate.
async fn handle_threats(
    State(state): State<Arc<ProxyState>>,
) -> Json<Vec<crate::proxy::ThreatEntry>> {
    let threats = state.threat_log.read().await;
    // Restituisce le ultime 50 minacce (ordine cronologico inverso)
    let recent: Vec<_> = threats.iter().rev().take(50).cloned().collect();
    Json(recent)
}

/// POST /api/scan — Test diretto del sanitizer su un payload.
async fn handle_scan(
    State(state): State<Arc<ProxyState>>,
    Json(req): Json<ScanRequest>,
) -> Json<ScanResponse> {
    let result = state.sanitizer.inspect(&req.payload);

    Json(ScanResponse {
        blocked: result.blocked,
        reason: result.reason,
        engine: result.engine,
        latency: format!("{:.2}µs", result.latency_us),
    })
}

/// POST /api/keys — Salva chiavi API in modo sicuro.
async fn handle_keys(
    State(state): State<Arc<ProxyState>>,
    Json(req): Json<KeysRequest>,
) -> Json<KeysResponse> {
    let mut config = state.config.write().await;

    if let Some(key) = req.openrouter_key {
        config.set_openrouter_key(key);
    }
    if let Some(key) = req.groq_key {
        config.set_groq_key(key);
    }

    Json(KeysResponse {
        saved: true,
        message: "Chiavi API salvate in modo sicuro.".to_string(),
    })
}

/// GET /api/config — Restituisce la configurazione (chiavi mascherate).
async fn handle_config(State(state): State<Arc<ProxyState>>) -> Json<serde_json::Value> {
    let config = state.config.read().await;

    Json(serde_json::json!({
        "ollama_host": config.ollama_host,
        "ollama_model": config.ollama_model,
        "openrouter_key": AntaiConfig::masked_key(&config.openrouter_key),
        "groq_key": AntaiConfig::masked_key(&config.groq_key),
        "defense_mode": config.defense_mode,
        "proxy_port": config.proxy_port,
        "bridge_port": config.bridge_port,
    }))
}

/// GET/POST /api/system/scan — Scansione del sistema (RAM, CPU, Processi sospetti).
async fn handle_system_scan(
    State(state): State<Arc<ProxyState>>,
) -> Json<crate::system_scanner::SystemScanReport> {
    let report = state.scanner.scan();
    Json(report)
}

/// GET /api/system/processes — Elenco dei top processi di sistema per RAM.
async fn handle_system_processes(
    State(state): State<Arc<ProxyState>>,
) -> Json<Vec<crate::system_scanner::ProcessInfo>> {
    let list = state.scanner.get_top_processes(30);
    Json(list)
}

/// Determina il motore IA attivo in base alla configurazione.
fn determine_active_engine(config: &AntaiConfig) -> String {
    if config.openrouter_key.is_some() {
        "Asymmetric (Ollama + OpenRouter)".to_string()
    } else if config.groq_key.is_some() {
        "Asymmetric (Ollama + Groq Free)".to_string()
    } else {
        "Ollama Locale (0$)".to_string()
    }
}
