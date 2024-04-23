// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub access_token: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `auth` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xee, 0x05, 0x0a, 0x0f, 0x61, 0x75, 0x74, 0x68, 0x2f, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x61, 0x75, 0x74, 0x68, 0x22, 0x27, 0x0a, 0x0f, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a,
    0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x6d,
    0x61, 0x69, 0x6c, 0x22, 0x24, 0x0a, 0x0c, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x05, 0x65, 0x6d, 0x61, 0x69, 0x6c, 0x22, 0x2a, 0x0a, 0x05, 0x54, 0x6f, 0x6b,
    0x65, 0x6e, 0x12, 0x21, 0x0a, 0x0c, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73,
    0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x32, 0x64, 0x0a, 0x04, 0x41, 0x75, 0x74, 0x68, 0x12, 0x30, 0x0a,
    0x08, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x12, 0x15, 0x2e, 0x61, 0x75, 0x74, 0x68,
    0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x1a, 0x0b, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x00, 0x12,
    0x2a, 0x0a, 0x05, 0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x12, 0x12, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x2e,
    0x4c, 0x6f, 0x67, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x0b, 0x2e, 0x61,
    0x75, 0x74, 0x68, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x22, 0x00, 0x42, 0x1b, 0x0a, 0x11, 0x69,
    0x6f, 0x2e, 0x67, 0x72, 0x70, 0x63, 0x2e, 0x61, 0x75, 0x74, 0x68, 0x2e, 0x41, 0x75, 0x74, 0x68,
    0x42, 0x04, 0x41, 0x75, 0x74, 0x68, 0x50, 0x01, 0x4a, 0xce, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00,
    0x00, 0x17, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x02, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x02,
    0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x03, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x03, 0x00, 0x2a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x04, 0x00,
    0x25, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x04, 0x00, 0x25, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x06, 0x00, 0x0d, 0x0a, 0x0a, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x08, 0x00,
    0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x0c, 0x0a, 0x0b,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x06, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x09, 0x10, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x09, 0x2a, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x06, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x0d, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x24, 0x29, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x0d, 0x00, 0x0f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x02,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x09, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x11, 0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x11, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x12, 0x02, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x12, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x12, 0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02,
    0x12, 0x04, 0x15, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x15,
    0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x16, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x18, 0x19, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x33,
];
include!("auth.serde.rs");
include!("auth.tonic.rs");
// @@protoc_insertion_point(module)