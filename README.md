# 🇮🇹 ANTAI — Digital Immune System & AI Cyber Sentinel

<p align="center">
  <img src="antai_logo.png" alt="ANTAI Logo" width="180">
</p>

<p align="center">
  <b>Un'infrastruttura di Cybersicurezza per la Protezione Attiva dei Sistemi di Intelligenza Artificiale</b><br>
  <i>Ispirata al Sistema Immunitario Biologico (Deception Engine & Memoria Immunologica)</i>
</p>

<p align="center">
  <a href="#-visione--metafora-immunologica"><img src="https://img.shields.io/badge/Architecture-Biological%20Immune-008751.svg" alt="Biological Immune System"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Rust-1.95-cd212a.svg" alt="Rust Native Core"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Latency-%3C%200.05%C2%B5s-1a73e8.svg" alt="Ultra Low Latency"></a>
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

## 🏗️ Architettura del Sistema

ANTAI è progettato con un'architettura **Asimmetrica e Ad Alte Prestazioni**:

```
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │                         Client App / Web Browser                            │
 │              (SDK Universal Proxy / Intercettatore Rest Fetch)              │
 └──────────────────────────────────────┬──────────────────────────────────────┘
                                        │ (Intercettazione HTTP / API)
                                        ▼
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ ⚡ ANTAI NATIVE RUST CORE (Porta 8090 Interceptor / Porta 8091 REST Bridge)  │
 ├─────────────────────────────────────────────────────────────────────────────┤
 │                                                                             │
 │  1. 🧬 Sanitizer Istintivo (Micro-Latenza < 0.05µs, Zero GC)                │
 │     Analisi Euristica ad altissima frequenza per blocco immediato          │
 │                                                                             │
 │  2. 🪤 Deception Engine (Esche dinamiche & Contromisure attive)             │
 │     Neutralizzazione attiva degli agenti ostili tramite risposte fittizie   │
 │                                                                             │
 │  3. 💉 Memoria Immunologica (Registro Anticorpi Evolutivi)                 │
 │     Apprendimento continuo e generazione di regole protettive dinamiche    │
 │                                                                             │
 │  4. 🛡️ System Audit Engine (sysinfo & process monitoring)                  │
 │     Monitoraggio integrità del sistema host in background                   │
 │                                                                             │
 └──────────────────────────────────────┬──────────────────────────────────────┘
                                        │
                      ┌─────────────────┴─────────────────┐
                      ▼                                   ▼
          🛑 MINACCIA BLOCCATA                 ✅ TRAFFICO PULITO
       (HTTP 403 / Deception Sandbox)       (Inoltro sicuro a LLM / DB)
```

---

## 💻 Stack Tecnologico

Il progetto è costruito sfruttando tecnologie ad altissima efficienza e stabilità:

### Backend Engine (Core Native)
- **Rust (v1.95+)**: Linguaggio core scelto per la massima sicurezza di memoria, assenza di Garbage Collector ed esecuzione nativa.
- **Axum & Tokio**: Framework web asincrono a microsecondi per la gestione a bassissima latenza dei socket e del proxy intercettatore.
- **Sysinfo & Regex Nativi**: Moduli compilati direttamente per il monitoraggio dei processi host e la scansione euristica ultra-rapida.

### Frontend Control Room (Dashboard Enterprise)
- **HTML5 & Vanilla JavaScript**: Nessun overhead di framework pesanti. Rendering immediato e reattivo.
- **Google Material Enterprise Design System**: Interfaccia pulita, chiara, schematizzata e riposante per la vista con dettagli dell'eccellenza Tricolore italiana.
- **Lucide Icons**: Iconografia moderna e pulita.

### Client Integration (SDK Universal)
- **ANTAI JS SDK (`antai-sdk.js`)**: Wrapper universale per applicazioni React, Next.js, Vue, Node.js e ambienti No-Code / Browser (Lovable, Bolt.new).

---

## ⚡ Caratteristiche Principali

- **Latenza Trascurabile ($\le 0.05\mu s$)**: Il primo filtro lavora in memoria a velocità nativa hardware.
- **Zero Costi di API Esterni**: L'analisi iniziale e la deception funzionano in locale su macchina.
- **Modalità Strict & Asimmetrica**:
  - *Strict Shield*: Blocco rigido ed immediato di qualsiasi anomalia rilevata.
  - *Deception / Hackademy Loop*: Risposta guidata che neutralizza l'attaccante e ne studia le mosse per rafforzare le difese.
- **Privacy & Sovranità dei Dati**: Nessun dato sensibile o prompt utente abbandona la rete locale se non esplicitamente autorizzato.

---

## 🚀 Avvio Rapido

### Requisiti
- **Rust** (versione 1.80 o superiore)
- Un browser web moderno (Chrome, Edge, Firefox)

### 1. Compilazione dell'Engine Rust
```bash
cd antai-core
cargo build --release
```

### 2. Avvio Sentinella ANTAI
Esegui lo script automatico:
- **Windows**: Doppio click su `start_antai.bat`
- **Linux / macOS**: Esegui `./target/release/antai-core`

Apri il file `index.html` nel browser per accedere alla **Control Room**.

---

## 📄 Integrazione Client (SDK)

Includi l'SDK in qualsiasi web app per intercettare e proteggere automaticamente tutte le chiamate API:

```javascript
import { Antai } from './sdk/antai-sdk.js';

const sentinel = new Antai({
    endpoint: 'http://localhost:8090/intercept',
    failOpen: true // Garantisce continuità operativa
});

// Attiva la protezione globale sulle fetch del client
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
