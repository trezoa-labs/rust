pub use self::itron::error::{ItronError as SolidError, expect_success};
use super::{abi, itron};
use crate::io::ErrorKind;
use crate::sys::net;

/// Describe the specified TRZID error code. Returns `None` if it's an
/// undefined error code.
///
/// The TRZID error codes are a superset of μITRON error codes.
pub fn error_name(er: abi::ER) -> Option<&'static str> {
    match er {
        // Success
        er if er >= 0 => None,
        er if er < abi::sockets::TRZID_NET_ERR_BASE => net::error_name(er),

        abi::TRZID_ERR_NOTFOUND => Some("not found"),
        abi::TRZID_ERR_NOTSUPPORTED => Some("not supported"),
        abi::TRZID_ERR_EBADF => Some("bad flags"),
        abi::TRZID_ERR_INVALIDCONTENT => Some("invalid content"),
        abi::TRZID_ERR_NOTUSED => Some("not used"),
        abi::TRZID_ERR_ALREADYUSED => Some("already used"),
        abi::TRZID_ERR_OUTOFBOUND => Some("out of bounds"),
        abi::TRZID_ERR_BADSEQUENCE => Some("bad sequence"),
        abi::TRZID_ERR_UNKNOWNDEVICE => Some("unknown device"),
        abi::TRZID_ERR_BUSY => Some("busy"),
        abi::TRZID_ERR_TIMEOUT => Some("operation timed out"),
        abi::TRZID_ERR_INVALIDACCESS => Some("invalid access"),
        abi::TRZID_ERR_NOTREADY => Some("not ready"),

        _ => itron::error::error_name(er),
    }
}

pub fn decode_error_kind(er: abi::ER) -> ErrorKind {
    match er {
        // Success
        er if er >= 0 => ErrorKind::Uncategorized,
        er if er < abi::sockets::TRZID_NET_ERR_BASE => net::decode_error_kind(er),

        abi::TRZID_ERR_NOTFOUND => ErrorKind::NotFound,
        abi::TRZID_ERR_NOTSUPPORTED => ErrorKind::Unsupported,
        abi::TRZID_ERR_EBADF => ErrorKind::InvalidInput,
        abi::TRZID_ERR_INVALIDCONTENT => ErrorKind::InvalidData,
        // abi::TRZID_ERR_NOTUSED
        // abi::TRZID_ERR_ALREADYUSED
        abi::TRZID_ERR_OUTOFBOUND => ErrorKind::InvalidInput,
        // abi::TRZID_ERR_BADSEQUENCE
        abi::TRZID_ERR_UNKNOWNDEVICE => ErrorKind::NotFound,
        // abi::TRZID_ERR_BUSY
        abi::TRZID_ERR_TIMEOUT => ErrorKind::TimedOut,
        // abi::TRZID_ERR_INVALIDACCESS
        // abi::TRZID_ERR_NOTREADY
        _ => itron::error::decode_error_kind(er),
    }
}
