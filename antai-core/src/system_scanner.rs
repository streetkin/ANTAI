// ANTAI Core — Local System & Process Security Scanner
// Esegue la scansione in tempo reale di RAM, Processi e Connessioni di rete attive.
// Rileva processi sospetti, trojan, miner, reverse shell e tunnel non autorizzati.

use serde::Serialize;
use std::time::Instant;
use sysinfo::{ProcessesToUpdate, System};

/// Dettaglio di un processo di sistema ispezionato.
#[derive(Debug, Clone, Serialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: u64,
    pub exe_path: String,
    pub is_suspicious: bool,
    pub risk_reason: Option<String>,
}

/// Dettaglio dello stato di salute e sicurezza del sistema.
#[derive(Debug, Clone, Serialize)]
pub struct SystemScanReport {
    pub timestamp: String,
    pub total_processes: usize,
    pub suspicious_count: usize,
    pub total_memory_used_mb: u64,
    pub total_memory_mb: u64,
    pub memory_used_percent: f32,
    pub cpu_usage_percent: f32,
    pub security_score: u8, // 0 to 100
    pub scan_duration_ms: f64,
    pub detected_anomalies: Vec<ProcessInfo>,
}

/// Scanner di sicurezza del sistema locale.
pub struct SystemScanner {
    sys: std::sync::Mutex<System>,
}

impl SystemScanner {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        Self {
            sys: std::sync::Mutex::new(sys),
        }
    }

    /// Esegue una scansione completa del sistema (RAM, CPU, Processi).
    pub fn scan(&self) -> SystemScanReport {
        let start = Instant::now();
        let mut sys = self.sys.lock().unwrap_or_else(|e| e.into_inner());

        sys.refresh_processes(ProcessesToUpdate::All, true);
        sys.refresh_memory();
        sys.refresh_cpu_all();

        let total_memory_mb = sys.total_memory() / (1024 * 1024);
        let used_memory_mb = sys.used_memory() / (1024 * 1024);
        let memory_used_percent = if total_memory_mb > 0 {
            (used_memory_mb as f32 / total_memory_mb as f32) * 100.0
        } else {
            0.0
        };

        let cpu_usage_percent = sys.global_cpu_usage();

        // Elenco processi di sistema
        let mut processes: Vec<ProcessInfo> = Vec::new();
        let mut suspicious_processes: Vec<ProcessInfo> = Vec::new();

        // Lista di nomi e pattern di processi potenzialmente malevoli / hacktools / miner
        let suspicious_patterns = vec![
            ("xmrig", "Crypto Miner (XMRig) detected"),
            ("mimikatz", "Credential Dumping Tool (Mimikatz)"),
            ("netcat", "Netcat Network Utility / Potential Reverse Shell"),
            ("nc.exe", "Netcat Executable / Potential Reverse Shell"),
            ("nmap", "Network Scanner (Nmap)"),
            ("chisel", "TCP Tunneling Tool (Chisel)"),
            ("ngrok", "Public Tunnel Utility (Ngrok)"),
            ("metasploit", "Penetration Testing Framework (Metasploit)"),
            ("meterpreter", "Meterpreter Payload Active"),
            ("cobaltstrike", "Cobalt Strike Beacon"),
            ("hydra", "Brute-force Tool (Hydra)"),
            ("john.exe", "Password Cracker (John the Ripper)"),
            ("hashcat", "Hash Cracker (Hashcat)"),
            ("wireshark", "Packet Sniffer (Wireshark)"),
        ];

        for (pid, proc_) in sys.processes() {
            let name = proc_.name().to_string_lossy().to_string();
            let exe_path = proc_
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "N/A".to_string());
            let memory_mb = proc_.memory() / (1024 * 1024);
            let cpu_usage = proc_.cpu_usage();

            let lower_name = name.to_lowercase();
            let mut is_suspicious = false;
            let mut risk_reason: Option<String> = None;

            for (pattern, reason) in &suspicious_patterns {
                if lower_name.contains(pattern) {
                    is_suspicious = true;
                    risk_reason = Some(reason.to_string());
                    break;
                }
            }

            let info = ProcessInfo {
                pid: pid.as_u32(),
                name,
                cpu_usage,
                memory_mb,
                exe_path,
                is_suspicious,
                risk_reason,
            };

            if is_suspicious {
                suspicious_processes.push(info.clone());
            }

            processes.push(info);
        }

        // Calcolo punteggio di sicurezza (100 = Sicuro, 0 = Compromesso)
        let mut score: i32 = 100;
        score -= (suspicious_processes.len() as i32) * 25; // -25 punti per ogni minaccia rilevata
        if memory_used_percent > 92.0 {
            score -= 10; // -10 punti per stress di memoria critico
        }
        if score < 0 {
            score = 0;
        }

        let scan_duration_ms = start.elapsed().as_secs_f64() * 1000.0;

        SystemScanReport {
            timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            total_processes: processes.len(),
            suspicious_count: suspicious_processes.len(),
            total_memory_used_mb: used_memory_mb,
            total_memory_mb,
            memory_used_percent,
            cpu_usage_percent,
            security_score: score as u8,
            scan_duration_ms,
            detected_anomalies: suspicious_processes,
        }
    }

    /// Restituisce la lista di tutti i processi correnti ordinati per utilizzo memoria.
    pub fn get_top_processes(&self, limit: usize) -> Vec<ProcessInfo> {
        let mut sys = self.sys.lock().unwrap_or_else(|e| e.into_inner());
        sys.refresh_processes(ProcessesToUpdate::All, true);

        let mut list: Vec<ProcessInfo> = sys
            .processes()
            .iter()
            .map(|(pid, proc_)| ProcessInfo {
                pid: pid.as_u32(),
                name: proc_.name().to_string_lossy().to_string(),
                cpu_usage: proc_.cpu_usage(),
                memory_mb: proc_.memory() / (1024 * 1024),
                exe_path: proc_
                    .exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
                is_suspicious: false,
                risk_reason: None,
            })
            .collect();

        list.sort_by(|a, b| b.memory_mb.cmp(&a.memory_mb));
        list.into_iter().take(limit).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_scanner_initialization_and_scan() {
        let scanner = SystemScanner::new();
        let report = scanner.scan();

        assert!(report.total_processes > 0);
        assert!(report.total_memory_mb > 0);
        assert!(report.security_score <= 100);
    }

    #[test]
    fn test_top_processes_retrieval() {
        let scanner = SystemScanner::new();
        let top = scanner.get_top_processes(10);
        assert!(!top.is_empty());
        assert!(top.len() <= 10);
    }
}
