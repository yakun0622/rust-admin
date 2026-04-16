#!/usr/bin/env bash
set -euo pipefail

BASE_URL="${BASE_URL:-http://127.0.0.1:8080}"
ADMIN_USERNAME="${ADMIN_USERNAME:-admin}"
ADMIN_PASSWORD="${ADMIN_PASSWORD:-admin123456}"

if ! command -v curl >/dev/null 2>&1; then
  echo "curl is required"
  exit 1
fi

green() {
  printf "\033[32m%s\033[0m\n" "$1"
}

red() {
  printf "\033[31m%s\033[0m\n" "$1"
}

assert_contains() {
  local text="$1"
  local needle="$2"
  local step="$3"
  if [[ "$text" != *"$needle"* ]]; then
    red "✗ ${step} failed"
    echo "response: $text"
    exit 1
  fi
  green "✓ ${step}"
}

assert_not_contains() {
  local text="$1"
  local needle="$2"
  local step="$3"
  if [[ "$text" == *"$needle"* ]]; then
    red "✗ ${step} failed"
    echo "response: $text"
    exit 1
  fi
  green "✓ ${step}"
}

extract_numeric_id() {
  local text="$1"
  printf '%s' "$text" | sed -n 's/.*"id":\([0-9][0-9]*\).*/\1/p' | head -n 1
}

assert_id() {
  local id="$1"
  local step="$2"
  local resp="$3"
  if [[ -z "${id}" ]]; then
    red "✗ ${step} failed"
    echo "response: $resp"
    exit 1
  fi
  green "✓ ${step} ${id}"
}

extract_token() {
  local text="$1"
  local token
  token="$(printf '%s' "$text" | sed -n 's/.*"access_token":"\([^"]*\)".*/\1/p')"
  if [[ -z "${token}" ]]; then
    token="$(printf '%s' "$text" | sed -n 's/.*"token":"\([^"]*\)".*/\1/p')"
  fi
  printf '%s' "${token}"
}

login() {
  local username="$1"
  local password="$2"
  curl -sS "${BASE_URL}/api/system/auth/login" \
    -H "Content-Type: application/json" \
    -d "{\"username\":\"${username}\",\"password\":\"${password}\"}"
}

auth_get() {
  local token="$1"
  local path="$2"
  curl -sS "${BASE_URL}${path}" -H "Authorization: Bearer ${token}"
}

auth_post() {
  local token="$1"
  local path="$2"
  local body="$3"
  curl -sS "${BASE_URL}${path}" \
    -H "Authorization: Bearer ${token}" \
    -H "Content-Type: application/json" \
    -d "${body}"
}

auth_delete() {
  local token="$1"
  local path="$2"
  curl -sS -X DELETE "${BASE_URL}${path}" -H "Authorization: Bearer ${token}"
}

echo "BASE_URL=${BASE_URL}"

health_resp="$(curl -sS "${BASE_URL}/health")"
assert_contains "$health_resp" "\"status\"" "health"

admin_login_resp="$(login "${ADMIN_USERNAME}" "${ADMIN_PASSWORD}")"
assert_contains "$admin_login_resp" "\"code\":200" "admin login"

admin_token="$(extract_token "$admin_login_resp")"
if [[ -z "${admin_token}" ]]; then
  red "✗ admin token parse failed"
  echo "response: $admin_login_resp"
  exit 1
fi
green "✓ admin token parsed"

admin_profile_resp="$(auth_get "${admin_token}" "/api/system/auth/profile")"
assert_contains "$admin_profile_resp" "\"code\":200" "profile by admin"
assert_contains "$admin_profile_resp" "\"*:*:*\"" "super admin wildcard permission"
assert_contains "$admin_profile_resp" "\"system:user:view\"" "explicit permission exists"

admin_user_list_resp="$(auth_get "${admin_token}" "/api/system/user")"
assert_contains "$admin_user_list_resp" "\"code\":200" "authorized access by admin"

suffix="$(date +%s)"
no_perm_username="perm_norole_${suffix}"

create_no_perm_user_resp="$(auth_post "${admin_token}" "/api/system/user" "{\"username\":\"${no_perm_username}\",\"nickname\":\"无权用户${suffix}\",\"phone\":\"13800139000\",\"status\":\"enabled\"}")"
assert_contains "$create_no_perm_user_resp" "\"code\":200" "create no-permission user"

no_perm_user_id="$(extract_numeric_id "$create_no_perm_user_resp")"
assert_id "$no_perm_user_id" "parse no-permission user id" "$create_no_perm_user_resp"

no_perm_login_resp="$(login "${no_perm_username}" "${ADMIN_PASSWORD}")"
assert_contains "$no_perm_login_resp" "\"code\":200" "no-permission user login"

no_perm_token="$(extract_token "$no_perm_login_resp")"
if [[ -z "${no_perm_token}" ]]; then
  red "✗ no-permission token parse failed"
  echo "response: $no_perm_login_resp"
  exit 1
fi
green "✓ no-permission user token parsed"

no_perm_profile_resp="$(auth_get "${no_perm_token}" "/api/system/auth/profile")"
assert_contains "$no_perm_profile_resp" "\"code\":200" "profile by no-permission user"
assert_not_contains "$no_perm_profile_resp" "\"*:*:*\"" "no wildcard permission for no-permission user"

forbidden_read_resp="$(auth_get "${no_perm_token}" "/api/system/user")"
assert_contains "$forbidden_read_resp" "\"code\":403" "forbidden read for no-permission user"

forbidden_write_resp="$(auth_post "${no_perm_token}" "/api/system/config" "{\"name\":\"perm-block-${suffix}\",\"value\":\"v1\",\"status\":\"enabled\"}")"
assert_contains "$forbidden_write_resp" "\"code\":403" "forbidden write for no-permission user"

cleanup_resp="$(auth_delete "${admin_token}" "/api/system/user/${no_perm_user_id}")"
assert_contains "$cleanup_resp" "\"code\":200" "cleanup no-permission user"

green "Permission regression checks passed."
