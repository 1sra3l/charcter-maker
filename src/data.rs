//! The stat data related code
use std::fmt::Debug;

extern crate tini;
use tini::Ini;

use crate::ini::{IniDetails, *};

/// # Various constants to provide autocompletion to ensure correct typing each time!
/// The field name for health points
pub const HP:&str = "hp";
/// The field name for mana points
pub const MP:&str = "mp";
/// The field name for experience points
pub const XP:&str = "xp";
/// The field name for the next amount of experience points to level
pub const XP_NEXT:&str = "xp_next";
/// The field name for gold points
pub const GP:&str = "gp";
/// The field name for the current level
pub const LEVEL:&str = "level";
/// The field name for the speed
pub const SPEED:&str = "speed";
/// The field name for the attack amount
pub const ATK:&str = "atk";
/// The field name for the defense amount
pub const DEF:&str = "def";
/// The field name for the mana attack amount
pub const M_ATK:&str = "m_atk";
/// The field name for the mana defense amount
pub const M_DEF:&str = "m_def";
/// The field name for the type of creature
pub const TYPE:&str = "type";
/// The field name for the class
pub const CLASS:&str = "class";
/// The field name for the image filename
pub const IMAGE:&str = "image";
/// The field name for the clan
pub const CLAN:&str = "clan";
/// The field name for mana weakness
pub const M_WEAK:&str = "m_weak";
/// The field name for mana strength
pub const M_STRONG:&str = "m_strong";
/// The field name for mana attacks
pub const M_ATTACKS:&str = "m_attacks";
/// The field name for the Mana type
pub const M_TYPE:&str = "m_type";


#[derive(Debug, Clone)]
pub struct Stats {
    /// Experience Points
    pub xp:f64,
    /// Health Points
    pub hp:f64,
    /// Mana Points
    pub mp:f64,
    /// Attack
    pub atk:f64,
    /// Defense
    pub def:f64,
    /// Mana Attack
    pub m_atk:f64,
    /// Mana Defense
    pub m_def:f64,
    /// Experience Points multiplier for next level
    pub xp_next:f64,
    /// Max Health Points
    pub hp_max:f64,
    /// Max Mana Points
    pub mp_max:f64,
    /// Current Level
    pub level:f64,
    /// The speed
    pub speed:f64,
    /// your currency points
    pub gp:f64,
    /// The name
    pub name:String,
    /// The image filename
    pub image:String,
    /// The class
    pub class:String,
    /// The type
    pub c_type:String,
    /// Ini details
    pub ini_details:IniDetails,
    /// The clan
    pub clan:String,
    /// The mana this character is weak against
    pub m_weak:String,
    /// The mana this character is strong against
    pub m_strong:String,
    /// The mana attacks
    pub m_attacks:Vec<String>,
    /// The Mana type
    pub m_type:String,
}
impl Stats {
/// Save the Stats to the ini file
    pub fn save(&self, save_filename:String) -> bool {
        let test_ini = match Ini::from_file(&save_filename) {
            Ok(test_ini) => test_ini,
            Err(e) => {
                println!("save ERROR: {:?}, could be just a blank file",e);
                Ini::new()
            },
        };
        let conf = test_ini;
        if conf.section(self.name.to_owned())
                                  .item(HP,self.hp_max)
                                  .item(MP,self.mp_max)
                                  .item(XP,self.xp)
                                  .item(GP,self.gp)
                                  .item(LEVEL,self.level)
                                  .item(ATK,self.atk)
                                  .item(DEF,self.def)
                                  .item(M_ATK,self.m_atk)
                                  .item(M_DEF,self.m_def)
                                  .item(SPEED,self.speed)
                                  .item(TYPE,self.c_type.to_owned())
                                  .item(CLASS,self.class.to_owned())
                                  .item(IMAGE,self.image.to_owned())
                                  .item(CLAN,self.clan.to_owned())
                                  .item(M_WEAK,self.m_weak.to_owned())
                                  .item(M_STRONG,self.m_strong.to_owned())
                                  .item_vec_with_sep(M_ATTACKS, &self.m_attacks.clone(), ",")
                                  .item(M_TYPE,self.m_type.to_owned())
                                  .to_file(&save_filename.to_owned())
                                  .is_err() { return false }
        true
    }
    /// read stats from `IniDetails`
    pub fn load(ini_details:IniDetails) -> Self where Self:Sized {
        // these are the stats with some sane defaults
        let gp:f64 = get_or_zero_f64(GP, ini_details.clone());
        let xp_next:f64 = get_or_zero_f64(XP_NEXT, ini_details.clone());
        let xp:f64 = get_or_zero_f64(XP, ini_details.clone());
        let hp:f64 = get_or_zero_f64(HP, ini_details.clone());
        let mp:f64 = get_or_zero_f64(MP, ini_details.clone());
        let atk:f64 = get_or_zero_f64(ATK, ini_details.clone());
        let def:f64 = get_or_zero_f64(DEF, ini_details.clone());
        let m_atk:f64 = get_or_zero_f64(M_ATK, ini_details.clone());
        let m_def:f64 = get_or_zero_f64(M_DEF, ini_details.clone());
        let level:f64 = get_or_zero_f64(LEVEL, ini_details.clone());
        let speed:f64 = get_or_zero_f64(SPEED, ini_details.clone());
        let name:String = ini_details.clone().section.to_owned();
        let image:String = get_or_default(IMAGE, ini_details.clone());
        let class:String = get_or_default(CLASS, ini_details.clone());
        let c_type:String = get_or_default(TYPE, ini_details.clone());
        let clan:String = get_or_default(CLAN, ini_details.clone());
        let m_weak:String = get_or_default(M_WEAK, ini_details.clone());
        let m_strong:String = get_or_default(M_STRONG, ini_details.clone());
        //TODO
        let m_attacks:Vec<String> = get_vec(M_ATTACKS, ini_details.clone());
        let m_type:String = get_or_default(M_TYPE, ini_details.clone());
        Stats {
            xp:xp,
            xp_next:xp_next,
            mp:mp,
            hp:hp,
            atk:atk,
            def:def,
            m_atk:m_atk,
            m_def:m_def,
            mp_max:mp,
            hp_max:hp,
            level:level,
            speed:speed,
            gp:gp,
            name:name,
            image:image,
            class:class,
            c_type:c_type,
            ini_details:ini_details.clone(),
            clan:clan,
            m_weak:m_weak,
            m_strong:m_strong,
            m_attacks:m_attacks,
            m_type:m_type,
        }
    }

    /// make empty stats
    pub fn empty() -> Self where Self:Sized {
        Stats {
            xp:0.0,
            xp_next:0.0,
            mp:0.0,
            hp:0.0,
            mp_max:0.0,
            hp_max:0.0,
            atk:0.0,
            def:0.0,
            m_atk:0.0,
            m_def:0.0,
            level:0.0,
            speed:0.0,
            gp:0.0,
            name:String::from(""),
            image:String::from(""),
            class:String::from(""),
            c_type:String::from(""),
            ini_details:IniDetails::empty(),
            clan:String::from(""),
            m_weak:String::from(""),
            m_strong:String::from(""),
            m_attacks:vec![],
            m_type:String::from(""),
        }
    }
}

impl Default for Stats {
    /// By default create an empty struct
    fn default() -> Self {
        Self::empty()
    }
}
