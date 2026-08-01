# 🇮🇹 ANTAI — Digital Immune System & AI Cyber Sentinel

<p align="center">
  <img src="antai_logo.png" alt="ANTAI Logo" width="180">
</p>

<p align="center">
  <b>Un'infrastruttura di Cybersicurezza Nativa per la Protezione Attiva dei Sistemi di Intelligenza Artificiale</b><br>
  <i>Ispirata al Sistema Immunitario Biologico (Deception Engine & Memoria Immunologica)</i>
</p>

<p align="center">
  <a href="#-visione--metafora-immunologica"><img src="https://img.shields.io/badge/Architecture-Biological%20Immune-008751.svg" alt="Biological Immune System"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Rust-1.95-cd212a.svg" alt="Rust Native Core"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Desktop%20App-Tauri%202.0-1a73e8.svg" alt="Tauri Native Desktop"></a>
  <a href="#-requisiti-di-sistema--compatibilita-pc"><img src="https://img.shields.io/badge/Latency-%3C%200.05%C2%B5s-1a73e8.svg" alt="Ultra Low Latency"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-050508.svg" alt="License"></a>
</p>

---

## 💡 Visione & Metafora Immunologica

**ANTAI** (AI Network Threat Neutralization & Adaptive Immunity) introduce un nuovo paradigma per la difesa dei sistemi IA enterprise, concepito come un **Sistema Immunitario Digitale Evolutivo**.

Mentre i firewall e le guardrail tradizionali agiscono come barriere rigide e statiche, ANTAI apprende e reagisce dinamicamente di fronte agli attacchi degli avversari (siano essi umani o agenti IA ostili):

1. **Riconoscimento Istintivo (Immunità Innata)**: Intercettazione ultra-rapida a livello di memoria e socket in tempo sub-microsecondo ($\le 0.05\mu s$).
2. **Deception Engine (Trappole & Contromisure)**: Generazione dinamica di ambienti *Decoy* (esche MCP/API) per depistare gli attaccanti, rallentarli e studiarne il comportamento senza esporre dati reali.
3. **Memoria Immunologica (Anticorpi IA)**: Quando viene identificato un nuovo vettore di attacco o un tentativo di bypass, ANTAI genera un "anticorpo digitale" riutilizzabile che immunizza l'intera rete aziendale contro varianti future dello stesso payload.

---

## 🏗️ Architettura del Sistema (Hybrid Enterprise Native)

<p align="center">
  <img src="antai_logo.png" alt="ANTAI System Architecture Logo" width="220">
</p>

ANTAI è strutturato come un vero software di sicurezza professionale a doppio livello:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                      SOFTWARE DESKTOP NATIVO (.EXE)                         │
└─────────────────────────────────────────────────────────────────────────────┘
                                       │
        ┌──────────────────────────────┴──────────────────────────────┐
        ▼                                                             ▼
┌──────────────────────────────────────────┐    ┌─────────────────────────────┐
│ ⚡ ANTAI RUST CORE ENGINE (Background)    │    │ 💻 ANTAI DESKTOP GUI        │
├──────────────────────────────────────────┤    ├─────────────────────────────┤
│ • Porta 8090: Interception Proxy         │    │ • Finestra Nativa (Tauri)   │
│ • Porta 8091: REST Bridge API            │ ◄─►│ • Control Room & Telemetria │
│ • Intercettazione < 0.05µs ($0 Costi)    │    │ • Nessun browser richiesto  │
│ • Deception Sandbox & Audit RAM          │    │ • Icona hi-res nella barra  │
└──────────────────────────────────────────┘    └─────────────────────────────┘
```

---

## 🖥️ Requisiti di Sistema & Compatibilità PC

ANTAI è stato ingegnerizzato in **Rust nativo** ed ottimizzato con **LTO (Link-Time Optimization)**. È estremamente leggero e può essere eseguito su un'ampia gamma di personal computer e server:

### ⚙️ Requisiti Hardware Minimi:
- **CPU**: Qualsiasi processore x86_64 o ARM64 (Intel Core i3 / AMD Ryzen 3 / Apple Silicon M1 o superiori).
- **RAM**: **Soltanto 128 MB di RAM libera** (il consumo medio a regime è inferiore a 40 MB).
- **Spazio su Disco**: ~50 MB di spazio libero.
- **Connessione Internet**: Non richiesta per la protezione base (funziona 100% offline). Opzionale per la sincronizzazione delle chiavi API esterne.

### 💻 Sistemi Operativi Supportati:
- **Windows**: Windows 10 e Windows 11 (64-bit) — *Supporto nativo WebView2*.
- **Linux**: Ubuntu 20.04+, Debian 11+, Fedora, Arch Linux.
- **macOS**: macOS 11 Big Sur o superiori (compresi chip M1/M2/M3/M4).

---

## 🚀 Guida all'Installazione e Compilazione

### Opzione A: Avvio Diretto Eseguibili Nativi (Pronto all'Uso)
Se hai scaricato il pacchetto compilato:
1. Fai doppio click su **`ANTAI-Sentinel-Desktop.exe`**: si aprirà l'applicazione Desktop nativa.
2. L'engine di protezione **`antai-core.exe`** verrà avviato automaticamente in background per proteggere le chiamate di rete.

---

### Opzione B: Compilazione Nativa dai Sorgenti (Developer / Build Automation)

Se vuoi compilare il software da zero sul tuo computer:

#### 1. Requisiti di Sviluppo
- Installa **Rust** (`rustup` / cargo 1.80+)
- Su Windows: installa Microsoft Visual Studio Build Tools (C++ workload).

#### 2. Compilazione Automatica
Esegui lo script automatizzato nella cartella principale del progetto:
- **Windows**: Fai doppio click sul file **`build_release_package.bat`** o compila da terminale:

```powershell
# Esegui lo script di Build Automatica
.\build_release_package.bat
```

Lo script eseguirà automaticamente:
1. Compilazione dell'Engine Rust Backend (`antai-core.exe`) in modalità `release`.
2. Sincronizzazione della directory isolata delle risorse `dist/`.
3. Compilazione dell'Applicazione Desktop Nativa (`ANTAI-Sentinel-Desktop.exe`) tramite Tauri 2.0.

---

## 📘 Guida all'Utilizzo del Software

### 1. Control Room (Dashboard Principale)
All'avvio dell'applicazione desktop ti troverai nella **Control Room**:
- **Stato Sentinella**: Mostra in tempo reale lo stato dell'engine (ONLINE / OFFLINE) e la latenza di analisi (tipicamente $< 0.05\mu s$).
- **Modalità Operativa**: 
  - `Shield Strict`: Blocco immediato e rigido di tutti i tentativi di injection o exploit.
  - `Deception Mode`: Attivazione delle esche per confondere gli agenti ostili.

### 2. Scansione Sicurezza PC
Accedi al tab **Scansione Sicurezza PC** e clicca su *Avvia Scansione Completa*:
- ANTAI analizzerà i processi attivi nella memoria RAM del computer alla ricerca di hacktool noti, miner o processi anomali.

### 3. Integrazione nelle Tue Applicazioni (SDK Hub)
Nel tab **Integrazione SDK / App** trovi i blueprint pronti da copiare per integrare ANTAI nelle tue Web App, server Node.js, Next.js, Python o Go:

```javascript
import { Antai } from './sdk/antai-sdk.js';

