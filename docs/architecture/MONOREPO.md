# Monorepo dependency rules

- Platform crates depend inward on `bh-runtime-api`.
- Core code never depends on Android or CIAOS adapters.
- Kotlin communicates through BHCP and does not contain privileged runtime logic.
- Go services never perform mounts, `setns`, `pivot_root`, or cgroup writes directly.
- Generated schemas are never edited manually.
- Future packages are excluded from production builds and cannot import node-private packages.
