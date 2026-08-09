# Security Policy

[简体中文](./SECURITY.zh-CN.md)

## Supported Versions

| Version | Support status |
| --------- | ---------------- |
| Latest GitHub release | Supported |
| `main` | Receives fixes for the next release |
| Older releases | Not supported unless explicitly listed in a security advisory |

qr_forge is maintained by a small team. Security fixes are prioritized, but no service-level response or backport guarantee is provided.

## Report a Vulnerability Privately

Do not open a public issue, discussion, or pull request for a suspected vulnerability.

[Open a private security advisory](https://github.com/Tinkora/qr_forge/security/advisories/new)

GitHub private vulnerability reporting is the canonical security channel. Include:

- The affected version, commit, URL, or browser
- Reproduction steps or a minimal proof of concept
- Expected and observed behavior
- Security impact and required user interaction
- Any suggested mitigation or disclosure constraint

Maintainers aim to acknowledge complete reports within five business days. Investigation time depends on reproducibility, severity, and maintainer availability. Reporters will receive status updates through the private advisory.

## In Scope

- User input leaving the browser because of application behavior
- Script or markup injection through generated SVG, previews, filenames, or status messages
- Unsafe parsing of PNG logo data or generated image data
- Bypasses of documented input and resource limits
- WebAssembly memory-safety or sandbox-boundary issues attributable to this project
- Dependency vulnerabilities that materially affect qr_forge
- GitHub Actions, release artifacts, or Pages deployment supply-chain compromise

## Usually Out of Scope

- Browser, scanner, or dependency issues that do not affect qr_forge
- Hosting-provider logs for ordinary static asset requests
- Social engineering, physical attacks, or denial of service against GitHub
- Reports based only on automated scanner output without a reproducible impact
- Unsupported forks or modified deployments

Maintainers may still help route a credible upstream issue.

## Security and Privacy Model

qr_forge is a static browser application. QR and barcode generation runs in WebAssembly in the page. The application has no account system, database, analytics, or generation API, and it does not intentionally transmit user-entered payloads.

This boundary does not make the entire browsing session anonymous:

- GitHub Pages or another host receives normal requests for HTML, CSS, JavaScript, WebAssembly, and related metadata.
- Browser extensions, a compromised browser, or a modified deployment can observe page content.
- Downloaded SVG and PNG files contain the encoded payload and must be handled according to its sensitivity.
- A QR code with a logo is not guaranteed to scan in every physical condition; verify critical output independently.

Generated SVG uses project-created geometric elements rather than user-supplied markup. PNG logos are decoded locally and have browser UI limits in addition to core validation.

## Coordinated Disclosure

The maintainer will validate the report, assess affected versions, prepare a fix and tests, and coordinate a release through the private advisory. Public disclosure should wait until a fix or agreed mitigation is available. Credit is optional and will follow the reporter's preference.
