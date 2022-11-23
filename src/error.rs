use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Other(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    WebView(#[from] tauri::Error),

    #[error(transparent)]
    Threading(#[from] std::sync::mpsc::RecvError),

    #[error(transparent)]
    Surface(#[from] webkit2gtk::Error),

    #[error("the data for key `{0}` is not available")]
    ToImageSurface(String),

    #[error(transparent)]
    CairoIo(#[from] cairo::IoError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
