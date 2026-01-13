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

// use iced::{
//     Element, alignment,
//     widget::{button, center, column, container, mouse_area, opaque, row, space, stack, text},
// };

// #[derive(Debug, Clone)]
// pub enum KillModalMessage {
//     Open { app_name: String },
//     Yes,
//     No,
//     Close,
// }

// #[derive(Default, Clone)]
// pub struct KillModal {
//     pub visible: bool,
//     pub app_name: String,
// }

// impl KillModal {
//     pub fn update(&mut self, msg: &KillModalMessage) -> Option<bool> {
//         match msg {
//             KillModalMessage::Open { app_name } => {
//                 self.app_name = app_name.clone();
//                 self.visible = true;
//                 None
//             }

//             KillModalMessage::Yes => {
//                 self.visible = false;
//                 Some(true)
//             }

//             KillModalMessage::No | KillModalMessage::Close => {
//                 self.visible = false;
//                 Some(false)
//             }
//         }
//     }

//     pub fn view<'a>(&self) -> Element<'a, KillModalMessage> {
//         let dialog = container(
//             column![
//                 text("Application Still Running")
//                     .size(20)
//                     .align_x(alignment::Horizontal::Center),
//                 space::vertical(),
//                 text(format!(
//                     "The app \"{}\" is still running.\n\
//                          Do you want to kill its running process?\n\
//                          Be sure to save your work first.",
//                     self.app_name
//                 )),
//                 space::vertical(),
//                 row![
//                     button("No").on_press(KillModalMessage::No),
//                     space::horizontal(),
//                     button("Yes").on_press(KillModalMessage::Yes),
//                 ]
//             ]
//             .spacing(10),
//         )
//         .padding(20)
//         .width(420)
//         .style(container::rounded_box);

//         stack![opaque(
//             mouse_area(center(opaque(dialog))).on_press(KillModalMessage::Close)
//         )]
//         .into()
//     }
// }
