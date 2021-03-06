//! Matrix room version identifiers.

use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[cfg(feature = "diesel")]
use diesel::sql_types::Text;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{deserialize_id, error::Error};

/// Room version identifiers cannot be more than 32 code points.
const MAX_CODE_POINTS: usize = 32;

/// A Matrix room version ID.
///
/// A `RoomVersionId` can be or converted or deserialized from a string slice, and can be converted
/// or serialized back into a string as needed.
///
/// ```
/// # use std::convert::TryFrom;
/// # use ruma_identifiers::RoomVersionId;
/// assert_eq!(RoomVersionId::try_from("1").unwrap().to_string(), "1");
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "diesel", derive(FromSqlRow, QueryId, AsExpression, SqlType))]
#[cfg_attr(feature = "diesel", sql_type = "Text")]
pub struct RoomVersionId(InnerRoomVersionId);

/// Possibile values for room version, distinguishing between official Matrix versions and custom
/// versions.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum InnerRoomVersionId {
    /// A version 1 room.
    Version1,

    /// A version 2 room.
    Version2,

    /// A version 3 room.
    Version3,

    /// A version 4 room.
    Version4,

    /// A version 5 room.
    Version5,

    /// A custom room version.
    Custom(String),
}

impl RoomVersionId {
    /// Creates a version 1 room ID.
    pub fn version_1() -> Self {
        Self(InnerRoomVersionId::Version1)
    }

    /// Creates a version 2 room ID.
    pub fn version_2() -> Self {
        Self(InnerRoomVersionId::Version2)
    }

    /// Creates a version 3 room ID.
    pub fn version_3() -> Self {
        Self(InnerRoomVersionId::Version3)
    }

    /// Creates a version 4 room ID.
    pub fn version_4() -> Self {
        Self(InnerRoomVersionId::Version4)
    }

    /// Creates a version 5 room ID.
    pub fn version_5() -> Self {
        Self(InnerRoomVersionId::Version5)
    }

    /// Creates a custom room version ID from the given string slice.
    pub fn custom(id: &str) -> Self {
        Self(InnerRoomVersionId::Custom(id.to_string()))
    }

    /// Whether or not this room version is an official one specified by the Matrix protocol.
    pub fn is_official(&self) -> bool {
        !self.is_custom()
    }

    /// Whether or not this is a custom room version.
    pub fn is_custom(&self) -> bool {
        match self.0 {
            InnerRoomVersionId::Custom(_) => true,
            _ => false,
        }
    }

    /// Whether or not this is a version 1 room.
    pub fn is_version_1(&self) -> bool {
        self.0 == InnerRoomVersionId::Version1
    }

    /// Whether or not this is a version 2 room.
    pub fn is_version_2(&self) -> bool {
        self.0 == InnerRoomVersionId::Version2
    }

    /// Whether or not this is a version 3 room.
    pub fn is_version_3(&self) -> bool {
        self.0 == InnerRoomVersionId::Version3
    }

    /// Whether or not this is a version 4 room.
    pub fn is_version_4(&self) -> bool {
        self.0 == InnerRoomVersionId::Version4
    }

    /// Whether or not this is a version 5 room.
    pub fn is_version_5(&self) -> bool {
        self.0 == InnerRoomVersionId::Version5
    }
}

impl Display for RoomVersionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let message = match self.0 {
            InnerRoomVersionId::Version1 => "1",
            InnerRoomVersionId::Version2 => "2",
            InnerRoomVersionId::Version3 => "3",
            InnerRoomVersionId::Version4 => "4",
            InnerRoomVersionId::Version5 => "5",
            InnerRoomVersionId::Custom(ref version) => version,
        };

        write!(f, "{}", message)
    }
}

impl Serialize for RoomVersionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for RoomVersionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_id(deserializer, "a Matrix room version ID as a string")
    }
}

impl TryFrom<&str> for RoomVersionId {
    type Error = Error;

