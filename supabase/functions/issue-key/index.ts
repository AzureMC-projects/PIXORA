import { createClient } from "https://esm.sh/@supabase/supabase-js@2";

const corsHeaders = {
  "Access-Control-Allow-Origin": Deno.env.get("PIXORA_ALLOWED_ORIGIN") ?? "https://azuremc-projects.github.io",
  "Access-Control-Allow-Headers": "authorization, x-client-info, apikey, content-type",
  "Access-Control-Allow-Methods": "POST, OPTIONS",
  "Content-Type": "application/json",
};

function response(body: unknown, status = 200) {
  return new Response(JSON.stringify(body), { status, headers: corsHeaders });
}

async function sha256(value: string): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", new TextEncoder().encode(value));
  return Array.from(new Uint8Array(digest), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

Deno.serve(async (request) => {
  if (request.method === "OPTIONS") return new Response("ok", { headers: corsHeaders });
  if (request.method !== "POST") return response({ error: "POST required" }, 405);

  try {
    const { turnstileToken } = await request.json();
    if (typeof turnstileToken !== "string" || turnstileToken.length < 10) {
      return response({ error: "Complete the bot-verification challenge first." }, 400);
    }

    const secret = Deno.env.get("TURNSTILE_SECRET_KEY");
    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!secret || !supabaseUrl || !serviceRoleKey) {
      return response({ error: "Key service is not configured yet." }, 503);
    }

    const verifyForm = new URLSearchParams({ secret, response: turnstileToken });
    const remoteIp = request.headers.get("cf-connecting-ip");
    if (remoteIp) verifyForm.set("remoteip", remoteIp);
    const verifyResponse = await fetch("https://challenges.cloudflare.com/turnstile/v0/siteverify", {
      method: "POST",
      body: verifyForm,
    });
    const verification = await verifyResponse.json();
    if (!verification.success) return response({ error: "Bot verification failed. Please try again." }, 403);

    const rawKey = `PX-${crypto.randomUUID().replaceAll("-", "").toUpperCase()}-${Array.from(crypto.getRandomValues(new Uint8Array(8)), (byte) => byte.toString(16).padStart(2, "0")).join("").toUpperCase()}`;
    const keyHash = await sha256(rawKey);
    const supabase = createClient(supabaseUrl, serviceRoleKey, {
      auth: { persistSession: false, autoRefreshToken: false },
    });
    const { error } = await supabase.from("pixora_access_keys").insert({ key_hash: keyHash });
    if (error) {
      console.error("Access key insert failed", error.code);
      return response({ error: "Could not issue a key right now. Please try again later." }, 500);
    }

    // The plaintext key is returned once and is never stored in the database or logged.
    return response({ key: rawKey, activationDeadlineHours: 24, accessDaysAfterActivation: 10 });
  } catch {
    return response({ error: "Invalid request." }, 400);
  }
});
