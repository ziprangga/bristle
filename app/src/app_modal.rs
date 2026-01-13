use iced::Element;
use iced::widget::{Column, Container, Row, Text};
use iced::{Color, Length, alignment};

use widget::button_style::{CustomButton, blank_border_style, danger_style};

#[derive(Clone, Default)]
pub struct ModalAsk {
    pub show_modal: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum ModalAskMessage {
    ConfirmMsg(bool),
}

impl ModalAsk {
    /// Sets the modal message and shows it
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = msg.into();
        self.show_modal = true;
    }

    /// Hides the modal
    pub fn hide(&mut self) {
        self.show_modal = false;
        self.message.clear();
    }

    /// Updates based on the user answer
    pub fn update(&mut self, msg: ModalAskMessage) -> Option<bool> {
        match msg {
            ModalAskMessage::ConfirmMsg(answer) => {
                self.hide();
                Some(answer)
            }
        }
    }

    pub fn view(&self) -> Option<Element<'_, ModalAskMessage>> {
        if !self.show_modal {
            return None;
        }

        Some({
            // modal text
            let modal_text = Text::new(&self.message)
                .size(14)
                .width(Length::Fill)
                .align_x(alignment::Horizontal::Center);

            // Yes button
            let yes_btn = CustomButton::new("Yes")
                .text_size(12)
                .width(Length::Fill)
                .on_press(ModalAskMessage::ConfirmMsg(true))
                .style(blank_border_style)
                .view();

            // No button
            let no_btn = CustomButton::new("No")
                .text_size(12)
                .width(Length::Fill)
                .on_press(ModalAskMessage::ConfirmMsg(false))
                .style(danger_style)
                .view();

            // buttons row
            let buttons_row = Row::new().spacing(10).push(yes_btn).push(no_btn);

            // modal content column
            let modal_column = Column::new().spacing(12).push(modal_text).push(buttons_row);

            // full modal container
            Container::new(modal_column)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .style(|_| iced::widget::container::Style {
                    background: Some(iced::Background::Color(Color::from_rgba8(0, 0, 0, 120.0))),
                    ..Default::default()
                })
                .into()
        })
    }
}
