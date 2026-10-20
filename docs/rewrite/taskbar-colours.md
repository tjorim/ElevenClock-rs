# Taskbar colours

The System theme follows the Windows system light/dark mode and the separate
**Show accent colour on Start and taskbar** toggle. An accent is used only in
dark system mode with that toggle enabled. Explicit Light/Dark preferences
remain overrides; Windows high-contrast colours take priority over all three.
Text uses whichever of black or white has greater contrast against the accent.

The native adapter reads HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize:
`SystemUsesLightTheme` and `ColorPrevalence`. It does not use the similarly named
DWM value, which controls title bars and borders. Automatic accent selection is
not itself permission to colour the taskbar.

When enabled, the adapter reads Explorer's resolved `AccentColorMenu` DWORD from
HKCU\Software\Microsoft\Windows\CurrentVersion\Explorer\Accent (ABGR byte
order, alpha ignored). This is an isolated, best-effort shell compatibility
detail, not a documented API for the taskbar's final pixel colour. Missing or
unreadable colour data falls back to the dark palette. Windows resolves manual
and wallpaper-derived accents; the app does not derive colours from wallpaper.

Theme/settings notifications and `WM_DWMCOLORIZATIONCOLORCHANGED` trigger a
fresh read. Unchanged appearance is deduplicated before repainting.

The clock background is opaque. Transparency, blur and wallpaper composition can
make the surrounding taskbar differ from this base accent; exact pixel matching
is not claimed. Windows validation is still needed for manual and automatic
accents, toggle on/off, custom app/system modes, transparency on/off, high
contrast, and accent changes while running on multiple monitors.

References:
- [Windows personalization settings](https://learn.microsoft.com/en-us/windows/apps/develop/settings/settings-common#personalization---colors)
- [Colour-change notification](https://learn.microsoft.com/en-us/windows/win32/dwm/wm-dwmcolorizationcolorchanged)
- [Windows colour options](https://support.microsoft.com/en-us/windows/experience/personalization/personalize-your-colors-in-windows)
