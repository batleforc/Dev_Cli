use kube::Error as KubeError;

#[derive(Debug)]
pub enum GetPodEnvvarsError {
    AttachError(GetAttachProcess),
    EmptyReturn,
    StdoutAttachError,
    IoError(std::io::Error),
}

#[derive(Debug)]
pub enum GetAttachProcess {
    AttachError(KubeError),
    EmptyCommand,
}
