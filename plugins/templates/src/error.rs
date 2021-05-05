use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    TemplateError(handlebars::TemplateError),
    RenderError(handlebars::RenderError),
    TemplateRenderError(handlebars::TemplateRenderError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<handlebars::TemplateError> for Error {
    fn from(err: handlebars::TemplateError) -> Self {
        Self::TemplateError(err)
    }
}

impl From<handlebars::RenderError> for Error {
    fn from(err: handlebars::RenderError) -> Self {
        Self::RenderError(err)
    }
}

impl From<handlebars::TemplateRenderError> for Error {
    fn from(err: handlebars::TemplateRenderError) -> Self {
        Self::TemplateRenderError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
