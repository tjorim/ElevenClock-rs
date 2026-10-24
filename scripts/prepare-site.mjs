// Copies the shared files the site publishes into the generated site/public/.
import { cp, mkdir, rm, writeFile } from 'node:fs/promises';

const root = new URL('../', import.meta.url);
const publicDir = new URL('site/public/', root);
await rm(publicDir, { recursive: true, force: true });
await mkdir(publicDir, { recursive: true });
await cp(new URL('assets/', root), new URL('assets/', publicDir), { recursive: true });
// Documentation images and samples stay next to their Markdown sources.
await cp(new URL('docs/design/images/', root), new URL('docs/design/images/', publicDir), { recursive: true });
// The pre-Astro site served these from its root; keep the old URLs (and the
// Open Graph image of links already shared) working.
await cp(new URL('assets/icon.svg', root), new URL('icon.svg', publicDir));
await cp(new URL('assets/icon.png', root), new URL('icon.png', publicDir));
await cp(new URL('docs/design/images/settings/first-run.png', root), new URL('settings.png', publicDir));
await writeFile(new URL('.nojekyll', publicDir), '');
