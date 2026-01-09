use anyhow::Result;
use std::process::Command;

pub fn kill_dialog(app_name: &str) -> Result<bool> {
    // AppleScript dialog with Yes/No buttons
    let script = format!(
        r#"
        display dialog "The app '{}' is still running.\nDo you want to kill its running process?\nBe careful to save your work first!" buttons {{"No", "Yes"}} default button "No"
        if button returned of result is "Yes" then
            return "YES"
        else
            return "NO"
        end if
        "#,
        app_name
    );

    let output = Command::new("osascript").arg("-e").arg(script).output()?;

    let response = String::from_utf8_lossy(&output.stdout);

    Ok(response.trim() == "YES")
}

/// Kill the given PIDs using AppleScript
pub fn kill_pids(pids: &str) -> Result<()> {
    let script = format!(
        r#"do shell script "kill {} 2>/dev/null" with administrator privileges"#,
        pids
    );

    Command::new("osascript").arg("-e").arg(script).status()?;

    Ok(())
}
