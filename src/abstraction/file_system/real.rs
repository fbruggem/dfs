#[derive(Clone, Debug)]
pub struct StdFs {
    root: PathBuf,
}

impl StdFs {
    /// Create a filesystem rooted at `root`. The directory is created
    /// if it doesn't exist. All `open` / `delete` paths are joined to
    /// this root (and forced to be relative).
    pub fn new(root: impl AsRef<Path>) -> io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }

    /// Resolve a caller-supplied path under `root`, stripping leading
    /// separators so an absolute-looking path can't escape the sandbox
    /// via the std `Path::join` "absolute wins" rule.
    ///
    /// NOTE: this does *not* defend against `..` traversal. For
    /// production use, canonicalize the result and check it's still a
    /// prefix of `root`, or use `openat2`/`O_BENEATH` on Linux.
    fn resolve(&self, path: &str) -> PathBuf {
        let rel = path.trim_start_matches(|c: char| c == '/' || c == '\\');
        self.root.join(rel)
    }
}

impl FileSystem for StdFs {
    type File = StdFile;

    fn open(&self, path: &str, mode: OpenMode) -> io::Result<StdFile> {
        let full = self.resolve(path);
        if let Some(parent) = full.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut opts = std::fs::OpenOptions::new();
        match mode {
            OpenMode::Read => {
                opts.read(true);
            }
            OpenMode::Write => {
                opts.write(true).create(true).truncate(true);
            }
            OpenMode::ReadWrite => {
                opts.read(true).write(true).create(true);
            }
            OpenMode::Append => {
                opts.append(true).create(true);
            }
        }
        Ok(StdFile {
            inner: opts.open(full)?,
        })
    }

    fn delete(&self, path: &str) -> io::Result<()> {
        std::fs::remove_file(self.resolve(path))
    }
}

#[derive(Debug)]
pub struct StdFile {
    inner: std::fs::File,
}

impl File for StdFile {
    fn close(self) -> io::Result<()> {
        // If you care about flush/sync errors at close time, call
        // self.inner.sync_all()? before dropping.
        drop(self.inner);
        Ok(())
    }

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        io::Read::read(&mut self.inner, buf)
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        io::Write::write(&mut self.inner, buf)
    }

    fn lseek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        io::Seek::seek(&mut self.inner, pos)
    }
}
