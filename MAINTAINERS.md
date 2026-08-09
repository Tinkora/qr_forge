# Maintainers

## Project Ownership

qr_forge is stewarded by the [Tinkora organization](https://github.com/Tinkora). Maintainers are responsible for protecting the product contract, privacy boundary, release integrity, and an open contribution process.

## Current Maintainers

| Maintainer | Role | Responsibilities |
| --- | --- | --- |
| [@tinkeragora](https://github.com/tinkeragora) | Lead maintainer | Triage, review, releases, repository settings, and private security reports |

The project currently has a single active maintainer. This is a continuity risk, not a reason to lower review or release gates. The project welcomes sustained contributors who want to take on documented ownership.

## Maintainer Responsibilities

- Keep shipped behavior, tests, the product contract, and bilingual documentation aligned.
- Triage issues and discussions without promising unsupported timelines.
- Review pull requests for correctness, privacy, accessibility, compatibility, and maintainability.
- Ensure required automation passes before merge and release.
- Keep repository and organization permissions at least privilege.
- Coordinate vulnerabilities through GitHub private security advisories.
- Record product-scope and compatibility decisions in public issues or discussions.
- Avoid presenting proposed integrations as released capabilities.

## Decision Process

Routine fixes and documentation improvements are decided through pull request review. Changes to product scope, public Rust or WASM interfaces, privacy behavior, dependencies with material supply-chain impact, or release policy require a linked GitHub Discussion or issue that records:

1. The user problem and supporting evidence
2. Alternatives and maintenance cost
3. Privacy, security, and compatibility consequences
4. Test and release plan
5. The final maintainer decision

Seek consensus among active maintainers and affected contributors. If consensus is not possible, the lead maintainer makes the decision and records the rationale publicly.

## Review and Merge

- Authors should not approve their own pull requests.
- When a second maintainer or qualified reviewer is available, at least one independent approval is expected for code, workflow, dependency, security, or release changes.
- While the project has one maintainer, that maintainer may merge after all required automated checks pass and the pull request contains reproducible verification evidence.
- Security-sensitive changes may remain private until coordinated disclosure.
- Merge methods and commit messages must preserve English Conventional Commit history.

## Becoming a Maintainer

Maintainer access is earned through sustained contributions that demonstrate sound technical judgment, respectful review, reliable follow-through, and familiarity with the product contract and security model. A nomination should be recorded in a public issue and approved by current organization owners.

Access should be granted incrementally. Triage or review responsibility should precede repository administration and release authority.

## Inactivity and Succession

A maintainer who expects to be unavailable should identify open responsibilities and transfer security or release work privately when needed. Maintainer status may be moved to emeritus after six months without project activity, following public notice when account safety permits.

Organization ownership, package publishing, Pages, and release credentials must never depend on an undocumented personal token. GitHub teams, repository roles, environments, and audit logs should be the source of authority.

## Contact

Use [GitHub Discussions](https://github.com/Tinkora/qr_forge/discussions) for governance questions. Use [GitHub private vulnerability reporting](https://github.com/Tinkora/qr_forge/security/advisories/new) for security matters.
