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
pub struct Rgba {
    #[prost(int32, tag = "1")]
    pub red: i32,
    #[prost(int32, tag = "2")]
    pub green: i32,
    #[prost(int32, tag = "3")]
    pub blue: i32,
    #[prost(int32, tag = "4")]
    pub alpha: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationPoint {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationCircle {
    #[prost(float, tag = "1")]
    pub radius: f32,
    #[prost(message, optional, tag = "2")]
    pub center: ::core::option::Option<Point>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationArrow {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<Point>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<Point>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationLine {
    #[prost(message, optional, tag = "1")]
    pub source: ::core::option::Option<Point>,
    #[prost(message, optional, tag = "2")]
    pub destination: ::core::option::Option<Point>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotation {
    #[prost(message, optional, tag = "5")]
    pub fill_color: ::core::option::Option<Rgba>,
    #[prost(message, optional, tag = "6")]
    pub shape_color: ::core::option::Option<Rgba>,
    #[prost(oneof = "annotation::AnnotationType", tags = "1, 2, 3, 4")]
    pub annotation_type: ::core::option::Option<annotation::AnnotationType>,
}
/// Nested message and enum types in `Annotation`.
pub mod annotation {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AnnotationType {
        #[prost(message, tag = "1")]
        Point(super::AnnotationPoint),
        #[prost(message, tag = "2")]
        Circle(super::AnnotationCircle),
        #[prost(message, tag = "3")]
        Line(super::AnnotationLine),
        #[prost(message, tag = "4")]
        Arrow(super::AnnotationArrow),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Add {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub annotation: ::core::option::Option<Annotation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Remove {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveMultiple {
    #[prost(string, repeated, tag = "1")]
    pub id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clear {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotationWrapper {
    #[prost(oneof = "annotation_wrapper::Actions", tags = "1, 2, 3, 4")]
    pub actions: ::core::option::Option<annotation_wrapper::Actions>,
}
/// Nested message and enum types in `AnnotationWrapper`.
pub mod annotation_wrapper {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Actions {
        #[prost(message, tag = "1")]
        Add(super::Add),
        #[prost(message, tag = "2")]
        Remove(super::Remove),
        #[prost(message, tag = "3")]
        RemoveMultiple(super::RemoveMultiple),
        #[prost(message, tag = "4")]
        Clear(super::Clear),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Field {
    #[prost(float, tag = "1")]
    pub length: f32,
    #[prost(float, tag = "2")]
    pub width: f32,
    #[prost(float, tag = "3")]
    pub center_radius: f32,
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
pub struct Robot {
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
pub struct Ball {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwarePacket {
    #[prost(message, optional, tag = "1")]
    pub field: ::core::option::Option<Field>,
    #[prost(enumeration = "Color", tag = "2")]
    pub color_team: i32,
    #[prost(message, repeated, tag = "3")]
    pub allies: ::prost::alloc::vec::Vec<Robot>,
    #[prost(message, repeated, tag = "4")]
    pub opponents: ::prost::alloc::vec::Vec<Robot>,
    #[prost(message, optional, tag = "5")]
    pub ball: ::core::option::Option<Ball>,
    #[prost(message, repeated, tag = "6")]
    pub annotations: ::prost::alloc::vec::Vec<AnnotationWrapper>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Color {
    Yellow = 0,
    Blue = 1,
}
impl Color {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Color::Yellow => "YELLOW",
            Color::Blue => "BLUE",
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
