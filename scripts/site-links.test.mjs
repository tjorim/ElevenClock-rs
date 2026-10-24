import test from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { documentRoute, listSources, rewriteUrl } from '../site/lib/paths.mjs';
import { describe } from '../site/lib/describe.mjs';
import { navigation } from '../site/lib/nav.mjs';
import remarkLinks from '../site/lib/remark-links.mjs';

const base = '/ElevenClock-rs/';

test('Markdown routes keep docs/ layout and map the files outside it', () => {
  assert.equal(documentRoute('docs/README.md'), 'docs/');
  assert.equal(documentRoute('docs/migration.md'), 'docs/migration/');
  assert.equal(documentRoute('docs/design/settings.md'), 'docs/design/settings/');
  assert.equal(documentRoute('docs/dependency-notices/README.md'), 'docs/dependency-notices/');
  assert.equal(documentRoute('README.md'), 'docs/guide/');
  assert.equal(documentRoute('crates/clock-core/FORMAT.md'), 'docs/format/');
  assert.equal(documentRoute('site/README.md'), 'docs/website/');
});

test('third-party notices and other Markdown are not published', () => {
  assert.equal(documentRoute('docs/dependency-notices/anes-0.1.6/README.md'), undefined);
  assert.equal(documentRoute('assets/README.md'), undefined);
  assert.equal(documentRoute('CONTRIBUTING.md'), undefined);
});

test('every published page is in the documentation navigation', () => {
  const navigable = navigation.flatMap(([, pages]) => pages.map(([source]) => source));
  const published = listSources().filter(source => source !== 'docs/README.md');
  assert.deepEqual([...navigable].sort(), published);
});

test('page descriptions are the first prose paragraph as plain text', () => {
  const markdown = '# Title\n\n![badge](x.svg)\n\n> note\n\nA [guide](a.md) with `code` and **bold**\ntext.\n\nSecond paragraph.';
  assert.equal(describe(markdown, 'fallback'), 'A guide with code and bold text.');
  assert.equal(describe('# Only a title\n\n- item', 'fallback'), 'fallback');
  assert.equal(describe('<!<!-- x -->--\n\nHi', 'fallback'), 'Hi');
  assert.equal(describe('<!-- a\n\nb -->\n\nReal text', 'fallback'), 'Real text');
  const long = describe(`${'word '.repeat(60)}end`, 'fallback');
  assert.ok(long.length <= 160 && long.endsWith('…'));
});

test('source-relative links retain fragments and query strings under the base', () => {
  assert.equal(rewriteUrl('migration.md#going-back', 'docs/README.md'), `${base}docs/migration/#going-back`);
  assert.equal(rewriteUrl('../README.md#code-signing-policy', 'docs/README.md'), `${base}docs/guide/#code-signing-policy`);
  assert.equal(rewriteUrl('../../crates/clock-core/FORMAT.md', 'docs/design/settings.md'), `${base}docs/format/`);
  assert.equal(rewriteUrl('docs/migration.md?from=readme#x', 'README.md'), `${base}docs/migration/?from=readme#x`);
  assert.equal(rewriteUrl('../site/README.md#local-development', 'docs/README.md'), `${base}docs/website/#local-development`);
  assert.equal(rewriteUrl('dependency-notices', 'docs/README.md'), `${base}docs/dependency-notices/`);
});

test('images and downloads use the prefixed static assets', () => {
  assert.equal(rewriteUrl('images/settings/first-run.png', 'docs/design/settings.md'), `${base}docs/design/images/settings/first-run.png`);
  assert.equal(rewriteUrl('images/diagnostics/sample-report.txt', 'docs/design/diagnostics.md'), `${base}docs/design/images/diagnostics/sample-report.txt`);
  assert.equal(rewriteUrl('assets/icon.png', 'README.md'), `${base}assets/icon.png`);
  assert.equal(rewriteUrl('../assets/icon.svg', 'docs/README.md'), `${base}assets/icon.svg`);
});

test('other repository files link to GitHub', () => {
  assert.equal(rewriteUrl('../LICENSE', 'docs/README.md'), 'https://github.com/tjorim/ElevenClock-rs/blob/main/LICENSE');
  assert.equal(rewriteUrl('../scripts/', 'docs/README.md'), 'https://github.com/tjorim/ElevenClock-rs/tree/main/scripts');
  assert.equal(rewriteUrl('../../scripts/rust-release.py', 'docs/design/lifecycle.md'), 'https://github.com/tjorim/ElevenClock-rs/blob/main/scripts/rust-release.py');
});

test('external, mail and same-page links remain intact', () => {
  for (const url of ['https://example.com/page.md', 'mailto:someone@example.com', '#setup', '//example.com/image.png']) {
    assert.equal(rewriteUrl(url, 'docs/README.md'), url);
  }
});

test('Remark handles direct links, images and reference definitions', () => {
  const nodes = ['link', 'image', 'definition'].map(type => ({ type, url: '../assets/icon.svg' }));
  remarkLinks()({ children: nodes }, { path: fileURLToPath(new URL('../docs/README.md', import.meta.url)) });
  for (const node of nodes) assert.equal(node.url, `${base}assets/icon.svg`);
});
