mod error;

pub use error::Error;

pub(crate) use error::Result;

pub(crate) trait Parseable: Sized {
    type Parent;

    fn parse(parent: &Self::Parent, index: u16) -> Result<Self>;

    fn prefix() -> &'static str;
}
