//! Public, versioned templates contain presentation only. Links never fetch data
//! or carry startup, monitor selection, update consent or organization settings.
use crate::{Preferences, Theme};
use std::fmt;

pub const APP_PREFIX: &str = "elevenclock://template?";
pub const WEB_PREFIX: &str = "https://tjorim.github.io/ElevenClock-rs/#";
pub const MAX_LINK_BYTES: usize = 4096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClockTemplate {
    format: String,
    font_size_pt: u8,
    theme: Theme,
}

/// Maximum number of nonempty lines in a template. This is a template rule: the
/// format language itself has no line cap (see `FORMAT.md`).
pub const MAX_LINES: usize = 2;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TemplateError {
    /// A link or a format/font size that cannot be a template, or an unsupported version.
    Invalid,
    /// More than [`MAX_LINES`] lines.
    TooManyLines,
    /// The estimated expanded text exceeds the rendered-text limit.
    TooLong,
}
impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Invalid => "This template is invalid or uses an unsupported version.",
            Self::TooManyLines => "Templates support at most two lines.",
            Self::TooLong => "This format expands to too much text for a template.",
        })
    }
}
impl std::error::Error for TemplateError {}

impl ClockTemplate {
    pub fn new(format: &str, font_size_pt: u8, theme: Theme) -> Result<Self, TemplateError> {
        let validated = Preferences {
            format: format.into(),
            font_size_pt,
            theme,
            ..Preferences::default()
        }
        .validate()
        .map_err(|_| TemplateError::Invalid)?;
        let source = validated.format_source();
        if source.split('\n').count() > MAX_LINES {
            return Err(TemplateError::TooManyLines);
        }
        // Match the website's bounded English presentation expansion estimate.
        // Actual taskbar fit is still measured by the app before Apply.
        let mut chars = source.chars();
        let mut expanded = 0;
        while let Some(character) = chars.next() {
            expanded += if character == '%' {
                match chars.next().ok_or(TemplateError::Invalid)? {
                    'A' | 'B' => 9,
                    'a' | 'b' => 3,
                    'Y' | 'G' => 4,
                    '%' | 'u' => 1,
                    _ => 2,
                }
            } else {
                1
            };
        }
        if expanded > crate::format::MAX_RENDERED_SCALARS {
            return Err(TemplateError::TooLong);
        }
        Ok(Self {
            format: validated.format_source().into(),
            font_size_pt,
            theme,
        })
    }
    pub fn format(&self) -> &str {
        &self.format
    }
    pub fn font_size_pt(&self) -> u8 {
        self.font_size_pt
    }
    pub fn theme(&self) -> Theme {
        self.theme
    }

    fn query(&self, version_key: &str) -> String {
        let theme = match self.theme {
            Theme::System => "system",
            Theme::Light => "light",
            Theme::Dark => "dark",
        };
        format!(
            "{version_key}=1&format={}&font={}&theme={theme}",
            encode(&self.format),
            self.font_size_pt
        )
    }
    pub fn app_link(&self) -> String {
        format!("{APP_PREFIX}{}", self.query("v"))
    }
    pub fn share_link(&self) -> String {
        format!("{WEB_PREFIX}{}", self.query("template"))
    }

    pub fn from_link(link: &str) -> Result<Self, TemplateError> {
        if link.len() > MAX_LINK_BYTES {
            return Err(TemplateError::Invalid);
        }
        let (query, version_key) = if let Some(query) = link
            .strip_prefix(APP_PREFIX)
            .or_else(|| link.strip_prefix("elevenclock://template/?"))
        {
            (query, "v")
        } else if let Some(query) = link.strip_prefix(WEB_PREFIX) {
            (query, "template")
        } else {
            return Err(TemplateError::Invalid);
        };
        let mut version = None;
        let mut format = None;
        let mut font = None;
        let mut theme = None;
        for part in query.split('&') {
            let (key, value) = part.split_once('=').ok_or(TemplateError::Invalid)?;
            let slot = match key {
                key if key == version_key => &mut version,
                "format" => &mut format,
                "font" => &mut font,
                "theme" => &mut theme,
                _ => return Err(TemplateError::Invalid),
            };
            if slot.replace(decode(value)?).is_some() {
                return Err(TemplateError::Invalid);
            }
        }
        if version.as_deref() != Some("1") {
            return Err(TemplateError::Invalid);
        }
        let font = font
            .ok_or(TemplateError::Invalid)?
            .parse::<u8>()
            .map_err(|_| TemplateError::Invalid)?;
        let theme = Theme::try_from(theme.ok_or(TemplateError::Invalid)?.as_str())
            .map_err(|_| TemplateError::Invalid)?;
        Self::new(&format.ok_or(TemplateError::Invalid)?, font, theme)
    }
}

