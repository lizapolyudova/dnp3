/// prelude specifically for using the master API
pub mod master {
    pub use crate::app::enums::*;
    pub use crate::app::parse::DecodeLogLevel;
    pub use crate::app::retry::*;
    pub use crate::app::timeout::Timeout;
    pub use crate::app::variations::*;
    pub use crate::entry::master::serial::*;
    pub use crate::entry::master::tcp::*;
    pub use crate::entry::master::*;
    pub use crate::entry::EndpointAddress;
    pub use crate::master::association::*;
    pub use crate::master::handle::*;
    pub use crate::master::request::*;
}
