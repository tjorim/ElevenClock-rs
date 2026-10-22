//! Clock palettes resolved from the preference and the platform appearance.
//!
//! The resolution rules are portable and unit tested. Only the system query at
//! the bottom touches Win32. No screen pixels are ever sampled: opaque colors come
//! from per-user shell settings or high-contrast system colors. The accent is
//! an opaque approximation, not the taskbar's wallpaper/transparency composition.
use clock_core::Theme;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    /// WCAG relative luminance in 0.0..=1.0.
    fn luminance(self) -> f64 {
        let channel = |value: u8| {
            let c = f64::from(value) / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        0.2126 * channel(self.r) + 0.7152 * channel(self.g) + 0.0722 * channel(self.b)
    }
}
/// WCAG contrast ratio, 1.0 (identical) to 21.0 (black on white).
pub fn contrast_ratio(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (a.luminance(), b.luminance());
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// Why a palette was chosen. High contrast always wins over the cosmetic setting.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum PaletteKind {
    Light,
    Dark,
    Accent,
    HighContrast,
}
/// Fully opaque colors; there is no alpha channel by design.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Palette {
    pub kind: PaletteKind,
    pub background: Rgb,
    pub foreground: Rgb,
}
impl Palette {
    pub const LIGHT: Self = Self {
        kind: PaletteKind::Light,
        background: Rgb::new(243, 243, 243),
        foreground: Rgb::new(0, 0, 0),
    };
    pub const DARK: Self = Self {
        kind: PaletteKind::Dark,
        background: Rgb::new(32, 32, 32),
        foreground: Rgb::new(255, 255, 255),
    };
    /// Development harnesses that verify coverage by pixel color use fixed colors.
    pub const fn fixed(background: Rgb, foreground: Rgb) -> Self {
        Self {
            kind: PaletteKind::Dark,
            background,
            foreground,
        }
    }
}

/// The platform's resolved appearance, independent of the user's preference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SystemAppearance {
    Light,
    Dark,
    Accent { background: Rgb },
    HighContrast { background: Rgb, foreground: Rgb },
}

/// The accent selection and the taskbar opt-in are separate Windows settings.
/// Windows light mode does not display a taskbar accent. Unreadable/missing
/// accent data falls back to the ordinary system palette.
#[cfg(any(windows, test))]
fn taskbar_appearance(light: bool, accent_enabled: bool, accent: Option<Rgb>) -> SystemAppearance {
    if light {
        SystemAppearance::Light
    } else if accent_enabled && let Some(background) = accent {
        SystemAppearance::Accent { background }
    } else {
        SystemAppearance::Dark
    }
}

/// High contrast first, then an explicit Light/Dark choice, then the system.
pub fn resolve(preference: Theme, system: SystemAppearance) -> Palette {
    match (system, preference) {
        (
            SystemAppearance::HighContrast {
                background,
                foreground,
            },
            _,
        ) => Palette {
            kind: PaletteKind::HighContrast,
            background,
            foreground,
        },
        (SystemAppearance::Accent { background }, Theme::System) => {
            let black = Rgb::new(0, 0, 0);
            let white = Rgb::new(255, 255, 255);
            Palette {
                kind: PaletteKind::Accent,
                background,
                foreground: if contrast_ratio(background, black) > contrast_ratio(background, white)
                {
                    black
                } else {
                    white
                },
            }
        }
        (_, Theme::Light) | (SystemAppearance::Light, Theme::System) => Palette::LIGHT,
        (_, Theme::Dark) | (SystemAppearance::Dark, Theme::System) => Palette::DARK,
    }
}

/// Turns repeated platform notifications into R06's resolved appearance revision.
/// Only a distinct resolved appearance produces a new revision, so unrelated
/// `WM_SETTINGCHANGE` traffic does not repaint every overlay.
#[derive(Clone, Debug)]
pub struct AppearanceTracker {
    current: SystemAppearance,
    revision: u64,
}
impl AppearanceTracker {
    pub fn new(initial: SystemAppearance) -> Self {
        Self {
            current: initial,
            revision: 0,
        }
    }
    pub fn current(&self) -> SystemAppearance {
        self.current
    }
    pub fn revision(&self) -> u64 {
        self.revision
    }
    /// Returns the new revision for `Event::ThemeChanged` when the value changed.
    pub fn observe(&mut self, appearance: SystemAppearance) -> Option<u64> {
        if appearance == self.current {
            return None;
        }
        self.current = appearance;
        self.revision = self.revision.wrapping_add(1);
        Some(self.revision)
    }
}

