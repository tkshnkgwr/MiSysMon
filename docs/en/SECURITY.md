**English** | [日本語版](../ja/SECURITY.md)

# Security Policy (SECURITY.md) - MiSysMon

This document outlines the security policies, vulnerability reporting processes, and support models for the `MiSysMon` project.

---

## 1. Security Architecture & Safety

`MiSysMon` enforces high security and reliability through the following design rules:

1. **Memory Safety**:
   - Utilizing Rust's robust ownership system and compile-time type safety to eliminate class-wide vulnerabilities like buffer overflows, null-pointer dereferences, and use-after-free bugs.
2. **Privilege & Network Boundaries**:
   - Operates entirely locally. While administrative elevation might be suggested for AMD CPU thermal telemetry on Windows, the application performs no network operations, leaving no exposure to remote network threats.
3. **Automated Dependency Auditing**:
   - We utilize Dependabot to audit dependencies (like `sysinfo` and `eframe`) weekly, ensuring patches for downstream vulnerabilities are integrated promptly.

---

## 2. Supported Versions

Security updates are provided for the following versions:

| Version | Support Status |
| :--- | :---: |
| Latest Release (`v1.x.x`) | ✅ Supported |
| Legacy Releases | ❌ Unsupported |

---

## 3. Reporting Vulnerabilities

If you discover a potential security vulnerability in `MiSysMon`, please do not open a public issue. Instead, report it using the following steps:

1. **Contact Point**:
   - Directly contact the repository maintainers or email the dedicated security reporting address.
2. **Information to Include**:
   - Affected `MiSysMon` version and Windows environment.
   - Detailed description and steps to reproduce (PoC codes or command sequences).
3. **Remediation Process**:
   - We will acknowledge receipt of the report within 3 days, build and verify hotfixes, and publish a security patch release as quickly as possible.
