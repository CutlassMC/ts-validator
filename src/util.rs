pub(crate) type Result<T = (), E = anyhow::Error> = core::result::Result<T, E>;

pub(crate) trait TryResolve<T, E>: Sized {
    fn try_resolve<U, F: FnOnce(E) -> Result<T, U>>(self, f: F) -> Result<T, U>;
}

impl<T, E> TryResolve<T, E> for Result<T, E> {
    fn try_resolve<U, F: FnOnce(E) -> Result<T, U>>(self, f: F) -> Result<T, U> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => f(e),
        }
    }
}

pub(crate) static STDIN_PATHBUF: std::sync::LazyLock<std::path::PathBuf> =
    std::sync::LazyLock::new(|| std::path::PathBuf::from("<stdin>"));

pub(crate) trait OrStdin: Sized {
    fn or_stdin(&self) -> &std::path::PathBuf;
}

impl OrStdin for Option<&std::path::PathBuf> {
    fn or_stdin(&self) -> &std::path::PathBuf {
        self.unwrap_or(&*STDIN_PATHBUF)
    }
}
