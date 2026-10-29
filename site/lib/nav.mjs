// Documentation navigation: groups of [Markdown source, label]. scripts/site-links.test.mjs
// fails when a published page is missing here, so new pages cannot be forgotten.
export const navigation = [
  ['Use', [
    ['README.md', 'User guide'],
    ['docs/migration.md', 'Migration from ElevenClock'],
    ['docs/managed-deployment.md', 'Managed deployment'],
    ['docs/tested-configurations.md', 'Tested configurations'],
    ['crates/clock-core/FORMAT.md', 'Format strings'],
  ]],
  ['Design', [
    ['docs/design/state-and-scheduling.md', 'State and scheduling'],
    ['docs/design/preferences.md', 'Preferences'],
    ['docs/design/rendering.md', 'Rendering'],
    ['docs/design/lifecycle.md', 'Lifecycle'],
    ['docs/design/interaction.md', 'Interaction'],
    ['docs/design/settings.md', 'Settings window'],
    ['docs/design/diagnostics.md', 'Diagnostics'],
    ['docs/design/startup.md', 'Startup'],
  ]],
  ['Decisions', [
    ['docs/adr/0001-taskbar-discovery.md', 'ADR 0001: taskbar discovery'],
    ['docs/adr/0002-same-executable-discovery-helper.md', 'ADR 0002: discovery helper'],
    ['docs/adr/0003-update-checks.md', 'ADR 0003: update checks'],
    ['docs/adr/0004-managed-launcher.md', 'ADR 0004: managed launcher'],
  ]],
  ['Project', [
    ['docs/contributing.md', 'Contributor guide'],
    ['docs/releasing.md', 'Releasing'],
    ['docs/dependencies.md', 'Dependencies'],
    ['docs/dependency-notices/README.md', 'Dependency notices'],
    ['docs/licensing.md', 'Licensing'],
    ['docs/product-name.md', 'Product name'],
    ['site/README.md', 'Website development'],
    ['CODE_OF_CONDUCT.md', 'Code of conduct'],
  ]],
];
