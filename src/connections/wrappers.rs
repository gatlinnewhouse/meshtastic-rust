use crate::errors_internal::Error;

/// A helper struct representing the ID of a node in the mesh.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u32> for NodeId {
    fn eq(&self, other: &u32) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u32> for NodeId {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl NodeId {
    /// Creates a new `NodeId` from a `u32`.
    pub fn new(id: u32) -> NodeId {
        NodeId(id)
    }

    /// Returns the `u32` id of the `NodeId`.
    pub fn id(&self) -> u32 {
        self.0
    }
}

impl From<u32> for NodeId {
    fn from(value: u32) -> Self {
        NodeId(value)
    }
}

pub mod encoded_data {
    use bytes::{Bytes, BytesMut};

    /// A struct that represents incoming encoded data from a radio connection.
    /// The wrapped data may contain a whole packet, a partial packet, or multiple packets.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct IncomingStreamData(Bytes);

    impl std::fmt::Display for IncomingStreamData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl IncomingStreamData {
        /// Creates a new `IncomingStreamData` struct from a `BytesMut`.
        pub fn new(data: Bytes) -> IncomingStreamData {
            IncomingStreamData(data)
        }

        /// Transfers ownership of the `Bytes` data contained within the `IncomingStreamData` struct.
        pub fn data_bytes(self) -> Bytes {
            self.0
        }

        /// Returns a reference to the slice of the `Bytes`
        pub fn as_slice(&self) -> &[u8] {
            &self.0
        }
    }

    impl From<Bytes> for IncomingStreamData {
        fn from(value: Bytes) -> Self {
            IncomingStreamData(value)
        }
    }

    impl AsRef<[u8]> for IncomingStreamData {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    /// A struct that represents encoded binary data that will be used within the `protobufs::Data`
    /// field of an outgoing `protobufs::MeshPacket`.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct EncodedMeshPacketData(Bytes);

    impl std::fmt::Display for EncodedMeshPacketData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl EncodedMeshPacketData {
        /// Creates a new `EncodedMeshPacketData` struct from a `Vec<u8>`.
        pub fn new(data: Bytes) -> EncodedMeshPacketData {
            EncodedMeshPacketData(data)
        }

        /// Transfers ownership of the `Bytes` data contained within the `EncodedMeshPacketData` struct.
        pub fn data_bytes(self) -> Bytes {
            self.0
        }

        /// Returns a reference to the slice of the `Bytes`
        pub fn as_slice(&self) -> &[u8] {
            &self.0
        }
    }

    impl From<Bytes> for EncodedMeshPacketData {
        fn from(value: Bytes) -> Self {
            EncodedMeshPacketData(value)
        }
    }

    /// A struct that represents the binary encoding of an outgoing `protobufs::ToRadio` packet.
    /// This data **does not** include a packet header.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct EncodedToRadioPacket(BytesMut);

    impl std::fmt::Display for EncodedToRadioPacket {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl EncodedToRadioPacket {
        /// Creates a new `EncodedToRadioPacket` struct from a `BytesMut`.
        pub fn new(data: BytesMut) -> EncodedToRadioPacket {
            EncodedToRadioPacket(data)
        }

        /// Returns the `BytesMut` as `Bytes` data contained within the `EncodedToRadioPacket` struct.
        pub fn data_bytes(self) -> Bytes {
            self.0.freeze()
        }

        /// Returns a reference to the slice of the `BytesMut`
        pub fn as_slice(&self) -> &[u8] {
            &self.0
        }
    }

    impl From<BytesMut> for EncodedToRadioPacket {
        fn from(value: BytesMut) -> Self {
            EncodedToRadioPacket(value)
        }
    }

    /// A struct that represents the binary encoding of an outgoing `protobufs::ToRadio` packet.
    /// This encoding can be sent to a radio, as it includes the required 4-byte packet header.
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct EncodedToRadioPacketWithHeader(BytesMut);

    impl std::fmt::Display for EncodedToRadioPacketWithHeader {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl EncodedToRadioPacketWithHeader {
        /// Creates a new `EncodedToRadioPacketWithHeader` struct from a `BytesMut`.
        pub fn new(data: BytesMut) -> EncodedToRadioPacketWithHeader {
            EncodedToRadioPacketWithHeader(data)
        }

        /// Returns a reference to the `BytesMut` data contained within the `EncodedToRadioPacketWithHeader` struct.
        pub fn data(&self) -> &[u8] {
            &self.0
        }

        /// Returns the `BytesMut` as `Bytes` data contained within the `EncodedToRadioPacketWithHeader` struct.
        pub fn data_bytes(self) -> Bytes {
            self.0.freeze()
        }

        /// Returns a reference to the slice of the `Bytes`
        pub fn as_slice(&self) -> &[u8] {
            &self.0
        }
    }

    impl From<BytesMut> for EncodedToRadioPacketWithHeader {
        fn from(value: BytesMut) -> Self {
            EncodedToRadioPacketWithHeader(value)
        }
    }
}

pub mod mesh_channel {
    use super::*;

    /// A struct that represents a messaging channel on the mesh. This struct is used to
    /// limit the valid channel indices to be in the range [0..7].
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MeshChannel(u32);

    impl std::fmt::Display for MeshChannel {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl MeshChannel {
        /// Creates a new `MeshChannel` struct from a `u32`. This method will return an error
        /// if the passed `u32` is not in the range [0..7].
        pub fn new(channel: u32) -> Result<MeshChannel, Error> {
            if !(0..=7).contains(&channel) {
                return Err(Error::InvalidChannelIndex { channel });
            }

            Ok(MeshChannel(channel))
        }

        /// Returns the `u32` channel index of the `MeshChannel`.
        pub fn channel(&self) -> u32 {
            self.0
        }
    }

    impl From<u32> for MeshChannel {
        fn from(channel: u32) -> Self {
            MeshChannel(channel)
        }
    }
}
