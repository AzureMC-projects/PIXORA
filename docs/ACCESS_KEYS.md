# Access Keys and Activation

**Status:** Design defined; the key website, activation API, and app integration are not implemented yet.

## Policy

- Key delivery: users request a key on the official PIXORA website; no email is required.
- Redemption: each key can be redeemed exactly once.
- Access duration: 10 days from the first successful activation.
- Device scope: one installation/device per key. The activation is bound to a per-installation cryptographic public key, not a hardware fingerprint.
- Renewal: after expiration, the user returns to the website and requests another key.

## User flow

1. The user visits the PIXORA key page and completes an anti-abuse check (for example, a CAPTCHA or proof-of-work challenge plus rate limits).
2. The server creates a cryptographically random key and displays it once on the page. The user copies it; no email or account is required for key delivery.
3. In PIXORA, the user enters the key. The app creates or loads a local installation keypair and sends the public key with the activation request over HTTPS.
4. The server atomically marks the key as redeemed and binds it to that installation public key. It starts a 10-day entitlement.
5. The app periodically checks the entitlement with the server and caches a short-lived signed authorization response for temporary offline use.
6. The same key cannot activate a second installation. After expiration, the user requests a new key from the website.

## Key website protections

A public page that gives out keys without email or payment can be abused by bots. Before production, add:

- CAPTCHA or another abuse challenge, server-side verified.
- Per-IP and global issuance limits, with a cooldown and monitoring.
- A maximum number of keys issued per time window and emergency issuance disable switch.
- Cryptographically secure random keys with enough entropy to resist guessing.
- Hash-only storage of keys; display each raw key only once and never log it.
- Generic error responses, HTTPS, security headers, and monitoring without collecting unnecessary personal data.

A no-email flow cannot reliably limit a person to one key per human. The above measures raise abuse costs but do not prove a unique human identity.

## Server and database rules

Use a dedicated PIXORA production backend (for example, a new Supabase project) with server-side functions for key issuance, activation, and entitlement checks. Do not connect this feature to an unrelated project without explicit approval. Keep all privileged secrets on the server; never include a service-role key or key-generation secret in the desktop app or browser bundle.

Suggested tables:

- `access_keys`: key hash, issued timestamp, redeemed timestamp, bound installation public-key fingerprint, expiration timestamp, and revocation status.
- `issuance_limits`: time-bucketed counters and abuse-control metadata with a short retention period.
- `activation_events`: minimal audit events for activation, renewal, revocation, and rejected reuse attempts.

Enable row-level security on exposed tables. Clients must not be able to list keys, create arbitrary valid keys, change redemption status, or inspect other users' records. Privileged operations must go through server-side functions. Redemption must be atomic so two simultaneous requests cannot use the same key.

## Device binding and recovery

- Generate an installation keypair using platform-supported cryptography and keep the private key in the operating system's secure storage where available.
- Send only the public key to the server. Do not rely on easily changed hardware IDs or invasive device fingerprinting.
- A reinstall or device change may lose the installation key. Define a support/recovery process before launch; without email or accounts, recovery is harder and may require manual review.
- Do not claim the key system is impossible to bypass: an open-source local client can be modified. Server checks can protect the official hosted service and official builds, but cannot enforce rules against every modified community build.

## Rollout checklist

1. Choose and configure a dedicated production backend and hosting domain.
2. Build the public key-request page, CAPTCHA verification, issuance limits, and one-time key display.
3. Build atomic activation and entitlement-check endpoints.
4. Add a PIXORA activation screen and secure installation key storage to the desktop and browser clients.
5. Test expired, reused, revoked, malformed, and concurrently redeemed keys; test clock skew, lost installation keys, rate limits, and service outages.
6. Publish a short privacy notice and recovery/support instructions before launch.
