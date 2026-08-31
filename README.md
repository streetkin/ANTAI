# 🛡️ ANTAI — Autonomous AI Cyber Security Gateway & Deception Fabric

<p align="center">
  <img src="antai_logo.png" alt="ANTAI Logo" width="180">
</p>

<p align="center">
  <b>Un Gateway di Sicurezza Nativo in Rust a Costo Zero ($0) per Sviluppatori, AI Builders e Web Apps</b><br>
  <i>Sviluppato da <b>streetkin</b> — Protezione locale, ingegneria di inganno ed ispezione asimmetrica a latenza microsecondo.</i>
</p>

<p align="center">
  <a href="https://github.com/streetkin/ANTAI/raw/main/ANTAI-Setup-v1.0.0.exe"><img src="https://img.shields.io/badge/Download-ANTAI%20Setup%20v1.0.0%20(.exe)-ff003c?style=for-the-badge&logo=windows" alt="Download ANTAI Setup"></a>
</p>

<p align="center">
  <a href="#-architettura-del-sistema"><img src="https://img.shields.io/badge/Architecture-4--Layer%20Pipeline-00f0ff.svg" alt="4-Layer Pipeline"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Rust-Native%20Core-ff003c.svg" alt="Rust Native Core"></a>
  <a href="#-stack-tecnologico"><img src="https://img.shields.io/badge/Desktop%20App-Tauri%202.0-10b981.svg" alt="Tauri Native Desktop"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-ffffff.svg" alt="License MIT"></a>
</p>

---

## 🚀 Download & Installazione Nativa Windows

Con **ANTAI** puoi proteggere i tuoi chatbot e le tue applicazioni Web create con Lovable, Bolt.new, React o Next.js senza costi di abbonamento.

