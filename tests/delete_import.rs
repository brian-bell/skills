use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use skill_importer::{
    DeleteImportRequest, DiscoveryRoots, EnableSkillRequest, ImportMarkdownRequest,
    PromoteSkillRequest, SkillActionKind, SkillAgent, SkillOperationError,
    delete_unpromoted_import, discover_skills, enable_skill, import_markdown_skill,
    promote_imported_skill,
};

#[test]
fn deleting_unpromoted_import_removes_storage_reports_action_and_updates_inventory() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    import_markdown(&roots, "delete-helper");
    let import_path =
        fs::canonicalize(roots.imports_root.join("delete-helper")).expect("import path");

    let result = delete_unpromoted_import(
        &roots,
        DeleteImportRequest {
            skill_name: "delete-helper",
        },
    )
    .expect("delete succeeds");

    assert_eq!(result.skill_name, "delete-helper");
    assert_eq!(result.actions.len(), 1);
    assert_eq!(result.actions[0].action, SkillActionKind::RemoveDirectory);
    assert_eq!(result.actions[0].path, import_path);
    assert!(!roots.imports_root.join("delete-helper").exists());
    let inventory = discover_skills(&roots).expect("inventory");
    assert!(
        inventory
            .skills
            .iter()
            .all(|skill| skill.name != "delete-helper")
    );
}

#[test]
fn deleting_import_enabled_for_either_agent_is_blocked_without_mutation() {
    for agent in [SkillAgent::ClaudeCode, SkillAgent::Codex] {
        let temp = tempfile::tempdir().expect("tempdir");
        let roots = roots(temp.path());
        let import = import_markdown(&roots, "enabled-helper");
        enable_skill(
            &roots,
            EnableSkillRequest {
                skill_name: "enabled-helper",
                agents: &[agent],
            },
        )
        .expect("enable");

        let error = delete_unpromoted_import(
            &roots,
            DeleteImportRequest {
                skill_name: "enabled-helper",
            },
        )
        .expect_err("enabled import blocks deletion");

        assert!(matches!(
            error.error,
            SkillOperationError::EnabledImport { name, .. } if name == "enabled-helper"
        ));
        assert!(roots.imports_root.join("enabled-helper").exists());
        assert_eq!(
            fs::canonicalize(agent_root(&roots, agent).join("enabled-helper"))
                .expect("agent target"),
            import.skill_path
        );
    }
}

#[test]
fn delete_reports_unknown_canonical_agent_only_and_promoted_imports() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());

    let unknown = delete_unpromoted_import(
        &roots,
        DeleteImportRequest {
            skill_name: "missing-helper",
        },
    )
    .expect_err("unknown");
    assert!(matches!(
        unknown.error,
        SkillOperationError::UnknownSkill { name } if name == "missing-helper"
    ));

    write_skill(&roots.canonical_root, "canonical-helper");
    let canonical = delete_unpromoted_import(
        &roots,
        DeleteImportRequest {
            skill_name: "canonical-helper",
        },
    )
    .expect_err("canonical unsupported");
    assert!(matches!(
        canonical.error,
        SkillOperationError::UnsupportedSkillSource { name } if name == "canonical-helper"
    ));

    let agent_only = write_skill(&temp.path().join("external"), "agent-helper");
    fs::create_dir_all(&roots.claude_code_root).expect("claude root");
    unix_fs::symlink(agent_only, roots.claude_code_root.join("agent-helper"))
        .expect("agent symlink");
    let agent = delete_unpromoted_import(
        &roots,
        DeleteImportRequest {
            skill_name: "agent-helper",
        },
    )
    .expect_err("agent unsupported");
    assert!(matches!(
        agent.error,
        SkillOperationError::UnsupportedSkillSource { name } if name == "agent-helper"
    ));

    import_markdown(&roots, "promoted-helper");
    promote_imported_skill(
        &roots,
        PromoteSkillRequest {
            skill_name: "promoted-helper",
        },
    )
    .expect("promote");
    let promoted = delete_unpromoted_import(
        &roots,
        DeleteImportRequest {
            skill_name: "promoted-helper",
        },
    )
    .expect_err("promoted import");
    assert!(matches!(
        promoted.error,
        SkillOperationError::AlreadyPromoted { name } if name == "promoted-helper"
    ));
}

#[test]
fn delete_ignores_unrelated_same_name_agent_entries_without_touching_them() {
    for case in [
        UnsafeEntry::Directory,
        UnsafeEntry::File,
        UnsafeEntry::ExternalSymlink,
        UnsafeEntry::BrokenSymlink,
        UnsafeEntry::WrongManagedSymlink,
    ] {
        let temp = tempfile::tempdir().expect("tempdir");
        let roots = roots(temp.path());
        import_markdown(&roots, "cleanup-helper");
        place_unrelated_agent_entry(&roots, "cleanup-helper", case);

        delete_unpromoted_import(
            &roots,
            DeleteImportRequest {
                skill_name: "cleanup-helper",
            },
        )
        .expect("delete succeeds");

        assert!(!roots.imports_root.join("cleanup-helper").exists());
        assert_entry_still_exists(&roots.claude_code_root.join("cleanup-helper"), case);
    }
}

#[test]
fn delete_command_emits_action_json_and_reports_enabled_imports() {
    let temp = tempfile::tempdir().expect("tempdir");
    let roots = roots(temp.path());
    import_markdown(&roots, "command-delete");

    let output = skill_importer_command()
        .args(["delete", "--json", "--skill", "command-delete"])
        .args(root_args(&roots))
        .output()
        .expect("run delete");
    assert!(
        output.status.success(),
        "delete failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).expect("delete json");
    assert_eq!(json["skill_name"], "command-delete");
    assert_eq!(json["actions"][0]["action"], "remove_directory");

    import_markdown(&roots, "command-enabled");
    enable_skill(
        &roots,
        EnableSkillRequest {
            skill_name: "command-enabled",
            agents: &[SkillAgent::ClaudeCode],
        },
    )
    .expect("enable");
    let output = skill_importer_command()
        .args(["delete", "--json", "--skill", "command-enabled"])
        .args(root_args(&roots))
        .output()
        .expect("run failing delete");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("failed to delete import"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("disable it first"), "stderr: {stderr}");
    assert!(roots.imports_root.join("command-enabled").exists());
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

fn import_markdown(roots: &DiscoveryRoots, name: &str) -> skill_importer::ImportResult {
    import_markdown_skill(
        roots,
        ImportMarkdownRequest {
            markdown: &skill_markdown(name),
            source_location: None,
        },
    )
    .expect("import markdown")
}

fn skill_markdown(name: &str) -> String {
    format!(
        r#"---
name: {name}
description: Delete test skill.
---

# Delete Test
"#
    )
}

fn write_skill(root: &Path, name: &str) -> PathBuf {
    let skill_dir = root.join(name);
    fs::create_dir_all(&skill_dir).expect("skill dir");
    fs::write(skill_dir.join("SKILL.md"), skill_markdown(name)).expect("skill file");
    fs::canonicalize(skill_dir).expect("canonical skill dir")
}

fn place_unrelated_agent_entry(roots: &DiscoveryRoots, name: &str, case: UnsafeEntry) {
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

fn agent_root(roots: &DiscoveryRoots, agent: SkillAgent) -> PathBuf {
    match agent {
        SkillAgent::ClaudeCode => roots.claude_code_root.clone(),
        SkillAgent::Codex => roots.codex_root.clone(),
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