// Inizializza l'intercettatore locale
const sentinel = new Antai({
    endpoint: 'http://localhost:8090/intercept',
    failOpen: true
});

// Intercetta e protegge automaticamente tutte le chiamate API outgoing
sentinel.protectFetch();
```

---

## 🔒 Note sulla Sicurezza e Riservatezza

> **Nota di Riservatezza:** I dettagli specifici sulle euristiche avanzate di auto-evoluzione, i vettori esatti delle trappole del Deception Engine e i registri completi delle firme degli anticorpi **non sono esposti pubblicamente** nel presente repository per impedire il reverse-engineering da parte di agenti malevoli o avversari.

---

<p align="center">
  <b>ANTAI — Progettato e Sviluppato in Italia per la Sicurezza dell'Intelligenza Artificiale.</b>
</p> Audits active processes, system RAM/CPU load, and flags malicious hacktools/miners (`xmrig`, `mimikatz`, `netcat`, `chisel`, `ngrok`).
- **🌐 Universal Web SDK (Lovable, Bolt.new, React, Next.js, Vercel):** Single-line client integration with `protectFetch()` and zero-downtime `failOpen` mode.
- **🎨 Imperial Fluid UI (100vh Single Viewport):** Container-less design adhering to Imperial Crimson (`#ff003c`) aesthetics with real-time attack topology canvas graphs.

---

## 🚀 Quick Start (Windows / Linux / macOS)

### 1. Clone & Build

```bash
# Clone repository
git clone https://github.com/your-username/ANTAI.git
cd ANTAI

# Build native release binary
cd antai-core
cargo build --release
```

### 2. Run ANTAI Sentinel

Double click **`start_antai.bat`** (Windows) or execute:

```bash
# Start Native Engine & Control Room UI
./target/release/antai-core
```

Open `index.html` in your web browser to view the **Imperial Control Room**.

---

## 🔌 Developer Integration (Browser & Edge Middleware)

### Client-Side (Lovable / Bolt.new / React / HTML)

Include `sdk/antai-sdk.js` in your project:

```html
<script src="sdk/antai-sdk.js"></script>
<script>
  const sentinel = new Antai({ failOpen: true });
  sentinel.protectFetch(); // Automatically intercepts all outgoing prompt payloads
</script>
```

### Next.js / Vercel Edge Middleware

In your `middleware.ts`:

```typescript
import { antaiNextMiddleware } from './sdk/antai-middleware';

export default antaiNextMiddleware;

export const config = {
  matcher: '/api/:path*',
};
```

---

## 🛡️ OWASP LLM Top 10 Threat Coverage

| OWASP Vector | Threat Category | ANTAI Mitigation |
| :--- | :--- | :--- |
| **LLM01** | Direct & Indirect Prompt Injection | Microsecond RegexSet Match + Cross-Model Verdict |
| **LLM02** | Sensitive Information Disclosure / SSRF | AWS/GCP Cloud Metadata & Local File Inclusion Sanitizer |
| **LLM06** | Excessive Agency / Credential Theft | DB Dump & Secret Exposure Pattern Block |
| **LLM07** | Insecure Plugin / RAG Context Poisoning | Hidden Directive & HTML Comment Strip Filter |
| **LLM08** | Agentic Command & Tool Hijacking | OS Subprocess & Command Execution Piping Intercept |

---

## 📄 License

Distributed under the **MIT License** — Free and open-source for everyone.
