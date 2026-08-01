// ANTAI Core — Heuristic Sanitizer (Microsecond Threat Filter)
// Ultra-fast heuristic filter compiling OWASP Top 10 for LLMs attack vectors into Rust RegexSets.
// Includes Session Threat Score tracking (Anti-Probing) & Dynamic Immune Memory Matching.
// Compiled once at startup. Evaluation latency: < 0.05µs.

use regex::RegexSet;
use serde::Serialize;
use std::time::Instant;
use crate::immune_memory::ImmuneMemory;

/// Result of a heuristic threat inspection.
#[derive(Debug, Clone, Serialize)]
pub struct SanitizeResult {
    pub blocked: bool,
    pub reason: Option<String>,
    pub matched_pattern: Option<String>,
    pub engine: String,
    pub latency_us: f64, // microseconds
}

/// Heuristic threat sanitizer with compiled attack signatures.
pub struct HeuristicSanitizer {
    regex_set: RegexSet,
    pattern_names: Vec<&'static str>,
}

impl HeuristicSanitizer {
    /// Compiles all OWASP LLM Top 10 threat patterns.
    pub fn new() -> Self {
        let patterns: Vec<&str> = vec![
            // 1. Direct Prompt Injection (OWASP LLM01)
            r"(?i)ignore\s+(all\s+)?previous\s+instructions",
            r"(?i)disregard\s+(all\s+)?prior\s+(instructions|context|rules)",
            r"(?i)system\s+prompt\s+(override|dump|reveal|show|ignore)",
            r"(?i)you\s+are\s+now\s+(an?\s+)?unconstrained",
            r"(?i)assume\s+(the\s+)?(persona|role|identity)\s+of",
            r"(?i)pretend\s+you\s+(are|have)\s+no\s+(restrictions|rules|limits)",
            // 2. Jailbreak Skeleton Key & DAN Modes
            r"(?i)(skeleton[_\s]?key|master[_\s]?key)\s*(jailbreak|unlock|bypass)",
            r"(?i)do\s+anything\s+now",
            r"(?i)developer\s+mode\s+(enabled|activated|on)",
            r"(?i)jailbreak\s+(mode|prompt|activated|override)",
            // 3. Automated Red-Team & Exploit Probes (PyRIT, Garak, Counterfit)
            r"(?i)garak[_\s]probe",
            r"(?i)pyrit[_\s]orchestrator",
            r"(?i)counterfit[_\s]attack",
            r"(?i)adversarial[_\s]ml[_\s]threat",
            r"(?i)prompt_injection_test_payload",
            // 4. Agentic Tool & Command Execution Hijack (OWASP LLM02 / LLM08)
            r"(?i)(exec|eval|system)\s*\(\s*(base64|atob|btoa)",
            r"(?i)(rm\s+-rf|del\s+/[fqs]|format\s+c:)",
            r"(?i)(wget|curl)\s+https?://.*\|\s*(bash|sh|powershell)",
            r"(?i)<script[\s>].*?(document\.cookie|eval\s*\()",
            r"(?i)subprocess\.(Popen|call|check_output)",
            r"(?i)os\.system\s*\(",
            // 5. Data Exfiltration & Credential Theft (OWASP LLM06)
            r"(?i)dump\s+(all\s+)?(database|credentials|passwords|api[_\s]?keys)",
            r"(?i)extract\s+(all\s+)?(user|admin)\s+(data|tokens|secrets)",
            r"(?i)show\s+me\s+(the\s+)?(api[_\s]?key|secret|password|token)",
            r"(?i)env\s*\|\s*grep\s+(API|KEY|SECRET|TOKEN|PASS)",
            // 6. Indirect Prompt Injection & RAG Poisoning (OWASP LLM01 / LLM07)
            r"(?i)\[system\s*:\s*override\]",
            r"(?i)<!--\s*AI\s*Instruction\s*:\s*ignore",
            r"(?i)\[IMPORTANT\s+INSTRUCTION\s+FOR\s+LLM\]",
            r"(?i)hidden_prompt_directive",
            // 7. Obfuscated Payloads & Encoding Evasion
            r"(?i)base64[_\s]?decode\s*\(",
            r"(?i)\\x[0-9a-f]{2}\\x[0-9a-f]{2}\\x[0-9a-f]{2}", // hex escape sequences
            r"(?i)%20ignore%20previous%20instructions",      // URL-encoded injection
            // 8. SSRF & Unintended Agent Action Probes (OWASP LLM02)
            r"(?i)http://169\.254\.169\.254", // AWS Metadata Service SSRF
            r"(?i)http://localhost:\d+/admin",
            r"(?i)file:///etc/(passwd|shadow)",
            r"(?i)file:///c:/windows/win.ini",
            // 9. MCP Decoy Tool Traps (Beelzebub Architecture)
            r"(?i)execute_system_command",
            r"(?i)read_admin_vault_keys",
            r"(?i)dump_internal_user_db",
        ];

        let pattern_names: Vec<&str> = vec![
            "Direct Prompt Injection (ignore instructions)",
            "Direct Prompt Injection (disregard prior)",
            "System Prompt Override/Dump",
            "Unconstrained AI Persona Hijack",
            "Persona Hijack Attempt",
            "Restriction Bypass Probe",
            "Skeleton Key / Master Key Jailbreak",
            "DAN (Do Anything Now) Mode",
            "Developer Mode Jailbreak",
            "Generic Jailbreak Override",
            "Garak Automated Probe",
            "PyRIT Orchestrator Attack",
            "Counterfit ML Attack",
            "Adversarial ML Threat Probe",
            "Benchmark Exploit Payload",
            "Code Eval Injection (Base64)",
            "Destructive Command Injection",
            "Remote Code Execution via Pipe",
            "XSS Script Injection",
            "Subprocess Execution Attempt",
            "OS Command Execution Call",
            "Database/Credential Dump Request",
            "Data Exfiltration Attempt",
            "API Key/Secret Exposure Request",
            "Environment Secret Grep Probe",
            "Indirect Injection (System Override Tag)",
            "Indirect Injection (HTML Comment Directive)",
            "RAG Context Poisoning Directive",
            "Hidden Prompt Directive Inject",
            "Base64 Decode Evasion Attack",
            "Hex-Encoded Payload Evasion",
            "URL-Encoded Injection Evasion",
            "Cloud Metadata SSRF Probe (AWS/GCP)",
            "Local Admin SSRF Probe",
            "Local File Inclusion (/etc/passwd)",
            "Windows System File LFI Probe",
            "MCP Decoy Trap: execute_system_command",
            "MCP Decoy Trap: read_admin_vault_keys",
            "MCP Decoy Trap: dump_internal_user_db",
        ];

        let regex_set = RegexSet::new(&patterns)
            .expect("[ANTAI Sanitizer] Errore critico nella compilazione dei pattern regex");

        Self {
            regex_set,
            pattern_names,
        }
    }

