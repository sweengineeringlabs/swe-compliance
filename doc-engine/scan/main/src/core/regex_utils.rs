use std::sync::LazyLock;
use regex::Regex;

/// Matches Unix absolute paths (/mnt/, /home/, /Users/, /tmp/, /var/, /opt/, /etc/)
/// and Windows drive-letter paths (e.g. C:\...).
pub(crate) static HARDCODED_PATH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(/mnt/|/home/|/Users/|/tmp/|/var/|/opt/|/etc/)\S+|[A-Za-z]:\\[^\s]+").unwrap()
});

/// Matches source-code file extensions that indicate implementation details.
pub(crate) static SOURCE_FILE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\.\b(rs|py|ts|tsx|js|jsx|go|java|rb|c|cpp|hpp|h|cs|swift|kt)\b").unwrap()
});

/// Returns true if `text` contains a source-code file reference.
pub(crate) fn contains_source_file_ref(text: &str) -> bool {
    SOURCE_FILE_RE.is_match(text)
}

/// Matches downstream SDLC phase directory references (phases 2-7).
pub(crate) static DOWNSTREAM_ARTIFACT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[2-7]-(planning|design|development|testing|deployment|operations)/").unwrap()
});

/// Returns true if `text` contains a downstream SDLC artifact reference.
pub(crate) fn contains_downstream_ref(text: &str) -> bool {
    DOWNSTREAM_ARTIFACT_RE.is_match(text)
}

