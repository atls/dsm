use super::Config;
use chrono::{TimeZone, Utc};
use std::collections::HashMap;

fn required_values() -> HashMap<&'static str, String> {
    HashMap::from([
        ("GITHUB_TOKEN", "token".to_string()),
        ("GITHUB_REPO_OWNER", "example".to_string()),
        ("GITHUB_REPO_NAME", "service".to_string()),
    ])
}

#[test]
fn loads_action_inputs_and_formats_the_title_in_the_requested_timezone() {
    let mut values = required_values();
    values.insert("DSM_TEAM_SLUG", "platform".to_string());
    values.insert("DSM_TEMPLATE_PATH", "templates/standup.md".to_string());
    values.insert("DSM_TIMEZONE", "Europe/Moscow".to_string());

    let config = Config::from_lookup(|name| values.get(name).cloned()).unwrap();
    let now = Utc.with_ymd_and_hms(2026, 9, 10, 21, 30, 0).unwrap();

    assert_eq!(config.team_slug, "platform");
    assert_eq!(config.template_path.to_str(), Some("templates/standup.md"));
    assert_eq!(config.title(now), "[DSM] Fri Sep 11 2026");
}

#[test]
fn keeps_the_existing_launcher_defaults() {
    let values = required_values();

    let config = Config::from_lookup(|name| values.get(name).cloned()).unwrap();
    let now = Utc.with_ymd_and_hms(2026, 9, 10, 21, 30, 0).unwrap();

    assert_eq!(config.team_slug, "DSM");
    assert_eq!(
        config.template_path.to_str(),
        Some("./.github/ISSUE_TEMPLATE/dsm.md")
    );
    assert_eq!(config.title(now), "[DSM] Thu Sep 10 2026");
}

#[test]
fn rejects_an_unknown_timezone_before_github_is_called() {
    let mut values = required_values();
    values.insert("DSM_TIMEZONE", "Mars/Olympus_Mons".to_string());

    let error = Config::from_lookup(|name| values.get(name).cloned())
        .err()
        .unwrap();

    assert_eq!(
        error.to_string(),
        "invalid DSM_TIMEZONE `Mars/Olympus_Mons`"
    );
}
