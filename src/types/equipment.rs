// Equipment data structures (weapons, shields, gadgets)
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Weapon {
    pub weapon_type: WeaponType,
    pub power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WeaponType {
    PulseLaser = 0,
    BeamLaser = 1,
    MilitaryLaser = 2,
    MorgansLaser = 3,
    PhotonDisruptor = 4,
    QuantumDisruptor = 5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Shield {
    pub shield_type: ShieldType,
    pub power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShieldType {
    EnergyShield = 0,
    ReflectiveShield = 1,
    LightningShield = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Gadget {
    pub gadget_type: GadgetType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GadgetType {
    ExtraCargoBays = 0,
    AutoRepairSystem = 1,
    NavigatingSystem = 2,
    TargetingSystem = 3,
    CloakingDevice = 4,
    FuelCompactor = 5,
}

impl WeaponType {
    pub fn name(&self) -> &'static str {
        match self {
            WeaponType::PulseLaser => "Pulse Laser",
            WeaponType::BeamLaser => "Beam Laser",
            WeaponType::MilitaryLaser => "Military Laser",
            WeaponType::MorgansLaser => "Morgan's Laser",
            WeaponType::PhotonDisruptor => "Photon Disruptor",
            WeaponType::QuantumDisruptor => "Quantum Disruptor",
        }
    }
    
    pub fn power(&self) -> i32 {
        match self {
            WeaponType::PulseLaser => 15,
            WeaponType::BeamLaser => 25,
            WeaponType::MilitaryLaser => 35,
            WeaponType::MorgansLaser => 85,
            WeaponType::PhotonDisruptor => 60,
            WeaponType::QuantumDisruptor => 100,
        }
    }
    
    pub fn price(&self) -> i32 {
        match self {
            WeaponType::PulseLaser => 2000,
            WeaponType::BeamLaser => 12500,
            WeaponType::MilitaryLaser => 35000,
            WeaponType::MorgansLaser => 50000,
            WeaponType::PhotonDisruptor => 150000,
            WeaponType::QuantumDisruptor => 500000,
        }
    }
}

impl ShieldType {
    pub fn name(&self) -> &'static str {
        match self {
            ShieldType::EnergyShield => "Energy Shield",
            ShieldType::ReflectiveShield => "Reflective Shield",
            ShieldType::LightningShield => "Lightning Shield",
        }
    }
    
    pub fn power(&self) -> i32 {
        match self {
            ShieldType::EnergyShield => 100,
            ShieldType::ReflectiveShield => 200,
            ShieldType::LightningShield => 350,
        }
    }
    
    pub fn price(&self) -> i32 {
        match self {
            ShieldType::EnergyShield => 5000,
            ShieldType::ReflectiveShield => 20000,
            ShieldType::LightningShield => 45000,
        }
    }
}

impl GadgetType {
    pub fn name(&self) -> &'static str {
        match self {
            GadgetType::ExtraCargoBays => "5 Extra Cargo Bays",
            GadgetType::AutoRepairSystem => "Auto-Repair System",
            GadgetType::NavigatingSystem => "Navigating System",
            GadgetType::TargetingSystem => "Targeting System",
            GadgetType::CloakingDevice => "Cloaking Device",
            GadgetType::FuelCompactor => "Fuel Compactor",
        }
    }
    
    pub fn price(&self) -> i32 {
        match self {
            GadgetType::ExtraCargoBays => 2500,
            GadgetType::AutoRepairSystem => 7500,
            GadgetType::NavigatingSystem => 15000,
            GadgetType::TargetingSystem => 25000,
            GadgetType::CloakingDevice => 100000,
            GadgetType::FuelCompactor => 30000,
        }
    }
}
