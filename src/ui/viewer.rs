use iced::widget::{center, column, container, image, row, scrollable, text};
use iced::{Element, Length};

use crate::app::Message;

pub fn viewer_view<'a>(
    page_image: Option<&'a iced::widget::image::Handle>,
    current_page: usize,
    page_count: usize,
    zoom: f32,
) -> Element<'a, Message> {
    let content: Element<'a, Message> = match page_image {
        Some(handle) => {
            let img = image(handle.clone());

            container(
                scrollable(
                    container(img)
                        .padding(20)
                        .center_x(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
        None => {
            center(
                text("No document loaded\n\nPress Ctrl+O or drag a PDF file to open")
                    .size(18)
                    .center(),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
        }
    };

    let status_bar = container(
        row![
            text(if page_count > 0 {
                format!("Page {} of {}", current_page + 1, page_count)
            } else {
                String::from("No document")
            })
            .size(14),
            text(format!("  |  Zoom: {:.0}%", zoom * 100.0)).size(14),
        ]
        .spacing(10),
    )
    .padding(8)
    .style(container::bordered_box)
    .width(Length::Fill);

    column![content, status_bar].into()
}
