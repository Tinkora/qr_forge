# Changelog

All notable changes to qr_forge are recorded here. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and the project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Nothing yet.

## [0.1.0] - 2026-08-10

### Added

- Browser-local QR generation for URL or text, Wi-Fi, vCard 3.0, phone, and email payloads.
- QR error correction levels L, M, Q, and H; configurable module size, quiet zone, foreground, and background.
- SVG and PNG rendering, plus PNG logo overlay with a validated 5% to 30% ratio.
- Code 128 output for printable ASCII, using subset C for even-length numeric input and subset B otherwise.
- EAN-13 output from 12 digits with an automatically calculated check digit.
- Standard Code 128 and EAN-13 quiet zones.
- English-first browser interface with a Simplified Chinese switch, keyboard-operable tabs, accessible form labels, and download status announcements.
- Stable machine-readable `CoreError` codes for generation, validation, rendering, and logo failures.
- Independent PNG decoding tests for QR, Code 128, and EAN-13 output.
- English and Simplified Chinese product scope, contract, maturity, contribution, security, support, and release documentation.

### Fixed

- Preserve meaningful leading and trailing whitespace in Wi-Fi SSIDs and passwords.
- Encode all 12 EAN-13 input digits before calculating the check digit.
- Reject Code 128 payloads over 128 bytes and unsupported control or non-ASCII characters.
- Reject non-finite and out-of-range logo ratios before decoding image data.
- Disable SVG download when a PNG logo is active because SVG logo composition is not implemented.

[Unreleased]: https://github.com/Tinkora/qr_forge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Tinkora/qr_forge/releases/tag/v0.1.0
