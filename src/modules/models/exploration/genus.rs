use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

use crate::modules::models::exploration::species::Species;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Genus {
    #[serde(rename = "$Codex_Ent_Aleoids_Genus_Name;")]
    Aleoida,

    AmphoraPlant,
    Anemone,
    BarkMound,

    #[serde(rename = "$Codex_Ent_Bacterial_Genus_Name;")]
    Bacterium,

    #[serde(rename = "$Codex_Ent_Brancae_Name;")]
    BrainTree,

    #[serde(rename = "$Codex_Ent_Cactoid_Genus_Name;")]
    Cactoida,

    #[serde(rename = "$Codex_Ent_Clypeus_Genus_Name;")]
    Clypeus,

    #[serde(rename = "$Codex_Ent_Conchas_Genus_Name;")]
    Concha,

    #[serde(rename = "$Codex_Ent_Ground_Struct_Ice_Name;")]
    CrystallineShards,

    #[serde(rename = "$Codex_Ent_Electricae_Genus_Name;")]
    Electricae,

    #[serde(rename = "$Codex_Ent_Fonticulus_Genus_Name;")]
    Fonticulua,

    #[serde(rename = "$Codex_Ent_Shrubs_Genus_Name;")]
    Fruxeta,

    #[serde(rename = "$Codex_Ent_Fumerolas_Genus_Name;")]
    Fumerola,

    #[serde(rename = "$Codex_Ent_Fungoids_Genus_Name;")]
    Fungoida,

    #[serde(rename = "$Codex_Ent_Osseus_Genus_Name;")]
    Osseus,

    #[serde(rename = "$Codex_Ent_Recepta_Genus_Name;")]
    Recepta,

    SinuousTubers,

    #[serde(rename = "$Codex_Ent_Stratum_Genus_Name;")]
    Stratum,

    #[serde(rename = "$Codex_Ent_Tubus_Genus_Name;")]
    Tubus,

    #[serde(rename = "$Codex_Ent_Tussocks_Genus_Name;")]
    Tussock,

    Trutexa,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl From<&Species> for Genus {
    fn from(value: &Species) -> Self {
        match value {
            Species::AleoidaLaminiae
            | Species::AleoidaGravis
            | Species::AleoidaSpica
            | Species::AleoidaCoronamus
            | Species::AleoidaArcus => Genus::Aleoida,

            Species::AmphoraPlant => Genus::AmphoraPlant,

            Species::AnemonePrasinus
            | Species::AnemonePrasinumBioluminescent
            | Species::AnemonePuniceus
            | Species::AnemonePuniceum
            | Species::AnemoneRoseus
            | Species::AnemoneRoseumBioluminescent
            | Species::AnemoneRoseum
            | Species::AnemoneBlattinus
            | Species::AnemoneBlatteumBioluminescent
            | Species::AnemoneLuteus
            | Species::AnemoneLuteolum
            | Species::AnemoneRubens
            | Species::AnemoneRubeumBioluminescent
            | Species::AnemoneCroceus
            | Species::AnemoneCroceum => Genus::Anemone,

            Species::BarkMound => Genus::BarkMound,

            Species::BacteriumNebulus
            | Species::BacteriumAcies
            | Species::BacteriumOmentum
            | Species::BacteriumScopulum
            | Species::BacteriumVerrata
            | Species::BacteriumBullaris
            | Species::BacteriumAlcyoneum
            | Species::BacteriumVesicula
            | Species::BacteriumCerbrus
            | Species::BacteriumAurasus
            | Species::BacteriumInformem
            | Species::BacteriumVolu
            | Species::BacteriumTela => Genus::Bacterium,

            Species::BrainTreeAureum
            | Species::BrainTreeOstrinum
            | Species::BrainTreePuniceum
            | Species::BrainTreeLindigoticum
            | Species::BrainTreeGypseeum
            | Species::BrainTreeLividum
            | Species::BrainTreeViride
            | Species::BrainTreeRoseum => Genus::BrainTree,

            Species::CactoidaLapis
            | Species::CactoidaPullulanta
            | Species::CactoidaCortexum
            | Species::CactoidaVermis
            | Species::CactoidaPeperatis => Genus::Cactoida,

            Species::CrystallineShards => Genus::CrystallineShards,

            Species::ClypeusSpeculumi | Species::ClypeusLacrimam | Species::ClypeusMargaritus => {
                Genus::Clypeus
            }

            Species::ConchaRenibus
            | Species::ConchaAureolas
            | Species::ConchaLabiata
            | Species::ConchaBiconcavis => Genus::Concha,

            Species::ElectricaePluma | Species::ElectricaeRadialem => Genus::Electricae,

            Species::FonticuluaCampestris
            | Species::FonticuluaSegmentatus
            | Species::FonticuluaDigitos
            | Species::FonticuluaUpupam
            | Species::FonticuluaLapida
            | Species::FonticuluaFluctus => Genus::Fonticulua,

            Species::FrutexaFlabellum
            | Species::FrutexaFlammasis
            | Species::FrutexaMetallicum
            | Species::FrutexaAcus
            | Species::FrutexaFera
            | Species::FrutexaSponsae
            | Species::FrutexaCollum => Genus::Fruxeta,

            Species::FumerolaAquatis
            | Species::FumerolaCarbosis
            | Species::FumerolaExtremus
            | Species::FumerolaNitris => Genus::Fumerola,

            Species::FungoidaSetisis
            | Species::FungoidaGelata
            | Species::FungoidaBullarum
            | Species::FungoidaStabitis => Genus::Fungoida,

            Species::OsseusFractus
            | Species::OsseusSpiralis
            | Species::OsseusCornibus
            | Species::OsseusPumice
            | Species::OsseusPellebantus
            | Species::OsseusDiscus => Genus::Osseus,

            Species::ReceptaUmbrux | Species::ReceptaDeltahedronix | Species::ReceptaConditivus => {
                Genus::Recepta
            }

            Species::SinuousTubersAlbidum
            | Species::SinuousTubersBlatteum
            | Species::SinuousTubersCaeruleum
            | Species::SinuousTubersLindigoticum
            | Species::SinuousTubersPrasinum
            | Species::SinuousTubersRoseum
            | Species::SinuousTubersViolaceum
            | Species::SinuousTubersViride => Genus::SinuousTubers,

            Species::StratumTectonicas
            | Species::StratumPaleas
            | Species::StratumFrigus
            | Species::StratumLaminamus
            | Species::StratumExcutitus
            | Species::StratumLimaxus
            | Species::StratumCucumisis
            | Species::StratumAraneamus => Genus::Stratum,

            Species::TubusConifer
            | Species::TubusSororibus
            | Species::TubusRosarium
            | Species::TubusCavas
            | Species::TubusCompagibus => Genus::Tubus,

            Species::TussockPennata
            | Species::TussockVentusa
            | Species::TussockIgnis
            | Species::TussockCultro
            | Species::TussockSerrati
            | Species::TussockAlbata
            | Species::TussockDivisa
            | Species::TussockCaputus
            | Species::TussockTriticum
            | Species::TussockStigmasis
            | Species::TussockCapillum
            | Species::TussockCatena
            | Species::TussockPropagito
            | Species::TussockPennatis
            | Species::TussockVirgam => Genus::Tussock,
        }
    }
}

impl From<Species> for Genus {
    fn from(value: Species) -> Self {
        (&value).into()
    }
}

impl Display for Genus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Genus::Aleoida => "Aleoida",
                Genus::AmphoraPlant => "Amphora Plant",
                Genus::Anemone => "Anemone",
                Genus::BarkMound => "Bark Mound",
                Genus::Bacterium => "Bacterium",
                Genus::BrainTree => "Brain Tree",
                Genus::Cactoida => "Cactoida",
                Genus::Clypeus => "Clypeus",
                Genus::Concha => "Concha",
                Genus::CrystallineShards => "Crystalline Shards",
                Genus::Electricae => "Electricae",
                Genus::Fonticulua => "Fonticulua",
                Genus::Fruxeta => "Fruxeta",
                Genus::Fumerola => "Fumerola",
                Genus::Fungoida => "Fungoida",
                Genus::Osseus => "Osseus",
                Genus::Recepta => "Recepta",
                Genus::SinuousTubers => "Sinuous Tubers",
                Genus::Stratum => "Stratum",
                Genus::Tubus => "Tubus",
                Genus::Tussock => "Tussock",
                Genus::Trutexa => "Trutexa",

                #[cfg(not(feature = "strict"))]
                Genus::Unknown(unknown) => return write!(f, "Unknown genus: {}", unknown),
            }
        )
    }
}

impl Genus {
    /// The minimum distance in meters required between two samples.
    pub fn minimum_distance(&self) -> u16 {
        match self {
            Genus::Aleoida => 150,
            Genus::AmphoraPlant => 100,
            Genus::Anemone => 0,
            Genus::BarkMound => 0,
            Genus::Bacterium => 0,
            Genus::BrainTree => 0,
            Genus::Cactoida => 0,
            Genus::Clypeus => 0,
            Genus::Concha => 0,
            Genus::CrystallineShards => 0,
            Genus::Electricae => 0,
            Genus::Fonticulua => 0,
            Genus::Fruxeta => 0,
            Genus::Fumerola => 0,
            Genus::Fungoida => 0,
            Genus::Osseus => 0,
            Genus::Recepta => 0,
            Genus::SinuousTubers => 0,
            Genus::Stratum => 0,
            Genus::Tubus => 0,
            Genus::Tussock => 0,
            Genus::Trutexa => 0,

            #[cfg(not(feature = "strict"))]
            Genus::Unknown(_) => 0,
        }
    }
}
