/**
 * ANTAI Security SDK v2.0.0 (Browser / Web Apps / Lovable / Bolt / Node.js)
 * Client-side sentinel that intercepts outgoing LLM prompts and user inputs.
 * Connects to local ANTAI Rust Proxy (http://127.0.0.1:8090/intercept).
 * Supports Deception Honeypot diversion and Cognitive Counter-Payloads.
 */

(function (global, factory) {
  if (typeof module === "object" && typeof module.exports === "object") {
    module.exports = factory();
  } else if (typeof define === "function" && define.amd) {
    define([], factory);
  } else {
    global.Antai = factory();
  }
}(typeof window !== "undefined" ? window : this, function () {
  "use strict";

  const DEFAULT_CONFIG = {
    proxyUrl: "http://127.0.0.1:8090/intercept",
    failOpen: true, // Se ANTAI locale è spento, fa passare la richiesta senza bloccare l'app
    timeoutMs: 1500, // Timeout massimo di verifica (1.5s)
    onThreatDetected: null, // Callback custom: (threat) => void
  };

  class AntaiSentinel {
    constructor(options = {}) {
      this.config = Object.assign({}, DEFAULT_CONFIG, options);
    }

    /**
     * Valuta un payload di testo prima di inviarlo a un'API o LLM.
     * @param {string} payload - Il testo/prompt da verificare.
     * @returns {Promise<{allowed: boolean, status: string, reason?: string, engine?: string, simulated_payload?: string}>}
     */
    async scan(payload) {
      if (!payload || typeof payload !== "string" || payload.trim() === "") {
        return { allowed: true, status: "clean", engine: "ANTAI Passthrough" };
      }

      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), this.config.timeoutMs);

      try {
        const response = await fetch(this.config.proxyUrl, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ payload }),
          signal: controller.signal,
        });

        clearTimeout(timeoutId);

        const data = await response.json();

        if (response.status === 403 || data.status === "blocked") {
          const result = {
            allowed: false,
            status: "blocked",
            reason: data.reason || "Payload identificato come minaccia informatica.",
            engine: data.engine || "ANTAI Core",
            latency: data.latency,
          };

          if (typeof this.config.onThreatDetected === "function") {
            this.config.onThreatDetected(result);
          }

          return result;
        }

        if (data.status === "diverted" || data.status === "counterattacked") {
          const result = {
            allowed: false, // Intercettato
            status: data.status,
            reason: data.reason,
            engine: data.engine,
            latency: data.latency,
            simulated_payload: data.simulated_payload,
            canary_token: data.canary_token,
          };

          if (typeof this.config.onThreatDetected === "function") {
            this.config.onThreatDetected(result);
          }

          return result;
        }

        return {
          allowed: true,
          status: "clean",
          engine: data.engine || "ANTAI Core",
          latency: data.latency,
        };
      } catch (err) {
        clearTimeout(timeoutId);

        if (this.config.failOpen) {
          return {
            allowed: true,
            status: "clean_fallback",
            reason: "ANTAI Core non raggiungibile (fail-open attivo).",
            engine: "Fallback Passthrough",
          };
        }

        return {
          allowed: false,
          status: "error",
          reason: "ANTAI Core non raggiungibile.",
          engine: "ANTAI Sentinel",
        };
      }
    }

    /**
     * Patch automatica del global `window.fetch` per intercettare trasparentemente le chiamate API/LLM.
     */
    protectFetch(urlPattern = /\/(chat|completions|generate|api\/v1)/i) {
      if (typeof window === "undefined" || !window.fetch) return;

      const originalFetch = window.fetch;
      const self = this;

      window.fetch = async function (input, init) {
        const url = typeof input === "string" ? input : (input && input.url ? input.url : "");

        if (init && init.body && typeof init.body === "string" && urlPattern.test(url)) {
          try {
            const bodyObj = JSON.parse(init.body);
            let payloadToScan = "";

            if (typeof bodyObj.prompt === "string") {
              payloadToScan = bodyObj.prompt;
            } else if (Array.isArray(bodyObj.messages)) {
              payloadToScan = bodyObj.messages.map(m => m.content).join("\n");
            } else {
              payloadToScan = init.body;
            }

            const verdict = await self.scan(payloadToScan);

            if (!verdict.allowed) {
              if (verdict.status === "diverted" || verdict.status === "counterattacked") {
                console.warn("[ANTAI DECEPTION] Threat diverted to synthetic environment:", verdict.reason);
                return new Response(verdict.simulated_payload || "{}", {
                  status: 200,
                  headers: { "Content-Type": "application/json" },
                });
              }

              console.warn("[ANTAI SHIELD] Attacco bloccato:", verdict.reason);
              return new Response(
                JSON.stringify({
                  error: {
                    message: `[ANTAI SECURITY BLOCKED] ${verdict.reason}`,
                    type: "antai_threat_blocked",
                    code: 403,
                  },
                }),
                {
                  status: 403,
                  headers: { "Content-Type": "application/json" },
                }
              );
            }
          } catch (e) {
            // Se il body non è JSON, procedi con l'originario
          }
        }

        return originalFetch.apply(this, arguments);
      };

      console.log("[ANTAI SDK v2.0] 🛡️ Automatic fetch protection & deception active.");
    }
  }

  return AntaiSentinel;
}));