#[cfg(windows)]
mod native {
    use super::{Rgb, SystemAppearance, taskbar_appearance};
    use crate::bindings::{
        COLOR_WINDOW, COLOR_WINDOWTEXT, GetSysColor, HCF_HIGHCONTRASTON, HIGHCONTRASTW,
        HKEY_CURRENT_USER, LPARAM, RRF_RT_REG_DWORD, RegGetValueW, SPI_GETHIGHCONTRAST,
        SystemParametersInfoW, WM_DWMCOLORIZATIONCOLORCHANGED, WM_SETTINGCHANGE, WM_SYSCOLORCHANGE,
        WM_THEMECHANGED,
    };
    use windows_core::{PCWSTR, w};

    fn colorref(value: u32) -> Rgb {
        Rgb::new(value as u8, (value >> 8) as u8, (value >> 16) as u8)
    }
    fn high_contrast() -> bool {
        let mut info = HIGHCONTRASTW {
            cbSize: std::mem::size_of::<HIGHCONTRASTW>() as u32,
            ..Default::default()
        };
        // SAFETY: `info` is a correctly sized, writable HIGHCONTRASTW.
        unsafe {
            SystemParametersInfoW(
                SPI_GETHIGHCONTRAST as u32,
                info.cbSize,
                (&mut info as *mut HIGHCONTRASTW).cast(),
                0,
            )
        }
        .as_bool()
            && info.dwFlags & HCF_HIGHCONTRASTON as u32 != 0
    }
    fn read_dword(key: PCWSTR, value: PCWSTR) -> Option<u32> {
        let mut data = 0u32;
        let mut size = std::mem::size_of::<u32>() as u32;
        // SAFETY: the buffer and size describe `data`; callers pass static strings.
        let status = unsafe {
            RegGetValueW(
                HKEY_CURRENT_USER,
                key,
                value,
                RRF_RT_REG_DWORD as u32,
                None,
                Some((&mut data as *mut u32).cast()),
                Some(&mut size),
            )
        };
        (status.0 == 0 && size == 4).then_some(data)
    }

