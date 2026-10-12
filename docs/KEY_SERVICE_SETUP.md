# PIXORA key service setup

The key portal is a static website; key issuance must run on a trusted server. The repository now includes a Supabase Edge Function implementation and SQL schema, but they are **source files only** until deployed to a dedicated Supabase project.

## Security requirements

- Create a new Supabase project dedicated to PIXORA. Do not reuse a personal or unrelated project.
- Create a Cloudflare Turnstile widget for the GitHub Pages domain.
- Never place the Supabase service-role key or Turnstile secret in the repository, the portal JavaScript, or the desktop application.
- The issue function verifies Turnstile server-side, stores only a SHA-256 hash of the generated key, and returns the plaintext key once.
- Issued keys must be activated within 24 hours. Activation starts a 10-day access period.
- Treat public, open-source clients as modifiable: server-side entitlement checks are required for any service or features that must actually be gated.

## Deploy outline

1. Create a dedicated Supabase project.
2. Apply `supabase/migrations/202610120001_access_keys.sql` in that project's SQL editor.
3. Deploy the functions `issue-key` and `activate-key` with the Supabase CLI.
4. Set Edge Function secrets `TURNSTILE_SECRET_KEY` and `PIXORA_ALLOWED_ORIGIN` (the latter should be the exact published portal origin). Supabase provides `SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY` to Edge Functions.
5. Set the portal's public `apiBaseUrl` to `https://YOUR_PROJECT.supabase.co/functions/v1`, `publishableKey` to the Supabase publishable/anon key, and `turnstileSiteKey` to the Turnstile **site key** in `key-portal/portal-config.js`.
6. Publish the portal and test valid, invalid, expired, and repeated activation attempts before announcing the service.

Example CLI commands (replace project reference and keep secrets out of shell history where possible):

```sh
supabase link --project-ref YOUR_PROJECT_REF
supabase db push
supabase functions deploy issue-key
supabase functions deploy activate-key
supabase secrets set TURNSTILE_SECRET_KEY=YOUR_SECRET
supabase secrets set PIXORA_ALLOWED_ORIGIN=https://azuremc-projects.github.io
```

## Important implementation caveat

The activation function currently checks then conditionally updates the key, but a strict production-grade implementation should move activation into a single PostgreSQL transaction/RPC to eliminate races and add rate limits/monitoring. Add an authenticated entitlement-check endpoint and integrate it into PIXORA before treating access restrictions as enforceable. These functions are a starting implementation, not a claim that a production key service is live.
