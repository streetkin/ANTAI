// ANTAI Core — Secure Configuration Manager
// Gestione sicura delle chiavi API e della configurazione locale.
// Le chiavi non vengono MAI esposte nei log — solo hash parziali per conferma.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Configurazione principale di ANTAI salvata in locale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntaiConfig {
    pub ollama_host: String,
    pub ollama_model: String,
    pub openrouter_key: Option<String>,
    pub groq_key: Option<String>,
    pub defense_mode: String, // "asymmetric" | "strict"
    pub proxy_port: u16,
    pub bridge_port: u16,
}

impl Default for AntaiConfig {
    fn default() -> Self {
        Self {
            ollama_host: "http://127.0.0.1:11434".to_string(),
            ollama_model: "qwen2.5:0.5b".to_string(),
            openrouter_key: None,
            groq_key: None,
            defense_mode: "asymmetric".to_string(),
            proxy_port: 8090,
            bridge_port: 8091,
        }
    }
}

impl AntaiConfig {
    /// Percorso del file di configurazione nella home dell'utente.
    fn config_path() -> PathBuf {
        let home = dirs_fallback();
        home.join(".antai").join("config.json")
    }

    /// Carica la configurazione dal disco. Se non esiste, crea quella di default.
    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    serde_json::from_str(&content).unwrap_or_else(|_| {
                        eprintln!("[ANTAI Config] File di configurazione corrotto, uso default.");
                        Self::default()
                    })
                }
                Err(_) => Self::default(),
            }
        } else {
            let cfg = Self::default();
            cfg.save();
            cfg
        }
    }

    /// Salva la configurazione su disco in modo sicuro.
    pub fn save(&self) {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(&path, json);
        }
    }

    /// Aggiorna la chiave OpenRouter.
    pub fn set_openrouter_key(&mut self, key: String) {
        if key.trim().is_empty() {
            self.openrouter_key = None;
        } else {
            self.openrouter_key = Some(key);
        }
        self.save();
    }

    /// Aggiorna la chiave Groq.
    pub fn set_groq_key(&mut self, key: String) {
        if key.trim().is_empty() {
            self.groq_key = None;
        } else {
            self.groq_key = Some(key);
        }
        self.save();
    }

    /// Restituisce una versione mascherata della chiave (per log/UI sicuri).
    pub fn masked_key(key: &Option<String>) -> String {
        match key {
            Some(k) if k.len() > 8 => {
                let prefix = &k[..4];
                let suffix = &k[k.len() - 4..];
                format!("{}****{}", prefix, suffix)
            }
            Some(_) => "****".to_string(),
            None => "Non configurata".to_string(),
        }
    }
}

/// Fallback per ottenere la home directory senza dipendenze esterne.
fn dirs_fallback() -> PathBuf {
    if let Ok(home) = std::env::var("USERPROFILE") {
        PathBuf::from(home)
    } else if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home)
    } else {
        PathBuf::from(".")
    }
}
