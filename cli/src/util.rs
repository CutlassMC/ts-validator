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
