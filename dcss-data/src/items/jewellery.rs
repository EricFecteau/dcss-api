use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) struct Jewellery {
    pub(crate) data_collected: bool,
    pub(crate) jewellery_type: JewelleryType,
    pub(crate) amulet_type: AmuletType,
    pub(crate) ring_type: RingType,
    pub(crate) rating: i32,
}

#[derive(Clone, Debug)]
pub(crate) enum JewelleryType {
    Unknown,
    Amulet,
    Ring,
}

#[derive(Clone, Debug)]
pub(crate) enum AmuletType {
    Unknown,
    Spirit,
    Acrobat,
    Faith,
    RegenMP,
    Reflect,
    Regen,
}

#[derive(Clone, Debug)]
pub(crate) enum RingType {
    Unknown,
    MPP9,
    SlayP4,
    SeeInv,
    IntP6,
    StrP6,
    ACP4,
    FireResistance,
    Wiz,
    DexP6,
    NegativeResistance,
    EvasionP4,
    EvasionP5,
    Ice,
    Fire,
    ColdResistance,
    Flight,
    CorrosionResistance,
    WillPower,
    PoisonResistance,
}

impl Jewellery {
    pub(crate) fn new() -> Self {
        Self {
            data_collected: false,
            jewellery_type: JewelleryType::Unknown,
            amulet_type: AmuletType::Unknown,
            ring_type: RingType::Unknown,
            rating: -100,
        }
    }

    pub(crate) fn update_jewellery(&mut self, jewellery_desc: Value) {
        self.data_collected = true;

        let title = jewellery_desc["title"].to_string();
        let body = jewellery_desc["body"].to_string();

        if title.contains("amulet") {
            self.jewellery_type = JewelleryType::Amulet;
            self.amulet_type = amulet_type(body);
            self.rating = self.amulet_rating();
        } else if title.contains("ring") {
            self.jewellery_type = JewelleryType::Ring;
            self.ring_type = ring_type(body);
            self.rating = self.ring_rating();
        } else {
            unimplemented!("No jewellery type");
        }
    }

    pub(crate) fn amulet_rating(&self) -> i32 {
        // TODO: Get a better way to rate amulets;
        match self.amulet_type {
            AmuletType::Regen => 4,
            AmuletType::Spirit => 3,
            AmuletType::Reflect => 2,
            AmuletType::Acrobat => 1,
            AmuletType::RegenMP => 0,
            AmuletType::Faith => -100,
            _ => unimplemented!("Failed to identify the amulet type"),
        }
    }

    pub(crate) fn ring_rating(&self) -> i32 {
        // TODO: Get a better way to rate rings;
        match self.ring_type {
            RingType::MPP9 => 1,
            RingType::PoisonResistance => 2,
            RingType::SlayP4 => 3,
            RingType::SeeInv => 4,
            RingType::IntP6 => 5,
            RingType::StrP6 => 6,
            RingType::ACP4 => 7,
            RingType::FireResistance => 8,
            RingType::Wiz => 9,
            RingType::DexP6 => 10,
            RingType::NegativeResistance => 11,
            RingType::EvasionP5 => 12,
            RingType::EvasionP4 => 13,
            RingType::Ice => 14,
            RingType::Fire => 15,
            RingType::ColdResistance => 16,
            RingType::Flight => 17,
            RingType::CorrosionResistance => 18,
            RingType::WillPower => 19,
            _ => unimplemented!("Failed to identify the ring type"),
        }
    }
}

pub(crate) fn amulet_type(amulet_desc: String) -> AmuletType {
    if amulet_desc.contains("{Spirit}") {
        return AmuletType::Spirit;
    } else if amulet_desc.contains("{Acrobat}") {
        return AmuletType::Acrobat;
    } else if amulet_desc.contains("{Reflect}") {
        return AmuletType::Reflect;
    } else if amulet_desc.contains("{Regen}") {
        return AmuletType::Regen;
    } else if amulet_desc.contains("{RegenMP}") {
        return AmuletType::RegenMP;
    } else if amulet_desc.contains("{Faith}") {
        return AmuletType::Faith;
    }

    unimplemented!("Failed to identify the amulet type");
}

pub(crate) fn ring_type(ring_desc: String) -> RingType {
    // Implement looking at "useless_item" tag

    if ring_desc.contains("{Slay+4}") {
        return RingType::SlayP4;
    } else if ring_desc.contains("{sInv}") {
        return RingType::SeeInv;
    } else if ring_desc.contains("{Int+6}") {
        return RingType::IntP6;
    } else if ring_desc.contains("{Str+6}") {
        return RingType::StrP6;
    } else if ring_desc.contains("{rF+}") {
        return RingType::FireResistance;
    } else if ring_desc.contains("{Wiz}") {
        return RingType::Wiz;
    } else if ring_desc.contains("{Dex+6}") {
        return RingType::DexP6;
    } else if ring_desc.contains("{rN+}") {
        return RingType::NegativeResistance;
    } else if ring_desc.contains("{EV+4}") {
        return RingType::EvasionP4;
    } else if ring_desc.contains("{EV+5}") {
        return RingType::EvasionP5;
    } else if ring_desc.contains("{Ice rC+ rF-}") {
        return RingType::Ice;
    } else if ring_desc.contains("{Fire rF+ rC-}") {
        return RingType::Fire;
    } else if ring_desc.contains("{rC+}") {
        return RingType::ColdResistance;
    } else if ring_desc.contains("{Fly}") {
        return RingType::Flight;
    } else if ring_desc.contains("{rCorr}") {
        return RingType::CorrosionResistance;
    } else if ring_desc.contains("{Will+}") {
        return RingType::WillPower;
    } else if ring_desc.contains("{rPois}") {
        return RingType::PoisonResistance;
    } else if ring_desc.contains("{AC+4}") {
        return RingType::ACP4;
    } else if ring_desc.contains("{MP+9}") {
        return RingType::MPP9;
    }

    unimplemented!("Failed to identify the ring type")
}
