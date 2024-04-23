mod error;

pub use error::Error;

pub(crate) use error::Result;

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> Result<Self>;

    fn prefix() -> &'static str;
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
pub(crate) trait AsyncParseable: Sized {
    type Parent;

    async fn parse(parent: &Self::Parent, index: u16) -> Result<Self>;

    fn prefix() -> &'static str;
}