    /// Attempts to create a new Matrix room version ID from a string representation.
    fn try_from(room_version_id: &str) -> Result<Self, Error> {
        let version = match room_version_id {
            "1" => Self(InnerRoomVersionId::Version1),
            "2" => Self(InnerRoomVersionId::Version2),
            "3" => Self(InnerRoomVersionId::Version3),
            "4" => Self(InnerRoomVersionId::Version4),
            "5" => Self(InnerRoomVersionId::Version5),
            custom => {
                if custom.is_empty() {
                    return Err(Error::MinimumLengthNotSatisfied);
                } else if custom.chars().count() > MAX_CODE_POINTS {
                    return Err(Error::MaximumLengthExceeded);
                } else {
                    Self(InnerRoomVersionId::Custom(custom.to_string()))
                }
            }
        };

        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use serde_json::{from_str, to_string};

    use super::RoomVersionId;
    use crate::error::Error;

    #[test]
    fn valid_version_1_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("1")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "1"
        );
    }

    #[test]
    fn valid_version_2_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("2")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "2"
        );
    }

    #[test]
    fn valid_version_3_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("3")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "3"
        );
    }

    #[test]
    fn valid_version_4_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("4")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "4"
        );
    }

    #[test]
    fn valid_version_5_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("5")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "5"
        );
    }

    #[test]
    fn valid_custom_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("io.ruma.1")
                .expect("Failed to create RoomVersionId.")
                .to_string(),
            "io.ruma.1"
        );
    }

    #[test]
    fn empty_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from(""),
            Err(Error::MinimumLengthNotSatisfied)
        );
    }

    #[test]
    fn over_max_code_point_room_version_id() {
        assert_eq!(
            RoomVersionId::try_from("0123456789012345678901234567890123456789"),
            Err(Error::MaximumLengthExceeded)
        );
    }

    #[test]
    fn serialize_official_room_id() {
        assert_eq!(
            to_string(&RoomVersionId::try_from("1").expect("Failed to create RoomVersionId."))
                .expect("Failed to convert RoomVersionId to JSON."),
            r#""1""#
        );
    }

    #[test]
    fn deserialize_official_room_id() {
        let deserialized =
            from_str::<RoomVersionId>(r#""1""#).expect("Failed to convert RoomVersionId to JSON.");

        assert!(deserialized.is_version_1());
        assert!(deserialized.is_official());

        assert_eq!(
            deserialized,
            RoomVersionId::try_from("1").expect("Failed to create RoomVersionId.")
        );
    }

    #[test]
    fn serialize_custom_room_id() {
        assert_eq!(
            to_string(
                &RoomVersionId::try_from("io.ruma.1").expect("Failed to create RoomVersionId.")
            )
            .expect("Failed to convert RoomVersionId to JSON."),
            r#""io.ruma.1""#
        );
    }

    #[test]
    fn deserialize_custom_room_id() {
        let deserialized = from_str::<RoomVersionId>(r#""io.ruma.1""#)
            .expect("Failed to convert RoomVersionId to JSON.");

        assert!(deserialized.is_custom());

        assert_eq!(
            deserialized,
            RoomVersionId::try_from("io.ruma.1").expect("Failed to create RoomVersionId.")
        );
    }

    #[test]
    fn constructors() {
        assert!(RoomVersionId::version_1().is_version_1());
        assert!(RoomVersionId::version_2().is_version_2());
        assert!(RoomVersionId::version_3().is_version_3());
        assert!(RoomVersionId::version_4().is_version_4());
        assert!(RoomVersionId::version_5().is_version_5());
        assert!(RoomVersionId::custom("foo").is_custom());
    }

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn predicate_methods() {
        let version_1 = RoomVersionId::try_from("1").expect("Failed to create RoomVersionId.");
        let version_2 = RoomVersionId::try_from("2").expect("Failed to create RoomVersionId.");
        let version_3 = RoomVersionId::try_from("3").expect("Failed to create RoomVersionId.");
        let version_4 = RoomVersionId::try_from("4").expect("Failed to create RoomVersionId.");
        let version_5 = RoomVersionId::try_from("5").expect("Failed to create RoomVersionId.");
        let custom = RoomVersionId::try_from("io.ruma.1").expect("Failed to create RoomVersionId.");

        assert!(version_1.is_version_1());
        assert!(version_2.is_version_2());
        assert!(version_3.is_version_3());
        assert!(version_4.is_version_4());
        assert!(version_5.is_version_5());

        assert!(!version_1.is_version_2());
        assert!(!version_1.is_version_3());
        assert!(!version_1.is_version_4());
        assert!(!version_1.is_version_5());

        assert!(version_1.is_official());
        assert!(version_2.is_official());
        assert!(version_3.is_official());
        assert!(version_4.is_official());
        assert!(version_5.is_official());

        assert!(!version_1.is_custom());
        assert!(!version_2.is_custom());
        assert!(!version_3.is_custom());
        assert!(!version_4.is_custom());
        assert!(!version_5.is_custom());

        assert!(custom.is_custom());
        assert!(!custom.is_official());
        assert!(!custom.is_version_1());
        assert!(!custom.is_version_2());
        assert!(!custom.is_version_3());
        assert!(!custom.is_version_4());
        assert!(!custom.is_version_5());
    }
}
