use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;
use skill_importer::{
    AgentEnablement, AgentEntryStatus, DisableSkillRequest, DiscoveryRoots, EnableSkillRequest,
    ImportMarkdownRequest, SkillActionKind, SkillAgent, SkillOperationError, SkillSource,
    disable_skill, discover_skills, enable_skill, import_markdown_skill,
};

#[test]
fn enabling_imported_skill_for_claude_creates_root_symlink_and_actions() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    let import = import_skill(&roots, "draft-helper");

    let result = enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "draft-helper",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect("enable succeeds");

    let link = roots.claude_code_root.join("draft-helper");
    assert_eq!(
        fs::canonicalize(&link).expect("link target"),
        import.skill_path
    );
    assert_eq!(result.skill_name, "draft-helper");
    assert_eq!(result.actions.len(), 2);
    assert_eq!(result.actions[0].action, SkillActionKind::CreateDirectory);
    assert_eq!(result.actions[0].agent, Some(SkillAgent::ClaudeCode));
    assert_eq!(result.actions[0].path, roots.claude_code_root);
    assert_eq!(result.actions[1].action, SkillActionKind::CreateSymlink);
    assert_eq!(result.actions[1].agent, Some(SkillAgent::ClaudeCode));
    assert_eq!(result.actions[1].path, link);
    assert_eq!(result.actions[1].target, Some(import.skill_path));
}

#[test]
fn enabling_canonical_skill_for_codex_and_both_agents_updates_discovery() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    let canonical = write_skill(&roots.canonical_root, "canonical-helper");

    let codex_result = enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "canonical-helper",
            agents: &[SkillAgent::Codex],
        },
    )
    .expect("enable codex");
    assert_eq!(
        fs::canonicalize(roots.codex_root.join("canonical-helper")).expect("codex target"),
        canonical
    );
    assert!(
        codex_result
            .actions
            .iter()
            .any(|action| action.action == SkillActionKind::CreateSymlink)
    );

    let both_result = enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "canonical-helper",
            agents: &[SkillAgent::ClaudeCode, SkillAgent::Codex],
        },
    )
    .expect("enable both");
    assert!(
        both_result
            .actions
            .iter()
            .any(|action| action.action == SkillActionKind::SkipUnchanged
                && action.agent == Some(SkillAgent::Codex))
    );

    let inventory = discover_skills(&roots).expect("inventory");
    let skill = inventory
        .skills
        .iter()
        .find(|skill| skill.name == "canonical-helper")
        .expect("canonical skill");
    assert_eq!(skill.source, SkillSource::Canonical);
    assert_eq!(skill.enablement, AgentEnablement::Both);
    assert_eq!(
        skill.agent_entries.claude_code,
        AgentEntryStatus::CanonicalSymlink
    );
    assert_eq!(
        skill.agent_entries.codex,
        AgentEntryStatus::CanonicalSymlink
    );
}

