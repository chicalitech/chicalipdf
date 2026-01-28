use iced::widget::{button, column, container, row, text, Space};
use iced::{Element, Length};

use crate::app::Message;

pub fn menu_bar_view(file_menu_open: bool) -> Element<'static, Message> {
    let file_button = button(text("File").size(14))
        .on_press(Message::ToggleFileMenu)
        .padding([4, 12])
        .style(button::text);

    let menu_bar = row![file_button]
        .spacing(4)
        .padding([4, 8]);

    let menu_bar_container = container(menu_bar)
        .style(container::bordered_box)
        .width(Length::Fill);

    if file_menu_open {
        let menu_items = column![
            menu_item("Open File", Some(Message::OpenFile), "Ctrl+O"),
            menu_item("Open Folder", None, ""),
            menu_separator(),
            menu_item("Save", None, "Ctrl+S"),
            menu_item("Save As...", None, ""),
            menu_separator(),
            menu_item("Exit", None, ""),
        ]
        .spacing(0);

        let dropdown = container(menu_items)
            .style(container::bordered_box)
            .padding(4)
            .width(Length::Fixed(180.0));

        // Stack the menu bar and dropdown
        column![
            menu_bar_container,
            row![
                Space::with_width(8),
                dropdown,
            ],
        ]
        .into()
    } else {
        column![menu_bar_container].into()
    }
}

fn menu_item<'a>(
    label: &'a str,
    message: Option<Message>,
    shortcut: &'a str,
) -> Element<'a, Message> {
    let label_text = text(label).size(13);
    let shortcut_text = text(shortcut).size(12);

    let content = row![
        label_text,
        Space::with_width(Length::Fill),
        shortcut_text,
    ]
    .spacing(8)
    .padding([6, 8]);

    let mut btn = button(content)
        .width(Length::Fill)
        .style(button::text);

    if let Some(msg) = message {
        btn = btn.on_press(msg);
    }

    btn.into()
}

fn menu_separator<'a>() -> Element<'a, Message> {
    container(Space::with_height(1))
        .width(Length::Fill)
        .style(container::bordered_box)
        .padding([4, 0])
        .into()
}
