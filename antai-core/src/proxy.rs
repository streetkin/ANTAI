// ANTAI Core — HTTP Interceptor Proxy (porta 8090)
// Server proxy ultraleggero che intercetta il traffico e lo valuta
// attraverso la pipeline: Sanitizer Euristico → Immune Memory → Deception Engine.

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::AntaiConfig;
use crate::sanitizer::HeuristicSanitizer;
use crate::asymmetric_engine::AsymmetricEngine;
use crate::system_scanner::SystemScanner;
use crate::deception::DeceptionEngine;
use crate::immune_memory::ImmuneMemory;

/// Stato condiviso del proxy.
pub struct ProxyState {
    pub sanitizer: HeuristicSanitizer,
    pub ai_engine: AsymmetricEngine,
    pub scanner: SystemScanner,
    pub deception: RwLock<DeceptionEngine>,
    pub immune_memory: RwLock<ImmuneMemory>,
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
    pub status: String, // "BLOCKED" | "HONEYPOT_DIVERTED" | "IMMUNE_COUNTERATTACK"
    pub canary_token: Option<String>,
}

/// Richiesta di intercettazione in ingresso.
#[derive(Deserialize)]
pub struct InterceptRequest {
    pub payload: String,
    #[serde(default)]
    pub source_ip: Option<String>,
}

/// Risposta dell'intercettore.
#[derive(Serialize)]
pub struct InterceptResponse {
    pub status: String, // "clean" | "blocked" | "diverted" | "counterattacked"
    pub reason: Option<String>,
    pub engine: String,
    pub latency: String,
    pub simulated_payload: Option<String>,
    pub canary_token: Option<String>,
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
    let immune_mem = state.immune_memory.read().await;
    let config = state.config.read().await.clone();

    // FASE 1: Filtro Euristico & Memoria Immunitaria (microsecondi, 0$)
    let heuristic_result = state.sanitizer.inspect(&req.payload, &immune_mem);
    drop(immune_mem);

    if heuristic_result.blocked {
        let attack_type = heuristic_result.matched_pattern.clone().unwrap_or_else(|| "OWASP Threat".to_string());

        // Hackademy Loop: registra automaticamente un anticorpo nella memoria immunitaria
        let mut immune_mem_write = state.immune_memory.write().await;
        let _ = immune_mem_write.register_antibody(&req.payload, &attack_type);
        drop(immune_mem_write);

        let defense_mode = config.defense_mode.to_lowercase();

        match defense_mode.as_str() {
            "deception" | "beelzebub" => {
                // MODALITÀ DECEPTION: Risposta Honeypot Polimorfica (<0.1ms + jitter casuale)
                let deception = state.deception.read().await;
                let (honeypot_resp, jitter_ms) = deception.generate_polymorphic_honeypot(&req.payload).await;
                drop(deception);

                let entry = ThreatEntry {
                    timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
                    payload_preview: truncate_for_log(&req.payload, 80),
                    attack_type: attack_type.clone(),
                    engine_used: "ANTAI Beelzebub Deception Core".to_string(),
                    latency: format!("{:.2}µs + {}ms jitter", heuristic_result.latency_us, jitter_ms),
                    status: "HONEYPOT_DIVERTED".to_string(),
                    canary_token: None,
                };
                state.threat_log.write().await.push(entry);

                return (
                    StatusCode::OK,
                    Json(InterceptResponse {
                        status: "diverted".to_string(),
                        reason: Some("Diverted to Synthetic Beelzebub Honeypot Environment".to_string()),
                        engine: "ANTAI Beelzebub Deception Core".to_string(),
                        latency: format!("{:.2}µs", heuristic_result.latency_us),
                        simulated_payload: Some(honeypot_resp),
                        canary_token: None,
                    }),
                );
            }
            "immune_counter" | "counterattack" => {
                // MODALITÀ IMMUNE COUNTERATTACK: Canary Token + Payload di Contrattacco Cognitivo
                let deception = state.deception.read().await;
                let canary = deception.generate_canary_token(&attack_type);
                let poison_payload = deception.generate_cognitive_poison();
                drop(deception);

                let entry = ThreatEntry {
                    timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
                    payload_preview: truncate_for_log(&req.payload, 80),
                    attack_type: attack_type.clone(),
                    engine_used: "ANTAI Active Immune Counterattack".to_string(),
                    latency: format!("{:.2}µs", heuristic_result.latency_us),
                    status: "IMMUNE_COUNTERATTACK".to_string(),
                    canary_token: Some(canary.clone()),
                };
                state.threat_log.write().await.push(entry);

                return (
                    StatusCode::OK,
                    Json(InterceptResponse {
                        status: "counterattacked".to_string(),
                        reason: Some("Cognitive Paradox Poison Payload Injected into Attacking AI Agent".to_string()),
                        engine: "ANTAI Active Immune Counterattack".to_string(),
                        latency: format!("{:.2}µs", heuristic_result.latency_us),
                        simulated_payload: Some(poison_payload),
                        canary_token: Some(canary),
                    }),
                );
            }
            _ => {
                // MODALITÀ SHIELD (Standard 403 Forbidden)
                let entry = ThreatEntry {
                    timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
                    payload_preview: truncate_for_log(&req.payload, 80),
                    attack_type: attack_type.clone(),
                    engine_used: heuristic_result.engine.clone(),
                    latency: format!("{:.2}µs", heuristic_result.latency_us),
                    status: "BLOCKED".to_string(),
                    canary_token: None,
                };
                state.threat_log.write().await.push(entry);

                return (
                    StatusCode::FORBIDDEN,
                    Json(InterceptResponse {
                        status: "blocked".to_string(),
                        reason: heuristic_result.reason,
                        engine: heuristic_result.engine,
                        latency: format!("{:.2}µs", heuristic_result.latency_us),
                        simulated_payload: None,
                        canary_token: None,
                    }),
                );
            }
        }
    }

    // FASE 2: Valutazione IA Asimmetrica (se il filtro euristico non ha bloccato)
    let ai_result = state.ai_engine.evaluate(&req.payload, &config).await;

    if ai_result.blocked {
        let entry = ThreatEntry {
            timestamp: chrono::Utc::now().format("%H:%M:%S%.3f").to_string(),
            payload_preview: truncate_for_log(&req.payload, 80),
            attack_type: "AI-Detected Threat".to_string(),
            engine_used: ai_result.engine.clone(),
            latency: format!("{:.1}ms", ai_result.latency_ms),
            status: "BLOCKED".to_string(),
            canary_token: None,
        };

        state.threat_log.write().await.push(entry);

        return (
            StatusCode::FORBIDDEN,
            Json(InterceptResponse {
                status: "blocked".to_string(),
                reason: Some(format!("AI Assessment: {}", ai_result.ai_response)),
                engine: ai_result.engine,
                latency: format!("{:.1}ms", ai_result.latency_ms),
                simulated_payload: None,
                canary_token: None,
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
            simulated_payload: None,
            canary_token: None,
        }),
    )
}

/// Tronca il payload per il log.
fn truncate_for_log(payload: &str, max_len: usize) -> String {
    if payload.len() > max_len {
        format!("{}...", &payload[..max_len])
    } else {
        payload.to_string()
    }
}
