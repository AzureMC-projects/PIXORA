# Access Keys and Activation

**Status (2026-10-12):** A dedicated Supabase project, database schema, and the `issue-key` / `activate-key` Edge Functions are deployed. The portal has been changed to remove CAPTCHA and Turnstile. Server-side issuance limits are enabled: 3 requests per IP per 10-minute bucket and 100 total requests per hour. The desktop app has an activation screen and stores the returned expiry locally. Ongoing server-side entitlement checks, robust cryptographic device binding, monitoring, and transactional hardening remain incomplete; this is not production-grade access enforcement.

## Policy

- Key delivery: users request a key on the official PIXORA website; no email is required.
- Anti-abuse: no CAPTCHA. The backend uses per-IP and global request limits; limits may affect multiple users sharing an IP.
- Redemption: each key can be redeemed exactly once.
- Access duration: 10 days from the first successful activation.
- Device scope: one installation/device per key is the intended policy. The current desktop client sends an installation identifier, but it is not yet a robust cryptographic device binding.
- Renewal: after expiration, the user returns to the website and requests another key.

## User flow

1. The user visits the PIXORA key page and requests a key. No account, email, or CAPTCHA is required.
2. The server applies rate limits, creates a cryptographically random key, stores only its hash, and displays the plaintext key once.
3. In PIXORA, the user enters the key and sends an installation identifier with the activation request over HTTPS.
4. The server attempts to mark the key as redeemed and starts a 10-day entitlement.
5. The current desktop app caches expiry locally. A server-side entitlement-check endpoint and secure device binding are still needed for stronger enforcement.
6. The same key should not activate a second installation. After expiration, the user requests a new key from the website.

## Key website protections

The public no-email/no-CAPTCHA flow can be abused by automated clients. Current protections:

- Per-IP limit of 3 key requests per 10-minute bucket.
- Global limit of 100 requests per hour to help protect the free backend.
- IP addresses are hashed before being stored in the limiter table.
- Cryptographically random keys with enough entropy to resist guessing.
- Hash-only storage of keys; display each raw key only once and never log it.
- HTTPS and generic errors for service failures.

Rate limits make casual abuse harder but do not prove a unique human or reliably stop distributed bots. A no-email flow cannot reliably limit a person to one key per human. Monitor usage and adjust limits if necessary.

## Server and database rules

Use the dedicated PIXORA Supabase project for server-side functions. Keep all privileged secrets on the server; never include a service-role key or key-generation secret in the desktop app or browser bundle.

The database uses a private access-key table and a separate issuance-counter table with row-level security enabled. Clients must not be able to list keys, create arbitrary valid keys, change redemption status, or inspect other users' records. Privileged operations go through server-side functions.

## Device binding and recovery

- The current desktop app's installation identifier is not a hardware fingerprint or a cryptographic proof of device identity.
- A reinstall or device change may lose local activation data. Define a support/recovery process before launch; without email or accounts, recovery is harder and may require manual review.
- Do not claim the key system is impossible to bypass: an open-source local client can be modified. Server checks can protect the official hosted service and official builds, but cannot enforce rules against every modified community build.

## Remaining rollout work

1. Verify GitHub Pages is publishing the updated portal.
2. Test issuance limits and confirm that HTTP 429 responses display a helpful message.
3. Test expired, reused, malformed, and concurrently redeemed keys.
4. Implement atomic activation and a server-side entitlement-check endpoint.
5. Improve installation key storage/device binding, and publish privacy and recovery guidance before launch.
