use std::path::PathBuf;

use iced::keyboard;
use iced::widget::{column, container, row};
use iced::{Element, Length, Subscription, Task};

use crate::pdf::{render_page, render_thumbnail, Document};
use crate::ui::{menu_bar_view, sidebar_view, viewer_view};

pub struct ChicaliPdf {
    document: Option<Document>,
    current_page: usize,
    zoom: f32,
    thumbnails: Vec<iced::widget::image::Handle>,
    current_page_image: Option<iced::widget::image::Handle>,
    error_message: Option<String>,
    file_menu_open: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenFile,
    FileOpened(Option<PathBuf>),
    DocumentLoaded(Result<DocumentData, String>),
    GoToPage(usize),
    NextPage,
    PrevPage,
    ZoomIn,
    ZoomOut,
    KeyPressed(keyboard::Key, keyboard::Modifiers),
    ThumbnailsLoaded(Vec<iced::widget::image::Handle>),
    PageRendered(usize, iced::widget::image::Handle),
    ToggleFileMenu,
}

#[derive(Debug, Clone)]
pub struct DocumentData {
    pub path: PathBuf,
}

impl Default for ChicaliPdf {
    fn default() -> Self {
        Self {
            document: None,
            current_page: 0,
            zoom: 1.0,
            thumbnails: Vec::new(),
            current_page_image: None,
            error_message: None,
            file_menu_open: false,
        }
    }
}

impl ChicaliPdf {
    pub fn new() -> (Self, Task<Message>) {
        let args: Vec<String> = std::env::args().collect();

        if args.len() > 1 {
            let path = PathBuf::from(&args[1]);
            (Self::default(), Task::done(Message::FileOpened(Some(path))))
        } else {
            (Self::default(), Task::none())
        }
    }

