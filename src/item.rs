use std::fmt;
use actor::{self, Slot};
use rand::{self, Rng};

pub trait Item : Send+Sync+fmt::Debug {
    fn description(&self) -> &str;
    fn type_(&self) -> Type;
    fn slot(&self) -> Slot;
    fn clone_item<'a>(&self) -> Box<Item + 'a> where Self: 'a;
    fn stats(&self) -> actor::Stats;
}

impl<'a> Clone for Box<Item+'a> {
    fn clone(&self) -> Box<Item+'a> {
        self.clone_item()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Type {
    Weapon,
    Armor,
}

pub fn random(level : i32) -> Box<Item> {

    let r = rand::thread_rng().gen_range(0i32, 5) +
        rand::thread_rng().gen_range(0, 5) +
        rand::thread_rng().gen_range(0, 5) - 9;


    match level + r {
        1|2 => Weapon::new(weapon::Sword).to_item(),
        3 => Armor::new(armor::Leather).to_item(),
        5 => Weapon::new(weapon::Sword).to_item(),
        7 => Armor::new(armor::Plate).to_item(),
        _ => Weapon::new(weapon::Knife).to_item(),
    }
}

pub use self::weapon::Weapon;
pub use self::armor::Armor;

pub mod weapon {
    use super::Item;
    use super::Type as ItemType;
    use actor::{self, Slot};
    pub use self::Type::*;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Type {
        Knife,
        Sword,
        Axe,
    }


    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Weapon {
        type_ : Type,
    }

    impl Weapon {
        pub fn new(type_ : Type) -> Weapon {
            Weapon {
                type_: type_,
            }
        }

        pub fn to_item(self) -> Box<Item> {
            Box::new(self)
        }
    }

    impl Item for Weapon {
        fn description(&self) -> &str {
            match self.type_ {
                Knife => "knife",
                Sword => "sword",
                Axe => "axe",
            }
        }

        fn type_(&self) -> ItemType {
            ItemType::Weapon
        }

        fn clone_item<'a>(&self) -> Box<Item + 'a> where Self: 'a {
            Box::new(self.clone())
        }

        fn slot(&self) -> Slot {
            Slot::RHand
        }

        fn stats(&self) -> actor::Stats {
            let mut stats = actor::Stats::zero();

            match self.type_ {
                Knife => {
                    stats.melee_dmg += 1;
                },
                Sword => {
                    stats.melee_dmg += 3;
                    stats.melee_cd += 1;
                },
                Axe => {
                    stats.melee_dmg += 4;
                    stats.melee_cd += 2;
                },
            }

            stats
        }
    }
}

pub mod armor {
    use super::Item;
    use super::Type as ItemType;
    use actor::{self, Slot};

    pub use self::Type::*;

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub enum Type {
        Leather,
        Plate,
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct Armor {
        pub type_ : Type,
    }

    impl Armor {
        pub fn new(type_ : Type) -> Armor {
            Armor{
                type_: type_,
            }
        }

        pub fn to_item(self) -> Box<Item> {
            Box::new(self)
        }
    }

    impl Item for Armor {
        fn description(&self) -> &str {
            match self.type_ {
                Type::Plate => "plate armor",
                Type::Leather => "leather armor",
            }
        }

        fn type_(&self) -> ItemType {
            ItemType::Armor
        }

        fn clone_item<'a>(&self) -> Box<Item + 'a> where Self: 'a {
            Box::new(self.clone())
        }

        fn slot(&self) -> Slot {
            Slot::Body
        }

        fn stats(&self) -> actor::Stats {
            let mut stats = actor::Stats::zero();

            match self.type_ {
                Plate => {
                    stats.ac += 4;
                    stats.ev -= 2;
                },
                Leather => {
                    stats.ac += 1;
                },
            }

            stats
        }
    }
}
