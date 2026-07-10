# ADR 0001: Foundation contracts

- Status: Proposed
- Date: 2026-07-10
- Owners: BLACKHEAD maintainers

## Context

BLACKHEAD needs stable domain contracts before privileged Linux, Android, CIAOS, image, storage, and IPC implementations can evolve independently. The first implementation milestone therefore defines error identifiers, operation identifiers, runtime capability reporting, backend contracts, and container lifecycle transitions.

## Decision

1. Public errors use stable machine-readable codes and safe user-facing messages.
2. Every mutating operation carries an opaque `OperationId` supplied by the caller or generated at the API edge.
3. Runtime capability discovery is explicit and reports supported, degraded, or unavailable states with reasons.
4. Runtime backends implement one semantic trait; Linux and CIAOS adapters translate that contract into platform operations.
5. Container lifecycle transitions are centralized in `bh-core` and reject illegal transitions before reaching a privileged backend.
6. Identifiers validate length and character constraints at construction boundaries.
7. Foundation crates forbid unsafe Rust.

## Security impact

The contracts fail closed. Unsupported isolation features cannot be silently treated as supported, invalid transitions are rejected before privileged work, and user-facing errors do not expose sensitive internal details.

## Compatibility and migration

These contracts are pre-1.0 and may change through reviewed ADRs. Once persisted formats or BHCP fields depend on them, compatibility tests and migration rules become mandatory.

## Alternatives considered

- Platform-specific lifecycle logic: rejected because it would create inconsistent behavior.
- Boolean capability flags: rejected because they cannot explain degraded or unavailable states.
- Free-form string errors: rejected because clients cannot reliably classify them.

## Consequences

Backend implementations must translate native failures into the shared error registry and must expose capability evidence before accepting workloads.

## Validation

Unit tests cover identifier validation, error classification, capability states, and every legal or illegal lifecycle transition.
