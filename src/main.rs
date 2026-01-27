mod app;
mod pdf;
mod ui;

use iced::Size;

fn main() -> iced::Result {
    iced::application(app::ChicaliPdf::title, app::ChicaliPdf::update, app::ChicaliPdf::view)
        .subscription(app::ChicaliPdf::subscription)
        .window_size(Size::new(1024.0, 768.0))
        .run_with(app::ChicaliPdf::new)
}
