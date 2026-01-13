// Design modal not yet implementation
// still using spawn osascript for help to Mac native dialog
use anyhow::Result;
use std::process::Command;

pub fn modal_process_kill_dialog(app_name: &str) -> Result<bool> {
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

// use anyhow::Result;
// use objc2::rc::autoreleasepool;
// use objc2::{class, msg_send};
// use objc2_app_kit::NSAlert;
// use objc2_foundation::NSString;

// pub fn modal_process_kill_dialog(app_name: &str) -> Result<bool> {
//     autoreleasepool(|_| unsafe {
//         // NSAlert *alert = [[NSAlert alloc] init];
//         let alert: *mut NSAlert = msg_send![class!(NSAlert), alloc];
//         let alert: *mut NSAlert = msg_send![alert, init];

//         // NSStrings
//         let message = NSString::from_str(&format!("The app '{}' is still running.", app_name));
//         let info = NSString::from_str(
//             "Do you want to kill its running process?\nBe careful to save your work first!",
//         );

//         // Set texts
//         let _: () = msg_send![alert, setMessageText: &*message];
//         let _: () = msg_send![alert, setInformativeText: &*info];

//         let yes = NSString::from_str("Yes");
//         let no = NSString::from_str("No");

//         let _: () = msg_send![alert, addButtonWithTitle: &*yes];
//         let _: () = msg_send![alert, addButtonWithTitle: &*no];

//         // Run modal
//         let response: isize = msg_send![alert, runModal];

//         // NSAlertFirstButtonReturn == 1000
//         Ok(response == 1000)
//     })
// }
