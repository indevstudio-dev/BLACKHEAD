# BLACKHEAD (BH)

BLACKHEAD is a systemless, OCI-oriented container runtime and node-management platform for rooted Android devices, conventional Linux systems, and a future CIAOS backend.

> **Status:** Early engineering scaffold. No production runtime, APK, Magisk payload, release signature, or cloud service is shipped from this repository yet.

## Active direction

The first milestone is BH Core: one dependable OCI container on one rooted Android device, controlled locally through the Android application or `bhctl` over the authenticated Blackhead Control Protocol (BHCP).

Active areas:

- Rust daemon, supervisor, runtime contracts, policy, storage, IPC, telemetry, networking, and shim crates.
- Go OCI image acquisition and validation service.
- Kotlin and Jetpack Compose Android control application.
- Magisk packaging and boot integration.
- Rooted Android/Linux backend and CIAOS core adapter boundary.
- Protobuf schemas, conformance tests, security tests, release tooling, and documentation.

Deferred systems live under `future/` as documentation-only skeletons and are excluded from production builds.

## Repository safety

This initial repository structure deliberately fails closed. Placeholder scripts do not install or start privileged components, and no fake APK, signature, or native runtime binary is committed. Release artifacts must be produced by the signed build pipeline.

## Ownership

- **Project Owner / Lead Architect:** Infexjay
- **Organization:** INDEVSTUDIO Inc.

## Development status

The repository is being initialized from the active BLACKHEAD technical specification. Interfaces, persistent formats, security boundaries, and release behavior remain subject to Architecture Decision Records until the first compatibility baseline is declared.
