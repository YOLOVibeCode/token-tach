# Releasing Token Tach

The release pipeline is **automated end-to-end** once secrets are configured. You merge PRs; release-please proposes a version bump; you merge that; CI builds, signs, notarizes, and publishes.

> **TL;DR for the impatient maintainer**
>
> 1. Land work as PRs whose **titles follow Conventional Commits** (`feat: …`, `fix: …`).
> 2. release-please opens a `chore(main): release vX.Y.Z` PR — review, then merge it.
> 3. Wait ~20 min for the matrix build to finish; **publish** the draft Release on GitHub.
> 4. Done. Existing installs auto-update on next launch (once the updater is enabled).

---

## The four release rules

### Rule 1 — Every commit on `main` MUST be a Conventional Commit

| Prefix | Meaning | Bumps |
|---|---|---|
| `feat:` | new user-visible capability | **minor** |
| `fix:` | bug fix | **patch** |
| `perf:` | perf improvement | patch |
| `refactor:` | internal restructure, no behavior change | patch |
| `docs:` | docs only | none |
| `test:` | tests only | none |
| `chore:` / `build:` / `ci:` / `style:` | tooling / infra / formatting | none |
| `feat!:` or any with `BREAKING CHANGE:` footer | breaking change | **major** |

**Why:** `release-please` reads commit titles to decide the next version and write the changelog.

### Rule 2 — Run `pnpm preflight` before pushing

```bash
pnpm preflight   # mirrors CI: typecheck + vitest + cargo fmt/clippy/test
```

### Rule 3 — Never push directly to `main`

Branch protection blocks force-pushes and requires status checks. Open a PR. Squash-merge.

### Rule 4 — Don't manually edit `package.json` / `Cargo.toml` / `tauri.conf.json` versions

release-please owns the version field in all three.

---

## Pipeline overview

```
   merge feat/fix/perf PR              tag pushed (auto by release-please)
            │                                       │
            ▼                                       ▼
   release-please opens / updates           ┌──────────────────────────┐
   "chore(main): release X.Y.Z" PR          │ release.yml workflow     │
            │                               │  matrix:                 │
            ▼                               │   • macOS arm64 + x64    │
        merge that PR ─────────────────►    │   • Linux x86_64         │
                                            │   • Windows x86_64       │
                                            │  per platform:           │
                                            │   • tauri build          │
                                            │   • sign + notarize      │
                                            │   • upload to Release    │
                                            └──────────────────────────┘
                                                       │
                                                       ▼
                                            publish the draft Release
```

---

## Per-release runbook

### Pre-flight

- [ ] Local `main` is clean: `git status` shows nothing.
- [ ] Local `main` is current: `git pull`.
- [ ] CI on the latest `main` commit is green: `gh run list --workflow=ci.yml --limit 1`.
- [ ] `pnpm preflight` passes locally.
- [ ] Smoke-launch the app: `pnpm tauri dev`.

### Cut

- [ ] Open the release-please PR: `gh pr list --label "autorelease: pending"`.
- [ ] Read the proposed `CHANGELOG.md` diff.
- [ ] Merge the release-please PR (squash, default message).
- [ ] release-please pushes tag `vX.Y.Z`.
- [ ] `release.yml` workflow fires. Watch: `gh run watch --workflow=release.yml`.

### Publish

- [ ] After ~20 min, the matrix build finishes. A **draft Release** appears.
- [ ] Inspect the artifacts (4 platforms).
- [ ] Click **Publish release**.

---

## One-time setup

### 1. Apple Developer (~$99/yr)

See termgrid RELEASING.md for full Apple signing instructions. Add to GitHub Secrets:
`APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`.

### 2. Windows Authenticode (optional)

Add `WINDOWS_CERTIFICATE` and `WINDOWS_CERTIFICATE_PASSWORD` to GitHub Secrets.

### 3. Tauri auto-updater

The updater is **removed for v0.0.x** to keep builds clean. Re-add when ready:

```bash
pnpm tauri signer generate -w ~/.tauri/token-tach.key
# Add TAURI_SIGNING_PRIVATE_KEY + TAURI_SIGNING_PRIVATE_KEY_PASSWORD to Secrets
cd src-tauri && cargo add tauri-plugin-updater tauri-plugin-process
cd .. && pnpm add @tauri-apps/plugin-updater @tauri-apps/plugin-process
# Register plugins in lib.rs, add permissions, configure tauri.conf.json
```

---

## Tags are immutable

Once pushed, never force-move a tag. If a release needs a fix, bump the patch version.
