/// `Result` with default types.
pub type AnyRes<T = (), E = anyhow::Error> = Result<T, E>;
