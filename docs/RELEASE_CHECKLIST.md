# Release Checklist

[简体中文](./RELEASE_CHECKLIST.zh-CN.md)

Use this checklist for every qr_forge release. Record the completed checklist in the release pull request or tracking issue. A checked box must point to command output, a required GitHub check, or a manual verification result.

## Release Record

- Version: `vX.Y.Z`
- Target commit:
- Release owner:
- Reviewer:
- Planned date:
- Tracking issue or pull request:

## 1. Scope and Contract

- [ ] Every shipped behavior is implemented; proposed behavior is not described as available.
- [ ] Changes fit [Product Scope](./PRODUCT_SCOPE.md), or an approved scope decision is linked.
- [ ] [Product Contract](./CONTRACT.md) matches current Rust, WASM, and browser behavior.
- [ ] [Maturity](./MATURITY.md) reports experimental and known limitations honestly.
- [ ] English and Simplified Chinese documents have equivalent meaning.
- [ ] `CHANGELOG.md` contains user-visible changes, fixes, security notes, and migrations under the release version.
- [ ] No old organization URLs, private paths, credentials, generated packages, or internal migration notes are present.
- [ ] Public code comments and commits are English; Markdown contains no emoji.

## 2. Version and Repository State

- [ ] The working tree is clean and the release commit is on `main`.
- [ ] `Cargo.toml`, both crate manifests, and `Cargo.lock` agree on the intended version and Rust 1.85 baseline.
- [ ] Repository, homepage, license, description, and README metadata resolve to `Tinkora/qr_forge`.
- [ ] The tag `vX.Y.Z` and GitHub release do not already exist.
- [ ] All required pull request reviews and branch protection checks are satisfied.

## 3. Rust and WebAssembly Verification

Run with the repository's pinned or declared toolchain, including Rust 1.85 for MSRV evidence:

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
rustup target add wasm32-unknown-unknown
cargo check -p qr_forge_core --target wasm32-unknown-unknown
cargo check -p qr_forge_web --target wasm32-unknown-unknown
wasm-pack build --target web crates/qr_forge_web -- --locked

cd crates/qr_forge_web
npm ci
npx playwright install chromium
npm run test:wasm-smoke:local
```

- [ ] Formatting passes.
- [ ] All workspace tests pass.
- [ ] Strict Clippy passes without broad lint suppression.
- [ ] Both crates compile for `wasm32-unknown-unknown`.
- [ ] `wasm-pack` produces the JavaScript and WebAssembly package without missing metadata warnings.
- [ ] The Playwright WASM smoke suite passes in all four configured viewports.
- [ ] Required CI, MSRV, dependency, license, security, and CodeQL checks pass on the exact release commit.
- [ ] Generated QR, Code 128, and EAN-13 PNG tests pass with the independent decoder.

## 4. Browser Verification

Build and serve the exact candidate:

```bash
mkdir -p crates/qr_forge_web/static/pkg
cp crates/qr_forge_web/pkg/* crates/qr_forge_web/static/pkg/
python3 -m http.server 8080 --directory crates/qr_forge_web/static
```

- [ ] The app loads with no console errors or warnings and the WASM request returns successfully.
- [ ] No unexpected external runtime request occurs and entered payloads are not transmitted.
- [ ] English is the default; the Simplified Chinese switch updates visible and accessible text.
- [ ] URL or text, Wi-Fi, vCard, phone, email, Code 128, and EAN-13 modes generate expected output.
- [ ] Wi-Fi significant whitespace and escaped delimiter cases are checked.
- [ ] EAN-13 invalid length and characters, Code 128 invalid characters and length, and empty QR input show useful errors.
- [ ] SVG and PNG downloads contain the latest generated payload.
- [ ] A valid PNG logo previews in PNG output, selects H correction, disables SVG download, and keeps PNG download available.
- [ ] Invalid logo type, size, and ratio paths fail without stale or misleading output.
- [ ] Password visibility, tab arrow keys, Home and End, focus order, labels, and status announcements work.
- [ ] At widths 375, 768, 1024, and 1440 pixels, no horizontal overflow, clipping, or incoherent overlap is present.
- [ ] Reduced-motion behavior and visible focus are preserved.
- [ ] Critical sample outputs scan with an independent real camera or scanner in addition to automated decoding.

## 5. Security and Supply Chain

- [ ] GitHub private vulnerability reporting is enabled and its link opens a private advisory form.
- [ ] No unresolved security advisory blocks release.
- [ ] Dependency review, `cargo audit`, and `cargo deny` or their repository workflow equivalents pass.
- [ ] GitHub Actions are pinned to full commit SHAs and use least-privilege permissions.
- [ ] Build and Pages workflows do not use `curl | sh`, untrusted pull request secrets, or mutable release inputs.
- [ ] Release artifacts include provenance, checksums, or attestations required by repository policy.
- [ ] The release commit has passed secret scanning and contains no test credentials or sensitive QR samples.

## 6. Publish

- [ ] Merge the release pull request into `main` only after all gates pass.
- [ ] Create the signed or repository-policy-compliant annotated tag `vX.Y.Z` from the verified commit.
- [ ] Let the release workflow build artifacts from the tag; do not upload unverified local artifacts.
- [ ] Publish the GitHub release with changelog-derived notes and any compatibility warning.
- [ ] Verify the Pages deployment is tied to the release commit or an explicitly recorded later commit.
- [ ] Confirm release assets, checksums or attestations, source links, and license files are downloadable.

## 7. Post-Release

- [ ] Open the public Pages URL in a clean browser session and generate and download one QR, one Code 128, and one EAN-13 output.
- [ ] Confirm README badges, documentation links, issue templates, Discussions, and security reporting links resolve.
- [ ] Confirm the release and deployment workflows are green on GitHub.
- [ ] Announce only capabilities present in the released contract.
- [ ] Create follow-up issues for deferred work; do not leave unowned release TODOs in source or public claims.

## Stop Conditions

Do not publish if required checks fail, scanner output is not reproducible, the privacy boundary is violated, a critical or high-severity vulnerability is unresolved, bilingual documents disagree materially, or the release cannot be rebuilt from the target commit.

If a faulty release is already public, mark it clearly in the GitHub release, stop or roll back Pages deployment when appropriate, publish a corrected patch version from a reviewed commit, and document the incident in the changelog or security advisory.
