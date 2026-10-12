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
    const { key, installationPublicKey } = await request.json();
    if (typeof key !== "string" || !/^PX-[A-F0-9]{32}-[A-F0-9]{16}$/.test(key)) {
      return response({ error: "Access key format is invalid." }, 400);
    }
    if (typeof installationPublicKey !== "string" || installationPublicKey.length < 32 || installationPublicKey.length > 512) {
      return response({ error: "Installation public key is invalid." }, 400);
    }

    const supabaseUrl = Deno.env.get("SUPABASE_URL");
    const serviceRoleKey = Deno.env.get("SUPABASE_SERVICE_ROLE_KEY");
    if (!supabaseUrl || !serviceRoleKey) return response({ error: "Activation service is not configured." }, 503);

    const supabase = createClient(supabaseUrl, serviceRoleKey, {
      auth: { persistSession: false, autoRefreshToken: false },
    });
    const keyHash = await sha256(key);
    const { data, error } = await supabase
      .from("pixora_access_keys")
      .select("id,status,activation_deadline")
      .eq("key_hash", keyHash)
      .maybeSingle();

    if (error) return response({ error: "Activation service could not check the key." }, 500);
    if (!data || data.status !== "issued" || new Date(data.activation_deadline).getTime() < Date.now()) {
      return response({ error: "Key is invalid, already used, or its activation window has expired." }, 403);
    }

    const activatedAt = new Date();
    const expiresAt = new Date(activatedAt.getTime() + 10 * 24 * 60 * 60 * 1000);
    const { data: updated, error: updateError } = await supabase
      .from("pixora_access_keys")
      .update({
        status: "active",
        activated_at: activatedAt.toISOString(),
        expires_at: expiresAt.toISOString(),
        installation_public_key: installationPublicKey,
      })
      .eq("id", data.id)
      .eq("status", "issued")
      .select("id")
      .maybeSingle();

    if (updateError) return response({ error: "Could not activate key." }, 500);
    if (!updated) return response({ error: "This key was already activated. Please request a new key." }, 409);
    return response({ activated: true, expiresAt: expiresAt.toISOString() });
  } catch {
    return response({ error: "Invalid request." }, 400);
  }
});
