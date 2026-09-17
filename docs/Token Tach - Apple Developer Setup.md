# Apple Developer Setup — Token Tach

## Identity
- **Organization:** NoctuSoft, Inc.
- **Team ID:** N42FM5L5KD
- **Apple ID:** apple@darkware.net
- **Signing Identity:** `Developer ID Application: NoctuSoft, Inc. (N42FM5L5KD)`
- **Certificate Hash:** `131C7A353DE14CC7984D64770D733A807F5D6F7F`

## GitHub Secrets (YOLOVibeCode/token-tach)

All 6 secrets are configured as of 2026-05-15:

| Secret | Value / Notes |
|---|---|
| `APPLE_CERTIFICATE` | Base64 of .p12 export (all identities from login keychain) |
| `APPLE_CERTIFICATE_PASSWORD` | `TokenTach2026!` |
| `APPLE_SIGNING_IDENTITY` | `Developer ID Application: NoctuSoft, Inc. (N42FM5L5KD)` |
| `APPLE_TEAM_ID` | `N42FM5L5KD` |
| `APPLE_ID` | `apple@darkware.net` |
| `APPLE_PASSWORD` | App-specific password: `rkqh-gfxx-zznb-fphf` |

## How It Works

### Code Signing (automatic on release)
1. `release.yml` imports the .p12 certificate into a temporary keychain on the GitHub Actions runner
2. Tauri reads `APPLE_SIGNING_IDENTITY` and signs the .app bundle
3. The signed app is bundled into .dmg and .app.tar.gz

### Notarization (automatic on release)
1. Tauri submits the signed app to Apple's notarization service
2. Uses `APPLE_ID` + `APPLE_PASSWORD` (app-specific) + `APPLE_TEAM_ID`
3. Apple scans the binary and returns a notarization ticket
4. The ticket is stapled to the .dmg so users don't get Gatekeeper warnings

## Regenerating Credentials

### If the certificate expires
1. Go to https://developer.apple.com/account/resources/certificates
2. Create a new **Developer ID Application** certificate
3. Export as .p12 from Keychain Access
4. Base64 and update `APPLE_CERTIFICATE` secret:
   ```bash
   base64 -i new-cert.p12 | gh secret set APPLE_CERTIFICATE --repo YOLOVibeCode/token-tach
   gh secret set APPLE_CERTIFICATE_PASSWORD --repo YOLOVibeCode/token-tach --body "new-password"
   ```

### If the app-specific password is revoked
1. Go to https://account.apple.com > Sign-In and Security > App-Specific Passwords
2. Generate a new one (name: "Token Tach Notarize")
3. Update the secret:
   ```bash
   gh secret set APPLE_PASSWORD --repo YOLOVibeCode/token-tach --body "new-password"
   ```

## Important Notes
- The Apple ID for this project is **apple@darkware.net** (NoctuSoft) — NOT ambientconsulting
- The app-specific password is NOT your Apple ID password — it's a separate credential
- The .p12 export includes all identities from the login keychain; only the Developer ID Application cert is used for signing
