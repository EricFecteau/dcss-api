pub(crate) mod armours;
pub(crate) mod jewellery;
pub(crate) mod missiles;
pub(crate) mod potions;
pub(crate) mod scrolls;
pub(crate) mod staves;
pub(crate) mod wands;
pub(crate) mod weapons;

use armours::{Armour, ArmourType};
use jewellery::{AmuletType, Jewellery, RingType};
use missiles::Missile;
use potions::Potion;
use potions::PotionType;
use scrolls::Scroll;
use scrolls::ScrollType;
use staves::Staff;
use wands::Wand;
use weapons::Weapon;

#[derive(Clone, Debug)]
pub(crate) enum Item {
    None,
    Weapon(Weapon),
    Missile(Missile),
    Armour(Armour),
    Wand(Wand),
    _Unknown4, // Unknown item
    Scroll(Scroll),
    Jewellery(Jewellery),
    Potion(Potion),
    _Unknown8, // Unknown item
    Staff(Staff),
}

#[derive(Clone, Debug)]
pub(crate) enum WearSlots {
    Weapon,
    Cloak,
    Helmet,
    Gloves,
    Boots,
    Shield,
    Body,
    _RingLeft,
    _RingRight,
    Amulet,
    _Unknown4,
}

impl Item {
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub(crate) fn data_collected(&self) -> bool {
        match self {
            Self::None => false,
            Self::Weapon(item) => item.data_collected,
            Self::Missile(item) => item.data_collected,
            Self::Armour(item) => item.data_collected,
            Self::Wand(item) => item.data_collected,
            Self::_Unknown4 => unimplemented!(),
            Self::Scroll(item) => item.data_collected,
            Self::Jewellery(item) => item.data_collected,
            Self::Potion(item) => item.data_collected,
            Self::_Unknown8 => unimplemented!(),
            Self::Staff(item) => item.data_collected,
        }
    }

    pub(crate) fn rating(&self) -> i32 {
        match self {
            Self::Weapon(item) => item.rating,
            Self::Armour(item) => item.rating,
            Self::Jewellery(item) => item.rating,
            _ => -100,
        }
    }

    pub(crate) fn _armour_type(&self) -> ArmourType {
        match self {
            Self::Armour(item) => item.armour_type.clone(),
            _ => ArmourType::None,
        }
    }

    pub(crate) fn _amulet_type(&self) -> AmuletType {
        match self {
            Self::Jewellery(item) => item.amulet_type.clone(),
            _ => AmuletType::Unknown,
        }
    }

    pub(crate) fn _scroll_type(&self) -> ScrollType {
        match self {
            Self::Scroll(item) => item.scroll_type.clone(),
            _ => ScrollType::Unknown,
        }
    }

    pub(crate) fn potion_type(&self) -> PotionType {
        match self {
            Self::Potion(item) => item.potion_type.clone(),
            _ => PotionType::Unknown,
        }
    }

    pub(crate) fn _ring_type(&self) -> RingType {
        match self {
            Self::Jewellery(item) => item.ring_type.clone(),
            _ => RingType::Unknown,
        }
    }

    pub(crate) fn is_identified(&self) -> bool {
        match self {
            Self::Scroll(item) => item.identified,
            Self::Potion(item) => item.identified,
            _ => unreachable!(),
        }
    }

    pub(crate) fn identified(&mut self) {
        match self {
            Self::Scroll(item) => item.identified = true,
            Self::Potion(item) => item.identified = true,
            _ => unreachable!(),
        }
    }
}
