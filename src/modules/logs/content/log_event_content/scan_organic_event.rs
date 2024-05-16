use serde::{Serialize, Deserialize};

use crate::modules::models::exploration::genus::Genus;
use crate::modules::models::exploration::species::Species;
use crate::modules::models::exploration::variant::Variant;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScanOrganicEvent {
    /// Possible values seem to be "Sample", "Analyze", "Log". It seems that the first scan for
    /// a bio species uses `Sample`, then the second consists of two back to back events: one with
    /// `Sample` and the one immediately after with `Analyze`. The contents seem to be the same. And
    /// the third and last entry seems to be `Log`.
    pub scan_type: ScanOrganicEventScanType,
    pub genus: Genus,

    #[serde(rename = "Genus_Localised")]
    pub genus_localized: Option<String>,

    pub species: Species,

    #[serde(rename = "Species_Localised")]
    pub species_localized: Option<String>,

    pub variant: Option<Variant>,

    #[serde(rename = "Variant_Localised")]
    pub variant_localized: Option<String>,

    pub system_address: u64,
    pub body: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum ScanOrganicEventScanType {
    Sample,
    Analyse,
    Log,
}