#[test]
fn disabling_imported_skill_removes_only_managed_symlink_and_is_idempotent() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    import_skill(&roots, "draft-helper");
    enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "draft-helper",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect("enable");

    let result = disable_skill(
        &roots,
        DisableSkillRequest {
            skill_name: "draft-helper",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect("disable");

    assert_eq!(result.actions[0].action, SkillActionKind::RemoveSymlink);
    assert!(!roots.claude_code_root.join("draft-helper").exists());
    let inventory = discover_skills(&roots).expect("inventory");
    assert_eq!(inventory.skills[0].enablement, AgentEnablement::Neither);

    let second = disable_skill(
        &roots,
        DisableSkillRequest {
            skill_name: "draft-helper",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect("disable missing");
    assert_eq!(second.actions[0].action, SkillActionKind::SkipUnchanged);
    assert_eq!(second.actions[0].agent, Some(SkillAgent::ClaudeCode));
}

#[test]
fn enable_and_disable_refuse_unsafe_agent_entries_without_mutating_them() {
    let cases = [
        UnsafeEntry::Directory,
        UnsafeEntry::File,
        UnsafeEntry::ExternalSymlink,
        UnsafeEntry::BrokenSymlink,
        UnsafeEntry::WrongManagedSymlink,
    ];

    for case in cases {
        let temp = tempfile::tempdir().expect("case tempdir");
        let roots = roots(temp.path());
        import_skill(&roots, "draft-helper");
        place_unsafe_entry(&roots, "draft-helper", case);

        let error = enable_skill(
            &roots,
            EnableSkillRequest {
                skill_name: "draft-helper",
                agents: &[SkillAgent::ClaudeCode],
            },
        )
        .expect_err("enable refuses unsafe entry");
        assert_unsafe_path(error.error, roots.claude_code_root.join("draft-helper"));
        assert_entry_still_exists(&roots.claude_code_root.join("draft-helper"), case);

        let error = disable_skill(
            &roots,
            DisableSkillRequest {
                skill_name: "draft-helper",
                agents: &[SkillAgent::ClaudeCode],
            },
        )
        .expect_err("disable refuses unsafe entry");
        assert_unsafe_path(error.error, roots.claude_code_root.join("draft-helper"));
        assert_entry_still_exists(&roots.claude_code_root.join("draft-helper"), case);
    }
}

#[test]
fn enable_and_disable_report_unknown_or_agent_only_skills() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());

    let unknown = enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "missing",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect_err("unknown skill");
    assert!(matches!(
        unknown.error,
        SkillOperationError::UnknownSkill { name } if name == "missing"
    ));

    let agent_only = write_skill(&temp.path().join("external"), "agent-only");
    fs::create_dir_all(&roots.claude_code_root).expect("claude root");
    unix_fs::symlink(&agent_only, roots.claude_code_root.join("agent-only"))
        .expect("agent-only symlink");

    let unsupported = enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "agent-only",
            agents: &[SkillAgent::Codex],
        },
    )
    .expect_err("agent-only unsupported");
    assert!(matches!(
        unsupported.error,
        SkillOperationError::UnsupportedSkillSource { name } if name == "agent-only"
    ));

    let unknown_disable = disable_skill(
        &roots,
        DisableSkillRequest {
            skill_name: "missing",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect_err("unknown disable");
    assert!(matches!(
        unknown_disable.error,
        SkillOperationError::UnknownSkill { name } if name == "missing"
    ));
}

#[test]
fn enable_and_disable_commands_emit_action_json() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    import_skill(&roots, "command-helper");

    let output = skill_importer_command()
        .args([
            "enable",
            "--json",
            "--skill",
            "command-helper",
            "--agent",
            "claude-code",
        ])
        .args(root_args(&roots))
        .output()
        .expect("run enable");
    assert!(
        output.status.success(),
        "enable failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("enable json");
    assert_eq!(json["skill_name"], "command-helper");
    assert_eq!(json["actions"][1]["action"], "create_symlink");
    assert_eq!(json["actions"][1]["agent"], "claude_code");

    let output = skill_importer_command()
        .args([
            "disable",
            "--json",
            "--skill",
            "command-helper",
            "--agent",
            "claude-code",
        ])
        .args(root_args(&roots))
        .output()
        .expect("run disable");
    assert!(
        output.status.success(),
        "disable failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("disable json");
    assert_eq!(json["actions"][0]["action"], "remove_symlink");
    assert_eq!(json["actions"][0]["agent"], "claude_code");
}

#[test]
fn disable_command_reports_unsafe_agent_entries() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    import_skill(&roots, "unsafe-helper");
    fs::create_dir_all(&roots.claude_code_root).expect("claude root");
    fs::write(roots.claude_code_root.join("unsafe-helper"), "mine").expect("regular file");

    let output = skill_importer_command()
        .args([
            "disable",
            "--json",
            "--skill",
            "unsafe-helper",
            "--agent",
            "claude-code",
        ])
        .args(root_args(&roots))
        .output()
        .expect("run disable");

    assert!(!output.status.success(), "unsafe disable should fail");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to disable skill"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("unsafe agent entry"), "stderr: {stderr}");
    assert!(
        roots.claude_code_root.join("unsafe-helper").is_file(),
        "regular file should remain untouched"
    );
}

#[derive(Debug, Clone, Copy)]
enum UnsafeEntry {
    Directory,
    File,
    ExternalSymlink,
    BrokenSymlink,
    WrongManagedSymlink,
}

fn roots(base: &Path) -> DiscoveryRoots {
    DiscoveryRoots {
        canonical_root: base.join("canonical"),
        imports_root: base.join("imports"),
        claude_code_root: base.join("claude"),
        codex_root: base.join("codex"),
    }
}

fn import_skill(roots: &DiscoveryRoots, name: &str) -> skill_importer::ImportResult {
    import_markdown_skill(
        roots,
        ImportMarkdownRequest {
            markdown: &format!(
                r#"---
name: {name}
description: Imported for enablement tests.
---

# Test Skill
"#
            ),
            source_location: None,
        },
    )
    .expect("import skill")
}

fn write_skill(root: &Path, name: &str) -> std::path::PathBuf {
    let skill_dir = root.join(name);
    fs::create_dir_all(&skill_dir).expect("skill dir");
    fs::write(
        skill_dir.join("SKILL.md"),
        format!(
            r#"---
name: {name}
description: Test skill.
---
"#
        ),
    )
    .expect("skill file");
    fs::canonicalize(skill_dir).expect("canonical skill dir")
}

fn place_unsafe_entry(roots: &DiscoveryRoots, name: &str, case: UnsafeEntry) {
    fs::create_dir_all(&roots.claude_code_root).expect("claude root");
    let path = roots.claude_code_root.join(name);
    match case {
        UnsafeEntry::Directory => {
            write_skill(&roots.claude_code_root, name);
        }
        UnsafeEntry::File => {
            fs::write(path, "mine").expect("regular file");
        }
        UnsafeEntry::ExternalSymlink => {
            let external = write_skill(&roots.canonical_root.join("external-root"), name);
            unix_fs::symlink(external, path).expect("external symlink");
        }
        UnsafeEntry::BrokenSymlink => {
            unix_fs::symlink(roots.claude_code_root.join("missing"), path).expect("broken symlink");
        }
        UnsafeEntry::WrongManagedSymlink => {
            let other = write_skill(&roots.imports_root, "other-helper");
            unix_fs::symlink(other, path).expect("wrong managed symlink");
        }
    }
}

fn assert_unsafe_path(error: SkillOperationError, path: std::path::PathBuf) {
    match error {
        SkillOperationError::UnsafeAgentEntry { path: actual, .. } => assert_eq!(actual, path),
        error => panic!("expected unsafe agent entry, got {error:?}"),
    }
}

fn assert_entry_still_exists(path: &Path, case: UnsafeEntry) {
    let metadata = fs::symlink_metadata(path).expect("entry still exists");
    match case {
        UnsafeEntry::Directory => assert!(metadata.is_dir()),
        UnsafeEntry::File => assert!(metadata.is_file()),
        UnsafeEntry::ExternalSymlink
        | UnsafeEntry::BrokenSymlink
        | UnsafeEntry::WrongManagedSymlink => assert!(metadata.file_type().is_symlink()),
    }
}

fn skill_importer_command() -> Command {
    Command::new(std::env::var("CARGO_BIN_EXE_skill-importer").expect("binary path"))
}

fn root_args(roots: &DiscoveryRoots) -> Vec<String> {
    vec![
        "--canonical-root".to_string(),
        roots.canonical_root.to_string_lossy().into_owned(),
        "--imports-root".to_string(),
        roots.imports_root.to_string_lossy().into_owned(),
        "--claude-code-root".to_string(),
        roots.claude_code_root.to_string_lossy().into_owned(),
        "--codex-root".to_string(),
        roots.codex_root.to_string_lossy().into_owned(),
    ]
}
