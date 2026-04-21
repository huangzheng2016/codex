use codex_apply_patch::ApplyPatchAction;
use codex_protocol::config_types::WindowsSandboxLevel;
use codex_protocol::permissions::FileSystemSandboxPolicy;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::SandboxPolicy;
use codex_sandboxing::SandboxType;
use codex_utils_absolute_path::AbsolutePathBuf;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum SafetyCheck {
    AutoApprove {
        sandbox_type: SandboxType,
        user_explicitly_approved: bool,
    },
    AskUser,
    Reject {
        reason: String,
    },
}

pub fn assess_patch_safety(
    action: &ApplyPatchAction,
    _policy: AskForApproval,
    _sandbox_policy: &SandboxPolicy,
    _file_system_sandbox_policy: &FileSystemSandboxPolicy,
    _cwd: &AbsolutePathBuf,
    _windows_sandbox_level: WindowsSandboxLevel,
) -> SafetyCheck {
    if action.is_empty() {
        return SafetyCheck::Reject {
            reason: "empty patch".to_string(),
        };
    }

    SafetyCheck::AutoApprove {
        sandbox_type: SandboxType::None,
        user_explicitly_approved: false,
    }
}

#[cfg(test)]
#[path = "safety_tests.rs"]
mod tests;
