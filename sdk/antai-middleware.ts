/**
 * ANTAI Security Middleware v1.0.0 (Next.js / Vercel / Node.js Edge)
 * Middleware per proteggere endpoint API server-side da prompt injection e minacce cibernetiche.
 */

export interface AntaiMiddlewareOptions {
  proxyUrl?: string;
  failOpen?: boolean;
  timeoutMs?: number;
  routesToProtect?: string[];
}

export interface AntaiVerdict {
  allowed: boolean;
  status: 'clean' | 'blocked' | 'error' | 'clean_fallback';
  reason?: string;
  engine?: string;
  latency?: string;
}

const DEFAULT_OPTIONS: AntaiMiddlewareOptions = {
  proxyUrl: 'http://127.0.0.1:8090/intercept',
  failOpen: true,
  timeoutMs: 1500,
  routesToProtect: ['/api/chat', '/api/generate', '/api/llm'],
};

/**
 * Scansiona un payload stringa tramite il Proxy ANTAI locale in Rust.
 */
export async function scanPayload(
  payload: string,
  options: AntaiMiddlewareOptions = {}
): Promise<AntaiVerdict> {
  const opts = { ...DEFAULT_OPTIONS, ...options };
  const proxyUrl = opts.proxyUrl || 'http://127.0.0.1:8090/intercept';

  if (!payload || payload.trim() === '') {
    return { allowed: true, status: 'clean', engine: 'ANTAI Passthrough' };
  }

  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), opts.timeoutMs);

  try {
    const res = await fetch(proxyUrl, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ payload }),
      signal: controller.signal,
    });

    clearTimeout(timeoutId);

    const data = await res.json();

    if (res.status === 403 || data.status === 'blocked') {
      return {
        allowed: false,
        status: 'blocked',
        reason: data.reason || 'Attacco informatico bloccato da ANTAI Sentinel.',
        engine: data.engine || 'ANTAI Core (Rust)',
        latency: data.latency,
      };
    }

    return {
      allowed: true,
      status: 'clean',
      engine: data.engine || 'ANTAI Core (Rust)',
      latency: data.latency,
    };
  } catch (error) {
    clearTimeout(timeoutId);

    if (opts.failOpen) {
      return {
        allowed: true,
        status: 'clean_fallback',
        reason: 'ANTAI Core non attivo (fail-open consentito).',
        engine: 'Fallback',
      };
    }

    return {
      allowed: false,
      status: 'error',
      reason: 'ANTAI Core non raggiungibile.',
      engine: 'ANTAI Sentinel',
    };
  }
}

/**
 * Middleware Next.js Edge (App Router / Pages Router)
 * Uso in `middleware.ts`:
 * ```ts
 * import { antaiNextMiddleware } from './sdk/antai-middleware';
 * export default antaiNextMiddleware;
 * ```
 */
export async function antaiNextMiddleware(req: Request) {
  const url = new URL(req.url);

  // Filtra solo rotte API o LLM
  if (!url.pathname.startsWith('/api/')) {
    return;
  }

  if (req.method === 'POST' || req.method === 'PUT') {
    try {
      const clonedReq = req.clone();
      const body = await clonedReq.text();

      let textToScan = body;
      try {
        const json = JSON.parse(body);
        if (json.prompt) textToScan = json.prompt;
        else if (Array.isArray(json.messages)) {
          textToScan = json.messages.map((m: any) => m.content).join('\n');
        }
      } catch (e) {}

      const verdict = await scanPayload(textToScan);

      if (!verdict.allowed) {
        return new Response(
          JSON.stringify({
            error: '[ANTAI SECURITY BLOCKED] Threat detected',
            reason: verdict.reason,
            engine: verdict.engine,
          }),
          {
            status: 403,
            headers: { 'Content-Type': 'application/json' },
          }
        );
      }
    } catch (err) {
      // Ignora errori di parsing ed esegui la request normale
    }
  }
}
