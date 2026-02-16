mod messages_en;
mod messages_fr;

use messages_en::MESSAGES_EN;
use messages_fr::MESSAGES_FR;
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Fr,
}

pub struct I18n {
    messages_en: &'static HashMap<&'static str, &'static str>,
    messages_fr: &'static HashMap<&'static str, &'static str>,
    default_locale: Locale,
}

impl I18n {
    pub fn new(default_locale: Locale) -> Self {
        I18n {
            messages_en: &MESSAGES_EN,
            messages_fr: &MESSAGES_FR,
            default_locale,
        }
    }

    pub fn text(&self, locale: Locale, key: &str) -> String {
        let candidate = match locale {
            Locale::En => self.messages_en.get(key).copied(),
            Locale::Fr => self
                .messages_fr
                .get(key)
                .copied()
                .or_else(|| self.messages_en.get(key).copied()),
        };

        candidate.unwrap_or_else(|| key).to_string()
    }

    pub fn text_default(&self, key: &str) -> String {
        self.text(self.default_locale, key)
    }
}

pub static I18N: Lazy<I18n> = Lazy::new(|| I18n::new(Locale::En));
