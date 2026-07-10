// Package internal contains non-privileged OCI distribution, validation, and unpacking components.
// It must not perform mount, setns, pivot_root, or cgroup writes directly.
package internal
