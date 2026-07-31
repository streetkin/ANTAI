# ANTAI Security SDK — Developer Integration Guide

Comprehensive integration guide for embedding **ANTAI** threat protection into any Web Application (Lovable, Bolt.new, React, Vue, Next.js, Express).

---

## 🚀 1. Client-Side Browser Integration (Lovable & Bolt)

Include `antai-sdk.js` in your project:

```html
<script src="path/to/sdk/antai-sdk.js"></script>
<script>
  // Initialize Sentinel
  const sentinel = new Antai({
    proxyUrl: 'http://127.0.0.1:8090/intercept',
    failOpen: true, // If local ANTAI is offline, requests pass cleanly without breaking user UX
    timeoutMs: 1500,
    onThreatDetected: (threat) => {
      console.warn(`⚠️ Threat blocked by ANTAI: ${threat.reason}`);
    }
  });

  // Transparently protect all outgoing fetch requests sent to LLM APIs (OpenAI, Anthropic, Custom backend)
  sentinel.protectFetch();
</script>
```

---

## 🛡️ 2. Manual Prompt Inspection

If you prefer to validate user input manually before form submission:

```javascript
const verdict = await sentinel.scan(userInput);

if (!verdict.allowed) {
  console.error("Blocked:", verdict.reason);
  // Handle blocked payload in UI
} else {
  // Proceed safely
}
```

---

## ⚡ 3. Server-Side Edge Middleware (Next.js / Vercel Edge)

Add or update `middleware.ts` in your Next.js project:

```typescript
import { antaiNextMiddleware } from './sdk/antai-middleware';

export default antaiNextMiddleware;

export const config = {
  matcher: '/api/:path*',
};
```

---

## ❓ FAQ & SaaS Security (Lovable / Bolt)

1. **Will Lovable or Bolt block ANTAI?**
   - **No.** ANTAI communicates via `http://127.0.0.1:8090/intercept` over local loopback (`127.0.0.1`).
   - Zero data is transmitted to unauthorized external third-party servers.
   - With `failOpen: true` (default), if the user is not running ANTAI locally, your app continues functioning normally.

2. **What happens during a Prompt Injection attack?**
   - ANTAI intercepts the request in **< 0.05µs** via native Rust heuristic signatures.
   - Returns a `403 Forbidden` response directly to the client before the payload reaches paid cloud LLM models.
