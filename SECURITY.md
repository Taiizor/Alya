# Security Policy

## Supported Versions

Security fixes and vulnerabilities are actively monitored and supported for the following versions of Alya:

| Version | Supported          |
| ------- | ------------------ |
| 0.0.x   | :white_check_mark: |

---

## Reporting a Vulnerability

The Alya team takes security bugs seriously. If you discover a vulnerability or security issue in the compiler or runtime:

1. **Do not create a public GitHub issue.**
2. Please disclose the vulnerability responsibly through [GitHub Security Advisories](https://github.com/alya-lang/alya/security/advisories/new).
3. If GitHub Advisories are not accessible, email details to [taiizor@vegalya.com](mailto:taiizor@vegalya.com).

### What to Include in Your Report

To help us triage and fix the vulnerability as fast as possible, please provide:
- A description of the issue and potential impact (e.g. buffer overflow, memory unsafety in runtime assembly, arbitrary execution).
- A minimal reproducible example (`.alya` file).
- The exact compiler flags and target architecture/OS.
- Any suggestions or fixes if you have them.

### Response Timeline

- **Acknowledgment:** Within 48 hours.
- **Assessment & Triage:** Within 5 business days.
- **Fix & Disclosure:** We will work with you to release a patch and publish a credit in the release notes.
