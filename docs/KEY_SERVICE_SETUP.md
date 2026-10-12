# PIXORA key service setup

The key portal is a static website; key issuance runs on a trusted Supabase Edge Function. The dedicated Supabase project, database schema, and `issue-key` / `activate-key` Edge Functions are deployed. The portal no longer uses CAPTCHA or Turnstile. The issuance function applies server-side per-IP and global rate limits.

## Security requirements

- Keep the dedicated Supabase project for PIXORA; do not reuse a personal or unrelated project.
- Never place the Supabase service-role key in the repository, portal JavaScript, or desktop application.
- The issue function generates cryptographically random keys, stores only a SHA-256 hash, and returns the plaintext key once.
- Issuance limits: at most 3 requests per IP per 10-minute bucket and 100 total requests per hour. Requests beyond either limit receive HTTP 429. IP addresses are hashed before storage in the limiter table.
- Issued keys must be activated within 24 hours. Activation starts a 10-day access period.
- Rate limits reduce casual abuse but are not proof of a unique human and are weaker than CAPTCHA against distributed bots. Monitor usage and adjust limits if needed.
- Treat public, open-source clients as modifiable: server-side entitlement checks are required for any service or features that must actually be gated.

## Deploy outline

1. Dedicated project: `PIXORA Access Service` (project ref `vrwumbswbcwpyhbpsiyx`, region `eu-west-2`).
2. Migration `202610120001_access_keys.sql` has already been applied; the issuance-limiter migration has also been applied.
3. The `issue-key` and `activate-key` Edge Functions have been deployed. `issue-key` no longer requires Turnstile.
4. Supabase provides `SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY` to Edge Functions. Set `PIXORA_ALLOWED_ORIGIN` to the exact published portal origin if needed.
5. The portal's `apiBaseUrl` and public publishable key are configured in `key-portal/portal-config.js`; no CAPTCHA site key or secret is required.
6. Publish the portal and test key issuance, rate limiting, valid/invalid activation, expired keys, and repeated activation attempts before announcing the service.

Example CLI commands (replace project reference and keep secrets out of shell history where possible):

```sh
supabase link --project-ref YOUR_PROJECT_REF
supabase db push
supabase functions deploy issue-key
supabase functions deploy activate-key
supabase secrets set PIXORA_ALLOWED_ORIGIN=https://azuremc-projects.github.io
```

## Important implementation caveat

The activation function currently checks then conditionally updates the key, but a strict production-grade implementation should move activation into a single PostgreSQL transaction/RPC to eliminate races. Add an authenticated entitlement-check endpoint and integrate it into PIXORA before treating access restrictions as enforceable. The desktop app currently caches expiry locally, so this is not yet production-grade access enforcement.