📦 **[Scarica l'Installatore Diretto Nativo (.exe)](https://github.com/streetkin/ANTAI/raw/main/ANTAI-Setup-v1.0.0.exe)**  
*Installazione guidata in 10 secondi con icone automatiche su Desktop e Menu Start.*

---

## 💡 Cos'è ANTAI

**ANTAI** (AI Network Threat Neutralization & Adaptive Immunity) è un **AI Security Gateway** locale e leggero scritto in **Rust nativo**. Si posiziona l'applicazione Web / il Chatbot e l'LLM, intercettando i messaggi prima che vengano inviati al modello per prevenire attacchi di Prompt Injection, SSRF e RCE Agentico.

---

## 🏗️ Architettura difensiva a 4 Livelli

```text
  █████╗ ███╗   ██╗████████╗█████╗ ██╗
 ██╔══██╗████╗  ██║╚══██╔══╝██╔══██╗██║   AI CYBER SECURITY GATEWAY
 ███████║██╔██╗ ██║   ██║   ███████║██║   AUTONOMOUS RUST ENGINE
 ██╔══██║██║╚██╗██║   ██║   ██╔══██║██║   MICROSECOND LOCAL DEFENSE
 ██║  ██║██║ ╚████║   ██║   ██║  ██║██║
 ╚═╝  ╚═╝╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝
```

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                       INTERCETTORE PROXY NATIVO RUST                        │
└─────────────────────────────────────────────────────────────────────────────┘
                                       │
        ┌──────────────────────────────┴──────────────────────────────┐
        ▼                                                             ▼
┌──────────────────────────────────────────┐    ┌─────────────────────────────┐
│ ⚡ LAYER 1: Rust Proxy (< 0.05µs)        │    │ 🪤 LAYER 4: Beelzebub Fabric│
├──────────────────────────────────────────┤    ├─────────────────────────────┤
│ • RegexSet Euristico Determinostico     │    │ • Risposte Honeypot Fittizie│
│ • Blocco Prompt Injection & SSRF         │    │ • Trappole Esche MCP        │
└──────────────────────────────────────────┘    └─────────────────────────────┘
        │                                                             ▲
        ▼                                                             │
┌──────────────────────────────────────────┐    ┌─────────────────────────────┐
│ 🧠 LAYER 3: Asymmetric AI Classifier     │ ◄──│ 🧬 LAYER 2: Concept Memory  │
├──────────────────────────────────────────┤    ├─────────────────────────────┤
│ • Ollama Locale ($0 Costi API)           │    │ • Impronte Concettuali      │
│ • Fallback Cloud su Groq / OpenRouter    │    │ • Protezione Anti-Poisoning │
└──────────────────────────────────────────┘    └─────────────────────────────┘
```

1. **🚀 Layer 1 — Proxy Euristico in Rust (<0.05µs, $0 Costi):**
   - Intercettazione ad altissima velocità su socket locale (porta 8090) con `RegexSet` compilato nativamente.
2. **🧬 Layer 2 — Memoria Concettuale Adaptiva (Concept Fingerprinting):**
   - Generazione ed archiviazione di impronte concettuali (anticorpi) senza memorizzare testi grezzi, prevenendo falsi positivi ed avvelenamento della memoria.
3. **🧠 Layer 3 — Ispezione AI Asimmetrica:**
   - Valutazione semantica con Ollama o LM Studio locale ($0 costi API). Fallback automatico su Groq o OpenRouter.
4. **🪤 Layer 4 — Beelzebub Deception Fabric:**
   - Deviazione automatica degli attacchi verso risposte honeypot polimorfiche e trappole per strumenti MCP fittizi.

---

## 🛡️ Rilevamento Minacce & Copertura Eurisitica

| Vettore | Descrizione Minaccia | Meccanismo di Intercettazione ANTAI |
| :--- | :--- | :--- |
| **Agentic Shell RCE** | Iniezione comandi distruttivi (`rm -rf /`, `subprocess.call`) | Layer 1 Rust RegexSet + Layer 4 Beelzebub Trap |
| **Cloud IAM Metadata SSRF** | Furto credenziali Cloud AWS/GCP (`169.254.169.254`) | Layer 1 Rust Filter (< 0.05µs) |
| **SQL RAG Exfiltration** | Estrazione schemi DB e furto dati dal contesto | Layer 1 Regex + Layer 2 Concept Memory |
| **System Prompt Dump** | Sovrascrittura istruzioni di sistema (`ignore previous instructions`) | Layer 1 Regex + Layer 3 Asymmetric AI |
| **PyPI Slopsquatting Malware** | Inserimento pacchetti e comandi offuscati base64 | Layer 1 Regex + Layer 4 Polymorphic Honeypot |

---

## 🔍 Process & Diagnostic Security Scanner

ANTAI include uno scanner di processi locale trasparente e bilanciato:
- **Analisi Processi PID**: Monitora l'utilizzo di risorse (RAM e CPU) del sistema.
- **Rilevamento Minacce Critiche**: Identifica miner di criptovalute e tool di exfiltration non autorizzati.
- **Identificazione Strumenti di Diagnostica**: Riconosce gli strumenti di diagnostica e sviluppo (es. Wireshark, Ngrok, Nmap) come *"Strumenti Sensibili"* (-5 punti) offrendo massima trasparenza allo sviluppatore senza generare allarmismi.

---

## 🔌 Guida all'Integrazione SDK (Lovable, Next.js, React, Python, Node)

### 🩵 Lovable & Bolt.new (CDN Script)
Incolla nel file `index.html` del tuo progetto generato con Lovable o invia questo prompt a Lovable:
> *"Aggiungi questo script di protezione ANTAI nel tag <head> di index.html per intercettare le chiamate del chatbot contro prompt injection."*

```html
<script src="http://127.0.0.1:8090/sdk/antai-sdk.js"></script>
<script>
  const sentinel = new Antai({
    proxyUrl: 'http://127.0.0.1:8090/intercept',
    failOpen: true
  });
  sentinel.protectFetch();
</script>
```

### ⚛️ Next.js (App Router)
Incolla in `app/layout.tsx`:
```tsx
import Script from 'next/script';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="it">
      <head>
        <Script src="http://127.0.0.1:8090/sdk/antai-sdk.js" strategy="beforeInteractive" />
      </head>
      <body>{children}</body>
    </html>
  );
}
```

### 🐍 Python FastAPI
Incolla in `main.py`:
```python
from fastapi import FastAPI, Request
import requests

app = FastAPI()
ANTAI_PROXY_URL = "http://127.0.0.1:8090/intercept"

@app.middleware("http")
async def antai_security_middleware(request: Request, call_next):
    if request.url.path in ["/api/chat", "/v1/chat/completions"]:
        body = await request.json()
        resp = requests.post(ANTAI_PROXY_URL, json={"payload": body.get("prompt", "")})
        if resp.json().get("status") == "blocked":
            return JSONResponse(status_code=403, content={"error": "Prompt Blocked by ANTAI"})
    return await call_next(request)
```

---

## 🖥️ Requisiti di Sistema

- **CPU**: Qualsiasi processore x86_64 o ARM64.
- **RAM**: ~30 MB di consumo medio per il Proxy Rust.
- **OS**: Windows 10/11 (64-bit), Linux, macOS.

---

## 📜 Licenza

Rilasciato sotto licenza [MIT](LICENSE).
