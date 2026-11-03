import test from 'node:test';
import assert from 'node:assert/strict';
import { validateTemplate, templateLinks, parseTemplateLink, splitFormat, createTemplateLoader } from '../site/src/scripts/template.js';
test('template codec matches the app contract and retains literal Unicode text', () => {
  const value = { format: '%H:%M\né & 日 %%', font: 12, theme: 'dark' };
  const links = templateLinks(value);
  for (const link of Object.values(links)) assert.deepEqual(parseTemplateLink(link), value);
  assert.equal(templateLinks({format:'%H:%M',font:9,theme:'system'}).app,
    'elevenclock://template?v=1&format=%25H%3A%25M&font=9&theme=system');
  assert.equal(splitFormat(value.format).join(''), value.format);
});
test('links reject unsupported data, extra fields, duplicate fields and malformed encoding', () => {
  const valid = templateLinks({format:'%H',font:9,theme:'dark'}).app;
  assert.deepEqual(parseTemplateLink(valid.replace('template?', 'template/?')), parseTemplateLink(valid));
  for (const link of [valid.replace('v=1','v=2'),valid+'&startup=true',valid+'&font=10',
    valid.replaceAll('%25H','%FF'),valid.replaceAll('%25H','%00'),valid.replaceAll('%25H','%2'),
    valid.replace('font=9','font=25'),valid+'#extra']) assert.throws(() => parseTemplateLink(link));
  for (const format of ['', '%Q', '%', '%H\n\n%M', '%H\n%M\n%S', '%B'.repeat(128), 'x'.repeat(257), '%H\t%M']) assert.throws(() => validateTemplate({format,font:9,theme:'system'}));
});
test('malformed UTF-8 rejects repeated and mixed encodings without recursive decoding', () => {
  const link = format => `elevenclock://template?v=1&format=${format}&font=9&theme=dark`;
  for (const format of ['%FF%FF', '%25H%00%25M', '%25H%ff%25M', '%C0%80', '%ED%A0%80', '%GG']) {
    assert.throws(() => parseTemplateLink(link(format)));
  }
  assert.deepEqual(parseTemplateLink(link('%25%25FF%20%2B%20%26%20%3D')),
    { format: '%%FF + & =', font: 9, theme: 'dark' });
});
test('incoming links preserve dirty edits unless confirmed and reset the baseline after loading', () => {
  let draft = { format: '%H', font: 9, theme: 'system' };
  let allow = false, prompts = 0, writes = 0;
  const load = createTemplateLoader(() => draft, template => { draft = template; writes++; },
    () => { prompts++; return allow; });
  const incoming = templateLinks({ format: '%M', font: 12, theme: 'dark' }).web;
  draft.theme = 'light';
  assert.equal(load(incoming), false);
  assert.deepEqual(draft, { format: '%H', font: 9, theme: 'light' });
  assert.equal(writes, 0);
  allow = true;
  assert.equal(load(incoming), true);
  assert.deepEqual(draft, { format: '%M', font: 12, theme: 'dark' });
  assert.equal(prompts, 2);
  assert.equal(load(incoming), true);
  assert.equal(prompts, 2);
  draft.format = 'my unsaved text';
  assert.throws(() => load(incoming + '&startup=true'));
  assert.equal(draft.format, 'my unsaved text');
  assert.equal(writes, 2);
});
test('website and app share one template contract (crates/clock-core/tests/fixtures/template-vectors.json)', async () => {
  const { readFile } = await import('node:fs/promises');
  const vectors = JSON.parse(await readFile(new URL('../crates/clock-core/tests/fixtures/template-vectors.json', import.meta.url), 'utf8'));
  for (const { link, format, font, theme } of vectors.accept) {
    assert.deepEqual(parseTemplateLink(link), { format, font, theme });
    assert.equal(templateLinks({ format, font, theme }).app, link);
  }
  for (const link of vectors.reject) assert.throws(() => parseTemplateLink(link));
});
