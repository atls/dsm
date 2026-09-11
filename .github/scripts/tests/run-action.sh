#!/usr/bin/env bash

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
action_script="${repository_root}/.github/scripts/run-action.sh"
temporary_directory="$(mktemp -d)"
trap 'rm -rf "${temporary_directory}"' EXIT

fail() {
  printf '%s\n' "$1" >&2
  exit 1
}

assert_contains() {
  local expected="$1"
  local file="$2"

  grep -F -- "${expected}" "${file}" >/dev/null || fail "missing '${expected}' in ${file}"
}

fixture_directory="${temporary_directory}/fixture"
fake_bin="${temporary_directory}/bin"
runner_temp="${temporary_directory}/runner"
mkdir -p "${fixture_directory}" "${fake_bin}" "${runner_temp}"

cat >"${fixture_directory}/dsm-launcher" <<'LAUNCHER'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' \
  "token=${GITHUB_TOKEN}" \
  "owner=${GITHUB_REPO_OWNER}" \
  "repository=${GITHUB_REPO_NAME}" \
  "team=${DSM_TEAM_SLUG}" \
  "template=${DSM_TEMPLATE_PATH}" \
  "timezone=${DSM_TIMEZONE}" >"${TEST_OUTPUT}"
LAUNCHER
chmod +x "${fixture_directory}/dsm-launcher"
tar -czf "${temporary_directory}/dsm-launcher-x86_64-unknown-linux-musl.tar.gz" \
  -C "${fixture_directory}" dsm-launcher

cat >"${fake_bin}/gh" <<'GH'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >>"${TEST_GH_LOG}"

if [[ "$1 $2" == "release download" ]]; then
  while (($#)); do
    if [[ "$1" == "--dir" ]]; then
      cp "${TEST_ARCHIVE}" "$2/"
      exit 0
    fi
    shift
  done
  exit 2
fi

if [[ "$1 $2" == "release verify-asset" ]]; then
  [[ "${TEST_TAMPERED:-0}" != "1" ]]
  exit
fi

exit 2
GH
chmod +x "${fake_bin}/gh"

output="${temporary_directory}/launcher-output"
gh_log="${temporary_directory}/gh-log"

PATH="${fake_bin}:${PATH}" \
TEST_ARCHIVE="${temporary_directory}/dsm-launcher-x86_64-unknown-linux-musl.tar.gz" \
TEST_GH_LOG="${gh_log}" \
TEST_OUTPUT="${output}" \
DSM_GITHUB_TOKEN="test-token" \
DSM_TEAM_SLUG="platform" \
DSM_TEMPLATE_PATH=".github/standup.md" \
DSM_TIMEZONE="Asia/Tokyo" \
GITHUB_REPOSITORY="example/service" \
GITHUB_REPOSITORY_OWNER="example" \
GITHUB_WORKSPACE="/workspace" \
RUNNER_TEMP="${runner_temp}" \
"${action_script}"

assert_contains "token=test-token" "${output}"
assert_contains "owner=example" "${output}"
assert_contains "repository=service" "${output}"
assert_contains "team=platform" "${output}"
assert_contains "template=/workspace/.github/standup.md" "${output}"
assert_contains "timezone=Asia/Tokyo" "${output}"
assert_contains "release download v1.0.0 --repo atls/dsm --pattern dsm-launcher-x86_64-unknown-linux-musl.tar.gz" "${gh_log}"
assert_contains "release verify-asset v1.0.0" "${gh_log}"

rm -f "${output}"
if PATH="${fake_bin}:${PATH}" \
  TEST_ARCHIVE="${temporary_directory}/dsm-launcher-x86_64-unknown-linux-musl.tar.gz" \
  TEST_GH_LOG="${gh_log}" \
  TEST_OUTPUT="${output}" \
  TEST_TAMPERED="1" \
  DSM_GITHUB_TOKEN="test-token" \
  DSM_TEAM_SLUG="platform" \
  DSM_TEMPLATE_PATH=".github/standup.md" \
  DSM_TIMEZONE="UTC" \
  GITHUB_REPOSITORY="example/service" \
  GITHUB_REPOSITORY_OWNER="example" \
  GITHUB_WORKSPACE="/workspace" \
  RUNNER_TEMP="${runner_temp}" \
  "${action_script}"; then
  fail "tampered launcher archive was accepted"
fi

[[ ! -e "${output}" ]] || fail "launcher ran after release verification failed"

action_metadata="${repository_root}/action.yml"
assert_contains "DSM_GITHUB_TOKEN: \${{ inputs.github-token }}" "${action_metadata}"
assert_contains "DSM_TEAM_SLUG: \${{ inputs.team-slug }}" "${action_metadata}"
assert_contains "DSM_TEMPLATE_PATH: \${{ inputs.template-path }}" "${action_metadata}"
assert_contains "DSM_TIMEZONE: \${{ inputs.timezone }}" "${action_metadata}"

if grep -R --exclude-dir=target -E 'latest:[[:space:]]*true|release download[[:space:]]+--latest' \
  "${repository_root}/action.yml" \
  "${repository_root}/.github/scripts" \
  "${repository_root}/.github/workflows" >/dev/null; then
  fail "mutable latest release lookup remains"
fi
