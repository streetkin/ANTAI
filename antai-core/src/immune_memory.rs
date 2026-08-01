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

    /// Genera e registra un nuovo anticorpo dopo aver superato la validazione anti-avvelenamento.
    pub fn register_antibody(&mut self, raw_pattern: &str, attack_type: &str) -> Option<Antibody> {
        // Anti-Memory Poisoning: Se il pattern è troppo generico, corto o privo di senso, lo rifiuta per evitare avvelenamento
        if raw_pattern.trim().len() < 4 || raw_pattern.contains("<script>") {
            return None;
        }

        let clean_pattern = raw_pattern.trim().to_lowercase();

        // Evita duplicati
        if let Some(pos) = self.antibodies.iter().position(|a| a.pattern == clean_pattern) {
            self.antibodies[pos].times_triggered += 1;
            let _ = self.save();
            return Some(self.antibodies[pos].clone());
        }

        let new_ab = Antibody {
            id: format!("AB-{:03}", self.antibodies.len() + 1),
            pattern: clean_pattern,
            description: format!("Auto-generated antibody from captured {} attack", attack_type),
            severity: "CRITICAL".to_string(),
            created_at: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            times_triggered: 1,
            is_sanitized: true,
        };

        self.antibodies.push(new_ab.clone());
        let _ = self.save();
        Some(new_ab)
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
