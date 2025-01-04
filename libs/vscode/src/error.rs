#[derive(Debug)]
pub enum GetInstalledExtensionsError {
    EmptyReturn,
    StdoutAttachError,
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum InstallExtensionError {
    EmptyReturn,
    StdoutAttachError,
    FailedToInstallExtension(String),
    GetInstalledExtensionsError(GetInstalledExtensionsError),
    IoError(std::io::Error),
}
