use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::{env, path::PathBuf};

const DEFAULT_TEAM_SLUG: &str = "DSM";
const DEFAULT_TEMPLATE_PATH: &str = "./.github/ISSUE_TEMPLATE/dsm.md";
const DEFAULT_TIMEZONE: &str = "UTC";
const TITLE_DATE_FORMAT: &str = "%a %b %d %Y";

pub struct Config {
    pub github_token: String,
    pub repo_owner: String,
    pub repo_name: String,
    pub team_slug: String,
    pub template_path: PathBuf,
    timezone: Tz,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Self::from_lookup(|name| env::var(name).ok())
    }

    pub fn title(&self, now: DateTime<Utc>) -> String {
        format!(
            "[DSM] {}",
            now.with_timezone(&self.timezone)
                .date_naive()
                .format(TITLE_DATE_FORMAT)
        )
    }

    fn from_lookup<F>(mut lookup: F) -> Result<Self>
    where
        F: FnMut(&str) -> Option<String>,
    {
        let timezone_name = lookup("DSM_TIMEZONE").unwrap_or_else(|| DEFAULT_TIMEZONE.to_string());
        let timezone = timezone_name
            .parse::<Tz>()
            .with_context(|| format!("invalid DSM_TIMEZONE `{timezone_name}`"))?;

        Ok(Self {
            github_token: lookup("GITHUB_TOKEN").context("GITHUB_TOKEN is required")?,
            repo_owner: lookup("GITHUB_REPO_OWNER").context("GITHUB_REPO_OWNER is required")?,
            repo_name: lookup("GITHUB_REPO_NAME").context("GITHUB_REPO_NAME is required")?,
            team_slug: lookup("DSM_TEAM_SLUG").unwrap_or_else(|| DEFAULT_TEAM_SLUG.to_string()),
            template_path: lookup("DSM_TEMPLATE_PATH")
                .unwrap_or_else(|| DEFAULT_TEMPLATE_PATH.to_string())
                .into(),
            timezone,
        })
    }
}

#[cfg(test)]
mod tests;
