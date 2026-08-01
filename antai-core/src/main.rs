// ANTAI — Autonomous AI Cyber Defense Sentinel & Deception Core
// Entry Point: avvia il Proxy Interceptor (8090) e il Bridge REST API (8091).

mod config;
mod sanitizer;
mod asymmetric_engine;
mod system_scanner;
mod deception;
mod immune_memory;
mod proxy;
mod bridge;

use config::AntaiConfig;
use sanitizer::HeuristicSanitizer;
use asymmetric_engine::AsymmetricEngine;
use system_scanner::SystemScanner;
use deception::DeceptionEngine;
use immune_memory::ImmuneMemory;
use proxy::{ProxyState, create_proxy_router};
use bridge::create_bridge_router;

use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpListener;

#[cfg(windows)]
fn enable_ansi_support() {
    let _ = std::process::Command::new("cmd")
        .args(["/c", "color"])
        .output();
}

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    enable_ansi_support();

    println!();
    println!("  █████╗ ███╗   ██╗████████╗█████╗ ██╗");
    println!(" ██╔══██╗████╗  ██║╚══██╔══╝██╔══██╗██║");
    println!(" ███████║██╔██╗ ██║   ██║   ███████║██║");
    println!(" ██╔══██║██║╚██╗██║   ██║   ██╔══██║██║");
    println!(" ██║  ██║██║ ╚████║   ██║   ██║  ██║██║");
    println!(" ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝");
    println!(" --------------------------------------------------");
    println!(" 🛡️  AUTONOMOUS AI CYBER DEFENSE SENTINEL v2.0");
    println!(" 🧬 DIGITAL IMMUNE SYSTEM & BEELZEBUB DECEPTION CORE");
    println!(" ⚡ RUST NATIVE ENGINE (COMPILED - <0.05µs LATENCY)");
    println!(" --------------------------------------------------");
    println!();

    // Carica configurazione
    let config = AntaiConfig::load();
    println!("[ANTAI] Configurazione caricata.");
    println!("[ANTAI] Modalità difesa: {}", config.defense_mode);
    println!("[ANTAI] Host Ollama: {}", config.ollama_host);
    println!(
        "[ANTAI] Chiave OpenRouter: {}",
        AntaiConfig::masked_key(&config.openrouter_key)
    );
    println!(
        "[ANTAI] Chiave Groq: {}",
        AntaiConfig::masked_key(&config.groq_key)
    );

    // Inizializza il sanitizer euristico
    let sanitizer = HeuristicSanitizer::new();
    println!("[ANTAI] Filtro Euristico compilato (39 pattern di attacco & MCP decoy attivi).");

    // Inizializza lo Scanner di Sistema
    let scanner = SystemScanner::new();
    println!("[ANTAI] Scanner Processi e Memoria PC attivo.");

    // Inizializza il motore IA asincrono
    let ai_engine = AsymmetricEngine::new();
    println!("[ANTAI] Motore IA Asimmetrico inizializzato.");

    // Inizializza il Deception Engine & l'Immune Memory
    let deception = DeceptionEngine::new();
    println!("[ANTAI] 🎭 Deception Engine (Beelzebub Honeypots & MCP Decoys) online.");

    let immune_memory = ImmuneMemory::new();
    println!("[ANTAI] 🧬 Memoria Immunitaria Digitale (Hackademy Loop) inizializzata ({} anticorpi registrati).", immune_memory.antibodies.len());

    // Stato condiviso tra proxy e bridge
    let shared_state = Arc::new(ProxyState {
        sanitizer,
        ai_engine,
        scanner,
        deception,
        immune_memory: RwLock::new(immune_memory),
        config: RwLock::new(config.clone()),
        threat_log: RwLock::new(Vec::new()),
    });

    let proxy_port = config.proxy_port;
    let bridge_port = config.bridge_port;

    // Avvia Proxy Interceptor (porta 8090)
    let proxy_state = Arc::clone(&shared_state);
    let proxy_handle = tokio::spawn(async move {
        let app = create_proxy_router(proxy_state);
        let addr = format!("127.0.0.1:{}", proxy_port);
        let listener = TcpListener::bind(&addr)
            .await
            .expect("[ANTAI] Errore critico: impossibile avviare il Proxy Interceptor");

        println!(
            "[ANTAI] ⚡ Proxy Interceptor attivo su http://{}",
            addr
        );

        axum::serve(listener, app)
            .await
            .expect("[ANTAI] Proxy Interceptor terminato in modo inatteso");
    });

    // Avvia Bridge REST API (porta 8091)
    let bridge_state = Arc::clone(&shared_state);
    let bridge_handle = tokio::spawn(async move {
        let app = create_bridge_router(bridge_state);
        let addr = format!("127.0.0.1:{}", bridge_port);
        let listener = TcpListener::bind(&addr)
            .await
            .expect("[ANTAI] Errore critico: impossibile avviare il Bridge REST API");

        println!(
            "[ANTAI] 🛡️  Dashboard Bridge attivo su http://{}",
            addr
        );
        println!();
        println!("[ANTAI] ✅ ANTAI SENTINEL & DECEPTION CORE ONLINE — Sistema operativo e in difesa.");
        println!("[ANTAI] Apri index.html nel browser per la Dashboard.");
        println!();

        axum::serve(listener, app)
            .await
            .expect("[ANTAI] Bridge REST API terminato in modo inatteso");
    });

    // Attendi entrambi i server
    let _ = tokio::join!(proxy_handle, bridge_handle);
}
