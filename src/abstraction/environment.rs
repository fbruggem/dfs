use crate::abstraction::file_system::FileSystem;

pub struct Environment<FS: FileSystem> {
    file_system: FS,
}