    pub fn title(&self) -> String {
        match &self.document {
            Some(doc) => {
                let filename = std::path::Path::new(doc.path())
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown");
                format!("{} - chicalipdf", filename)
            }
            None => String::from("chicalipdf"),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFile => {
                self.file_menu_open = false;
                return Task::perform(
                    async {
                        let file = rfd::AsyncFileDialog::new()
                            .add_filter("PDF", &["pdf"])
                            .pick_file()
                            .await;
                        file.map(|f| f.path().to_path_buf())
                    },
                    Message::FileOpened,
                );
            }

            Message::FileOpened(Some(path)) => {
                return Task::perform(
                    async move {
                        match Document::open(&path) {
                            Ok(_) => Ok(DocumentData { path }),
                            Err(e) => Err(e.to_string()),
                        }
                    },
                    Message::DocumentLoaded,
                );
            }

            Message::FileOpened(None) => {}

            Message::DocumentLoaded(Ok(data)) => {
                match Document::open(&data.path) {
                    Ok(doc) => {
                        self.document = Some(doc);
                        self.current_page = 0;
                        self.thumbnails.clear();
                        self.current_page_image = None;
                        self.error_message = None;

                        // Render current page and thumbnails
                        return Task::batch([
                            self.render_current_page(),
                            self.load_thumbnails(),
                        ]);
                    }
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                    }
                }
            }

            Message::DocumentLoaded(Err(e)) => {
                self.error_message = Some(e);
            }

            Message::GoToPage(page) => {
                if let Some(doc) = &self.document {
                    if page < doc.page_count() {
                        self.current_page = page;
                        return self.render_current_page();
                    }
                }
            }

            Message::NextPage => {
                if let Some(doc) = &self.document {
                    if self.current_page + 1 < doc.page_count() {
                        self.current_page += 1;
                        return self.render_current_page();
                    }
                }
            }

            Message::PrevPage => {
                if self.current_page > 0 {
                    self.current_page -= 1;
                    return self.render_current_page();
                }
            }

            Message::ZoomIn => {
                self.zoom = (self.zoom * 1.25).min(4.0);
                return self.render_current_page();
            }

            Message::ZoomOut => {
                self.zoom = (self.zoom / 1.25).max(0.25);
                return self.render_current_page();
            }

            Message::KeyPressed(key, modifiers) => {
                match key.as_ref() {
                    keyboard::Key::Named(keyboard::key::Named::ArrowRight)
                    | keyboard::Key::Named(keyboard::key::Named::ArrowDown)
                    | keyboard::Key::Named(keyboard::key::Named::PageDown) => {
                        return self.update(Message::NextPage);
                    }
                    keyboard::Key::Named(keyboard::key::Named::ArrowLeft)
                    | keyboard::Key::Named(keyboard::key::Named::ArrowUp)
                    | keyboard::Key::Named(keyboard::key::Named::PageUp) => {
                        return self.update(Message::PrevPage);
                    }
                    keyboard::Key::Named(keyboard::key::Named::Home) => {
                        return self.update(Message::GoToPage(0));
                    }
                    keyboard::Key::Named(keyboard::key::Named::End) => {
                        if let Some(doc) = &self.document {
                            let last_page = doc.page_count().saturating_sub(1);
                            return self.update(Message::GoToPage(last_page));
                        }
                    }
                    keyboard::Key::Character(c) => {
                        if modifiers.command() {
                            match c.as_ref() {
                                "o" => return self.update(Message::OpenFile),
                                "+" | "=" => return self.update(Message::ZoomIn),
                                "-" => return self.update(Message::ZoomOut),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }

            Message::ThumbnailsLoaded(thumbnails) => {
                self.thumbnails = thumbnails;
            }

            Message::PageRendered(page, handle) => {
                if page == self.current_page {
                    self.current_page_image = Some(handle);
                }
            }

            Message::ToggleFileMenu => {
                self.file_menu_open = !self.file_menu_open;
            }
        }

        Task::none()
    }

    fn render_current_page(&self) -> Task<Message> {
        if let Some(doc) = &self.document {
            let path = doc.path().to_string();
            let page = self.current_page;
            let zoom = self.zoom;

            Task::perform(
                async move {
                    let doc = Document::open(&path).ok()?;
                    let img = render_page(&doc, page, zoom).ok()?;
                    let handle = iced::widget::image::Handle::from_rgba(
                        img.width(),
                        img.height(),
                        img.into_raw(),
                    );
                    Some((page, handle))
                },
                |result| match result {
                    Some((page, handle)) => Message::PageRendered(page, handle),
                    None => Message::GoToPage(0), // Fallback, shouldn't happen
                },
            )
        } else {
            Task::none()
        }
    }

    fn load_thumbnails(&self) -> Task<Message> {
        if let Some(doc) = &self.document {
            let path = doc.path().to_string();
            let page_count = doc.page_count();

            Task::perform(
                async move {
                    let doc = match Document::open(&path) {
                        Ok(d) => d,
                        Err(_) => return Vec::new(),
                    };

                    let mut thumbnails = Vec::with_capacity(page_count);
                    for i in 0..page_count {
                        if let Ok(img) = render_thumbnail(&doc, i, 120) {
                            let width = img.width();
                            let height = img.height();
                            let handle = iced::widget::image::Handle::from_rgba(
                                width,
                                height,
                                img.into_raw(),
                            );
                            thumbnails.push(handle);
                        }
                    }
                    thumbnails
                },
                Message::ThumbnailsLoaded,
            )
        } else {
            Task::none()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let page_count = self.document.as_ref().map(|d| d.page_count()).unwrap_or(0);

        let menu_bar = menu_bar_view(self.file_menu_open);

        let sidebar = sidebar_view(&self.thumbnails, self.current_page, page_count);

        let viewer = viewer_view(
            self.current_page_image.as_ref(),
            self.current_page,
            page_count,
            self.zoom,
        );

        let main_content = row![sidebar, viewer].spacing(0);

        let content = column![menu_bar, main_content];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key, modifiers| {
            Some(Message::KeyPressed(key, modifiers))
        })
    }
}
