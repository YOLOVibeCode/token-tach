# Release Pipeline — Token Tach

Mirrored from the termgrid project. Fully automated via Conventional Commits + release-please + GitHub Actions.

## Pipeline Flow

```
merge feat/fix PR to main
        │
        ▼
release-please opens "chore(main): release vX.Y.Z" PR
        │
        ▼
merge that PR (squash, default message)
        │
        ▼
release-please pushes tag vX.Y.Z
        │
        ▼
release.yml fires → 4-platform matrix build
        │
        ▼
artifacts uploaded as draft GitHub Release
        │
        ▼
manually publish the draft Release
```

## GitHub Actions Workflows

### ci.yml (every push/PR to main)
- **Frontend:** pnpm install, tsc --noEmit, vitest
- **Rust:** cargo fmt --check, cargo clippy -D warnings, cargo test --lib
- **Build smoke:** pnpm tauri build --debug --no-bundle on ubuntu, macos, windows

### release-please.yml (push to main)
- googleapis/release-please-action v5
- Reads conventional commit titles → determines version bump
- Auto-updates version in: `package.json`, `Cargo.toml`, `tauri.conf.json`

### release.yml (tag push v*)
- 4-platform matrix:
  - macOS Apple Silicon (aarch64-apple-darwin) → .app.tar.gz, .dmg
  - macOS Intel (x86_64-apple-darwin) → .app.tar.gz, .dmg
  - Linux x86_64 → .deb, .rpm, .AppImage
  - Windows x86_64 → .msi, .nsis
- Apple code signing + notarization on macOS builds
- Uploads all artifacts to GitHub Release

## Conventional Commit Prefixes

| Prefix | Bumps | Changelog Section |
|---|---|---|
| `feat:` | minor | Features |
| `fix:` | patch | Bug Fixes |
| `perf:` | patch | Performance |
| `refactor:` | patch | Refactor |
| `docs:` | none | Docs |
| `test:` | none | hidden |
| `chore:` / `build:` / `ci:` | none | hidden |
| `feat!:` / `BREAKING CHANGE:` | major | Features |

## Pre-flight Check

```bash
pnpm preflight
# Runs: typecheck → vitest → cargo fmt --check → cargo clippy → cargo test
```

## Version Sync

release-please keeps these 3 files in sync:
- `package.json` → `version`
- `src-tauri/Cargo.toml` → `package.version`
- `src-tauri/tauri.conf.json` → `version`

Never edit versions manually — release-please owns them.

## Config Files
- `release-please-config.json` — release type, changelog sections, extra-files
- `.release-please-manifest.json` — current version tracker (`{ ".": "0.0.1" }`)
