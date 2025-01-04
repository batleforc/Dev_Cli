#[derive(Debug)]
pub enum GetInstalledExtensionsError {
    EmptyReturn,
    StdoutAttachError,
    IoError(std::io::Error),
}
