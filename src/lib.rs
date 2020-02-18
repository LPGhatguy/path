use std::path::{Path, PathBuf};

macro_rules! path_traits {
    ( $struct: ident) => {
        impl AsRef<Path> for $struct {
            fn as_ref(&self) -> &Path {
                &self.inner
            }
        }
    };
}

macro_rules! path_buf_traits {
    ( $struct: ident ) => {
        impl AsRef<PathBuf> for $struct {
            fn as_ref(&self) -> &PathBuf {
                &self.inner
            }
        }

        impl AsMut<PathBuf> for $struct {
            fn as_mut(&mut self) -> &mut PathBuf {
                &mut self.inner
            }
        }
    };
}

/// A wrapper around [`PathBuf`][std::path::PathBuf] that is guaranteed to be an
/// absolute path.
///
/// [std::path::PathBuf]: https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
pub struct AbsolutePathBuf {
    inner: PathBuf,
}

path_traits!(AbsolutePathBuf);
path_buf_traits!(AbsolutePathBuf);

/// A wrapper around [`PathBuf`][std::path::PathBuf] that is guaranteed to be a
/// relative path.
///
/// [std::path::PathBuf]: https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html
pub struct RelativePathBuf {
    inner: PathBuf,
}

path_traits!(RelativePathBuf);
path_buf_traits!(RelativePathBuf);

/// A wrapper around [`Path`][std::path::Path] that is guaranteed to be an
/// absolute path.
///
/// [std::path::Path]: https://doc.rust-lang.org/stable/std/path/struct.Path.html
pub struct AbsolutePath {
    inner: Path,
}

path_traits!(AbsolutePath);

/// A wrapper around [`Path`][std::path::Path] that is guaranteed to be a
/// relative path.
///
/// [std::path::Path]: https://doc.rust-lang.org/stable/std/path/struct.Path.html
pub struct RelativePath {
    inner: Path,
}

path_traits!(RelativePath);
