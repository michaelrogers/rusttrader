// Solar system data structures

use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarSystem {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub tech_level: TechLevel,
    pub politics: Politics,
    pub size: SystemSize,
    pub special_resource: SpecialResource,
    pub special_event: i32, // Special event ID (-1 for none)
    pub visited: bool,
    pub price_increase: [i32; 10], // Price increases for trade goods
    pub qty: [i32; 10], // Quantities available for trade goods
    pub price_history: Vec<Vec<i32>>, // Per-good price history (last few prices)
    pub news: Vec<String>, // Local news headlines affecting prices
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TechLevel {
    PreAgricultural = 0,
    Agricultural = 1,
    Medieval = 2,
    Renaissance = 3,
    EarlyIndustrial = 4,
    Industrial = 5,
    PostIndustrial = 6,
    HiTech = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Politics {
    Anarchy = 0,
    Capitalist = 1,
    Communist = 2,
    Confederacy = 3,
    Corporate = 4,
    Cybernetic = 5,
    Democracy = 6,
    Dictatorship = 7,
    Fascist = 8,
    Feudal = 9,
    Military = 10,
    Monarchy = 11,
    Pacifist = 12,
    Socialist = 13,
    Satori = 14,
    Technocracy = 15,
    Theocracy = 16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemSize {
    Tiny = 0,
    Small = 1,
    Medium = 2,
    Large = 3,
    Huge = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpecialResource {
    Nothing = 0,
    MineralRich = 1,
    MineralPoor = 2,
    Desert = 3,
    LotsOfWater = 4,
    RichSoil = 5,
    PoorSoil = 6,
    RichFauna = 7,
    Lifeless = 8,
    WeirdMushrooms = 9,
    LotsOfHerbs = 10,
    Artistic = 11,
    Warlike = 12,
}

impl SolarSystem {
    /// Generate the galaxy - ported from Traveler.c StartNewGame()
    pub fn generate_galaxy() -> Vec<Self> {
        let mut rng = rand::thread_rng();
        let mut systems: Vec<Self> = Vec::with_capacity(120);
        
        // System names from the original game
        let names = SYSTEM_NAMES;
        
        for i in 0..120 {
            let mut x;
            let mut y;
            let mut too_close;
            
            // Keep generating coordinates until we find one not too close to existing systems
            loop {
                if i < 6 {
                    // First 6 systems (wormholes) placed near center
                    x = rng.gen_range(40..110);
                    y = rng.gen_range(40..110);
                } else {
                    x = rng.gen_range(0..150);
                    y = rng.gen_range(0..150);
                }
                
                // Check if too close to existing systems
                too_close = false;
                for existing in &systems {
                    let dx = x - existing.x;
                    let dy = y - existing.y;
                    let dist_squared = dx * dx + dy * dy;
                    if dist_squared < 36 { // 6^2 = 36 minimum distance
                        too_close = true;
                        break;
                    }
                }
                
                if !too_close {
                    break;
                }
            }
            
            let tech_level = match rng.gen_range(0..8) {
                0 => TechLevel::PreAgricultural,
                1 => TechLevel::Agricultural,
                2 => TechLevel::Medieval,
                3 => TechLevel::Renaissance,
                4 => TechLevel::EarlyIndustrial,
                5 => TechLevel::Industrial,
                6 => TechLevel::PostIndustrial,
                _ => TechLevel::HiTech,
            };
            
            let politics = match rng.gen_range(0..17) {
                0 => Politics::Anarchy,
                1 => Politics::Capitalist,
                2 => Politics::Communist,
                3 => Politics::Confederacy,
                4 => Politics::Corporate,
                5 => Politics::Cybernetic,
                6 => Politics::Democracy,
                7 => Politics::Dictatorship,
                8 => Politics::Fascist,
                9 => Politics::Feudal,
                10 => Politics::Military,
                11 => Politics::Monarchy,
                12 => Politics::Pacifist,
                13 => Politics::Socialist,
                14 => Politics::Satori,
                15 => Politics::Technocracy,
                _ => Politics::Theocracy,
            };
            
            let size = match rng.gen_range(0..5) {
                0 => SystemSize::Tiny,
                1 => SystemSize::Small,
                2 => SystemSize::Medium,
                3 => SystemSize::Large,
                _ => SystemSize::Huge,
            };
            
            let special_resource = if rng.gen_range(0..4) == 0 {
                match rng.gen_range(0..13) {
                    0 => SpecialResource::Nothing,
                    1 => SpecialResource::MineralRich,
                    2 => SpecialResource::MineralPoor,
                    3 => SpecialResource::Desert,
                    4 => SpecialResource::LotsOfWater,
                    5 => SpecialResource::RichSoil,
                    6 => SpecialResource::PoorSoil,
                    7 => SpecialResource::RichFauna,
                    8 => SpecialResource::Lifeless,
                    9 => SpecialResource::WeirdMushrooms,
                    10 => SpecialResource::LotsOfHerbs,
                    11 => SpecialResource::Artistic,
                    _ => SpecialResource::Warlike,
                }
            } else {
                SpecialResource::Nothing
            };
            
            systems.push(SolarSystem {
                name: names[i].to_string(),
                x,
                y,
                tech_level,
                politics,
                size,
                special_resource,
                special_event: -1,
                visited: false,
                price_increase: [0; 10],
                qty: [100; 10], // Start with 100 of each trade good
                price_history: vec![Vec::new(); 10],
                news: Vec::new(),
            });
        }
        
        systems
    }
    
    pub fn distance_to(&self, other: &SolarSystem) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

/// System names from the original game (Global.c)
const SYSTEM_NAMES: &[&str] = &[
    "Acamar", "Adahn", "Aldea", "Andevian", "Antedi", "Balosnee",
    "Baratas", "Brax", "Bretel", "Calondia", "Campor", "Capelle",
    "Carzon", "Castor", "Cestus", "Cheron", "Courteney", "Daled",
    "Damast", "Davlos", "Deneb", "Deneva", "Devidia", "Draylon",
    "Drema", "Endor", "Esmee", "Exo", "Ferris", "Festen",
    "Fourmi", "Frolix", "Gemulon", "Guinifer", "Hades", "Hamlet",
    "Helena", "Hulst", "Iodine", "Iralius", "Janus", "Japori",
    "Jarada", "Jason", "Kaylon", "Khefka", "Kira", "Klaatu",
    "Klaestron", "Korma", "Kravat", "Krios", "Laertes", "Largo",
    "Lave", "Ligon", "Lowry", "Magrat", "Malcoria", "Melina",
    "Mentar", "Merik", "Mintaka", "Montor", "Mordan", "Myrthe",
    "Nelvana", "Nix", "Nyle", "Odet", "Og", "Omega",
    "Omphalos", "Orias", "Othello", "Parade", "Penthara", "Picard",
    "Pollux", "Quator", "Rakhar", "Ran", "Regulas", "Relva",
    "Rhymus", "Rochani", "Rubicum", "Rutia", "Sarpeidon", "Sefalla",
    "Seltrice", "Sigma", "Sol", "Somari", "Stakoron", "Styris",
    "Talani", "Tamus", "Tantalos", "Tanuga", "Tarchannen", "Terosa",
    "Thera", "Titan", "Torin", "Triacus", "Turkana", "Tyrus",
    "Umberlee", "Utopia", "Vadera", "Vagra", "Vandor", "Ventax",
    "Xenon", "Xerxes", "Yew", "Yojimbo", "Zalkon", "Zuul",
];
