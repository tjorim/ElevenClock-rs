import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { delimiter, dirname, join, resolve, basename } from 'node:path';
import { pathToFileURL } from 'node:url';
import { spawnSync } from 'node:child_process';
import { templateLinks } from '../site/src/scripts/template.js';

function browserPath() {
  if (process.env.CLOCK_TEST_BROWSER) return process.env.CLOCK_TEST_BROWSER;
  const directories = (process.env.PATH ?? '').split(delimiter);
  const candidates = process.platform === 'win32'
    ? [join(process.env['ProgramFiles(x86)'] ?? '', 'Microsoft/Edge/Application/msedge.exe'),
       join(process.env.ProgramFiles ?? '', 'Microsoft/Edge/Application/msedge.exe'),
       join(process.env.ProgramFiles ?? '', 'Google/Chrome/Application/chrome.exe')]
    : directories.flatMap(directory => ['google-chrome', 'chromium', 'chromium-browser', 'microsoft-edge'].map(name => join(directory, name)));
  return candidates.find(candidate => existsSync(candidate));
}

test('editor loads initial template hashes and handles real hashchange events without discarding dirty edits', { timeout: 40_000 }, (t) => {
  const browser = browserPath();
  if (!browser) {
    assert.ok(!process.env.CI, 'Install Chromium, Chrome or Edge, or set CLOCK_TEST_BROWSER to its executable.');
    t.skip('No Chromium, Chrome or Edge found; set CLOCK_TEST_BROWSER to run this test.');
    return;
  }
  const directory = mkdtempSync(join(tmpdir(), 'elevenclock-editor-'));
  try {
    const first = { format: '%H:%M', font: 12, theme: 'dark' };
    const second = { format: '%g%V.%u', font: 11, theme: 'light' };
    const third = { format: '%d', font: 9, theme: 'system' };
    const editor = pathToFileURL(resolve('site/src/scripts/editor.js')).href;
    // Actual browser elements/events exercise setupEditor itself. The preview
    // callback is irrelevant to template loading and does not replace DOM APIs.
    const html = `<!doctype html><html><body>
      <section id="try"><textarea id="format">%H</textarea>
      <input id="template-font" type="number" value="9">
      <select id="template-theme"><option>system</option><option>light</option><option>dark</option></select>
      <ol id="format-parts"></ol><div id="token-palette"></div>
      <span id="template-error"></span><span id="template-status"></span>
      <input id="template-share"><a id="template-open"></a><input id="literal-text">
      ${['piece-left', 'piece-right', 'piece-remove', 'add-line', 'add-literal', 'copy-format', 'copy-template'].map(id => `<button id="${id}"></button>`).join('')}
      </section><output id="result" data-status="pending"></output>
      <script type="module">
        import { setupEditor } from ${JSON.stringify(editor)};
        const result = document.getElementById('result');
        const first = ${JSON.stringify(first)}, second = ${JSON.stringify(second)}, third = ${JSON.stringify(third)};
        const secondHash = ${JSON.stringify(new URL(templateLinks(second).web).hash)};
        const thirdHash = ${JSON.stringify(new URL(templateLinks(third).web).hash)};
        const fields = () => ({ format: document.getElementById('format').value,
          font: Number(document.getElementById('template-font').value), theme: document.getElementById('template-theme').value });
        const check = (expected, label) => { if (JSON.stringify(fields()) !== JSON.stringify(expected)) throw Error(label); };
        const navigate = hash => new Promise((resolve, reject) => {
          const timeout = setTimeout(() => reject(Error('hashchange was not delivered')), 2000);
          window.addEventListener('hashchange', () => { clearTimeout(timeout); resolve(); }, { once: true });
          location.hash = hash;
        });
        try {
          setupEditor(() => {});
          check(first, 'initial template hash was not loaded');
          await navigate(secondHash);
          check(second, 'hashchange did not load the next template');
          document.getElementById('format').value = '%M';
          document.getElementById('format').dispatchEvent(new Event('input', { bubbles: true }));
          let prompts = 0;
          window.confirm = () => { prompts++; return false; };
          await navigate(thirdHash);
          check({ ...second, format: '%M' }, 'declining replacement discarded the dirty draft');
          if (prompts !== 1) throw Error('dirty hashchange did not ask for confirmation');
          window.confirm = () => { prompts++; return true; };
          await navigate(secondHash);
          check(second, 'confirmed replacement did not load the template');
          if (prompts !== 2) throw Error('confirmed hashchange did not ask for confirmation');
          result.dataset.status = 'passed';
        } catch (error) { result.dataset.status = 'failed'; result.textContent = error.stack; }
      </script></body></html>`;
    const fixture = join(directory, 'editor.html');
    writeFileSync(fixture, html);
    const run = spawnSync(browser, ['--headless', '--disable-gpu', '--no-first-run', '--no-default-browser-check',
      '--allow-file-access-from-files', `--user-data-dir=${join(directory, 'profile')}`,
      '--dump-dom', '--virtual-time-budget=5000', pathToFileURL(fixture).href + new URL(templateLinks(first).web).hash],
      { encoding: 'utf8', timeout: 30_000, maxBuffer: 4 * 1024 * 1024, windowsHide: true });
    assert.ifError(run.error);
    assert.equal(run.status, 0, run.stderr);
    assert.match(run.stdout, /<output id="result" data-status="passed">/, run.stdout);
  } finally {
    // Verify the exact disposable directory before recursive cleanup on Windows.
    assert.equal(dirname(resolve(directory)), resolve(tmpdir()));
    assert.ok(basename(directory).startsWith('elevenclock-editor-'));
    rmSync(directory, { recursive: true, force: true, maxRetries: 20, retryDelay: 50 });
  }
});