/// Returns the first hardcoded-path match in `line`, or `None`.
/// Skips matches embedded inside a URL (e.g. `https://example.com/home/page`).
pub(crate) fn find_hardcoded_path(line: &str) -> Option<regex::Match<'_>> {
    for m in HARDCODED_PATH_RE.find_iter(line) {
        let before = &line[..m.start()];
        if let Some(proto_pos) = before.rfind("://") {
            let between = &before[proto_pos + 3..];
            if !between.contains(char::is_whitespace) {
                continue;
            }
        }
        return Some(m);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix_home() {
        let m = find_hardcoded_path("Edit /home/alice/.bashrc").unwrap();
        assert_eq!(m.as_str(), "/home/alice/.bashrc");
    }

    #[test]
    fn unix_mnt() {
        let m = find_hardcoded_path("See /mnt/data/file.txt for details").unwrap();
        assert_eq!(m.as_str(), "/mnt/data/file.txt");
    }

    #[test]
    fn unix_users() {
        let m = find_hardcoded_path("Open /Users/bob/docs").unwrap();
        assert_eq!(m.as_str(), "/Users/bob/docs");
    }

    #[test]
    fn unix_tmp() {
        let m = find_hardcoded_path("Stored in /tmp/build").unwrap();
        assert_eq!(m.as_str(), "/tmp/build");
    }

    #[test]
    fn unix_var() {
        let m = find_hardcoded_path("Logs at /var/log/app.log").unwrap();
        assert_eq!(m.as_str(), "/var/log/app.log");
    }

    #[test]
    fn unix_opt() {
        let m = find_hardcoded_path("Installed in /opt/tool/bin").unwrap();
        assert_eq!(m.as_str(), "/opt/tool/bin");
    }

    #[test]
    fn unix_etc() {
        let m = find_hardcoded_path("Config at /etc/app.conf").unwrap();
        assert_eq!(m.as_str(), "/etc/app.conf");
    }

    #[test]
    fn windows_drive() {
        let m = find_hardcoded_path(r"Edit C:\Users\alice\file.txt").unwrap();
        assert_eq!(m.as_str(), r"C:\Users\alice\file.txt");
    }

    #[test]
    fn windows_lower_drive() {
        let m = find_hardcoded_path(r"See d:\data\report.csv").unwrap();
        assert_eq!(m.as_str(), r"d:\data\report.csv");
    }

    #[test]
    fn skips_https_url() {
        assert!(find_hardcoded_path("See https://example.com/home/page").is_none());
    }

    #[test]
    fn skips_http_url() {
        assert!(find_hardcoded_path("See http://example.com/var/data").is_none());
    }

    #[test]
    fn skips_file_url() {
        assert!(find_hardcoded_path("Open file:///home/user/doc").is_none());
    }

    #[test]
    fn relative_path_no_match() {
        assert!(find_hardcoded_path("Use ./config or ../data").is_none());
    }

    #[test]
    fn no_path() {
        assert!(find_hardcoded_path("Just a normal sentence.").is_none());
    }

    // ── contains_source_file_ref tests ──

    #[test]
    fn source_ref_rs() {
        assert!(contains_source_file_ref("core/rules.rs"));
    }

    #[test]
    fn source_ref_py() {
        assert!(contains_source_file_ref("scripts/build.py"));
    }

    #[test]
    fn source_ref_go() {
        assert!(contains_source_file_ref("cmd/main.go"));
    }

    #[test]
    fn source_ref_java() {
        assert!(contains_source_file_ref("src/Main.java"));
    }

    #[test]
    fn source_ref_ts() {
        assert!(contains_source_file_ref("index.ts"));
    }

    #[test]
    fn source_ref_cpp() {
        assert!(contains_source_file_ref("lib/parser.cpp"));
    }

    #[test]
    fn source_ref_no_match_md() {
        assert!(!contains_source_file_ref("docs/srs.md"));
    }

    #[test]
    fn source_ref_no_match_toml() {
        assert!(!contains_source_file_ref("config/rules.toml"));
    }

    #[test]
    fn source_ref_no_match_yaml() {
        assert!(!contains_source_file_ref("spec.yaml"));
    }

    #[test]
    fn source_ref_no_match_json() {
        assert!(!contains_source_file_ref("package.json"));
    }

    #[test]
    fn source_ref_no_match_rst() {
        // .rst should not match — only "rs" is in the alternation and \b prevents partial
        assert!(!contains_source_file_ref("docs/guide.rst"));
    }

    #[test]
    fn source_ref_no_match_plain_text() {
        assert!(!contains_source_file_ref("Just a normal sentence."));
    }

    // ── contains_downstream_ref tests ──

    #[test]
    fn downstream_ref_design() {
        assert!(contains_downstream_ref("docs/3-design/architecture.md"));
    }

    #[test]
    fn downstream_ref_testing() {
        assert!(contains_downstream_ref("docs/5-testing/test_plan.md"));
    }

    #[test]
    fn downstream_ref_planning() {
        assert!(contains_downstream_ref("../2-planning/backlog.md"));
    }

    #[test]
    fn downstream_ref_development() {
        assert!(contains_downstream_ref("4-development/guide.md"));
    }

    #[test]
    fn downstream_ref_deployment() {
        assert!(contains_downstream_ref("6-deployment/release.md"));
    }

    #[test]
    fn downstream_ref_operations() {
        assert!(contains_downstream_ref("7-operations/runbook.md"));
    }

    #[test]
    fn downstream_ref_no_match_requirements() {
        assert!(!contains_downstream_ref("1-requirements/srs.md"));
    }

    #[test]
    fn downstream_ref_no_match_ideation() {
        assert!(!contains_downstream_ref("0-ideation/conops.md"));
    }

    #[test]
    fn downstream_ref_no_match_stakeholder() {
        assert!(!contains_downstream_ref("STK-01"));
    }

    #[test]
    fn downstream_ref_no_match_check_id() {
        assert!(!contains_downstream_ref("Check 130"));
    }

    #[test]
    fn downstream_ref_no_match_rules_toml() {
        assert!(!contains_downstream_ref("rules.toml"));
    }

    #[test]
    fn downstream_ref_no_match_plain_text() {
        assert!(!contains_downstream_ref("Just a normal sentence."));
    }
}
