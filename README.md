# 🛡️ ANTAI — Digital Immune System & AI Cyber Sentinel

<p align="center">
  <img src="antai_logo.png" alt="ANTAI Logo" width="180">
</p>

<p align="center">
  <b>Un Sistema Immunitario Digitale Open-Source in Rust Nativo per la Difesa delle Web App e degli Agenti IA</b><br>
  <i>Sviluppato da <b>streetkin</b> — Protezione multilivello a costo zero per sviluppatori, startup ed il web.</i>
</p>

<p align="center">
  <a href="#-visione--metafora-immunologica"><img src="https://img.shields.io/badge/Architecture-Biological%20Immune-008751.svg" alt="Biological Immune System"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Rust-1.95-cd212a.svg" alt="Rust Native Core"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Desktop%20App-Tauri%202.0-1a73e8.svg" alt="Tauri Native Desktop"></a>
  <a href="#-requisiti-di-sistema--compatibilita-pc"><img src="https://img.shields.io/badge/Layer%201%20Latency-%3C%200.05%C2%B5s-1a73e8.svg" alt="Layer 1 Ultra Low Latency"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-050508.svg" alt="License MIT"></a>
</p>

---

## 💡 Visione & Metafora Immunologica

**ANTAI** (AI Network Threat Neutralization & Adaptive Immunity) è un progetto open-source sperimentale ed avveniristico concepito come un **Sistema Immunitario Digitale Evolutivo** per difendere le applicazioni web e gli agenti IA dagli attacchi della lista OWASP LLM 2026.

Invece di affidarsi solo a rigide porte blindate statiche, ANTAI organizza la difesa su più strati coordinati:

1. **🚀 Layer 1 — Barriera Epiteliale Cutanea (Filtro Euristico in Rust):**
   - Intercettazione deterministica in memoria a latenza sub-microsecondo ($\le 0.05\mu s$) e costo \$0 tramite `RegexSet` compilati nativamente.
2. **🧠 Layer 2 — Validazione Semantica Asimmetrica (Cross-Model AI Inspection):**
   - Valutazione asincrona e profonda dell'intento dell'input (tempo stimato: $50ms - 200ms$) per mezzo di modelli IA trasversali (Ollama locale 0\$, OpenRouter o Groq).
3. **🧫 Fagociti & Audit PC:**
   - Scansione della memoria RAM del sistema operativo e dei processi attivi per individuare e neutralizzare hacktool o miner.
4. **🧬 Memoria Immunologica (Registro Anticorpi):**
   - Generazione di impronte molecolari (anticorpi) per memorizzare i pattern neutralizzati ed immunizzare il sistema contro tentativi futuri.

---

## 🏗️ Architettura del Sistema

```text
  █████╗ ███╗   ██╗████████╗█████╗ ██╗
 ██╔══██╗████╗  ██║╚══██╔══╝██╔══██╗██║   AI CYBER SENTINEL
 ███████║██╔██╗ ██║   ██║   ███████║██║   AUTONOMOUS RUST ENGINE
 ██╔══██║██║╚██╗██║   ██║   ██╔══██║██║   MICROSECOND AI DEFENSE
 ██║  ██║██║ ╚████║   ██║   ██║  ██║██║
 ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝
```

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
│ • Local Proxy (127.0.0.1 Loopback)       │    │ • Finestra Nativa (Tauri)   │
│ • Secure REST Bridge API                 │ ◄─►│ • Control Room & Telemetria │
│ • Layer 1 Rust Regex (< 0.05µs)          │    │ • Nessun browser richiesto  │
│ • Layer 2 AI Inspection (50-200ms)       │    │ • Icona hi-res nella barra  │
└──────────────────────────────────────────┘    └─────────────────────────────┘
```

---

## 🛡️ OWASP LLM Top 10 Threat Coverage & Tempi di Esecuzione

| Vettore OWASP | Categoria Minaccia | Stratificazione Difensiva ANTAI | Tempo di Esecuzione Reale |
| :--- | :--- | :--- | :--- |
| **LLM01** | Direct & Indirect Prompt Injection | Layer 1 Rust Heuristic + Layer 2 Cross-Model | Layer 1: $< 0.05\mu s$ \| Layer 2: $50-200ms$ |
| **LLM02** | Sensitive Information Disclosure / SSRF | AWS/GCP Cloud Metadata (169.254.169.254) & LFI Block | Layer 1 Native Rust: $< 0.05\mu s$ |
| **LLM06** | Excessive Agency / Credential Theft | DB Dump & Secret Exposure Pattern Filter | Layer 1 Native Rust: $< 0.05\mu s$ |
| **LLM07** | Insecure Plugin / RAG Context Poisoning | Hidden Directive & HTML Comment Strip Filter | Layer 1 Native Rust: $< 0.05\mu s$ |
| **LLM08** | Agentic Command & Tool Hijacking | OS Subprocess & Command Execution Piping Intercept | Layer 1 Native Rust: $< 0.05\mu s$ |

---

## 🖥️ Requisiti di Sistema & Compatibilità PC

ANTAI è stato ingegnerizzato in **Rust nativo** ed ottimizzato con **LTO (Link-Time Optimization)**:

- **CPU**: Qualsiasi processore x86_64 o ARM64 (Intel Core i3 / AMD Ryzen 3 / Apple Silicon M1 o superiori).
- **RAM**: **Soltanto 128 MB di RAM libera** (il consumo medio del proxy Rust è inferiore a 30 MB).
- **Spazio su Disco**: ~50 MB di spazio libero.
- **Sistemi Operativi**: Windows 10/11 (64-bit), Linux (Ubuntu/Debian/Arch), macOS 11+.

---

## 🔌 Integrazione per Sviluppatori (SDK Web & Middleware)

### Browser & Frontend (Lovable / Bolt.new / React / HTML)

Incolla `sdk/antai-sdk.js` nel tuo progetto web:

```html
<script src="sdk/antai-sdk.js"></script>
<script>
  const sentinel = new Antai({
    proxyUrl: 'http://127.0.0.1:8090/intercept',
    failOpen: true
  });
  sentinel.protectFetch(); // Intercetta e bonifica automaticamente le chiamate agli LLM
</script>
```

### Next.js / Vercel Edge Middleware

Incolla nel tuo `middleware.ts`:

```typescript
import { antaiNextMiddleware } from './sdk/antai-middleware';

export default antaiNextMiddleware;

export const config = {
  matcher: '/api/:path*',
};
```

---

## 🔓 Trasparenza Open-Source & Licenza MIT

ANTAI è un progetto al 100% **Open-Source e Trasparente**. 

Tutte le firme euristiche, il codice sorgente Rust del proxy ed i moduli di scansione sono completamente aperti, audestabili e verificabili dalla community di cybersicurezza su GitHub sotto licenza **MIT License**.

> **Progettato da streetkin per la community di sviluppatori.**