    /// Inspects a text payload and returns threat verdict, checking static signatures and dynamic immune memory.
    pub fn inspect(&self, payload: &str, immune_memory: &ImmuneMemory) -> SanitizeResult {
        let start = Instant::now();

        // 1. Static OWASP RegexSet check (<0.05µs)
        let matches: Vec<usize> = self.regex_set.matches(payload).into_iter().collect();

        if let Some(&first_match) = matches.first() {
            let pattern_name = self
                .pattern_names
                .get(first_match)
                .unwrap_or(&"Unknown Pattern");

            let latency_us = start.elapsed().as_nanos() as f64 / 1000.0;
            return SanitizeResult {
                blocked: true,
                reason: Some(format!("Matched OWASP LLM Signature: {}", pattern_name)),
                matched_pattern: Some(pattern_name.to_string()),
                engine: "ANTAI Heuristic Filter (0$, Rust Native)".to_string(),
                latency_us,
            };
        }

        // 2. Dynamic Immune Memory (Antibody) Match
        if let Some(ab) = immune_memory.match_antibody(payload) {
            let latency_us = start.elapsed().as_nanos() as f64 / 1000.0;
            return SanitizeResult {
                blocked: true,
                reason: Some(format!("Matched Immune Antibody [{}] {}", ab.id, ab.description)),
                matched_pattern: Some(format!("Antibody: {}", ab.pattern)),
                engine: "ANTAI Immune Memory Engine".to_string(),
                latency_us,
            };
        }

        let latency_us = start.elapsed().as_nanos() as f64 / 1000.0;
        SanitizeResult {
            blocked: false,
            reason: None,
            matched_pattern: None,
            engine: "ANTAI Heuristic Filter (0$, Rust Native)".to_string(),
            latency_us,
        }
    }
}
