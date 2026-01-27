use std::path::Path;

#[derive(Debug)]
pub enum DocumentError {
    OpenFailed(String),
    PageOutOfBounds(usize),
    RenderFailed(String),
}

impl std::fmt::Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentError::OpenFailed(msg) => write!(f, "Failed to open document: {}", msg),
            DocumentError::PageOutOfBounds(idx) => write!(f, "Page {} is out of bounds", idx),
            DocumentError::RenderFailed(msg) => write!(f, "Failed to render page: {}", msg),
        }
    }
}

impl std::error::Error for DocumentError {}

pub struct Document {
    inner: mupdf::Document,
    path: String,
}

impl Document {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, DocumentError> {
        let path_str = path.as_ref().to_string_lossy().to_string();
        let inner = mupdf::Document::open(&path_str)
            .map_err(|e| DocumentError::OpenFailed(e.to_string()))?;

        Ok(Self {
            inner,
            path: path_str,
        })
    }

    pub fn page_count(&self) -> usize {
        self.inner.page_count().unwrap_or(0) as usize
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn get_page(&self, index: usize) -> Result<mupdf::Page, DocumentError> {
        if index >= self.page_count() {
            return Err(DocumentError::PageOutOfBounds(index));
        }
        self.inner
            .load_page(index as i32)
            .map_err(|e| DocumentError::RenderFailed(e.to_string()))
    }
}

impl Clone for Document {
    fn clone(&self) -> Self {
        // Re-open the document for cloning
        // This is safe since we store the path
        Self::open(&self.path).expect("Failed to clone document")
    }
}
