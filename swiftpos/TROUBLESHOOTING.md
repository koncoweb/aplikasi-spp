# SwiftPOS Troubleshooting Guide

This document outlines the pitfalls, mistakes, and issues encountered during development, along with their solutions.

---

## Table of Contents

1. [Application Startup Crash](#1-application-startup-crash)
2. [Environment Variables Not Loading in Release Mode](#2-environment-variables-not-loading-in-release-mode)
3. [TailwindCSS v4 Compatibility Issues](#3-tailwindcss-v4-compatibility-issues)
4. [Login Form Icon Too Large](#4-login-form-icon-too-large)
5. [Redundant dotenv() Call](#5-redundant-dotenv-call)
6. [.env File Exposed in Executable](#6-env-file-exposed-in-executable)

---

## 1. Application Startup Crash

### Symptom
The application crashes immediately upon launching the production (release) build.

### Root Cause
The application was attempting to read environment variables from a `.env` file that was not bundled with the executable. In development mode, the `.env` file is present in the project root, but in release mode, the executable runs from a different directory.

### Solution
Added the `dotenv` crate to load environment variables at runtime from multiple possible locations:

```rust
// swiftpos/src-tauri/src/lib.rs

// Try multiple locations to support development and production
let mut env_paths = vec![
    std::path::PathBuf::from(".env"),
    std::path::PathBuf::from("../.env"),
    std::path::PathBuf::from("resources/.env"),
];

// Also try to find .env in the executable's directory
if let Ok(exe_path) = std::env::current_exe() {
    if let Some(exe_dir) = exe_path.parent() {
        env_paths.insert(0, exe_dir.join(".env"));
    }
}

// Load from custom paths first, then fallback to default
let mut loaded = false;
for env_path in &env_paths {
    if env_path.exists() {
        if dotenv::from_path(env_path).is_ok() {
            info!("Loaded environment variables from: {:?}", env_path);
            loaded = true;
            break;
        }
    }
}

if !loaded {
    dotenv().ok();
}
```

**Files Modified:**
- `swiftpos/src-tauri/Cargo.toml` - Added `dotenv = "0.15"` dependency
- `swiftpos/src-tauri/src/lib.rs` - Added dotenv loading logic
- `swiftpos/src-tauri/tauri.conf.json` - Bundled .env in executable

---

## 2. Environment Variables Not Loading in Release Mode

### Symptom
Database connection fails, authentication errors, or missing configuration in production builds.

### Root Cause
The `.env` file was not included in the Tauri bundle, causing the application to run without environment variables in production.

### Solution

**Option A: Bundle .env in executable (Quick Fix)**
```json
// tauri.conf.json
{
  "bundle": {
    "resources": {
      "../.env": ".env"
    }
  }
}
```

**Option B: Use system environment variables (Recommended for Production)**
Set environment variables at the system level rather than relying on a `.env` file:

```bash
# Windows
set DATABASE_URL=postgresql://...
set JWT_SECRET=your-secret-key

# Linux/macOS
export DATABASE_URL="postgresql://..."
export JWT_SECRET="your-secret-key"
```

**Option C: Use a config file (Alternative)**
Create a JSON/YAML configuration file that gets installed with the application.

---

## 3. TailwindCSS v4 Compatibility Issues

### Symptom
CSS styles not being applied correctly, visual glitches, or Tailwind directives not working.

### Root Cause
TailwindCSS v4 was installed but has different syntax and configuration requirements than v3. The project was using v3 directives that are incompatible.

### Solution
Downgraded to TailwindCSS v3.4.17:

```bash
npm uninstall tailwindcss postcss autoprefixer
npm install tailwindcss@3.4.17 postcss autoprefixer
```

**Configuration:**
```javascript
// tailwind.config.js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,js}",
    "./src/**/*.{html,js}",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

**Build CSS locally:**
```bash
npx tailwindcss -i ./src/styles.css -o ./src/output.css --minify
```

---

## 4. Login Form Icon Too Large

### Symptom
The login form displays an oversized icon in the center of the screen.

### Root Cause
The icon container and SVG were using larger sizes than intended:
- Container: `w-16 h-16` (64px)
- SVG: `w-8 h-8` (32px)

### Solution
Reduced the icon sizes in `index.html`:

```html
<!-- Before -->
<div class="inline-flex items-center justify-center w-16 h-16 bg-blue-500 rounded-full mb-4">
  <svg xmlns="http://www.w3.org/2000/svg" class="w-8 h-8 text-white" ...>

<!-- After -->
<div class="inline-flex items-center justify-center w-12 h-12 bg-blue-500 rounded-full mb-4">
  <svg xmlns="http://www.w3.org/2000/svg" class="w-6 h-6 text-white" ...>
```

**File Modified:** `swiftpos/src/index.html` (lines 63-64)

---

## 5. Redundant dotenv() Call

### Symptom
N/A (Code quality issue)

### Root Cause
After iterating through custom .env paths and potentially loading a file, the code unconditionally called `dotenv().ok()` again, which is redundant.

### Solution
Added a flag to track if .env was already loaded:

```rust
// swiftpos/src-tauri/src/lib.rs

let mut loaded = false;
for env_path in &env_paths {
    if env_path.exists() {
        if dotenv::from_path(env_path).is_ok() {
            info!("Loaded environment variables from: {:?}", env_path);
            loaded = true;
            break;
        }
    }
}

// Fallback to default location if no custom path worked
if !loaded {
    dotenv().ok();
}
```

**File Modified:** `swiftpos/src-tauri/src/lib.rs` (lines 104-118)

---

## 6. .env File Exposed in Executable

### Symptom
N/A (Security concern)

### Root Cause
The `.env` file containing sensitive credentials (database URL, JWT secret, API keys) was bundled directly into the executable. This exposes these secrets to anyone who extracts the executable.

### Risk
- Database credentials exposed
- JWT secret exposed  
- API keys for Stack Auth exposed
- Potential unauthorized access to backend services

### Solution (To Be Implemented)

**Option A: Use System Environment Variables (Recommended)**
```json
// tauri.conf.json - Remove the resources section
"bundle": {
  // Remove "resources" entry
}
```

Then set environment variables at deployment time.

**Option B: Use a Secrets Management Service**
- HashiCorp Vault
- AWS Secrets Manager
- Azure Key Vault
- Google Cloud Secret Manager

**Option C: Encrypted Configuration File**
- Store an encrypted config file with the application
- Decrypt at runtime using a key derived from machine-specific information
- Or prompt user for decryption password on first launch

**Option D: Build-Time Variable Injection**
- Use CI/CD pipelines to inject secrets during build
- Store secrets in CI/CD environment variables
- Not bundled in final executable

---

## Summary of Changes

| Issue | File(s) Modified | Solution |
|-------|------------------|----------|
| Startup crash | `Cargo.toml`, `lib.rs`, `tauri.conf.json` | Added dotenv crate + bundled .env |
| Env vars not loading | `tauri.conf.json` | Bundled .env in executable |
| TailwindCSS v4 issues | `package.json`, `tailwind.config.js` | Downgraded to v3.4.17 |
| Large login icon | `index.html` | Reduced icon size |
| Redundant dotenv call | `lib.rs` | Added loaded flag |
| .env exposed | `tauri.conf.json` | To be addressed separately |

---

## Best Practices for Future Development

1. **Always test release builds** - Don't rely solely on development mode testing
2. **Use environment-specific configs** - Different configs for dev/staging/prod
3. **Never commit secrets** - Use `.gitignore` and secret management tools
4. **Test CSS changes locally** - Build CSS before testing
5. **Use semantic versioning** - Track changes systematically
6. **Document configuration requirements** - Clear instructions for deployment
