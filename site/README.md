# Website development and publishing

The website at <https://tjorim.github.io/ElevenClock-rs/> is an Astro static build: the
custom landing page, with its interactive format preview, and the project's Markdown
documentation rendered in the same shell (header, footer and documentation
navigation). It needs no server runtime and loads no remote fonts, analytics or
framework on the client.

## Local development

From the repository root, use Node **24.14.0** (`nvm use`) and npm **10.9.8**. Astro and
the Markdown processor are pinned in `package.json`; commit changes to
`package-lock.json` together with dependency changes. CI installs the pinned npm
version before running `npm ci`.

```sh
npm install --global npm@10.9.8
npm ci
npm run dev
```

Open <http://127.0.0.1:4321/ElevenClock-rs/>. Markdown, layout and CSS edits refresh
in development. After adding assets, restart the command so they are copied again.
To inspect exactly what will be published:

```sh
npm test
npm run build
npm run check:site
npm run preview
```

Preview serves the same URL and repository prefix. `npm test` runs the routing and
link-rewriting tests and works right after `npm ci`. After `npm run build`,
`npm run check:site` validates every output page: internal links, fragments, local
assets and redirect targets must resolve under `/ElevenClock-rs/`.

## Project structure

- `site/src/pages/index.astro`: the landing page.
- `site/src/pages/404.astro`: the not-found page GitHub Pages serves for unknown URLs.
- `site/lib/describe.mjs`: derives each documentation page's meta description from its
  first paragraph.
- `site/src/scripts/clock.js`: the format preview and hero clock. It is a small
  JavaScript copy of the format language in
  [`FORMAT.md`](../crates/clock-core/FORMAT.md); keep the tokens in sync when the
  language changes.
- `site/src/layouts/SiteLayout.astro`: shared metadata, skip link, header and footer.
- `site/src/pages/docs/[...slug].astro`: the documentation shell, sidebar, table of
  contents and the source and edit links on GitHub.
- `site/lib/nav.mjs`: the documentation navigation. `npm test` fails when a published
  page is missing from it.
- `site/lib/paths.mjs` and `remark-links.mjs`: routing and Markdown URL rewriting
  shared by the build and the tests.
- `site/src/styles/global.css`: the design (the app icon's navy, ivory, orange and mint)
  for the landing page and the documentation.
- `scripts/prepare-site.mjs`: copies `assets/` and `docs/design/images/` into the
  generated `site/public/` and keeps the pre-Astro `icon.svg`, `icon.png` and
  `settings.png` URLs working.
- `_site/`: the generated output, ignored by Git together with `site/public/`,
  `site/.astro/` and `node_modules/`. There are no hand-maintained HTML copies of the
  Markdown.

## Markdown routes and links

The Markdown files stay where they are, so GitHub links keep working. Routes follow
their location:

| Source | Published at |
| --- | --- |
| `docs/README.md` | `docs/` |
| `docs/<path>.md` | `docs/<path>/` (for example `docs/design/settings/`) |
| `README.md` | `docs/guide/` (the user guide, including the code signing and AI assistance policies) |
| `crates/clock-core/FORMAT.md` | `docs/format/` |
| `CODE_OF_CONDUCT.md` | `docs/code-of-conduct/` |
| `site/README.md` | `docs/website/` (this page) |

The third-party notices under `docs/dependency-notices/` are bundled license texts,
so only their index is published. `CONTRIBUTING.md` is a short GitHub entry point and
is not published. To publish another file, extend `special` (or the folder rules) in
`site/lib/paths.mjs`, add it to the glob in `[...slug].astro` and to
`site/lib/nav.mjs`.

Astro's Markdown processor supplies GitHub-flavored tables, highlighted code blocks and
heading IDs. A Remark transform resolves relative links from the **source file's
directory**, maps Markdown links to published routes and keeps queries and anchors.
Images and samples under `assets/` and `docs/design/images/` are served from the same
paths within the prefix. Links to any other repository file or folder open it on
GitHub. External URLs, mail links and same-page anchors are left alone. Raw HTML in
Markdown does not pass through the transform, so keep its URLs absolute.

GitHub Pages cannot send HTTP redirects. Astro emits a static refresh page at each
Markdown path (for example `docs/migration.md`) that leads to the published page.

## Repository prefix

`site/lib/paths.mjs` defines the origin, the `/ElevenClock-rs/` base and the GitHub
repository once; the layouts, landing page, Markdown transform and redirects all use
them. Astro prefixes its own generated assets. Do not add a second prefix by hand.

## CI and deployment

In GitHub **Settings > Pages**, choose **GitHub Actions** as the source. The Website
workflow (`.github/workflows/pages.yml`) installs the pinned toolchain, runs `npm ci`,
builds the site and checks the output for pull requests and for pushes to `main`. Only
pushes to `main` (and manual runs on `main`) upload the result with
`upload-pages-artifact` and deploy it. Its path filters include the site sources,
Markdown, assets, build scripts, dependency manifests and the Node pin.

Search is deferred: about two dozen pages are reachable through the sidebar and the
per-page contents list.
