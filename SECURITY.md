# Security Policy

PIXORA is in early development and does not yet promise stable-release security support.

## Reporting a vulnerability

Please do not publish exploitable details in a public issue. Use GitHub's private vulnerability reporting feature if enabled. Otherwise, contact the repository maintainer privately through a verified GitHub contact method.

Include the affected commit or version, environment, impact, reproduction steps, and safe proof of concept. Do not include secrets or real user data.

## Security-minded development

- Treat imported documents and media as untrusted input.
- Avoid unsafe Rust and carefully review exceptions.
- Review dependency provenance and licenses.
- Never commit credentials, signing keys, tokens, or private user data.
- Add regression tests for confirmed vulnerabilities where practical.
