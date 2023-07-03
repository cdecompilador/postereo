#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An unexpected error occured, the fetching from the api failed
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),

    ///
    #[error("api error {0:?}")]
    Api(shared::Error)
}
