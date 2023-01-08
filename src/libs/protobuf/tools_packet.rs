#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolsField {
    #[prost(float, tag = "1")]
    pub length: f32,
    #[prost(float, tag = "2")]
    pub width: f32,
    #[prost(float, tag = "3")]
    pub radius_center: f32,
    #[prost(float, tag = "4")]
    pub goal_width: f32,
    #[prost(float, tag = "5")]
    pub goal_depth: f32,
    #[prost(float, tag = "6")]
    pub penalty_width: f32,
    #[prost(float, tag = "7")]
    pub penalty_depth: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolsRobot {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(float, tag = "2")]
    pub x: f32,
    #[prost(float, tag = "3")]
    pub y: f32,
    #[prost(float, tag = "4")]
    pub orientation: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolsBall {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolsSoftwarePacket {
    #[prost(message, optional, tag = "1")]
    pub field: ::core::option::Option<ToolsField>,
    #[prost(enumeration = "ToolsColor", tag = "2")]
    pub color_team: i32,
    #[prost(message, repeated, tag = "3")]
    pub allies: ::prost::alloc::vec::Vec<ToolsRobot>,
    #[prost(message, repeated, tag = "4")]
    pub opponents: ::prost::alloc::vec::Vec<ToolsRobot>,
    #[prost(message, optional, tag = "5")]
    pub ball: ::core::option::Option<ToolsBall>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ToolsColor {
    Yellow = 0,
    Blue = 1,
}
impl ToolsColor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ToolsColor::Yellow => "YELLOW",
            ToolsColor::Blue => "BLUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "YELLOW" => Some(Self::Yellow),
            "BLUE" => Some(Self::Blue),
            _ => None,
        }
    }
}
