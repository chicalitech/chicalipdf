use image::{ImageBuffer, Rgba, RgbaImage};
use mupdf::Matrix;

use super::document::{Document, DocumentError};

pub fn render_page(
    document: &Document,
    page_index: usize,
    scale: f32,
) -> Result<RgbaImage, DocumentError> {
    let page = document.get_page(page_index)?;

    let matrix = Matrix::new_scale(scale, scale);
    let pixmap = page
        .to_pixmap(&matrix, &mupdf::Colorspace::device_rgb(), 1.0, true)
        .map_err(|e| DocumentError::RenderFailed(e.to_string()))?;

    let width = pixmap.width() as u32;
    let height = pixmap.height() as u32;
    let samples = pixmap.samples();

    // MuPDF returns RGBA data
    let img: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let idx = ((y * width + x) * 4) as usize;
        if idx + 3 < samples.len() {
            Rgba([samples[idx], samples[idx + 1], samples[idx + 2], samples[idx + 3]])
        } else {
            Rgba([255, 255, 255, 255])
        }
    });

    Ok(img)
}

pub fn render_thumbnail(
    document: &Document,
    page_index: usize,
    max_width: u32,
) -> Result<RgbaImage, DocumentError> {
    let page = document.get_page(page_index)?;
    let bounds = page
        .bounds()
        .map_err(|e| DocumentError::RenderFailed(e.to_string()))?;

    let page_width = bounds.x1 - bounds.x0;
    let scale = max_width as f32 / page_width;

    render_page(document, page_index, scale)
}
