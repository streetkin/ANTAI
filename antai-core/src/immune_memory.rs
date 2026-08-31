// ANTAI Core — Immune Memory & Hackademy Loop (Sistema Immunitario Digitale)
// Registro dinamico degli anticorpi (firme di attacco apprese).
// Include protezione anti-avvelenamento (Write-Time Memory Sanitization).

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Antibody {
    pub id: String,
    pub pattern: String,
    pub description: String,
    pub severity: String,
    pub created_at: String,
    pub times_triggered: u32,
    pub is_sanitized: bool,
}

#[derive(Debug, Clone)]
pub struct ImmuneMemory {
    pub antibodies: Vec<Antibody>,
    storage_path: String,
}

impl ImmuneMemory {
    pub fn new() -> Self {
        let storage_path = "antibodies.json".to_string();
        let mut memory = ImmuneMemory {
            antibodies: Vec::new(),
            storage_path,
        };

        memory.load();
        memory
    }

    /// Carica gli anticorpi memorizzati su disco
    fn load(&mut self) {
        if Path::new(&self.storage_path).exists() {
            if let Ok(content) = fs::read_to_string(&self.storage_path) {
                if let Ok(loaded) = serde_json::from_str::<Vec<Antibody>>(&content) {
                    self.antibodies = loaded;
                    return;
                }
            }
        }

        // Anticorpi di base (Seed Immune Memory)
        self.antibodies = vec![
            Antibody {
                id: "AB-001".to_string(),
                pattern: "system override recursive prompt".to_string(),
                description: "Recognized recursive instruction hijacking attempt".to_string(),
                severity: "HIGH".to_string(),
                created_at: "2026-08-01 02:00:00".to_string(),
                times_triggered: 1,
                is_sanitized: true,
            },
            Antibody {
                id: "AB-002".to_string(),
                pattern: "decoy_exec_cmd".to_string(),
                description: "Attempted execution of MCP Decoy Tool execute_system_command".to_string(),
                severity: "CRITICAL".to_string(),
                created_at: "2026-08-01 02:05:00".to_string(),
                times_triggered: 3,
                is_sanitized: true,
            },
        ];

        let _ = self.save();
    }

    /// Salva gli anticorpi su disco
    pub fn save(&self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.antibodies).map_err(|e| e.to_string())?;
        fs::write(&self.storage_path, json).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Genera e registra un nuovo anticorpo dopo la l'estrazione dell'impronta concettuale e la validazione anti-avvelenamento.
    pub fn register_antibody(&mut self, raw_pattern: &str, attack_type: &str) -> Option<Antibody> {
        let clean = raw_pattern.trim().to_lowercase();
        
        // Protezione anti-avvelenamento: rifiuta stringhe troppo brevi o parole comuni generiche
        if clean.len() < 5 || clean.contains("<script>") {
            return None;
        }

        // parole comuni generiche vietate come singoli anticorpi per evitare falsi positivi
        let forbidden_generic = ["hello", "test", "prompt", "user", "admin", "system", "please", "query"];
        if forbidden_generic.contains(&clean.as_str()) {
            return None;
        }

        // Estrazione Impronta Digitale Concettuale (Fingerprint Signature)
        let fingerprint = Self::extract_concept_fingerprint(&clean);

        // Evita duplicati sull'impronta concettuale
        if let Some(pos) = self.antibodies.iter().position(|a| a.pattern == fingerprint) {
            self.antibodies[pos].times_triggered += 1;
            let _ = self.save();
            return Some(self.antibodies[pos].clone());
        }

        let new_ab = Antibody {
            id: format!("AB-{:03}", self.antibodies.len() + 1),
            pattern: fingerprint.clone(),
            description: format!("Concept antibody fingerprint for {} threat family", attack_type),
            severity: "CRITICAL".to_string(),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            times_triggered: 1,
            is_sanitized: true,
        };

        self.antibodies.push(new_ab.clone());
        let _ = self.save();
        Some(new_ab)
    }

    /// Estrattore di impronta concettuale (normalizza il payload in pattern euristico).
    fn extract_concept_fingerprint(clean: &str) -> String {
        let known_triggers = [
            "ignore previous instructions",
            "disregard prior instructions",
            "execute_system_command",
            "169.254.169.254",
            "dump system prompt",
            "select * from",
            "rm -rf",
            "os.system",
            "subprocess",
            "base64_decode",
        ];

        for trigger in known_triggers {
            if clean.contains(trigger) {
                return trigger.to_string();
            }
        }

        // Se non trova un trigger noto, estrae le 3 parole chiave più distintive (>3 caratteri)
        let words: Vec<&str> = clean
            .split_whitespace()
            .filter(|w| w.len() > 3 && !["this", "that", "with", "from", "your", "have"].contains(w))
            .take(3)
            .collect();

        if words.is_empty() {
            clean.chars().take(30).collect()
        } else {
            words.join(" ")
        }
    }

    /// Verifica se un payload corrisponde a uno degli anticorpi memorizzati
    pub fn match_antibody(&self, payload: &str) -> Option<Antibody> {
        let payload_lower = payload.to_lowercase();
        for ab in &self.antibodies {
            if payload_lower.contains(&ab.pattern) {
                return Some(ab.clone());
            }
        }
        None
    }
}
