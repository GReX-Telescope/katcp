//! Standard set of imports for katcp.
//! This is useful to `use katcp::prelude::*;` to satisfy all the imports
//! for deriving `KatcpMessage` from the `katcp_derive` trait

pub use crate::{
    messages::common::{
        FromKatcpArgument, FromKatcpArguments, KatcpArgument, KatcpMessage, RetCode,
        ToKatcpArgument, ToKatcpArguments,
    },
    protocol::{KatcpError, Message, MessageKind, MessageResult},
};
