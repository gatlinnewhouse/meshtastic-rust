///
/// This information can be encoded as a QRcode/url so that other users can configure
/// their radio to join the same channel.
/// A note about how channel names are shown to users: channelname-X
/// poundsymbol is a prefix used to indicate this is a channel name (idea from @professr).
/// Where X is a letter from A-Z (base 26) representing a hash of the PSK for this
/// channel - so that if the user changes anything about the channel (which does
/// force a new PSK) this letter will also change. Thus preventing user confusion if
/// two friends try to type in a channel name of "BobsChan" and then can't talk
/// because their PSKs will be different.
/// The PSK is hashed into this letter by "0x41 + [xor all bytes of the psk ] modulo 26"
/// This also allows the option of someday if people have the PSK off (zero), the
/// users COULD type in a channel name and be able to talk.
/// FIXME: Add description of multi-channel support and how primary vs secondary channels are used.
/// FIXME: explain how apps use channels for security.
/// explain how remote settings and remote gpio are managed as an example
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSettings {
    ///
    /// Deprecated in favor of LoraConfig.channel_num
    #[deprecated]
    #[prost(uint32, tag = "1")]
    pub channel_num: u32,
    ///
    /// A simple pre-shared key for now for crypto.
    /// Must be either 0 bytes (no crypto), 16 bytes (AES128), or 32 bytes (AES256).
    /// A special shorthand is used for 1 byte long psks.
    /// These psks should be treated as only minimally secure,
    /// because they are listed in this source code.
    /// Those bytes are mapped using the following scheme:
    /// `0` = No crypto
    /// `1` = The special "default" channel key: {0xd4, 0xf1, 0xbb, 0x3a, 0x20, 0x29, 0x07, 0x59, 0xf0, 0xbc, 0xff, 0xab, 0xcf, 0x4e, 0x69, 0x01}
    /// `2` through 10 = The default channel key, except with 1 through 9 added to the last byte.
    /// Shown to user as simple1 through 10
    #[prost(bytes = "vec", tag = "2")]
    pub psk: ::prost::alloc::vec::Vec<u8>,
    ///
    /// A SHORT name that will be packed into the URL.
    /// Less than 12 bytes.
    /// Something for end users to call the channel
    /// If this is the empty string it is assumed that this channel
    /// is the special (minimally secure) "Default"channel.
    /// In user interfaces it should be rendered as a local language translation of "X".
    /// For channel_num hashing empty string will be treated as "X".
    /// Where "X" is selected based on the English words listed above for ModemPreset
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Used to construct a globally unique channel ID.
    /// The full globally unique ID will be: "name.id" where ID is shown as base36.
    /// Assuming that the number of meshtastic users is below 20K (true for a long time)
    /// the chance of this 64 bit random number colliding with anyone else is super low.
    /// And the penalty for collision is low as well, it just means that anyone trying to decrypt channel messages might need to
    /// try multiple candidate channels.
    /// Any time a non wire compatible change is made to a channel, this field should be regenerated.
    /// There are a small number of 'special' globally known (and fairly) insecure standard channels.
    /// Those channels do not have a numeric id included in the settings, but instead it is pulled from
    /// a table of well known IDs.
    /// (see Well Known Channels FIXME)
    #[prost(fixed32, tag = "4")]
    pub id: u32,
    ///
    /// If true, messages on the mesh will be sent to the *public* internet by any gateway ndoe
    #[prost(bool, tag = "5")]
    pub uplink_enabled: bool,
    ///
    /// If true, messages seen on the internet will be forwarded to the local mesh.
    #[prost(bool, tag = "6")]
    pub downlink_enabled: bool,
    ///
    /// Per-channel module settings.
    #[prost(message, optional, tag = "7")]
    pub module_settings: ::core::option::Option<ModuleSettings>,
}
///
/// This message is specifically for modules to store per-channel configuration data.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleSettings {
    ///
    /// Bits of precision for the location sent in position packets.
    #[prost(uint32, tag = "1")]
    pub position_precision: u32,
    ///
    /// Controls whether or not the phone / clients should mute the current channel
    /// Useful for noisy public channels you don't necessarily want to disable
    #[prost(bool, tag = "2")]
    pub is_client_muted: bool,
}
///
/// A pair of a channel number, mode and the (sharable) settings for that channel
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    ///
    /// The index of this channel in the channel table (from 0 to MAX_NUM_CHANNELS-1)
    /// (Someday - not currently implemented) An index of -1 could be used to mean "set by name",
    /// in which case the target node will find and set the channel by settings.name.
    #[prost(int32, tag = "1")]
    pub index: i32,
    ///
    /// The new settings, or NULL to disable that channel
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<ChannelSettings>,
    ///
    /// TODO: REPLACE
    #[prost(enumeration = "channel::Role", tag = "3")]
    pub role: i32,
}
/// Nested message and enum types in `Channel`.
pub mod channel {
    ///
    /// How this channel is being used (or not).
    /// Note: this field is an enum to give us options for the future.
    /// In particular, someday we might make a 'SCANNING' option.
    /// SCANNING channels could have different frequencies and the radio would
    /// occasionally check that freq to see if anything is being transmitted.
    /// For devices that have multiple physical radios attached, we could keep multiple PRIMARY/SCANNING channels active at once to allow
    /// cross band routing as needed.
    /// If a device has only a single radio (the common case) only one channel can be PRIMARY at a time
    /// (but any number of SECONDARY channels can't be sent received on that common frequency)
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Role {
        ///
        /// This channel is not in use right now
        Disabled = 0,
        ///
        /// This channel is used to set the frequency for the radio - all other enabled channels must be SECONDARY
        Primary = 1,
        ///
        /// Secondary channels are only used for encryption/decryption/authentication purposes.
        /// Their radio settings (freq etc) are ignored, only psk is used.
        Secondary = 2,
    }
    impl Role {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Role::Disabled => "DISABLED",
                Role::Primary => "PRIMARY",
                Role::Secondary => "SECONDARY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DISABLED" => Some(Self::Disabled),
                "PRIMARY" => Some(Self::Primary),
                "SECONDARY" => Some(Self::Secondary),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceUiConfig {
    ///
    /// A version integer used to invalidate saved files when we make incompatible changes.
    #[prost(uint32, tag = "1")]
    pub version: u32,
    ///
    /// TFT display brightness 1..255
    #[prost(uint32, tag = "2")]
    pub screen_brightness: u32,
    ///
    /// Screen timeout 0..900
    #[prost(uint32, tag = "3")]
    pub screen_timeout: u32,
    ///
    /// Screen/Settings lock enabled
    #[prost(bool, tag = "4")]
    pub screen_lock: bool,
    #[prost(bool, tag = "5")]
    pub settings_lock: bool,
    #[prost(uint32, tag = "6")]
    pub pin_code: u32,
    ///
    /// Color theme
    #[prost(enumeration = "Theme", tag = "7")]
    pub theme: i32,
    ///
    /// Audible message, banner and ring tone
    #[prost(bool, tag = "8")]
    pub alert_enabled: bool,
    #[prost(bool, tag = "9")]
    pub banner_enabled: bool,
    #[prost(uint32, tag = "10")]
    pub ring_tone_id: u32,
    ///
    /// Localization
    #[prost(enumeration = "Language", tag = "11")]
    pub language: i32,
    ///
    /// Node list filter
    #[prost(message, optional, tag = "12")]
    pub node_filter: ::core::option::Option<NodeFilter>,
    ///
    /// Node list highlightening
    #[prost(message, optional, tag = "13")]
    pub node_highlight: ::core::option::Option<NodeHighlight>,
    ///
    /// 8 integers for screen calibration data
    #[prost(bytes = "vec", tag = "14")]
    pub calibration_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeFilter {
    ///
    /// Filter unknown nodes
    #[prost(bool, tag = "1")]
    pub unknown_switch: bool,
    ///
    /// Filter offline nodes
    #[prost(bool, tag = "2")]
    pub offline_switch: bool,
    ///
    /// Filter nodes w/o public key
    #[prost(bool, tag = "3")]
    pub public_key_switch: bool,
    ///
    /// Filter based on hops away
    #[prost(int32, tag = "4")]
    pub hops_away: i32,
    ///
    /// Filter nodes w/o position
    #[prost(bool, tag = "5")]
    pub position_switch: bool,
    ///
    /// Filter nodes by matching name string
    #[prost(string, tag = "6")]
    pub node_name: ::prost::alloc::string::String,
    ///
    /// Filter based on channel
    #[prost(int32, tag = "7")]
    pub channel: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeHighlight {
    ///
    /// Hightlight nodes w/ active chat
    #[prost(bool, tag = "1")]
    pub chat_switch: bool,
    ///
    /// Highlight nodes w/ position
    #[prost(bool, tag = "2")]
    pub position_switch: bool,
    ///
    /// Highlight nodes w/ telemetry data
    #[prost(bool, tag = "3")]
    pub telemetry_switch: bool,
    ///
    /// Highlight nodes w/ iaq data
    #[prost(bool, tag = "4")]
    pub iaq_switch: bool,
    ///
    /// Highlight nodes by matching name string
    #[prost(string, tag = "5")]
    pub node_name: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Theme {
    ///
    /// Dark
    Dark = 0,
    ///
    /// Light
    Light = 1,
    ///
    /// Red
    Red = 2,
}
impl Theme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Theme::Dark => "DARK",
            Theme::Light => "LIGHT",
            Theme::Red => "RED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DARK" => Some(Self::Dark),
            "LIGHT" => Some(Self::Light),
            "RED" => Some(Self::Red),
            _ => None,
        }
    }
}
///
/// Localization
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Language {
    ///
    /// English
    English = 0,
    ///
    /// French
    French = 1,
    ///
    /// German
    German = 2,
    ///
    /// Italian
    Italian = 3,
    ///
    /// Portuguese
    Portuguese = 4,
    ///
    /// Spanish
    Spanish = 5,
    ///
    /// Swedish
    Swedish = 6,
    ///
    /// Finnish
    Finnish = 7,
    ///
    /// Polish
    Polish = 8,
    ///
    /// Turkish
    Turkish = 9,
    ///
    /// Serbian
    Serbian = 10,
    ///
    /// Russian
    Russian = 11,
    ///
    /// Dutch
    Dutch = 12,
    ///
    /// Greek
    Greek = 13,
    ///
    /// Norwegian
    Norwegian = 14,
    ///
    /// Slovenian
    Slovenian = 15,
    ///
    /// Simplified Chinese (experimental)
    SimplifiedChinese = 30,
    ///
    /// Traditional Chinese (experimental)
    TraditionalChinese = 31,
}
impl Language {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Language::English => "ENGLISH",
            Language::French => "FRENCH",
            Language::German => "GERMAN",
            Language::Italian => "ITALIAN",
            Language::Portuguese => "PORTUGUESE",
            Language::Spanish => "SPANISH",
            Language::Swedish => "SWEDISH",
            Language::Finnish => "FINNISH",
            Language::Polish => "POLISH",
            Language::Turkish => "TURKISH",
            Language::Serbian => "SERBIAN",
            Language::Russian => "RUSSIAN",
            Language::Dutch => "DUTCH",
            Language::Greek => "GREEK",
            Language::Norwegian => "NORWEGIAN",
            Language::Slovenian => "SLOVENIAN",
            Language::SimplifiedChinese => "SIMPLIFIED_CHINESE",
            Language::TraditionalChinese => "TRADITIONAL_CHINESE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENGLISH" => Some(Self::English),
            "FRENCH" => Some(Self::French),
            "GERMAN" => Some(Self::German),
            "ITALIAN" => Some(Self::Italian),
            "PORTUGUESE" => Some(Self::Portuguese),
            "SPANISH" => Some(Self::Spanish),
            "SWEDISH" => Some(Self::Swedish),
            "FINNISH" => Some(Self::Finnish),
            "POLISH" => Some(Self::Polish),
            "TURKISH" => Some(Self::Turkish),
            "SERBIAN" => Some(Self::Serbian),
            "RUSSIAN" => Some(Self::Russian),
            "DUTCH" => Some(Self::Dutch),
            "GREEK" => Some(Self::Greek),
            "NORWEGIAN" => Some(Self::Norwegian),
            "SLOVENIAN" => Some(Self::Slovenian),
            "SIMPLIFIED_CHINESE" => Some(Self::SimplifiedChinese),
            "TRADITIONAL_CHINESE" => Some(Self::TraditionalChinese),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    ///
    /// Payload Variant
    #[prost(oneof = "config::PayloadVariant", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10")]
    pub payload_variant: ::core::option::Option<config::PayloadVariant>,
}
/// Nested message and enum types in `Config`.
pub mod config {
    ///
    /// Configuration
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeviceConfig {
        ///
        /// Sets the role of node
        #[prost(enumeration = "device_config::Role", tag = "1")]
        pub role: i32,
        ///
        /// Disabling this will disable the SerialConsole by not initilizing the StreamAPI
        /// Moved to SecurityConfig
        #[deprecated]
        #[prost(bool, tag = "2")]
        pub serial_enabled: bool,
        ///
        /// For boards without a hard wired button, this is the pin number that will be used
        /// Boards that have more than one button can swap the function with this one. defaults to BUTTON_PIN if defined.
        #[prost(uint32, tag = "4")]
        pub button_gpio: u32,
        ///
        /// For boards without a PWM buzzer, this is the pin number that will be used
        /// Defaults to PIN_BUZZER if defined.
        #[prost(uint32, tag = "5")]
        pub buzzer_gpio: u32,
        ///
        /// Sets the role of node
        #[prost(enumeration = "device_config::RebroadcastMode", tag = "6")]
        pub rebroadcast_mode: i32,
        ///
        /// Send our nodeinfo this often
        /// Defaults to 900 Seconds (15 minutes)
        #[prost(uint32, tag = "7")]
        pub node_info_broadcast_secs: u32,
        ///
        /// Treat double tap interrupt on supported accelerometers as a button press if set to true
        #[prost(bool, tag = "8")]
        pub double_tap_as_button_press: bool,
        ///
        /// If true, device is considered to be "managed" by a mesh administrator
        /// Clients should then limit available configuration and administrative options inside the user interface
        /// Moved to SecurityConfig
        #[deprecated]
        #[prost(bool, tag = "9")]
        pub is_managed: bool,
        ///
        /// Disables the triple-press of user button to enable or disable GPS
        #[prost(bool, tag = "10")]
        pub disable_triple_click: bool,
        ///
        /// POSIX Timezone definition string from <https://github.com/nayarsystems/posix_tz_db/blob/master/zones.csv.>
        #[prost(string, tag = "11")]
        pub tzdef: ::prost::alloc::string::String,
        ///
        /// If true, disable the default blinking LED (LED_PIN) behavior on the device
        #[prost(bool, tag = "12")]
        pub led_heartbeat_disabled: bool,
    }
    /// Nested message and enum types in `DeviceConfig`.
    pub mod device_config {
        ///
        /// Defines the device's role on the Mesh network
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum Role {
            ///
            /// Description: App connected or stand alone messaging device.
            /// Technical Details: Default Role
            Client = 0,
            ///
            ///   Description: Device that does not forward packets from other devices.
            ClientMute = 1,
            ///
            /// Description: Infrastructure node for extending network coverage by relaying messages. Visible in Nodes list.
            /// Technical Details: Mesh packets will prefer to be routed over this node. This node will not be used by client apps.
            ///    The wifi radio and the oled screen will be put to sleep.
            ///    This mode may still potentially have higher power usage due to it's preference in message rebroadcasting on the mesh.
            Router = 2,
            RouterClient = 3,
            ///
            /// Description: Infrastructure node for extending network coverage by relaying messages with minimal overhead. Not visible in Nodes list.
            /// Technical Details: Mesh packets will simply be rebroadcasted over this node. Nodes configured with this role will not originate NodeInfo, Position, Telemetry
            ///    or any other packet type. They will simply rebroadcast any mesh packets on the same frequency, channel num, spread factor, and coding rate.
            Repeater = 4,
            ///
            /// Description: Broadcasts GPS position packets as priority.
            /// Technical Details: Position Mesh packets will be prioritized higher and sent more frequently by default.
            ///    When used in conjunction with power.is_power_saving = true, nodes will wake up,
            ///    send position, and then sleep for position.position_broadcast_secs seconds.
            Tracker = 5,
            ///
            /// Description: Broadcasts telemetry packets as priority.
            /// Technical Details: Telemetry Mesh packets will be prioritized higher and sent more frequently by default.
            ///    When used in conjunction with power.is_power_saving = true, nodes will wake up,
            ///    send environment telemetry, and then sleep for telemetry.environment_update_interval seconds.
            Sensor = 6,
            ///
            /// Description: Optimized for ATAK system communication and reduces routine broadcasts.
            /// Technical Details: Used for nodes dedicated for connection to an ATAK EUD.
            ///     Turns off many of the routine broadcasts to favor CoT packet stream
            ///     from the Meshtastic ATAK plugin -> IMeshService -> Node
            Tak = 7,
            ///
            /// Description: Device that only broadcasts as needed for stealth or power savings.
            /// Technical Details: Used for nodes that "only speak when spoken to"
            ///     Turns all of the routine broadcasts but allows for ad-hoc communication
            ///     Still rebroadcasts, but with local only rebroadcast mode (known meshes only)
            ///     Can be used for clandestine operation or to dramatically reduce airtime / power consumption
            ClientHidden = 8,
            ///
            /// Description: Broadcasts location as message to default channel regularly for to assist with device recovery.
            /// Technical Details: Used to automatically send a text message to the mesh
            ///     with the current position of the device on a frequent interval:
            ///     "I'm lost! Position: lat / long"
            LostAndFound = 9,
            ///
            /// Description: Enables automatic TAK PLI broadcasts and reduces routine broadcasts.
            /// Technical Details: Turns off many of the routine broadcasts to favor ATAK CoT packet stream
            ///     and automatic TAK PLI (position location information) broadcasts.
            ///     Uses position module configuration to determine TAK PLI broadcast interval.
            TakTracker = 10,
            ///
            /// Description: Will always rebroadcast packets, but will do so after all other modes.
            /// Technical Details: Used for router nodes that are intended to provide additional coverage
            ///     in areas not already covered by other routers, or to bridge around problematic terrain,
            ///     but should not be given priority over other routers in order to avoid unnecessaraily
            ///     consuming hops.
            RouterLate = 11,
        }
        impl Role {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Role::Client => "CLIENT",
                    Role::ClientMute => "CLIENT_MUTE",
                    Role::Router => "ROUTER",
                    Role::RouterClient => "ROUTER_CLIENT",
                    Role::Repeater => "REPEATER",
                    Role::Tracker => "TRACKER",
                    Role::Sensor => "SENSOR",
                    Role::Tak => "TAK",
                    Role::ClientHidden => "CLIENT_HIDDEN",
                    Role::LostAndFound => "LOST_AND_FOUND",
                    Role::TakTracker => "TAK_TRACKER",
                    Role::RouterLate => "ROUTER_LATE",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CLIENT" => Some(Self::Client),
                    "CLIENT_MUTE" => Some(Self::ClientMute),
                    "ROUTER" => Some(Self::Router),
                    "ROUTER_CLIENT" => Some(Self::RouterClient),
                    "REPEATER" => Some(Self::Repeater),
                    "TRACKER" => Some(Self::Tracker),
                    "SENSOR" => Some(Self::Sensor),
                    "TAK" => Some(Self::Tak),
                    "CLIENT_HIDDEN" => Some(Self::ClientHidden),
                    "LOST_AND_FOUND" => Some(Self::LostAndFound),
                    "TAK_TRACKER" => Some(Self::TakTracker),
                    "ROUTER_LATE" => Some(Self::RouterLate),
                    _ => None,
                }
            }
        }
        ///
        /// Defines the device's behavior for how messages are rebroadcast
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum RebroadcastMode {
            ///
            /// Default behavior.
            /// Rebroadcast any observed message, if it was on our private channel or from another mesh with the same lora params.
            All = 0,
            ///
            /// Same as behavior as ALL but skips packet decoding and simply rebroadcasts them.
            /// Only available in Repeater role. Setting this on any other roles will result in ALL behavior.
            AllSkipDecoding = 1,
            ///
            /// Ignores observed messages from foreign meshes that are open or those which it cannot decrypt.
            /// Only rebroadcasts message on the nodes local primary / secondary channels.
            LocalOnly = 2,
            ///
            /// Ignores observed messages from foreign meshes like LOCAL_ONLY,
            /// but takes it step further by also ignoring messages from nodenums not in the node's known list (NodeDB)
            KnownOnly = 3,
            ///
            /// Only permitted for SENSOR, TRACKER and TAK_TRACKER roles, this will inhibit all rebroadcasts, not unlike CLIENT_MUTE role.
            None = 4,
            ///
            /// Ignores packets from non-standard portnums such as: TAK, RangeTest, PaxCounter, etc.
            /// Only rebroadcasts packets with standard portnums: NodeInfo, Text, Position, Telemetry, and Routing.
            CorePortnumsOnly = 5,
        }
        impl RebroadcastMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RebroadcastMode::All => "ALL",
                    RebroadcastMode::AllSkipDecoding => "ALL_SKIP_DECODING",
                    RebroadcastMode::LocalOnly => "LOCAL_ONLY",
                    RebroadcastMode::KnownOnly => "KNOWN_ONLY",
                    RebroadcastMode::None => "NONE",
                    RebroadcastMode::CorePortnumsOnly => "CORE_PORTNUMS_ONLY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ALL" => Some(Self::All),
                    "ALL_SKIP_DECODING" => Some(Self::AllSkipDecoding),
                    "LOCAL_ONLY" => Some(Self::LocalOnly),
                    "KNOWN_ONLY" => Some(Self::KnownOnly),
                    "NONE" => Some(Self::None),
                    "CORE_PORTNUMS_ONLY" => Some(Self::CorePortnumsOnly),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Position Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionConfig {
        ///
        /// We should send our position this often (but only if it has changed significantly)
        /// Defaults to 15 minutes
        #[prost(uint32, tag = "1")]
        pub position_broadcast_secs: u32,
        ///
        /// Adaptive position braoadcast, which is now the default.
        #[prost(bool, tag = "2")]
        pub position_broadcast_smart_enabled: bool,
        ///
        /// If set, this node is at a fixed position.
        /// We will generate GPS position updates at the regular interval, but use whatever the last lat/lon/alt we have for the node.
        /// The lat/lon/alt can be set by an internal GPS or with the help of the app.
        #[prost(bool, tag = "3")]
        pub fixed_position: bool,
        ///
        /// Is GPS enabled for this node?
        #[deprecated]
        #[prost(bool, tag = "4")]
        pub gps_enabled: bool,
        ///
        /// How often should we try to get GPS position (in seconds)
        /// or zero for the default of once every 30 seconds
        /// or a very large value (maxint) to update only once at boot.
        #[prost(uint32, tag = "5")]
        pub gps_update_interval: u32,
        ///
        /// Deprecated in favor of using smart / regular broadcast intervals as implicit attempt time
        #[deprecated]
        #[prost(uint32, tag = "6")]
        pub gps_attempt_time: u32,
        ///
        /// Bit field of boolean configuration options for POSITION messages
        /// (bitwise OR of PositionFlags)
        #[prost(uint32, tag = "7")]
        pub position_flags: u32,
        ///
        /// (Re)define GPS_RX_PIN for your board.
        #[prost(uint32, tag = "8")]
        pub rx_gpio: u32,
        ///
        /// (Re)define GPS_TX_PIN for your board.
        #[prost(uint32, tag = "9")]
        pub tx_gpio: u32,
        ///
        /// The minimum distance in meters traveled (since the last send) before we can send a position to the mesh if position_broadcast_smart_enabled
        #[prost(uint32, tag = "10")]
        pub broadcast_smart_minimum_distance: u32,
        ///
        /// The minimum number of seconds (since the last send) before we can send a position to the mesh if position_broadcast_smart_enabled
        #[prost(uint32, tag = "11")]
        pub broadcast_smart_minimum_interval_secs: u32,
        ///
        /// (Re)define PIN_GPS_EN for your board.
        #[prost(uint32, tag = "12")]
        pub gps_en_gpio: u32,
        ///
        /// Set where GPS is enabled, disabled, or not present
        #[prost(enumeration = "position_config::GpsMode", tag = "13")]
        pub gps_mode: i32,
    }
    /// Nested message and enum types in `PositionConfig`.
    pub mod position_config {
        ///
        /// Bit field of boolean configuration options, indicating which optional
        /// fields to include when assembling POSITION messages.
        /// Longitude, latitude, altitude, speed, heading, and DOP
        /// are always included (also time if GPS-synced)
        /// NOTE: the more fields are included, the larger the message will be -
        ///    leading to longer airtime and a higher risk of packet loss
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum PositionFlags {
            ///
            /// Required for compilation
            Unset = 0,
            ///
            /// Include an altitude value (if available)
            Altitude = 1,
            ///
            /// Altitude value is MSL
            AltitudeMsl = 2,
            ///
            /// Include geoidal separation
            GeoidalSeparation = 4,
            ///
            /// Include the DOP value ; PDOP used by default, see below
            Dop = 8,
            ///
            /// If POS_DOP set, send separate HDOP / VDOP values instead of PDOP
            Hvdop = 16,
            ///
            /// Include number of "satellites in view"
            Satinview = 32,
            ///
            /// Include a sequence number incremented per packet
            SeqNo = 64,
            ///
            /// Include positional timestamp (from GPS solution)
            Timestamp = 128,
            ///
            /// Include positional heading
            /// Intended for use with vehicle not walking speeds
            /// walking speeds are likely to be error prone like the compass
            Heading = 256,
            ///
            /// Include positional speed
            /// Intended for use with vehicle not walking speeds
            /// walking speeds are likely to be error prone like the compass
            Speed = 512,
        }
        impl PositionFlags {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PositionFlags::Unset => "UNSET",
                    PositionFlags::Altitude => "ALTITUDE",
                    PositionFlags::AltitudeMsl => "ALTITUDE_MSL",
                    PositionFlags::GeoidalSeparation => "GEOIDAL_SEPARATION",
                    PositionFlags::Dop => "DOP",
                    PositionFlags::Hvdop => "HVDOP",
                    PositionFlags::Satinview => "SATINVIEW",
                    PositionFlags::SeqNo => "SEQ_NO",
                    PositionFlags::Timestamp => "TIMESTAMP",
                    PositionFlags::Heading => "HEADING",
                    PositionFlags::Speed => "SPEED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNSET" => Some(Self::Unset),
                    "ALTITUDE" => Some(Self::Altitude),
                    "ALTITUDE_MSL" => Some(Self::AltitudeMsl),
                    "GEOIDAL_SEPARATION" => Some(Self::GeoidalSeparation),
                    "DOP" => Some(Self::Dop),
                    "HVDOP" => Some(Self::Hvdop),
                    "SATINVIEW" => Some(Self::Satinview),
                    "SEQ_NO" => Some(Self::SeqNo),
                    "TIMESTAMP" => Some(Self::Timestamp),
                    "HEADING" => Some(Self::Heading),
                    "SPEED" => Some(Self::Speed),
                    _ => None,
                }
            }
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum GpsMode {
            ///
            /// GPS is present but disabled
            Disabled = 0,
            ///
            /// GPS is present and enabled
            Enabled = 1,
            ///
            /// GPS is not present on the device
            NotPresent = 2,
        }
        impl GpsMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    GpsMode::Disabled => "DISABLED",
                    GpsMode::Enabled => "ENABLED",
                    GpsMode::NotPresent => "NOT_PRESENT",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DISABLED" => Some(Self::Disabled),
                    "ENABLED" => Some(Self::Enabled),
                    "NOT_PRESENT" => Some(Self::NotPresent),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Power Config\
    /// See [Power Config](/docs/settings/config/power) for additional power config details.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PowerConfig {
        ///
        /// Description: Will sleep everything as much as possible, for the tracker and sensor role this will also include the lora radio.
        /// Don't use this setting if you want to use your device with the phone apps or are using a device without a user button.
        /// Technical Details: Works for ESP32 devices and NRF52 devices in the Sensor or Tracker roles
        #[prost(bool, tag = "1")]
        pub is_power_saving: bool,
        ///
        ///   Description: If non-zero, the device will fully power off this many seconds after external power is removed.
        #[prost(uint32, tag = "2")]
        pub on_battery_shutdown_after_secs: u32,
        ///
        /// Ratio of voltage divider for battery pin eg. 3.20 (R1=100k, R2=220k)
        /// Overrides the ADC_MULTIPLIER defined in variant for battery voltage calculation.
        /// <https://meshtastic.org/docs/configuration/radio/power/#adc-multiplier-override>
        /// Should be set to floating point value between 2 and 6
        #[prost(float, tag = "3")]
        pub adc_multiplier_override: f32,
        ///
        ///   Description: The number of seconds for to wait before turning off BLE in No Bluetooth states
        ///   Technical Details: ESP32 Only 0 for default of 1 minute
        #[prost(uint32, tag = "4")]
        pub wait_bluetooth_secs: u32,
        ///
        /// Super Deep Sleep Seconds
        /// While in Light Sleep if mesh_sds_timeout_secs is exceeded we will lower into super deep sleep
        /// for this value (default 1 year) or a button press
        /// 0 for default of one year
        #[prost(uint32, tag = "6")]
        pub sds_secs: u32,
        ///
        /// Description: In light sleep the CPU is suspended, LoRa radio is on, BLE is off an GPS is on
        /// Technical Details: ESP32 Only 0 for default of 300
        #[prost(uint32, tag = "7")]
        pub ls_secs: u32,
        ///
        /// Description: While in light sleep when we receive packets on the LoRa radio we will wake and handle them and stay awake in no BLE mode for this value
        /// Technical Details: ESP32 Only 0 for default of 10 seconds
        #[prost(uint32, tag = "8")]
        pub min_wake_secs: u32,
        ///
        /// I2C address of INA_2XX to use for reading device battery voltage
        #[prost(uint32, tag = "9")]
        pub device_battery_ina_address: u32,
        ///
        /// If non-zero, we want powermon log outputs.  With the particular (bitfield) sources enabled.
        /// Note: we picked an ID of 32 so that lower more efficient IDs can be used for more frequently used options.
        #[prost(uint64, tag = "32")]
        pub powermon_enables: u64,
    }
    ///
    /// Network Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkConfig {
        ///
        /// Enable WiFi (disables Bluetooth)
        #[prost(bool, tag = "1")]
        pub wifi_enabled: bool,
        ///
        /// If set, this node will try to join the specified wifi network and
        /// acquire an address via DHCP
        #[prost(string, tag = "3")]
        pub wifi_ssid: ::prost::alloc::string::String,
        ///
        /// If set, will be use to authenticate to the named wifi
        #[prost(string, tag = "4")]
        pub wifi_psk: ::prost::alloc::string::String,
        ///
        /// NTP server to use if WiFi is conneced, defaults to `0.pool.ntp.org`
        #[prost(string, tag = "5")]
        pub ntp_server: ::prost::alloc::string::String,
        ///
        /// Enable Ethernet
        #[prost(bool, tag = "6")]
        pub eth_enabled: bool,
        ///
        /// acquire an address via DHCP or assign static
        #[prost(enumeration = "network_config::AddressMode", tag = "7")]
        pub address_mode: i32,
        ///
        /// struct to keep static address
        #[prost(message, optional, tag = "8")]
        pub ipv4_config: ::core::option::Option<network_config::IpV4Config>,
        ///
        /// rsyslog Server and Port
        #[prost(string, tag = "9")]
        pub rsyslog_server: ::prost::alloc::string::String,
        ///
        /// Flags for enabling/disabling network protocols
        #[prost(uint32, tag = "10")]
        pub enabled_protocols: u32,
    }
    /// Nested message and enum types in `NetworkConfig`.
    pub mod network_config {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct IpV4Config {
            ///
            /// Static IP address
            #[prost(fixed32, tag = "1")]
            pub ip: u32,
            ///
            /// Static gateway address
            #[prost(fixed32, tag = "2")]
            pub gateway: u32,
            ///
            /// Static subnet mask
            #[prost(fixed32, tag = "3")]
            pub subnet: u32,
            ///
            /// Static DNS server address
            #[prost(fixed32, tag = "4")]
            pub dns: u32,
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum AddressMode {
            ///
            /// obtain ip address via DHCP
            Dhcp = 0,
            ///
            /// use static ip address
            Static = 1,
        }
        impl AddressMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    AddressMode::Dhcp => "DHCP",
                    AddressMode::Static => "STATIC",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DHCP" => Some(Self::Dhcp),
                    "STATIC" => Some(Self::Static),
                    _ => None,
                }
            }
        }
        ///
        /// Available flags auxiliary network protocols
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ProtocolFlags {
            ///
            /// Do not broadcast packets over any network protocol
            NoBroadcast = 0,
            ///
            /// Enable broadcasting packets via UDP over the local network
            UdpBroadcast = 1,
        }
        impl ProtocolFlags {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ProtocolFlags::NoBroadcast => "NO_BROADCAST",
                    ProtocolFlags::UdpBroadcast => "UDP_BROADCAST",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NO_BROADCAST" => Some(Self::NoBroadcast),
                    "UDP_BROADCAST" => Some(Self::UdpBroadcast),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Display Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DisplayConfig {
        ///
        /// Number of seconds the screen stays on after pressing the user button or receiving a message
        /// 0 for default of one minute MAXUINT for always on
        #[prost(uint32, tag = "1")]
        pub screen_on_secs: u32,
        ///
        /// How the GPS coordinates are formatted on the OLED screen.
        #[prost(enumeration = "display_config::GpsCoordinateFormat", tag = "2")]
        pub gps_format: i32,
        ///
        /// Automatically toggles to the next page on the screen like a carousel, based the specified interval in seconds.
        /// Potentially useful for devices without user buttons.
        #[prost(uint32, tag = "3")]
        pub auto_screen_carousel_secs: u32,
        ///
        /// If this is set, the displayed compass will always point north. if unset, the old behaviour
        /// (top of display is heading direction) is used.
        #[prost(bool, tag = "4")]
        pub compass_north_top: bool,
        ///
        /// Flip screen vertically, for cases that mount the screen upside down
        #[prost(bool, tag = "5")]
        pub flip_screen: bool,
        ///
        /// Perferred display units
        #[prost(enumeration = "display_config::DisplayUnits", tag = "6")]
        pub units: i32,
        ///
        /// Override auto-detect in screen
        #[prost(enumeration = "display_config::OledType", tag = "7")]
        pub oled: i32,
        ///
        /// Display Mode
        #[prost(enumeration = "display_config::DisplayMode", tag = "8")]
        pub displaymode: i32,
        ///
        /// Print first line in pseudo-bold? FALSE is original style, TRUE is bold
        #[prost(bool, tag = "9")]
        pub heading_bold: bool,
        ///
        /// Should we wake the screen up on accelerometer detected motion or tap
        #[prost(bool, tag = "10")]
        pub wake_on_tap_or_motion: bool,
        ///
        /// Indicates how to rotate or invert the compass output to accurate display on the display.
        #[prost(enumeration = "display_config::CompassOrientation", tag = "11")]
        pub compass_orientation: i32,
    }
    /// Nested message and enum types in `DisplayConfig`.
    pub mod display_config {
        ///
        /// How the GPS coordinates are displayed on the OLED screen.
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum GpsCoordinateFormat {
            ///
            /// GPS coordinates are displayed in the normal decimal degrees format:
            /// DD.DDDDDD DDD.DDDDDD
            Dec = 0,
            ///
            /// GPS coordinates are displayed in the degrees minutes seconds format:
            /// DD°MM'SS"C DDD°MM'SS"C, where C is the compass point representing the locations quadrant
            Dms = 1,
            ///
            /// Universal Transverse Mercator format:
            /// ZZB EEEEEE NNNNNNN, where Z is zone, B is band, E is easting, N is northing
            Utm = 2,
            ///
            /// Military Grid Reference System format:
            /// ZZB CD EEEEE NNNNN, where Z is zone, B is band, C is the east 100k square, D is the north 100k square,
            /// E is easting, N is northing
            Mgrs = 3,
            ///
            /// Open Location Code (aka Plus Codes).
            Olc = 4,
            ///
            /// Ordnance Survey Grid Reference (the National Grid System of the UK).
            /// Format: AB EEEEE NNNNN, where A is the east 100k square, B is the north 100k square,
            /// E is the easting, N is the northing
            Osgr = 5,
        }
        impl GpsCoordinateFormat {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    GpsCoordinateFormat::Dec => "DEC",
                    GpsCoordinateFormat::Dms => "DMS",
                    GpsCoordinateFormat::Utm => "UTM",
                    GpsCoordinateFormat::Mgrs => "MGRS",
                    GpsCoordinateFormat::Olc => "OLC",
                    GpsCoordinateFormat::Osgr => "OSGR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEC" => Some(Self::Dec),
                    "DMS" => Some(Self::Dms),
                    "UTM" => Some(Self::Utm),
                    "MGRS" => Some(Self::Mgrs),
                    "OLC" => Some(Self::Olc),
                    "OSGR" => Some(Self::Osgr),
                    _ => None,
                }
            }
        }
        ///
        /// Unit display preference
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum DisplayUnits {
            ///
            /// Metric (Default)
            Metric = 0,
            ///
            /// Imperial
            Imperial = 1,
        }
        impl DisplayUnits {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DisplayUnits::Metric => "METRIC",
                    DisplayUnits::Imperial => "IMPERIAL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "METRIC" => Some(Self::Metric),
                    "IMPERIAL" => Some(Self::Imperial),
                    _ => None,
                }
            }
        }
        ///
        /// Override OLED outo detect with this if it fails.
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum OledType {
            ///
            /// Default / Auto
            OledAuto = 0,
            ///
            /// Default / Auto
            OledSsd1306 = 1,
            ///
            /// Default / Auto
            OledSh1106 = 2,
            ///
            /// Can not be auto detected but set by proto. Used for 128x128 screens
            OledSh1107 = 3,
        }
        impl OledType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    OledType::OledAuto => "OLED_AUTO",
                    OledType::OledSsd1306 => "OLED_SSD1306",
                    OledType::OledSh1106 => "OLED_SH1106",
                    OledType::OledSh1107 => "OLED_SH1107",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "OLED_AUTO" => Some(Self::OledAuto),
                    "OLED_SSD1306" => Some(Self::OledSsd1306),
                    "OLED_SH1106" => Some(Self::OledSh1106),
                    "OLED_SH1107" => Some(Self::OledSh1107),
                    _ => None,
                }
            }
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum DisplayMode {
            ///
            /// Default. The old style for the 128x64 OLED screen
            Default = 0,
            ///
            /// Rearrange display elements to cater for bicolor OLED displays
            Twocolor = 1,
            ///
            /// Same as TwoColor, but with inverted top bar. Not so good for Epaper displays
            Inverted = 2,
            ///
            /// TFT Full Color Displays (not implemented yet)
            Color = 3,
        }
        impl DisplayMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    DisplayMode::Default => "DEFAULT",
                    DisplayMode::Twocolor => "TWOCOLOR",
                    DisplayMode::Inverted => "INVERTED",
                    DisplayMode::Color => "COLOR",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "TWOCOLOR" => Some(Self::Twocolor),
                    "INVERTED" => Some(Self::Inverted),
                    "COLOR" => Some(Self::Color),
                    _ => None,
                }
            }
        }
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum CompassOrientation {
            ///
            /// The compass and the display are in the same orientation.
            Degrees0 = 0,
            ///
            /// Rotate the compass by 90 degrees.
            Degrees90 = 1,
            ///
            /// Rotate the compass by 180 degrees.
            Degrees180 = 2,
            ///
            /// Rotate the compass by 270 degrees.
            Degrees270 = 3,
            ///
            /// Don't rotate the compass, but invert the result.
            Degrees0Inverted = 4,
            ///
            /// Rotate the compass by 90 degrees and invert.
            Degrees90Inverted = 5,
            ///
            /// Rotate the compass by 180 degrees and invert.
            Degrees180Inverted = 6,
            ///
            /// Rotate the compass by 270 degrees and invert.
            Degrees270Inverted = 7,
        }
        impl CompassOrientation {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    CompassOrientation::Degrees0 => "DEGREES_0",
                    CompassOrientation::Degrees90 => "DEGREES_90",
                    CompassOrientation::Degrees180 => "DEGREES_180",
                    CompassOrientation::Degrees270 => "DEGREES_270",
                    CompassOrientation::Degrees0Inverted => "DEGREES_0_INVERTED",
                    CompassOrientation::Degrees90Inverted => "DEGREES_90_INVERTED",
                    CompassOrientation::Degrees180Inverted => "DEGREES_180_INVERTED",
                    CompassOrientation::Degrees270Inverted => "DEGREES_270_INVERTED",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEGREES_0" => Some(Self::Degrees0),
                    "DEGREES_90" => Some(Self::Degrees90),
                    "DEGREES_180" => Some(Self::Degrees180),
                    "DEGREES_270" => Some(Self::Degrees270),
                    "DEGREES_0_INVERTED" => Some(Self::Degrees0Inverted),
                    "DEGREES_90_INVERTED" => Some(Self::Degrees90Inverted),
                    "DEGREES_180_INVERTED" => Some(Self::Degrees180Inverted),
                    "DEGREES_270_INVERTED" => Some(Self::Degrees270Inverted),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Lora Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoRaConfig {
        ///
        /// When enabled, the `modem_preset` fields will be adhered to, else the `bandwidth`/`spread_factor`/`coding_rate`
        /// will be taked from their respective manually defined fields
        #[prost(bool, tag = "1")]
        pub use_preset: bool,
        ///
        /// Either modem_config or bandwidth/spreading/coding will be specified - NOT BOTH.
        /// As a heuristic: If bandwidth is specified, do not use modem_config.
        /// Because protobufs take ZERO space when the value is zero this works out nicely.
        /// This value is replaced by bandwidth/spread_factor/coding_rate.
        /// If you'd like to experiment with other options add them to MeshRadio.cpp in the device code.
        #[prost(enumeration = "lo_ra_config::ModemPreset", tag = "2")]
        pub modem_preset: i32,
        ///
        /// Bandwidth in MHz
        /// Certain bandwidth numbers are 'special' and will be converted to the
        /// appropriate floating point value: 31 -> 31.25MHz
        #[prost(uint32, tag = "3")]
        pub bandwidth: u32,
        ///
        /// A number from 7 to 12.
        /// Indicates number of chirps per symbol as 1<<spread_factor.
        #[prost(uint32, tag = "4")]
        pub spread_factor: u32,
        ///
        /// The denominator of the coding rate.
        /// ie for 4/5, the value is 5. 4/8 the value is 8.
        #[prost(uint32, tag = "5")]
        pub coding_rate: u32,
        ///
        /// This parameter is for advanced users with advanced test equipment, we do not recommend most users use it.
        /// A frequency offset that is added to to the calculated band center frequency.
        /// Used to correct for crystal calibration errors.
        #[prost(float, tag = "6")]
        pub frequency_offset: f32,
        ///
        /// The region code for the radio (US, CN, EU433, etc...)
        #[prost(enumeration = "lo_ra_config::RegionCode", tag = "7")]
        pub region: i32,
        ///
        /// Maximum number of hops. This can't be greater than 7.
        /// Default of 3
        /// Attempting to set a value > 7 results in the default
        #[prost(uint32, tag = "8")]
        pub hop_limit: u32,
        ///
        /// Disable TX from the LoRa radio. Useful for hot-swapping antennas and other tests.
        /// Defaults to false
        #[prost(bool, tag = "9")]
        pub tx_enabled: bool,
        ///
        /// If zero, then use default max legal continuous power (ie. something that won't
        /// burn out the radio hardware)
        /// In most cases you should use zero here.
        /// Units are in dBm.
        #[prost(int32, tag = "10")]
        pub tx_power: i32,
        ///
        /// This controls the actual hardware frequency the radio transmits on.
        /// Most users should never need to be exposed to this field/concept.
        /// A channel number between 1 and NUM_CHANNELS (whatever the max is in the current region).
        /// If ZERO then the rule is "use the old channel name hash based
        /// algorithm to derive the channel number")
        /// If using the hash algorithm the channel number will be: hash(channel_name) %
        /// NUM_CHANNELS (Where num channels depends on the regulatory region).
        #[prost(uint32, tag = "11")]
        pub channel_num: u32,
        ///
        /// If true, duty cycle limits will be exceeded and thus you're possibly not following
        /// the local regulations if you're not a HAM.
        /// Has no effect if the duty cycle of the used region is 100%.
        #[prost(bool, tag = "12")]
        pub override_duty_cycle: bool,
        ///
        /// If true, sets RX boosted gain mode on SX126X based radios
        #[prost(bool, tag = "13")]
        pub sx126x_rx_boosted_gain: bool,
        ///
        /// This parameter is for advanced users and licensed HAM radio operators.
        /// Ignore Channel Calculation and use this frequency instead. The frequency_offset
        /// will still be applied. This will allow you to use out-of-band frequencies.
        /// Please respect your local laws and regulations. If you are a HAM, make sure you
        /// enable HAM mode and turn off encryption.
        #[prost(float, tag = "14")]
        pub override_frequency: f32,
        ///
        /// If true, disable the build-in PA FAN using pin define in RF95_FAN_EN.
        #[prost(bool, tag = "15")]
        pub pa_fan_disabled: bool,
        ///
        /// For testing it is useful sometimes to force a node to never listen to
        /// particular other nodes (simulating radio out of range). All nodenums listed
        /// in ignore_incoming will have packets they send dropped on receive (by router.cpp)
        #[prost(uint32, repeated, tag = "103")]
        pub ignore_incoming: ::prost::alloc::vec::Vec<u32>,
        ///
        /// If true, the device will not process any packets received via LoRa that passed via MQTT anywhere on the path towards it.
        #[prost(bool, tag = "104")]
        pub ignore_mqtt: bool,
        ///
        /// Sets the ok_to_mqtt bit on outgoing packets
        #[prost(bool, tag = "105")]
        pub config_ok_to_mqtt: bool,
    }
    /// Nested message and enum types in `LoRaConfig`.
    pub mod lo_ra_config {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum RegionCode {
            ///
            /// Region is not set
            Unset = 0,
            ///
            /// United States
            Us = 1,
            ///
            /// European Union 433mhz
            Eu433 = 2,
            ///
            /// European Union 868mhz
            Eu868 = 3,
            ///
            /// China
            Cn = 4,
            ///
            /// Japan
            Jp = 5,
            ///
            /// Australia / New Zealand
            Anz = 6,
            ///
            /// Korea
            Kr = 7,
            ///
            /// Taiwan
            Tw = 8,
            ///
            /// Russia
            Ru = 9,
            ///
            /// India
            In = 10,
            ///
            /// New Zealand 865mhz
            Nz865 = 11,
            ///
            /// Thailand
            Th = 12,
            ///
            /// WLAN Band
            Lora24 = 13,
            ///
            /// Ukraine 433mhz
            Ua433 = 14,
            ///
            /// Ukraine 868mhz
            Ua868 = 15,
            ///
            /// Malaysia 433mhz
            My433 = 16,
            ///
            /// Malaysia 919mhz
            My919 = 17,
            ///
            /// Singapore 923mhz
            Sg923 = 18,
            ///
            /// Philippines 433mhz
            Ph433 = 19,
            ///
            /// Philippines 868mhz
            Ph868 = 20,
            ///
            /// Philippines 915mhz
            Ph915 = 21,
        }
        impl RegionCode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    RegionCode::Unset => "UNSET",
                    RegionCode::Us => "US",
                    RegionCode::Eu433 => "EU_433",
                    RegionCode::Eu868 => "EU_868",
                    RegionCode::Cn => "CN",
                    RegionCode::Jp => "JP",
                    RegionCode::Anz => "ANZ",
                    RegionCode::Kr => "KR",
                    RegionCode::Tw => "TW",
                    RegionCode::Ru => "RU",
                    RegionCode::In => "IN",
                    RegionCode::Nz865 => "NZ_865",
                    RegionCode::Th => "TH",
                    RegionCode::Lora24 => "LORA_24",
                    RegionCode::Ua433 => "UA_433",
                    RegionCode::Ua868 => "UA_868",
                    RegionCode::My433 => "MY_433",
                    RegionCode::My919 => "MY_919",
                    RegionCode::Sg923 => "SG_923",
                    RegionCode::Ph433 => "PH_433",
                    RegionCode::Ph868 => "PH_868",
                    RegionCode::Ph915 => "PH_915",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "UNSET" => Some(Self::Unset),
                    "US" => Some(Self::Us),
                    "EU_433" => Some(Self::Eu433),
                    "EU_868" => Some(Self::Eu868),
                    "CN" => Some(Self::Cn),
                    "JP" => Some(Self::Jp),
                    "ANZ" => Some(Self::Anz),
                    "KR" => Some(Self::Kr),
                    "TW" => Some(Self::Tw),
                    "RU" => Some(Self::Ru),
                    "IN" => Some(Self::In),
                    "NZ_865" => Some(Self::Nz865),
                    "TH" => Some(Self::Th),
                    "LORA_24" => Some(Self::Lora24),
                    "UA_433" => Some(Self::Ua433),
                    "UA_868" => Some(Self::Ua868),
                    "MY_433" => Some(Self::My433),
                    "MY_919" => Some(Self::My919),
                    "SG_923" => Some(Self::Sg923),
                    "PH_433" => Some(Self::Ph433),
                    "PH_868" => Some(Self::Ph868),
                    "PH_915" => Some(Self::Ph915),
                    _ => None,
                }
            }
        }
        ///
        /// Standard predefined channel settings
        /// Note: these mappings must match ModemPreset Choice in the device code.
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum ModemPreset {
            ///
            /// Long Range - Fast
            LongFast = 0,
            ///
            /// Long Range - Slow
            LongSlow = 1,
            ///
            /// Very Long Range - Slow
            /// Deprecated in 2.5: Works only with txco and is unusably slow
            VeryLongSlow = 2,
            ///
            /// Medium Range - Slow
            MediumSlow = 3,
            ///
            /// Medium Range - Fast
            MediumFast = 4,
            ///
            /// Short Range - Slow
            ShortSlow = 5,
            ///
            /// Short Range - Fast
            ShortFast = 6,
            ///
            /// Long Range - Moderately Fast
            LongModerate = 7,
            ///
            /// Short Range - Turbo
            /// This is the fastest preset and the only one with 500kHz bandwidth.
            /// It is not legal to use in all regions due to this wider bandwidth.
            ShortTurbo = 8,
        }
        impl ModemPreset {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    ModemPreset::LongFast => "LONG_FAST",
                    ModemPreset::LongSlow => "LONG_SLOW",
                    ModemPreset::VeryLongSlow => "VERY_LONG_SLOW",
                    ModemPreset::MediumSlow => "MEDIUM_SLOW",
                    ModemPreset::MediumFast => "MEDIUM_FAST",
                    ModemPreset::ShortSlow => "SHORT_SLOW",
                    ModemPreset::ShortFast => "SHORT_FAST",
                    ModemPreset::LongModerate => "LONG_MODERATE",
                    ModemPreset::ShortTurbo => "SHORT_TURBO",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "LONG_FAST" => Some(Self::LongFast),
                    "LONG_SLOW" => Some(Self::LongSlow),
                    "VERY_LONG_SLOW" => Some(Self::VeryLongSlow),
                    "MEDIUM_SLOW" => Some(Self::MediumSlow),
                    "MEDIUM_FAST" => Some(Self::MediumFast),
                    "SHORT_SLOW" => Some(Self::ShortSlow),
                    "SHORT_FAST" => Some(Self::ShortFast),
                    "LONG_MODERATE" => Some(Self::LongModerate),
                    "SHORT_TURBO" => Some(Self::ShortTurbo),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BluetoothConfig {
        ///
        /// Enable Bluetooth on the device
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// Determines the pairing strategy for the device
        #[prost(enumeration = "bluetooth_config::PairingMode", tag = "2")]
        pub mode: i32,
        ///
        /// Specified PIN for PairingMode.FixedPin
        #[prost(uint32, tag = "3")]
        pub fixed_pin: u32,
    }
    /// Nested message and enum types in `BluetoothConfig`.
    pub mod bluetooth_config {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum PairingMode {
            ///
            /// Device generates a random PIN that will be shown on the screen of the device for pairing
            RandomPin = 0,
            ///
            /// Device requires a specified fixed PIN for pairing
            FixedPin = 1,
            ///
            /// Device requires no PIN for pairing
            NoPin = 2,
        }
        impl PairingMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PairingMode::RandomPin => "RANDOM_PIN",
                    PairingMode::FixedPin => "FIXED_PIN",
                    PairingMode::NoPin => "NO_PIN",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "RANDOM_PIN" => Some(Self::RandomPin),
                    "FIXED_PIN" => Some(Self::FixedPin),
                    "NO_PIN" => Some(Self::NoPin),
                    _ => None,
                }
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityConfig {
        ///
        /// The public key of the user's device.
        /// Sent out to other nodes on the mesh to allow them to compute a shared secret key.
        #[prost(bytes = "vec", tag = "1")]
        pub public_key: ::prost::alloc::vec::Vec<u8>,
        ///
        /// The private key of the device.
        /// Used to create a shared key with a remote device.
        #[prost(bytes = "vec", tag = "2")]
        pub private_key: ::prost::alloc::vec::Vec<u8>,
        ///
        /// The public key authorized to send admin messages to this node.
        #[prost(bytes = "vec", repeated, tag = "3")]
        pub admin_key: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
        ///
        /// If true, device is considered to be "managed" by a mesh administrator via admin messages
        /// Device is managed by a mesh administrator.
        #[prost(bool, tag = "4")]
        pub is_managed: bool,
        ///
        /// Serial Console over the Stream API."
        #[prost(bool, tag = "5")]
        pub serial_enabled: bool,
        ///
        /// By default we turn off logging as soon as an API client connects (to keep shared serial link quiet).
        /// Output live debug logging over serial or bluetooth is set to true.
        #[prost(bool, tag = "6")]
        pub debug_log_api_enabled: bool,
        ///
        /// Allow incoming device control over the insecure legacy admin channel.
        #[prost(bool, tag = "8")]
        pub admin_channel_enabled: bool,
    }
    ///
    /// Blank config request, strictly for getting the session key
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SessionkeyConfig {}
    ///
    /// Payload Variant
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        #[prost(message, tag = "1")]
        Device(DeviceConfig),
        #[prost(message, tag = "2")]
        Position(PositionConfig),
        #[prost(message, tag = "3")]
        Power(PowerConfig),
        #[prost(message, tag = "4")]
        Network(NetworkConfig),
        #[prost(message, tag = "5")]
        Display(DisplayConfig),
        #[prost(message, tag = "6")]
        Lora(LoRaConfig),
        #[prost(message, tag = "7")]
        Bluetooth(BluetoothConfig),
        #[prost(message, tag = "8")]
        Security(SecurityConfig),
        #[prost(message, tag = "9")]
        Sessionkey(SessionkeyConfig),
        #[prost(message, tag = "10")]
        DeviceUi(super::DeviceUiConfig),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceConnectionStatus {
    ///
    /// WiFi Status
    #[prost(message, optional, tag = "1")]
    pub wifi: ::core::option::Option<WifiConnectionStatus>,
    ///
    /// WiFi Status
    #[prost(message, optional, tag = "2")]
    pub ethernet: ::core::option::Option<EthernetConnectionStatus>,
    ///
    /// Bluetooth Status
    #[prost(message, optional, tag = "3")]
    pub bluetooth: ::core::option::Option<BluetoothConnectionStatus>,
    ///
    /// Serial Status
    #[prost(message, optional, tag = "4")]
    pub serial: ::core::option::Option<SerialConnectionStatus>,
}
///
/// WiFi connection status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WifiConnectionStatus {
    ///
    /// Connection status
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<NetworkConnectionStatus>,
    ///
    /// WiFi access point SSID
    #[prost(string, tag = "2")]
    pub ssid: ::prost::alloc::string::String,
    ///
    /// RSSI of wireless connection
    #[prost(int32, tag = "3")]
    pub rssi: i32,
}
///
/// Ethernet connection status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthernetConnectionStatus {
    ///
    /// Connection status
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<NetworkConnectionStatus>,
}
///
/// Ethernet or WiFi connection status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConnectionStatus {
    ///
    /// IP address of device
    #[prost(fixed32, tag = "1")]
    pub ip_address: u32,
    ///
    /// Whether the device has an active connection or not
    #[prost(bool, tag = "2")]
    pub is_connected: bool,
    ///
    /// Whether the device has an active connection to an MQTT broker or not
    #[prost(bool, tag = "3")]
    pub is_mqtt_connected: bool,
    ///
    /// Whether the device is actively remote syslogging or not
    #[prost(bool, tag = "4")]
    pub is_syslog_connected: bool,
}
///
/// Bluetooth connection status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BluetoothConnectionStatus {
    ///
    /// The pairing PIN for bluetooth
    #[prost(uint32, tag = "1")]
    pub pin: u32,
    ///
    /// RSSI of bluetooth connection
    #[prost(int32, tag = "2")]
    pub rssi: i32,
    ///
    /// Whether the device has an active connection or not
    #[prost(bool, tag = "3")]
    pub is_connected: bool,
}
///
/// Serial connection status
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerialConnectionStatus {
    ///
    /// Serial baud rate
    #[prost(uint32, tag = "1")]
    pub baud: u32,
    ///
    /// Whether the device has an active connection or not
    #[prost(bool, tag = "2")]
    pub is_connected: bool,
}
///
/// Module Config
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleConfig {
    ///
    /// TODO: REPLACE
    #[prost(
        oneof = "module_config::PayloadVariant",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13"
    )]
    pub payload_variant: ::core::option::Option<module_config::PayloadVariant>,
}
/// Nested message and enum types in `ModuleConfig`.
pub mod module_config {
    ///
    /// MQTT Client Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MqttConfig {
        ///
        /// If a meshtastic node is able to reach the internet it will normally attempt to gateway any channels that are marked as
        /// is_uplink_enabled or is_downlink_enabled.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// The server to use for our MQTT global message gateway feature.
        /// If not set, the default server will be used
        #[prost(string, tag = "2")]
        pub address: ::prost::alloc::string::String,
        ///
        /// MQTT username to use (most useful for a custom MQTT server).
        /// If using a custom server, this will be honoured even if empty.
        /// If using the default server, this will only be honoured if set, otherwise the device will use the default username
        #[prost(string, tag = "3")]
        pub username: ::prost::alloc::string::String,
        ///
        /// MQTT password to use (most useful for a custom MQTT server).
        /// If using a custom server, this will be honoured even if empty.
        /// If using the default server, this will only be honoured if set, otherwise the device will use the default password
        #[prost(string, tag = "4")]
        pub password: ::prost::alloc::string::String,
        ///
        /// Whether to send encrypted or decrypted packets to MQTT.
        /// This parameter is only honoured if you also set server
        /// (the default official mqtt.meshtastic.org server can handle encrypted packets)
        /// Decrypted packets may be useful for external systems that want to consume meshtastic packets
        #[prost(bool, tag = "5")]
        pub encryption_enabled: bool,
        ///
        /// Whether to send / consume json packets on MQTT
        #[prost(bool, tag = "6")]
        pub json_enabled: bool,
        ///
        /// If true, we attempt to establish a secure connection using TLS
        #[prost(bool, tag = "7")]
        pub tls_enabled: bool,
        ///
        /// The root topic to use for MQTT messages. Default is "msh".
        /// This is useful if you want to use a single MQTT server for multiple meshtastic networks and separate them via ACLs
        #[prost(string, tag = "8")]
        pub root: ::prost::alloc::string::String,
        ///
        /// If true, we can use the connected phone / client to proxy messages to MQTT instead of a direct connection
        #[prost(bool, tag = "9")]
        pub proxy_to_client_enabled: bool,
        ///
        /// If true, we will periodically report unencrypted information about our node to a map via MQTT
        #[prost(bool, tag = "10")]
        pub map_reporting_enabled: bool,
        ///
        /// Settings for reporting information about our node to a map via MQTT
        #[prost(message, optional, tag = "11")]
        pub map_report_settings: ::core::option::Option<MapReportSettings>,
    }
    ///
    /// Settings for reporting unencrypted information about our node to a map via MQTT
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MapReportSettings {
        ///
        /// How often we should report our info to the map (in seconds)
        #[prost(uint32, tag = "1")]
        pub publish_interval_secs: u32,
        ///
        /// Bits of precision for the location sent (default of 32 is full precision).
        #[prost(uint32, tag = "2")]
        pub position_precision: u32,
    }
    ///
    /// RemoteHardwareModule Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RemoteHardwareConfig {
        ///
        /// Whether the Module is enabled
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// Whether the Module allows consumers to read / write to pins not defined in available_pins
        #[prost(bool, tag = "2")]
        pub allow_undefined_pin_access: bool,
        ///
        /// Exposes the available pins to the mesh for reading and writing
        #[prost(message, repeated, tag = "3")]
        pub available_pins: ::prost::alloc::vec::Vec<super::RemoteHardwarePin>,
    }
    ///
    /// NeighborInfoModule Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NeighborInfoConfig {
        ///
        /// Whether the Module is enabled
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// Neighbor Info (minimum is 14400, i.e., 4 hours)
        #[prost(uint32, tag = "2")]
        pub update_interval: u32,
        ///
        /// Whether in addition to sending it to MQTT and the PhoneAPI, our NeighborInfo should be transmitted over LoRa.
        /// Note that this is not available on a channel with default key and name.
        #[prost(bool, tag = "3")]
        pub transmit_over_lora: bool,
    }
    ///
    /// Detection Sensor Module Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DetectionSensorConfig {
        ///
        /// Whether the Module is enabled
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// Interval in seconds of how often we can send a message to the mesh when a
        /// trigger event is detected
        #[prost(uint32, tag = "2")]
        pub minimum_broadcast_secs: u32,
        ///
        /// Interval in seconds of how often we should send a message to the mesh
        /// with the current state regardless of trigger events When set to 0, only
        /// trigger events will be broadcasted Works as a sort of status heartbeat
        /// for peace of mind
        #[prost(uint32, tag = "3")]
        pub state_broadcast_secs: u32,
        ///
        /// Send ASCII bell with alert message
        /// Useful for triggering ext. notification on bell
        #[prost(bool, tag = "4")]
        pub send_bell: bool,
        ///
        /// Friendly name used to format message sent to mesh
        /// Example: A name "Motion" would result in a message "Motion detected"
        /// Maximum length of 20 characters
        #[prost(string, tag = "5")]
        pub name: ::prost::alloc::string::String,
        ///
        /// GPIO pin to monitor for state changes
        #[prost(uint32, tag = "6")]
        pub monitor_pin: u32,
        ///
        /// The type of trigger event to be used
        #[prost(enumeration = "detection_sensor_config::TriggerType", tag = "7")]
        pub detection_trigger_type: i32,
        ///
        /// Whether or not use INPUT_PULLUP mode for GPIO pin
        /// Only applicable if the board uses pull-up resistors on the pin
        #[prost(bool, tag = "8")]
        pub use_pullup: bool,
    }
    /// Nested message and enum types in `DetectionSensorConfig`.
    pub mod detection_sensor_config {
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum TriggerType {
            /// Event is triggered if pin is low
            LogicLow = 0,
            /// Event is triggered if pin is high
            LogicHigh = 1,
            /// Event is triggered when pin goes high to low
            FallingEdge = 2,
            /// Event is triggered when pin goes low to high
            RisingEdge = 3,
            /// Event is triggered on every pin state change, low is considered to be
            /// "active"
            EitherEdgeActiveLow = 4,
            /// Event is triggered on every pin state change, high is considered to be
            /// "active"
            EitherEdgeActiveHigh = 5,
        }
        impl TriggerType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    TriggerType::LogicLow => "LOGIC_LOW",
                    TriggerType::LogicHigh => "LOGIC_HIGH",
                    TriggerType::FallingEdge => "FALLING_EDGE",
                    TriggerType::RisingEdge => "RISING_EDGE",
                    TriggerType::EitherEdgeActiveLow => "EITHER_EDGE_ACTIVE_LOW",
                    TriggerType::EitherEdgeActiveHigh => "EITHER_EDGE_ACTIVE_HIGH",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "LOGIC_LOW" => Some(Self::LogicLow),
                    "LOGIC_HIGH" => Some(Self::LogicHigh),
                    "FALLING_EDGE" => Some(Self::FallingEdge),
                    "RISING_EDGE" => Some(Self::RisingEdge),
                    "EITHER_EDGE_ACTIVE_LOW" => Some(Self::EitherEdgeActiveLow),
                    "EITHER_EDGE_ACTIVE_HIGH" => Some(Self::EitherEdgeActiveHigh),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Audio Config for codec2 voice
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudioConfig {
        ///
        /// Whether Audio is enabled
        #[prost(bool, tag = "1")]
        pub codec2_enabled: bool,
        ///
        /// PTT Pin
        #[prost(uint32, tag = "2")]
        pub ptt_pin: u32,
        ///
        /// The audio sample rate to use for codec2
        #[prost(enumeration = "audio_config::AudioBaud", tag = "3")]
        pub bitrate: i32,
        ///
        /// I2S Word Select
        #[prost(uint32, tag = "4")]
        pub i2s_ws: u32,
        ///
        /// I2S Data IN
        #[prost(uint32, tag = "5")]
        pub i2s_sd: u32,
        ///
        /// I2S Data OUT
        #[prost(uint32, tag = "6")]
        pub i2s_din: u32,
        ///
        /// I2S Clock
        #[prost(uint32, tag = "7")]
        pub i2s_sck: u32,
    }
    /// Nested message and enum types in `AudioConfig`.
    pub mod audio_config {
        ///
        /// Baudrate for codec2 voice
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum AudioBaud {
            Codec2Default = 0,
            Codec23200 = 1,
            Codec22400 = 2,
            Codec21600 = 3,
            Codec21400 = 4,
            Codec21300 = 5,
            Codec21200 = 6,
            Codec2700 = 7,
            Codec2700b = 8,
        }
        impl AudioBaud {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    AudioBaud::Codec2Default => "CODEC2_DEFAULT",
                    AudioBaud::Codec23200 => "CODEC2_3200",
                    AudioBaud::Codec22400 => "CODEC2_2400",
                    AudioBaud::Codec21600 => "CODEC2_1600",
                    AudioBaud::Codec21400 => "CODEC2_1400",
                    AudioBaud::Codec21300 => "CODEC2_1300",
                    AudioBaud::Codec21200 => "CODEC2_1200",
                    AudioBaud::Codec2700 => "CODEC2_700",
                    AudioBaud::Codec2700b => "CODEC2_700B",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "CODEC2_DEFAULT" => Some(Self::Codec2Default),
                    "CODEC2_3200" => Some(Self::Codec23200),
                    "CODEC2_2400" => Some(Self::Codec22400),
                    "CODEC2_1600" => Some(Self::Codec21600),
                    "CODEC2_1400" => Some(Self::Codec21400),
                    "CODEC2_1300" => Some(Self::Codec21300),
                    "CODEC2_1200" => Some(Self::Codec21200),
                    "CODEC2_700" => Some(Self::Codec2700),
                    "CODEC2_700B" => Some(Self::Codec2700b),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Config for the Paxcounter Module
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PaxcounterConfig {
        ///
        /// Enable the Paxcounter Module
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        #[prost(uint32, tag = "2")]
        pub paxcounter_update_interval: u32,
        ///
        /// WiFi RSSI threshold. Defaults to -80
        #[prost(int32, tag = "3")]
        pub wifi_threshold: i32,
        ///
        /// BLE RSSI threshold. Defaults to -80
        #[prost(int32, tag = "4")]
        pub ble_threshold: i32,
    }
    ///
    /// Serial Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SerialConfig {
        ///
        /// Preferences for the SerialModule
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// TODO: REPLACE
        #[prost(bool, tag = "2")]
        pub echo: bool,
        ///
        /// RX pin (should match Arduino gpio pin number)
        #[prost(uint32, tag = "3")]
        pub rxd: u32,
        ///
        /// TX pin (should match Arduino gpio pin number)
        #[prost(uint32, tag = "4")]
        pub txd: u32,
        ///
        /// Serial baud rate
        #[prost(enumeration = "serial_config::SerialBaud", tag = "5")]
        pub baud: i32,
        ///
        /// TODO: REPLACE
        #[prost(uint32, tag = "6")]
        pub timeout: u32,
        ///
        /// Mode for serial module operation
        #[prost(enumeration = "serial_config::SerialMode", tag = "7")]
        pub mode: i32,
        ///
        /// Overrides the platform's defacto Serial port instance to use with Serial module config settings
        /// This is currently only usable in output modes like NMEA / CalTopo and may behave strangely or not work at all in other modes
        /// Existing logging over the Serial Console will still be present
        #[prost(bool, tag = "8")]
        pub override_console_serial_port: bool,
    }
    /// Nested message and enum types in `SerialConfig`.
    pub mod serial_config {
        ///
        /// TODO: REPLACE
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum SerialBaud {
            BaudDefault = 0,
            Baud110 = 1,
            Baud300 = 2,
            Baud600 = 3,
            Baud1200 = 4,
            Baud2400 = 5,
            Baud4800 = 6,
            Baud9600 = 7,
            Baud19200 = 8,
            Baud38400 = 9,
            Baud57600 = 10,
            Baud115200 = 11,
            Baud230400 = 12,
            Baud460800 = 13,
            Baud576000 = 14,
            Baud921600 = 15,
        }
        impl SerialBaud {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SerialBaud::BaudDefault => "BAUD_DEFAULT",
                    SerialBaud::Baud110 => "BAUD_110",
                    SerialBaud::Baud300 => "BAUD_300",
                    SerialBaud::Baud600 => "BAUD_600",
                    SerialBaud::Baud1200 => "BAUD_1200",
                    SerialBaud::Baud2400 => "BAUD_2400",
                    SerialBaud::Baud4800 => "BAUD_4800",
                    SerialBaud::Baud9600 => "BAUD_9600",
                    SerialBaud::Baud19200 => "BAUD_19200",
                    SerialBaud::Baud38400 => "BAUD_38400",
                    SerialBaud::Baud57600 => "BAUD_57600",
                    SerialBaud::Baud115200 => "BAUD_115200",
                    SerialBaud::Baud230400 => "BAUD_230400",
                    SerialBaud::Baud460800 => "BAUD_460800",
                    SerialBaud::Baud576000 => "BAUD_576000",
                    SerialBaud::Baud921600 => "BAUD_921600",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "BAUD_DEFAULT" => Some(Self::BaudDefault),
                    "BAUD_110" => Some(Self::Baud110),
                    "BAUD_300" => Some(Self::Baud300),
                    "BAUD_600" => Some(Self::Baud600),
                    "BAUD_1200" => Some(Self::Baud1200),
                    "BAUD_2400" => Some(Self::Baud2400),
                    "BAUD_4800" => Some(Self::Baud4800),
                    "BAUD_9600" => Some(Self::Baud9600),
                    "BAUD_19200" => Some(Self::Baud19200),
                    "BAUD_38400" => Some(Self::Baud38400),
                    "BAUD_57600" => Some(Self::Baud57600),
                    "BAUD_115200" => Some(Self::Baud115200),
                    "BAUD_230400" => Some(Self::Baud230400),
                    "BAUD_460800" => Some(Self::Baud460800),
                    "BAUD_576000" => Some(Self::Baud576000),
                    "BAUD_921600" => Some(Self::Baud921600),
                    _ => None,
                }
            }
        }
        ///
        /// TODO: REPLACE
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum SerialMode {
            Default = 0,
            Simple = 1,
            Proto = 2,
            Textmsg = 3,
            Nmea = 4,
            /// NMEA messages specifically tailored for CalTopo
            Caltopo = 5,
            /// Ecowitt WS85 weather station
            Ws85 = 6,
        }
        impl SerialMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SerialMode::Default => "DEFAULT",
                    SerialMode::Simple => "SIMPLE",
                    SerialMode::Proto => "PROTO",
                    SerialMode::Textmsg => "TEXTMSG",
                    SerialMode::Nmea => "NMEA",
                    SerialMode::Caltopo => "CALTOPO",
                    SerialMode::Ws85 => "WS85",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "DEFAULT" => Some(Self::Default),
                    "SIMPLE" => Some(Self::Simple),
                    "PROTO" => Some(Self::Proto),
                    "TEXTMSG" => Some(Self::Textmsg),
                    "NMEA" => Some(Self::Nmea),
                    "CALTOPO" => Some(Self::Caltopo),
                    "WS85" => Some(Self::Ws85),
                    _ => None,
                }
            }
        }
    }
    ///
    /// External Notifications Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExternalNotificationConfig {
        ///
        /// Enable the ExternalNotificationModule
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// When using in On/Off mode, keep the output on for this many
        /// milliseconds. Default 1000ms (1 second).
        #[prost(uint32, tag = "2")]
        pub output_ms: u32,
        ///
        /// Define the output pin GPIO setting Defaults to
        /// EXT_NOTIFY_OUT if set for the board.
        /// In standalone devices this pin should drive the LED to match the UI.
        #[prost(uint32, tag = "3")]
        pub output: u32,
        ///
        /// Optional: Define a secondary output pin for a vibra motor
        /// This is used in standalone devices to match the UI.
        #[prost(uint32, tag = "8")]
        pub output_vibra: u32,
        ///
        /// Optional: Define a tertiary output pin for an active buzzer
        /// This is used in standalone devices to to match the UI.
        #[prost(uint32, tag = "9")]
        pub output_buzzer: u32,
        ///
        /// IF this is true, the 'output' Pin will be pulled active high, false
        /// means active low.
        #[prost(bool, tag = "4")]
        pub active: bool,
        ///
        /// True: Alert when a text message arrives (output)
        #[prost(bool, tag = "5")]
        pub alert_message: bool,
        ///
        /// True: Alert when a text message arrives (output_vibra)
        #[prost(bool, tag = "10")]
        pub alert_message_vibra: bool,
        ///
        /// True: Alert when a text message arrives (output_buzzer)
        #[prost(bool, tag = "11")]
        pub alert_message_buzzer: bool,
        ///
        /// True: Alert when the bell character is received (output)
        #[prost(bool, tag = "6")]
        pub alert_bell: bool,
        ///
        /// True: Alert when the bell character is received (output_vibra)
        #[prost(bool, tag = "12")]
        pub alert_bell_vibra: bool,
        ///
        /// True: Alert when the bell character is received (output_buzzer)
        #[prost(bool, tag = "13")]
        pub alert_bell_buzzer: bool,
        ///
        /// use a PWM output instead of a simple on/off output. This will ignore
        /// the 'output', 'output_ms' and 'active' settings and use the
        /// device.buzzer_gpio instead.
        #[prost(bool, tag = "7")]
        pub use_pwm: bool,
        ///
        /// The notification will toggle with 'output_ms' for this time of seconds.
        /// Default is 0 which means don't repeat at all. 60 would mean blink
        /// and/or beep for 60 seconds
        #[prost(uint32, tag = "14")]
        pub nag_timeout: u32,
        ///
        /// When true, enables devices with native I2S audio output to use the RTTTL over speaker like a buzzer
        /// T-Watch S3 and T-Deck for example have this capability
        #[prost(bool, tag = "15")]
        pub use_i2s_as_buzzer: bool,
    }
    ///
    /// Store and Forward Module Config
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StoreForwardConfig {
        ///
        /// Enable the Store and Forward Module
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// TODO: REPLACE
        #[prost(bool, tag = "2")]
        pub heartbeat: bool,
        ///
        /// TODO: REPLACE
        #[prost(uint32, tag = "3")]
        pub records: u32,
        ///
        /// TODO: REPLACE
        #[prost(uint32, tag = "4")]
        pub history_return_max: u32,
        ///
        /// TODO: REPLACE
        #[prost(uint32, tag = "5")]
        pub history_return_window: u32,
        ///
        /// Set to true to let this node act as a server that stores received messages and resends them upon request.
        #[prost(bool, tag = "6")]
        pub is_server: bool,
    }
    ///
    /// Preferences for the RangeTestModule
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RangeTestConfig {
        ///
        /// Enable the Range Test Module
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        ///
        /// Send out range test messages from this node
        #[prost(uint32, tag = "2")]
        pub sender: u32,
        ///
        /// Bool value indicating that this node should save a RangeTest.csv file.
        /// ESP32 Only
        #[prost(bool, tag = "3")]
        pub save: bool,
    }
    ///
    /// Configuration for both device and environment metrics
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TelemetryConfig {
        ///
        /// Interval in seconds of how often we should try to send our
        /// device metrics to the mesh
        #[prost(uint32, tag = "1")]
        pub device_update_interval: u32,
        #[prost(uint32, tag = "2")]
        pub environment_update_interval: u32,
        ///
        /// Preferences for the Telemetry Module (Environment)
        /// Enable/Disable the telemetry measurement module measurement collection
        #[prost(bool, tag = "3")]
        pub environment_measurement_enabled: bool,
        ///
        /// Enable/Disable the telemetry measurement module on-device display
        #[prost(bool, tag = "4")]
        pub environment_screen_enabled: bool,
        ///
        /// We'll always read the sensor in Celsius, but sometimes we might want to
        /// display the results in Fahrenheit as a "user preference".
        #[prost(bool, tag = "5")]
        pub environment_display_fahrenheit: bool,
        ///
        /// Enable/Disable the air quality metrics
        #[prost(bool, tag = "6")]
        pub air_quality_enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// air quality metrics to the mesh
        #[prost(uint32, tag = "7")]
        pub air_quality_interval: u32,
        ///
        /// Enable/disable Power metrics
        #[prost(bool, tag = "8")]
        pub power_measurement_enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// power metrics to the mesh
        #[prost(uint32, tag = "9")]
        pub power_update_interval: u32,
        ///
        /// Enable/Disable the power measurement module on-device display
        #[prost(bool, tag = "10")]
        pub power_screen_enabled: bool,
        ///
        /// Preferences for the (Health) Telemetry Module
        /// Enable/Disable the telemetry measurement module measurement collection
        #[prost(bool, tag = "11")]
        pub health_measurement_enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// health metrics to the mesh
        #[prost(uint32, tag = "12")]
        pub health_update_interval: u32,
        ///
        /// Enable/Disable the health telemetry module on-device display
        #[prost(bool, tag = "13")]
        pub health_screen_enabled: bool,
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CannedMessageConfig {
        ///
        /// Enable the rotary encoder #1. This is a 'dumb' encoder sending pulses on both A and B pins while rotating.
        #[prost(bool, tag = "1")]
        pub rotary1_enabled: bool,
        ///
        /// GPIO pin for rotary encoder A port.
        #[prost(uint32, tag = "2")]
        pub inputbroker_pin_a: u32,
        ///
        /// GPIO pin for rotary encoder B port.
        #[prost(uint32, tag = "3")]
        pub inputbroker_pin_b: u32,
        ///
        /// GPIO pin for rotary encoder Press port.
        #[prost(uint32, tag = "4")]
        pub inputbroker_pin_press: u32,
        ///
        /// Generate input event on CW of this kind.
        #[prost(enumeration = "canned_message_config::InputEventChar", tag = "5")]
        pub inputbroker_event_cw: i32,
        ///
        /// Generate input event on CCW of this kind.
        #[prost(enumeration = "canned_message_config::InputEventChar", tag = "6")]
        pub inputbroker_event_ccw: i32,
        ///
        /// Generate input event on Press of this kind.
        #[prost(enumeration = "canned_message_config::InputEventChar", tag = "7")]
        pub inputbroker_event_press: i32,
        ///
        /// Enable the Up/Down/Select input device. Can be RAK rotary encoder or 3 buttons. Uses the a/b/press definitions from inputbroker.
        #[prost(bool, tag = "8")]
        pub updown1_enabled: bool,
        ///
        /// Enable/disable CannedMessageModule.
        #[prost(bool, tag = "9")]
        pub enabled: bool,
        ///
        /// Input event origin accepted by the canned message module.
        /// Can be e.g. "rotEnc1", "upDownEnc1", "scanAndSelect", "cardkb", "serialkb", or keyword "_any"
        #[prost(string, tag = "10")]
        pub allow_input_source: ::prost::alloc::string::String,
        ///
        /// CannedMessageModule also sends a bell character with the messages.
        /// ExternalNotificationModule can benefit from this feature.
        #[prost(bool, tag = "11")]
        pub send_bell: bool,
    }
    /// Nested message and enum types in `CannedMessageConfig`.
    pub mod canned_message_config {
        ///
        /// TODO: REPLACE
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[allow(clippy::doc_lazy_continuation)]
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum InputEventChar {
            ///
            /// TODO: REPLACE
            None = 0,
            ///
            /// TODO: REPLACE
            Up = 17,
            ///
            /// TODO: REPLACE
            Down = 18,
            ///
            /// TODO: REPLACE
            Left = 19,
            ///
            /// TODO: REPLACE
            Right = 20,
            ///
            /// '\n'
            Select = 10,
            ///
            /// TODO: REPLACE
            Back = 27,
            ///
            /// TODO: REPLACE
            Cancel = 24,
        }
        impl InputEventChar {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    InputEventChar::None => "NONE",
                    InputEventChar::Up => "UP",
                    InputEventChar::Down => "DOWN",
                    InputEventChar::Left => "LEFT",
                    InputEventChar::Right => "RIGHT",
                    InputEventChar::Select => "SELECT",
                    InputEventChar::Back => "BACK",
                    InputEventChar::Cancel => "CANCEL",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "NONE" => Some(Self::None),
                    "UP" => Some(Self::Up),
                    "DOWN" => Some(Self::Down),
                    "LEFT" => Some(Self::Left),
                    "RIGHT" => Some(Self::Right),
                    "SELECT" => Some(Self::Select),
                    "BACK" => Some(Self::Back),
                    "CANCEL" => Some(Self::Cancel),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Ambient Lighting Module - Settings for control of onboard LEDs to allow users to adjust the brightness levels and respective color levels.
    /// Initially created for the RAK14001 RGB LED module.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AmbientLightingConfig {
        ///
        /// Sets LED to on or off.
        #[prost(bool, tag = "1")]
        pub led_state: bool,
        ///
        /// Sets the current for the LED output. Default is 10.
        #[prost(uint32, tag = "2")]
        pub current: u32,
        ///
        /// Sets the red LED level. Values are 0-255.
        #[prost(uint32, tag = "3")]
        pub red: u32,
        ///
        /// Sets the green LED level. Values are 0-255.
        #[prost(uint32, tag = "4")]
        pub green: u32,
        ///
        /// Sets the blue LED level. Values are 0-255.
        #[prost(uint32, tag = "5")]
        pub blue: u32,
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "1")]
        Mqtt(MqttConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "2")]
        Serial(SerialConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "3")]
        ExternalNotification(ExternalNotificationConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "4")]
        StoreForward(StoreForwardConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "5")]
        RangeTest(RangeTestConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "6")]
        Telemetry(TelemetryConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "7")]
        CannedMessage(CannedMessageConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "8")]
        Audio(AudioConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "9")]
        RemoteHardware(RemoteHardwareConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "10")]
        NeighborInfo(NeighborInfoConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "11")]
        AmbientLighting(AmbientLightingConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "12")]
        DetectionSensor(DetectionSensorConfig),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "13")]
        Paxcounter(PaxcounterConfig),
    }
}
///
/// A GPIO pin definition for remote hardware module
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteHardwarePin {
    ///
    /// GPIO Pin number (must match Arduino)
    #[prost(uint32, tag = "1")]
    pub gpio_pin: u32,
    ///
    /// Name for the GPIO pin (i.e. Front gate, mailbox, etc)
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Type of GPIO access available to consumers on the mesh
    #[prost(enumeration = "RemoteHardwarePinType", tag = "3")]
    pub r#type: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RemoteHardwarePinType {
    ///
    /// Unset/unused
    Unknown = 0,
    ///
    /// GPIO pin can be read (if it is high / low)
    DigitalRead = 1,
    ///
    /// GPIO pin can be written to (high / low)
    DigitalWrite = 2,
}
impl RemoteHardwarePinType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RemoteHardwarePinType::Unknown => "UNKNOWN",
            RemoteHardwarePinType::DigitalRead => "DIGITAL_READ",
            RemoteHardwarePinType::DigitalWrite => "DIGITAL_WRITE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN" => Some(Self::Unknown),
            "DIGITAL_READ" => Some(Self::DigitalRead),
            "DIGITAL_WRITE" => Some(Self::DigitalWrite),
            _ => None,
        }
    }
}
///
/// For any new 'apps' that run on the device or via sister apps on phones/PCs they should pick and use a
/// unique 'portnum' for their application.
/// If you are making a new app using meshtastic, please send in a pull request to add your 'portnum' to this
/// master table.
/// PortNums should be assigned in the following range:
/// 0-63   Core Meshtastic use, do not use for third party apps
/// 64-127 Registered 3rd party apps, send in a pull request that adds a new entry to portnums.proto to  register your application
/// 256-511 Use one of these portnums for your private applications that you don't want to register publically
/// All other values are reserved.
/// Note: This was formerly a Type enum named 'typ' with the same id #
/// We have change to this 'portnum' based scheme for specifying app handlers for particular payloads.
/// This change is backwards compatible by treating the legacy OPAQUE/CLEAR_TEXT values identically.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PortNum {
    ///
    /// Deprecated: do not use in new code (formerly called OPAQUE)
    /// A message sent from a device outside of the mesh, in a form the mesh does not understand
    /// NOTE: This must be 0, because it is documented in IMeshService.aidl to be so
    /// ENCODING: binary undefined
    UnknownApp = 0,
    ///
    /// A simple UTF-8 text message, which even the little micros in the mesh
    /// can understand and show on their screen eventually in some circumstances
    /// even signal might send messages in this form (see below)
    /// ENCODING: UTF-8 Plaintext (?)
    TextMessageApp = 1,
    ///
    /// Reserved for built-in GPIO/example app.
    /// See remote_hardware.proto/HardwareMessage for details on the message sent/received to this port number
    /// ENCODING: Protobuf
    RemoteHardwareApp = 2,
    ///
    /// The built-in position messaging app.
    /// Payload is a Position message.
    /// ENCODING: Protobuf
    PositionApp = 3,
    ///
    /// The built-in user info app.
    /// Payload is a User message.
    /// ENCODING: Protobuf
    NodeinfoApp = 4,
    ///
    /// Protocol control packets for mesh protocol use.
    /// Payload is a Routing message.
    /// ENCODING: Protobuf
    RoutingApp = 5,
    ///
    /// Admin control packets.
    /// Payload is a AdminMessage message.
    /// ENCODING: Protobuf
    AdminApp = 6,
    ///
    /// Compressed TEXT_MESSAGE payloads.
    /// ENCODING: UTF-8 Plaintext (?) with Unishox2 Compression
    /// NOTE: The Device Firmware converts a TEXT_MESSAGE_APP to TEXT_MESSAGE_COMPRESSED_APP if the compressed
    /// payload is shorter. There's no need for app developers to do this themselves. Also the firmware will decompress
    /// any incoming TEXT_MESSAGE_COMPRESSED_APP payload and convert to TEXT_MESSAGE_APP.
    TextMessageCompressedApp = 7,
    ///
    /// Waypoint payloads.
    /// Payload is a Waypoint message.
    /// ENCODING: Protobuf
    WaypointApp = 8,
    ///
    /// Audio Payloads.
    /// Encapsulated codec2 packets. On 2.4 GHZ Bandwidths only for now
    /// ENCODING: codec2 audio frames
    /// NOTE: audio frames contain a 3 byte header (0xc0 0xde 0xc2) and a one byte marker for the decompressed bitrate.
    /// This marker comes from the 'moduleConfig.audio.bitrate' enum minus one.
    AudioApp = 9,
    ///
    /// Same as Text Message but originating from Detection Sensor Module.
    /// NOTE: This portnum traffic is not sent to the public MQTT starting at firmware version 2.2.9
    DetectionSensorApp = 10,
    ///
    /// Same as Text Message but used for critical alerts.
    AlertApp = 11,
    ///
    /// Provides a 'ping' service that replies to any packet it receives.
    /// Also serves as a small example module.
    /// ENCODING: ASCII Plaintext
    ReplyApp = 32,
    ///
    /// Used for the python IP tunnel feature
    /// ENCODING: IP Packet. Handled by the python API, firmware ignores this one and pases on.
    IpTunnelApp = 33,
    ///
    /// Paxcounter lib included in the firmware
    /// ENCODING: protobuf
    PaxcounterApp = 34,
    ///
    /// Provides a hardware serial interface to send and receive from the Meshtastic network.
    /// Connect to the RX/TX pins of a device with 38400 8N1. Packets received from the Meshtastic
    /// network is forwarded to the RX pin while sending a packet to TX will go out to the Mesh network.
    /// Maximum packet size of 240 bytes.
    /// Module is disabled by default can be turned on by setting SERIAL_MODULE_ENABLED = 1 in SerialPlugh.cpp.
    /// ENCODING: binary undefined
    SerialApp = 64,
    ///
    /// STORE_FORWARD_APP (Work in Progress)
    /// Maintained by Jm Casler (MC Hamster) : jm@casler.org
    /// ENCODING: Protobuf
    StoreForwardApp = 65,
    ///
    /// Optional port for messages for the range test module.
    /// ENCODING: ASCII Plaintext
    /// NOTE: This portnum traffic is not sent to the public MQTT starting at firmware version 2.2.9
    RangeTestApp = 66,
    ///
    /// Provides a format to send and receive telemetry data from the Meshtastic network.
    /// Maintained by Charles Crossan (crossan007) : crossan007@gmail.com
    /// ENCODING: Protobuf
    TelemetryApp = 67,
    ///
    /// Experimental tools for estimating node position without a GPS
    /// Maintained by Github user a-f-G-U-C (a Meshtastic contributor)
    /// Project files at <https://github.com/a-f-G-U-C/Meshtastic-ZPS>
    /// ENCODING: arrays of int64 fields
    ZpsApp = 68,
    ///
    /// Used to let multiple instances of Linux native applications communicate
    /// as if they did using their LoRa chip.
    /// Maintained by GitHub user GUVWAF.
    /// Project files at <https://github.com/GUVWAF/Meshtasticator>
    /// ENCODING: Protobuf (?)
    SimulatorApp = 69,
    ///
    /// Provides a traceroute functionality to show the route a packet towards
    /// a certain destination would take on the mesh. Contains a RouteDiscovery message as payload.
    /// ENCODING: Protobuf
    TracerouteApp = 70,
    ///
    /// Aggregates edge info for the network by sending out a list of each node's neighbors
    /// ENCODING: Protobuf
    NeighborinfoApp = 71,
    ///
    /// ATAK Plugin
    /// Portnum for payloads from the official Meshtastic ATAK plugin
    AtakPlugin = 72,
    ///
    /// Provides unencrypted information about a node for consumption by a map via MQTT
    MapReportApp = 73,
    ///
    /// PowerStress based monitoring support (for automated power consumption testing)
    PowerstressApp = 74,
    ///
    /// Private applications should use portnums >= 256.
    /// To simplify initial development and testing you can use "PRIVATE_APP"
    /// in your code without needing to rebuild protobuf files (via \[regen-protos.sh\](<https://github.com/meshtastic/firmware/blob/master/bin/regen-protos.sh>))
    PrivateApp = 256,
    ///
    /// ATAK Forwarder Module <https://github.com/paulmandal/atak-forwarder>
    /// ENCODING: libcotshrink
    AtakForwarder = 257,
    ///
    /// Currently we limit port nums to no higher than this value
    Max = 511,
}
impl PortNum {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PortNum::UnknownApp => "UNKNOWN_APP",
            PortNum::TextMessageApp => "TEXT_MESSAGE_APP",
            PortNum::RemoteHardwareApp => "REMOTE_HARDWARE_APP",
            PortNum::PositionApp => "POSITION_APP",
            PortNum::NodeinfoApp => "NODEINFO_APP",
            PortNum::RoutingApp => "ROUTING_APP",
            PortNum::AdminApp => "ADMIN_APP",
            PortNum::TextMessageCompressedApp => "TEXT_MESSAGE_COMPRESSED_APP",
            PortNum::WaypointApp => "WAYPOINT_APP",
            PortNum::AudioApp => "AUDIO_APP",
            PortNum::DetectionSensorApp => "DETECTION_SENSOR_APP",
            PortNum::AlertApp => "ALERT_APP",
            PortNum::ReplyApp => "REPLY_APP",
            PortNum::IpTunnelApp => "IP_TUNNEL_APP",
            PortNum::PaxcounterApp => "PAXCOUNTER_APP",
            PortNum::SerialApp => "SERIAL_APP",
            PortNum::StoreForwardApp => "STORE_FORWARD_APP",
            PortNum::RangeTestApp => "RANGE_TEST_APP",
            PortNum::TelemetryApp => "TELEMETRY_APP",
            PortNum::ZpsApp => "ZPS_APP",
            PortNum::SimulatorApp => "SIMULATOR_APP",
            PortNum::TracerouteApp => "TRACEROUTE_APP",
            PortNum::NeighborinfoApp => "NEIGHBORINFO_APP",
            PortNum::AtakPlugin => "ATAK_PLUGIN",
            PortNum::MapReportApp => "MAP_REPORT_APP",
            PortNum::PowerstressApp => "POWERSTRESS_APP",
            PortNum::PrivateApp => "PRIVATE_APP",
            PortNum::AtakForwarder => "ATAK_FORWARDER",
            PortNum::Max => "MAX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNKNOWN_APP" => Some(Self::UnknownApp),
            "TEXT_MESSAGE_APP" => Some(Self::TextMessageApp),
            "REMOTE_HARDWARE_APP" => Some(Self::RemoteHardwareApp),
            "POSITION_APP" => Some(Self::PositionApp),
            "NODEINFO_APP" => Some(Self::NodeinfoApp),
            "ROUTING_APP" => Some(Self::RoutingApp),
            "ADMIN_APP" => Some(Self::AdminApp),
            "TEXT_MESSAGE_COMPRESSED_APP" => Some(Self::TextMessageCompressedApp),
            "WAYPOINT_APP" => Some(Self::WaypointApp),
            "AUDIO_APP" => Some(Self::AudioApp),
            "DETECTION_SENSOR_APP" => Some(Self::DetectionSensorApp),
            "ALERT_APP" => Some(Self::AlertApp),
            "REPLY_APP" => Some(Self::ReplyApp),
            "IP_TUNNEL_APP" => Some(Self::IpTunnelApp),
            "PAXCOUNTER_APP" => Some(Self::PaxcounterApp),
            "SERIAL_APP" => Some(Self::SerialApp),
            "STORE_FORWARD_APP" => Some(Self::StoreForwardApp),
            "RANGE_TEST_APP" => Some(Self::RangeTestApp),
            "TELEMETRY_APP" => Some(Self::TelemetryApp),
            "ZPS_APP" => Some(Self::ZpsApp),
            "SIMULATOR_APP" => Some(Self::SimulatorApp),
            "TRACEROUTE_APP" => Some(Self::TracerouteApp),
            "NEIGHBORINFO_APP" => Some(Self::NeighborinfoApp),
            "ATAK_PLUGIN" => Some(Self::AtakPlugin),
            "MAP_REPORT_APP" => Some(Self::MapReportApp),
            "POWERSTRESS_APP" => Some(Self::PowerstressApp),
            "PRIVATE_APP" => Some(Self::PrivateApp),
            "ATAK_FORWARDER" => Some(Self::AtakForwarder),
            "MAX" => Some(Self::Max),
            _ => None,
        }
    }
}
///
/// Key native device metrics such as battery level
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceMetrics {
    ///
    /// 0-100 (>100 means powered)
    #[prost(uint32, optional, tag = "1")]
    pub battery_level: ::core::option::Option<u32>,
    ///
    /// Voltage measured
    #[prost(float, optional, tag = "2")]
    pub voltage: ::core::option::Option<f32>,
    ///
    /// Utilization for the current channel, including well formed TX, RX and malformed RX (aka noise).
    #[prost(float, optional, tag = "3")]
    pub channel_utilization: ::core::option::Option<f32>,
    ///
    /// Percent of airtime for transmission used within the last hour.
    #[prost(float, optional, tag = "4")]
    pub air_util_tx: ::core::option::Option<f32>,
    ///
    /// How long the device has been running since the last reboot (in seconds)
    #[prost(uint32, optional, tag = "5")]
    pub uptime_seconds: ::core::option::Option<u32>,
}
///
/// Weather station or other environmental metrics
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentMetrics {
    ///
    /// Temperature measured
    #[prost(float, optional, tag = "1")]
    pub temperature: ::core::option::Option<f32>,
    ///
    /// Relative humidity percent measured
    #[prost(float, optional, tag = "2")]
    pub relative_humidity: ::core::option::Option<f32>,
    ///
    /// Barometric pressure in hPA measured
    #[prost(float, optional, tag = "3")]
    pub barometric_pressure: ::core::option::Option<f32>,
    ///
    /// Gas resistance in MOhm measured
    #[prost(float, optional, tag = "4")]
    pub gas_resistance: ::core::option::Option<f32>,
    ///
    /// Voltage measured (To be depreciated in favor of PowerMetrics in Meshtastic 3.x)
    #[prost(float, optional, tag = "5")]
    pub voltage: ::core::option::Option<f32>,
    ///
    /// Current measured (To be depreciated in favor of PowerMetrics in Meshtastic 3.x)
    #[prost(float, optional, tag = "6")]
    pub current: ::core::option::Option<f32>,
    ///
    /// relative scale IAQ value as measured by Bosch BME680 . value 0-500.
    /// Belongs to Air Quality but is not particle but VOC measurement. Other VOC values can also be put in here.
    #[prost(uint32, optional, tag = "7")]
    pub iaq: ::core::option::Option<u32>,
    ///
    /// RCWL9620 Doppler Radar Distance Sensor, used for water level detection. Float value in mm.
    #[prost(float, optional, tag = "8")]
    pub distance: ::core::option::Option<f32>,
    ///
    /// VEML7700 high accuracy ambient light(Lux) digital 16-bit resolution sensor.
    #[prost(float, optional, tag = "9")]
    pub lux: ::core::option::Option<f32>,
    ///
    /// VEML7700 high accuracy white light(irradiance) not calibrated digital 16-bit resolution sensor.
    #[prost(float, optional, tag = "10")]
    pub white_lux: ::core::option::Option<f32>,
    ///
    /// Infrared lux
    #[prost(float, optional, tag = "11")]
    pub ir_lux: ::core::option::Option<f32>,
    ///
    /// Ultraviolet lux
    #[prost(float, optional, tag = "12")]
    pub uv_lux: ::core::option::Option<f32>,
    ///
    /// Wind direction in degrees
    /// 0 degrees = North, 90 = East, etc...
    #[prost(uint32, optional, tag = "13")]
    pub wind_direction: ::core::option::Option<u32>,
    ///
    /// Wind speed in m/s
    #[prost(float, optional, tag = "14")]
    pub wind_speed: ::core::option::Option<f32>,
    ///
    /// Weight in KG
    #[prost(float, optional, tag = "15")]
    pub weight: ::core::option::Option<f32>,
    ///
    /// Wind gust in m/s
    #[prost(float, optional, tag = "16")]
    pub wind_gust: ::core::option::Option<f32>,
    ///
    /// Wind lull in m/s
    #[prost(float, optional, tag = "17")]
    pub wind_lull: ::core::option::Option<f32>,
    ///
    /// Radiation in µR/h
    #[prost(float, optional, tag = "18")]
    pub radiation: ::core::option::Option<f32>,
    ///
    /// Rainfall in the last hour in mm
    #[prost(float, optional, tag = "19")]
    pub rainfall_1h: ::core::option::Option<f32>,
    ///
    /// Rainfall in the last 24 hours in mm
    #[prost(float, optional, tag = "20")]
    pub rainfall_24h: ::core::option::Option<f32>,
}
///
/// Power Metrics (voltage / current / etc)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerMetrics {
    ///
    /// Voltage (Ch1)
    #[prost(float, optional, tag = "1")]
    pub ch1_voltage: ::core::option::Option<f32>,
    ///
    /// Current (Ch1)
    #[prost(float, optional, tag = "2")]
    pub ch1_current: ::core::option::Option<f32>,
    ///
    /// Voltage (Ch2)
    #[prost(float, optional, tag = "3")]
    pub ch2_voltage: ::core::option::Option<f32>,
    ///
    /// Current (Ch2)
    #[prost(float, optional, tag = "4")]
    pub ch2_current: ::core::option::Option<f32>,
    ///
    /// Voltage (Ch3)
    #[prost(float, optional, tag = "5")]
    pub ch3_voltage: ::core::option::Option<f32>,
    ///
    /// Current (Ch3)
    #[prost(float, optional, tag = "6")]
    pub ch3_current: ::core::option::Option<f32>,
}
///
/// Air quality metrics
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AirQualityMetrics {
    ///
    /// Concentration Units Standard PM1.0
    #[prost(uint32, optional, tag = "1")]
    pub pm10_standard: ::core::option::Option<u32>,
    ///
    /// Concentration Units Standard PM2.5
    #[prost(uint32, optional, tag = "2")]
    pub pm25_standard: ::core::option::Option<u32>,
    ///
    /// Concentration Units Standard PM10.0
    #[prost(uint32, optional, tag = "3")]
    pub pm100_standard: ::core::option::Option<u32>,
    ///
    /// Concentration Units Environmental PM1.0
    #[prost(uint32, optional, tag = "4")]
    pub pm10_environmental: ::core::option::Option<u32>,
    ///
    /// Concentration Units Environmental PM2.5
    #[prost(uint32, optional, tag = "5")]
    pub pm25_environmental: ::core::option::Option<u32>,
    ///
    /// Concentration Units Environmental PM10.0
    #[prost(uint32, optional, tag = "6")]
    pub pm100_environmental: ::core::option::Option<u32>,
    ///
    /// 0.3um Particle Count
    #[prost(uint32, optional, tag = "7")]
    pub particles_03um: ::core::option::Option<u32>,
    ///
    /// 0.5um Particle Count
    #[prost(uint32, optional, tag = "8")]
    pub particles_05um: ::core::option::Option<u32>,
    ///
    /// 1.0um Particle Count
    #[prost(uint32, optional, tag = "9")]
    pub particles_10um: ::core::option::Option<u32>,
    ///
    /// 2.5um Particle Count
    #[prost(uint32, optional, tag = "10")]
    pub particles_25um: ::core::option::Option<u32>,
    ///
    /// 5.0um Particle Count
    #[prost(uint32, optional, tag = "11")]
    pub particles_50um: ::core::option::Option<u32>,
    ///
    /// 10.0um Particle Count
    #[prost(uint32, optional, tag = "12")]
    pub particles_100um: ::core::option::Option<u32>,
    ///
    /// 10.0um Particle Count
    #[prost(uint32, optional, tag = "13")]
    pub co2: ::core::option::Option<u32>,
}
///
/// Local device mesh statistics
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalStats {
    ///
    /// How long the device has been running since the last reboot (in seconds)
    #[prost(uint32, tag = "1")]
    pub uptime_seconds: u32,
    ///
    /// Utilization for the current channel, including well formed TX, RX and malformed RX (aka noise).
    #[prost(float, tag = "2")]
    pub channel_utilization: f32,
    ///
    /// Percent of airtime for transmission used within the last hour.
    #[prost(float, tag = "3")]
    pub air_util_tx: f32,
    ///
    /// Number of packets sent
    #[prost(uint32, tag = "4")]
    pub num_packets_tx: u32,
    ///
    /// Number of packets received (both good and bad)
    #[prost(uint32, tag = "5")]
    pub num_packets_rx: u32,
    ///
    /// Number of packets received that are malformed or violate the protocol
    #[prost(uint32, tag = "6")]
    pub num_packets_rx_bad: u32,
    ///
    /// Number of nodes online (in the past 2 hours)
    #[prost(uint32, tag = "7")]
    pub num_online_nodes: u32,
    ///
    /// Number of nodes total
    #[prost(uint32, tag = "8")]
    pub num_total_nodes: u32,
    ///
    /// Number of received packets that were duplicates (due to multiple nodes relaying).
    /// If this number is high, there are nodes in the mesh relaying packets when it's unnecessary, for example due to the ROUTER/REPEATER role.
    #[prost(uint32, tag = "9")]
    pub num_rx_dupe: u32,
    ///
    /// Number of packets we transmitted that were a relay for others (not originating from ourselves).
    #[prost(uint32, tag = "10")]
    pub num_tx_relay: u32,
    ///
    /// Number of times we canceled a packet to be relayed, because someone else did it before us.
    /// This will always be zero for ROUTERs/REPEATERs. If this number is high, some other node(s) is/are relaying faster than you.
    #[prost(uint32, tag = "11")]
    pub num_tx_relay_canceled: u32,
}
///
/// Health telemetry metrics
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthMetrics {
    ///
    /// Heart rate (beats per minute)
    #[prost(uint32, optional, tag = "1")]
    pub heart_bpm: ::core::option::Option<u32>,
    ///
    /// SpO2 (blood oxygen saturation) level
    #[prost(uint32, optional, tag = "2")]
    pub sp_o2: ::core::option::Option<u32>,
    ///
    /// Body temperature in degrees Celsius
    #[prost(float, optional, tag = "3")]
    pub temperature: ::core::option::Option<f32>,
}
///
/// Types of Measurements the telemetry module is equipped to handle
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Telemetry {
    ///
    /// Seconds since 1970 - or 0 for unknown/unset
    #[prost(fixed32, tag = "1")]
    pub time: u32,
    #[prost(oneof = "telemetry::Variant", tags = "2, 3, 4, 5, 6, 7")]
    pub variant: ::core::option::Option<telemetry::Variant>,
}
/// Nested message and enum types in `Telemetry`.
pub mod telemetry {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        ///
        /// Key native device metrics such as battery level
        #[prost(message, tag = "2")]
        DeviceMetrics(super::DeviceMetrics),
        ///
        /// Weather station or other environmental metrics
        #[prost(message, tag = "3")]
        EnvironmentMetrics(super::EnvironmentMetrics),
        ///
        /// Air quality metrics
        #[prost(message, tag = "4")]
        AirQualityMetrics(super::AirQualityMetrics),
        ///
        /// Power Metrics
        #[prost(message, tag = "5")]
        PowerMetrics(super::PowerMetrics),
        ///
        /// Local device mesh statistics
        #[prost(message, tag = "6")]
        LocalStats(super::LocalStats),
        ///
        /// Health telemetry metrics
        #[prost(message, tag = "7")]
        HealthMetrics(super::HealthMetrics),
    }
}
///
/// NAU7802 Telemetry configuration, for saving to flash
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nau7802Config {
    ///
    /// The offset setting for the NAU7802
    #[prost(int32, tag = "1")]
    pub zero_offset: i32,
    ///
    /// The calibration factor for the NAU7802
    #[prost(float, tag = "2")]
    pub calibration_factor: f32,
}
///
/// Supported I2C Sensors for telemetry in Meshtastic
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TelemetrySensorType {
    ///
    /// No external telemetry sensor explicitly set
    SensorUnset = 0,
    ///
    /// High accuracy temperature, pressure, humidity
    Bme280 = 1,
    ///
    /// High accuracy temperature, pressure, humidity, and air resistance
    Bme680 = 2,
    ///
    /// Very high accuracy temperature
    Mcp9808 = 3,
    ///
    /// Moderate accuracy current and voltage
    Ina260 = 4,
    ///
    /// Moderate accuracy current and voltage
    Ina219 = 5,
    ///
    /// High accuracy temperature and pressure
    Bmp280 = 6,
    ///
    /// High accuracy temperature and humidity
    Shtc3 = 7,
    ///
    /// High accuracy pressure
    Lps22 = 8,
    ///
    /// 3-Axis magnetic sensor
    Qmc6310 = 9,
    ///
    /// 6-Axis inertial measurement sensor
    Qmi8658 = 10,
    ///
    /// 3-Axis magnetic sensor
    Qmc5883l = 11,
    ///
    /// High accuracy temperature and humidity
    Sht31 = 12,
    ///
    /// PM2.5 air quality sensor
    Pmsa003i = 13,
    ///
    /// INA3221 3 Channel Voltage / Current Sensor
    Ina3221 = 14,
    ///
    /// BMP085/BMP180 High accuracy temperature and pressure (older Version of BMP280)
    Bmp085 = 15,
    ///
    /// RCWL-9620 Doppler Radar Distance Sensor, used for water level detection
    Rcwl9620 = 16,
    ///
    /// Sensirion High accuracy temperature and humidity
    Sht4x = 17,
    ///
    /// VEML7700 high accuracy ambient light(Lux) digital 16-bit resolution sensor.
    Veml7700 = 18,
    ///
    /// MLX90632 non-contact IR temperature sensor.
    Mlx90632 = 19,
    ///
    /// TI OPT3001 Ambient Light Sensor
    Opt3001 = 20,
    ///
    /// Lite On LTR-390UV-01 UV Light Sensor
    Ltr390uv = 21,
    ///
    /// AMS TSL25911FN RGB Light Sensor
    Tsl25911fn = 22,
    ///
    /// AHT10 Integrated temperature and humidity sensor
    Aht10 = 23,
    ///
    /// DFRobot Lark Weather station (temperature, humidity, pressure, wind speed and direction)
    DfrobotLark = 24,
    ///
    /// NAU7802 Scale Chip or compatible
    Nau7802 = 25,
    ///
    /// BMP3XX High accuracy temperature and pressure
    Bmp3xx = 26,
    ///
    /// ICM-20948 9-Axis digital motion processor
    Icm20948 = 27,
    ///
    /// MAX17048 1S lipo battery sensor (voltage, state of charge, time to go)
    Max17048 = 28,
    ///
    /// Custom I2C sensor implementation based on <https://github.com/meshtastic/i2c-sensor>
    CustomSensor = 29,
    ///
    /// MAX30102 Pulse Oximeter and Heart-Rate Sensor
    Max30102 = 30,
    ///
    /// MLX90614 non-contact IR temperature sensor
    Mlx90614 = 31,
    ///
    /// SCD40/SCD41 CO2, humidity, temperature sensor
    Scd4x = 32,
    ///
    /// ClimateGuard RadSens, radiation, Geiger-Muller Tube
    Radsens = 33,
    ///
    /// High accuracy current and voltage
    Ina226 = 34,
    ///
    /// DFRobot Gravity tipping bucket rain gauge
    DfrobotRain = 35,
}
impl TelemetrySensorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TelemetrySensorType::SensorUnset => "SENSOR_UNSET",
            TelemetrySensorType::Bme280 => "BME280",
            TelemetrySensorType::Bme680 => "BME680",
            TelemetrySensorType::Mcp9808 => "MCP9808",
            TelemetrySensorType::Ina260 => "INA260",
            TelemetrySensorType::Ina219 => "INA219",
            TelemetrySensorType::Bmp280 => "BMP280",
            TelemetrySensorType::Shtc3 => "SHTC3",
            TelemetrySensorType::Lps22 => "LPS22",
            TelemetrySensorType::Qmc6310 => "QMC6310",
            TelemetrySensorType::Qmi8658 => "QMI8658",
            TelemetrySensorType::Qmc5883l => "QMC5883L",
            TelemetrySensorType::Sht31 => "SHT31",
            TelemetrySensorType::Pmsa003i => "PMSA003I",
            TelemetrySensorType::Ina3221 => "INA3221",
            TelemetrySensorType::Bmp085 => "BMP085",
            TelemetrySensorType::Rcwl9620 => "RCWL9620",
            TelemetrySensorType::Sht4x => "SHT4X",
            TelemetrySensorType::Veml7700 => "VEML7700",
            TelemetrySensorType::Mlx90632 => "MLX90632",
            TelemetrySensorType::Opt3001 => "OPT3001",
            TelemetrySensorType::Ltr390uv => "LTR390UV",
            TelemetrySensorType::Tsl25911fn => "TSL25911FN",
            TelemetrySensorType::Aht10 => "AHT10",
            TelemetrySensorType::DfrobotLark => "DFROBOT_LARK",
            TelemetrySensorType::Nau7802 => "NAU7802",
            TelemetrySensorType::Bmp3xx => "BMP3XX",
            TelemetrySensorType::Icm20948 => "ICM20948",
            TelemetrySensorType::Max17048 => "MAX17048",
            TelemetrySensorType::CustomSensor => "CUSTOM_SENSOR",
            TelemetrySensorType::Max30102 => "MAX30102",
            TelemetrySensorType::Mlx90614 => "MLX90614",
            TelemetrySensorType::Scd4x => "SCD4X",
            TelemetrySensorType::Radsens => "RADSENS",
            TelemetrySensorType::Ina226 => "INA226",
            TelemetrySensorType::DfrobotRain => "DFROBOT_RAIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SENSOR_UNSET" => Some(Self::SensorUnset),
            "BME280" => Some(Self::Bme280),
            "BME680" => Some(Self::Bme680),
            "MCP9808" => Some(Self::Mcp9808),
            "INA260" => Some(Self::Ina260),
            "INA219" => Some(Self::Ina219),
            "BMP280" => Some(Self::Bmp280),
            "SHTC3" => Some(Self::Shtc3),
            "LPS22" => Some(Self::Lps22),
            "QMC6310" => Some(Self::Qmc6310),
            "QMI8658" => Some(Self::Qmi8658),
            "QMC5883L" => Some(Self::Qmc5883l),
            "SHT31" => Some(Self::Sht31),
            "PMSA003I" => Some(Self::Pmsa003i),
            "INA3221" => Some(Self::Ina3221),
            "BMP085" => Some(Self::Bmp085),
            "RCWL9620" => Some(Self::Rcwl9620),
            "SHT4X" => Some(Self::Sht4x),
            "VEML7700" => Some(Self::Veml7700),
            "MLX90632" => Some(Self::Mlx90632),
            "OPT3001" => Some(Self::Opt3001),
            "LTR390UV" => Some(Self::Ltr390uv),
            "TSL25911FN" => Some(Self::Tsl25911fn),
            "AHT10" => Some(Self::Aht10),
            "DFROBOT_LARK" => Some(Self::DfrobotLark),
            "NAU7802" => Some(Self::Nau7802),
            "BMP3XX" => Some(Self::Bmp3xx),
            "ICM20948" => Some(Self::Icm20948),
            "MAX17048" => Some(Self::Max17048),
            "CUSTOM_SENSOR" => Some(Self::CustomSensor),
            "MAX30102" => Some(Self::Max30102),
            "MLX90614" => Some(Self::Mlx90614),
            "SCD4X" => Some(Self::Scd4x),
            "RADSENS" => Some(Self::Radsens),
            "INA226" => Some(Self::Ina226),
            "DFROBOT_RAIN" => Some(Self::DfrobotRain),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XModem {
    #[prost(enumeration = "x_modem::Control", tag = "1")]
    pub control: i32,
    #[prost(uint32, tag = "2")]
    pub seq: u32,
    #[prost(uint32, tag = "3")]
    pub crc16: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub buffer: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `XModem`.
pub mod x_modem {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Control {
        Nul = 0,
        Soh = 1,
        Stx = 2,
        Eot = 4,
        Ack = 6,
        Nak = 21,
        Can = 24,
        Ctrlz = 26,
    }
    impl Control {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Control::Nul => "NUL",
                Control::Soh => "SOH",
                Control::Stx => "STX",
                Control::Eot => "EOT",
                Control::Ack => "ACK",
                Control::Nak => "NAK",
                Control::Can => "CAN",
                Control::Ctrlz => "CTRLZ",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NUL" => Some(Self::Nul),
                "SOH" => Some(Self::Soh),
                "STX" => Some(Self::Stx),
                "EOT" => Some(Self::Eot),
                "ACK" => Some(Self::Ack),
                "NAK" => Some(Self::Nak),
                "CAN" => Some(Self::Can),
                "CTRLZ" => Some(Self::Ctrlz),
                _ => None,
            }
        }
    }
}
///
/// A GPS Position
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Position {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[prost(sfixed32, optional, tag = "1")]
    pub latitude_i: ::core::option::Option<i32>,
    ///
    /// TODO: REPLACE
    #[prost(sfixed32, optional, tag = "2")]
    pub longitude_i: ::core::option::Option<i32>,
    ///
    /// In meters above MSL (but see issue #359)
    #[prost(int32, optional, tag = "3")]
    pub altitude: ::core::option::Option<i32>,
    ///
    /// This is usually not sent over the mesh (to save space), but it is sent
    /// from the phone so that the local device can set its time if it is sent over
    /// the mesh (because there are devices on the mesh without GPS or RTC).
    /// seconds since 1970
    #[prost(fixed32, tag = "4")]
    pub time: u32,
    ///
    /// TODO: REPLACE
    #[prost(enumeration = "position::LocSource", tag = "5")]
    pub location_source: i32,
    ///
    /// TODO: REPLACE
    #[prost(enumeration = "position::AltSource", tag = "6")]
    pub altitude_source: i32,
    ///
    /// Positional timestamp (actual timestamp of GPS solution) in integer epoch seconds
    #[prost(fixed32, tag = "7")]
    pub timestamp: u32,
    ///
    /// Pos. timestamp milliseconds adjustment (rarely available or required)
    #[prost(int32, tag = "8")]
    pub timestamp_millis_adjust: i32,
    ///
    /// HAE altitude in meters - can be used instead of MSL altitude
    #[prost(sint32, optional, tag = "9")]
    pub altitude_hae: ::core::option::Option<i32>,
    ///
    /// Geoidal separation in meters
    #[prost(sint32, optional, tag = "10")]
    pub altitude_geoidal_separation: ::core::option::Option<i32>,
    ///
    /// Horizontal, Vertical and Position Dilution of Precision, in 1/100 units
    /// - PDOP is sufficient for most cases
    /// - for higher precision scenarios, HDOP and VDOP can be used instead,
    ///    in which case PDOP becomes redundant (PDOP=sqrt(HDOP^2 + VDOP^2))
    /// TODO: REMOVE/INTEGRATE
    #[prost(uint32, tag = "11")]
    pub pdop: u32,
    ///
    /// TODO: REPLACE
    #[prost(uint32, tag = "12")]
    pub hdop: u32,
    ///
    /// TODO: REPLACE
    #[prost(uint32, tag = "13")]
    pub vdop: u32,
    ///
    /// GPS accuracy (a hardware specific constant) in mm
    ///    multiplied with DOP to calculate positional accuracy
    /// Default: "'bout three meters-ish" :)
    #[prost(uint32, tag = "14")]
    pub gps_accuracy: u32,
    ///
    /// Ground speed in m/s and True North TRACK in 1/100 degrees
    /// Clarification of terms:
    /// - "track" is the direction of motion (measured in horizontal plane)
    /// - "heading" is where the fuselage points (measured in horizontal plane)
    /// - "yaw" indicates a relative rotation about the vertical axis
    /// TODO: REMOVE/INTEGRATE
    #[prost(uint32, optional, tag = "15")]
    pub ground_speed: ::core::option::Option<u32>,
    ///
    /// TODO: REPLACE
    #[prost(uint32, optional, tag = "16")]
    pub ground_track: ::core::option::Option<u32>,
    ///
    /// GPS fix quality (from NMEA GxGGA statement or similar)
    #[prost(uint32, tag = "17")]
    pub fix_quality: u32,
    ///
    /// GPS fix type 2D/3D (from NMEA GxGSA statement)
    #[prost(uint32, tag = "18")]
    pub fix_type: u32,
    ///
    /// GPS "Satellites in View" number
    #[prost(uint32, tag = "19")]
    pub sats_in_view: u32,
    ///
    /// Sensor ID - in case multiple positioning sensors are being used
    #[prost(uint32, tag = "20")]
    pub sensor_id: u32,
    ///
    /// Estimated/expected time (in seconds) until next update:
    /// - if we update at fixed intervals of X seconds, use X
    /// - if we update at dynamic intervals (based on relative movement etc),
    ///    but "AT LEAST every Y seconds", use Y
    #[prost(uint32, tag = "21")]
    pub next_update: u32,
    ///
    /// A sequence number, incremented with each Position message to help
    ///    detect lost updates if needed
    #[prost(uint32, tag = "22")]
    pub seq_number: u32,
    ///
    /// Indicates the bits of precision set by the sending node
    #[prost(uint32, tag = "23")]
    pub precision_bits: u32,
}
/// Nested message and enum types in `Position`.
pub mod position {
    ///
    /// How the location was acquired: manual, onboard GPS, external (EUD) GPS
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum LocSource {
        ///
        /// TODO: REPLACE
        LocUnset = 0,
        ///
        /// TODO: REPLACE
        LocManual = 1,
        ///
        /// TODO: REPLACE
        LocInternal = 2,
        ///
        /// TODO: REPLACE
        LocExternal = 3,
    }
    impl LocSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocSource::LocUnset => "LOC_UNSET",
                LocSource::LocManual => "LOC_MANUAL",
                LocSource::LocInternal => "LOC_INTERNAL",
                LocSource::LocExternal => "LOC_EXTERNAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LOC_UNSET" => Some(Self::LocUnset),
                "LOC_MANUAL" => Some(Self::LocManual),
                "LOC_INTERNAL" => Some(Self::LocInternal),
                "LOC_EXTERNAL" => Some(Self::LocExternal),
                _ => None,
            }
        }
    }
    ///
    /// How the altitude was acquired: manual, GPS int/ext, etc
    /// Default: same as location_source if present
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum AltSource {
        ///
        /// TODO: REPLACE
        AltUnset = 0,
        ///
        /// TODO: REPLACE
        AltManual = 1,
        ///
        /// TODO: REPLACE
        AltInternal = 2,
        ///
        /// TODO: REPLACE
        AltExternal = 3,
        ///
        /// TODO: REPLACE
        AltBarometric = 4,
    }
    impl AltSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AltSource::AltUnset => "ALT_UNSET",
                AltSource::AltManual => "ALT_MANUAL",
                AltSource::AltInternal => "ALT_INTERNAL",
                AltSource::AltExternal => "ALT_EXTERNAL",
                AltSource::AltBarometric => "ALT_BAROMETRIC",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ALT_UNSET" => Some(Self::AltUnset),
                "ALT_MANUAL" => Some(Self::AltManual),
                "ALT_INTERNAL" => Some(Self::AltInternal),
                "ALT_EXTERNAL" => Some(Self::AltExternal),
                "ALT_BAROMETRIC" => Some(Self::AltBarometric),
                _ => None,
            }
        }
    }
}
///
/// Broadcast when a newly powered mesh node wants to find a node num it can use
/// Sent from the phone over bluetooth to set the user id for the owner of this node.
/// Also sent from nodes to each other when a new node signs on (so all clients can have this info)
/// The algorithm is as follows:
/// when a node starts up, it broadcasts their user and the normal flow is for all
/// other nodes to reply with their User as well (so the new node can build its nodedb)
/// If a node ever receives a User (not just the first broadcast) message where
/// the sender node number equals our node number, that indicates a collision has
/// occurred and the following steps should happen:
/// If the receiving node (that was already in the mesh)'s macaddr is LOWER than the
/// new User who just tried to sign in: it gets to keep its nodenum.
/// We send a broadcast message of OUR User (we use a broadcast so that the other node can
/// receive our message, considering we have the same id - it also serves to let
/// observers correct their nodedb) - this case is rare so it should be okay.
/// If any node receives a User where the macaddr is GTE than their local macaddr,
/// they have been vetoed and should pick a new random nodenum (filtering against
/// whatever it knows about the nodedb) and rebroadcast their User.
/// A few nodenums are reserved and will never be requested:
/// 0xff - broadcast
/// 0 through 3 - for future use
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    ///
    /// A globally unique ID string for this user.
    /// In the case of Signal that would mean +16504442323, for the default macaddr derived id it would be !<8 hexidecimal bytes>.
    /// Note: app developers are encouraged to also use the following standard
    /// node IDs "^all" (for broadcast), "^local" (for the locally connected node)
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    ///
    /// A full name for this user, i.e. "Kevin Hester"
    #[prost(string, tag = "2")]
    pub long_name: ::prost::alloc::string::String,
    ///
    /// A VERY short name, ideally two characters.
    /// Suitable for a tiny OLED screen
    #[prost(string, tag = "3")]
    pub short_name: ::prost::alloc::string::String,
    ///
    /// Deprecated in Meshtastic 2.1.x
    /// This is the addr of the radio.
    /// Not populated by the phone, but added by the esp32 when broadcasting
    #[deprecated]
    #[prost(bytes = "vec", tag = "4")]
    pub macaddr: ::prost::alloc::vec::Vec<u8>,
    ///
    /// TBEAM, HELTEC, etc...
    /// Starting in 1.2.11 moved to hw_model enum in the NodeInfo object.
    /// Apps will still need the string here for older builds
    /// (so OTA update can find the right image), but if the enum is available it will be used instead.
    #[prost(enumeration = "HardwareModel", tag = "5")]
    pub hw_model: i32,
    ///
    /// In some regions Ham radio operators have different bandwidth limitations than others.
    /// If this user is a licensed operator, set this flag.
    /// Also, "long_name" should be their licence number.
    #[prost(bool, tag = "6")]
    pub is_licensed: bool,
    ///
    /// Indicates that the user's role in the mesh
    #[prost(enumeration = "config::device_config::Role", tag = "7")]
    pub role: i32,
    ///
    /// The public key of the user's device.
    /// This is sent out to other nodes on the mesh to allow them to compute a shared secret key.
    #[prost(bytes = "vec", tag = "8")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
