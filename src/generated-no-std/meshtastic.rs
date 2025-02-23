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
/// The PSK is hashed into this letter by "0x41 + \[xor all bytes of the psk \] modulo 26"
/// This also allows the option of someday if people have the PSK off (zero), the
/// users COULD type in a channel name and be able to talk.
/// FIXME: Add description of multi-channel support and how primary vs secondary channels are used.
/// FIXME: explain how apps use channels for security.
/// explain how remote settings and remote gpio are managed as an example
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ChannelSettings<'a> {
    ///
    /// Deprecated in favor of LoraConfig.channel_num
    #[deprecated]
    #[femtopb(uint32, tag = 1)]
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
    #[femtopb(bytes, tag = 2)]
    pub psk: &'a [u8],
    ///
    /// A SHORT name that will be packed into the URL.
    /// Less than 12 bytes.
    /// Something for end users to call the channel
    /// If this is the empty string it is assumed that this channel
    /// is the special (minimally secure) "Default"channel.
    /// In user interfaces it should be rendered as a local language translation of "X".
    /// For channel_num hashing empty string will be treated as "X".
    /// Where "X" is selected based on the English words listed above for ModemPreset
    #[femtopb(string, tag = 3)]
    pub name: &'a str,
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
    #[femtopb(fixed32, tag = 4)]
    pub id: u32,
    ///
    /// If true, messages on the mesh will be sent to the *public* internet by any gateway ndoe
    #[femtopb(bool, tag = 5)]
    pub uplink_enabled: bool,
    ///
    /// If true, messages seen on the internet will be forwarded to the local mesh.
    #[femtopb(bool, tag = 6)]
    pub downlink_enabled: bool,
    ///
    /// Per-channel module settings.
    #[femtopb(message, optional, tag = 7)]
    pub module_settings: ::core::option::Option<ModuleSettings<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This message is specifically for modules to store per-channel configuration data.
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct ModuleSettings<'a> {
    ///
    /// Bits of precision for the location sent in position packets.
    #[femtopb(uint32, tag = 1)]
    pub position_precision: u32,
    ///
    /// Controls whether or not the phone / clients should mute the current channel
    /// Useful for noisy public channels you don't necessarily want to disable
    #[femtopb(bool, tag = 2)]
    pub is_client_muted: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// A pair of a channel number, mode and the (sharable) settings for that channel
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Channel<'a> {
    ///
    /// The index of this channel in the channel table (from 0 to MAX_NUM_CHANNELS-1)
    /// (Someday - not currently implemented) An index of -1 could be used to mean "set by name",
    /// in which case the target node will find and set the channel by settings.name.
    #[femtopb(int32, tag = 1)]
    pub index: i32,
    ///
    /// The new settings, or NULL to disable that channel
    #[femtopb(message, optional, tag = 2)]
    pub settings: ::core::option::Option<ChannelSettings<'a>>,
    ///
    /// TODO: REPLACE
    #[femtopb(enumeration, tag = 3)]
    pub role: ::femtopb::enumeration::EnumValue<channel::Role>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
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
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Role {
        ///
        /// This channel is not in use right now
        #[default]
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
                Self::Disabled => "DISABLED",
                Self::Primary => "PRIMARY",
                Self::Secondary => "SECONDARY",
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
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Config<'a> {
    ///
    /// Payload Variant
    #[femtopb(oneof, tags = [1, 2, 3, 4, 5, 6, 7])]
    pub payload_variant: ::core::option::Option<config::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `Config`.
pub mod config {
    ///
    /// Configuration
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct DeviceConfig<'a> {
        ///
        /// Sets the role of node
        #[femtopb(enumeration, tag = 1)]
        pub role: ::femtopb::enumeration::EnumValue<device_config::Role>,
        ///
        /// Disabling this will disable the SerialConsole by not initilizing the StreamAPI
        #[femtopb(bool, tag = 2)]
        pub serial_enabled: bool,
        ///
        /// By default we turn off logging as soon as an API client connects (to keep shared serial link quiet).
        /// Set this to true to leave the debug log outputting even when API is active.
        #[femtopb(bool, tag = 3)]
        pub debug_log_enabled: bool,
        ///
        /// For boards without a hard wired button, this is the pin number that will be used
        /// Boards that have more than one button can swap the function with this one. defaults to BUTTON_PIN if defined.
        #[femtopb(uint32, tag = 4)]
        pub button_gpio: u32,
        ///
        /// For boards without a PWM buzzer, this is the pin number that will be used
        /// Defaults to PIN_BUZZER if defined.
        #[femtopb(uint32, tag = 5)]
        pub buzzer_gpio: u32,
        ///
        /// Sets the role of node
        #[femtopb(enumeration, tag = 6)]
        pub rebroadcast_mode: ::femtopb::enumeration::EnumValue<
            device_config::RebroadcastMode,
        >,
        ///
        /// Send our nodeinfo this often
        /// Defaults to 900 Seconds (15 minutes)
        #[femtopb(uint32, tag = 7)]
        pub node_info_broadcast_secs: u32,
        ///
        /// Treat double tap interrupt on supported accelerometers as a button press if set to true
        #[femtopb(bool, tag = 8)]
        pub double_tap_as_button_press: bool,
        ///
        /// If true, device is considered to be "managed" by a mesh administrator
        /// Clients should then limit available configuration and administrative options inside the user interface
        #[femtopb(bool, tag = 9)]
        pub is_managed: bool,
        ///
        /// Disables the triple-press of user button to enable or disable GPS
        #[femtopb(bool, tag = 10)]
        pub disable_triple_click: bool,
        ///
        /// POSIX Timezone definition string from <https://github.com/nayarsystems/posix_tz_db/blob/master/zones.csv.>
        #[femtopb(string, tag = 11)]
        pub tzdef: &'a str,
        ///
        /// If true, disable the default blinking LED (LED_PIN) behavior on the device
        #[femtopb(bool, tag = 12)]
        pub led_heartbeat_disabled: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `DeviceConfig`.
    pub mod device_config {
        ///
        /// Defines the device's role on the Mesh network
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum Role {
            ///
            /// Description: App connected or stand alone messaging device.
            /// Technical Details: Default Role
            #[default]
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
        }
        impl Role {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Client => "CLIENT",
                    Self::ClientMute => "CLIENT_MUTE",
                    Self::Router => "ROUTER",
                    Self::RouterClient => "ROUTER_CLIENT",
                    Self::Repeater => "REPEATER",
                    Self::Tracker => "TRACKER",
                    Self::Sensor => "SENSOR",
                    Self::Tak => "TAK",
                    Self::ClientHidden => "CLIENT_HIDDEN",
                    Self::LostAndFound => "LOST_AND_FOUND",
                    Self::TakTracker => "TAK_TRACKER",
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
                    _ => None,
                }
            }
        }
        ///
        /// Defines the device's behavior for how messages are rebroadcast
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum RebroadcastMode {
            ///
            /// Default behavior.
            /// Rebroadcast any observed message, if it was on our private channel or from another mesh with the same lora params.
            #[default]
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
        }
        impl RebroadcastMode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::All => "ALL",
                    Self::AllSkipDecoding => "ALL_SKIP_DECODING",
                    Self::LocalOnly => "LOCAL_ONLY",
                    Self::KnownOnly => "KNOWN_ONLY",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "ALL" => Some(Self::All),
                    "ALL_SKIP_DECODING" => Some(Self::AllSkipDecoding),
                    "LOCAL_ONLY" => Some(Self::LocalOnly),
                    "KNOWN_ONLY" => Some(Self::KnownOnly),
                    _ => None,
                }
            }
        }
    }
    ///
    /// Position Config
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct PositionConfig<'a> {
        ///
        /// We should send our position this often (but only if it has changed significantly)
        /// Defaults to 15 minutes
        #[femtopb(uint32, tag = 1)]
        pub position_broadcast_secs: u32,
        ///
        /// Adaptive position braoadcast, which is now the default.
        #[femtopb(bool, tag = 2)]
        pub position_broadcast_smart_enabled: bool,
        ///
        /// If set, this node is at a fixed position.
        /// We will generate GPS position updates at the regular interval, but use whatever the last lat/lon/alt we have for the node.
        /// The lat/lon/alt can be set by an internal GPS or with the help of the app.
        #[femtopb(bool, tag = 3)]
        pub fixed_position: bool,
        ///
        /// Is GPS enabled for this node?
        #[deprecated]
        #[femtopb(bool, tag = 4)]
        pub gps_enabled: bool,
        ///
        /// How often should we try to get GPS position (in seconds)
        /// or zero for the default of once every 30 seconds
        /// or a very large value (maxint) to update only once at boot.
        #[femtopb(uint32, tag = 5)]
        pub gps_update_interval: u32,
        ///
        /// Deprecated in favor of using smart / regular broadcast intervals as implicit attempt time
        #[deprecated]
        #[femtopb(uint32, tag = 6)]
        pub gps_attempt_time: u32,
        ///
        /// Bit field of boolean configuration options for POSITION messages
        /// (bitwise OR of PositionFlags)
        #[femtopb(uint32, tag = 7)]
        pub position_flags: u32,
        ///
        /// (Re)define GPS_RX_PIN for your board.
        #[femtopb(uint32, tag = 8)]
        pub rx_gpio: u32,
        ///
        /// (Re)define GPS_TX_PIN for your board.
        #[femtopb(uint32, tag = 9)]
        pub tx_gpio: u32,
        ///
        /// The minimum distance in meters traveled (since the last send) before we can send a position to the mesh if position_broadcast_smart_enabled
        #[femtopb(uint32, tag = 10)]
        pub broadcast_smart_minimum_distance: u32,
        ///
        /// The minimum number of seconds (since the last send) before we can send a position to the mesh if position_broadcast_smart_enabled
        #[femtopb(uint32, tag = 11)]
        pub broadcast_smart_minimum_interval_secs: u32,
        ///
        /// (Re)define PIN_GPS_EN for your board.
        #[femtopb(uint32, tag = 12)]
        pub gps_en_gpio: u32,
        ///
        /// Set where GPS is enabled, disabled, or not present
        #[femtopb(enumeration, tag = 13)]
        pub gps_mode: ::femtopb::enumeration::EnumValue<position_config::GpsMode>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum PositionFlags {
            ///
            /// Required for compilation
            #[default]
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
                    Self::Unset => "UNSET",
                    Self::Altitude => "ALTITUDE",
                    Self::AltitudeMsl => "ALTITUDE_MSL",
                    Self::GeoidalSeparation => "GEOIDAL_SEPARATION",
                    Self::Dop => "DOP",
                    Self::Hvdop => "HVDOP",
                    Self::Satinview => "SATINVIEW",
                    Self::SeqNo => "SEQ_NO",
                    Self::Timestamp => "TIMESTAMP",
                    Self::Heading => "HEADING",
                    Self::Speed => "SPEED",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum GpsMode {
            ///
            /// GPS is present but disabled
            #[default]
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
                    Self::Disabled => "DISABLED",
                    Self::Enabled => "ENABLED",
                    Self::NotPresent => "NOT_PRESENT",
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
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct PowerConfig<'a> {
        ///
        /// Description: Will sleep everything as much as possible, for the tracker and sensor role this will also include the lora radio.
        /// Don't use this setting if you want to use your device with the phone apps or are using a device without a user button.
        /// Technical Details: Works for ESP32 devices and NRF52 devices in the Sensor or Tracker roles
        #[femtopb(bool, tag = 1)]
        pub is_power_saving: bool,
        ///
        ///   Description: If non-zero, the device will fully power off this many seconds after external power is removed.
        #[femtopb(uint32, tag = 2)]
        pub on_battery_shutdown_after_secs: u32,
        ///
        /// Ratio of voltage divider for battery pin eg. 3.20 (R1=100k, R2=220k)
        /// Overrides the ADC_MULTIPLIER defined in variant for battery voltage calculation.
        /// <https://meshtastic.org/docs/configuration/radio/power/#adc-multiplier-override>
        /// Should be set to floating point value between 2 and 6
        #[femtopb(float, tag = 3)]
        pub adc_multiplier_override: f32,
        ///
        ///   Description: The number of seconds for to wait before turning off BLE in No Bluetooth states
        ///   Technical Details: ESP32 Only 0 for default of 1 minute
        #[femtopb(uint32, tag = 4)]
        pub wait_bluetooth_secs: u32,
        ///
        /// Super Deep Sleep Seconds
        /// While in Light Sleep if mesh_sds_timeout_secs is exceeded we will lower into super deep sleep
        /// for this value (default 1 year) or a button press
        /// 0 for default of one year
        #[femtopb(uint32, tag = 6)]
        pub sds_secs: u32,
        ///
        /// Description: In light sleep the CPU is suspended, LoRa radio is on, BLE is off an GPS is on
        /// Technical Details: ESP32 Only 0 for default of 300
        #[femtopb(uint32, tag = 7)]
        pub ls_secs: u32,
        ///
        /// Description: While in light sleep when we receive packets on the LoRa radio we will wake and handle them and stay awake in no BLE mode for this value
        /// Technical Details: ESP32 Only 0 for default of 10 seconds
        #[femtopb(uint32, tag = 8)]
        pub min_wake_secs: u32,
        ///
        /// I2C address of INA_2XX to use for reading device battery voltage
        #[femtopb(uint32, tag = 9)]
        pub device_battery_ina_address: u32,
        ///
        /// If non-zero, we want powermon log outputs.  With the particular (bitfield) sources enabled.
        /// Note: we picked an ID of 32 so that lower more efficient IDs can be used for more frequently used options.
        #[femtopb(uint64, tag = 32)]
        pub powermon_enables: u64,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Network Config
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct NetworkConfig<'a> {
        ///
        /// Enable WiFi (disables Bluetooth)
        #[femtopb(bool, tag = 1)]
        pub wifi_enabled: bool,
        ///
        /// If set, this node will try to join the specified wifi network and
        /// acquire an address via DHCP
        #[femtopb(string, tag = 3)]
        pub wifi_ssid: &'a str,
        ///
        /// If set, will be use to authenticate to the named wifi
        #[femtopb(string, tag = 4)]
        pub wifi_psk: &'a str,
        ///
        /// NTP server to use if WiFi is conneced, defaults to `0.pool.ntp.org`
        #[femtopb(string, tag = 5)]
        pub ntp_server: &'a str,
        ///
        /// Enable Ethernet
        #[femtopb(bool, tag = 6)]
        pub eth_enabled: bool,
        ///
        /// acquire an address via DHCP or assign static
        #[femtopb(enumeration, tag = 7)]
        pub address_mode: ::femtopb::enumeration::EnumValue<network_config::AddressMode>,
        ///
        /// struct to keep static address
        #[femtopb(message, optional, tag = 8)]
        pub ipv4_config: ::core::option::Option<network_config::IpV4Config<'a>>,
        ///
        /// rsyslog Server and Port
        #[femtopb(string, tag = 9)]
        pub rsyslog_server: &'a str,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `NetworkConfig`.
    pub mod network_config {
        #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
        pub struct IpV4Config<'a> {
            ///
            /// Static IP address
            #[femtopb(fixed32, tag = 1)]
            pub ip: u32,
            ///
            /// Static gateway address
            #[femtopb(fixed32, tag = 2)]
            pub gateway: u32,
            ///
            /// Static subnet mask
            #[femtopb(fixed32, tag = 3)]
            pub subnet: u32,
            ///
            /// Static DNS server address
            #[femtopb(fixed32, tag = 4)]
            pub dns: u32,
            #[femtopb(unknown_fields)]
            pub unknown_fields: femtopb::UnknownFields<'a>,
        }
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum AddressMode {
            ///
            /// obtain ip address via DHCP
            #[default]
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
                    Self::Dhcp => "DHCP",
                    Self::Static => "STATIC",
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
    }
    ///
    /// Display Config
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct DisplayConfig<'a> {
        ///
        /// Number of seconds the screen stays on after pressing the user button or receiving a message
        /// 0 for default of one minute MAXUINT for always on
        #[femtopb(uint32, tag = 1)]
        pub screen_on_secs: u32,
        ///
        /// How the GPS coordinates are formatted on the OLED screen.
        #[femtopb(enumeration, tag = 2)]
        pub gps_format: ::femtopb::enumeration::EnumValue<
            display_config::GpsCoordinateFormat,
        >,
        ///
        /// Automatically toggles to the next page on the screen like a carousel, based the specified interval in seconds.
        /// Potentially useful for devices without user buttons.
        #[femtopb(uint32, tag = 3)]
        pub auto_screen_carousel_secs: u32,
        ///
        /// If this is set, the displayed compass will always point north. if unset, the old behaviour
        /// (top of display is heading direction) is used.
        #[femtopb(bool, tag = 4)]
        pub compass_north_top: bool,
        ///
        /// Flip screen vertically, for cases that mount the screen upside down
        #[femtopb(bool, tag = 5)]
        pub flip_screen: bool,
        ///
        /// Perferred display units
        #[femtopb(enumeration, tag = 6)]
        pub units: ::femtopb::enumeration::EnumValue<display_config::DisplayUnits>,
        ///
        /// Override auto-detect in screen
        #[femtopb(enumeration, tag = 7)]
        pub oled: ::femtopb::enumeration::EnumValue<display_config::OledType>,
        ///
        /// Display Mode
        #[femtopb(enumeration, tag = 8)]
        pub displaymode: ::femtopb::enumeration::EnumValue<display_config::DisplayMode>,
        ///
        /// Print first line in pseudo-bold? FALSE is original style, TRUE is bold
        #[femtopb(bool, tag = 9)]
        pub heading_bold: bool,
        ///
        /// Should we wake the screen up on accelerometer detected motion or tap
        #[femtopb(bool, tag = 10)]
        pub wake_on_tap_or_motion: bool,
        ///
        /// Indicates how to rotate or invert the compass output to accurate display on the display.
        #[femtopb(enumeration, tag = 11)]
        pub compass_orientation: ::femtopb::enumeration::EnumValue<
            display_config::CompassOrientation,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `DisplayConfig`.
    pub mod display_config {
        ///
        /// How the GPS coordinates are displayed on the OLED screen.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum GpsCoordinateFormat {
            ///
            /// GPS coordinates are displayed in the normal decimal degrees format:
            /// DD.DDDDDD DDD.DDDDDD
            #[default]
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
                    Self::Dec => "DEC",
                    Self::Dms => "DMS",
                    Self::Utm => "UTM",
                    Self::Mgrs => "MGRS",
                    Self::Olc => "OLC",
                    Self::Osgr => "OSGR",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum DisplayUnits {
            ///
            /// Metric (Default)
            #[default]
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
                    Self::Metric => "METRIC",
                    Self::Imperial => "IMPERIAL",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum OledType {
            ///
            /// Default / Auto
            #[default]
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
                    Self::OledAuto => "OLED_AUTO",
                    Self::OledSsd1306 => "OLED_SSD1306",
                    Self::OledSh1106 => "OLED_SH1106",
                    Self::OledSh1107 => "OLED_SH1107",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum DisplayMode {
            ///
            /// Default. The old style for the 128x64 OLED screen
            #[default]
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
                    Self::Default => "DEFAULT",
                    Self::Twocolor => "TWOCOLOR",
                    Self::Inverted => "INVERTED",
                    Self::Color => "COLOR",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum CompassOrientation {
            ///
            /// The compass and the display are in the same orientation.
            #[default]
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
                    Self::Degrees0 => "DEGREES_0",
                    Self::Degrees90 => "DEGREES_90",
                    Self::Degrees180 => "DEGREES_180",
                    Self::Degrees270 => "DEGREES_270",
                    Self::Degrees0Inverted => "DEGREES_0_INVERTED",
                    Self::Degrees90Inverted => "DEGREES_90_INVERTED",
                    Self::Degrees180Inverted => "DEGREES_180_INVERTED",
                    Self::Degrees270Inverted => "DEGREES_270_INVERTED",
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
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct LoRaConfig<'a> {
        ///
        /// When enabled, the `modem_preset` fields will be adhered to, else the `bandwidth`/`spread_factor`/`coding_rate`
        /// will be taked from their respective manually defined fields
        #[femtopb(bool, tag = 1)]
        pub use_preset: bool,
        ///
        /// Either modem_config or bandwidth/spreading/coding will be specified - NOT BOTH.
        /// As a heuristic: If bandwidth is specified, do not use modem_config.
        /// Because protobufs take ZERO space when the value is zero this works out nicely.
        /// This value is replaced by bandwidth/spread_factor/coding_rate.
        /// If you'd like to experiment with other options add them to MeshRadio.cpp in the device code.
        #[femtopb(enumeration, tag = 2)]
        pub modem_preset: ::femtopb::enumeration::EnumValue<lo_ra_config::ModemPreset>,
        ///
        /// Bandwidth in MHz
        /// Certain bandwidth numbers are 'special' and will be converted to the
        /// appropriate floating point value: 31 -> 31.25MHz
        #[femtopb(uint32, tag = 3)]
        pub bandwidth: u32,
        ///
        /// A number from 7 to 12.
        /// Indicates number of chirps per symbol as 1<<spread_factor.
        #[femtopb(uint32, tag = 4)]
        pub spread_factor: u32,
        ///
        /// The denominator of the coding rate.
        /// ie for 4/5, the value is 5. 4/8 the value is 8.
        #[femtopb(uint32, tag = 5)]
        pub coding_rate: u32,
        ///
        /// This parameter is for advanced users with advanced test equipment, we do not recommend most users use it.
        /// A frequency offset that is added to to the calculated band center frequency.
        /// Used to correct for crystal calibration errors.
        #[femtopb(float, tag = 6)]
        pub frequency_offset: f32,
        ///
        /// The region code for the radio (US, CN, EU433, etc...)
        #[femtopb(enumeration, tag = 7)]
        pub region: ::femtopb::enumeration::EnumValue<lo_ra_config::RegionCode>,
        ///
        /// Maximum number of hops. This can't be greater than 7.
        /// Default of 3
        /// Attempting to set a value > 7 results in the default
        #[femtopb(uint32, tag = 8)]
        pub hop_limit: u32,
        ///
        /// Disable TX from the LoRa radio. Useful for hot-swapping antennas and other tests.
        /// Defaults to false
        #[femtopb(bool, tag = 9)]
        pub tx_enabled: bool,
        ///
        /// If zero, then use default max legal continuous power (ie. something that won't
        /// burn out the radio hardware)
        /// In most cases you should use zero here.
        /// Units are in dBm.
        #[femtopb(int32, tag = 10)]
        pub tx_power: i32,
        ///
        /// This controls the actual hardware frequency the radio transmits on.
        /// Most users should never need to be exposed to this field/concept.
        /// A channel number between 1 and NUM_CHANNELS (whatever the max is in the current region).
        /// If ZERO then the rule is "use the old channel name hash based
        /// algorithm to derive the channel number")
        /// If using the hash algorithm the channel number will be: hash(channel_name) %
        /// NUM_CHANNELS (Where num channels depends on the regulatory region).
        #[femtopb(uint32, tag = 11)]
        pub channel_num: u32,
        ///
        /// If true, duty cycle limits will be exceeded and thus you're possibly not following
        /// the local regulations if you're not a HAM.
        /// Has no effect if the duty cycle of the used region is 100%.
        #[femtopb(bool, tag = 12)]
        pub override_duty_cycle: bool,
        ///
        /// If true, sets RX boosted gain mode on SX126X based radios
        #[femtopb(bool, tag = 13)]
        pub sx126x_rx_boosted_gain: bool,
        ///
        /// This parameter is for advanced users and licensed HAM radio operators.
        /// Ignore Channel Calculation and use this frequency instead. The frequency_offset
        /// will still be applied. This will allow you to use out-of-band frequencies.
        /// Please respect your local laws and regulations. If you are a HAM, make sure you
        /// enable HAM mode and turn off encryption.
        #[femtopb(float, tag = 14)]
        pub override_frequency: f32,
        ///
        /// If true, disable the build-in PA FAN using pin define in RF95_FAN_EN.
        #[femtopb(bool, tag = 15)]
        pub pa_fan_disabled: bool,
        ///
        /// For testing it is useful sometimes to force a node to never listen to
        /// particular other nodes (simulating radio out of range). All nodenums listed
        /// in ignore_incoming will have packets they send dropped on receive (by router.cpp)
        #[femtopb(uint32, packed, tag = 103)]
        pub ignore_incoming: ::femtopb::packed::Packed<
            'a,
            u32,
            ::femtopb::item_encoding::UInt32,
        >,
        ///
        /// If true, the device will not process any packets received via LoRa that passed via MQTT anywhere on the path towards it.
        #[femtopb(bool, tag = 104)]
        pub ignore_mqtt: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `LoRaConfig`.
    pub mod lo_ra_config {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum RegionCode {
            ///
            /// Region is not set
            #[default]
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
        }
        impl RegionCode {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::Unset => "UNSET",
                    Self::Us => "US",
                    Self::Eu433 => "EU_433",
                    Self::Eu868 => "EU_868",
                    Self::Cn => "CN",
                    Self::Jp => "JP",
                    Self::Anz => "ANZ",
                    Self::Kr => "KR",
                    Self::Tw => "TW",
                    Self::Ru => "RU",
                    Self::In => "IN",
                    Self::Nz865 => "NZ_865",
                    Self::Th => "TH",
                    Self::Lora24 => "LORA_24",
                    Self::Ua433 => "UA_433",
                    Self::Ua868 => "UA_868",
                    Self::My433 => "MY_433",
                    Self::My919 => "MY_919",
                    Self::Sg923 => "SG_923",
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
                    _ => None,
                }
            }
        }
        ///
        /// Standard predefined channel settings
        /// Note: these mappings must match ModemPreset Choice in the device code.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum ModemPreset {
            ///
            /// Long Range - Fast
            #[default]
            LongFast = 0,
            ///
            /// Long Range - Slow
            LongSlow = 1,
            ///
            /// Very Long Range - Slow
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
        }
        impl ModemPreset {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Self::LongFast => "LONG_FAST",
                    Self::LongSlow => "LONG_SLOW",
                    Self::VeryLongSlow => "VERY_LONG_SLOW",
                    Self::MediumSlow => "MEDIUM_SLOW",
                    Self::MediumFast => "MEDIUM_FAST",
                    Self::ShortSlow => "SHORT_SLOW",
                    Self::ShortFast => "SHORT_FAST",
                    Self::LongModerate => "LONG_MODERATE",
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
                    _ => None,
                }
            }
        }
    }
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct BluetoothConfig<'a> {
        ///
        /// Enable Bluetooth on the device
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// Determines the pairing strategy for the device
        #[femtopb(enumeration, tag = 2)]
        pub mode: ::femtopb::enumeration::EnumValue<bluetooth_config::PairingMode>,
        ///
        /// Specified PIN for PairingMode.FixedPin
        #[femtopb(uint32, tag = 3)]
        pub fixed_pin: u32,
        ///
        /// Enables device (serial style logs) over Bluetooth
        #[femtopb(bool, tag = 4)]
        pub device_logging_enabled: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `BluetoothConfig`.
    pub mod bluetooth_config {
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum PairingMode {
            ///
            /// Device generates a random PIN that will be shown on the screen of the device for pairing
            #[default]
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
                    Self::RandomPin => "RANDOM_PIN",
                    Self::FixedPin => "FIXED_PIN",
                    Self::NoPin => "NO_PIN",
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
    ///
    /// Payload Variant
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        #[femtopb(message, tag = 1)]
        Device(DeviceConfig<'a>),
        #[femtopb(message, tag = 2)]
        Position(PositionConfig<'a>),
        #[femtopb(message, tag = 3)]
        Power(PowerConfig<'a>),
        #[femtopb(message, tag = 4)]
        Network(NetworkConfig<'a>),
        #[femtopb(message, tag = 5)]
        Display(DisplayConfig<'a>),
        #[femtopb(message, tag = 6)]
        Lora(LoRaConfig<'a>),
        #[femtopb(message, tag = 7)]
        Bluetooth(BluetoothConfig<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct DeviceConnectionStatus<'a> {
    ///
    /// WiFi Status
    #[femtopb(message, optional, tag = 1)]
    pub wifi: ::core::option::Option<WifiConnectionStatus<'a>>,
    ///
    /// WiFi Status
    #[femtopb(message, optional, tag = 2)]
    pub ethernet: ::core::option::Option<EthernetConnectionStatus<'a>>,
    ///
    /// Bluetooth Status
    #[femtopb(message, optional, tag = 3)]
    pub bluetooth: ::core::option::Option<BluetoothConnectionStatus<'a>>,
    ///
    /// Serial Status
    #[femtopb(message, optional, tag = 4)]
    pub serial: ::core::option::Option<SerialConnectionStatus<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// WiFi connection status
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct WifiConnectionStatus<'a> {
    ///
    /// Connection status
    #[femtopb(message, optional, tag = 1)]
    pub status: ::core::option::Option<NetworkConnectionStatus<'a>>,
    ///
    /// WiFi access point SSID
    #[femtopb(string, tag = 2)]
    pub ssid: &'a str,
    ///
    /// RSSI of wireless connection
    #[femtopb(int32, tag = 3)]
    pub rssi: i32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Ethernet connection status
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct EthernetConnectionStatus<'a> {
    ///
    /// Connection status
    #[femtopb(message, optional, tag = 1)]
    pub status: ::core::option::Option<NetworkConnectionStatus<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Ethernet or WiFi connection status
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct NetworkConnectionStatus<'a> {
    ///
    /// IP address of device
    #[femtopb(fixed32, tag = 1)]
    pub ip_address: u32,
    ///
    /// Whether the device has an active connection or not
    #[femtopb(bool, tag = 2)]
    pub is_connected: bool,
    ///
    /// Whether the device has an active connection to an MQTT broker or not
    #[femtopb(bool, tag = 3)]
    pub is_mqtt_connected: bool,
    ///
    /// Whether the device is actively remote syslogging or not
    #[femtopb(bool, tag = 4)]
    pub is_syslog_connected: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Bluetooth connection status
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct BluetoothConnectionStatus<'a> {
    ///
    /// The pairing PIN for bluetooth
    #[femtopb(uint32, tag = 1)]
    pub pin: u32,
    ///
    /// RSSI of bluetooth connection
    #[femtopb(int32, tag = 2)]
    pub rssi: i32,
    ///
    /// Whether the device has an active connection or not
    #[femtopb(bool, tag = 3)]
    pub is_connected: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Serial connection status
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct SerialConnectionStatus<'a> {
    ///
    /// Serial baud rate
    #[femtopb(uint32, tag = 1)]
    pub baud: u32,
    ///
    /// Whether the device has an active connection or not
    #[femtopb(bool, tag = 2)]
    pub is_connected: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Module Config
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ModuleConfig<'a> {
    ///
    /// TODO: REPLACE
    #[femtopb(oneof, tags = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13])]
    pub payload_variant: ::core::option::Option<module_config::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `ModuleConfig`.
pub mod module_config {
    ///
    /// MQTT Client Config
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct MqttConfig<'a> {
        ///
        /// If a meshtastic node is able to reach the internet it will normally attempt to gateway any channels that are marked as
        /// is_uplink_enabled or is_downlink_enabled.
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// The server to use for our MQTT global message gateway feature.
        /// If not set, the default server will be used
        #[femtopb(string, tag = 2)]
        pub address: &'a str,
        ///
        /// MQTT username to use (most useful for a custom MQTT server).
        /// If using a custom server, this will be honoured even if empty.
        /// If using the default server, this will only be honoured if set, otherwise the device will use the default username
        #[femtopb(string, tag = 3)]
        pub username: &'a str,
        ///
        /// MQTT password to use (most useful for a custom MQTT server).
        /// If using a custom server, this will be honoured even if empty.
        /// If using the default server, this will only be honoured if set, otherwise the device will use the default password
        #[femtopb(string, tag = 4)]
        pub password: &'a str,
        ///
        /// Whether to send encrypted or decrypted packets to MQTT.
        /// This parameter is only honoured if you also set server
        /// (the default official mqtt.meshtastic.org server can handle encrypted packets)
        /// Decrypted packets may be useful for external systems that want to consume meshtastic packets
        #[femtopb(bool, tag = 5)]
        pub encryption_enabled: bool,
        ///
        /// Whether to send / consume json packets on MQTT
        #[femtopb(bool, tag = 6)]
        pub json_enabled: bool,
        ///
        /// If true, we attempt to establish a secure connection using TLS
        #[femtopb(bool, tag = 7)]
        pub tls_enabled: bool,
        ///
        /// The root topic to use for MQTT messages. Default is "msh".
        /// This is useful if you want to use a single MQTT server for multiple meshtastic networks and separate them via ACLs
        #[femtopb(string, tag = 8)]
        pub root: &'a str,
        ///
        /// If true, we can use the connected phone / client to proxy messages to MQTT instead of a direct connection
        #[femtopb(bool, tag = 9)]
        pub proxy_to_client_enabled: bool,
        ///
        /// If true, we will periodically report unencrypted information about our node to a map via MQTT
        #[femtopb(bool, tag = 10)]
        pub map_reporting_enabled: bool,
        ///
        /// Settings for reporting information about our node to a map via MQTT
        #[femtopb(message, optional, tag = 11)]
        pub map_report_settings: ::core::option::Option<MapReportSettings<'a>>,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Settings for reporting unencrypted information about our node to a map via MQTT
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct MapReportSettings<'a> {
        ///
        /// How often we should report our info to the map (in seconds)
        #[femtopb(uint32, tag = 1)]
        pub publish_interval_secs: u32,
        ///
        /// Bits of precision for the location sent (default of 32 is full precision).
        #[femtopb(uint32, tag = 2)]
        pub position_precision: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// RemoteHardwareModule Config
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct RemoteHardwareConfig<'a> {
        ///
        /// Whether the Module is enabled
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// Whether the Module allows consumers to read / write to pins not defined in available_pins
        #[femtopb(bool, tag = 2)]
        pub allow_undefined_pin_access: bool,
        ///
        /// Exposes the available pins to the mesh for reading and writing
        #[femtopb(message, repeated, tag = 3)]
        pub available_pins: ::femtopb::repeated::Repeated<
            'a,
            super::RemoteHardwarePin<'a>,
            ::femtopb::item_encoding::Message<'a, super::RemoteHardwarePin<'a>>,
        >,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// NeighborInfoModule Config
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct NeighborInfoConfig<'a> {
        ///
        /// Whether the Module is enabled
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// Neighbor Info to the mesh
        #[femtopb(uint32, tag = 2)]
        pub update_interval: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Detection Sensor Module Config
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct DetectionSensorConfig<'a> {
        ///
        /// Whether the Module is enabled
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// Interval in seconds of how often we can send a message to the mesh when a state change is detected
        #[femtopb(uint32, tag = 2)]
        pub minimum_broadcast_secs: u32,
        ///
        /// Interval in seconds of how often we should send a message to the mesh with the current state regardless of changes
        /// When set to 0, only state changes will be broadcasted
        /// Works as a sort of status heartbeat for peace of mind
        #[femtopb(uint32, tag = 3)]
        pub state_broadcast_secs: u32,
        ///
        /// Send ASCII bell with alert message
        /// Useful for triggering ext. notification on bell
        #[femtopb(bool, tag = 4)]
        pub send_bell: bool,
        ///
        /// Friendly name used to format message sent to mesh
        /// Example: A name "Motion" would result in a message "Motion detected"
        /// Maximum length of 20 characters
        #[femtopb(string, tag = 5)]
        pub name: &'a str,
        ///
        /// GPIO pin to monitor for state changes
        #[femtopb(uint32, tag = 6)]
        pub monitor_pin: u32,
        ///
        /// Whether or not the GPIO pin state detection is triggered on HIGH (1)
        /// Otherwise LOW (0)
        #[femtopb(bool, tag = 7)]
        pub detection_triggered_high: bool,
        ///
        /// Whether or not use INPUT_PULLUP mode for GPIO pin
        /// Only applicable if the board uses pull-up resistors on the pin
        #[femtopb(bool, tag = 8)]
        pub use_pullup: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Audio Config for codec2 voice
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct AudioConfig<'a> {
        ///
        /// Whether Audio is enabled
        #[femtopb(bool, tag = 1)]
        pub codec2_enabled: bool,
        ///
        /// PTT Pin
        #[femtopb(uint32, tag = 2)]
        pub ptt_pin: u32,
        ///
        /// The audio sample rate to use for codec2
        #[femtopb(enumeration, tag = 3)]
        pub bitrate: ::femtopb::enumeration::EnumValue<audio_config::AudioBaud>,
        ///
        /// I2S Word Select
        #[femtopb(uint32, tag = 4)]
        pub i2s_ws: u32,
        ///
        /// I2S Data IN
        #[femtopb(uint32, tag = 5)]
        pub i2s_sd: u32,
        ///
        /// I2S Data OUT
        #[femtopb(uint32, tag = 6)]
        pub i2s_din: u32,
        ///
        /// I2S Clock
        #[femtopb(uint32, tag = 7)]
        pub i2s_sck: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `AudioConfig`.
    pub mod audio_config {
        ///
        /// Baudrate for codec2 voice
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum AudioBaud {
            #[default]
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
                    Self::Codec2Default => "CODEC2_DEFAULT",
                    Self::Codec23200 => "CODEC2_3200",
                    Self::Codec22400 => "CODEC2_2400",
                    Self::Codec21600 => "CODEC2_1600",
                    Self::Codec21400 => "CODEC2_1400",
                    Self::Codec21300 => "CODEC2_1300",
                    Self::Codec21200 => "CODEC2_1200",
                    Self::Codec2700 => "CODEC2_700",
                    Self::Codec2700b => "CODEC2_700B",
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
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct PaxcounterConfig<'a> {
        ///
        /// Enable the Paxcounter Module
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        #[femtopb(uint32, tag = 2)]
        pub paxcounter_update_interval: u32,
        ///
        /// WiFi RSSI threshold. Defaults to -80
        #[femtopb(int32, tag = 3)]
        pub wifi_threshold: i32,
        ///
        /// BLE RSSI threshold. Defaults to -80
        #[femtopb(int32, tag = 4)]
        pub ble_threshold: i32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Serial Config
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct SerialConfig<'a> {
        ///
        /// Preferences for the SerialModule
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// TODO: REPLACE
        #[femtopb(bool, tag = 2)]
        pub echo: bool,
        ///
        /// RX pin (should match Arduino gpio pin number)
        #[femtopb(uint32, tag = 3)]
        pub rxd: u32,
        ///
        /// TX pin (should match Arduino gpio pin number)
        #[femtopb(uint32, tag = 4)]
        pub txd: u32,
        ///
        /// Serial baud rate
        #[femtopb(enumeration, tag = 5)]
        pub baud: ::femtopb::enumeration::EnumValue<serial_config::SerialBaud>,
        ///
        /// TODO: REPLACE
        #[femtopb(uint32, tag = 6)]
        pub timeout: u32,
        ///
        /// Mode for serial module operation
        #[femtopb(enumeration, tag = 7)]
        pub mode: ::femtopb::enumeration::EnumValue<serial_config::SerialMode>,
        ///
        /// Overrides the platform's defacto Serial port instance to use with Serial module config settings
        /// This is currently only usable in output modes like NMEA / CalTopo and may behave strangely or not work at all in other modes
        /// Existing logging over the Serial Console will still be present
        #[femtopb(bool, tag = 8)]
        pub override_console_serial_port: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `SerialConfig`.
    pub mod serial_config {
        ///
        /// TODO: REPLACE
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum SerialBaud {
            #[default]
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
                    Self::BaudDefault => "BAUD_DEFAULT",
                    Self::Baud110 => "BAUD_110",
                    Self::Baud300 => "BAUD_300",
                    Self::Baud600 => "BAUD_600",
                    Self::Baud1200 => "BAUD_1200",
                    Self::Baud2400 => "BAUD_2400",
                    Self::Baud4800 => "BAUD_4800",
                    Self::Baud9600 => "BAUD_9600",
                    Self::Baud19200 => "BAUD_19200",
                    Self::Baud38400 => "BAUD_38400",
                    Self::Baud57600 => "BAUD_57600",
                    Self::Baud115200 => "BAUD_115200",
                    Self::Baud230400 => "BAUD_230400",
                    Self::Baud460800 => "BAUD_460800",
                    Self::Baud576000 => "BAUD_576000",
                    Self::Baud921600 => "BAUD_921600",
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
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum SerialMode {
            #[default]
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
                    Self::Default => "DEFAULT",
                    Self::Simple => "SIMPLE",
                    Self::Proto => "PROTO",
                    Self::Textmsg => "TEXTMSG",
                    Self::Nmea => "NMEA",
                    Self::Caltopo => "CALTOPO",
                    Self::Ws85 => "WS85",
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
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct ExternalNotificationConfig<'a> {
        ///
        /// Enable the ExternalNotificationModule
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// When using in On/Off mode, keep the output on for this many
        /// milliseconds. Default 1000ms (1 second).
        #[femtopb(uint32, tag = 2)]
        pub output_ms: u32,
        ///
        /// Define the output pin GPIO setting Defaults to
        /// EXT_NOTIFY_OUT if set for the board.
        /// In standalone devices this pin should drive the LED to match the UI.
        #[femtopb(uint32, tag = 3)]
        pub output: u32,
        ///
        /// Optional: Define a secondary output pin for a vibra motor
        /// This is used in standalone devices to match the UI.
        #[femtopb(uint32, tag = 8)]
        pub output_vibra: u32,
        ///
        /// Optional: Define a tertiary output pin for an active buzzer
        /// This is used in standalone devices to to match the UI.
        #[femtopb(uint32, tag = 9)]
        pub output_buzzer: u32,
        ///
        /// IF this is true, the 'output' Pin will be pulled active high, false
        /// means active low.
        #[femtopb(bool, tag = 4)]
        pub active: bool,
        ///
        /// True: Alert when a text message arrives (output)
        #[femtopb(bool, tag = 5)]
        pub alert_message: bool,
        ///
        /// True: Alert when a text message arrives (output_vibra)
        #[femtopb(bool, tag = 10)]
        pub alert_message_vibra: bool,
        ///
        /// True: Alert when a text message arrives (output_buzzer)
        #[femtopb(bool, tag = 11)]
        pub alert_message_buzzer: bool,
        ///
        /// True: Alert when the bell character is received (output)
        #[femtopb(bool, tag = 6)]
        pub alert_bell: bool,
        ///
        /// True: Alert when the bell character is received (output_vibra)
        #[femtopb(bool, tag = 12)]
        pub alert_bell_vibra: bool,
        ///
        /// True: Alert when the bell character is received (output_buzzer)
        #[femtopb(bool, tag = 13)]
        pub alert_bell_buzzer: bool,
        ///
        /// use a PWM output instead of a simple on/off output. This will ignore
        /// the 'output', 'output_ms' and 'active' settings and use the
        /// device.buzzer_gpio instead.
        #[femtopb(bool, tag = 7)]
        pub use_pwm: bool,
        ///
        /// The notification will toggle with 'output_ms' for this time of seconds.
        /// Default is 0 which means don't repeat at all. 60 would mean blink
        /// and/or beep for 60 seconds
        #[femtopb(uint32, tag = 14)]
        pub nag_timeout: u32,
        ///
        /// When true, enables devices with native I2S audio output to use the RTTTL over speaker like a buzzer
        /// T-Watch S3 and T-Deck for example have this capability
        #[femtopb(bool, tag = 15)]
        pub use_i2s_as_buzzer: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Store and Forward Module Config
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct StoreForwardConfig<'a> {
        ///
        /// Enable the Store and Forward Module
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// TODO: REPLACE
        #[femtopb(bool, tag = 2)]
        pub heartbeat: bool,
        ///
        /// TODO: REPLACE
        #[femtopb(uint32, tag = 3)]
        pub records: u32,
        ///
        /// TODO: REPLACE
        #[femtopb(uint32, tag = 4)]
        pub history_return_max: u32,
        ///
        /// TODO: REPLACE
        #[femtopb(uint32, tag = 5)]
        pub history_return_window: u32,
        ///
        /// Set to true to let this node act as a server that stores received messages and resends them upon request.
        #[femtopb(bool, tag = 6)]
        pub is_server: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Preferences for the RangeTestModule
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct RangeTestConfig<'a> {
        ///
        /// Enable the Range Test Module
        #[femtopb(bool, tag = 1)]
        pub enabled: bool,
        ///
        /// Send out range test messages from this node
        #[femtopb(uint32, tag = 2)]
        pub sender: u32,
        ///
        /// Bool value indicating that this node should save a RangeTest.csv file.
        /// ESP32 Only
        #[femtopb(bool, tag = 3)]
        pub save: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// Configuration for both device and environment metrics
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct TelemetryConfig<'a> {
        ///
        /// Interval in seconds of how often we should try to send our
        /// device metrics to the mesh
        #[femtopb(uint32, tag = 1)]
        pub device_update_interval: u32,
        #[femtopb(uint32, tag = 2)]
        pub environment_update_interval: u32,
        ///
        /// Preferences for the Telemetry Module (Environment)
        /// Enable/Disable the telemetry measurement module measurement collection
        #[femtopb(bool, tag = 3)]
        pub environment_measurement_enabled: bool,
        ///
        /// Enable/Disable the telemetry measurement module on-device display
        #[femtopb(bool, tag = 4)]
        pub environment_screen_enabled: bool,
        ///
        /// We'll always read the sensor in Celsius, but sometimes we might want to
        /// display the results in Fahrenheit as a "user preference".
        #[femtopb(bool, tag = 5)]
        pub environment_display_fahrenheit: bool,
        ///
        /// Enable/Disable the air quality metrics
        #[femtopb(bool, tag = 6)]
        pub air_quality_enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// air quality metrics to the mesh
        #[femtopb(uint32, tag = 7)]
        pub air_quality_interval: u32,
        ///
        /// Interval in seconds of how often we should try to send our
        /// air quality metrics to the mesh
        #[femtopb(bool, tag = 8)]
        pub power_measurement_enabled: bool,
        ///
        /// Interval in seconds of how often we should try to send our
        /// air quality metrics to the mesh
        #[femtopb(uint32, tag = 9)]
        pub power_update_interval: u32,
        ///
        /// Interval in seconds of how often we should try to send our
        /// air quality metrics to the mesh
        #[femtopb(bool, tag = 10)]
        pub power_screen_enabled: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// TODO: REPLACE
    #[derive(Clone, PartialEq, ::femtopb::Message)]
    pub struct CannedMessageConfig<'a> {
        ///
        /// Enable the rotary encoder #1. This is a 'dumb' encoder sending pulses on both A and B pins while rotating.
        #[femtopb(bool, tag = 1)]
        pub rotary1_enabled: bool,
        ///
        /// GPIO pin for rotary encoder A port.
        #[femtopb(uint32, tag = 2)]
        pub inputbroker_pin_a: u32,
        ///
        /// GPIO pin for rotary encoder B port.
        #[femtopb(uint32, tag = 3)]
        pub inputbroker_pin_b: u32,
        ///
        /// GPIO pin for rotary encoder Press port.
        #[femtopb(uint32, tag = 4)]
        pub inputbroker_pin_press: u32,
        ///
        /// Generate input event on CW of this kind.
        #[femtopb(enumeration, tag = 5)]
        pub inputbroker_event_cw: ::femtopb::enumeration::EnumValue<
            canned_message_config::InputEventChar,
        >,
        ///
        /// Generate input event on CCW of this kind.
        #[femtopb(enumeration, tag = 6)]
        pub inputbroker_event_ccw: ::femtopb::enumeration::EnumValue<
            canned_message_config::InputEventChar,
        >,
        ///
        /// Generate input event on Press of this kind.
        #[femtopb(enumeration, tag = 7)]
        pub inputbroker_event_press: ::femtopb::enumeration::EnumValue<
            canned_message_config::InputEventChar,
        >,
        ///
        /// Enable the Up/Down/Select input device. Can be RAK rotary encoder or 3 buttons. Uses the a/b/press definitions from inputbroker.
        #[femtopb(bool, tag = 8)]
        pub updown1_enabled: bool,
        ///
        /// Enable/disable CannedMessageModule.
        #[femtopb(bool, tag = 9)]
        pub enabled: bool,
        ///
        /// Input event origin accepted by the canned message module.
        /// Can be e.g. "rotEnc1", "upDownEnc1" or keyword "_any"
        #[femtopb(string, tag = 10)]
        pub allow_input_source: &'a str,
        ///
        /// CannedMessageModule also sends a bell character with the messages.
        /// ExternalNotificationModule can benefit from this feature.
        #[femtopb(bool, tag = 11)]
        pub send_bell: bool,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    /// Nested message and enum types in `CannedMessageConfig`.
    pub mod canned_message_config {
        ///
        /// TODO: REPLACE
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::femtopb::Enumeration
        )]
        #[repr(i32)]
        #[derive(Default)]
        pub enum InputEventChar {
            ///
            /// TODO: REPLACE
            #[default]
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
                    Self::None => "NONE",
                    Self::Up => "UP",
                    Self::Down => "DOWN",
                    Self::Left => "LEFT",
                    Self::Right => "RIGHT",
                    Self::Select => "SELECT",
                    Self::Back => "BACK",
                    Self::Cancel => "CANCEL",
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
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct AmbientLightingConfig<'a> {
        ///
        /// Sets LED to on or off.
        #[femtopb(bool, tag = 1)]
        pub led_state: bool,
        ///
        /// Sets the current for the LED output. Default is 10.
        #[femtopb(uint32, tag = 2)]
        pub current: u32,
        ///
        /// Sets the red LED level. Values are 0-255.
        #[femtopb(uint32, tag = 3)]
        pub red: u32,
        ///
        /// Sets the green LED level. Values are 0-255.
        #[femtopb(uint32, tag = 4)]
        pub green: u32,
        ///
        /// Sets the blue LED level. Values are 0-255.
        #[femtopb(uint32, tag = 5)]
        pub blue: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// TODO: REPLACE
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 1)]
        Mqtt(MqttConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 2)]
        Serial(SerialConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 3)]
        ExternalNotification(ExternalNotificationConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 4)]
        StoreForward(StoreForwardConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 5)]
        RangeTest(RangeTestConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 6)]
        Telemetry(TelemetryConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 7)]
        CannedMessage(CannedMessageConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 8)]
        Audio(AudioConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 9)]
        RemoteHardware(RemoteHardwareConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 10)]
        NeighborInfo(NeighborInfoConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 11)]
        AmbientLighting(AmbientLightingConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 12)]
        DetectionSensor(DetectionSensorConfig<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 13)]
        Paxcounter(PaxcounterConfig<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// A GPIO pin definition for remote hardware module
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct RemoteHardwarePin<'a> {
    ///
    /// GPIO Pin number (must match Arduino)
    #[femtopb(uint32, tag = 1)]
    pub gpio_pin: u32,
    ///
    /// Name for the GPIO pin (i.e. Front gate, mailbox, etc)
    #[femtopb(string, tag = 2)]
    pub name: &'a str,
    ///
    /// Type of GPIO access available to consumers on the mesh
    #[femtopb(enumeration, tag = 3)]
    pub r#type: ::femtopb::enumeration::EnumValue<RemoteHardwarePinType>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum RemoteHardwarePinType {
    ///
    /// Unset/unused
    #[default]
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
            Self::Unknown => "UNKNOWN",
            Self::DigitalRead => "DIGITAL_READ",
            Self::DigitalWrite => "DIGITAL_WRITE",
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
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum PortNum {
    ///
    /// Deprecated: do not use in new code (formerly called OPAQUE)
    /// A message sent from a device outside of the mesh, in a form the mesh does not understand
    /// NOTE: This must be 0, because it is documented in IMeshService.aidl to be so
    /// ENCODING: binary undefined
    #[default]
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
    /// a certain destination would take on the mesh.
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
    /// in your code without needing to rebuild protobuf files (via [regen-protos.sh](<https://github.com/meshtastic/firmware/blob/master/bin/regen-protos.sh>))
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
            Self::UnknownApp => "UNKNOWN_APP",
            Self::TextMessageApp => "TEXT_MESSAGE_APP",
            Self::RemoteHardwareApp => "REMOTE_HARDWARE_APP",
            Self::PositionApp => "POSITION_APP",
            Self::NodeinfoApp => "NODEINFO_APP",
            Self::RoutingApp => "ROUTING_APP",
            Self::AdminApp => "ADMIN_APP",
            Self::TextMessageCompressedApp => "TEXT_MESSAGE_COMPRESSED_APP",
            Self::WaypointApp => "WAYPOINT_APP",
            Self::AudioApp => "AUDIO_APP",
            Self::DetectionSensorApp => "DETECTION_SENSOR_APP",
            Self::ReplyApp => "REPLY_APP",
            Self::IpTunnelApp => "IP_TUNNEL_APP",
            Self::PaxcounterApp => "PAXCOUNTER_APP",
            Self::SerialApp => "SERIAL_APP",
            Self::StoreForwardApp => "STORE_FORWARD_APP",
            Self::RangeTestApp => "RANGE_TEST_APP",
            Self::TelemetryApp => "TELEMETRY_APP",
            Self::ZpsApp => "ZPS_APP",
            Self::SimulatorApp => "SIMULATOR_APP",
            Self::TracerouteApp => "TRACEROUTE_APP",
            Self::NeighborinfoApp => "NEIGHBORINFO_APP",
            Self::AtakPlugin => "ATAK_PLUGIN",
            Self::MapReportApp => "MAP_REPORT_APP",
            Self::PowerstressApp => "POWERSTRESS_APP",
            Self::PrivateApp => "PRIVATE_APP",
            Self::AtakForwarder => "ATAK_FORWARDER",
            Self::Max => "MAX",
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
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct DeviceMetrics<'a> {
    ///
    /// 0-100 (>100 means powered)
    #[femtopb(uint32, tag = 1)]
    pub battery_level: u32,
    ///
    /// Voltage measured
    #[femtopb(float, tag = 2)]
    pub voltage: f32,
    ///
    /// Utilization for the current channel, including well formed TX, RX and malformed RX (aka noise).
    #[femtopb(float, tag = 3)]
    pub channel_utilization: f32,
    ///
    /// Percent of airtime for transmission used within the last hour.
    #[femtopb(float, tag = 4)]
    pub air_util_tx: f32,
    ///
    /// How long the device has been running since the last reboot (in seconds)
    #[femtopb(uint32, tag = 5)]
    pub uptime_seconds: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Weather station or other environmental metrics
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct EnvironmentMetrics<'a> {
    ///
    /// Temperature measured
    #[femtopb(float, tag = 1)]
    pub temperature: f32,
    ///
    /// Relative humidity percent measured
    #[femtopb(float, tag = 2)]
    pub relative_humidity: f32,
    ///
    /// Barometric pressure in hPA measured
    #[femtopb(float, tag = 3)]
    pub barometric_pressure: f32,
    ///
    /// Gas resistance in MOhm measured
    #[femtopb(float, tag = 4)]
    pub gas_resistance: f32,
    ///
    /// Voltage measured (To be depreciated in favor of PowerMetrics in Meshtastic 3.x)
    #[femtopb(float, tag = 5)]
    pub voltage: f32,
    ///
    /// Current measured (To be depreciated in favor of PowerMetrics in Meshtastic 3.x)
    #[femtopb(float, tag = 6)]
    pub current: f32,
    ///
    /// relative scale IAQ value as measured by Bosch BME680 . value 0-500.
    /// Belongs to Air Quality but is not particle but VOC measurement. Other VOC values can also be put in here.
    #[femtopb(uint32, tag = 7)]
    pub iaq: u32,
    ///
    /// RCWL9620 Doppler Radar Distance Sensor, used for water level detection. Float value in mm.
    #[femtopb(float, tag = 8)]
    pub distance: f32,
    ///
    /// VEML7700 high accuracy ambient light(Lux) digital 16-bit resolution sensor.
    #[femtopb(float, tag = 9)]
    pub lux: f32,
    ///
    /// VEML7700 high accuracy white light(irradiance) not calibrated digital 16-bit resolution sensor.
    #[femtopb(float, tag = 10)]
    pub white_lux: f32,
    ///
    /// Infrared lux
    #[femtopb(float, tag = 11)]
    pub ir_lux: f32,
    ///
    /// Ultraviolet lux
    #[femtopb(float, tag = 12)]
    pub uv_lux: f32,
    ///
    /// Wind direction in degrees
    /// 0 degrees = North, 90 = East, etc...
    #[femtopb(uint32, tag = 13)]
    pub wind_direction: u32,
    ///
    /// Wind speed in m/s
    #[femtopb(float, tag = 14)]
    pub wind_speed: f32,
    ///
    /// Weight in KG
    #[femtopb(float, tag = 15)]
    pub weight: f32,
    ///
    /// Wind gust in m/s
    #[femtopb(float, tag = 16)]
    pub wind_gust: f32,
    ///
    /// Wind lull in m/s
    #[femtopb(float, tag = 17)]
    pub wind_lull: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Power Metrics (voltage / current / etc)
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PowerMetrics<'a> {
    ///
    /// Voltage (Ch1)
    #[femtopb(float, tag = 1)]
    pub ch1_voltage: f32,
    ///
    /// Current (Ch1)
    #[femtopb(float, tag = 2)]
    pub ch1_current: f32,
    ///
    /// Voltage (Ch2)
    #[femtopb(float, tag = 3)]
    pub ch2_voltage: f32,
    ///
    /// Current (Ch2)
    #[femtopb(float, tag = 4)]
    pub ch2_current: f32,
    ///
    /// Voltage (Ch3)
    #[femtopb(float, tag = 5)]
    pub ch3_voltage: f32,
    ///
    /// Current (Ch3)
    #[femtopb(float, tag = 6)]
    pub ch3_current: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Air quality metrics
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct AirQualityMetrics<'a> {
    ///
    /// Concentration Units Standard PM1.0
    #[femtopb(uint32, tag = 1)]
    pub pm10_standard: u32,
    ///
    /// Concentration Units Standard PM2.5
    #[femtopb(uint32, tag = 2)]
    pub pm25_standard: u32,
    ///
    /// Concentration Units Standard PM10.0
    #[femtopb(uint32, tag = 3)]
    pub pm100_standard: u32,
    ///
    /// Concentration Units Environmental PM1.0
    #[femtopb(uint32, tag = 4)]
    pub pm10_environmental: u32,
    ///
    /// Concentration Units Environmental PM2.5
    #[femtopb(uint32, tag = 5)]
    pub pm25_environmental: u32,
    ///
    /// Concentration Units Environmental PM10.0
    #[femtopb(uint32, tag = 6)]
    pub pm100_environmental: u32,
    ///
    /// 0.3um Particle Count
    #[femtopb(uint32, tag = 7)]
    pub particles_03um: u32,
    ///
    /// 0.5um Particle Count
    #[femtopb(uint32, tag = 8)]
    pub particles_05um: u32,
    ///
    /// 1.0um Particle Count
    #[femtopb(uint32, tag = 9)]
    pub particles_10um: u32,
    ///
    /// 2.5um Particle Count
    #[femtopb(uint32, tag = 10)]
    pub particles_25um: u32,
    ///
    /// 5.0um Particle Count
    #[femtopb(uint32, tag = 11)]
    pub particles_50um: u32,
    ///
    /// 10.0um Particle Count
    #[femtopb(uint32, tag = 12)]
    pub particles_100um: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Types of Measurements the telemetry module is equipped to handle
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Telemetry<'a> {
    ///
    /// Seconds since 1970 - or 0 for unknown/unset
    #[femtopb(fixed32, tag = 1)]
    pub time: u32,
    #[femtopb(oneof, tags = [2, 3, 4, 5])]
    pub variant: ::core::option::Option<telemetry::Variant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `Telemetry`.
pub mod telemetry {
    #[derive(Clone, Copy, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Variant<'a> {
        ///
        /// Key native device metrics such as battery level
        #[femtopb(message, tag = 2)]
        DeviceMetrics(super::DeviceMetrics<'a>),
        ///
        /// Weather station or other environmental metrics
        #[femtopb(message, tag = 3)]
        EnvironmentMetrics(super::EnvironmentMetrics<'a>),
        ///
        /// Air quality metrics
        #[femtopb(message, tag = 4)]
        AirQualityMetrics(super::AirQualityMetrics<'a>),
        ///
        /// Power Metrics
        #[femtopb(message, tag = 5)]
        PowerMetrics(super::PowerMetrics<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// NAU7802 Telemetry configuration, for saving to flash
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Nau7802Config<'a> {
    ///
    /// The offset setting for the NAU7802
    #[femtopb(int32, tag = 1)]
    pub zero_offset: i32,
    ///
    /// The calibration factor for the NAU7802
    #[femtopb(float, tag = 2)]
    pub calibration_factor: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Supported I2C Sensors for telemetry in Meshtastic
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum TelemetrySensorType {
    ///
    /// No external telemetry sensor explicitly set
    #[default]
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
}
impl TelemetrySensorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SensorUnset => "SENSOR_UNSET",
            Self::Bme280 => "BME280",
            Self::Bme680 => "BME680",
            Self::Mcp9808 => "MCP9808",
            Self::Ina260 => "INA260",
            Self::Ina219 => "INA219",
            Self::Bmp280 => "BMP280",
            Self::Shtc3 => "SHTC3",
            Self::Lps22 => "LPS22",
            Self::Qmc6310 => "QMC6310",
            Self::Qmi8658 => "QMI8658",
            Self::Qmc5883l => "QMC5883L",
            Self::Sht31 => "SHT31",
            Self::Pmsa003i => "PMSA003I",
            Self::Ina3221 => "INA3221",
            Self::Bmp085 => "BMP085",
            Self::Rcwl9620 => "RCWL9620",
            Self::Sht4x => "SHT4X",
            Self::Veml7700 => "VEML7700",
            Self::Mlx90632 => "MLX90632",
            Self::Opt3001 => "OPT3001",
            Self::Ltr390uv => "LTR390UV",
            Self::Tsl25911fn => "TSL25911FN",
            Self::Aht10 => "AHT10",
            Self::DfrobotLark => "DFROBOT_LARK",
            Self::Nau7802 => "NAU7802",
            Self::Bmp3xx => "BMP3XX",
            Self::Icm20948 => "ICM20948",
            Self::Max17048 => "MAX17048",
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
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct XModem<'a> {
    #[femtopb(enumeration, tag = 1)]
    pub control: ::femtopb::enumeration::EnumValue<x_modem::Control>,
    #[femtopb(uint32, tag = 2)]
    pub seq: u32,
    #[femtopb(uint32, tag = 3)]
    pub crc16: u32,
    #[femtopb(bytes, tag = 4)]
    pub buffer: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `XModem`.
pub mod x_modem {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Control {
        #[default]
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
                Self::Nul => "NUL",
                Self::Soh => "SOH",
                Self::Stx => "STX",
                Self::Eot => "EOT",
                Self::Ack => "ACK",
                Self::Nak => "NAK",
                Self::Can => "CAN",
                Self::Ctrlz => "CTRLZ",
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
/// a gps position
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Position<'a> {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[femtopb(sfixed32, tag = 1)]
    pub latitude_i: i32,
    ///
    /// TODO: REPLACE
    #[femtopb(sfixed32, tag = 2)]
    pub longitude_i: i32,
    ///
    /// In meters above MSL (but see issue #359)
    #[femtopb(int32, tag = 3)]
    pub altitude: i32,
    ///
    /// This is usually not sent over the mesh (to save space), but it is sent
    /// from the phone so that the local device can set its time if it is sent over
    /// the mesh (because there are devices on the mesh without GPS or RTC).
    /// seconds since 1970
    #[femtopb(fixed32, tag = 4)]
    pub time: u32,
    ///
    /// TODO: REPLACE
    #[femtopb(enumeration, tag = 5)]
    pub location_source: ::femtopb::enumeration::EnumValue<position::LocSource>,
    ///
    /// TODO: REPLACE
    #[femtopb(enumeration, tag = 6)]
    pub altitude_source: ::femtopb::enumeration::EnumValue<position::AltSource>,
    ///
    /// Positional timestamp (actual timestamp of GPS solution) in integer epoch seconds
    #[femtopb(fixed32, tag = 7)]
    pub timestamp: u32,
    ///
    /// Pos. timestamp milliseconds adjustment (rarely available or required)
    #[femtopb(int32, tag = 8)]
    pub timestamp_millis_adjust: i32,
    ///
    /// HAE altitude in meters - can be used instead of MSL altitude
    #[femtopb(sint32, tag = 9)]
    pub altitude_hae: i32,
    ///
    /// Geoidal separation in meters
    #[femtopb(sint32, tag = 10)]
    pub altitude_geoidal_separation: i32,
    ///
    /// Horizontal, Vertical and Position Dilution of Precision, in 1/100 units
    /// - PDOP is sufficient for most cases
    /// - for higher precision scenarios, HDOP and VDOP can be used instead,
    ///    in which case PDOP becomes redundant (PDOP=sqrt(HDOP^2 + VDOP^2))
    /// TODO: REMOVE/INTEGRATE
    #[femtopb(uint32, tag = 11)]
    pub pdop: u32,
    ///
    /// TODO: REPLACE
    #[femtopb(uint32, tag = 12)]
    pub hdop: u32,
    ///
    /// TODO: REPLACE
    #[femtopb(uint32, tag = 13)]
    pub vdop: u32,
    ///
    /// GPS accuracy (a hardware specific constant) in mm
    ///    multiplied with DOP to calculate positional accuracy
    /// Default: "'bout three meters-ish" :)
    #[femtopb(uint32, tag = 14)]
    pub gps_accuracy: u32,
    ///
    /// Ground speed in m/s and True North TRACK in 1/100 degrees
    /// Clarification of terms:
    /// - "track" is the direction of motion (measured in horizontal plane)
    /// - "heading" is where the fuselage points (measured in horizontal plane)
    /// - "yaw" indicates a relative rotation about the vertical axis
    /// TODO: REMOVE/INTEGRATE
    #[femtopb(uint32, tag = 15)]
    pub ground_speed: u32,
    ///
    /// TODO: REPLACE
    #[femtopb(uint32, tag = 16)]
    pub ground_track: u32,
    ///
    /// GPS fix quality (from NMEA GxGGA statement or similar)
    #[femtopb(uint32, tag = 17)]
    pub fix_quality: u32,
    ///
    /// GPS fix type 2D/3D (from NMEA GxGSA statement)
    #[femtopb(uint32, tag = 18)]
    pub fix_type: u32,
    ///
    /// GPS "Satellites in View" number
    #[femtopb(uint32, tag = 19)]
    pub sats_in_view: u32,
    ///
    /// Sensor ID - in case multiple positioning sensors are being used
    #[femtopb(uint32, tag = 20)]
    pub sensor_id: u32,
    ///
    /// Estimated/expected time (in seconds) until next update:
    /// - if we update at fixed intervals of X seconds, use X
    /// - if we update at dynamic intervals (based on relative movement etc),
    ///    but "AT LEAST every Y seconds", use Y
    #[femtopb(uint32, tag = 21)]
    pub next_update: u32,
    ///
    /// A sequence number, incremented with each Position message to help
    ///    detect lost updates if needed
    #[femtopb(uint32, tag = 22)]
    pub seq_number: u32,
    ///
    /// Indicates the bits of precision set by the sending node
    #[femtopb(uint32, tag = 23)]
    pub precision_bits: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `Position`.
pub mod position {
    ///
    /// How the location was acquired: manual, onboard GPS, external (EUD) GPS
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum LocSource {
        ///
        /// TODO: REPLACE
        #[default]
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
                Self::LocUnset => "LOC_UNSET",
                Self::LocManual => "LOC_MANUAL",
                Self::LocInternal => "LOC_INTERNAL",
                Self::LocExternal => "LOC_EXTERNAL",
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
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum AltSource {
        ///
        /// TODO: REPLACE
        #[default]
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
                Self::AltUnset => "ALT_UNSET",
                Self::AltManual => "ALT_MANUAL",
                Self::AltInternal => "ALT_INTERNAL",
                Self::AltExternal => "ALT_EXTERNAL",
                Self::AltBarometric => "ALT_BAROMETRIC",
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
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct User<'a> {
    ///
    /// A globally unique ID string for this user.
    /// In the case of Signal that would mean +16504442323, for the default macaddr derived id it would be !<8 hexidecimal bytes>.
    /// Note: app developers are encouraged to also use the following standard
    /// node IDs "^all" (for broadcast), "^local" (for the locally connected node)
    #[femtopb(string, tag = 1)]
    pub id: &'a str,
    ///
    /// A full name for this user, i.e. "Kevin Hester"
    #[femtopb(string, tag = 2)]
    pub long_name: &'a str,
    ///
    /// A VERY short name, ideally two characters.
    /// Suitable for a tiny OLED screen
    #[femtopb(string, tag = 3)]
    pub short_name: &'a str,
    ///
    /// Deprecated in Meshtastic 2.1.x
    /// This is the addr of the radio.
    /// Not populated by the phone, but added by the esp32 when broadcasting
    #[deprecated]
    #[femtopb(bytes, tag = 4)]
    pub macaddr: &'a [u8],
    ///
    /// TBEAM, HELTEC, etc...
    /// Starting in 1.2.11 moved to hw_model enum in the NodeInfo object.
    /// Apps will still need the string here for older builds
    /// (so OTA update can find the right image), but if the enum is available it will be used instead.
    #[femtopb(enumeration, tag = 5)]
    pub hw_model: ::femtopb::enumeration::EnumValue<HardwareModel>,
    ///
    /// In some regions Ham radio operators have different bandwidth limitations than others.
    /// If this user is a licensed operator, set this flag.
    /// Also, "long_name" should be their licence number.
    #[femtopb(bool, tag = 6)]
    pub is_licensed: bool,
    ///
    /// Indicates that the user's role in the mesh
    #[femtopb(enumeration, tag = 7)]
    pub role: ::femtopb::enumeration::EnumValue<config::device_config::Role>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// A message used in our Dynamic Source Routing protocol (RFC 4728 based)
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct RouteDiscovery<'a> {
    ///
    /// The list of nodenums this packet has visited so far
    #[femtopb(fixed32, packed, tag = 1)]
    pub route: ::femtopb::packed::Packed<'a, u32, ::femtopb::item_encoding::Fixed32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// A Routing control Data packet handled by the routing module
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Routing<'a> {
    #[femtopb(oneof, tags = [1, 2, 3])]
    pub variant: ::core::option::Option<routing::Variant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `Routing`.
pub mod routing {
    ///
    /// A failure in delivering a message (usually used for routing control messages, but might be provided in addition to ack.fail_id to provide
    /// details on the type of failure).
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Error {
        ///
        /// This message is not a failure
        #[default]
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
    }
    impl Error {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::None => "NONE",
                Self::NoRoute => "NO_ROUTE",
                Self::GotNak => "GOT_NAK",
                Self::Timeout => "TIMEOUT",
                Self::NoInterface => "NO_INTERFACE",
                Self::MaxRetransmit => "MAX_RETRANSMIT",
                Self::NoChannel => "NO_CHANNEL",
                Self::TooLarge => "TOO_LARGE",
                Self::NoResponse => "NO_RESPONSE",
                Self::DutyCycleLimit => "DUTY_CYCLE_LIMIT",
                Self::BadRequest => "BAD_REQUEST",
                Self::NotAuthorized => "NOT_AUTHORIZED",
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
                _ => None,
            }
        }
    }
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Variant<'a> {
        ///
        /// A route request going from the requester
        #[femtopb(message, tag = 1)]
        RouteRequest(super::RouteDiscovery<'a>),
        ///
        /// A route reply
        #[femtopb(message, tag = 2)]
        RouteReply(super::RouteDiscovery<'a>),
        ///
        /// A failure in delivering a message (usually used for routing control messages, but might be provided
        /// in addition to ack.fail_id to provide details on the type of failure).
        #[femtopb(enumeration, tag = 3)]
        ErrorReason(::femtopb::enumeration::EnumValue<Error>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// (Formerly called SubPacket)
/// The payload portion fo a packet, this is the actual bytes that are sent
/// inside a radio packet (because from/to are broken out by the comms library)
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Data<'a> {
    ///
    /// Formerly named typ and of type Type
    #[femtopb(enumeration, tag = 1)]
    pub portnum: ::femtopb::enumeration::EnumValue<PortNum>,
    ///
    /// TODO: REPLACE
    #[femtopb(bytes, tag = 2)]
    pub payload: &'a [u8],
    ///
    /// Not normally used, but for testing a sender can request that recipient
    /// responds in kind (i.e. if it received a position, it should unicast back it's position).
    /// Note: that if you set this on a broadcast you will receive many replies.
    #[femtopb(bool, tag = 3)]
    pub want_response: bool,
    ///
    /// The address of the destination node.
    /// This field is is filled in by the mesh radio device software, application
    /// layer software should never need it.
    /// RouteDiscovery messages _must_ populate this.
    /// Other message types might need to if they are doing multihop routing.
    #[femtopb(fixed32, tag = 4)]
    pub dest: u32,
    ///
    /// The address of the original sender for this message.
    /// This field should _only_ be populated for reliable multihop packets (to keep
    /// packets small).
    #[femtopb(fixed32, tag = 5)]
    pub source: u32,
    ///
    /// Only used in routing or response messages.
    /// Indicates the original message ID that this message is reporting failure on. (formerly called original_id)
    #[femtopb(fixed32, tag = 6)]
    pub request_id: u32,
    ///
    /// If set, this message is intened to be a reply to a previously sent message with the defined id.
    #[femtopb(fixed32, tag = 7)]
    pub reply_id: u32,
    ///
    /// Defaults to false. If true, then what is in the payload should be treated as an emoji like giving
    /// a message a heart or poop emoji.
    #[femtopb(fixed32, tag = 8)]
    pub emoji: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Waypoint message, used to share arbitrary locations across the mesh
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Waypoint<'a> {
    ///
    /// Id of the waypoint
    #[femtopb(uint32, tag = 1)]
    pub id: u32,
    ///
    /// latitude_i
    #[femtopb(sfixed32, tag = 2)]
    pub latitude_i: i32,
    ///
    /// longitude_i
    #[femtopb(sfixed32, tag = 3)]
    pub longitude_i: i32,
    ///
    /// Time the waypoint is to expire (epoch)
    #[femtopb(uint32, tag = 4)]
    pub expire: u32,
    ///
    /// If greater than zero, treat the value as a nodenum only allowing them to update the waypoint.
    /// If zero, the waypoint is open to be edited by any member of the mesh.
    #[femtopb(uint32, tag = 5)]
    pub locked_to: u32,
    ///
    /// Name of the waypoint - max 30 chars
    #[femtopb(string, tag = 6)]
    pub name: &'a str,
    ///
    /// Description of the waypoint - max 100 chars
    #[femtopb(string, tag = 7)]
    pub description: &'a str,
    ///
    /// Designator icon for the waypoint in the form of a unicode emoji
    #[femtopb(fixed32, tag = 8)]
    pub icon: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This message will be proxied over the PhoneAPI for the client to deliver to the MQTT server
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MqttClientProxyMessage<'a> {
    ///
    /// The MQTT topic this message will be sent /received on
    #[femtopb(string, tag = 1)]
    pub topic: &'a str,
    ///
    /// Whether the message should be retained (or not)
    #[femtopb(bool, tag = 4)]
    pub retained: bool,
    ///
    /// The actual service envelope payload or text for mqtt pub / sub
    #[femtopb(oneof, tags = [2, 3])]
    pub payload_variant: ::core::option::Option<
        mqtt_client_proxy_message::PayloadVariant<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `MqttClientProxyMessage`.
pub mod mqtt_client_proxy_message {
    ///
    /// The actual service envelope payload or text for mqtt pub / sub
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// Bytes
        #[femtopb(bytes, tag = 2)]
        Data(&'a [u8]),
        ///
        /// Text
        #[femtopb(string, tag = 3)]
        Text(&'a str),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// A packet envelope sent/received over the mesh
/// only payload_variant is sent in the payload portion of the LORA packet.
/// The other fields are either not sent at all, or sent in the special 16 byte LORA header.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MeshPacket<'a> {
    ///
    /// The sending node number.
    /// Note: Our crypto implementation uses this field as well.
    /// See [crypto](/docs/overview/encryption) for details.
    #[femtopb(fixed32, tag = 1)]
    pub from: u32,
    ///
    /// The (immediate) destination for this packet
    #[femtopb(fixed32, tag = 2)]
    pub to: u32,
    ///
    /// (Usually) If set, this indicates the index in the secondary_channels table that this packet was sent/received on.
    /// If unset, packet was on the primary channel.
    /// A particular node might know only a subset of channels in use on the mesh.
    /// Therefore channel_index is inherently a local concept and meaningless to send between nodes.
    /// Very briefly, while sending and receiving deep inside the device Router code, this field instead
    /// contains the 'channel hash' instead of the index.
    /// This 'trick' is only used while the payload_variant is an 'encrypted'.
    #[femtopb(uint32, tag = 3)]
    pub channel: u32,
    ///
    /// A unique ID for this packet.
    /// Always 0 for no-ack packets or non broadcast packets (and therefore take zero bytes of space).
    /// Otherwise a unique ID for this packet, useful for flooding algorithms.
    /// ID only needs to be unique on a _per sender_ basis, and it only
    /// needs to be unique for a few minutes (long enough to last for the length of
    /// any ACK or the completion of a mesh broadcast flood).
    /// Note: Our crypto implementation uses this id as well.
    /// See [crypto](/docs/overview/encryption) for details.
    #[femtopb(fixed32, tag = 6)]
    pub id: u32,
    ///
    /// The time this message was received by the esp32 (secs since 1970).
    /// Note: this field is _never_ sent on the radio link itself (to save space) Times
    /// are typically not sent over the mesh, but they will be added to any Packet
    /// (chain of SubPacket) sent to the phone (so the phone can know exact time of reception)
    #[femtopb(fixed32, tag = 7)]
    pub rx_time: u32,
    ///
    /// *Never* sent over the radio links.
    /// Set during reception to indicate the SNR of this packet.
    /// Used to collect statistics on current link quality.
    #[femtopb(float, tag = 8)]
    pub rx_snr: f32,
    ///
    /// If unset treated as zero (no forwarding, send to adjacent nodes only)
    /// if 1, allow hopping through one node, etc...
    /// For our usecase real world topologies probably have a max of about 3.
    /// This field is normally placed into a few of bits in the header.
    #[femtopb(uint32, tag = 9)]
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
    #[femtopb(bool, tag = 10)]
    pub want_ack: bool,
    ///
    /// The priority of this message for sending.
    /// See MeshPacket.Priority description for more details.
    #[femtopb(enumeration, tag = 11)]
    pub priority: ::femtopb::enumeration::EnumValue<mesh_packet::Priority>,
    ///
    /// rssi of received packet. Only sent to phone for dispay purposes.
    #[femtopb(int32, tag = 12)]
    pub rx_rssi: i32,
    ///
    /// Describe if this message is delayed
    #[deprecated]
    #[femtopb(enumeration, tag = 13)]
    pub delayed: ::femtopb::enumeration::EnumValue<mesh_packet::Delayed>,
    ///
    /// Describes whether this packet passed via MQTT somewhere along the path it currently took.
    #[femtopb(bool, tag = 14)]
    pub via_mqtt: bool,
    ///
    /// Hop limit with which the original packet started. Sent via LoRa using three bits in the unencrypted header.
    /// When receiving a packet, the difference between hop_start and hop_limit gives how many hops it traveled.
    #[femtopb(uint32, tag = 15)]
    pub hop_start: u32,
    #[femtopb(oneof, tags = [4, 5])]
    pub payload_variant: ::core::option::Option<mesh_packet::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
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
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Priority {
        ///
        /// Treated as Priority.DEFAULT
        #[default]
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
                Self::Unset => "UNSET",
                Self::Min => "MIN",
                Self::Background => "BACKGROUND",
                Self::Default => "DEFAULT",
                Self::Reliable => "RELIABLE",
                Self::Ack => "ACK",
                Self::Max => "MAX",
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
                "ACK" => Some(Self::Ack),
                "MAX" => Some(Self::Max),
                _ => None,
            }
        }
    }
    ///
    /// Identify if this is a delayed packet
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Delayed {
        ///
        /// If unset, the message is being sent in real time.
        #[default]
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
                Self::NoDelay => "NO_DELAY",
                Self::Broadcast => "DELAYED_BROADCAST",
                Self::Direct => "DELAYED_DIRECT",
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
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 4)]
        Decoded(super::Data<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(bytes, tag = 5)]
        Encrypted(&'a [u8]),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
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
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NodeInfo<'a> {
    ///
    /// The node number
    #[femtopb(uint32, tag = 1)]
    pub num: u32,
    ///
    /// The user info for this node
    #[femtopb(message, optional, tag = 2)]
    pub user: ::core::option::Option<User<'a>>,
    ///
    /// This position data. Note: before 1.2.14 we would also store the last time we've heard from this node in position.time, that is no longer true.
    /// Position.time now indicates the last time we received a POSITION from that node.
    #[femtopb(message, optional, tag = 3)]
    pub position: ::core::option::Option<Position<'a>>,
    ///
    /// Returns the Signal-to-noise ratio (SNR) of the last received message,
    /// as measured by the receiver. Return SNR of the last received message in dB
    #[femtopb(float, tag = 4)]
    pub snr: f32,
    ///
    /// Set to indicate the last time we received a packet from this node
    #[femtopb(fixed32, tag = 5)]
    pub last_heard: u32,
    ///
    /// The latest device metrics for the node.
    #[femtopb(message, optional, tag = 6)]
    pub device_metrics: ::core::option::Option<DeviceMetrics<'a>>,
    ///
    /// local channel index we heard that node on. Only populated if its not the default channel.
    #[femtopb(uint32, tag = 7)]
    pub channel: u32,
    ///
    /// True if we witnessed the node over MQTT instead of LoRA transport
    #[femtopb(bool, tag = 8)]
    pub via_mqtt: bool,
    ///
    /// Number of hops away from us this node is (0 if adjacent)
    #[femtopb(uint32, tag = 9)]
    pub hops_away: u32,
    ///
    /// True if node is in our favorites list
    /// Persists between NodeDB internal clean ups
    #[femtopb(bool, tag = 10)]
    pub is_favorite: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Unique local debugging info for this node
/// Note: we don't include position or the user info, because that will come in the
/// Sent to the phone in response to WantNodes.
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct MyNodeInfo<'a> {
    ///
    /// Tells the phone what our node number is, default starting value is
    /// lowbyte of macaddr, but it will be fixed if that is already in use
    #[femtopb(uint32, tag = 1)]
    pub my_node_num: u32,
    ///
    /// The total number of reboots this node has ever encountered
    /// (well - since the last time we discarded preferences)
    #[femtopb(uint32, tag = 8)]
    pub reboot_count: u32,
    ///
    /// The minimum app version that can talk to this device.
    /// Phone/PC apps should compare this to their build number and if too low tell the user they must update their app
    #[femtopb(uint32, tag = 11)]
    pub min_app_version: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Debug output from the device.
/// To minimize the size of records inside the device code, if a time/source/level is not set
/// on the message it is assumed to be a continuation of the previously sent message.
/// This allows the device code to use fixed maxlen 64 byte strings for messages,
/// and then extend as needed by emitting multiple records.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LogRecord<'a> {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[femtopb(string, tag = 1)]
    pub message: &'a str,
    ///
    /// Seconds since 1970 - or 0 for unknown/unset
    #[femtopb(fixed32, tag = 2)]
    pub time: u32,
    ///
    /// Usually based on thread name - if known
    #[femtopb(string, tag = 3)]
    pub source: &'a str,
    ///
    /// Not yet set
    #[femtopb(enumeration, tag = 4)]
    pub level: ::femtopb::enumeration::EnumValue<log_record::Level>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `LogRecord`.
pub mod log_record {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Level {
        ///
        /// Log levels, chosen to match python logging conventions.
        #[default]
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
                Self::Unset => "UNSET",
                Self::Critical => "CRITICAL",
                Self::Error => "ERROR",
                Self::Warning => "WARNING",
                Self::Info => "INFO",
                Self::Debug => "DEBUG",
                Self::Trace => "TRACE",
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
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct QueueStatus<'a> {
    /// Last attempt to queue status, ErrorCode
    #[femtopb(int32, tag = 1)]
    pub res: i32,
    /// Free entries in the outgoing queue
    #[femtopb(uint32, tag = 2)]
    pub free: u32,
    /// Maximum entries in the outgoing queue
    #[femtopb(uint32, tag = 3)]
    pub maxlen: u32,
    /// What was mesh packet id that generated this response?
    #[femtopb(uint32, tag = 4)]
    pub mesh_packet_id: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Packets from the radio to the phone will appear on the fromRadio characteristic.
/// It will support READ and NOTIFY. When a new packet arrives the device will BLE notify?
/// It will sit in that descriptor until consumed by the phone,
/// at which point the next item in the FIFO will be populated.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FromRadio<'a> {
    ///
    /// The packet id, used to allow the phone to request missing read packets from the FIFO,
    /// see our bluetooth docs
    #[femtopb(uint32, tag = 1)]
    pub id: u32,
    ///
    /// Log levels, chosen to match python logging conventions.
    #[femtopb(oneof, tags = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])]
    pub payload_variant: ::core::option::Option<from_radio::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `FromRadio`.
pub mod from_radio {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// Log levels, chosen to match python logging conventions.
        #[femtopb(message, tag = 2)]
        Packet(super::MeshPacket<'a>),
        ///
        /// Tells the phone what our node number is, can be -1 if we've not yet joined a mesh.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[femtopb(message, tag = 3)]
        MyInfo(super::MyNodeInfo<'a>),
        ///
        /// One packet is sent for each node in the on radio DB
        /// starts over with the first node in our DB
        #[femtopb(message, tag = 4)]
        NodeInfo(super::NodeInfo<'a>),
        ///
        /// Include a part of the config (was: RadioConfig radio)
        #[femtopb(message, tag = 5)]
        Config(super::Config<'a>),
        ///
        /// Set to send debug console output over our protobuf stream
        #[femtopb(message, tag = 6)]
        LogRecord(super::LogRecord<'a>),
        ///
        /// Sent as true once the device has finished sending all of the responses to want_config
        /// recipient should check if this ID matches our original request nonce, if
        /// not, it means your config responses haven't started yet.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[femtopb(uint32, tag = 7)]
        ConfigCompleteId(u32),
        ///
        /// Sent to tell clients the radio has just rebooted.
        /// Set to true if present.
        /// Not used on all transports, currently just used for the serial console.
        /// NOTE: This ID must not change - to keep (minimal) compatibility with <1.2 version of android apps.
        #[femtopb(bool, tag = 8)]
        Rebooted(bool),
        ///
        /// Include module config
        #[femtopb(message, tag = 9)]
        ModuleConfig(super::ModuleConfig<'a>),
        ///
        /// One packet is sent for each channel
        #[femtopb(message, tag = 10)]
        Channel(super::Channel<'a>),
        ///
        /// Queue status info
        #[femtopb(message, tag = 11)]
        QueueStatus(super::QueueStatus<'a>),
        ///
        /// File Transfer Chunk
        #[femtopb(message, tag = 12)]
        XmodemPacket(super::XModem<'a>),
        ///
        /// Device metadata message
        #[femtopb(message, tag = 13)]
        Metadata(super::DeviceMetadata<'a>),
        ///
        /// MQTT Client Proxy Message (device sending to client / phone for publishing to MQTT)
        #[femtopb(message, tag = 14)]
        MqttClientProxyMessage(super::MqttClientProxyMessage<'a>),
        ///
        /// File system manifest messages
        #[femtopb(message, tag = 15)]
        FileInfo(super::FileInfo<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// Individual File info for the device
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct FileInfo<'a> {
    ///
    /// The fully qualified path of the file
    #[femtopb(string, tag = 1)]
    pub file_name: &'a str,
    ///
    /// The size of the file in bytes
    #[femtopb(uint32, tag = 2)]
    pub size_bytes: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Packets/commands to the radio will be written (reliably) to the toRadio characteristic.
/// Once the write completes the phone can assume it is handled.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ToRadio<'a> {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[femtopb(oneof, tags = [1, 3, 4, 5, 6, 7])]
    pub payload_variant: ::core::option::Option<to_radio::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `ToRadio`.
pub mod to_radio {
    ///
    /// Log levels, chosen to match python logging conventions.
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// Send this packet on the mesh
        #[femtopb(message, tag = 1)]
        Packet(super::MeshPacket<'a>),
        ///
        /// Phone wants radio to send full node db to the phone, This is
        /// typically the first packet sent to the radio when the phone gets a
        /// bluetooth connection. The radio will respond by sending back a
        /// MyNodeInfo, a owner, a radio config and a series of
        /// FromRadio.node_infos, and config_complete
        /// the integer you write into this field will be reported back in the
        /// config_complete_id response this allows clients to never be confused by
        /// a stale old partially sent config.
        #[femtopb(uint32, tag = 3)]
        WantConfigId(u32),
        ///
        /// Tell API server we are disconnecting now.
        /// This is useful for serial links where there is no hardware/protocol based notification that the client has dropped the link.
        /// (Sending this message is optional for clients)
        #[femtopb(bool, tag = 4)]
        Disconnect(bool),
        #[femtopb(message, tag = 5)]
        XmodemPacket(super::XModem<'a>),
        ///
        /// MQTT Client Proxy Message (for client / phone subscribed to MQTT sending to device)
        #[femtopb(message, tag = 6)]
        MqttClientProxyMessage(super::MqttClientProxyMessage<'a>),
        ///
        /// Heartbeat message (used to keep the device connection awake on serial)
        #[femtopb(message, tag = 7)]
        Heartbeat(super::Heartbeat<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// Compressed message payload
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Compressed<'a> {
    ///
    /// PortNum to determine the how to handle the compressed payload.
    #[femtopb(enumeration, tag = 1)]
    pub portnum: ::femtopb::enumeration::EnumValue<PortNum>,
    ///
    /// Compressed data.
    #[femtopb(bytes, tag = 2)]
    pub data: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Full info on edges for a single node
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NeighborInfo<'a> {
    ///
    /// The node ID of the node sending info on its neighbors
    #[femtopb(uint32, tag = 1)]
    pub node_id: u32,
    ///
    /// Field to pass neighbor info for the next sending cycle
    #[femtopb(uint32, tag = 2)]
    pub last_sent_by_id: u32,
    ///
    /// Broadcast interval of the represented node (in seconds)
    #[femtopb(uint32, tag = 3)]
    pub node_broadcast_interval_secs: u32,
    ///
    /// The list of out edges from this node
    #[femtopb(message, repeated, tag = 4)]
    pub neighbors: ::femtopb::repeated::Repeated<
        'a,
        Neighbor<'a>,
        ::femtopb::item_encoding::Message<'a, Neighbor<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// A single edge in the mesh
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Neighbor<'a> {
    ///
    /// Node ID of neighbor
    #[femtopb(uint32, tag = 1)]
    pub node_id: u32,
    ///
    /// SNR of last heard message
    #[femtopb(float, tag = 2)]
    pub snr: f32,
    ///
    /// Reception time (in secs since 1970) of last message that was last sent by this ID.
    /// Note: this is for local storage only and will not be sent out over the mesh.
    #[femtopb(fixed32, tag = 3)]
    pub last_rx_time: u32,
    ///
    /// Broadcast interval of this neighbor (in seconds).
    /// Note: this is for local storage only and will not be sent out over the mesh.
    #[femtopb(uint32, tag = 4)]
    pub node_broadcast_interval_secs: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Device metadata response
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct DeviceMetadata<'a> {
    ///
    /// Device firmware version string
    #[femtopb(string, tag = 1)]
    pub firmware_version: &'a str,
    ///
    /// Device state version
    #[femtopb(uint32, tag = 2)]
    pub device_state_version: u32,
    ///
    /// Indicates whether the device can shutdown CPU natively or via power management chip
    #[femtopb(bool, tag = 3)]
    pub can_shutdown: bool,
    ///
    /// Indicates that the device has native wifi capability
    #[femtopb(bool, tag = 4)]
    pub has_wifi: bool,
    ///
    /// Indicates that the device has native bluetooth capability
    #[femtopb(bool, tag = 5)]
    pub has_bluetooth: bool,
    ///
    /// Indicates that the device has an ethernet peripheral
    #[femtopb(bool, tag = 6)]
    pub has_ethernet: bool,
    ///
    /// Indicates that the device's role in the mesh
    #[femtopb(enumeration, tag = 7)]
    pub role: ::femtopb::enumeration::EnumValue<config::device_config::Role>,
    ///
    /// Indicates the device's current enabled position flags
    #[femtopb(uint32, tag = 8)]
    pub position_flags: u32,
    ///
    /// Device hardware model
    #[femtopb(enumeration, tag = 9)]
    pub hw_model: ::femtopb::enumeration::EnumValue<HardwareModel>,
    ///
    /// Has Remote Hardware enabled
    #[femtopb(bool, tag = 10)]
    pub has_remote_hardware: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// A heartbeat message is sent to the node from the client to keep the connection alive.
/// This is currently only needed to keep serial connections alive, but can be used by any PhoneAPI.
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Heartbeat<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// RemoteHardwarePins associated with a node
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NodeRemoteHardwarePin<'a> {
    ///
    /// The node_num exposing the available gpio pin
    #[femtopb(uint32, tag = 1)]
    pub node_num: u32,
    ///
    /// The the available gpio pin for usage with RemoteHardware module
    #[femtopb(message, optional, tag = 2)]
    pub pin: ::core::option::Option<RemoteHardwarePin<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ChunkedPayload<'a> {
    ///
    /// The ID of the entire payload
    #[femtopb(uint32, tag = 1)]
    pub payload_id: u32,
    ///
    /// The total number of chunks in the payload
    #[femtopb(uint32, tag = 2)]
    pub chunk_count: u32,
    ///
    /// The current chunk index in the total
    #[femtopb(uint32, tag = 3)]
    pub chunk_index: u32,
    ///
    /// The binary data of the current chunk
    #[femtopb(bytes, tag = 4)]
    pub payload_chunk: &'a [u8],
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Wrapper message for broken repeated oneof support
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ResendChunks<'a> {
    #[femtopb(uint32, packed, tag = 1)]
    pub chunks: ::femtopb::packed::Packed<'a, u32, ::femtopb::item_encoding::UInt32>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Responses to a ChunkedPayload request
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ChunkedPayloadResponse<'a> {
    ///
    /// The ID of the entire payload
    #[femtopb(uint32, tag = 1)]
    pub payload_id: u32,
    #[femtopb(oneof, tags = [2, 3, 4])]
    pub payload_variant: ::core::option::Option<
        chunked_payload_response::PayloadVariant<'a>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `ChunkedPayloadResponse`.
pub mod chunked_payload_response {
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// Request to transfer chunked payload
        #[femtopb(bool, tag = 2)]
        RequestTransfer(bool),
        ///
        /// Accept the transfer chunked payload
        #[femtopb(bool, tag = 3)]
        AcceptTransfer(bool),
        ///
        /// Request missing indexes in the chunked payload
        #[femtopb(message, tag = 4)]
        ResendChunks(super::ResendChunks<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// Note: these enum names must EXACTLY match the string used in the device
/// bin/build-all.sh script.
/// Because they will be used to find firmware filenames in the android app for OTA updates.
/// To match the old style filenames, _ is converted to -, p is converted to .
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum HardwareModel {
    ///
    /// TODO: REPLACE
    #[default]
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
    /// M5 esp32 based MCU modules with enclosure, TFT and LORA Shields. All Variants (Basic, Core, Fire, Core2, Paper) <https://m5stack.com/>
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
            Self::Unset => "UNSET",
            Self::TloraV2 => "TLORA_V2",
            Self::TloraV1 => "TLORA_V1",
            Self::TloraV211p6 => "TLORA_V2_1_1P6",
            Self::Tbeam => "TBEAM",
            Self::HeltecV20 => "HELTEC_V2_0",
            Self::TbeamV0p7 => "TBEAM_V0P7",
            Self::TEcho => "T_ECHO",
            Self::TloraV11p3 => "TLORA_V1_1P3",
            Self::Rak4631 => "RAK4631",
            Self::HeltecV21 => "HELTEC_V2_1",
            Self::HeltecV1 => "HELTEC_V1",
            Self::LilygoTbeamS3Core => "LILYGO_TBEAM_S3_CORE",
            Self::Rak11200 => "RAK11200",
            Self::NanoG1 => "NANO_G1",
            Self::TloraV211p8 => "TLORA_V2_1_1P8",
            Self::TloraT3S3 => "TLORA_T3_S3",
            Self::NanoG1Explorer => "NANO_G1_EXPLORER",
            Self::NanoG2Ultra => "NANO_G2_ULTRA",
            Self::LoraType => "LORA_TYPE",
            Self::Wiphone => "WIPHONE",
            Self::WioWm1110 => "WIO_WM1110",
            Self::Rak2560 => "RAK2560",
            Self::HeltecHru3601 => "HELTEC_HRU_3601",
            Self::StationG1 => "STATION_G1",
            Self::Rak11310 => "RAK11310",
            Self::SenseloraRp2040 => "SENSELORA_RP2040",
            Self::SenseloraS3 => "SENSELORA_S3",
            Self::Canaryone => "CANARYONE",
            Self::Rp2040Lora => "RP2040_LORA",
            Self::StationG2 => "STATION_G2",
            Self::LoraRelayV1 => "LORA_RELAY_V1",
            Self::Nrf52840dk => "NRF52840DK",
            Self::Ppr => "PPR",
            Self::Genieblocks => "GENIEBLOCKS",
            Self::Nrf52Unknown => "NRF52_UNKNOWN",
            Self::Portduino => "PORTDUINO",
            Self::AndroidSim => "ANDROID_SIM",
            Self::DiyV1 => "DIY_V1",
            Self::Nrf52840Pca10059 => "NRF52840_PCA10059",
            Self::DrDev => "DR_DEV",
            Self::M5stack => "M5STACK",
            Self::HeltecV3 => "HELTEC_V3",
            Self::HeltecWslV3 => "HELTEC_WSL_V3",
            Self::Betafpv2400Tx => "BETAFPV_2400_TX",
            Self::Betafpv900NanoTx => "BETAFPV_900_NANO_TX",
            Self::RpiPico => "RPI_PICO",
            Self::HeltecWirelessTracker => "HELTEC_WIRELESS_TRACKER",
            Self::HeltecWirelessPaper => "HELTEC_WIRELESS_PAPER",
            Self::TDeck => "T_DECK",
            Self::TWatchS3 => "T_WATCH_S3",
            Self::PicomputerS3 => "PICOMPUTER_S3",
            Self::HeltecHt62 => "HELTEC_HT62",
            Self::EbyteEsp32S3 => "EBYTE_ESP32_S3",
            Self::Esp32S3Pico => "ESP32_S3_PICO",
            Self::Chatter2 => "CHATTER_2",
            Self::HeltecWirelessPaperV10 => "HELTEC_WIRELESS_PAPER_V1_0",
            Self::HeltecWirelessTrackerV10 => "HELTEC_WIRELESS_TRACKER_V1_0",
            Self::Unphone => "UNPHONE",
            Self::TdLorac => "TD_LORAC",
            Self::CdebyteEoraS3 => "CDEBYTE_EORA_S3",
            Self::TwcMeshV4 => "TWC_MESH_V4",
            Self::Nrf52PromicroDiy => "NRF52_PROMICRO_DIY",
            Self::Radiomaster900BanditNano => "RADIOMASTER_900_BANDIT_NANO",
            Self::HeltecCapsuleSensorV3 => "HELTEC_CAPSULE_SENSOR_V3",
            Self::HeltecVisionMasterT190 => "HELTEC_VISION_MASTER_T190",
            Self::HeltecVisionMasterE213 => "HELTEC_VISION_MASTER_E213",
            Self::HeltecVisionMasterE290 => "HELTEC_VISION_MASTER_E290",
            Self::HeltecMeshNodeT114 => "HELTEC_MESH_NODE_T114",
            Self::SensecapIndicator => "SENSECAP_INDICATOR",
            Self::TrackerT1000E => "TRACKER_T1000_E",
            Self::Rak3172 => "RAK3172",
            Self::WioE5 => "WIO_E5",
            Self::Radiomaster900Bandit => "RADIOMASTER_900_BANDIT",
            Self::PrivateHw => "PRIVATE_HW",
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
            "PRIVATE_HW" => Some(Self::PrivateHw),
            _ => None,
        }
    }
}
///
/// Shared constants between device and phone
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum Constants {
    ///
    /// First enum must be zero, and we are just using this enum to
    /// pass int constants between two very different environments
    #[default]
    Zero = 0,
    ///
    /// From mesh.options
    /// note: this payload length is ONLY the bytes that are sent inside of the Data protobuf (excluding protobuf overhead). The 16 byte header is
    /// outside of this envelope
    DataPayloadLen = 237,
}
impl Constants {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Zero => "ZERO",
            Self::DataPayloadLen => "DATA_PAYLOAD_LEN",
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
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum CriticalErrorCode {
    ///
    /// TODO: REPLACE
    #[default]
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
            Self::None => "NONE",
            Self::TxWatchdog => "TX_WATCHDOG",
            Self::SleepEnterWait => "SLEEP_ENTER_WAIT",
            Self::NoRadio => "NO_RADIO",
            Self::Unspecified => "UNSPECIFIED",
            Self::UbloxUnitFailed => "UBLOX_UNIT_FAILED",
            Self::NoAxp192 => "NO_AXP192",
            Self::InvalidRadioSetting => "INVALID_RADIO_SETTING",
            Self::TransmitFailed => "TRANSMIT_FAILED",
            Self::Brownout => "BROWNOUT",
            Self::Sx1262Failure => "SX1262_FAILURE",
            Self::RadioSpiBug => "RADIO_SPI_BUG",
            Self::FlashCorruptionRecoverable => "FLASH_CORRUPTION_RECOVERABLE",
            Self::FlashCorruptionUnrecoverable => "FLASH_CORRUPTION_UNRECOVERABLE",
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
/// This message is handled by the Admin module and is responsible for all settings/channel read/write operations.
/// This message is used to do settings operations to both remote AND local nodes.
/// (Prior to 1.2 these operations were done via special ToRadio operations)
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct AdminMessage<'a> {
    ///
    /// TODO: REPLACE
    #[femtopb(
        oneof,
        tags = [1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        10,
        11,
        12,
        13,
        14,
        15,
        16,
        17,
        18,
        19,
        20,
        21,
        22,
        23,
        32,
        33,
        34,
        35,
        36,
        37,
        38,
        39,
        40,
        41,
        42,
        64,
        65,
        94,
        95,
        96,
        97,
        98,
        99,
        100]
    )]
    pub payload_variant: ::core::option::Option<admin_message::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `AdminMessage`.
pub mod admin_message {
    ///
    /// TODO: REPLACE
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum ConfigType {
        ///
        /// TODO: REPLACE
        #[default]
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
    }
    impl ConfigType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::DeviceConfig => "DEVICE_CONFIG",
                Self::PositionConfig => "POSITION_CONFIG",
                Self::PowerConfig => "POWER_CONFIG",
                Self::NetworkConfig => "NETWORK_CONFIG",
                Self::DisplayConfig => "DISPLAY_CONFIG",
                Self::LoraConfig => "LORA_CONFIG",
                Self::BluetoothConfig => "BLUETOOTH_CONFIG",
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
                _ => None,
            }
        }
    }
    ///
    /// TODO: REPLACE
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum ModuleConfigType {
        ///
        /// TODO: REPLACE
        #[default]
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
                Self::MqttConfig => "MQTT_CONFIG",
                Self::SerialConfig => "SERIAL_CONFIG",
                Self::ExtnotifConfig => "EXTNOTIF_CONFIG",
                Self::StoreforwardConfig => "STOREFORWARD_CONFIG",
                Self::RangetestConfig => "RANGETEST_CONFIG",
                Self::TelemetryConfig => "TELEMETRY_CONFIG",
                Self::CannedmsgConfig => "CANNEDMSG_CONFIG",
                Self::AudioConfig => "AUDIO_CONFIG",
                Self::RemotehardwareConfig => "REMOTEHARDWARE_CONFIG",
                Self::NeighborinfoConfig => "NEIGHBORINFO_CONFIG",
                Self::AmbientlightingConfig => "AMBIENTLIGHTING_CONFIG",
                Self::DetectionsensorConfig => "DETECTIONSENSOR_CONFIG",
                Self::PaxcounterConfig => "PAXCOUNTER_CONFIG",
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
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// Send the specified channel in the response to this message
        /// NOTE: This field is sent with the channel index + 1 (to ensure we never try to send 'zero' - which protobufs treats as not present)
        #[femtopb(uint32, tag = 1)]
        GetChannelRequest(u32),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 2)]
        GetChannelResponse(super::Channel<'a>),
        ///
        /// Send the current owner data in the response to this message.
        #[femtopb(bool, tag = 3)]
        GetOwnerRequest(bool),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 4)]
        GetOwnerResponse(super::User<'a>),
        ///
        /// Ask for the following config data to be sent
        #[femtopb(enumeration, tag = 5)]
        GetConfigRequest(::femtopb::enumeration::EnumValue<ConfigType>),
        ///
        /// Send the current Config in the response to this message.
        #[femtopb(message, tag = 6)]
        GetConfigResponse(super::Config<'a>),
        ///
        /// Ask for the following config data to be sent
        #[femtopb(enumeration, tag = 7)]
        GetModuleConfigRequest(::femtopb::enumeration::EnumValue<ModuleConfigType>),
        ///
        /// Send the current Config in the response to this message.
        #[femtopb(message, tag = 8)]
        GetModuleConfigResponse(super::ModuleConfig<'a>),
        ///
        /// Get the Canned Message Module messages in the response to this message.
        #[femtopb(bool, tag = 10)]
        GetCannedMessageModuleMessagesRequest(bool),
        ///
        /// Get the Canned Message Module messages in the response to this message.
        #[femtopb(string, tag = 11)]
        GetCannedMessageModuleMessagesResponse(&'a str),
        ///
        /// Request the node to send device metadata (firmware, protobuf version, etc)
        #[femtopb(bool, tag = 12)]
        GetDeviceMetadataRequest(bool),
        ///
        /// Device metadata response
        #[femtopb(message, tag = 13)]
        GetDeviceMetadataResponse(super::DeviceMetadata<'a>),
        ///
        /// Get the Ringtone in the response to this message.
        #[femtopb(bool, tag = 14)]
        GetRingtoneRequest(bool),
        ///
        /// Get the Ringtone in the response to this message.
        #[femtopb(string, tag = 15)]
        GetRingtoneResponse(&'a str),
        ///
        /// Request the node to send it's connection status
        #[femtopb(bool, tag = 16)]
        GetDeviceConnectionStatusRequest(bool),
        ///
        /// Device connection status response
        #[femtopb(message, tag = 17)]
        GetDeviceConnectionStatusResponse(super::DeviceConnectionStatus<'a>),
        ///
        /// Setup a node for licensed amateur (ham) radio operation
        #[femtopb(message, tag = 18)]
        SetHamMode(super::HamParameters<'a>),
        ///
        /// Get the mesh's nodes with their available gpio pins for RemoteHardware module use
        #[femtopb(bool, tag = 19)]
        GetNodeRemoteHardwarePinsRequest(bool),
        ///
        /// Respond with the mesh's nodes with their available gpio pins for RemoteHardware module use
        #[femtopb(message, tag = 20)]
        GetNodeRemoteHardwarePinsResponse(super::NodeRemoteHardwarePinsResponse<'a>),
        ///
        /// Enter (UF2) DFU mode
        /// Only implemented on NRF52 currently
        #[femtopb(bool, tag = 21)]
        EnterDfuModeRequest(bool),
        ///
        /// Delete the file by the specified path from the device
        #[femtopb(string, tag = 22)]
        DeleteFileRequest(&'a str),
        ///
        /// Set zero and offset for scale chips
        #[femtopb(uint32, tag = 23)]
        SetScale(u32),
        ///
        /// Set the owner for this node
        #[femtopb(message, tag = 32)]
        SetOwner(super::User<'a>),
        ///
        /// Set channels (using the new API).
        /// A special channel is the "primary channel".
        /// The other records are secondary channels.
        /// Note: only one channel can be marked as primary.
        /// If the client sets a particular channel to be primary, the previous channel will be set to SECONDARY automatically.
        #[femtopb(message, tag = 33)]
        SetChannel(super::Channel<'a>),
        ///
        /// Set the current Config
        #[femtopb(message, tag = 34)]
        SetConfig(super::Config<'a>),
        ///
        /// Set the current Config
        #[femtopb(message, tag = 35)]
        SetModuleConfig(super::ModuleConfig<'a>),
        ///
        /// Set the Canned Message Module messages text.
        #[femtopb(string, tag = 36)]
        SetCannedMessageModuleMessages(&'a str),
        ///
        /// Set the ringtone for ExternalNotification.
        #[femtopb(string, tag = 37)]
        SetRingtoneMessage(&'a str),
        ///
        /// Remove the node by the specified node-num from the NodeDB on the device
        #[femtopb(uint32, tag = 38)]
        RemoveByNodenum(u32),
        ///
        /// Set specified node-num to be favorited on the NodeDB on the device
        #[femtopb(uint32, tag = 39)]
        SetFavoriteNode(u32),
        ///
        /// Set specified node-num to be un-favorited on the NodeDB on the device
        #[femtopb(uint32, tag = 40)]
        RemoveFavoriteNode(u32),
        ///
        /// Set fixed position data on the node and then set the position.fixed_position = true
        #[femtopb(message, tag = 41)]
        SetFixedPosition(super::Position<'a>),
        ///
        /// Clear fixed position coordinates and then set position.fixed_position = false
        #[femtopb(bool, tag = 42)]
        RemoveFixedPosition(bool),
        ///
        /// Begins an edit transaction for config, module config, owner, and channel settings changes
        /// This will delay the standard *implicit* save to the file system and subsequent reboot behavior until committed (commit_edit_settings)
        #[femtopb(bool, tag = 64)]
        BeginEditSettings(bool),
        ///
        /// Commits an open transaction for any edits made to config, module config, owner, and channel settings
        #[femtopb(bool, tag = 65)]
        CommitEditSettings(bool),
        ///
        /// Tell the node to factory reset config everything; all device state and configuration will be returned to factory defaults and BLE bonds will be cleared.
        #[femtopb(int32, tag = 94)]
        FactoryResetDevice(i32),
        ///
        /// Tell the node to reboot into the OTA Firmware in this many seconds (or <0 to cancel reboot)
        /// Only Implemented for ESP32 Devices. This needs to be issued to send a new main firmware via bluetooth.
        #[femtopb(int32, tag = 95)]
        RebootOtaSeconds(i32),
        ///
        /// This message is only supported for the simulator Portduino build.
        /// If received the simulator will exit successfully.
        #[femtopb(bool, tag = 96)]
        ExitSimulator(bool),
        ///
        /// Tell the node to reboot in this many seconds (or <0 to cancel reboot)
        #[femtopb(int32, tag = 97)]
        RebootSeconds(i32),
        ///
        /// Tell the node to shutdown in this many seconds (or <0 to cancel shutdown)
        #[femtopb(int32, tag = 98)]
        ShutdownSeconds(i32),
        ///
        /// Tell the node to factory reset config; all device state and configuration will be returned to factory defaults; BLE bonds will be preserved.
        #[femtopb(int32, tag = 99)]
        FactoryResetConfig(i32),
        ///
        /// Tell the node to reset the nodedb.
        #[femtopb(int32, tag = 100)]
        NodedbReset(i32),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// Parameters for setting up Meshtastic for ameteur radio usage
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct HamParameters<'a> {
    ///
    /// Amateur radio call sign, eg. KD2ABC
    #[femtopb(string, tag = 1)]
    pub call_sign: &'a str,
    ///
    /// Transmit power in dBm at the LoRA transceiver, not including any amplification
    #[femtopb(int32, tag = 2)]
    pub tx_power: i32,
    ///
    /// The selected frequency of LoRA operation
    /// Please respect your local laws, regulations, and band plans.
    /// Ensure your radio is capable of operating of the selected frequency before setting this.
    #[femtopb(float, tag = 3)]
    pub frequency: f32,
    ///
    /// Optional short name of user
    #[femtopb(string, tag = 4)]
    pub short_name: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Response envelope for node_remote_hardware_pins
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NodeRemoteHardwarePinsResponse<'a> {
    ///
    /// Nodes and their respective remote hardware GPIO pins
    #[femtopb(message, repeated, tag = 1)]
    pub node_remote_hardware_pins: ::femtopb::repeated::Repeated<
        'a,
        NodeRemoteHardwarePin<'a>,
        ::femtopb::item_encoding::Message<'a, NodeRemoteHardwarePin<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This is the most compact possible representation for a set of channels.
/// It includes only one PRIMARY channel (which must be first) and
/// any SECONDARY channels.
/// No DISABLED channels are included.
/// This abstraction is used only on the the 'app side' of the world (ie python, javascript and android etc) to show a group of Channels as a (long) URL
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ChannelSet<'a> {
    ///
    /// Channel list with settings
    #[femtopb(message, repeated, tag = 1)]
    pub settings: ::femtopb::repeated::Repeated<
        'a,
        ChannelSettings<'a>,
        ::femtopb::item_encoding::Message<'a, ChannelSettings<'a>>,
    >,
    ///
    /// LoRa config
    #[femtopb(message, optional, tag = 2)]
    pub lora_config: ::core::option::Option<config::LoRaConfig<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Packets for the official ATAK Plugin
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct TakPacket<'a> {
    ///
    /// Are the payloads strings compressed for LoRA transport?
    #[femtopb(bool, tag = 1)]
    pub is_compressed: bool,
    ///
    /// The contact / callsign for ATAK user
    #[femtopb(message, optional, tag = 2)]
    pub contact: ::core::option::Option<Contact<'a>>,
    ///
    /// The group for ATAK user
    #[femtopb(message, optional, tag = 3)]
    pub group: ::core::option::Option<Group<'a>>,
    ///
    /// The status of the ATAK EUD
    #[femtopb(message, optional, tag = 4)]
    pub status: ::core::option::Option<Status<'a>>,
    ///
    /// The payload of the packet
    #[femtopb(oneof, tags = [5, 6])]
    pub payload_variant: ::core::option::Option<tak_packet::PayloadVariant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `TAKPacket`.
pub mod tak_packet {
    ///
    /// The payload of the packet
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum PayloadVariant<'a> {
        ///
        /// TAK position report
        #[femtopb(message, tag = 5)]
        Pli(super::Pli<'a>),
        ///
        /// ATAK GeoChat message
        #[femtopb(message, tag = 6)]
        Chat(super::GeoChat<'a>),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
///
/// ATAK GeoChat message
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct GeoChat<'a> {
    ///
    /// The text message
    #[femtopb(string, tag = 1)]
    pub message: &'a str,
    ///
    /// Uid recipient of the message
    #[femtopb(string, optional, tag = 2)]
    pub to: ::core::option::Option<&'a str>,
    ///
    /// Callsign of the recipient for the message
    #[femtopb(string, optional, tag = 3)]
    pub to_callsign: ::core::option::Option<&'a str>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// ATAK Group
/// <__group role='Team Member' name='Cyan'/>
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Group<'a> {
    ///
    /// Role of the group member
    #[femtopb(enumeration, tag = 1)]
    pub role: ::femtopb::enumeration::EnumValue<MemberRole>,
    ///
    /// Team (color)
    /// Default Cyan
    #[femtopb(enumeration, tag = 2)]
    pub team: ::femtopb::enumeration::EnumValue<Team>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// ATAK EUD Status
/// <status battery='100' />
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Status<'a> {
    ///
    /// Battery level
    #[femtopb(uint32, tag = 1)]
    pub battery: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// ATAK Contact
/// <contact endpoint='0.0.0.0:4242:tcp' phone='+12345678' callsign='FALKE'/>
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct Contact<'a> {
    ///
    /// Callsign
    #[femtopb(string, tag = 1)]
    pub callsign: &'a str,
    ///
    /// Device callsign
    ///
    ///
    /// IP address of endpoint in integer form (0.0.0.0 default)
    #[femtopb(string, tag = 2)]
    pub device_callsign: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Position Location Information from ATAK
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Pli<'a> {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[femtopb(sfixed32, tag = 1)]
    pub latitude_i: i32,
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[femtopb(sfixed32, tag = 2)]
    pub longitude_i: i32,
    ///
    /// Altitude (ATAK prefers HAE)
    #[femtopb(int32, tag = 3)]
    pub altitude: i32,
    ///
    /// Speed
    #[femtopb(uint32, tag = 4)]
    pub speed: u32,
    ///
    /// Course in degrees
    #[femtopb(uint32, tag = 5)]
    pub course: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum Team {
    ///
    /// Unspecifed
    #[default]
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
            Self::UnspecifedColor => "Unspecifed_Color",
            Self::White => "White",
            Self::Yellow => "Yellow",
            Self::Orange => "Orange",
            Self::Magenta => "Magenta",
            Self::Red => "Red",
            Self::Maroon => "Maroon",
            Self::Purple => "Purple",
            Self::DarkBlue => "Dark_Blue",
            Self::Blue => "Blue",
            Self::Cyan => "Cyan",
            Self::Teal => "Teal",
            Self::Green => "Green",
            Self::DarkGreen => "Dark_Green",
            Self::Brown => "Brown",
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
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum MemberRole {
    ///
    /// Unspecifed
    #[default]
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
            Self::Unspecifed => "Unspecifed",
            Self::TeamMember => "TeamMember",
            Self::TeamLead => "TeamLead",
            Self::Hq => "HQ",
            Self::Sniper => "Sniper",
            Self::Medic => "Medic",
            Self::ForwardObserver => "ForwardObserver",
            Self::Rto => "RTO",
            Self::K9 => "K9",
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
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct CannedMessageModuleConfig<'a> {
    ///
    /// Predefined messages for canned message module separated by '|' characters.
    #[femtopb(string, tag = 1)]
    pub messages: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LocalConfig<'a> {
    ///
    /// The part of the config that is specific to the Device
    #[femtopb(message, optional, tag = 1)]
    pub device: ::core::option::Option<config::DeviceConfig<'a>>,
    ///
    /// The part of the config that is specific to the GPS Position
    #[femtopb(message, optional, tag = 2)]
    pub position: ::core::option::Option<config::PositionConfig<'a>>,
    ///
    /// The part of the config that is specific to the Power settings
    #[femtopb(message, optional, tag = 3)]
    pub power: ::core::option::Option<config::PowerConfig<'a>>,
    ///
    /// The part of the config that is specific to the Wifi Settings
    #[femtopb(message, optional, tag = 4)]
    pub network: ::core::option::Option<config::NetworkConfig<'a>>,
    ///
    /// The part of the config that is specific to the Display
    #[femtopb(message, optional, tag = 5)]
    pub display: ::core::option::Option<config::DisplayConfig<'a>>,
    ///
    /// The part of the config that is specific to the Lora Radio
    #[femtopb(message, optional, tag = 6)]
    pub lora: ::core::option::Option<config::LoRaConfig<'a>>,
    ///
    /// The part of the config that is specific to the Bluetooth settings
    #[femtopb(message, optional, tag = 7)]
    pub bluetooth: ::core::option::Option<config::BluetoothConfig<'a>>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[femtopb(uint32, tag = 8)]
    pub version: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct LocalModuleConfig<'a> {
    ///
    /// The part of the config that is specific to the MQTT module
    #[femtopb(message, optional, tag = 1)]
    pub mqtt: ::core::option::Option<module_config::MqttConfig<'a>>,
    ///
    /// The part of the config that is specific to the Serial module
    #[femtopb(message, optional, tag = 2)]
    pub serial: ::core::option::Option<module_config::SerialConfig<'a>>,
    ///
    /// The part of the config that is specific to the ExternalNotification module
    #[femtopb(message, optional, tag = 3)]
    pub external_notification: ::core::option::Option<
        module_config::ExternalNotificationConfig<'a>,
    >,
    ///
    /// The part of the config that is specific to the Store & Forward module
    #[femtopb(message, optional, tag = 4)]
    pub store_forward: ::core::option::Option<module_config::StoreForwardConfig<'a>>,
    ///
    /// The part of the config that is specific to the RangeTest module
    #[femtopb(message, optional, tag = 5)]
    pub range_test: ::core::option::Option<module_config::RangeTestConfig<'a>>,
    ///
    /// The part of the config that is specific to the Telemetry module
    #[femtopb(message, optional, tag = 6)]
    pub telemetry: ::core::option::Option<module_config::TelemetryConfig<'a>>,
    ///
    /// The part of the config that is specific to the Canned Message module
    #[femtopb(message, optional, tag = 7)]
    pub canned_message: ::core::option::Option<module_config::CannedMessageConfig<'a>>,
    ///
    /// The part of the config that is specific to the Audio module
    #[femtopb(message, optional, tag = 9)]
    pub audio: ::core::option::Option<module_config::AudioConfig<'a>>,
    ///
    /// The part of the config that is specific to the Remote Hardware module
    #[femtopb(message, optional, tag = 10)]
    pub remote_hardware: ::core::option::Option<module_config::RemoteHardwareConfig<'a>>,
    ///
    /// The part of the config that is specific to the Neighbor Info module
    #[femtopb(message, optional, tag = 11)]
    pub neighbor_info: ::core::option::Option<module_config::NeighborInfoConfig<'a>>,
    ///
    /// The part of the config that is specific to the Ambient Lighting module
    #[femtopb(message, optional, tag = 12)]
    pub ambient_lighting: ::core::option::Option<
        module_config::AmbientLightingConfig<'a>,
    >,
    ///
    /// The part of the config that is specific to the Detection Sensor module
    #[femtopb(message, optional, tag = 13)]
    pub detection_sensor: ::core::option::Option<
        module_config::DetectionSensorConfig<'a>,
    >,
    ///
    /// Paxcounter Config
    #[femtopb(message, optional, tag = 14)]
    pub paxcounter: ::core::option::Option<module_config::PaxcounterConfig<'a>>,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[femtopb(uint32, tag = 8)]
    pub version: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This abstraction is used to contain any configuration for provisioning a node on any client.
/// It is useful for importing and exporting configurations.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct DeviceProfile<'a> {
    ///
    /// Long name for the node
    #[femtopb(string, optional, tag = 1)]
    pub long_name: ::core::option::Option<&'a str>,
    ///
    /// Short name of the node
    #[femtopb(string, optional, tag = 2)]
    pub short_name: ::core::option::Option<&'a str>,
    ///
    /// The url of the channels from our node
    #[femtopb(string, optional, tag = 3)]
    pub channel_url: ::core::option::Option<&'a str>,
    ///
    /// The Config of the node
    #[femtopb(message, optional, tag = 4)]
    pub config: ::core::option::Option<LocalConfig<'a>>,
    ///
    /// The ModuleConfig of the node
    #[femtopb(message, optional, tag = 5)]
    pub module_config: ::core::option::Option<LocalModuleConfig<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Position with static location information only for NodeDBLite
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PositionLite<'a> {
    ///
    /// The new preferred location encoding, multiply by 1e-7 to get degrees
    /// in floating point
    #[femtopb(sfixed32, tag = 1)]
    pub latitude_i: i32,
    ///
    /// TODO: REPLACE
    #[femtopb(sfixed32, tag = 2)]
    pub longitude_i: i32,
    ///
    /// In meters above MSL (but see issue #359)
    #[femtopb(int32, tag = 3)]
    pub altitude: i32,
    ///
    /// This is usually not sent over the mesh (to save space), but it is sent
    /// from the phone so that the local device can set its RTC If it is sent over
    /// the mesh (because there are devices on the mesh without GPS), it will only
    /// be sent by devices which has a hardware GPS clock.
    /// seconds since 1970
    #[femtopb(fixed32, tag = 4)]
    pub time: u32,
    ///
    /// TODO: REPLACE
    #[femtopb(enumeration, tag = 5)]
    pub location_source: ::femtopb::enumeration::EnumValue<position::LocSource>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct NodeInfoLite<'a> {
    ///
    /// The node number
    #[femtopb(uint32, tag = 1)]
    pub num: u32,
    ///
    /// The user info for this node
    #[femtopb(message, optional, tag = 2)]
    pub user: ::core::option::Option<User<'a>>,
    ///
    /// This position data. Note: before 1.2.14 we would also store the last time we've heard from this node in position.time, that is no longer true.
    /// Position.time now indicates the last time we received a POSITION from that node.
    #[femtopb(message, optional, tag = 3)]
    pub position: ::core::option::Option<PositionLite<'a>>,
    ///
    /// Returns the Signal-to-noise ratio (SNR) of the last received message,
    /// as measured by the receiver. Return SNR of the last received message in dB
    #[femtopb(float, tag = 4)]
    pub snr: f32,
    ///
    /// Set to indicate the last time we received a packet from this node
    #[femtopb(fixed32, tag = 5)]
    pub last_heard: u32,
    ///
    /// The latest device metrics for the node.
    #[femtopb(message, optional, tag = 6)]
    pub device_metrics: ::core::option::Option<DeviceMetrics<'a>>,
    ///
    /// local channel index we heard that node on. Only populated if its not the default channel.
    #[femtopb(uint32, tag = 7)]
    pub channel: u32,
    ///
    /// True if we witnessed the node over MQTT instead of LoRA transport
    #[femtopb(bool, tag = 8)]
    pub via_mqtt: bool,
    ///
    /// Number of hops away from us this node is (0 if adjacent)
    #[femtopb(uint32, tag = 9)]
    pub hops_away: u32,
    ///
    /// True if node is in our favorites list
    /// Persists between NodeDB internal clean ups
    #[femtopb(bool, tag = 10)]
    pub is_favorite: bool,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This message is never sent over the wire, but it is used for serializing DB
/// state to flash in the device code
/// FIXME, since we write this each time we enter deep sleep (and have infinite
/// flash) it would be better to use some sort of append only data structure for
/// the receive queue and use the preferences store for the other stuff
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct DeviceState<'a> {
    ///
    /// Read only settings/info about this node
    #[femtopb(message, optional, tag = 2)]
    pub my_node: ::core::option::Option<MyNodeInfo<'a>>,
    ///
    /// My owner info
    #[femtopb(message, optional, tag = 3)]
    pub owner: ::core::option::Option<User<'a>>,
    ///
    /// Received packets saved for delivery to the phone
    #[femtopb(message, repeated, tag = 5)]
    pub receive_queue: ::femtopb::repeated::Repeated<
        'a,
        MeshPacket<'a>,
        ::femtopb::item_encoding::Message<'a, MeshPacket<'a>>,
    >,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[femtopb(uint32, tag = 8)]
    pub version: u32,
    ///
    /// We keep the last received text message (only) stored in the device flash,
    /// so we can show it on the screen.
    /// Might be null
    #[femtopb(message, optional, tag = 7)]
    pub rx_text_message: ::core::option::Option<MeshPacket<'a>>,
    ///
    /// Used only during development.
    /// Indicates developer is testing and changes should never be saved to flash.
    /// Deprecated in 2.3.1
    #[deprecated]
    #[femtopb(bool, tag = 9)]
    pub no_save: bool,
    ///
    /// Some GPS receivers seem to have bogus settings from the factory, so we always do one factory reset.
    #[femtopb(bool, tag = 11)]
    pub did_gps_reset: bool,
    ///
    /// We keep the last received waypoint stored in the device flash,
    /// so we can show it on the screen.
    /// Might be null
    #[femtopb(message, optional, tag = 12)]
    pub rx_waypoint: ::core::option::Option<MeshPacket<'a>>,
    ///
    /// The mesh's nodes with their available gpio pins for RemoteHardware module
    #[femtopb(message, repeated, tag = 13)]
    pub node_remote_hardware_pins: ::femtopb::repeated::Repeated<
        'a,
        NodeRemoteHardwarePin<'a>,
        ::femtopb::item_encoding::Message<'a, NodeRemoteHardwarePin<'a>>,
    >,
    ///
    /// New lite version of NodeDB to decrease memory footprint
    #[femtopb(message, repeated, tag = 14)]
    pub node_db_lite: ::femtopb::repeated::Repeated<
        'a,
        NodeInfoLite<'a>,
        ::femtopb::item_encoding::Message<'a, NodeInfoLite<'a>>,
    >,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// The on-disk saved channels
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ChannelFile<'a> {
    ///
    /// The channels our node knows about
    #[femtopb(message, repeated, tag = 1)]
    pub channels: ::femtopb::repeated::Repeated<
        'a,
        Channel<'a>,
        ::femtopb::item_encoding::Message<'a, Channel<'a>>,
    >,
    ///
    /// A version integer used to invalidate old save files when we make
    /// incompatible changes This integer is set at build time and is private to
    /// NodeDB.cpp in the device code.
    #[femtopb(uint32, tag = 2)]
    pub version: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// This can be used for customizing the firmware distribution. If populated,
/// show a secondary bootup screen with custom logo and text for 2.5 seconds.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct OemStore<'a> {
    ///
    /// The Logo width in Px
    #[femtopb(uint32, tag = 1)]
    pub oem_icon_width: u32,
    ///
    /// The Logo height in Px
    #[femtopb(uint32, tag = 2)]
    pub oem_icon_height: u32,
    ///
    /// The Logo in XBM bytechar format
    #[femtopb(bytes, tag = 3)]
    pub oem_icon_bits: &'a [u8],
    ///
    /// Use this font for the OEM text.
    #[femtopb(enumeration, tag = 4)]
    pub oem_font: ::femtopb::enumeration::EnumValue<ScreenFonts>,
    ///
    /// Use this font for the OEM text.
    #[femtopb(string, tag = 5)]
    pub oem_text: &'a str,
    ///
    /// The default device encryption key, 16 or 32 byte
    #[femtopb(bytes, tag = 6)]
    pub oem_aes_key: &'a [u8],
    ///
    /// A Preset LocalConfig to apply during factory reset
    #[femtopb(message, optional, tag = 7)]
    pub oem_local_config: ::core::option::Option<LocalConfig<'a>>,
    ///
    /// A Preset LocalModuleConfig to apply during factory reset
    #[femtopb(message, optional, tag = 8)]
    pub oem_local_module_config: ::core::option::Option<LocalModuleConfig<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Font sizes for the device screen
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::femtopb::Enumeration
)]
#[repr(i32)]
#[derive(Default)]
pub enum ScreenFonts {
    ///
    /// TODO: REPLACE
    #[default]
    FontSmall = 0,
    ///
    /// TODO: REPLACE
    FontMedium = 1,
    ///
    /// TODO: REPLACE
    FontLarge = 2,
}
impl ScreenFonts {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::FontSmall => "FONT_SMALL",
            Self::FontMedium => "FONT_MEDIUM",
            Self::FontLarge => "FONT_LARGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FONT_SMALL" => Some(Self::FontSmall),
            "FONT_MEDIUM" => Some(Self::FontMedium),
            "FONT_LARGE" => Some(Self::FontLarge),
            _ => None,
        }
    }
}
///
/// This message wraps a MeshPacket with extra metadata about the sender and how it arrived.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct ServiceEnvelope<'a> {
    ///
    /// The (probably encrypted) packet
    #[femtopb(message, optional, tag = 1)]
    pub packet: ::core::option::Option<MeshPacket<'a>>,
    ///
    /// The global channel ID it was sent on
    #[femtopb(string, tag = 2)]
    pub channel_id: &'a str,
    ///
    /// The sending gateway node ID. Can we use this to authenticate/prevent fake
    /// nodeid impersonation for senders? - i.e. use gateway/mesh id (which is authenticated) + local node id as
    /// the globally trusted nodenum
    #[femtopb(string, tag = 3)]
    pub gateway_id: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// Information about a node intended to be reported unencrypted to a map using MQTT.
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct MapReport<'a> {
    ///
    /// A full name for this user, i.e. "Kevin Hester"
    #[femtopb(string, tag = 1)]
    pub long_name: &'a str,
    ///
    /// A VERY short name, ideally two characters.
    /// Suitable for a tiny OLED screen
    #[femtopb(string, tag = 2)]
    pub short_name: &'a str,
    ///
    /// Role of the node that applies specific settings for a particular use-case
    #[femtopb(enumeration, tag = 3)]
    pub role: ::femtopb::enumeration::EnumValue<config::device_config::Role>,
    ///
    /// Hardware model of the node, i.e. T-Beam, Heltec V3, etc...
    #[femtopb(enumeration, tag = 4)]
    pub hw_model: ::femtopb::enumeration::EnumValue<HardwareModel>,
    ///
    /// Device firmware version string
    #[femtopb(string, tag = 5)]
    pub firmware_version: &'a str,
    ///
    /// The region code for the radio (US, CN, EU433, etc...)
    #[femtopb(enumeration, tag = 6)]
    pub region: ::femtopb::enumeration::EnumValue<config::lo_ra_config::RegionCode>,
    ///
    /// Modem preset used by the radio (LongFast, MediumSlow, etc...)
    #[femtopb(enumeration, tag = 7)]
    pub modem_preset: ::femtopb::enumeration::EnumValue<
        config::lo_ra_config::ModemPreset,
    >,
    ///
    /// Whether the node has a channel with default PSK and name (LongFast, MediumSlow, etc...)
    /// and it uses the default frequency slot given the region and modem preset.
    #[femtopb(bool, tag = 8)]
    pub has_default_channel: bool,
    ///
    /// Latitude: multiply by 1e-7 to get degrees in floating point
    #[femtopb(sfixed32, tag = 9)]
    pub latitude_i: i32,
    ///
    /// Longitude: multiply by 1e-7 to get degrees in floating point
    #[femtopb(sfixed32, tag = 10)]
    pub longitude_i: i32,
    ///
    /// Altitude in meters above MSL
    #[femtopb(int32, tag = 11)]
    pub altitude: i32,
    ///
    /// Indicates the bits of precision for latitude and longitude set by the sending node
    #[femtopb(uint32, tag = 12)]
    pub position_precision: u32,
    ///
    /// Number of online nodes (heard in the last 2 hours) this node has in its list that were received locally (not via MQTT)
    #[femtopb(uint32, tag = 13)]
    pub num_online_local_nodes: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// TODO: REPLACE
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct Paxcount<'a> {
    ///
    /// seen Wifi devices
    #[femtopb(uint32, tag = 1)]
    pub wifi: u32,
    ///
    /// Seen BLE devices
    #[femtopb(uint32, tag = 2)]
    pub ble: u32,
    ///
    /// Uptime in seconds
    #[femtopb(uint32, tag = 3)]
    pub uptime: u32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Note: There are no 'PowerMon' messages normally in use (PowerMons are sent only as structured logs - slogs).
/// But we wrap our State enum in this message to effectively nest a namespace (without our linter yelling at us)
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PowerMon<'a> {
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `PowerMon`.
pub mod power_mon {
    /// Any significant power changing event in meshtastic should be tagged with a powermon state transition.
    /// If you are making new meshtastic features feel free to add new entries at the end of this definition.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum State {
        #[default]
        None = 0,
        CpuDeepSleep = 1,
        CpuLightSleep = 2,
        ///
        /// The external Vext1 power is on.  Many boards have auxillary power rails that the CPU turns on only
        /// occasionally.  In cases where that rail has multiple devices on it we usually want to have logging on
        /// the state of that rail as an independent record.
        /// For instance on the Heltec Tracker 1.1 board, this rail is the power source for the GPS and screen.
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
                Self::None => "None",
                Self::CpuDeepSleep => "CPU_DeepSleep",
                Self::CpuLightSleep => "CPU_LightSleep",
                Self::Vext1On => "Vext1_On",
                Self::LoraRxOn => "Lora_RXOn",
                Self::LoraTxOn => "Lora_TXOn",
                Self::LoraRxActive => "Lora_RXActive",
                Self::BtOn => "BT_On",
                Self::LedOn => "LED_On",
                Self::ScreenOn => "Screen_On",
                Self::ScreenDrawing => "Screen_Drawing",
                Self::WifiOn => "Wifi_On",
                Self::GpsActive => "GPS_Active",
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
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct PowerStressMessage<'a> {
    ///
    /// What type of HardwareMessage is this?
    #[femtopb(enumeration, tag = 1)]
    pub cmd: ::femtopb::enumeration::EnumValue<power_stress_message::Opcode>,
    #[femtopb(float, tag = 2)]
    pub num_seconds: f32,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `PowerStressMessage`.
pub mod power_stress_message {
    ///
    /// What operation would we like the UUT to perform.
    /// note: senders should probably set want_response in their request packets, so that they can know when the state
    /// machine has started processing their request
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Opcode {
        ///
        /// Unset/unused
        #[default]
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
                Self::Unset => "UNSET",
                Self::PrintInfo => "PRINT_INFO",
                Self::ForceQuiet => "FORCE_QUIET",
                Self::EndQuiet => "END_QUIET",
                Self::ScreenOn => "SCREEN_ON",
                Self::ScreenOff => "SCREEN_OFF",
                Self::CpuIdle => "CPU_IDLE",
                Self::CpuDeepsleep => "CPU_DEEPSLEEP",
                Self::CpuFullon => "CPU_FULLON",
                Self::LedOn => "LED_ON",
                Self::LedOff => "LED_OFF",
                Self::LoraOff => "LORA_OFF",
                Self::LoraTx => "LORA_TX",
                Self::LoraRx => "LORA_RX",
                Self::BtOff => "BT_OFF",
                Self::BtOn => "BT_ON",
                Self::WifiOff => "WIFI_OFF",
                Self::WifiOn => "WIFI_ON",
                Self::GpsOff => "GPS_OFF",
                Self::GpsOn => "GPS_ON",
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
#[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
pub struct HardwareMessage<'a> {
    ///
    /// What type of HardwareMessage is this?
    #[femtopb(enumeration, tag = 1)]
    pub r#type: ::femtopb::enumeration::EnumValue<hardware_message::Type>,
    ///
    /// What gpios are we changing. Not used for all MessageTypes, see MessageType for details
    #[femtopb(uint64, tag = 2)]
    pub gpio_mask: u64,
    ///
    /// For gpios that were listed in gpio_mask as valid, what are the signal levels for those gpios.
    /// Not used for all MessageTypes, see MessageType for details
    #[femtopb(uint64, tag = 3)]
    pub gpio_value: u64,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `HardwareMessage`.
pub mod hardware_message {
    ///
    /// TODO: REPLACE
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum Type {
        ///
        /// Unset/unused
        #[default]
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
                Self::Unset => "UNSET",
                Self::WriteGpios => "WRITE_GPIOS",
                Self::WatchGpios => "WATCH_GPIOS",
                Self::GpiosChanged => "GPIOS_CHANGED",
                Self::ReadGpios => "READ_GPIOS",
                Self::ReadGpiosReply => "READ_GPIOS_REPLY",
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
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct RtttlConfig<'a> {
    ///
    /// Ringtone for PWM Buzzer in RTTTL Format.
    #[femtopb(string, tag = 1)]
    pub ringtone: &'a str,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
///
/// TODO: REPLACE
#[derive(Clone, PartialEq, ::femtopb::Message)]
pub struct StoreAndForward<'a> {
    ///
    /// TODO: REPLACE
    #[femtopb(enumeration, tag = 1)]
    pub rr: ::femtopb::enumeration::EnumValue<store_and_forward::RequestResponse>,
    ///
    /// TODO: REPLACE
    #[femtopb(oneof, tags = [2, 3, 4, 5])]
    pub variant: ::core::option::Option<store_and_forward::Variant<'a>>,
    #[femtopb(unknown_fields)]
    pub unknown_fields: femtopb::UnknownFields<'a>,
}
/// Nested message and enum types in `StoreAndForward`.
pub mod store_and_forward {
    ///
    /// TODO: REPLACE
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct Statistics<'a> {
        ///
        /// Number of messages we have ever seen
        #[femtopb(uint32, tag = 1)]
        pub messages_total: u32,
        ///
        /// Number of messages we have currently saved our history.
        #[femtopb(uint32, tag = 2)]
        pub messages_saved: u32,
        ///
        /// Maximum number of messages we will save
        #[femtopb(uint32, tag = 3)]
        pub messages_max: u32,
        ///
        /// Router uptime in seconds
        #[femtopb(uint32, tag = 4)]
        pub up_time: u32,
        ///
        /// Number of times any client sent a request to the S&F.
        #[femtopb(uint32, tag = 5)]
        pub requests: u32,
        ///
        /// Number of times the history was requested.
        #[femtopb(uint32, tag = 6)]
        pub requests_history: u32,
        ///
        /// Is the heartbeat enabled on the server?
        #[femtopb(bool, tag = 7)]
        pub heartbeat: bool,
        ///
        /// Maximum number of messages the server will return.
        #[femtopb(uint32, tag = 8)]
        pub return_max: u32,
        ///
        /// Maximum history window in minutes the server will return messages from.
        #[femtopb(uint32, tag = 9)]
        pub return_window: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// TODO: REPLACE
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct History<'a> {
        ///
        /// Number of that will be sent to the client
        #[femtopb(uint32, tag = 1)]
        pub history_messages: u32,
        ///
        /// The window of messages that was used to filter the history client requested
        #[femtopb(uint32, tag = 2)]
        pub window: u32,
        ///
        /// Index in the packet history of the last message sent in a previous request to the server.
        /// Will be sent to the client before sending the history and can be set in a subsequent request to avoid getting packets the server already sent to the client.
        #[femtopb(uint32, tag = 3)]
        pub last_request: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// TODO: REPLACE
    #[derive(Clone, Copy, PartialEq, ::femtopb::Message)]
    pub struct Heartbeat<'a> {
        ///
        /// Period in seconds that the heartbeat is sent out that will be sent to the client
        #[femtopb(uint32, tag = 1)]
        pub period: u32,
        ///
        /// If set, this is not the primary Store & Forward router on the mesh
        #[femtopb(uint32, tag = 2)]
        pub secondary: u32,
        #[femtopb(unknown_fields)]
        pub unknown_fields: femtopb::UnknownFields<'a>,
    }
    ///
    /// 001 - 063 = From Router
    /// 064 - 127 = From Client
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::femtopb::Enumeration
    )]
    #[repr(i32)]
    #[derive(Default)]
    pub enum RequestResponse {
        ///
        /// Unset/unused
        #[default]
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
                Self::Unset => "UNSET",
                Self::RouterError => "ROUTER_ERROR",
                Self::RouterHeartbeat => "ROUTER_HEARTBEAT",
                Self::RouterPing => "ROUTER_PING",
                Self::RouterPong => "ROUTER_PONG",
                Self::RouterBusy => "ROUTER_BUSY",
                Self::RouterHistory => "ROUTER_HISTORY",
                Self::RouterStats => "ROUTER_STATS",
                Self::RouterTextDirect => "ROUTER_TEXT_DIRECT",
                Self::RouterTextBroadcast => "ROUTER_TEXT_BROADCAST",
                Self::ClientError => "CLIENT_ERROR",
                Self::ClientHistory => "CLIENT_HISTORY",
                Self::ClientStats => "CLIENT_STATS",
                Self::ClientPing => "CLIENT_PING",
                Self::ClientPong => "CLIENT_PONG",
                Self::ClientAbort => "CLIENT_ABORT",
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
    #[derive(Clone, PartialEq, ::femtopb::Oneof)]
    #[non_exhaustive]
    pub enum Variant<'a> {
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 2)]
        Stats(Statistics<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 3)]
        History(History<'a>),
        ///
        /// TODO: REPLACE
        #[femtopb(message, tag = 4)]
        Heartbeat(Heartbeat<'a>),
        ///
        /// Text from history message.
        #[femtopb(bytes, tag = 5)]
        Text(&'a [u8]),
        #[femtopb(phantom)]
        _Phantom(::core::marker::PhantomData<&'a ()>),
    }
}
