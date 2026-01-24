use serde::{Deserialize, Serialize};
use smithay::input::keyboard::XkbConfig;
use xkbcommon::xkb::Keysym;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct KeybindShortcut {
    #[serde(with = "keysym_serde")]
    pub key: Keysym,
    #[serde(deserialize_with = "modifier_mask_serde::deserialize")]
    #[serde(default)]
    pub modifiers: u8
}

pub trait ToMask {
    fn to_mask(&self) -> u8;
}

impl ToMask for [KeyboardModifier] {
    fn to_mask(&self) -> u8 {
        self.iter().fold(0u8, |acc, m| acc | (*m as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyboardModifier {
    Alt =   1 << 0,
    Ctrl =  1 << 1,
    Shift = 1 << 2,
    Logo =  1 << 3
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeybindAction {
    Kill,
    Exec(String),
    CycleNext,
    CyclePrev,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybind {
    pub shortcut: KeybindShortcut,
    pub action: KeybindAction
}


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub keybinds: Vec<Keybind>,
    #[serde(default)]
    pub keyboard: KeyboardConfig,
    #[serde(default)]
    pub remaps: Vec<KeyboardRemap>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyboardRemap {
    #[serde(with = "keysym_serde")]
    pub from: Keysym,
    #[serde(with = "keysym_serde")]
    pub into: Keysym
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KeyboardConfig {
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: Option<String>
}

impl<'a> From<&'a KeyboardConfig> for XkbConfig<'a> {
    fn from(cfg: &'a KeyboardConfig) -> XkbConfig<'a> {
        XkbConfig {
            rules: &cfg.rules,
            model: &cfg.model,
            layout: &cfg.layout,
            variant: &cfg.variant,
            options: cfg.options.clone(),
        }
    }
}


impl KeybindShortcut {
    pub fn new_verbose(key: impl Into<Keysym>, alt: bool, ctrl: bool, shift: bool, logo: bool) -> Self {
        Self {
            key: key.into(),
            modifiers: alt as u8 | (ctrl as u8) << 1 | (shift as u8) << 2 | (logo as u8) << 3
        }
    }

    pub fn new(key: impl Into<Keysym>, modifiers: Vec<KeyboardModifier>) -> Self {
        Self {
            key: key.into(),
            modifiers: modifiers.iter().fold(0u8, |acc, m| acc | (*m as u8))
        }
    }
}



mod keysym_serde {
    use serde::{Serializer, Deserializer, de::{Error, Visitor}};
    use xkbcommon::xkb::{KEYSYM_CASE_INSENSITIVE, KEYSYM_NO_FLAGS, Keysym, keysym_from_name};
    use std::fmt;

    pub fn serialize<S>(key: &Keysym, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(key.raw())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Keysym, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(KeysymVisitor)
    }

    // https://kolulu23.github.io/serde-rs.github.io/examples/string-or-struct/
    struct KeysymVisitor;

    impl<'de> Visitor<'de> for KeysymVisitor {
        type Value = Keysym;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "a u32, char or key name")
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E> {
            Ok(Keysym::new(v))
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Keysym::new(v as u32))
        }

        fn visit_char<E>(self, v: char) -> Result<Self::Value, E> {
            Ok(Keysym::from_char(v))
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error, {
            let mut key = keysym_from_name(&v, KEYSYM_NO_FLAGS);    // without this if user provides character as "Q" or "q" it'd return XK_q instead of XK_Q, which causes keybind to not work
            if key != Keysym::NoSymbol {
                return Ok(key)
            }
            key = keysym_from_name(&v, KEYSYM_CASE_INSENSITIVE);
            if key == Keysym::NoSymbol {
                return Err(Error::custom("Invalid key name"))
            }
            Ok(key)
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: Error, {
            let mut key = keysym_from_name(&v, KEYSYM_NO_FLAGS);
            if key != Keysym::NoSymbol {
                return Ok(key)
            }
            key = keysym_from_name(&v, KEYSYM_CASE_INSENSITIVE);
            if key == Keysym::NoSymbol {
                return Err(Error::custom("Invalid key name"))
            }
            Ok(key)
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: Error, {
            let mut key = keysym_from_name(&v, KEYSYM_NO_FLAGS);
            if key != Keysym::NoSymbol {
                return Ok(key)
            }
            key = keysym_from_name(&v, KEYSYM_CASE_INSENSITIVE);
            if key == Keysym::NoSymbol {
                return Err(Error::custom("Invalid key name"))
            }
            Ok(key)
        }
    }

}

mod modifier_mask_serde {
    use serde::{Deserializer, Deserialize};
    use crate::config::KeyboardModifier;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mods: Vec<KeyboardModifier> = Vec::deserialize(deserializer)?;
        Ok(mods.iter().fold(0u8, |acc, m| acc | (*m as u8)))
    }
}