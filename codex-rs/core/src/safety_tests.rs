use super::*;
use codex_apply_patch::MaybeApplyPatchVerified;
use codex_exec_server::LOCAL_FS;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::SandboxPolicy;
use codex_sandboxing::SandboxType;
use core_test_support::PathExt;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

#[tokio::test]
async fn empty_patch_is_rejected() {
    let tmp = TempDir::new().unwrap();
    let cwd = tmp.path().abs();
    let argv = vec![
        "apply_patch".to_string(),
        "*** Begin Patch\n*** End Patch".to_string(),
    ];
    let action = match codex_apply_patch::maybe_parse_apply_patch_verified(
        &argv,
        &cwd,
        LOCAL_FS.as_ref(),
        /*sandbox*/ None,
    )
    .await
    {
        MaybeApplyPatchVerified::Body(action) => action,
        other => panic!("expected patch body, got {other:?}"),
    };
    let sandbox_policy = SandboxPolicy::new_read_only_policy();
    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::from_legacy_sandbox_policy(&sandbox_policy, &cwd);

    assert_eq!(
        assess_patch_safety(
            &action,
            AskForApproval::Never,
            &sandbox_policy,
            &file_system_sandbox_policy,
            &cwd,
            WindowsSandboxLevel::Disabled,
        ),
        SafetyCheck::Reject {
            reason: "empty patch".to_string(),
        },
    );
}

#[test]
fn non_empty_patch_is_auto_approved_regardless_of_policy() {
    let tmp = TempDir::new().unwrap();
    let cwd = tmp.path().abs();
    let action = ApplyPatchAction::new_add_for_test(&cwd.join("inside.txt"), "".to_string());
    let sandbox_policy = SandboxPolicy::new_read_only_policy();
    let file_system_sandbox_policy =
        FileSystemSandboxPolicy::from_legacy_sandbox_policy(&sandbox_policy, &cwd);

    assert_eq!(
        assess_patch_safety(
            &action,
            AskForApproval::UnlessTrusted,
            &sandbox_policy,
            &file_system_sandbox_policy,
            &cwd,
            WindowsSandboxLevel::Disabled,
        ),
        SafetyCheck::AutoApprove {
            sandbox_type: SandboxType::None,
            user_explicitly_approved: false,
        },
    );
}
