# Security Policy

## Supported Versions

Support is per package. Only the latest release line of each package on `main` receives security fixes.

| Package | Path | Version | Supported |
| ------- | ---- | ------- | --------- |
| Stellar/Soroban contract (`kora-stellar`) | `stellar-contracts/` | 0.1.x | :white_check_mark: |
| Celo/Solidity contracts (`KoraRegistry`) | `celo-contracts/` | 1.0.x | :white_check_mark: |

Reporting scope: vulnerabilities in the contract source of the packages above at the revision on `main`. Deployed contract revisions are supported only if they match a supported package version; maintainers must confirm the deployed revision (commit hash) before triage. The companion `kora-2fa` backend is out of scope for this repository (see `docs/environment-variables.md`).

## Reporting a Vulnerability

If you discover a security vulnerability, please report it privately:

1. **DO NOT** create a public GitHub issue
2. Email: [security contact needed]
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Response Timeline

- **24 hours**: Initial response acknowledging receipt
- **72 hours**: Initial assessment and severity classification
- **7 days**: Detailed response with fix timeline

## Security Best Practices

When contributing:
- Always validate inputs
- Use proper authentication checks
- Follow principle of least privilege
- Test edge cases and error conditions
- Review code for potential vulnerabilities

## Encryption Key Derivation

Sensitive pet and owner fields use a deterministic key derived at runtime from:
- a fixed domain separator (`kora:encryption-key:v1`)
- the current contract address
- admin context (legacy single admin or first multisig admin when configured)

This replaces the previous static all-zero key. As a result:
- encrypted storage no longer mirrors plaintext by default
- key material is contract-scoped and not hardcoded
- rotating admin configuration can change derived key context, so migrations/rollouts should account for backward compatibility expectations

## Audit Record

| Field    | Value |
|----------|-------|
| **Date** | 2026-05-29 |
| **Scope** | `stellar-contracts/src/lib.rs` |
| **Auditor** | Internal |
| **Result** | Internal review reported all 23 checklist items passing. Fixed: temp vet access expiry now enforced on all read paths via `check_and_expire_access`; scope escalation prevented in `grant_temp_vet_access` with explicit `ScopeEscalation` error. |

**Evidence and limitations:** The 23-item checklist is not versioned in this repository and the audited git revision was not recorded, so this result cannot be independently reproduced and is not an external audit. Reproducible checks available today (stellar contract only; the Celo package was not covered):

```bash
cd stellar-contracts && cargo test
```

These tests cover the two fixes above but do not prove the unpublished checklist. Publishing the checklist at a pinned commit is tracked as follow-up.
