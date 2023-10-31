use crate::esp::records::record::{RecordType, SubRecord};
use nom::{
    combinator::{all_consuming, map, rest},
    Parser,
};

#[derive(Debug)]
pub struct DATA(Vec<u8>);

impl SubRecord for DATA {
    const TYPE: RecordType = RecordType::from_value(b"DATA");

    fn parse(input: &[u8]) -> nom::IResult<&[u8], Self> {
        map(rest, |rest: &[u8]| DATA(rest.to_vec()))(input)
    }
}

impl DATA {
    fn try_parse<'a, F, O>(&'a self, parser: F) -> Result<O, nom::Err<nom::error::Error<&'a [u8]>>>
    where
        F: Parser<&'a [u8], O, nom::error::Error<&'a [u8]>>,
    {
        let (_, value) = all_consuming(parser).parse(&self.0)?;
        Ok(value)
    }
}
