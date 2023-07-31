// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub timestamp: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub log_index: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transfers {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<Transfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub decimal: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(enumeration="ActionType", tag="1")]
    pub action_type: i32,
    #[prost(string, tag="9")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub transfer: ::core::option::Option<Transfer>,
    #[prost(string, tag="6")]
    pub tx_hash: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub block_number: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub timestamp: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ActionType {
    Wrap = 0,
    Unwrap = 1,
    Send = 2,
    Approve = 3,
    Other = 4,
}
impl ActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ActionType::Wrap => "wrap",
            ActionType::Unwrap => "unwrap",
            ActionType::Send => "send",
            ActionType::Approve => "approve",
            ActionType::Other => "other",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "wrap" => Some(Self::Wrap),
            "unwrap" => Some(Self::Unwrap),
            "send" => Some(Self::Send),
            "approve" => Some(Self::Approve),
            "other" => Some(Self::Other),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