    /// Cheap enough for notification handlers; never call from a paint loop.
    pub fn system_appearance() -> SystemAppearance {
        if high_contrast() {
            // SAFETY: GetSysColor accepts these documented indices.
            let (background, foreground) =
                unsafe { (GetSysColor(COLOR_WINDOW), GetSysColor(COLOR_WINDOWTEXT)) };
            SystemAppearance::HighContrast {
                background: colorref(background),
                foreground: colorref(foreground),
            }
        } else {
            let key = w!(r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize");
            let light = read_dword(key, w!("SystemUsesLightTheme")).unwrap_or(1) != 0;
            // This is the Start/taskbar toggle, NOT DWM\ColorPrevalence
            // (which applies to title bars and window borders).
            let enabled = read_dword(key, w!("ColorPrevalence")) == Some(1);
            // Explorer's resolved accent tracks manual and wallpaper-derived
            // selection alike. This registry value is a shell compatibility
            // detail, not a supported API for the final composited taskbar RGB.
            // Its DWORD stores ABGR; use the low RGB bytes, ignoring alpha.
            let accent = if !light && enabled {
                read_dword(
                    w!(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent"),
                    w!("AccentColorMenu"),
                )
                .map(colorref)
            } else {
                None
            };
            taskbar_appearance(light, enabled, accent)
        }
    }

    /// True for messages after which the appearance must be re-read. The caller
    /// feeds the result to `AppearanceTracker::observe`.
    pub fn is_appearance_message(message: u32, lparam: LPARAM) -> bool {
        match message as i32 {
            WM_SYSCOLORCHANGE | WM_THEMECHANGED | WM_DWMCOLORIZATIONCOLORCHANGED => true,
            WM_SETTINGCHANGE => {
                if lparam.0 == 0 {
                    return true;
                }
                // SAFETY: for WM_SETTINGCHANGE a nonzero LPARAM is a NUL-terminated
                // string valid for the duration of the message.
                let text =
                    unsafe { PCWSTR(lparam.0 as *const u16).to_string() }.unwrap_or_default();
                text == "ImmersiveColorSet" || text == "WindowsThemeElement"
            }
            _ => false,
        }
    }
}
#[cfg(windows)]
pub use native::{is_appearance_message, system_appearance};

#[cfg(test)]
mod tests {
    use super::*;

    const HC_BLACK: SystemAppearance = SystemAppearance::HighContrast {
        background: Rgb::new(0, 0, 0),
        foreground: Rgb::new(255, 255, 255),
    };

    #[test]
    fn taskbar_accent_requires_its_own_toggle_and_dark_system_mode() {
        let accent = Rgb::new(24, 80, 130);
        assert_eq!(
            taskbar_appearance(false, false, Some(accent)),
            SystemAppearance::Dark
        );
        assert_eq!(
            taskbar_appearance(true, true, Some(accent)),
            SystemAppearance::Light
        );
        assert_eq!(
            taskbar_appearance(false, true, None),
            SystemAppearance::Dark
        );
        assert_eq!(
            taskbar_appearance(false, true, Some(accent)),
            SystemAppearance::Accent { background: accent }
        );
    }
    #[test]
    fn accent_palettes_have_readable_text_and_respect_explicit_choices() {
        for background in [
            Rgb::new(0, 0, 0),
            Rgb::new(255, 255, 255),
            Rgb::new(24, 80, 130),
            Rgb::new(130, 100, 180),
        ] {
            let system = SystemAppearance::Accent { background };
            let palette = resolve(Theme::System, system);
            assert_eq!(palette.background, background);
            assert_eq!(palette.kind, PaletteKind::Accent);
            assert!(contrast_ratio(background, palette.foreground) >= 4.5);
            assert_eq!(resolve(Theme::Light, system), Palette::LIGHT);
            assert_eq!(resolve(Theme::Dark, system), Palette::DARK);
        }
    }
    #[test]
    fn automatic_accent_changes_advance_the_appearance_revision() {
        let first = SystemAppearance::Accent {
            background: Rgb::new(12, 34, 56),
        };
        let second = SystemAppearance::Accent {
            background: Rgb::new(78, 90, 12),
        };
        let mut tracker = AppearanceTracker::new(first);
        assert_eq!(tracker.observe(first), None);
        assert_eq!(tracker.observe(second), Some(1));
        assert_eq!(tracker.observe(second), None);
        assert_eq!(tracker.observe(SystemAppearance::Dark), Some(2));
    }

    #[test]
    fn explicit_theme_overrides_system_and_system_follows_it() {
        assert_eq!(
            resolve(Theme::Light, SystemAppearance::Dark),
            Palette::LIGHT
        );
        assert_eq!(resolve(Theme::Dark, SystemAppearance::Light), Palette::DARK);
        assert_eq!(
            resolve(Theme::System, SystemAppearance::Light),
            Palette::LIGHT
        );
        assert_eq!(
            resolve(Theme::System, SystemAppearance::Dark),
            Palette::DARK
        );
    }
    #[test]
    fn high_contrast_wins_over_every_cosmetic_choice() {
        for theme in [Theme::System, Theme::Light, Theme::Dark] {
            let palette = resolve(theme, HC_BLACK);
            assert_eq!(palette.kind, PaletteKind::HighContrast);
            assert_eq!(palette.background, Rgb::new(0, 0, 0));
            assert_eq!(palette.foreground, Rgb::new(255, 255, 255));
        }
    }
    #[test]
    fn built_in_palettes_are_readable() {
        assert!(contrast_ratio(Palette::LIGHT.background, Palette::LIGHT.foreground) >= 7.0);
        assert!(contrast_ratio(Palette::DARK.background, Palette::DARK.foreground) >= 7.0);
        assert!((contrast_ratio(Rgb::new(0, 0, 0), Rgb::new(255, 255, 255)) - 21.0).abs() < 1e-9);
        assert!((contrast_ratio(Rgb::new(9, 9, 9), Rgb::new(9, 9, 9)) - 1.0).abs() < 1e-9);
    }
    #[test]
    fn tracker_reports_only_distinct_appearance_changes() {
        let mut tracker = AppearanceTracker::new(SystemAppearance::Light);
        assert_eq!(tracker.observe(SystemAppearance::Light), None);
        assert_eq!(tracker.observe(SystemAppearance::Dark), Some(1));
        assert_eq!(tracker.observe(SystemAppearance::Dark), None);
        assert_eq!(tracker.observe(HC_BLACK), Some(2));
        // Different high-contrast colors are a different resolved appearance.
        let other = SystemAppearance::HighContrast {
            background: Rgb::new(255, 255, 255),
            foreground: Rgb::new(0, 0, 0),
        };
        assert_eq!(tracker.observe(other), Some(3));
        assert_eq!(tracker.current(), other);
        assert_eq!(tracker.revision(), 3);
    }
}
