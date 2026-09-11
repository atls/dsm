#!/usr/bin/env bash

set -euo pipefail

readonly LAUNCHER_REPOSITORY="atls/dsm"
readonly LAUNCHER_RELEASE="v1.0.0"
readonly LAUNCHER_ASSET="dsm-launcher-x86_64-unknown-linux-musl.tar.gz"

: "${DSM_GITHUB_TOKEN:?github-token input is required}"
: "${DSM_TEAM_SLUG:?team-slug input is required}"
: "${DSM_TEMPLATE_PATH:?template-path input is required}"
: "${DSM_TIMEZONE:?timezone input is required}"
: "${GITHUB_REPOSITORY:?GITHUB_REPOSITORY is required}"
: "${GITHUB_REPOSITORY_OWNER:?GITHUB_REPOSITORY_OWNER is required}"
: "${GITHUB_WORKSPACE:?GITHUB_WORKSPACE is required}"
: "${RUNNER_TEMP:?RUNNER_TEMP is required}"

command -v gh >/dev/null
command -v tar >/dev/null

launcher_directory="$(mktemp -d "${RUNNER_TEMP}/dsm-launcher.XXXXXX")"
launcher_archive="${launcher_directory}/${LAUNCHER_ASSET}"

GH_TOKEN="${DSM_GITHUB_TOKEN}" gh release download "${LAUNCHER_RELEASE}" \
  --repo "${LAUNCHER_REPOSITORY}" \
  --pattern "${LAUNCHER_ASSET}" \
  --dir "${launcher_directory}"
GH_TOKEN="${DSM_GITHUB_TOKEN}" gh release verify-asset "${LAUNCHER_RELEASE}" \
  "${launcher_archive}" \
  --repo "${LAUNCHER_REPOSITORY}" >/dev/null

tar -xzf "${launcher_archive}" -C "${launcher_directory}"

template_path="${DSM_TEMPLATE_PATH}"
if [[ "${template_path}" != /* ]]; then
  template_path="${GITHUB_WORKSPACE}/${template_path}"
fi

repo_name="${GITHUB_REPOSITORY#*/}"
if [[ "${repo_name}" == "${GITHUB_REPOSITORY}" ]]; then
  printf 'GITHUB_REPOSITORY must have the form owner/repository\n' >&2
  exit 1
fi

GITHUB_TOKEN="${DSM_GITHUB_TOKEN}" \
GITHUB_REPO_OWNER="${GITHUB_REPOSITORY_OWNER}" \
GITHUB_REPO_NAME="${repo_name}" \
DSM_TEAM_SLUG="${DSM_TEAM_SLUG}" \
DSM_TEMPLATE_PATH="${template_path}" \
DSM_TIMEZONE="${DSM_TIMEZONE}" \
exec "${launcher_directory}/dsm-launcher"
