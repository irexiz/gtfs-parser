use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Id;
use derivative::Derivative;

#[derive(Debug, Serialize, Deserialize)]
pub struct FareAttribute {
    /// Identifies a fare class.
    #[serde(rename = "fare_id")]
    pub id: String,

    /// Fare price, in the unit specified by currency_type.
    pub price: f64,

    /// Currency used to pay the fare.
    #[serde(rename = "currency_type")]
    pub currency: String,

    /// Indicates when the fare must be paid
    pub payment_method: PaymentMethod,

    /// Indicates the number of transfers permitted on this fare.
    #[serde(default)]
    pub transfers: Transfers,

    /// Identifies the relevant agency for a fare.
    /// This field is required for datasets with multiple agencies defined in agency.txt, otherwise it is optional.
    pub agency_id: Option<String>,

    /// Length of time in seconds before a transfer expires.
    /// When transfers=0 this field can be used to indicate how long a ticket is valid for or it can can be left empty.
    pub transfer_duration: Option<usize>,
}
impl Id for FareAttribute {
    fn id(&self) -> &str {
        &self.id
    }
}

#[non_exhaustive]
#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum PaymentMethod {
    /// Fare is paid on board.
    #[serde(rename = "0")]
    Aboard,

    /// Fare must be paid before boarding.
    #[serde(rename = "1")]
    PreBoarding,
}

#[non_exhaustive]
#[derive(Derivative, Debug, Copy, Clone, PartialEq)]
#[derivative(Default)]
pub enum Transfers {
    /// Unlimited transfers are permitted.
    #[derivative(Default)]
    Unlimited,

    /// No transfers permitted on this fare.
    NoTransfer,

    /// Riders may transfer once.
    UniqueTransfer,

    /// Riders may transfer twice.
    TwoTransfers,

    Other(u16),
}

impl<'de> Deserialize<'de> for Transfers {
    fn deserialize<D>(deserializer: D) -> Result<Transfers, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i = Option::<u16>::deserialize(deserializer)?;
        Ok(match i {
            Some(0) => Transfers::NoTransfer,
            Some(1) => Transfers::UniqueTransfer,
            Some(2) => Transfers::TwoTransfers,
            Some(a) => Transfers::Other(a),
            None => Transfers::Unlimited,
        })
    }
}

impl Serialize for Transfers {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Transfers::NoTransfer => serializer.serialize_u16(0),
            Transfers::UniqueTransfer => serializer.serialize_u16(1),
            Transfers::TwoTransfers => serializer.serialize_u16(2),
            Transfers::Other(a) => serializer.serialize_u16(*a),
            Transfers::Unlimited => serializer.serialize_none(),
        }
    }
}
