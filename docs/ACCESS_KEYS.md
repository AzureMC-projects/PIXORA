# Access Keys and Activation

**Status:** Proposed design; the activation service and key-request page are not implemented yet.

## User experience

1. A user opens the official PIXORA key-request page.
2. They enter an email address and complete an anti-abuse check.
3. PIXORA sends a one-time access key to that address. The initial rollout may issue keys manually until the request site is deployed.
4. The user signs in to PIXORA and enters the key.
5. The server atomically redeems the key and grants access for **10 days from first redemption**.
6. The same account can use PIXORA on supported desktop and browser clients while the entitlement is active. Redeemed keys cannot be reused or transferred to another account.
7. When the 10 days end, the app asks the user to request another key.

The key grants temporary access; it is not a payment receipt or a promise that a user will retain access indefinitely.

## Security requirements

- Generate keys on the server with a cryptographically secure random generator. Use enough entropy that keys cannot feasibly be guessed.
- Show/send the raw key only at issuance. Store only a cryptographic hash of each key in the database; never log raw keys.
- Redeem keys through an authenticated, rate-limited server endpoint. Make redemption atomic so concurrent requests cannot redeem the same key twice.
- Calculate expiration on the server at redemption time (`redeemed_at + 10 days`). Do not trust the device clock.
- Store account ID, key hash, issued time, redeemed time, expiration, and revocation status. Keep the minimum personal data needed to operate the service.
- Validate access with the server. Never embed an admin secret, database service-role key, or key-generation secret in the desktop app, browser bundle, or public repository.
- Rate-limit key requests, email delivery, sign-in, and redemption; use generic responses that do not reveal whether an email has an account.
- Provide a support/revocation path and document how email addresses and activation records are retained/deleted.
- Do not use invasive hardware fingerprinting. Account-based activation gives users a predictable way to use the app on their own supported devices.

## Suggested implementation

Use Supabase Auth for accounts and a Postgres table for key records, with server-side Edge Functions for requesting, issuing, redeeming, and checking entitlements. Enable row-level security; clients must not be able to list keys, inspect other users' records, or mark keys as redeemed. Keep privileged operations inside server-side functions and validate every request.

Suggested records:

- `access_keys`: key hash, status, issued timestamp, redeemed timestamp, redeemed account ID, expiration timestamp, and optional issuance batch/reference.
- `access_entitlements`: account ID, start timestamp, expiration timestamp, revocation timestamp/reason.
- `key_requests`: normalized email or account reference, request timestamp, delivery status, and abuse-control metadata with a documented retention period.

Use database constraints and a transaction or equivalent atomic function to enforce one-time redemption. Avoid storing raw keys in these tables.

## Rollout plan

1. Confirm the policy and publish the key-request URL/contact.
2. Configure the production Supabase project, Auth email delivery, allowed redirect URLs, secrets, rate limits, and database migrations.
3. Implement and test key issuance and atomic redemption.
4. Add a PIXORA activation screen and server-backed entitlement checks to desktop and browser clients.
5. Add tests for expired, reused, revoked, malformed, and concurrently redeemed keys; test server/device clock skew and service outages.
6. Publish a privacy notice and support instructions before collecting email addresses.

## Important limitation

A key system is not secure if the client alone decides whether a key is valid. Because PIXORA is intended to be open source, a determined person can modify a locally built client. Server-side validation can protect the official hosted service and official builds, but cannot make a public-source application impossible to bypass. Decide whether access gating is meant for the official service/builds or is a strict requirement for all community builds before investing in enforcement.
