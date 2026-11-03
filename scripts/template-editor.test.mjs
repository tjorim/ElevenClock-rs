import test from 'node:test';
import assert from 'node:assert/strict';
import { validateTemplate, templateLinks, parseTemplateLink, splitFormat } from '../site/src/scripts/template.js';
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
    valid.replace('%25H','%FF'),valid.replace('%25H','%00'),valid.replace('%25H','%2'),
    valid.replace('font=9','font=25'),valid+'#extra']) assert.throws(() => parseTemplateLink(link));
  for (const format of ['', '%Q', '%', '%H\n\n%M', '%H\n%M\n%S', '%B'.repeat(128), 'x'.repeat(257), '%H\t%M']) assert.throws(() => validateTemplate({format,font:9,theme:'system'}));
});
