#!/system/bin/sh
bh_enter_safe_mode() {
  echo "BLACKHEAD: safe-mode hook reached" >&2
  return 0
}