fn encode(value: &str) -> String {
    let mut encoded = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
            encoded.push(char::from(byte));
        } else {
            use fmt::Write;
            let _ = write!(encoded, "%{byte:02X}");
        }
    }
    encoded
}
fn decode(value: &str) -> Result<String, TemplateError> {
    let mut bytes = Vec::new();
    let mut input = value.bytes();
    while let Some(byte) = input.next() {
        bytes.push(match byte {
            b'%' => {
                let hi = char::from(input.next().ok_or(TemplateError::Invalid)?)
                    .to_digit(16)
                    .ok_or(TemplateError::Invalid)?;
                let lo = char::from(input.next().ok_or(TemplateError::Invalid)?)
                    .to_digit(16)
                    .ok_or(TemplateError::Invalid)?;
                (hi * 16 + lo) as u8
            }
            b'+' => b' ',
            byte if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) => byte,
            _ => return Err(TemplateError::Invalid),
        });
    }
    String::from_utf8(bytes).map_err(|_| TemplateError::Invalid)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn presentation_round_trips_without_other_preferences() {
        let template = ClockTemplate::new("%H:%M\né & 日 %%", 12, Theme::Dark).unwrap();
        for link in [template.app_link(), template.share_link()] {
            assert_eq!(ClockTemplate::from_link(&link), Ok(template.clone()));
            assert!(!link.contains("startup"));
        }
        assert_eq!(
            ClockTemplate::from_link(
                "elevenclock://template?v=1&format=%25H%3A%25M&font=9&theme=system"
            )
            .unwrap()
            .format(),
            "%H:%M"
        );
    }
    #[test]
    fn untrusted_links_cannot_extend_the_contract() {
        let valid = "elevenclock://template?v=1&format=%25H&font=9&theme=dark";
        assert_eq!(
            ClockTemplate::from_link(valid),
            ClockTemplate::from_link(&valid.replace("template?", "template/?"))
        );
        for link in [
            valid.replace("v=1", "v=2"),
            format!("{valid}&font=10"),
            format!("{valid}&startup=true"),
            valid.replace("font=9", "font=25"),
            valid.replace("%25H", "%25Q"),
            valid.replace("%25H", "%FF"),
            valid.replace("%25H", "%00"),
            valid.replace("%25H", "%2"),
            valid.replace("template?", "other?"),
            format!("{valid}#extra"),
            "x".repeat(MAX_LINK_BYTES + 1),
        ] {
            assert_eq!(
                ClockTemplate::from_link(&link),
                Err(TemplateError::Invalid),
                "{link}"
            );
        }
    }
    #[test]
    fn rejections_say_why() {
        let valid = "elevenclock://template?v=1&format=%25H&font=9&theme=dark";
        assert_eq!(
            ClockTemplate::from_link(&valid.replace("%25H", "%25H%0A%25M%0A%25S")),
            Err(TemplateError::TooManyLines)
        );
        assert_eq!(
            ClockTemplate::new("%H\n%M\n%S", 12, Theme::System),
            Err(TemplateError::TooManyLines)
        );
        assert_eq!(
            ClockTemplate::from_link(&valid.replace("%25H", &"%25B".repeat(128))),
            Err(TemplateError::TooLong)
        );
        assert_eq!(
            ClockTemplate::new("%Q", 12, Theme::System),
            Err(TemplateError::Invalid)
        );
        assert_eq!(
            TemplateError::TooManyLines.to_string(),
            "Templates support at most two lines."
        );
    }
}

#[cfg(test)]
mod shared_vectors {
    use super::*;
    use crate::Theme;
    /// The website (`site/src/scripts/template.js`) runs the same vectors in
    /// `scripts/template-editor.test.mjs`, so the two implementations cannot drift.
    const VECTORS: &str = include_str!("../tests/fixtures/template-vectors.json");
    #[test]
    fn rust_and_website_agree_on_the_template_contract() {
        let vectors: serde_json::Value = serde_json::from_str(VECTORS).unwrap();
        for case in vectors["accept"].as_array().unwrap() {
            let template = ClockTemplate::from_link(case["link"].as_str().unwrap()).unwrap();
            assert_eq!(template.format(), case["format"].as_str().unwrap());
            assert_eq!(
                u64::from(template.font_size_pt()),
                case["font"].as_u64().unwrap()
            );
            let theme = match case["theme"].as_str().unwrap() {
                "light" => Theme::Light,
                "dark" => Theme::Dark,
                _ => Theme::System,
            };
            assert_eq!(template.theme(), theme);
            assert_eq!(template.app_link(), case["link"].as_str().unwrap());
        }
        for link in vectors["reject"].as_array().unwrap() {
            assert!(
                ClockTemplate::from_link(link.as_str().unwrap()).is_err(),
                "{link}"
            );
        }
    }
}
