# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Created `TROUBLESHOOTING.md` with comprehensive debugging guide
- Created `CHANGELOG.md` for tracking fixes and updates

---

## [1.0.1] - 2026-03-18

### Fixed
- **Application Startup Crash**: Added `dotenv` crate to load environment variables from multiple locations (executable directory, project root, resources)
- **Environment Variables Not Loading**: Bundled `.env` file in executable using Tauri resources configuration
- **Large Login Icon**: Reduced icon container from `w-16 h-16` (64px) to `w-12 h-12` (48px), SVG from `w-8 h-8` to `w-6 h-6`
- **Redundant dotenv() Call**: Added `loaded` flag to prevent redundant fallback calls after successful .env loading

### Changed
- **TailwindCSS**: Downgraded from v4 to v3.4.17 for stable compatibility
- **CSS Build**: Switched from CDN to local build (`output.css`)
- **Tauri Config**: Removed `$schema` to avoid network validation errors

### Security
- **Known Issue**: `.env` file containing credentials is bundled in executable (to be addressed separately)

### Files Changed
```
swiftpos/.env                      # Updated with actual credentials
swiftpos/package-lock.json         # Updated dependencies
swiftpos/package.json              # Added tailwindcss v3.4.17
swiftpos/src-tauri/Cargo.lock      # Updated
swiftpos/src-tauri/Cargo.toml      # Added dotenv = "0.15"
swiftpos/src-tauri/src/lib.rs      # Added dotenv loading logic
swiftpos/src-tauri/tauri.conf.json # Added resources, removed schema
swiftpos/src/index.html            # Reduced icon size, switched to local CSS
swiftpos/src/output.css            # Regenerated with TailwindCSS v3
swiftpos/src/styles.css            # Updated with v3 directives
swiftpos/tailwind.config.js        # Created for v3
```

---

## [1.0.0] - 2026-03-?? (Initial Release)

### Added
- Initial SwiftPOS Tauri application with Rust backend
- Database integration (NeonDB/PostgreSQL)
- Authentication system (Stack Auth)
- Frontend with TailwindCSS styling
- POS features: Branches, Products, Categories, Transactions, Tenants

---

## How to Update This Changelog

When implementing fixes or updates:

1. Add entries under `[Unreleased]` section
2. Include:
   - Date of change
   - Description of fix/update
   - Files changed (can be generated with `git diff --stat`)
3. When releasing a new version, move `[Unreleased]` changes to a new version section

### Example Entry Format

```markdown
### Fixed
- **Issue Description**: Brief explanation of what was fixed (#issue-number if applicable)

### Changed
- **Component**: Description of what changed

### Added
- **Feature**: Description of new feature

### Files Changed
```
path/to/file.ext     # Description of change
```
```

---

## Version History

| Version | Date | Status |
|---------|------|--------|
| 1.0.1 | 2026-03-18 | Current |
| 1.0.0 | 2026-03-?? | Initial |
