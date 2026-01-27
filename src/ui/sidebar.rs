use iced::widget::{button, column, container, image, scrollable, text};
use iced::{Element, Length};

use crate::app::Message;

pub fn sidebar_view<'a>(
    thumbnails: &'a [iced::widget::image::Handle],
    current_page: usize,
    page_count: usize,
) -> Element<'a, Message> {
    let mut content = column![].spacing(8).padding(8);

    for (idx, thumb) in thumbnails.iter().enumerate() {
        let thumb_image = image(thumb.clone())
            .width(Length::Fixed(120.0));

        let page_label = text(format!("{}", idx + 1)).size(12);

        let thumb_container = container(
            column![thumb_image, page_label]
                .align_x(iced::Alignment::Center)
                .spacing(4),
        )
        .padding(4)
        .style(if idx == current_page {
            container::bordered_box
        } else {
            container::transparent
        });

        let thumb_button = button(thumb_container)
            .on_press(Message::GoToPage(idx))
            .padding(0)
            .style(button::text);

        content = content.push(thumb_button);
    }

    if page_count > 0 && thumbnails.is_empty() {
        content = content.push(text("Loading thumbnails...").size(12));
    }

    container(
        scrollable(content)
            .width(Length::Fixed(150.0))
            .height(Length::Fill),
    )
    .style(container::bordered_box)
    .height(Length::Fill)
    .into()
}
