# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.6.x   | :white_check_mark: |
| < 0.6   | :x:                |

## Reporting a Vulnerability

We take the security of Universal Android Debloater seriously. If you believe
you have found a security vulnerability, please report it to us as described below.

**Please do NOT report security vulnerabilities through public GitHub issues.**

### How to Report

1. **Email**: Send a detailed description to the project maintainers via
   [GitHub Security Advisories](https://github.com/0x192/universal-android-debloater/security/advisories/new)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 1 week
- **Fix Timeline**: Depends on severity
  - Critical: Within 1 week
  - High: Within 2 weeks
  - Medium: Within 1 month
  - Low: Next release cycle

### Disclosure Policy

- We will confirm receipt of your report
- We will investigate and keep you informed of progress
- We will coordinate public disclosure with you
- We will credit you in the release notes (unless you prefer to remain anonymous)

## Security Considerations

### ADB Access

UAD communicates with Android devices via ADB. Consider:

- ADB commands are executed on connected devices
- The application does not require root access
- System package modifications require USB debugging to be enabled
- Users should verify device identity before executing commands

### Network Communication

- Debloat lists are fetched from GitHub (`raw.githubusercontent.com`)
- Self-update checks query the GitHub API
- No personal data is transmitted
- No telemetry or analytics are collected

### Local Data

- Configuration is stored in OS-specific config directories
- Logs are stored in OS-specific cache directories
- Backup files contain package state information only
- No sensitive credentials are stored

### Supply Chain

- Dependencies are pinned via `Cargo.lock`
- The `iced` GUI library is sourced from git (main branch)
- All dependencies are audited via `cargo audit` in CI

## Best Practices for Users

1. **Always backup** your device data before using UAD
2. **Read the FAQ** before removing packages
3. **Start with Recommended** removals only
4. **Verify device identity** before executing commands
5. **Keep UAD updated** to benefit from the latest debloat lists

## Security Audit History

| Date | Scope | Result |
|------|-------|--------|
| N/A  | N/A   | No formal audit conducted |
