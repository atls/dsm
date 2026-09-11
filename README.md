# Daily Standup Meetings

[![GitHub Marketplace](https://img.shields.io/badge/Marketplace-Daily%20Standup%20Meetings-blue?logo=github)](https://github.com/marketplace/actions/daily-standup-meetings)

DSM creates a dated issue in the current repository and assigns every member of
the selected GitHub organization team. The issue body is read from a template in
the consumer repository; the token and all organization-specific settings stay
in that repository.

## Usage

```yaml
name: DSM

on:
  schedule:
    - cron: "0 4 * * 1-5"
  workflow_dispatch:

permissions:
  contents: read

jobs:
  dsm:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@11d5960a326750d5838078e36cf38b85af677262

      - uses: actions/create-github-app-token@fee1f7d63c2ff003460e3d139729b119787bc349
        id: app-token
        with:
          app-id: ${{ vars.DSM_APP_ID }}
          private-key: ${{ secrets.DSM_APP_PRIVATE_KEY }}
          owner: ${{ github.repository_owner }}
          repositories: ${{ github.event.repository.name }}

      - uses: atls/dsm@v1
        with:
          github-token: ${{ steps.app-token.outputs.token }}
          team-slug: engineering
          template-path: .github/ISSUE_TEMPLATE/dsm.md
          timezone: Europe/Moscow
```

The Action supports Linux x86-64 runners with Bash, `tar`, and a current GitHub
CLI containing `gh release verify-asset`. It downloads only the launcher asset
attached to the immutable `v1.0.0` release and verifies GitHub's signed release
attestation before execution. A missing attestation or digest mismatch fails the
step before the launcher receives the token.

For reviewed production workflows, replace `atls/dsm@v1` with the full 40
character commit SHA for the release. The `v1` tag is the update channel for
compatible releases; a full SHA runs only the Action source that was reviewed.

## Inputs

| Input | Description |
| --- | --- |
| `github-token` | GitHub App installation token used to read the team and create or close DSM issues. |
| `team-slug` | Slug of the organization team whose members are assigned. The same value must exist as a repository issue type. |
| `template-path` | Absolute path or path relative to `GITHUB_WORKSPACE` containing the issue body. |
| `timezone` | IANA timezone used to calculate the date in the issue title, for example `Europe/Moscow`. |

## GitHub App access

Configure the GitHub App with the following minimum permissions:

- Repository permissions: `Metadata: read` and `Issues: read and write`.
- Organization permissions: `Members: read` and `Issue Types: read`.

Install the App on the consumer organization and grant it access only to the
repositories where DSM may manage issues. Keep the App ID in repository
variables and its private key in repository secrets.

If the organization restricts Actions, allow `atls/dsm@*` in the selected
actions allowlist and enable the policy that requires actions to be pinned to a
full-length commit SHA. The example uses `@v1` to show the Marketplace update
channel; the production pin should be the reviewed release commit.

## Release boundary

Before creating `v1.0.0`, enable immutable releases for this repository; the
policy applies only to releases created after it is enabled. Create `v1.0.0` as
a draft at the exact release commit, attach
`dsm-launcher-x86_64-unknown-linux-musl.tar.gz`, and publish the draft only after
all assets are present. Confirm that GitHub marks the release immutable and that
both `gh release verify v1.0.0` and `gh release verify-asset v1.0.0 <asset>`
succeed before creating the compatible `v1` tag at the same commit or publishing
the Action to Marketplace.

License selection, immutable release and tag creation, Marketplace publication,
and acceptance in a private consumer repository are post-merge steps and are
not performed by this change.

## DSM template

#### Какие задачи выполнял вчера?
#### Какие задачи будешь делать сегодня? Укажи #issues
#### Что тебя блокирует? (Этот пункт используется когда тебя что-то блокирует)
#### Есть ли личные дела из-за которых нужно отсутствовать на рабочем месте в течение рабочего дня? (Этот пункт используется когда дела есть)
