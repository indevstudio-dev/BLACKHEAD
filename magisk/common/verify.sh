#!/system/bin/sh
bh_verify_payload() {
  echo "BLACKHEAD: no signed payload is present" >&2
  return 1
}
