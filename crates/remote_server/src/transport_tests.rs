use super::{Error, SetupStage};

#[test]
fn timeout_user_facing_error_explains_ssh_reconnect() {
    let error = Error::TimedOut.user_facing_error(SetupStage::CheckBinary);

    assert_eq!(error.body, "Failed to verify SSH extension");
    assert_eq!(
        error.detail.as_deref(),
        Some(
            "Timed out while waiting for the remote host to respond. Check that your SSH connection is stable, then reconnect to retry the Warp SSH extension."
        )
    );
}
