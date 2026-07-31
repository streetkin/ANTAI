// ANTAI Core — HTTP Interceptor Proxy (porta 8090)
// Server proxy ultraleggero che intercetta il traffico e lo valuta
// attraverso la pipeline: Sanitizer Euristico → Engine IA Asimmetrico.

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AntaiConfig;
use crate::sanitizer::HeuristicSanitizer;
use crate::asymmetric_engine::AsymmetricEngine;

use crate::system_scanner::SystemScanner;

/// Stato condiviso del proxy.
pub struct ProxyState {
    pub sanitizer: HeuristicSanitizer,
    pub ai_engine: AsymmetricEngine,
    pub scanner: SystemScanner,
    pub config: RwLock<AntaiConfig>,
    pub threat_log: RwLock<Vec<ThreatEntry>>,
}

/// Voce nel registro minacce.
#[derive(Debug, Clone, Serialize)]
pub struct ThreatEntry {
    pub timestamp: String,
    pub payload_preview: String,
    pub attack_type: String,
    pub engine_used: String,
    pub latency: String,
    pub status: String,
}

/// Richiesta di intercettazione in ingresso.
#[derive(Deserialize)]
pub struct InterceptRequest {
    pub payload: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub source_ip: Option<String>,
}

/// Risposta dell'intercettore.
#[derive(Serialize)]
pub struct InterceptResponse {
    pub status: String,         // "clean" | "blocked"
    pub reason: Option<String>,
    pub engine: String,
    pub latency: String,
}

use tower_http::cors::{Any, CorsLayer};
use axum::http::{header, Method};

/// Crea il router del proxy interceptor (porta 8090).
pub fn create_proxy_router(state: Arc<ProxyState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/intercept", post(handle_intercept))
        .layer(cors)
        .with_state(state)
}

/// Handler principale per l'intercettazione dei payload.
async fn handle_intercept(
    State(state): State<Arc<ProxyState>>,
    Json(req): Json<InterceptRequest>,
) -> (StatusCode, Json<InterceptResponse>) {
    // FASE 1: Filtro Euristico (microsecondi, 0$)
    let heuristic_result = state.sanitizer.inspect(&req.payload);

    if heuristic_result.blocked {
        // Registra la minaccia nel log
        let entry = ThreatEntry {
            timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
            payload_preview: truncate_for_log(&req.payload, 80),
            attack_type: heuristic_result
                .matched_pattern
                .clone()
                .unwrap_or_default(),
            engine_used: heuristic_result.engine.clone(),
            latency: format!("{:.2}µs", heuristic_result.latency_us),
            status: "BLOCKED".to_string(),
        };

        state.threat_log.write().await.push(entry);

        return (
            StatusCode::FORBIDDEN,
            Json(InterceptResponse {
                status: "blocked".to_string(),
                reason: heuristic_result.reason,
                engine: heuristic_result.engine,
                latency: format!("{:.2}µs", heuristic_result.latency_us),
            }),
        );
    }

    // FASE 2: Valutazione IA Asimmetrica (se il filtro euristico non ha bloccato)
    let config = state.config.read().await.clone();
    let ai_result = state.ai_engine.evaluate(&req.payload, &config).await;

    if ai_result.blocked {
        let entry = ThreatEntry {
            timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
            payload_preview: truncate_for_log(&req.payload, 80),
            attack_type: "AI-Detected Threat".to_string(),
            engine_used: ai_result.engine.clone(),
            latency: format!("{:.1}ms", ai_result.latency_ms),
            status: "BLOCKED".to_string(),
        };

        state.threat_log.write().await.push(entry);

        return (
            StatusCode::FORBIDDEN,
            Json(InterceptResponse {
                status: "blocked".to_string(),
                reason: Some(format!("AI Assessment: {}", ai_result.ai_response)),
                engine: ai_result.engine,
                latency: format!("{:.1}ms", ai_result.latency_ms),
            }),
        );
    }

    // Payload pulito
    (
        StatusCode::OK,
        Json(InterceptResponse {
            status: "clean".to_string(),
            reason: None,
            engine: ai_result.engine,
            latency: format!("{:.1}ms", ai_result.latency_ms),
        }),
    )
}

/// Tronca il payload per il log (senza esporre dati sensibili).
fn truncate_for_log(payload: &str, max_len: usize) -> String {
    if payload.len() > max_len {
        format!("{}...", &payload[..max_len])
    } else {
        payload.to_string()
    }
}
