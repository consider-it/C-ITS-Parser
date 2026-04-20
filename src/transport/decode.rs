use crate::transport::{BasicTransportAHeader, BasicTransportBHeader, IPv6Header};
use etherparse::PacketHeaders;
use nom::{
    bytes::streaming::take,
    combinator::{into, map_res},
    error::{ErrorKind, FromExternalError, ParseError},
    sequence::pair,
};

#[derive(Debug, PartialEq)]
pub enum DecodeError<I> {
    IntegerError(String),
    IPv6Parsing(String),
    Nom(I, ErrorKind),
    #[cfg(feature = "json")]
    Json(String),
}

impl<I> ParseError<I> for DecodeError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        DecodeError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I, E> FromExternalError<I, E> for DecodeError<I> {
    fn from_external_error(input: I, kind: ErrorKind, _: E) -> Self {
        DecodeError::Nom(input, kind)
    }
}

pub type IResult<I, T> = nom::IResult<I, T, DecodeError<I>>;

pub trait Decode: Sized {
    /// Decoder trait for decoding the individual fields of the transport header
    /// Takes byte slice as input.
    /// ### Usage
    /// ```rust
    /// # use etsi_transports::*;
    /// let input: &'static [u8] = &[0,1,0,2];
    /// let (_remaining_input, decoded) = BasicTransportAHeader::decode(input).unwrap();
    /// assert_eq!(
    ///     decoded,
    ///     BasicTransportAHeader {
    ///         destination_port: 1,
    ///         source_port: 2,
    ///     }
    /// );
    /// ```
    fn decode(input: &[u8]) -> IResult<&[u8], Self>;
}

impl Decode for BasicTransportAHeader {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        into(pair(u16_from_be_bytes, u16_from_be_bytes))(input)
    }
}

impl From<(u16, u16)> for BasicTransportAHeader {
    fn from(value: (u16, u16)) -> Self {
        Self {
            destination_port: value.0,
            source_port: value.1,
        }
    }
}

impl BasicTransportAHeader {
    #[cfg(feature = "json")]
    /// Decodes a BTP-A header from JSON
    ///
    /// # Errors
    /// Returns an error when parsing failed
    pub fn decode_from_json(input: &str) -> Result<Self, DecodeError<&str>> {
        serde_json::from_str(input)
            .map_err(|e| DecodeError::Json(format!("Error encoding to JSON: {e:?}")))
    }
}

impl Decode for BasicTransportBHeader {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        into(pair(u16_from_be_bytes, u16_from_be_bytes))(input)
    }
}

impl From<(u16, u16)> for BasicTransportBHeader {
    fn from(value: (u16, u16)) -> Self {
        Self {
            destination_port: value.0,
            destination_port_info: value.1,
        }
    }
}

impl BasicTransportBHeader {
    #[cfg(feature = "json")]
    /// Decodes a BTP-B header from JSON
    ///
    /// # Errors
    /// Returns an error when parsing failed
    pub fn decode_from_json(input: &str) -> Result<Self, DecodeError<&str>> {
        serde_json::from_str(input)
            .map_err(|e| DecodeError::Json(format!("Error encoding to JSON: {e:?}")))
    }
}

fn u16_from_be_bytes(input: &[u8]) -> IResult<&[u8], u16> {
    map_res(take(2usize), |slice: &[u8]| {
        slice.try_into().map(u16::from_be_bytes).map_err(|e| {
            DecodeError::IntegerError::<&[u8]>(format!(
                "Failed to construct integer from bytes: {e:?}"
            ))
        })
    })(input)
}

impl Decode for IPv6Header {
    fn decode(input: &[u8]) -> IResult<&[u8], Self> {
        etherparse::PacketHeaders::from_ip_slice(input)
            .map(|headers| {
                let first_after_headers = headers
                    .net
                    .as_ref()
                    .map_or(0, etherparse::NetHeaders::header_len)
                    + headers
                        .link
                        .as_ref()
                        .map_or(0, etherparse::LinkHeader::header_len)
                    + headers
                        .transport
                        .as_ref()
                        .map_or(0, etherparse::TransportHeader::header_len)
                    + headers
                        .link_exts
                        .first()
                        .map_or(0, etherparse::LinkExtHeader::header_len);
                (&input[first_after_headers..], IPv6Header::from(headers))
            })
            .map_err(|e| {
                nom::Err::Error(DecodeError::IPv6Parsing(format!(
                    "Error parsing IPv6 Header: {e:?}"
                )))
            })
    }
}

impl From<PacketHeaders<'_>> for IPv6Header {
    fn from(value: PacketHeaders<'_>) -> Self {
        Self {
            ip: value.net,
            link: value.link,
            transport: value.transport,
            link_ext: value.link_exts.first().cloned(),
        }
    }
}

#[cfg(test)]
mod tests {}
