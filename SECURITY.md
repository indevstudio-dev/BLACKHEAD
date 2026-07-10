# Security Policy

BLACKHEAD is pre-release privileged software. Do not deploy this scaffold on a production device.

- Keep SELinux enforcing.
- Never execute or install an unverified artifact.
- Never expose the daemon socket or BLACKHEAD private state to containers.
- Keep parsers and network-facing work outside the privileged supervisor.
- Remove secrets and device identifiers from public reports.