///
/// A message used in a traceroute
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RouteDiscovery {
    ///
    /// The list of nodenums this packet has visited so far to the destination.
    #[prost(fixed32, repeated, tag = "1")]
    pub route: ::prost::alloc::vec::Vec<u32>,
    ///
    /// The list of SNRs (in dB, scaled by 4) in the route towards the destination.
    #[prost(int32, repeated, tag = "2")]
    pub snr_towards: ::prost::alloc::vec::Vec<i32>,
    ///
    /// The list of nodenums the packet has visited on the way back from the destination.
    #[prost(fixed32, repeated, tag = "3")]
    pub route_back: ::prost::alloc::vec::Vec<u32>,
    ///
    /// The list of SNRs (in dB, scaled by 4) in the route back from the destination.
    #[prost(int32, repeated, tag = "4")]
    pub snr_back: ::prost::alloc::vec::Vec<i32>,
}
///
/// A Routing control Data packet handled by the routing module
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Routing {
    #[prost(oneof = "routing::Variant", tags = "1, 2, 3")]
    pub variant: ::core::option::Option<routing::Variant>,
}
/// Nested message and enum types in `Routing`.
pub mod routing {
    ///
    /// A failure in delivering a message (usually used for routing control messages, but might be provided in addition to ack.fail_id to provide
    /// details on the type of failure).
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Error {
        ///
        /// This message is not a failure
        None = 0,
        ///
        /// Our node doesn't have a route to the requested destination anymore.
        NoRoute = 1,
        ///
        /// We received a nak while trying to forward on your behalf
        GotNak = 2,
        ///
        /// TODO: REPLACE
        Timeout = 3,
        ///
        /// No suitable interface could be found for delivering this packet
        NoInterface = 4,
        ///
        /// We reached the max retransmission count (typically for naive flood routing)
        MaxRetransmit = 5,
        ///
        /// No suitable channel was found for sending this packet (i.e. was requested channel index disabled?)
        NoChannel = 6,
        ///
        /// The packet was too big for sending (exceeds interface MTU after encoding)
        TooLarge = 7,
        ///
        /// The request had want_response set, the request reached the destination node, but no service on that node wants to send a response
        /// (possibly due to bad channel permissions)
        NoResponse = 8,
        ///
        /// Cannot send currently because duty cycle regulations will be violated.
        DutyCycleLimit = 9,
        ///
        /// The application layer service on the remote node received your request, but considered your request somehow invalid
        BadRequest = 32,
        ///
        /// The application layer service on the remote node received your request, but considered your request not authorized
        /// (i.e you did not send the request on the required bound channel)
        NotAuthorized = 33,
        ///
        /// The client specified a PKI transport, but the node was unable to send the packet using PKI (and did not send the message at all)
        PkiFailed = 34,
        ///
        /// The receiving node does not have a Public Key to decode with
        PkiUnknownPubkey = 35,
        ///
        /// Admin packet otherwise checks out, but uses a bogus or expired session key
        AdminBadSessionKey = 36,
        ///
        /// Admin packet sent using PKC, but not from a public key on the admin key list
        AdminPublicKeyUnauthorized = 37,
    }
    impl Error {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Error::None => "NONE",
                Error::NoRoute => "NO_ROUTE",
                Error::GotNak => "GOT_NAK",
                Error::Timeout => "TIMEOUT",
                Error::NoInterface => "NO_INTERFACE",
                Error::MaxRetransmit => "MAX_RETRANSMIT",
                Error::NoChannel => "NO_CHANNEL",
                Error::TooLarge => "TOO_LARGE",
                Error::NoResponse => "NO_RESPONSE",
                Error::DutyCycleLimit => "DUTY_CYCLE_LIMIT",
                Error::BadRequest => "BAD_REQUEST",
                Error::NotAuthorized => "NOT_AUTHORIZED",
                Error::PkiFailed => "PKI_FAILED",
                Error::PkiUnknownPubkey => "PKI_UNKNOWN_PUBKEY",
                Error::AdminBadSessionKey => "ADMIN_BAD_SESSION_KEY",
                Error::AdminPublicKeyUnauthorized => "ADMIN_PUBLIC_KEY_UNAUTHORIZED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NONE" => Some(Self::None),
                "NO_ROUTE" => Some(Self::NoRoute),
                "GOT_NAK" => Some(Self::GotNak),
                "TIMEOUT" => Some(Self::Timeout),
                "NO_INTERFACE" => Some(Self::NoInterface),
                "MAX_RETRANSMIT" => Some(Self::MaxRetransmit),
                "NO_CHANNEL" => Some(Self::NoChannel),
                "TOO_LARGE" => Some(Self::TooLarge),
                "NO_RESPONSE" => Some(Self::NoResponse),
                "DUTY_CYCLE_LIMIT" => Some(Self::DutyCycleLimit),
                "BAD_REQUEST" => Some(Self::BadRequest),
                "NOT_AUTHORIZED" => Some(Self::NotAuthorized),
                "PKI_FAILED" => Some(Self::PkiFailed),
                "PKI_UNKNOWN_PUBKEY" => Some(Self::PkiUnknownPubkey),
                "ADMIN_BAD_SESSION_KEY" => Some(Self::AdminBadSessionKey),
                "ADMIN_PUBLIC_KEY_UNAUTHORIZED" => Some(Self::AdminPublicKeyUnauthorized),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        ///
        /// A route request going from the requester
        #[prost(message, tag = "1")]
        RouteRequest(super::RouteDiscovery),
        ///
        /// A route reply
        #[prost(message, tag = "2")]
        RouteReply(super::RouteDiscovery),
        ///
        /// A failure in delivering a message (usually used for routing control messages, but might be provided
        /// in addition to ack.fail_id to provide details on the type of failure).
        #[prost(enumeration = "Error", tag = "3")]
        ErrorReason(i32),
    }
}
///
/// (Formerly called SubPacket)
/// The payload portion fo a packet, this is the actual bytes that are sent
/// inside a radio packet (because from/to are broken out by the comms library)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    ///
    /// Formerly named typ and of type Type
    #[prost(enumeration = "PortNum", tag = "1")]
    pub portnum: i32,
    ///
    /// TODO: REPLACE
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    ///
    /// Not normally used, but for testing a sender can request that recipient
    /// responds in kind (i.e. if it received a position, it should unicast back it's position).
    /// Note: that if you set this on a broadcast you will receive many replies.
    #[prost(bool, tag = "3")]
    pub want_response: bool,
    ///
    /// The address of the destination node.
    /// This field is is filled in by the mesh radio device software, application
    /// layer software should never need it.
    /// RouteDiscovery messages _must_ populate this.
    /// Other message types might need to if they are doing multihop routing.
    #[prost(fixed32, tag = "4")]
    pub dest: u32,
    ///
    /// The address of the original sender for this message.
    /// This field should _only_ be populated for reliable multihop packets (to keep
    /// packets small).
    #[prost(fixed32, tag = "5")]
    pub source: u32,
    ///
    /// Only used in routing or response messages.
    /// Indicates the original message ID that this message is reporting failure on. (formerly called original_id)
    #[prost(fixed32, tag = "6")]
    pub request_id: u32,
    ///
    /// If set, this message is intened to be a reply to a previously sent message with the defined id.
    #[prost(fixed32, tag = "7")]
    pub reply_id: u32,
    ///
    /// Defaults to false. If true, then what is in the payload should be treated as an emoji like giving
    /// a message a heart or poop emoji.
    #[prost(fixed32, tag = "8")]
    pub emoji: u32,
    ///
    /// Bitfield for extra flags. First use is to indicate that user approves the packet being uploaded to MQTT.
    #[prost(uint32, optional, tag = "9")]
    pub bitfield: ::core::option::Option<u32>,
}
///
/// Waypoint message, used to share arbitrary locations across the mesh
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    ///
    /// Id of the waypoint
    #[prost(uint32, tag = "1")]
    pub id: u32,
    ///
    /// latitude_i
    #[prost(sfixed32, optional, tag = "2")]
    pub latitude_i: ::core::option::Option<i32>,
    ///
    /// longitude_i
    #[prost(sfixed32, optional, tag = "3")]
    pub longitude_i: ::core::option::Option<i32>,
    ///
    /// Time the waypoint is to expire (epoch)
    #[prost(uint32, tag = "4")]
    pub expire: u32,
    ///
    /// If greater than zero, treat the value as a nodenum only allowing them to update the waypoint.
    /// If zero, the waypoint is open to be edited by any member of the mesh.
    #[prost(uint32, tag = "5")]
    pub locked_to: u32,
    ///
    /// Name of the waypoint - max 30 chars
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    ///
    /// Description of the waypoint - max 100 chars
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    ///
    /// Designator icon for the waypoint in the form of a unicode emoji
    #[prost(fixed32, tag = "8")]
    pub icon: u32,
}
///
/// This message will be proxied over the PhoneAPI for the client to deliver to the MQTT server
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MqttClientProxyMessage {
    ///
    /// The MQTT topic this message will be sent /received on
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    ///
    /// Whether the message should be retained (or not)
    #[prost(bool, tag = "4")]
    pub retained: bool,
    ///
    /// The actual service envelope payload or text for mqtt pub / sub
    #[prost(oneof = "mqtt_client_proxy_message::PayloadVariant", tags = "2, 3")]
    pub payload_variant: ::core::option::Option<
        mqtt_client_proxy_message::PayloadVariant,
    >,
}
/// Nested message and enum types in `MqttClientProxyMessage`.
pub mod mqtt_client_proxy_message {
    ///
    /// The actual service envelope payload or text for mqtt pub / sub
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// Bytes
        #[prost(bytes, tag = "2")]
        Data(::prost::alloc::vec::Vec<u8>),
        ///
        /// Text
        #[prost(string, tag = "3")]
        Text(::prost::alloc::string::String),
    }
}
///
/// A packet envelope sent/received over the mesh
/// only payload_variant is sent in the payload portion of the LORA packet.
/// The other fields are either not sent at all, or sent in the special 16 byte LORA header.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeshPacket {
    ///
    /// The sending node number.
    /// Note: Our crypto implementation uses this field as well.
    /// See \[crypto\](/docs/overview/encryption) for details.
    #[prost(fixed32, tag = "1")]
    pub from: u32,
    ///
    /// The (immediate) destination for this packet
    #[prost(fixed32, tag = "2")]
    pub to: u32,
    ///
    /// (Usually) If set, this indicates the index in the secondary_channels table that this packet was sent/received on.
    /// If unset, packet was on the primary channel.
    /// A particular node might know only a subset of channels in use on the mesh.
    /// Therefore channel_index is inherently a local concept and meaningless to send between nodes.
    /// Very briefly, while sending and receiving deep inside the device Router code, this field instead
    /// contains the 'channel hash' instead of the index.
    /// This 'trick' is only used while the payload_variant is an 'encrypted'.
    #[prost(uint32, tag = "3")]
    pub channel: u32,
    ///
    /// A unique ID for this packet.
    /// Always 0 for no-ack packets or non broadcast packets (and therefore take zero bytes of space).
    /// Otherwise a unique ID for this packet, useful for flooding algorithms.
    /// ID only needs to be unique on a _per sender_ basis, and it only
    /// needs to be unique for a few minutes (long enough to last for the length of
    /// any ACK or the completion of a mesh broadcast flood).
    /// Note: Our crypto implementation uses this id as well.
    /// See \[crypto\](/docs/overview/encryption) for details.
    #[prost(fixed32, tag = "6")]
    pub id: u32,
    ///
    /// The time this message was received by the esp32 (secs since 1970).
    /// Note: this field is _never_ sent on the radio link itself (to save space) Times
    /// are typically not sent over the mesh, but they will be added to any Packet
    /// (chain of SubPacket) sent to the phone (so the phone can know exact time of reception)
    #[prost(fixed32, tag = "7")]
    pub rx_time: u32,
    ///
    /// *Never* sent over the radio links.
    /// Set during reception to indicate the SNR of this packet.
    /// Used to collect statistics on current link quality.
    #[prost(float, tag = "8")]
    pub rx_snr: f32,
    ///
    /// If unset treated as zero (no forwarding, send to direct neighbor nodes only)
    /// if 1, allow hopping through one node, etc...
    /// For our usecase real world topologies probably have a max of about 3.
    /// This field is normally placed into a few of bits in the header.
    #[prost(uint32, tag = "9")]
    pub hop_limit: u32,
    ///
    /// This packet is being sent as a reliable message, we would prefer it to arrive at the destination.
    /// We would like to receive a ack packet in response.
    /// Broadcasts messages treat this flag specially: Since acks for broadcasts would
    /// rapidly flood the channel, the normal ack behavior is suppressed.
    /// Instead, the original sender listens to see if at least one node is rebroadcasting this packet (because naive flooding algorithm).
    /// If it hears that the odds (given typical LoRa topologies) the odds are very high that every node should eventually receive the message.
    /// So FloodingRouter.cpp generates an implicit ack which is delivered to the original sender.
    /// If after some time we don't hear anyone rebroadcast our packet, we will timeout and retransmit, using the regular resend logic.
    /// Note: This flag is normally sent in a flag bit in the header when sent over the wire
    #[prost(bool, tag = "10")]
    pub want_ack: bool,
    ///
    /// The priority of this message for sending.
    /// See MeshPacket.Priority description for more details.
    #[prost(enumeration = "mesh_packet::Priority", tag = "11")]
    pub priority: i32,
    ///
    /// rssi of received packet. Only sent to phone for dispay purposes.
    #[prost(int32, tag = "12")]
    pub rx_rssi: i32,
    ///
    /// Describe if this message is delayed
    #[deprecated]
    #[prost(enumeration = "mesh_packet::Delayed", tag = "13")]
    pub delayed: i32,
    ///
    /// Describes whether this packet passed via MQTT somewhere along the path it currently took.
    #[prost(bool, tag = "14")]
    pub via_mqtt: bool,
    ///
    /// Hop limit with which the original packet started. Sent via LoRa using three bits in the unencrypted header.
    /// When receiving a packet, the difference between hop_start and hop_limit gives how many hops it traveled.
    #[prost(uint32, tag = "15")]
    pub hop_start: u32,
    ///
    /// Records the public key the packet was encrypted with, if applicable.
    #[prost(bytes = "vec", tag = "16")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
    ///
    /// Indicates whether the packet was en/decrypted using PKI
    #[prost(bool, tag = "17")]
    pub pki_encrypted: bool,
    ///
    /// Last byte of the node number of the node that should be used as the next hop in routing.
    /// Set by the firmware internally, clients are not supposed to set this.
    #[prost(uint32, tag = "18")]
    pub next_hop: u32,
    ///
    /// Last byte of the node number of the node that will relay/relayed this packet.
    /// Set by the firmware internally, clients are not supposed to set this.
    #[prost(uint32, tag = "19")]
    pub relay_node: u32,
    ///
    /// *Never* sent over the radio links.
    /// Timestamp after which this packet may be sent.
    /// Set by the firmware internally, clients are not supposed to set this.
    #[prost(uint32, tag = "20")]
    pub tx_after: u32,
    #[prost(oneof = "mesh_packet::PayloadVariant", tags = "4, 5")]
    pub payload_variant: ::core::option::Option<mesh_packet::PayloadVariant>,
}
/// Nested message and enum types in `MeshPacket`.
pub mod mesh_packet {
    ///
    /// The priority of this message for sending.
    /// Higher priorities are sent first (when managing the transmit queue).
    /// This field is never sent over the air, it is only used internally inside of a local device node.
    /// API clients (either on the local node or connected directly to the node)
    /// can set this parameter if necessary.
    /// (values must be <= 127 to keep protobuf field to one byte in size.
    /// Detailed background on this field:
    /// I noticed a funny side effect of lora being so slow: Usually when making
    /// a protocol there isn’t much need to use message priority to change the order
    /// of transmission (because interfaces are fairly fast).
    /// But for lora where packets can take a few seconds each, it is very important
    /// to make sure that critical packets are sent ASAP.
    /// In the case of meshtastic that means we want to send protocol acks as soon as possible
    /// (to prevent unneeded retransmissions), we want routing messages to be sent next,
    /// then messages marked as reliable and finally 'background' packets like periodic position updates.
    /// So I bit the bullet and implemented a new (internal - not sent over the air)
    /// field in MeshPacket called 'priority'.
    /// And the transmission queue in the router object is now a priority queue.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Priority {
        ///
        /// Treated as Priority.DEFAULT
        Unset = 0,
        ///
        /// TODO: REPLACE
        Min = 1,
        ///
        /// Background position updates are sent with very low priority -
        /// if the link is super congested they might not go out at all
        Background = 10,
        ///
        /// This priority is used for most messages that don't have a priority set
        Default = 64,
        ///
        /// If priority is unset but the message is marked as want_ack,
        /// assume it is important and use a slightly higher priority
        Reliable = 70,
        ///
        /// If priority is unset but the packet is a response to a request, we want it to get there relatively quickly.
        /// Furthermore, responses stop relaying packets directed to a node early.
        Response = 80,
        ///
        /// Higher priority for specific message types (portnums) to distinguish between other reliable packets.
        High = 100,
        ///
        /// Higher priority alert message used for critical alerts which take priority over other reliable packets.
        Alert = 110,
        ///
        /// Ack/naks are sent with very high priority to ensure that retransmission
        /// stops as soon as possible
        Ack = 120,
        ///
        /// TODO: REPLACE
        Max = 127,
    }
    impl Priority {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Priority::Unset => "UNSET",
                Priority::Min => "MIN",
                Priority::Background => "BACKGROUND",
                Priority::Default => "DEFAULT",
                Priority::Reliable => "RELIABLE",
                Priority::Response => "RESPONSE",
                Priority::High => "HIGH",
                Priority::Alert => "ALERT",
                Priority::Ack => "ACK",
                Priority::Max => "MAX",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "MIN" => Some(Self::Min),
                "BACKGROUND" => Some(Self::Background),
                "DEFAULT" => Some(Self::Default),
                "RELIABLE" => Some(Self::Reliable),
                "RESPONSE" => Some(Self::Response),
                "HIGH" => Some(Self::High),
                "ALERT" => Some(Self::Alert),
                "ACK" => Some(Self::Ack),
                "MAX" => Some(Self::Max),
                _ => None,
            }
        }
    }
    ///
    /// Identify if this is a delayed packet
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Delayed {
        ///
        /// If unset, the message is being sent in real time.
        NoDelay = 0,
        ///
        /// The message is delayed and was originally a broadcast
        Broadcast = 1,
        ///
        /// The message is delayed and was originally a direct message
        Direct = 2,
    }
    impl Delayed {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Delayed::NoDelay => "NO_DELAY",
                Delayed::Broadcast => "DELAYED_BROADCAST",
                Delayed::Direct => "DELAYED_DIRECT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NO_DELAY" => Some(Self::NoDelay),
                "DELAYED_BROADCAST" => Some(Self::Broadcast),
                "DELAYED_DIRECT" => Some(Self::Direct),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "4")]
        Decoded(super::Data),
        ///
        /// TODO: REPLACE
        #[prost(bytes, tag = "5")]
        Encrypted(::prost::alloc::vec::Vec<u8>),
    }
}
///
/// The bluetooth to device link:
/// Old BTLE protocol docs from TODO, merge in above and make real docs...
/// use protocol buffers, and NanoPB
/// messages from device to phone:
/// POSITION_UPDATE (..., time)
/// TEXT_RECEIVED(from, text, time)
/// OPAQUE_RECEIVED(from, payload, time) (for signal messages or other applications)
/// messages from phone to device:
/// SET_MYID(id, human readable long, human readable short) (send down the unique ID
/// string used for this node, a human readable string shown for that id, and a very
/// short human readable string suitable for oled screen) SEND_OPAQUE(dest, payload)
/// (for signal messages or other applications) SEND_TEXT(dest, text) Get all
/// nodes() (returns list of nodes, with full info, last time seen, loc, battery
/// level etc) SET_CONFIG (switches device to a new set of radio params and
/// preshared key, drops all existing nodes, force our node to rejoin this new group)
/// Full information about a node on the mesh
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfo {
    ///
    /// The node number
    #[prost(uint32, tag = "1")]
    pub num: u32,
    ///
    /// The user info for this node
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<User>,
    ///
    /// This position data. Note: before 1.2.14 we would also store the last time we've heard from this node in position.time, that is no longer true.
    /// Position.time now indicates the last time we received a POSITION from that node.
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<Position>,
    ///
    /// Returns the Signal-to-noise ratio (SNR) of the last received message,
    /// as measured by the receiver. Return SNR of the last received message in dB
    #[prost(float, tag = "4")]
    pub snr: f32,
    ///
    /// Set to indicate the last time we received a packet from this node
    #[prost(fixed32, tag = "5")]
    pub last_heard: u32,
    ///
    /// The latest device metrics for the node.
    #[prost(message, optional, tag = "6")]
    pub device_metrics: ::core::option::Option<DeviceMetrics>,
    ///
    /// local channel index we heard that node on. Only populated if its not the default channel.
    #[prost(uint32, tag = "7")]
    pub channel: u32,
    ///
    /// True if we witnessed the node over MQTT instead of LoRA transport
    #[prost(bool, tag = "8")]
    pub via_mqtt: bool,
    ///
    /// Number of hops away from us this node is (0 if direct neighbor)
    #[prost(uint32, optional, tag = "9")]
    pub hops_away: ::core::option::Option<u32>,
    ///
    /// True if node is in our favorites list
    /// Persists between NodeDB internal clean ups
    #[prost(bool, tag = "10")]
    pub is_favorite: bool,
    ///
    /// True if node is in our ignored list
    /// Persists between NodeDB internal clean ups
    #[prost(bool, tag = "11")]
    pub is_ignored: bool,
}
///
/// Unique local debugging info for this node
/// Note: we don't include position or the user info, because that will come in the
/// Sent to the phone in response to WantNodes.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MyNodeInfo {
    ///
    /// Tells the phone what our node number is, default starting value is
    /// lowbyte of macaddr, but it will be fixed if that is already in use
    #[prost(uint32, tag = "1")]
    pub my_node_num: u32,
    ///
    /// The total number of reboots this node has ever encountered
    /// (well - since the last time we discarded preferences)
    #[prost(uint32, tag = "8")]
    pub reboot_count: u32,
    ///
    /// The minimum app version that can talk to this device.
    /// Phone/PC apps should compare this to their build number and if too low tell the user they must update their app
    #[prost(uint32, tag = "11")]
    pub min_app_version: u32,
    ///
    /// Unique hardware identifier for this device
    #[prost(bytes = "vec", tag = "12")]
    pub device_id: ::prost::alloc::vec::Vec<u8>,
    ///
    /// The PlatformIO environment used to build this firmware
    #[prost(string, tag = "13")]
    pub pio_env: ::prost::alloc::string::String,
}
///
/// Debug output from the device.
/// To minimize the size of records inside the device code, if a time/source/level is not set
/// on the message it is assumed to be a continuation of the previously sent message.
/// This allows the device code to use fixed maxlen 64 byte strings for messages,
/// and then extend as needed by emitting multiple records.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRecord {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    ///
    /// Seconds since 1970 - or 0 for unknown/unset
    #[prost(fixed32, tag = "2")]
    pub time: u32,
    ///
    /// Usually based on thread name - if known
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    ///
    /// Not yet set
    #[prost(enumeration = "log_record::Level", tag = "4")]
    pub level: i32,
}
/// Nested message and enum types in `LogRecord`.
pub mod log_record {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Level {
        ///
        /// Log levels, chosen to match python logging conventions.
        Unset = 0,
        ///
        /// Log levels, chosen to match python logging conventions.
        Critical = 50,
        ///
        /// Log levels, chosen to match python logging conventions.
        Error = 40,
        ///
        /// Log levels, chosen to match python logging conventions.
        Warning = 30,
        ///
        /// Log levels, chosen to match python logging conventions.
        Info = 20,
        ///
        /// Log levels, chosen to match python logging conventions.
        Debug = 10,
        ///
        /// Log levels, chosen to match python logging conventions.
        Trace = 5,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::Unset => "UNSET",
                Level::Critical => "CRITICAL",
                Level::Error => "ERROR",
                Level::Warning => "WARNING",
                Level::Info => "INFO",
                Level::Debug => "DEBUG",
                Level::Trace => "TRACE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "CRITICAL" => Some(Self::Critical),
                "ERROR" => Some(Self::Error),
                "WARNING" => Some(Self::Warning),
                "INFO" => Some(Self::Info),
                "DEBUG" => Some(Self::Debug),
                "TRACE" => Some(Self::Trace),
                _ => None,
            }
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueueStatus {
    /// Last attempt to queue status, ErrorCode
    #[prost(int32, tag = "1")]
    pub res: i32,
    /// Free entries in the outgoing queue
    #[prost(uint32, tag = "2")]
    pub free: u32,
    /// Maximum entries in the outgoing queue
    #[prost(uint32, tag = "3")]
    pub maxlen: u32,
    /// What was mesh packet id that generated this response?
    #[prost(uint32, tag = "4")]
    pub mesh_packet_id: u32,
}
///
/// Packets from the radio to the phone will appear on the fromRadio characteristic.
/// It will support READ and NOTIFY. When a new packet arrives the device will BLE notify?
/// It will sit in that descriptor until consumed by the phone,
/// at which point the next item in the FIFO will be populated.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FromRadio {
    ///
    /// The packet id, used to allow the phone to request missing read packets from the FIFO,
    /// see our bluetooth docs
    #[prost(uint32, tag = "1")]
    pub id: u32,
    ///
    /// Log levels, chosen to match python logging conventions.
    #[prost(
        oneof = "from_radio::PayloadVariant",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17"
    )]
    pub payload_variant: ::core::option::Option<from_radio::PayloadVariant>,
}
/// Nested message and enum types in `FromRadio`.
pub mod from_radio {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// Log levels, chosen to match python logging conventions.
        #[prost(message, tag = "2")]
        Packet(super::MeshPacket),
        ///
        /// Tells the phone what our node number is, can be -1 if we've not yet joined a mesh.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[prost(message, tag = "3")]
        MyInfo(super::MyNodeInfo),
        ///
        /// One packet is sent for each node in the on radio DB
        /// starts over with the first node in our DB
        #[prost(message, tag = "4")]
        NodeInfo(super::NodeInfo),
        ///
        /// Include a part of the config (was: RadioConfig radio)
        #[prost(message, tag = "5")]
        Config(super::Config),
        ///
        /// Set to send debug console output over our protobuf stream
        #[prost(message, tag = "6")]
        LogRecord(super::LogRecord),
        ///
        /// Sent as true once the device has finished sending all of the responses to want_config
        /// recipient should check if this ID matches our original request nonce, if
        /// not, it means your config responses haven't started yet.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[prost(uint32, tag = "7")]
        ConfigCompleteId(u32),
        ///
        /// Sent to tell clients the radio has just rebooted.
        /// Set to true if present.
        /// Not used on all transports, currently just used for the serial console.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[prost(bool, tag = "8")]
        Rebooted(bool),
        ///
        /// Include module config
        #[prost(message, tag = "9")]
        ModuleConfig(super::ModuleConfig),
        ///
        /// One packet is sent for each channel
        #[prost(message, tag = "10")]
        Channel(super::Channel),
        ///
        /// Queue status info
        #[prost(message, tag = "11")]
        QueueStatus(super::QueueStatus),
        ///
        /// File Transfer Chunk
        #[prost(message, tag = "12")]
        XmodemPacket(super::XModem),
        ///
        /// Device metadata message
        #[prost(message, tag = "13")]
        Metadata(super::DeviceMetadata),
        ///
        /// MQTT Client Proxy Message (device sending to client / phone for publishing to MQTT)
        #[prost(message, tag = "14")]
        MqttClientProxyMessage(super::MqttClientProxyMessage),
        ///
        /// File system manifest messages
        #[prost(message, tag = "15")]
        FileInfo(super::FileInfo),
        ///
        /// Notification message to the client
        #[prost(message, tag = "16")]
        ClientNotification(super::ClientNotification),
        ///
        /// Persistent data for device-ui
        #[prost(message, tag = "17")]
        DeviceuiConfig(super::DeviceUiConfig),
    }
}
///
/// A notification message from the device to the client
/// To be used for important messages that should to be displayed to the user
/// in the form of push notifications or validation messages when saving
/// invalid configuration.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientNotification {
    ///
    /// The id of the packet we're notifying in response to
    #[prost(uint32, optional, tag = "1")]
    pub reply_id: ::core::option::Option<u32>,
    ///
    /// Seconds since 1970 - or 0 for unknown/unset
    #[prost(fixed32, tag = "2")]
    pub time: u32,
    ///
    /// The level type of notification
    #[prost(enumeration = "log_record::Level", tag = "3")]
    pub level: i32,
    ///
    /// The message body of the notification
    #[prost(string, tag = "4")]
    pub message: ::prost::alloc::string::String,
}
///
/// Individual File info for the device
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    ///
    /// The fully qualified path of the file
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    ///
    /// The size of the file in bytes
    #[prost(uint32, tag = "2")]
    pub size_bytes: u32,
}
///
/// Packets/commands to the radio will be written (reliably) to the toRadio characteristic.
/// Once the write completes the phone can assume it is handled.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToRadio {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[prost(oneof = "to_radio::PayloadVariant", tags = "1, 3, 4, 5, 6, 7")]
    pub payload_variant: ::core::option::Option<to_radio::PayloadVariant>,
}
/// Nested message and enum types in `ToRadio`.
pub mod to_radio {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// Send this packet on the mesh
        #[prost(message, tag = "1")]
        Packet(super::MeshPacket),
        ///
        /// Phone wants radio to send full node db to the phone, This is
        /// typically the first packet sent to the radio when the phone gets a
        /// bluetooth connection. The radio will respond by sending back a
        /// MyNodeInfo, a owner, a radio config and a series of
        /// FromRadio.node_infos, and config_complete
        /// the integer you write into this field will be reported back in the
        /// config_complete_id response this allows clients to never be confused by
        /// a stale old partially sent config.
        #[prost(uint32, tag = "3")]
        WantConfigId(u32),
        ///
        /// Tell API server we are disconnecting now.
        /// This is useful for serial links where there is no hardware/protocol based notification that the client has dropped the link.
        /// (Sending this message is optional for clients)
        #[prost(bool, tag = "4")]
        Disconnect(bool),
        #[prost(message, tag = "5")]
        XmodemPacket(super::XModem),
        ///
        /// MQTT Client Proxy Message (for client / phone subscribed to MQTT sending to device)
        #[prost(message, tag = "6")]
        MqttClientProxyMessage(super::MqttClientProxyMessage),
        ///
        /// Heartbeat message (used to keep the device connection awake on serial)
        #[prost(message, tag = "7")]
        Heartbeat(super::Heartbeat),
    }
}
///
/// Compressed message payload
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Compressed {
    ///
    /// PortNum to determine the how to handle the compressed payload.
    #[prost(enumeration = "PortNum", tag = "1")]
    pub portnum: i32,
    ///
    /// Compressed data.
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
///
/// Full info on edges for a single node
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeighborInfo {
    ///
    /// The node ID of the node sending info on its neighbors
    #[prost(uint32, tag = "1")]
    pub node_id: u32,
    ///
    /// Field to pass neighbor info for the next sending cycle
    #[prost(uint32, tag = "2")]
    pub last_sent_by_id: u32,
    ///
    /// Broadcast interval of the represented node (in seconds)
    #[prost(uint32, tag = "3")]
    pub node_broadcast_interval_secs: u32,
    ///
    /// The list of out edges from this node
    #[prost(message, repeated, tag = "4")]
    pub neighbors: ::prost::alloc::vec::Vec<Neighbor>,
}
///
/// A single edge in the mesh
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Neighbor {
    ///
    /// Node ID of neighbor
    #[prost(uint32, tag = "1")]
    pub node_id: u32,
    ///
    /// SNR of last heard message
    #[prost(float, tag = "2")]
    pub snr: f32,
    ///
    /// Reception time (in secs since 1970) of last message that was last sent by this ID.
    /// Note: this is for local storage only and will not be sent out over the mesh.
    #[prost(fixed32, tag = "3")]
    pub last_rx_time: u32,
    ///
    /// Broadcast interval of this neighbor (in seconds).
    /// Note: this is for local storage only and will not be sent out over the mesh.
    #[prost(uint32, tag = "4")]
    pub node_broadcast_interval_secs: u32,
}
///
/// Device metadata response
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceMetadata {
    ///
    /// Device firmware version string
    #[prost(string, tag = "1")]
    pub firmware_version: ::prost::alloc::string::String,
    ///
    /// Device state version
    #[prost(uint32, tag = "2")]
    pub device_state_version: u32,
    ///
    /// Indicates whether the device can shutdown CPU natively or via power management chip
    #[prost(bool, tag = "3")]
    pub can_shutdown: bool,
    ///
    /// Indicates that the device has native wifi capability
    #[prost(bool, tag = "4")]
    pub has_wifi: bool,
    ///
    /// Indicates that the device has native bluetooth capability
    #[prost(bool, tag = "5")]
    pub has_bluetooth: bool,
    ///
    /// Indicates that the device has an ethernet peripheral
    #[prost(bool, tag = "6")]
    pub has_ethernet: bool,
    ///
    /// Indicates that the device's role in the mesh
    #[prost(enumeration = "config::device_config::Role", tag = "7")]
    pub role: i32,
    ///
    /// Indicates the device's current enabled position flags
    #[prost(uint32, tag = "8")]
    pub position_flags: u32,
    ///
    /// Device hardware model
    #[prost(enumeration = "HardwareModel", tag = "9")]
    pub hw_model: i32,
    ///
    /// Has Remote Hardware enabled
    #[prost(bool, tag = "10")]
    pub has_remote_hardware: bool,
    ///
    /// Has PKC capabilities
    #[prost(bool, tag = "11")]
    pub has_pkc: bool,
    ///
    /// Bit field of boolean for excluded modules
    /// (bitwise OR of ExcludedModules)
    #[prost(uint32, tag = "12")]
    pub excluded_modules: u32,
}
///
/// A heartbeat message is sent to the node from the client to keep the connection alive.
/// This is currently only needed to keep serial connections alive, but can be used by any PhoneAPI.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Heartbeat {}
///
/// RemoteHardwarePins associated with a node
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeRemoteHardwarePin {
    ///
    /// The node_num exposing the available gpio pin
    #[prost(uint32, tag = "1")]
    pub node_num: u32,
    ///
    /// The the available gpio pin for usage with RemoteHardware module
    #[prost(message, optional, tag = "2")]
    pub pin: ::core::option::Option<RemoteHardwarePin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkedPayload {
    ///
    /// The ID of the entire payload
    #[prost(uint32, tag = "1")]
    pub payload_id: u32,
    ///
    /// The total number of chunks in the payload
    #[prost(uint32, tag = "2")]
    pub chunk_count: u32,
    ///
    /// The current chunk index in the total
    #[prost(uint32, tag = "3")]
    pub chunk_index: u32,
    ///
    /// The binary data of the current chunk
    #[prost(bytes = "vec", tag = "4")]
    pub payload_chunk: ::prost::alloc::vec::Vec<u8>,
}
///
/// Wrapper message for broken repeated oneof support
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendChunks {
    #[prost(uint32, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<u32>,
}
///
/// Responses to a ChunkedPayload request
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunkedPayloadResponse {
    ///
    /// The ID of the entire payload
    #[prost(uint32, tag = "1")]
    pub payload_id: u32,
    #[prost(oneof = "chunked_payload_response::PayloadVariant", tags = "2, 3, 4")]
    pub payload_variant: ::core::option::Option<
        chunked_payload_response::PayloadVariant,
    >,
}
/// Nested message and enum types in `ChunkedPayloadResponse`.
pub mod chunked_payload_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// Request to transfer chunked payload
        #[prost(bool, tag = "2")]
        RequestTransfer(bool),
        ///
        /// Accept the transfer chunked payload
        #[prost(bool, tag = "3")]
        AcceptTransfer(bool),
        ///
        /// Request missing indexes in the chunked payload
        #[prost(message, tag = "4")]
        ResendChunks(super::ResendChunks),
    }
}
///
/// Note: these enum names must EXACTLY match the string used in the device
/// bin/build-all.sh script.
/// Because they will be used to find firmware filenames in the android app for OTA updates.
/// To match the old style filenames, _ is converted to -, p is converted to .
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HardwareModel {
    ///
    /// TODO: REPLACE
    Unset = 0,
    ///
    /// TODO: REPLACE
    TloraV2 = 1,
    ///
    /// TODO: REPLACE
    TloraV1 = 2,
    ///
    /// TODO: REPLACE
    TloraV211p6 = 3,
    ///
    /// TODO: REPLACE
    Tbeam = 4,
    ///
    /// The original heltec WiFi_Lora_32_V2, which had battery voltage sensing hooked to GPIO 13
    /// (see HELTEC_V2 for the new version).
    HeltecV20 = 5,
    ///
    /// TODO: REPLACE
    TbeamV0p7 = 6,
    ///
    /// TODO: REPLACE
    TEcho = 7,
    ///
    /// TODO: REPLACE
    TloraV11p3 = 8,
    ///
    /// TODO: REPLACE
    Rak4631 = 9,
    ///
    /// The new version of the heltec WiFi_Lora_32_V2 board that has battery sensing hooked to GPIO 37.
    /// Sadly they did not update anything on the silkscreen to identify this board
    HeltecV21 = 10,
    ///
    /// Ancient heltec WiFi_Lora_32 board
    HeltecV1 = 11,
    ///
    /// New T-BEAM with ESP32-S3 CPU
    LilygoTbeamS3Core = 12,
    ///
    /// RAK WisBlock ESP32 core: <https://docs.rakwireless.com/Product-Categories/WisBlock/RAK11200/Overview/>
    Rak11200 = 13,
    ///
    /// B&Q Consulting Nano Edition G1: <https://uniteng.com/wiki/doku.php?id=meshtastic:nano>
    NanoG1 = 14,
    ///
    /// TODO: REPLACE
    TloraV211p8 = 15,
    ///
    /// TODO: REPLACE
    TloraT3S3 = 16,
    ///
    /// B&Q Consulting Nano G1 Explorer: <https://wiki.uniteng.com/en/meshtastic/nano-g1-explorer>
    NanoG1Explorer = 17,
    ///
    /// B&Q Consulting Nano G2 Ultra: <https://wiki.uniteng.com/en/meshtastic/nano-g2-ultra>
    NanoG2Ultra = 18,
    ///
    /// LoRAType device: <https://loratype.org/>
    LoraType = 19,
    ///
    /// wiphone <https://www.wiphone.io/>
    Wiphone = 20,
    ///
    /// WIO Tracker WM1110 family from Seeed Studio. Includes wio-1110-tracker and wio-1110-sdk
    WioWm1110 = 21,
    ///
    /// RAK2560 Solar base station based on RAK4630
    Rak2560 = 22,
    ///
    /// Heltec HRU-3601: <https://heltec.org/project/hru-3601/>
    HeltecHru3601 = 23,
    ///
    /// Heltec Wireless Bridge
    HeltecWirelessBridge = 24,
    ///
    /// B&Q Consulting Station Edition G1: <https://uniteng.com/wiki/doku.php?id=meshtastic:station>
    StationG1 = 25,
    ///
    /// RAK11310 (RP2040 + SX1262)
    Rak11310 = 26,
    ///
    /// Makerfabs SenseLoRA Receiver (RP2040 + RFM96)
    SenseloraRp2040 = 27,
    ///
    /// Makerfabs SenseLoRA Industrial Monitor (ESP32-S3 + RFM96)
    SenseloraS3 = 28,
    ///
    /// Canary Radio Company - CanaryOne: <https://canaryradio.io/products/canaryone>
    Canaryone = 29,
    ///
    /// Waveshare RP2040 LoRa - <https://www.waveshare.com/rp2040-lora.htm>
    Rp2040Lora = 30,
    ///
    /// B&Q Consulting Station G2: <https://wiki.uniteng.com/en/meshtastic/station-g2>
    StationG2 = 31,
    ///
    /// ---------------------------------------------------------------------------
    /// Less common/prototype boards listed here (needs one more byte over the air)
    /// ---------------------------------------------------------------------------
    LoraRelayV1 = 32,
    ///
    /// TODO: REPLACE
    Nrf52840dk = 33,
    ///
    /// TODO: REPLACE
    Ppr = 34,
    ///
    /// TODO: REPLACE
    Genieblocks = 35,
    ///
    /// TODO: REPLACE
    Nrf52Unknown = 36,
    ///
    /// TODO: REPLACE
    Portduino = 37,
    ///
    /// The simulator built into the android app
    AndroidSim = 38,
    ///
    /// Custom DIY device based on @NanoVHF schematics: <https://github.com/NanoVHF/Meshtastic-DIY/tree/main/Schematics>
    DiyV1 = 39,
    ///
    /// nRF52840 Dongle : <https://www.nordicsemi.com/Products/Development-hardware/nrf52840-dongle/>
    Nrf52840Pca10059 = 40,
    ///
    /// Custom Disaster Radio esp32 v3 device <https://github.com/sudomesh/disaster-radio/tree/master/hardware/board_esp32_v3>
    DrDev = 41,
    ///
    /// M5 esp32 based MCU modules with enclosure, TFT and LORA Shields. All Variants (Basic, Core, Fire, Core2, CoreS3, Paper) <https://m5stack.com/>
    M5stack = 42,
    ///
    /// New Heltec LoRA32 with ESP32-S3 CPU
    HeltecV3 = 43,
    ///
    /// New Heltec Wireless Stick Lite with ESP32-S3 CPU
    HeltecWslV3 = 44,
    ///
    /// New BETAFPV ELRS Micro TX Module 2.4G with ESP32 CPU
    Betafpv2400Tx = 45,
    ///
    /// BetaFPV ExpressLRS "Nano" TX Module 900MHz with ESP32 CPU
    Betafpv900NanoTx = 46,
    ///
    /// Raspberry Pi Pico (W) with Waveshare SX1262 LoRa Node Module
    RpiPico = 47,
    ///
    /// Heltec Wireless Tracker with ESP32-S3 CPU, built-in GPS, and TFT
    /// Newer V1.1, version is written on the PCB near the display.
    HeltecWirelessTracker = 48,
    ///
    /// Heltec Wireless Paper with ESP32-S3 CPU and E-Ink display
    HeltecWirelessPaper = 49,
    ///
    /// LilyGo T-Deck with ESP32-S3 CPU, Keyboard and IPS display
    TDeck = 50,
    ///
    /// LilyGo T-Watch S3 with ESP32-S3 CPU and IPS display
    TWatchS3 = 51,
    ///
    /// Bobricius Picomputer with ESP32-S3 CPU, Keyboard and IPS display
    PicomputerS3 = 52,
    ///
    /// Heltec HT-CT62 with ESP32-C3 CPU and SX1262 LoRa
    HeltecHt62 = 53,
    ///
    /// EBYTE SPI LoRa module and ESP32-S3
    EbyteEsp32S3 = 54,
    ///
    /// Waveshare ESP32-S3-PICO with PICO LoRa HAT and 2.9inch e-Ink
    Esp32S3Pico = 55,
    ///
    /// CircuitMess Chatter 2 LLCC68 Lora Module and ESP32 Wroom
    /// Lora module can be swapped out for a Heltec RA-62 which is "almost" pin compatible
    /// with one cut and one jumper Meshtastic works
    Chatter2 = 56,
    ///
    /// Heltec Wireless Paper, With ESP32-S3 CPU and E-Ink display
    /// Older "V1.0" Variant, has no "version sticker"
    /// E-Ink model is DEPG0213BNS800
    /// Tab on the screen protector is RED
    /// Flex connector marking is FPC-7528B
    HeltecWirelessPaperV10 = 57,
    ///
    /// Heltec Wireless Tracker with ESP32-S3 CPU, built-in GPS, and TFT
    /// Older "V1.0" Variant
    HeltecWirelessTrackerV10 = 58,
    ///
    /// unPhone with ESP32-S3, TFT touchscreen,  LSM6DS3TR-C accelerometer and gyroscope
    Unphone = 59,
    ///
    /// Teledatics TD-LORAC NRF52840 based M.2 LoRA module
    /// Compatible with the TD-WRLS development board
    TdLorac = 60,
    ///
    /// CDEBYTE EoRa-S3 board using their own MM modules, clone of LILYGO T3S3
    CdebyteEoraS3 = 61,
    ///
    /// TWC_MESH_V4
    /// Adafruit NRF52840 feather express with SX1262, SSD1306 OLED and NEO6M GPS
    TwcMeshV4 = 62,
    ///
    /// NRF52_PROMICRO_DIY
    /// Promicro NRF52840 with SX1262/LLCC68, SSD1306 OLED and NEO6M GPS
    Nrf52PromicroDiy = 63,
    ///
    /// RadioMaster 900 Bandit Nano, <https://www.radiomasterrc.com/products/bandit-nano-expresslrs-rf-module>
    /// ESP32-D0WDQ6 With SX1276/SKY66122, SSD1306 OLED and No GPS
    Radiomaster900BanditNano = 64,
    ///
    /// Heltec Capsule Sensor V3 with ESP32-S3 CPU, Portable LoRa device that can replace GNSS modules or sensors
    HeltecCapsuleSensorV3 = 65,
    ///
    /// Heltec Vision Master T190 with ESP32-S3 CPU, and a 1.90 inch TFT display
    HeltecVisionMasterT190 = 66,
    ///
    /// Heltec Vision Master E213 with ESP32-S3 CPU, and a 2.13 inch E-Ink display
    HeltecVisionMasterE213 = 67,
    ///
    /// Heltec Vision Master E290 with ESP32-S3 CPU, and a 2.9 inch E-Ink display
    HeltecVisionMasterE290 = 68,
    ///
    /// Heltec Mesh Node T114 board with nRF52840 CPU, and a 1.14 inch TFT display, Ultimate low-power design,
    /// specifically adapted for the Meshtatic project
    HeltecMeshNodeT114 = 69,
    ///
    /// Sensecap Indicator from Seeed Studio. ESP32-S3 device with TFT and RP2040 coprocessor
    SensecapIndicator = 70,
    ///
    /// Seeed studio T1000-E tracker card. NRF52840 w/ LR1110 radio, GPS, button, buzzer, and sensors.
    TrackerT1000E = 71,
    ///
    /// RAK3172 STM32WLE5 Module (<https://store.rakwireless.com/products/wisduo-lpwan-module-rak3172>)
    Rak3172 = 72,
    ///
    /// Seeed Studio Wio-E5 (either mini or Dev kit) using STM32WL chip.
    WioE5 = 73,
    ///
    /// RadioMaster 900 Bandit, <https://www.radiomasterrc.com/products/bandit-expresslrs-rf-module>
    /// SSD1306 OLED and No GPS
    Radiomaster900Bandit = 74,
    ///
    /// Minewsemi ME25LS01 (ME25LE01_V1.0). NRF52840 w/ LR1110 radio, buttons and leds and pins.
    Me25ls014y10td = 75,
    ///
    /// RP2040_FEATHER_RFM95
    /// Adafruit Feather RP2040 with RFM95 LoRa Radio RFM95 with SX1272, SSD1306 OLED
    /// <https://www.adafruit.com/product/5714>
    /// <https://www.adafruit.com/product/326>
    /// <https://www.adafruit.com/product/938>
    ///   ^^^ short A0 to switch to I2C address 0x3C
    ///
    Rp2040FeatherRfm95 = 76,
    /// M5 esp32 based MCU modules with enclosure, TFT and LORA Shields. All Variants (Basic, Core, Fire, Core2, CoreS3, Paper) <https://m5stack.com/>
    M5stackCorebasic = 77,
    M5stackCore2 = 78,
    /// Pico2 with Waveshare Hat, same as Pico
    RpiPico2 = 79,
    /// M5 esp32 based MCU modules with enclosure, TFT and LORA Shields. All Variants (Basic, Core, Fire, Core2, CoreS3, Paper) <https://m5stack.com/>
    M5stackCores3 = 80,
    /// Seeed XIAO S3 DK
    SeeedXiaoS3 = 81,
    ///
    /// Nordic nRF52840+Semtech SX1262 LoRa BLE Combo Module. nRF52840+SX1262 MS24SF1
    Ms24sf1 = 82,
    ///
    /// Lilygo TLora-C6 with the new ESP32-C6 MCU
    TloraC6 = 83,
    ///
    /// WisMesh Tap
    /// RAK-4631 w/ TFT in injection modled case
    WismeshTap = 84,
    ///
    /// Similar to PORTDUINO but used by Routastic devices, this is not any
    /// particular device and does not run Meshtastic's code but supports
    /// the same frame format.
    /// Runs on linux, see <https://github.com/Jorropo/routastic>
    Routastic = 85,
    ///
    /// Mesh-Tab, esp32 based
    /// <https://github.com/valzzu/Mesh-Tab>
    MeshTab = 86,
    ///
    /// MeshLink board developed by LoraItalia. NRF52840, eByte E22900M22S (Will also come with other frequencies), 25w MPPT solar charger (5v,12v,18v selectable), support for gps, buzzer, oled or e-ink display, 10 gpios, hardware watchdog
    /// <https://www.loraitalia.it>
    Meshlink = 87,
    ///
    /// ------------------------------------------------------------------------------------------------------------------------------------------
    /// Reserved ID For developing private Ports. These will show up in live traffic sparsely, so we can use a high number. Keep it within 8 bits.
    /// ------------------------------------------------------------------------------------------------------------------------------------------
    PrivateHw = 255,
}
impl HardwareModel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            HardwareModel::Unset => "UNSET",
            HardwareModel::TloraV2 => "TLORA_V2",
            HardwareModel::TloraV1 => "TLORA_V1",
            HardwareModel::TloraV211p6 => "TLORA_V2_1_1P6",
            HardwareModel::Tbeam => "TBEAM",
            HardwareModel::HeltecV20 => "HELTEC_V2_0",
            HardwareModel::TbeamV0p7 => "TBEAM_V0P7",
            HardwareModel::TEcho => "T_ECHO",
            HardwareModel::TloraV11p3 => "TLORA_V1_1P3",
            HardwareModel::Rak4631 => "RAK4631",
            HardwareModel::HeltecV21 => "HELTEC_V2_1",
            HardwareModel::HeltecV1 => "HELTEC_V1",
            HardwareModel::LilygoTbeamS3Core => "LILYGO_TBEAM_S3_CORE",
            HardwareModel::Rak11200 => "RAK11200",
            HardwareModel::NanoG1 => "NANO_G1",
            HardwareModel::TloraV211p8 => "TLORA_V2_1_1P8",
            HardwareModel::TloraT3S3 => "TLORA_T3_S3",
            HardwareModel::NanoG1Explorer => "NANO_G1_EXPLORER",
            HardwareModel::NanoG2Ultra => "NANO_G2_ULTRA",
            HardwareModel::LoraType => "LORA_TYPE",
            HardwareModel::Wiphone => "WIPHONE",
            HardwareModel::WioWm1110 => "WIO_WM1110",
            HardwareModel::Rak2560 => "RAK2560",
            HardwareModel::HeltecHru3601 => "HELTEC_HRU_3601",
            HardwareModel::HeltecWirelessBridge => "HELTEC_WIRELESS_BRIDGE",
            HardwareModel::StationG1 => "STATION_G1",
            HardwareModel::Rak11310 => "RAK11310",
            HardwareModel::SenseloraRp2040 => "SENSELORA_RP2040",
            HardwareModel::SenseloraS3 => "SENSELORA_S3",
            HardwareModel::Canaryone => "CANARYONE",
            HardwareModel::Rp2040Lora => "RP2040_LORA",
            HardwareModel::StationG2 => "STATION_G2",
            HardwareModel::LoraRelayV1 => "LORA_RELAY_V1",
            HardwareModel::Nrf52840dk => "NRF52840DK",
            HardwareModel::Ppr => "PPR",
            HardwareModel::Genieblocks => "GENIEBLOCKS",
            HardwareModel::Nrf52Unknown => "NRF52_UNKNOWN",
            HardwareModel::Portduino => "PORTDUINO",
            HardwareModel::AndroidSim => "ANDROID_SIM",
            HardwareModel::DiyV1 => "DIY_V1",
            HardwareModel::Nrf52840Pca10059 => "NRF52840_PCA10059",
            HardwareModel::DrDev => "DR_DEV",
            HardwareModel::M5stack => "M5STACK",
            HardwareModel::HeltecV3 => "HELTEC_V3",
            HardwareModel::HeltecWslV3 => "HELTEC_WSL_V3",
            HardwareModel::Betafpv2400Tx => "BETAFPV_2400_TX",
            HardwareModel::Betafpv900NanoTx => "BETAFPV_900_NANO_TX",
            HardwareModel::RpiPico => "RPI_PICO",
            HardwareModel::HeltecWirelessTracker => "HELTEC_WIRELESS_TRACKER",
            HardwareModel::HeltecWirelessPaper => "HELTEC_WIRELESS_PAPER",
            HardwareModel::TDeck => "T_DECK",
            HardwareModel::TWatchS3 => "T_WATCH_S3",
            HardwareModel::PicomputerS3 => "PICOMPUTER_S3",
            HardwareModel::HeltecHt62 => "HELTEC_HT62",
            HardwareModel::EbyteEsp32S3 => "EBYTE_ESP32_S3",
            HardwareModel::Esp32S3Pico => "ESP32_S3_PICO",
            HardwareModel::Chatter2 => "CHATTER_2",
            HardwareModel::HeltecWirelessPaperV10 => "HELTEC_WIRELESS_PAPER_V1_0",
            HardwareModel::HeltecWirelessTrackerV10 => "HELTEC_WIRELESS_TRACKER_V1_0",
            HardwareModel::Unphone => "UNPHONE",
            HardwareModel::TdLorac => "TD_LORAC",
            HardwareModel::CdebyteEoraS3 => "CDEBYTE_EORA_S3",
            HardwareModel::TwcMeshV4 => "TWC_MESH_V4",
            HardwareModel::Nrf52PromicroDiy => "NRF52_PROMICRO_DIY",
            HardwareModel::Radiomaster900BanditNano => "RADIOMASTER_900_BANDIT_NANO",
            HardwareModel::HeltecCapsuleSensorV3 => "HELTEC_CAPSULE_SENSOR_V3",
            HardwareModel::HeltecVisionMasterT190 => "HELTEC_VISION_MASTER_T190",
            HardwareModel::HeltecVisionMasterE213 => "HELTEC_VISION_MASTER_E213",
            HardwareModel::HeltecVisionMasterE290 => "HELTEC_VISION_MASTER_E290",
            HardwareModel::HeltecMeshNodeT114 => "HELTEC_MESH_NODE_T114",
            HardwareModel::SensecapIndicator => "SENSECAP_INDICATOR",
            HardwareModel::TrackerT1000E => "TRACKER_T1000_E",
            HardwareModel::Rak3172 => "RAK3172",
            HardwareModel::WioE5 => "WIO_E5",
            HardwareModel::Radiomaster900Bandit => "RADIOMASTER_900_BANDIT",
            HardwareModel::Me25ls014y10td => "ME25LS01_4Y10TD",
            HardwareModel::Rp2040FeatherRfm95 => "RP2040_FEATHER_RFM95",
            HardwareModel::M5stackCorebasic => "M5STACK_COREBASIC",
            HardwareModel::M5stackCore2 => "M5STACK_CORE2",
            HardwareModel::RpiPico2 => "RPI_PICO2",
            HardwareModel::M5stackCores3 => "M5STACK_CORES3",
            HardwareModel::SeeedXiaoS3 => "SEEED_XIAO_S3",
            HardwareModel::Ms24sf1 => "MS24SF1",
            HardwareModel::TloraC6 => "TLORA_C6",
            HardwareModel::WismeshTap => "WISMESH_TAP",
            HardwareModel::Routastic => "ROUTASTIC",
            HardwareModel::MeshTab => "MESH_TAB",
            HardwareModel::Meshlink => "MESHLINK",
            HardwareModel::PrivateHw => "PRIVATE_HW",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSET" => Some(Self::Unset),
            "TLORA_V2" => Some(Self::TloraV2),
            "TLORA_V1" => Some(Self::TloraV1),
            "TLORA_V2_1_1P6" => Some(Self::TloraV211p6),
            "TBEAM" => Some(Self::Tbeam),
            "HELTEC_V2_0" => Some(Self::HeltecV20),
            "TBEAM_V0P7" => Some(Self::TbeamV0p7),
            "T_ECHO" => Some(Self::TEcho),
            "TLORA_V1_1P3" => Some(Self::TloraV11p3),
            "RAK4631" => Some(Self::Rak4631),
            "HELTEC_V2_1" => Some(Self::HeltecV21),
            "HELTEC_V1" => Some(Self::HeltecV1),
            "LILYGO_TBEAM_S3_CORE" => Some(Self::LilygoTbeamS3Core),
            "RAK11200" => Some(Self::Rak11200),
            "NANO_G1" => Some(Self::NanoG1),
            "TLORA_V2_1_1P8" => Some(Self::TloraV211p8),
            "TLORA_T3_S3" => Some(Self::TloraT3S3),
            "NANO_G1_EXPLORER" => Some(Self::NanoG1Explorer),
            "NANO_G2_ULTRA" => Some(Self::NanoG2Ultra),
            "LORA_TYPE" => Some(Self::LoraType),
            "WIPHONE" => Some(Self::Wiphone),
            "WIO_WM1110" => Some(Self::WioWm1110),
            "RAK2560" => Some(Self::Rak2560),
            "HELTEC_HRU_3601" => Some(Self::HeltecHru3601),
            "HELTEC_WIRELESS_BRIDGE" => Some(Self::HeltecWirelessBridge),
            "STATION_G1" => Some(Self::StationG1),
            "RAK11310" => Some(Self::Rak11310),
            "SENSELORA_RP2040" => Some(Self::SenseloraRp2040),
            "SENSELORA_S3" => Some(Self::SenseloraS3),
            "CANARYONE" => Some(Self::Canaryone),
            "RP2040_LORA" => Some(Self::Rp2040Lora),
            "STATION_G2" => Some(Self::StationG2),
            "LORA_RELAY_V1" => Some(Self::LoraRelayV1),
            "NRF52840DK" => Some(Self::Nrf52840dk),
            "PPR" => Some(Self::Ppr),
            "GENIEBLOCKS" => Some(Self::Genieblocks),
            "NRF52_UNKNOWN" => Some(Self::Nrf52Unknown),
            "PORTDUINO" => Some(Self::Portduino),
            "ANDROID_SIM" => Some(Self::AndroidSim),
            "DIY_V1" => Some(Self::DiyV1),
            "NRF52840_PCA10059" => Some(Self::Nrf52840Pca10059),
            "DR_DEV" => Some(Self::DrDev),
            "M5STACK" => Some(Self::M5stack),
            "HELTEC_V3" => Some(Self::HeltecV3),
            "HELTEC_WSL_V3" => Some(Self::HeltecWslV3),
            "BETAFPV_2400_TX" => Some(Self::Betafpv2400Tx),
            "BETAFPV_900_NANO_TX" => Some(Self::Betafpv900NanoTx),
            "RPI_PICO" => Some(Self::RpiPico),
            "HELTEC_WIRELESS_TRACKER" => Some(Self::HeltecWirelessTracker),
            "HELTEC_WIRELESS_PAPER" => Some(Self::HeltecWirelessPaper),
            "T_DECK" => Some(Self::TDeck),
            "T_WATCH_S3" => Some(Self::TWatchS3),
            "PICOMPUTER_S3" => Some(Self::PicomputerS3),
            "HELTEC_HT62" => Some(Self::HeltecHt62),
            "EBYTE_ESP32_S3" => Some(Self::EbyteEsp32S3),
            "ESP32_S3_PICO" => Some(Self::Esp32S3Pico),
            "CHATTER_2" => Some(Self::Chatter2),
            "HELTEC_WIRELESS_PAPER_V1_0" => Some(Self::HeltecWirelessPaperV10),
            "HELTEC_WIRELESS_TRACKER_V1_0" => Some(Self::HeltecWirelessTrackerV10),
            "UNPHONE" => Some(Self::Unphone),
            "TD_LORAC" => Some(Self::TdLorac),
            "CDEBYTE_EORA_S3" => Some(Self::CdebyteEoraS3),
            "TWC_MESH_V4" => Some(Self::TwcMeshV4),
            "NRF52_PROMICRO_DIY" => Some(Self::Nrf52PromicroDiy),
            "RADIOMASTER_900_BANDIT_NANO" => Some(Self::Radiomaster900BanditNano),
            "HELTEC_CAPSULE_SENSOR_V3" => Some(Self::HeltecCapsuleSensorV3),
            "HELTEC_VISION_MASTER_T190" => Some(Self::HeltecVisionMasterT190),
            "HELTEC_VISION_MASTER_E213" => Some(Self::HeltecVisionMasterE213),
            "HELTEC_VISION_MASTER_E290" => Some(Self::HeltecVisionMasterE290),
            "HELTEC_MESH_NODE_T114" => Some(Self::HeltecMeshNodeT114),
            "SENSECAP_INDICATOR" => Some(Self::SensecapIndicator),
            "TRACKER_T1000_E" => Some(Self::TrackerT1000E),
            "RAK3172" => Some(Self::Rak3172),
            "WIO_E5" => Some(Self::WioE5),
            "RADIOMASTER_900_BANDIT" => Some(Self::Radiomaster900Bandit),
            "ME25LS01_4Y10TD" => Some(Self::Me25ls014y10td),
            "RP2040_FEATHER_RFM95" => Some(Self::Rp2040FeatherRfm95),
            "M5STACK_COREBASIC" => Some(Self::M5stackCorebasic),
            "M5STACK_CORE2" => Some(Self::M5stackCore2),
            "RPI_PICO2" => Some(Self::RpiPico2),
            "M5STACK_CORES3" => Some(Self::M5stackCores3),
            "SEEED_XIAO_S3" => Some(Self::SeeedXiaoS3),
            "MS24SF1" => Some(Self::Ms24sf1),
            "TLORA_C6" => Some(Self::TloraC6),
            "WISMESH_TAP" => Some(Self::WismeshTap),
            "ROUTASTIC" => Some(Self::Routastic),
            "MESH_TAB" => Some(Self::MeshTab),
            "MESHLINK" => Some(Self::Meshlink),
            "PRIVATE_HW" => Some(Self::PrivateHw),
            _ => None,
        }
    }
}
///
/// Shared constants between device and phone
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Constants {
    ///
    /// First enum must be zero, and we are just using this enum to
    /// pass int constants between two very different environments
    Zero = 0,
    ///
    /// From mesh.options
    /// note: this payload length is ONLY the bytes that are sent inside of the Data protobuf (excluding protobuf overhead). The 16 byte header is
    /// outside of this envelope
    DataPayloadLen = 233,
}
impl Constants {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Constants::Zero => "ZERO",
            Constants::DataPayloadLen => "DATA_PAYLOAD_LEN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ZERO" => Some(Self::Zero),
            "DATA_PAYLOAD_LEN" => Some(Self::DataPayloadLen),
            _ => None,
        }
    }
}
///
/// Error codes for critical errors
/// The device might report these fault codes on the screen.
/// If you encounter a fault code, please post on the meshtastic.discourse.group
/// and we'll try to help.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CriticalErrorCode {
    ///
    /// TODO: REPLACE
    None = 0,
    ///
    /// A software bug was detected while trying to send lora
    TxWatchdog = 1,
    ///
    /// A software bug was detected on entry to sleep
    SleepEnterWait = 2,
    ///
    /// No Lora radio hardware could be found
    NoRadio = 3,
    ///
    /// Not normally used
    Unspecified = 4,
    ///
    /// We failed while configuring a UBlox GPS
    UbloxUnitFailed = 5,
    ///
    /// This board was expected to have a power management chip and it is missing or broken
    NoAxp192 = 6,
    ///
    /// The channel tried to set a radio setting which is not supported by this chipset,
    /// radio comms settings are now undefined.
    InvalidRadioSetting = 7,
    ///
    /// Radio transmit hardware failure. We sent data to the radio chip, but it didn't
    /// reply with an interrupt.
    TransmitFailed = 8,
    ///
    /// We detected that the main CPU voltage dropped below the minimum acceptable value
    Brownout = 9,
    /// Selftest of SX1262 radio chip failed
    Sx1262Failure = 10,
    ///
    /// A (likely software but possibly hardware) failure was detected while trying to send packets.
    /// If this occurs on your board, please post in the forum so that we can ask you to collect some information to allow fixing this bug
    RadioSpiBug = 11,
    ///
    /// Corruption was detected on the flash filesystem but we were able to repair things.
    /// If you see this failure in the field please post in the forum because we are interested in seeing if this is occurring in the field.
    FlashCorruptionRecoverable = 12,
    ///
    /// Corruption was detected on the flash filesystem but we were unable to repair things.
    /// NOTE: Your node will probably need to be reconfigured the next time it reboots (it will lose the region code etc...)
    /// If you see this failure in the field please post in the forum because we are interested in seeing if this is occurring in the field.
    FlashCorruptionUnrecoverable = 13,
}
impl CriticalErrorCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CriticalErrorCode::None => "NONE",
            CriticalErrorCode::TxWatchdog => "TX_WATCHDOG",
            CriticalErrorCode::SleepEnterWait => "SLEEP_ENTER_WAIT",
            CriticalErrorCode::NoRadio => "NO_RADIO",
            CriticalErrorCode::Unspecified => "UNSPECIFIED",
            CriticalErrorCode::UbloxUnitFailed => "UBLOX_UNIT_FAILED",
            CriticalErrorCode::NoAxp192 => "NO_AXP192",
            CriticalErrorCode::InvalidRadioSetting => "INVALID_RADIO_SETTING",
            CriticalErrorCode::TransmitFailed => "TRANSMIT_FAILED",
            CriticalErrorCode::Brownout => "BROWNOUT",
            CriticalErrorCode::Sx1262Failure => "SX1262_FAILURE",
            CriticalErrorCode::RadioSpiBug => "RADIO_SPI_BUG",
            CriticalErrorCode::FlashCorruptionRecoverable => {
                "FLASH_CORRUPTION_RECOVERABLE"
            }
            CriticalErrorCode::FlashCorruptionUnrecoverable => {
                "FLASH_CORRUPTION_UNRECOVERABLE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "TX_WATCHDOG" => Some(Self::TxWatchdog),
            "SLEEP_ENTER_WAIT" => Some(Self::SleepEnterWait),
            "NO_RADIO" => Some(Self::NoRadio),
            "UNSPECIFIED" => Some(Self::Unspecified),
            "UBLOX_UNIT_FAILED" => Some(Self::UbloxUnitFailed),
            "NO_AXP192" => Some(Self::NoAxp192),
            "INVALID_RADIO_SETTING" => Some(Self::InvalidRadioSetting),
            "TRANSMIT_FAILED" => Some(Self::TransmitFailed),
            "BROWNOUT" => Some(Self::Brownout),
            "SX1262_FAILURE" => Some(Self::Sx1262Failure),
            "RADIO_SPI_BUG" => Some(Self::RadioSpiBug),
            "FLASH_CORRUPTION_RECOVERABLE" => Some(Self::FlashCorruptionRecoverable),
            "FLASH_CORRUPTION_UNRECOVERABLE" => Some(Self::FlashCorruptionUnrecoverable),
            _ => None,
        }
    }
}
///
/// Enum for modules excluded from a device's configuration.
/// Each value represents a ModuleConfigType that can be toggled as excluded
/// by setting its corresponding bit in the `excluded_modules` bitmask field.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExcludedModules {
    ///
    /// Default value of 0 indicates no modules are excluded.
    ExcludedNone = 0,
    ///
    /// MQTT module
    MqttConfig = 1,
    ///
    /// Serial module
    SerialConfig = 2,
    ///
    /// External Notification module
    ExtnotifConfig = 4,
    ///
    /// Store and Forward module
    StoreforwardConfig = 8,
    ///
    /// Range Test module
    RangetestConfig = 16,
    ///
    /// Telemetry module
    TelemetryConfig = 32,
    ///
    /// Canned Message module
    CannedmsgConfig = 64,
    ///
    /// Audio module
    AudioConfig = 128,
    ///
    /// Remote Hardware module
    RemotehardwareConfig = 256,
    ///
    /// Neighbor Info module
    NeighborinfoConfig = 512,
    ///
    /// Ambient Lighting module
    AmbientlightingConfig = 1024,
    ///
    /// Detection Sensor module
    DetectionsensorConfig = 2048,
    ///
    /// Paxcounter module
    PaxcounterConfig = 4096,
}
impl ExcludedModules {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExcludedModules::ExcludedNone => "EXCLUDED_NONE",
            ExcludedModules::MqttConfig => "MQTT_CONFIG",
            ExcludedModules::SerialConfig => "SERIAL_CONFIG",
            ExcludedModules::ExtnotifConfig => "EXTNOTIF_CONFIG",
            ExcludedModules::StoreforwardConfig => "STOREFORWARD_CONFIG",
            ExcludedModules::RangetestConfig => "RANGETEST_CONFIG",
            ExcludedModules::TelemetryConfig => "TELEMETRY_CONFIG",
            ExcludedModules::CannedmsgConfig => "CANNEDMSG_CONFIG",
            ExcludedModules::AudioConfig => "AUDIO_CONFIG",
            ExcludedModules::RemotehardwareConfig => "REMOTEHARDWARE_CONFIG",
            ExcludedModules::NeighborinfoConfig => "NEIGHBORINFO_CONFIG",
            ExcludedModules::AmbientlightingConfig => "AMBIENTLIGHTING_CONFIG",
            ExcludedModules::DetectionsensorConfig => "DETECTIONSENSOR_CONFIG",
            ExcludedModules::PaxcounterConfig => "PAXCOUNTER_CONFIG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXCLUDED_NONE" => Some(Self::ExcludedNone),
            "MQTT_CONFIG" => Some(Self::MqttConfig),
            "SERIAL_CONFIG" => Some(Self::SerialConfig),
            "EXTNOTIF_CONFIG" => Some(Self::ExtnotifConfig),
            "STOREFORWARD_CONFIG" => Some(Self::StoreforwardConfig),
            "RANGETEST_CONFIG" => Some(Self::RangetestConfig),
            "TELEMETRY_CONFIG" => Some(Self::TelemetryConfig),
            "CANNEDMSG_CONFIG" => Some(Self::CannedmsgConfig),
            "AUDIO_CONFIG" => Some(Self::AudioConfig),
            "REMOTEHARDWARE_CONFIG" => Some(Self::RemotehardwareConfig),
            "NEIGHBORINFO_CONFIG" => Some(Self::NeighborinfoConfig),
            "AMBIENTLIGHTING_CONFIG" => Some(Self::AmbientlightingConfig),
            "DETECTIONSENSOR_CONFIG" => Some(Self::DetectionsensorConfig),
            "PAXCOUNTER_CONFIG" => Some(Self::PaxcounterConfig),
            _ => None,
        }
    }
}
///
/// This message is handled by the Admin module and is responsible for all settings/channel read/write operations.
/// This message is used to do settings operations to both remote AND local nodes.
/// (Prior to 1.2 these operations were done via special ToRadio operations)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminMessage {
    ///
    /// The node generates this key and sends it with any get_x_response packets.
    /// The client MUST include the same key with any set_x commands. Key expires after 300 seconds.
    /// Prevents replay attacks for admin messages.
    #[prost(bytes = "vec", tag = "101")]
    pub session_passkey: ::prost::alloc::vec::Vec<u8>,
    ///
    /// TODO: REPLACE
    #[prost(
        oneof = "admin_message::PayloadVariant",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 64, 65, 94, 95, 96, 97, 98, 99, 100"
    )]
    pub payload_variant: ::core::option::Option<admin_message::PayloadVariant>,
}
/// Nested message and enum types in `AdminMessage`.
pub mod admin_message {
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ConfigType {
        ///
        /// TODO: REPLACE
        DeviceConfig = 0,
        ///
        /// TODO: REPLACE
        PositionConfig = 1,
        ///
        /// TODO: REPLACE
        PowerConfig = 2,
        ///
        /// TODO: REPLACE
        NetworkConfig = 3,
        ///
        /// TODO: REPLACE
        DisplayConfig = 4,
        ///
        /// TODO: REPLACE
        LoraConfig = 5,
        ///
        /// TODO: REPLACE
        BluetoothConfig = 6,
        ///
        /// TODO: REPLACE
        SecurityConfig = 7,
        ///
        ///
        SessionkeyConfig = 8,
        ///
        /// device-ui config
        DeviceuiConfig = 9,
    }
    impl ConfigType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConfigType::DeviceConfig => "DEVICE_CONFIG",
                ConfigType::PositionConfig => "POSITION_CONFIG",
                ConfigType::PowerConfig => "POWER_CONFIG",
                ConfigType::NetworkConfig => "NETWORK_CONFIG",
                ConfigType::DisplayConfig => "DISPLAY_CONFIG",
                ConfigType::LoraConfig => "LORA_CONFIG",
                ConfigType::BluetoothConfig => "BLUETOOTH_CONFIG",
                ConfigType::SecurityConfig => "SECURITY_CONFIG",
                ConfigType::SessionkeyConfig => "SESSIONKEY_CONFIG",
                ConfigType::DeviceuiConfig => "DEVICEUI_CONFIG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEVICE_CONFIG" => Some(Self::DeviceConfig),
                "POSITION_CONFIG" => Some(Self::PositionConfig),
                "POWER_CONFIG" => Some(Self::PowerConfig),
                "NETWORK_CONFIG" => Some(Self::NetworkConfig),
                "DISPLAY_CONFIG" => Some(Self::DisplayConfig),
                "LORA_CONFIG" => Some(Self::LoraConfig),
                "BLUETOOTH_CONFIG" => Some(Self::BluetoothConfig),
                "SECURITY_CONFIG" => Some(Self::SecurityConfig),
                "SESSIONKEY_CONFIG" => Some(Self::SessionkeyConfig),
                "DEVICEUI_CONFIG" => Some(Self::DeviceuiConfig),
                _ => None,
            }
        }
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ModuleConfigType {
        ///
        /// TODO: REPLACE
        MqttConfig = 0,
        ///
        /// TODO: REPLACE
        SerialConfig = 1,
        ///
        /// TODO: REPLACE
        ExtnotifConfig = 2,
        ///
        /// TODO: REPLACE
        StoreforwardConfig = 3,
        ///
        /// TODO: REPLACE
        RangetestConfig = 4,
        ///
        /// TODO: REPLACE
        TelemetryConfig = 5,
        ///
        /// TODO: REPLACE
        CannedmsgConfig = 6,
        ///
        /// TODO: REPLACE
        AudioConfig = 7,
        ///
        /// TODO: REPLACE
        RemotehardwareConfig = 8,
        ///
        /// TODO: REPLACE
        NeighborinfoConfig = 9,
        ///
        /// TODO: REPLACE
        AmbientlightingConfig = 10,
        ///
        /// TODO: REPLACE
        DetectionsensorConfig = 11,
        ///
        /// TODO: REPLACE
        PaxcounterConfig = 12,
    }
    impl ModuleConfigType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ModuleConfigType::MqttConfig => "MQTT_CONFIG",
                ModuleConfigType::SerialConfig => "SERIAL_CONFIG",
                ModuleConfigType::ExtnotifConfig => "EXTNOTIF_CONFIG",
                ModuleConfigType::StoreforwardConfig => "STOREFORWARD_CONFIG",
                ModuleConfigType::RangetestConfig => "RANGETEST_CONFIG",
                ModuleConfigType::TelemetryConfig => "TELEMETRY_CONFIG",
                ModuleConfigType::CannedmsgConfig => "CANNEDMSG_CONFIG",
                ModuleConfigType::AudioConfig => "AUDIO_CONFIG",
                ModuleConfigType::RemotehardwareConfig => "REMOTEHARDWARE_CONFIG",
                ModuleConfigType::NeighborinfoConfig => "NEIGHBORINFO_CONFIG",
                ModuleConfigType::AmbientlightingConfig => "AMBIENTLIGHTING_CONFIG",
                ModuleConfigType::DetectionsensorConfig => "DETECTIONSENSOR_CONFIG",
                ModuleConfigType::PaxcounterConfig => "PAXCOUNTER_CONFIG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MQTT_CONFIG" => Some(Self::MqttConfig),
                "SERIAL_CONFIG" => Some(Self::SerialConfig),
                "EXTNOTIF_CONFIG" => Some(Self::ExtnotifConfig),
                "STOREFORWARD_CONFIG" => Some(Self::StoreforwardConfig),
                "RANGETEST_CONFIG" => Some(Self::RangetestConfig),
                "TELEMETRY_CONFIG" => Some(Self::TelemetryConfig),
                "CANNEDMSG_CONFIG" => Some(Self::CannedmsgConfig),
                "AUDIO_CONFIG" => Some(Self::AudioConfig),
                "REMOTEHARDWARE_CONFIG" => Some(Self::RemotehardwareConfig),
                "NEIGHBORINFO_CONFIG" => Some(Self::NeighborinfoConfig),
                "AMBIENTLIGHTING_CONFIG" => Some(Self::AmbientlightingConfig),
                "DETECTIONSENSOR_CONFIG" => Some(Self::DetectionsensorConfig),
                "PAXCOUNTER_CONFIG" => Some(Self::PaxcounterConfig),
                _ => None,
            }
        }
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// Send the specified channel in the response to this message
        /// NOTE: This field is sent with the channel index + 1 (to ensure we never try to send 'zero' - which protobufs treats as not present)
        #[prost(uint32, tag = "1")]
        GetChannelRequest(u32),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "2")]
        GetChannelResponse(super::Channel),
        ///
        /// Send the current owner data in the response to this message.
        #[prost(bool, tag = "3")]
        GetOwnerRequest(bool),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "4")]
        GetOwnerResponse(super::User),
        ///
        /// Ask for the following config data to be sent
        #[prost(enumeration = "ConfigType", tag = "5")]
        GetConfigRequest(i32),
        ///
        /// Send the current Config in the response to this message.
        #[prost(message, tag = "6")]
        GetConfigResponse(super::Config),
        ///
        /// Ask for the following config data to be sent
        #[prost(enumeration = "ModuleConfigType", tag = "7")]
        GetModuleConfigRequest(i32),
        ///
        /// Send the current Config in the response to this message.
        #[prost(message, tag = "8")]
        GetModuleConfigResponse(super::ModuleConfig),
        ///
        /// Get the Canned Message Module messages in the response to this message.
        #[prost(bool, tag = "10")]
        GetCannedMessageModuleMessagesRequest(bool),
        ///
        /// Get the Canned Message Module messages in the response to this message.
        #[prost(string, tag = "11")]
        GetCannedMessageModuleMessagesResponse(::prost::alloc::string::String),
        ///
        /// Request the node to send device metadata (firmware, protobuf version, etc)
        #[prost(bool, tag = "12")]
        GetDeviceMetadataRequest(bool),
        ///
        /// Device metadata response
        #[prost(message, tag = "13")]
        GetDeviceMetadataResponse(super::DeviceMetadata),
        ///
        /// Get the Ringtone in the response to this message.
        #[prost(bool, tag = "14")]
        GetRingtoneRequest(bool),
        ///
        /// Get the Ringtone in the response to this message.
        #[prost(string, tag = "15")]
        GetRingtoneResponse(::prost::alloc::string::String),
        ///
        /// Request the node to send it's connection status
        #[prost(bool, tag = "16")]
        GetDeviceConnectionStatusRequest(bool),
        ///
        /// Device connection status response
        #[prost(message, tag = "17")]
        GetDeviceConnectionStatusResponse(super::DeviceConnectionStatus),
        ///
        /// Setup a node for licensed amateur (ham) radio operation
        #[prost(message, tag = "18")]
        SetHamMode(super::HamParameters),
        ///
        /// Get the mesh's nodes with their available gpio pins for RemoteHardware module use
        #[prost(bool, tag = "19")]
        GetNodeRemoteHardwarePinsRequest(bool),
        ///
        /// Respond with the mesh's nodes with their available gpio pins for RemoteHardware module use
        #[prost(message, tag = "20")]
        GetNodeRemoteHardwarePinsResponse(super::NodeRemoteHardwarePinsResponse),
        ///
        /// Enter (UF2) DFU mode
        /// Only implemented on NRF52 currently
        #[prost(bool, tag = "21")]
        EnterDfuModeRequest(bool),
        ///
        /// Delete the file by the specified path from the device
        #[prost(string, tag = "22")]
        DeleteFileRequest(::prost::alloc::string::String),
        ///
        /// Set zero and offset for scale chips
        #[prost(uint32, tag = "23")]
        SetScale(u32),
        ///
        /// Set the owner for this node
        #[prost(message, tag = "32")]
        SetOwner(super::User),
        ///
        /// Set channels (using the new API).
        /// A special channel is the "primary channel".
        /// The other records are secondary channels.
        /// Note: only one channel can be marked as primary.
        /// If the client sets a particular channel to be primary, the previous channel will be set to SECONDARY automatically.
        #[prost(message, tag = "33")]
        SetChannel(super::Channel),
        ///
        /// Set the current Config
        #[prost(message, tag = "34")]
        SetConfig(super::Config),
        ///
        /// Set the current Config
        #[prost(message, tag = "35")]
        SetModuleConfig(super::ModuleConfig),
        ///
        /// Set the Canned Message Module messages text.
        #[prost(string, tag = "36")]
        SetCannedMessageModuleMessages(::prost::alloc::string::String),
        ///
        /// Set the ringtone for ExternalNotification.
        #[prost(string, tag = "37")]
        SetRingtoneMessage(::prost::alloc::string::String),
        ///
        /// Remove the node by the specified node-num from the NodeDB on the device
        #[prost(uint32, tag = "38")]
        RemoveByNodenum(u32),
        ///
        /// Set specified node-num to be favorited on the NodeDB on the device
        #[prost(uint32, tag = "39")]
        SetFavoriteNode(u32),
        ///
        /// Set specified node-num to be un-favorited on the NodeDB on the device
        #[prost(uint32, tag = "40")]
        RemoveFavoriteNode(u32),
        ///
        /// Set fixed position data on the node and then set the position.fixed_position = true
        #[prost(message, tag = "41")]
        SetFixedPosition(super::Position),
        ///
        /// Clear fixed position coordinates and then set position.fixed_position = false
        #[prost(bool, tag = "42")]
        RemoveFixedPosition(bool),
        ///
        /// Set time only on the node
        /// Convenience method to set the time on the node (as Net quality) without any other position data
        #[prost(fixed32, tag = "43")]
        SetTimeOnly(u32),
        ///
        /// Tell the node to send the stored ui data.
        #[prost(bool, tag = "44")]
        GetUiConfigRequest(bool),
        ///
        /// Reply stored device ui data.
        #[prost(message, tag = "45")]
        GetUiConfigResponse(super::DeviceUiConfig),
        ///
        /// Tell the node to store UI data persistently.
        #[prost(message, tag = "46")]
        StoreUiConfig(super::DeviceUiConfig),
        ///
        /// Set specified node-num to be ignored on the NodeDB on the device
        #[prost(uint32, tag = "47")]
        SetIgnoredNode(u32),
        ///
        /// Set specified node-num to be un-ignored on the NodeDB on the device
        #[prost(uint32, tag = "48")]
        RemoveIgnoredNode(u32),
        ///
        /// Begins an edit transaction for config, module config, owner, and channel settings changes
        /// This will delay the standard *implicit* save to the file system and subsequent reboot behavior until committed (commit_edit_settings)
        #[prost(bool, tag = "64")]
        BeginEditSettings(bool),
        ///
        /// Commits an open transaction for any edits made to config, module config, owner, and channel settings
        #[prost(bool, tag = "65")]
        CommitEditSettings(bool),
        ///
        /// Tell the node to factory reset config everything; all device state and configuration will be returned to factory defaults and BLE bonds will be cleared.
        #[prost(int32, tag = "94")]
        FactoryResetDevice(i32),
        ///
        /// Tell the node to reboot into the OTA Firmware in this many seconds (or <0 to cancel reboot)
        /// Only Implemented for ESP32 Devices. This needs to be issued to send a new main firmware via bluetooth.
        #[prost(int32, tag = "95")]
        RebootOtaSeconds(i32),
        ///
        /// This message is only supported for the simulator Portduino build.
        /// If received the simulator will exit successfully.
        #[prost(bool, tag = "96")]
        ExitSimulator(bool),
        ///
        /// Tell the node to reboot in this many seconds (or <0 to cancel reboot)
        #[prost(int32, tag = "97")]
        RebootSeconds(i32),
        ///
        /// Tell the node to shutdown in this many seconds (or <0 to cancel shutdown)
        #[prost(int32, tag = "98")]
        ShutdownSeconds(i32),
        ///
        /// Tell the node to factory reset config; all device state and configuration will be returned to factory defaults; BLE bonds will be preserved.
        #[prost(int32, tag = "99")]
        FactoryResetConfig(i32),
        ///
        /// Tell the node to reset the nodedb.
        #[prost(int32, tag = "100")]
        NodedbReset(i32),
    }
}
///
/// Parameters for setting up Meshtastic for ameteur radio usage
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HamParameters {
    ///
    /// Amateur radio call sign, eg. KD2ABC
    #[prost(string, tag = "1")]
    pub call_sign: ::prost::alloc::string::String,
    ///
    /// Transmit power in dBm at the LoRA transceiver, not including any amplification
    #[prost(int32, tag = "2")]
    pub tx_power: i32,
    ///
    /// The selected frequency of LoRA operation
    /// Please respect your local laws, regulations, and band plans.
    /// Ensure your radio is capable of operating of the selected frequency before setting this.
    #[prost(float, tag = "3")]
    pub frequency: f32,
    ///
    /// Optional short name of user
    #[prost(string, tag = "4")]
    pub short_name: ::prost::alloc::string::String,
}
///
/// Response envelope for node_remote_hardware_pins
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeRemoteHardwarePinsResponse {
    ///
    /// Nodes and their respective remote hardware GPIO pins
    #[prost(message, repeated, tag = "1")]
    pub node_remote_hardware_pins: ::prost::alloc::vec::Vec<NodeRemoteHardwarePin>,
}
///
/// This is the most compact possible representation for a set of channels.
/// It includes only one PRIMARY channel (which must be first) and
/// any SECONDARY channels.
/// No DISABLED channels are included.
/// This abstraction is used only on the the 'app side' of the world (ie python, javascript and android etc) to show a group of Channels as a (long) URL
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelSet {
    ///
    /// Channel list with settings
    #[prost(message, repeated, tag = "1")]
    pub settings: ::prost::alloc::vec::Vec<ChannelSettings>,
    ///
    /// LoRa config
    #[prost(message, optional, tag = "2")]
    pub lora_config: ::core::option::Option<config::LoRaConfig>,
}
///
/// Packets for the official ATAK Plugin
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TakPacket {
    ///
    /// Are the payloads strings compressed for LoRA transport?
    #[prost(bool, tag = "1")]
    pub is_compressed: bool,
    ///
    /// The contact / callsign for ATAK user
    #[prost(message, optional, tag = "2")]
    pub contact: ::core::option::Option<Contact>,
    ///
    /// The group for ATAK user
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<Group>,
    ///
    /// The status of the ATAK EUD
    #[prost(message, optional, tag = "4")]
    pub status: ::core::option::Option<Status>,
    ///
    /// The payload of the packet
    #[prost(oneof = "tak_packet::PayloadVariant", tags = "5, 6, 7")]
    pub payload_variant: ::core::option::Option<tak_packet::PayloadVariant>,
}
/// Nested message and enum types in `TAKPacket`.
pub mod tak_packet {
    ///
    /// The payload of the packet
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PayloadVariant {
        ///
        /// TAK position report
        #[prost(message, tag = "5")]
        Pli(super::Pli),
        ///
        /// ATAK GeoChat message
        #[prost(message, tag = "6")]
        Chat(super::GeoChat),
        ///
        /// Generic CoT detail XML
        /// May be compressed / truncated by the sender (EUD)
        #[prost(bytes, tag = "7")]
        Detail(::prost::alloc::vec::Vec<u8>),
    }
}
///
/// ATAK GeoChat message
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoChat {
    ///
    /// The text message
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    ///
    /// Uid recipient of the message
    #[prost(string, optional, tag = "2")]
    pub to: ::core::option::Option<::prost::alloc::string::String>,
    ///
    /// Callsign of the recipient for the message
    #[prost(string, optional, tag = "3")]
    pub to_callsign: ::core::option::Option<::prost::alloc::string::String>,
}
///
/// ATAK Group
/// <__group role='Team Member' name='Cyan'/>
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    ///
    /// Role of the group member
    #[prost(enumeration = "MemberRole", tag = "1")]
    pub role: i32,
    ///
    /// Team (color)
    /// Default Cyan
    #[prost(enumeration = "Team", tag = "2")]
    pub team: i32,
}
///
/// ATAK EUD Status
/// <status battery='100' />
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    ///
    /// Battery level
    #[prost(uint32, tag = "1")]
    pub battery: u32,
}
///
/// ATAK Contact
/// <contact endpoint='0.0.0.0:4242:tcp' phone='+12345678' callsign='FALKE'/>
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contact {
    ///
    /// Callsign
    #[prost(string, tag = "1")]
    pub callsign: ::prost::alloc::string::String,
    ///
    /// Device callsign
    ///
    ///
    /// IP address of endpoint in integer form (0.0.0.0 default)
    #[prost(string, tag = "2")]
    pub device_callsign: ::prost::alloc::string::String,
}
///
/// Position Location Information from ATAK
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pli {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[prost(sfixed32, tag = "1")]
    pub latitude_i: i32,
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[prost(sfixed32, tag = "2")]
    pub longitude_i: i32,
    ///
    /// Altitude (ATAK prefers HAE)
    #[prost(int32, tag = "3")]
    pub altitude: i32,
    ///
    /// Speed
    #[prost(uint32, tag = "4")]
    pub speed: u32,
    ///
    /// Course in degrees
    #[prost(uint32, tag = "5")]
    pub course: u32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Team {
    ///
    /// Unspecifed
    UnspecifedColor = 0,
    ///
    /// White
    White = 1,
    ///
    /// Yellow
    Yellow = 2,
    ///
    /// Orange
    Orange = 3,
    ///
    /// Magenta
    Magenta = 4,
    ///
    /// Red
    Red = 5,
    ///
    /// Maroon
    Maroon = 6,
    ///
    /// Purple
    Purple = 7,
    ///
    /// Dark Blue
    DarkBlue = 8,
    ///
    /// Blue
    Blue = 9,
    ///
    /// Cyan
    Cyan = 10,
    ///
    /// Teal
    Teal = 11,
    ///
    /// Green
    Green = 12,
    ///
    /// Dark Green
    DarkGreen = 13,
    ///
    /// Brown
    Brown = 14,
}
impl Team {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Team::UnspecifedColor => "Unspecifed_Color",
            Team::White => "White",
            Team::Yellow => "Yellow",
            Team::Orange => "Orange",
            Team::Magenta => "Magenta",
            Team::Red => "Red",
            Team::Maroon => "Maroon",
            Team::Purple => "Purple",
            Team::DarkBlue => "Dark_Blue",
            Team::Blue => "Blue",
            Team::Cyan => "Cyan",
            Team::Teal => "Teal",
            Team::Green => "Green",
            Team::DarkGreen => "Dark_Green",
            Team::Brown => "Brown",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecifed_Color" => Some(Self::UnspecifedColor),
            "White" => Some(Self::White),
            "Yellow" => Some(Self::Yellow),
            "Orange" => Some(Self::Orange),
            "Magenta" => Some(Self::Magenta),
            "Red" => Some(Self::Red),
            "Maroon" => Some(Self::Maroon),
            "Purple" => Some(Self::Purple),
            "Dark_Blue" => Some(Self::DarkBlue),
            "Blue" => Some(Self::Blue),
            "Cyan" => Some(Self::Cyan),
            "Teal" => Some(Self::Teal),
            "Green" => Some(Self::Green),
            "Dark_Green" => Some(Self::DarkGreen),
            "Brown" => Some(Self::Brown),
            _ => None,
        }
    }
}
///
/// Role of the group member
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemberRole {
    ///
    /// Unspecifed
    Unspecifed = 0,
    ///
    /// Team Member
    TeamMember = 1,
    ///
    /// Team Lead
    TeamLead = 2,
    ///
    /// Headquarters
    Hq = 3,
    ///
    /// Airsoft enthusiast
    Sniper = 4,
    ///
    /// Medic
    Medic = 5,
    ///
    /// ForwardObserver
    ForwardObserver = 6,
    ///
    /// Radio Telephone Operator
    Rto = 7,
    ///
    /// Doggo
    K9 = 8,
}
impl MemberRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemberRole::Unspecifed => "Unspecifed",
            MemberRole::TeamMember => "TeamMember",
            MemberRole::TeamLead => "TeamLead",
            MemberRole::Hq => "HQ",
            MemberRole::Sniper => "Sniper",
            MemberRole::Medic => "Medic",
            MemberRole::ForwardObserver => "ForwardObserver",
            MemberRole::Rto => "RTO",
            MemberRole::K9 => "K9",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecifed" => Some(Self::Unspecifed),
            "TeamMember" => Some(Self::TeamMember),
            "TeamLead" => Some(Self::TeamLead),
            "HQ" => Some(Self::Hq),
            "Sniper" => Some(Self::Sniper),
            "Medic" => Some(Self::Medic),
            "ForwardObserver" => Some(Self::ForwardObserver),
            "RTO" => Some(Self::Rto),
            "K9" => Some(Self::K9),
            _ => None,
        }
    }
}
///
/// Canned message module configuration.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CannedMessageModuleConfig {
    ///
    /// Predefined messages for canned message module separated by '|' characters.
    #[prost(string, tag = "1")]
    pub messages: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalConfig {
    ///
    /// The part of the config that is specific to the Device
    #[prost(message, optional, tag = "1")]
    pub device: ::core::option::Option<config::DeviceConfig>,
    ///
    /// The part of the config that is specific to the GPS Position
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<config::PositionConfig>,
    ///
    /// The part of the config that is specific to the Power settings
    #[prost(message, optional, tag = "3")]
    pub power: ::core::option::Option<config::PowerConfig>,
    ///
    /// The part of the config that is specific to the Wifi Settings
    #[prost(message, optional, tag = "4")]
    pub network: ::core::option::Option<config::NetworkConfig>,
    ///
    /// The part of the config that is specific to the Display
    #[prost(message, optional, tag = "5")]
    pub display: ::core::option::Option<config::DisplayConfig>,
    ///
    /// The part of the config that is specific to the Lora Radio
    #[prost(message, optional, tag = "6")]
    pub lora: ::core::option::Option<config::LoRaConfig>,
    ///
    /// The part of the config that is specific to the Bluetooth settings
    #[prost(message, optional, tag = "7")]
    pub bluetooth: ::core::option::Option<config::BluetoothConfig>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[prost(uint32, tag = "8")]
    pub version: u32,
    ///
    /// The part of the config that is specific to Security settings
    #[prost(message, optional, tag = "9")]
    pub security: ::core::option::Option<config::SecurityConfig>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalModuleConfig {
    ///
    /// The part of the config that is specific to the MQTT module
    #[prost(message, optional, tag = "1")]
    pub mqtt: ::core::option::Option<module_config::MqttConfig>,
    ///
    /// The part of the config that is specific to the Serial module
    #[prost(message, optional, tag = "2")]
    pub serial: ::core::option::Option<module_config::SerialConfig>,
    ///
    /// The part of the config that is specific to the ExternalNotification module
    #[prost(message, optional, tag = "3")]
    pub external_notification: ::core::option::Option<
        module_config::ExternalNotificationConfig,
    >,
    ///
    /// The part of the config that is specific to the Store & Forward module
    #[prost(message, optional, tag = "4")]
    pub store_forward: ::core::option::Option<module_config::StoreForwardConfig>,
    ///
    /// The part of the config that is specific to the RangeTest module
    #[prost(message, optional, tag = "5")]
    pub range_test: ::core::option::Option<module_config::RangeTestConfig>,
    ///
    /// The part of the config that is specific to the Telemetry module
    #[prost(message, optional, tag = "6")]
    pub telemetry: ::core::option::Option<module_config::TelemetryConfig>,
    ///
    /// The part of the config that is specific to the Canned Message module
    #[prost(message, optional, tag = "7")]
    pub canned_message: ::core::option::Option<module_config::CannedMessageConfig>,
    ///
    /// The part of the config that is specific to the Audio module
    #[prost(message, optional, tag = "9")]
    pub audio: ::core::option::Option<module_config::AudioConfig>,
    ///
    /// The part of the config that is specific to the Remote Hardware module
    #[prost(message, optional, tag = "10")]
    pub remote_hardware: ::core::option::Option<module_config::RemoteHardwareConfig>,
    ///
    /// The part of the config that is specific to the Neighbor Info module
    #[prost(message, optional, tag = "11")]
    pub neighbor_info: ::core::option::Option<module_config::NeighborInfoConfig>,
    ///
    /// The part of the config that is specific to the Ambient Lighting module
    #[prost(message, optional, tag = "12")]
    pub ambient_lighting: ::core::option::Option<module_config::AmbientLightingConfig>,
    ///
    /// The part of the config that is specific to the Detection Sensor module
    #[prost(message, optional, tag = "13")]
    pub detection_sensor: ::core::option::Option<module_config::DetectionSensorConfig>,
    ///
    /// Paxcounter Config
    #[prost(message, optional, tag = "14")]
    pub paxcounter: ::core::option::Option<module_config::PaxcounterConfig>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[prost(uint32, tag = "8")]
    pub version: u32,
}
///
/// This abstraction is used to contain any configuration for provisioning a node on any client.
/// It is useful for importing and exporting configurations.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceProfile {
    ///
    /// Long name for the node
    #[prost(string, optional, tag = "1")]
    pub long_name: ::core::option::Option<::prost::alloc::string::String>,
    ///
    /// Short name of the node
    #[prost(string, optional, tag = "2")]
    pub short_name: ::core::option::Option<::prost::alloc::string::String>,
    ///
    /// The url of the channels from our node
    #[prost(string, optional, tag = "3")]
    pub channel_url: ::core::option::Option<::prost::alloc::string::String>,
    ///
    /// The Config of the node
    #[prost(message, optional, tag = "4")]
    pub config: ::core::option::Option<LocalConfig>,
    ///
    /// The ModuleConfig of the node
    #[prost(message, optional, tag = "5")]
    pub module_config: ::core::option::Option<LocalModuleConfig>,
    ///
    /// Fixed position data
    #[prost(message, optional, tag = "6")]
    pub fixed_position: ::core::option::Option<Position>,
    ///
    /// Ringtone for ExternalNotification
    #[prost(string, optional, tag = "7")]
    pub ringtone: ::core::option::Option<::prost::alloc::string::String>,
    ///
    /// Predefined messages for CannedMessage
    #[prost(string, optional, tag = "8")]
    pub canned_messages: ::core::option::Option<::prost::alloc::string::String>,
}
///
/// Position with static location information only for NodeDBLite
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionLite {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[prost(sfixed32, tag = "1")]
    pub latitude_i: i32,
    ///
    /// TODO: REPLACE
    #[prost(sfixed32, tag = "2")]
    pub longitude_i: i32,
    ///
    /// In meters above MSL (but see issue #359)
    #[prost(int32, tag = "3")]
    pub altitude: i32,
    ///
    /// This is usually not sent over the mesh (to save space), but it is sent
    /// from the phone so that the local device can set its RTC If it is sent over
    /// the mesh (because there are devices on the mesh without GPS), it will only
    /// be sent by devices which has a hardware GPS clock.
    /// seconds since 1970
    #[prost(fixed32, tag = "4")]
    pub time: u32,
    ///
    /// TODO: REPLACE
    #[prost(enumeration = "position::LocSource", tag = "5")]
    pub location_source: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserLite {
    ///
    /// This is the addr of the radio.
    #[deprecated]
    #[prost(bytes = "vec", tag = "1")]
    pub macaddr: ::prost::alloc::vec::Vec<u8>,
    ///
    /// A full name for this user, i.e. "Kevin Hester"
    #[prost(string, tag = "2")]
    pub long_name: ::prost::alloc::string::String,
    ///
    /// A VERY short name, ideally two characters.
    /// Suitable for a tiny OLED screen
    #[prost(string, tag = "3")]
    pub short_name: ::prost::alloc::string::String,
    ///
    /// TBEAM, HELTEC, etc...
    /// Starting in 1.2.11 moved to hw_model enum in the NodeInfo object.
    /// Apps will still need the string here for older builds
    /// (so OTA update can find the right image), but if the enum is available it will be used instead.
    #[prost(enumeration = "HardwareModel", tag = "4")]
    pub hw_model: i32,
    ///
    /// In some regions Ham radio operators have different bandwidth limitations than others.
    /// If this user is a licensed operator, set this flag.
    /// Also, "long_name" should be their licence number.
    #[prost(bool, tag = "5")]
    pub is_licensed: bool,
    ///
    /// Indicates that the user's role in the mesh
    #[prost(enumeration = "config::device_config::Role", tag = "6")]
    pub role: i32,
    ///
    /// The public key of the user's device.
    /// This is sent out to other nodes on the mesh to allow them to compute a shared secret key.
    #[prost(bytes = "vec", tag = "7")]
    pub public_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeInfoLite {
    ///
    /// The node number
    #[prost(uint32, tag = "1")]
    pub num: u32,
    ///
    /// The user info for this node
    #[prost(message, optional, tag = "2")]
    pub user: ::core::option::Option<UserLite>,
    ///
    /// This position data. Note: before 1.2.14 we would also store the last time we've heard from this node in position.time, that is no longer true.
    /// Position.time now indicates the last time we received a POSITION from that node.
    #[prost(message, optional, tag = "3")]
    pub position: ::core::option::Option<PositionLite>,
    ///
    /// Returns the Signal-to-noise ratio (SNR) of the last received message,
    /// as measured by the receiver. Return SNR of the last received message in dB
    #[prost(float, tag = "4")]
    pub snr: f32,
    ///
    /// Set to indicate the last time we received a packet from this node
    #[prost(fixed32, tag = "5")]
    pub last_heard: u32,
    ///
    /// The latest device metrics for the node.
    #[prost(message, optional, tag = "6")]
    pub device_metrics: ::core::option::Option<DeviceMetrics>,
    ///
    /// local channel index we heard that node on. Only populated if its not the default channel.
    #[prost(uint32, tag = "7")]
    pub channel: u32,
    ///
    /// True if we witnessed the node over MQTT instead of LoRA transport
    #[prost(bool, tag = "8")]
    pub via_mqtt: bool,
    ///
    /// Number of hops away from us this node is (0 if direct neighbor)
    #[prost(uint32, optional, tag = "9")]
    pub hops_away: ::core::option::Option<u32>,
    ///
    /// True if node is in our favorites list
    /// Persists between NodeDB internal clean ups
    #[prost(bool, tag = "10")]
    pub is_favorite: bool,
    ///
    /// True if node is in our ignored list
    /// Persists between NodeDB internal clean ups
    #[prost(bool, tag = "11")]
    pub is_ignored: bool,
    ///
    /// Last byte of the node number of the node that should be used as the next hop to reach this node.
    #[prost(uint32, tag = "12")]
    pub next_hop: u32,
}
///
/// This message is never sent over the wire, but it is used for serializing DB
/// state to flash in the device code
/// FIXME, since we write this each time we enter deep sleep (and have infinite
/// flash) it would be better to use some sort of append only data structure for
/// the receive queue and use the preferences store for the other stuff
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceState {
    ///
    /// Read only settings/info about this node
    #[prost(message, optional, tag = "2")]
    pub my_node: ::core::option::Option<MyNodeInfo>,
    ///
    /// My owner info
    #[prost(message, optional, tag = "3")]
    pub owner: ::core::option::Option<User>,
    ///
    /// Received packets saved for delivery to the phone
    #[prost(message, repeated, tag = "5")]
    pub receive_queue: ::prost::alloc::vec::Vec<MeshPacket>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[prost(uint32, tag = "8")]
    pub version: u32,
    ///
    /// We keep the last received text message (only) stored in the device flash,
    /// so we can show it on the screen.
    /// Might be null
    #[prost(message, optional, tag = "7")]
    pub rx_text_message: ::core::option::Option<MeshPacket>,
    ///
    /// Used only during development.
    /// Indicates developer is testing and changes should never be saved to flash.
    /// Deprecated in 2.3.1
    #[deprecated]
    #[prost(bool, tag = "9")]
    pub no_save: bool,
    ///
    /// Some GPS receivers seem to have bogus settings from the factory, so we always do one factory reset.
    #[prost(bool, tag = "11")]
    pub did_gps_reset: bool,
    ///
    /// We keep the last received waypoint stored in the device flash,
    /// so we can show it on the screen.
    /// Might be null
    #[prost(message, optional, tag = "12")]
    pub rx_waypoint: ::core::option::Option<MeshPacket>,
    ///
    /// The mesh's nodes with their available gpio pins for RemoteHardware module
    #[prost(message, repeated, tag = "13")]
    pub node_remote_hardware_pins: ::prost::alloc::vec::Vec<NodeRemoteHardwarePin>,
    ///
    /// New lite version of NodeDB to decrease memory footprint
    #[prost(message, repeated, tag = "14")]
    pub node_db_lite: ::prost::alloc::vec::Vec<NodeInfoLite>,
}
///
/// The on-disk saved channels
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelFile {
    ///
    /// The channels our node knows about
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[prost(uint32, tag = "2")]
    pub version: u32,
}
///
/// This message wraps a MeshPacket with extra metadata about the sender and how it arrived.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceEnvelope {
    ///
    /// The (probably encrypted) packet
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<MeshPacket>,
    ///
    /// The global channel ID it was sent on
    #[prost(string, tag = "2")]
    pub channel_id: ::prost::alloc::string::String,
    ///
    /// The sending gateway node ID. Can we use this to authenticate/prevent fake
    /// nodeid impersonation for senders? - i.e. use gateway/mesh id (which is authenticated) + local node id as
    /// the globally trusted nodenum
    #[prost(string, tag = "3")]
    pub gateway_id: ::prost::alloc::string::String,
}
///
/// Information about a node intended to be reported unencrypted to a map using MQTT.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapReport {
    ///
    /// A full name for this user, i.e. "Kevin Hester"
    #[prost(string, tag = "1")]
    pub long_name: ::prost::alloc::string::String,
    ///
    /// A VERY short name, ideally two characters.
    /// Suitable for a tiny OLED screen
    #[prost(string, tag = "2")]
    pub short_name: ::prost::alloc::string::String,
    ///
    /// Role of the node that applies specific settings for a particular use-case
    #[prost(enumeration = "config::device_config::Role", tag = "3")]
    pub role: i32,
    ///
    /// Hardware model of the node, i.e. T-Beam, Heltec V3, etc...
    #[prost(enumeration = "HardwareModel", tag = "4")]
    pub hw_model: i32,
    ///
    /// Device firmware version string
    #[prost(string, tag = "5")]
    pub firmware_version: ::prost::alloc::string::String,
    ///
    /// The region code for the radio (US, CN, EU433, etc...)
    #[prost(enumeration = "config::lo_ra_config::RegionCode", tag = "6")]
    pub region: i32,
    ///
    /// Modem preset used by the radio (LongFast, MediumSlow, etc...)
    #[prost(enumeration = "config::lo_ra_config::ModemPreset", tag = "7")]
    pub modem_preset: i32,
    ///
    /// Whether the node has a channel with default PSK and name (LongFast, MediumSlow, etc...)
    /// and it uses the default frequency slot given the region and modem preset.
    #[prost(bool, tag = "8")]
    pub has_default_channel: bool,
    ///
    /// Latitude: multiply by 1e-7 to get degrees in floating point
    #[prost(sfixed32, tag = "9")]
    pub latitude_i: i32,
    ///
    /// Longitude: multiply by 1e-7 to get degrees in floating point
    #[prost(sfixed32, tag = "10")]
    pub longitude_i: i32,
    ///
    /// Altitude in meters above MSL
    #[prost(int32, tag = "11")]
    pub altitude: i32,
    ///
    /// Indicates the bits of precision for latitude and longitude set by the sending node
    #[prost(uint32, tag = "12")]
    pub position_precision: u32,
    ///
    /// Number of online nodes (heard in the last 2 hours) this node has in its list that were received locally (not via MQTT)
    #[prost(uint32, tag = "13")]
    pub num_online_local_nodes: u32,
}
///
/// TODO: REPLACE
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Paxcount {
    ///
    /// seen Wifi devices
    #[prost(uint32, tag = "1")]
    pub wifi: u32,
    ///
    /// Seen BLE devices
    #[prost(uint32, tag = "2")]
    pub ble: u32,
    ///
    /// Uptime in seconds
    #[prost(uint32, tag = "3")]
    pub uptime: u32,
}
/// Note: There are no 'PowerMon' messages normally in use (PowerMons are sent only as structured logs - slogs).
/// But we wrap our State enum in this message to effectively nest a namespace (without our linter yelling at us)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerMon {}
/// Nested message and enum types in `PowerMon`.
pub mod power_mon {
    /// Any significant power changing event in meshtastic should be tagged with a powermon state transition.
    /// If you are making new meshtastic features feel free to add new entries at the end of this definition.
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        None = 0,
        CpuDeepSleep = 1,
        CpuLightSleep = 2,
        ///
        /// The external Vext1 power is on.  Many boards have auxillary power rails that the CPU turns on only
        /// occasionally.  In cases where that rail has multiple devices on it we usually want to have logging on
        /// the state of that rail as an independent record.
        /// For instance on the Heltec Tracker 1.1 board, this rail is the power source for the GPS and screen.
        ///
        /// The log messages will be short and complete (see PowerMon.Event in the protobufs for details).
        /// something like "S:PM:C,0x00001234,REASON" where the hex number is the bitmask of all current states.
        /// (We use a bitmask for states so that if a log message gets lost it won't be fatal)
        Vext1On = 4,
        LoraRxOn = 8,
        LoraTxOn = 16,
        LoraRxActive = 32,
        BtOn = 64,
        LedOn = 128,
        ScreenOn = 256,
        ScreenDrawing = 512,
        WifiOn = 1024,
        ///
        /// GPS is actively trying to find our location
        /// See GPSPowerState for more details
        GpsActive = 2048,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::None => "None",
                State::CpuDeepSleep => "CPU_DeepSleep",
                State::CpuLightSleep => "CPU_LightSleep",
                State::Vext1On => "Vext1_On",
                State::LoraRxOn => "Lora_RXOn",
                State::LoraTxOn => "Lora_TXOn",
                State::LoraRxActive => "Lora_RXActive",
                State::BtOn => "BT_On",
                State::LedOn => "LED_On",
                State::ScreenOn => "Screen_On",
                State::ScreenDrawing => "Screen_Drawing",
                State::WifiOn => "Wifi_On",
                State::GpsActive => "GPS_Active",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "None" => Some(Self::None),
                "CPU_DeepSleep" => Some(Self::CpuDeepSleep),
                "CPU_LightSleep" => Some(Self::CpuLightSleep),
                "Vext1_On" => Some(Self::Vext1On),
                "Lora_RXOn" => Some(Self::LoraRxOn),
                "Lora_TXOn" => Some(Self::LoraTxOn),
                "Lora_RXActive" => Some(Self::LoraRxActive),
                "BT_On" => Some(Self::BtOn),
                "LED_On" => Some(Self::LedOn),
                "Screen_On" => Some(Self::ScreenOn),
                "Screen_Drawing" => Some(Self::ScreenDrawing),
                "Wifi_On" => Some(Self::WifiOn),
                "GPS_Active" => Some(Self::GpsActive),
                _ => None,
            }
        }
    }
}
///
/// PowerStress testing support via the C++ PowerStress module
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PowerStressMessage {
    ///
    /// What type of HardwareMessage is this?
    #[prost(enumeration = "power_stress_message::Opcode", tag = "1")]
    pub cmd: i32,
    #[prost(float, tag = "2")]
    pub num_seconds: f32,
}
/// Nested message and enum types in `PowerStressMessage`.
pub mod power_stress_message {
    ///
    /// What operation would we like the UUT to perform.
    /// note: senders should probably set want_response in their request packets, so that they can know when the state
    /// machine has started processing their request
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Opcode {
        ///
        /// Unset/unused
        Unset = 0,
        /// Print board version slog and send an ack that we are alive and ready to process commands
        PrintInfo = 1,
        /// Try to turn off all automatic processing of packets, screen, sleeping, etc (to make it easier to measure in isolation)
        ForceQuiet = 2,
        /// Stop powerstress processing - probably by just rebooting the board
        EndQuiet = 3,
        /// Turn the screen on
        ScreenOn = 16,
        /// Turn the screen off
        ScreenOff = 17,
        /// Let the CPU run but we assume mostly idling for num_seconds
        CpuIdle = 32,
        /// Force deep sleep for FIXME seconds
        CpuDeepsleep = 33,
        /// Spin the CPU as fast as possible for num_seconds
        CpuFullon = 34,
        /// Turn the LED on for num_seconds (and leave it on - for baseline power measurement purposes)
        LedOn = 48,
        /// Force the LED off for num_seconds
        LedOff = 49,
        /// Completely turn off the LORA radio for num_seconds
        LoraOff = 64,
        /// Send Lora packets for num_seconds
        LoraTx = 65,
        /// Receive Lora packets for num_seconds (node will be mostly just listening, unless an external agent is helping stress this by sending packets on the current channel)
        LoraRx = 66,
        /// Turn off the BT radio for num_seconds
        BtOff = 80,
        /// Turn on the BT radio for num_seconds
        BtOn = 81,
        /// Turn off the WIFI radio for num_seconds
        WifiOff = 96,
        /// Turn on the WIFI radio for num_seconds
        WifiOn = 97,
        /// Turn off the GPS radio for num_seconds
        GpsOff = 112,
        /// Turn on the GPS radio for num_seconds
        GpsOn = 113,
    }
    impl Opcode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Opcode::Unset => "UNSET",
                Opcode::PrintInfo => "PRINT_INFO",
                Opcode::ForceQuiet => "FORCE_QUIET",
                Opcode::EndQuiet => "END_QUIET",
                Opcode::ScreenOn => "SCREEN_ON",
                Opcode::ScreenOff => "SCREEN_OFF",
                Opcode::CpuIdle => "CPU_IDLE",
                Opcode::CpuDeepsleep => "CPU_DEEPSLEEP",
                Opcode::CpuFullon => "CPU_FULLON",
                Opcode::LedOn => "LED_ON",
                Opcode::LedOff => "LED_OFF",
                Opcode::LoraOff => "LORA_OFF",
                Opcode::LoraTx => "LORA_TX",
                Opcode::LoraRx => "LORA_RX",
                Opcode::BtOff => "BT_OFF",
                Opcode::BtOn => "BT_ON",
                Opcode::WifiOff => "WIFI_OFF",
                Opcode::WifiOn => "WIFI_ON",
                Opcode::GpsOff => "GPS_OFF",
                Opcode::GpsOn => "GPS_ON",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "PRINT_INFO" => Some(Self::PrintInfo),
                "FORCE_QUIET" => Some(Self::ForceQuiet),
                "END_QUIET" => Some(Self::EndQuiet),
                "SCREEN_ON" => Some(Self::ScreenOn),
                "SCREEN_OFF" => Some(Self::ScreenOff),
                "CPU_IDLE" => Some(Self::CpuIdle),
                "CPU_DEEPSLEEP" => Some(Self::CpuDeepsleep),
                "CPU_FULLON" => Some(Self::CpuFullon),
                "LED_ON" => Some(Self::LedOn),
                "LED_OFF" => Some(Self::LedOff),
                "LORA_OFF" => Some(Self::LoraOff),
                "LORA_TX" => Some(Self::LoraTx),
                "LORA_RX" => Some(Self::LoraRx),
                "BT_OFF" => Some(Self::BtOff),
                "BT_ON" => Some(Self::BtOn),
                "WIFI_OFF" => Some(Self::WifiOff),
                "WIFI_ON" => Some(Self::WifiOn),
                "GPS_OFF" => Some(Self::GpsOff),
                "GPS_ON" => Some(Self::GpsOn),
                _ => None,
            }
        }
    }
}
///
/// An example app to show off the module system. This message is used for
/// REMOTE_HARDWARE_APP PortNums.
/// Also provides easy remote access to any GPIO.
/// In the future other remote hardware operations can be added based on user interest
/// (i.e. serial output, spi/i2c input/output).
/// FIXME - currently this feature is turned on by default which is dangerous
/// because no security yet (beyond the channel mechanism).
/// It should be off by default and then protected based on some TBD mechanism
/// (a special channel once multichannel support is included?)
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HardwareMessage {
    ///
    /// What type of HardwareMessage is this?
    #[prost(enumeration = "hardware_message::Type", tag = "1")]
    pub r#type: i32,
    ///
    /// What gpios are we changing. Not used for all MessageTypes, see MessageType for details
    #[prost(uint64, tag = "2")]
    pub gpio_mask: u64,
    ///
    /// For gpios that were listed in gpio_mask as valid, what are the signal levels for those gpios.
    /// Not used for all MessageTypes, see MessageType for details
    #[prost(uint64, tag = "3")]
    pub gpio_value: u64,
}
/// Nested message and enum types in `HardwareMessage`.
pub mod hardware_message {
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        ///
        /// Unset/unused
        Unset = 0,
        ///
        /// Set gpio gpios based on gpio_mask/gpio_value
        WriteGpios = 1,
        ///
        /// We are now interested in watching the gpio_mask gpios.
        /// If the selected gpios change, please broadcast GPIOS_CHANGED.
        /// Will implicitly change the gpios requested to be INPUT gpios.
        WatchGpios = 2,
        ///
        /// The gpios listed in gpio_mask have changed, the new values are listed in gpio_value
        GpiosChanged = 3,
        ///
        /// Read the gpios specified in gpio_mask, send back a READ_GPIOS_REPLY reply with gpio_value populated
        ReadGpios = 4,
        ///
        /// A reply to READ_GPIOS. gpio_mask and gpio_value will be populated
        ReadGpiosReply = 5,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unset => "UNSET",
                Type::WriteGpios => "WRITE_GPIOS",
                Type::WatchGpios => "WATCH_GPIOS",
                Type::GpiosChanged => "GPIOS_CHANGED",
                Type::ReadGpios => "READ_GPIOS",
                Type::ReadGpiosReply => "READ_GPIOS_REPLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "WRITE_GPIOS" => Some(Self::WriteGpios),
                "WATCH_GPIOS" => Some(Self::WatchGpios),
                "GPIOS_CHANGED" => Some(Self::GpiosChanged),
                "READ_GPIOS" => Some(Self::ReadGpios),
                "READ_GPIOS_REPLY" => Some(Self::ReadGpiosReply),
                _ => None,
            }
        }
    }
}
///
/// Canned message module configuration.
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RtttlConfig {
    ///
    /// Ringtone for PWM Buzzer in RTTTL Format.
    #[prost(string, tag = "1")]
    pub ringtone: ::prost::alloc::string::String,
}
///
/// TODO: REPLACE
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::doc_lazy_continuation)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAndForward {
    ///
    /// TODO: REPLACE
    #[prost(enumeration = "store_and_forward::RequestResponse", tag = "1")]
    pub rr: i32,
    ///
    /// TODO: REPLACE
    #[prost(oneof = "store_and_forward::Variant", tags = "2, 3, 4, 5")]
    pub variant: ::core::option::Option<store_and_forward::Variant>,
}
/// Nested message and enum types in `StoreAndForward`.
pub mod store_and_forward {
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Statistics {
        ///
        /// Number of messages we have ever seen
        #[prost(uint32, tag = "1")]
        pub messages_total: u32,
        ///
        /// Number of messages we have currently saved our history.
        #[prost(uint32, tag = "2")]
        pub messages_saved: u32,
        ///
        /// Maximum number of messages we will save
        #[prost(uint32, tag = "3")]
        pub messages_max: u32,
        ///
        /// Router uptime in seconds
        #[prost(uint32, tag = "4")]
        pub up_time: u32,
        ///
        /// Number of times any client sent a request to the S&F.
        #[prost(uint32, tag = "5")]
        pub requests: u32,
        ///
        /// Number of times the history was requested.
        #[prost(uint32, tag = "6")]
        pub requests_history: u32,
        ///
        /// Is the heartbeat enabled on the server?
        #[prost(bool, tag = "7")]
        pub heartbeat: bool,
        ///
        /// Maximum number of messages the server will return.
        #[prost(uint32, tag = "8")]
        pub return_max: u32,
        ///
        /// Maximum history window in minutes the server will return messages from.
        #[prost(uint32, tag = "9")]
        pub return_window: u32,
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct History {
        ///
        /// Number of that will be sent to the client
        #[prost(uint32, tag = "1")]
        pub history_messages: u32,
        ///
        /// The window of messages that was used to filter the history client requested
        #[prost(uint32, tag = "2")]
        pub window: u32,
        ///
        /// Index in the packet history of the last message sent in a previous request to the server.
        /// Will be sent to the client before sending the history and can be set in a subsequent request to avoid getting packets the server already sent to the client.
        #[prost(uint32, tag = "3")]
        pub last_request: u32,
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Heartbeat {
        ///
        /// Period in seconds that the heartbeat is sent out that will be sent to the client
        #[prost(uint32, tag = "1")]
        pub period: u32,
        ///
        /// If set, this is not the primary Store & Forward router on the mesh
        #[prost(uint32, tag = "2")]
        pub secondary: u32,
    }
    ///
    /// 001 - 063 = From Router
    /// 064 - 127 = From Client
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RequestResponse {
        ///
        /// Unset/unused
        Unset = 0,
        ///
        /// Router is an in error state.
        RouterError = 1,
        ///
        /// Router heartbeat
        RouterHeartbeat = 2,
        ///
        /// Router has requested the client respond. This can work as a
        /// "are you there" message.
        RouterPing = 3,
        ///
        /// The response to a "Ping"
        RouterPong = 4,
        ///
        /// Router is currently busy. Please try again later.
        RouterBusy = 5,
        ///
        /// Router is responding to a request for history.
        RouterHistory = 6,
        ///
        /// Router is responding to a request for stats.
        RouterStats = 7,
        ///
        /// Router sends a text message from its history that was a direct message.
        RouterTextDirect = 8,
        ///
        /// Router sends a text message from its history that was a broadcast.
        RouterTextBroadcast = 9,
        ///
        /// Client is an in error state.
        ClientError = 64,
        ///
        /// Client has requested a replay from the router.
        ClientHistory = 65,
        ///
        /// Client has requested stats from the router.
        ClientStats = 66,
        ///
        /// Client has requested the router respond. This can work as a
        /// "are you there" message.
        ClientPing = 67,
        ///
        /// The response to a "Ping"
        ClientPong = 68,
        ///
        /// Client has requested that the router abort processing the client's request
        ClientAbort = 106,
    }
    impl RequestResponse {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RequestResponse::Unset => "UNSET",
                RequestResponse::RouterError => "ROUTER_ERROR",
                RequestResponse::RouterHeartbeat => "ROUTER_HEARTBEAT",
                RequestResponse::RouterPing => "ROUTER_PING",
                RequestResponse::RouterPong => "ROUTER_PONG",
                RequestResponse::RouterBusy => "ROUTER_BUSY",
                RequestResponse::RouterHistory => "ROUTER_HISTORY",
                RequestResponse::RouterStats => "ROUTER_STATS",
                RequestResponse::RouterTextDirect => "ROUTER_TEXT_DIRECT",
                RequestResponse::RouterTextBroadcast => "ROUTER_TEXT_BROADCAST",
                RequestResponse::ClientError => "CLIENT_ERROR",
                RequestResponse::ClientHistory => "CLIENT_HISTORY",
                RequestResponse::ClientStats => "CLIENT_STATS",
                RequestResponse::ClientPing => "CLIENT_PING",
                RequestResponse::ClientPong => "CLIENT_PONG",
                RequestResponse::ClientAbort => "CLIENT_ABORT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSET" => Some(Self::Unset),
                "ROUTER_ERROR" => Some(Self::RouterError),
                "ROUTER_HEARTBEAT" => Some(Self::RouterHeartbeat),
                "ROUTER_PING" => Some(Self::RouterPing),
                "ROUTER_PONG" => Some(Self::RouterPong),
                "ROUTER_BUSY" => Some(Self::RouterBusy),
                "ROUTER_HISTORY" => Some(Self::RouterHistory),
                "ROUTER_STATS" => Some(Self::RouterStats),
                "ROUTER_TEXT_DIRECT" => Some(Self::RouterTextDirect),
                "ROUTER_TEXT_BROADCAST" => Some(Self::RouterTextBroadcast),
                "CLIENT_ERROR" => Some(Self::ClientError),
                "CLIENT_HISTORY" => Some(Self::ClientHistory),
                "CLIENT_STATS" => Some(Self::ClientStats),
                "CLIENT_PING" => Some(Self::ClientPing),
                "CLIENT_PONG" => Some(Self::ClientPong),
                "CLIENT_ABORT" => Some(Self::ClientAbort),
                _ => None,
            }
        }
    }
    ///
    /// TODO: REPLACE
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[allow(clippy::doc_lazy_continuation)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Variant {
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "2")]
        Stats(Statistics),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "3")]
        History(History),
        ///
        /// TODO: REPLACE
        #[prost(message, tag = "4")]
        Heartbeat(Heartbeat),
        ///
        /// Text from history message.
        #[prost(bytes, tag = "5")]
        Text(::prost::alloc::vec::Vec<u8>),
    }
}
