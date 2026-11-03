import { TOKENS, WEB_PREFIX, MAX_FORMAT_CHARS, splitFormat, validateTemplate, templateLinks, createTemplateLoader } from './template.js';

export function setupEditor(updatePreview) {
  const input = document.getElementById('format');
  const parts = document.getElementById('format-parts');
  const font = document.getElementById('template-font');
  const theme = document.getElementById('template-theme');
  const error = document.getElementById('template-error');
  const status = document.getElementById('template-status');
  const shared = document.getElementById('template-share');
  const open = document.getElementById('template-open');
  // Oversized input is left to validateTemplate to reject; it is never split into chips.
  const piecesOf = text => [...text].length > MAX_FORMAT_CHARS ? [] : splitFormat(text);
  let pieces = piecesOf(input.value), selected = -1;
  let links;
  const describe = piece => piece === '\n' ? 'New line' : piece.startsWith('%') ? `${TOKENS[piece.slice(1)] ?? 'Token'} ${piece}` : `Text: ${piece}`;
  function draw() {
    parts.replaceChildren();
    pieces.forEach((piece, index) => {
      const li = document.createElement('li');
      const chip = document.createElement('button');
      chip.type = 'button'; chip.draggable = true; chip.textContent = describe(piece);
      chip.setAttribute('aria-pressed', String(index === selected));
      chip.addEventListener('click', () => { selected = index; draw(); parts.children[index]?.firstChild.focus(); });
      chip.addEventListener('dragstart', event => event.dataTransfer.setData('application/x-elevenclock-piece', JSON.stringify({ index })));
      li.addEventListener('dragover', event => event.preventDefault());
      li.addEventListener('drop', event => drop(event, index));
      li.append(chip); parts.append(li);
    });
    for (const id of ['piece-left', 'piece-right', 'piece-remove']) document.getElementById(id).disabled = selected < 0;
    document.getElementById('piece-left').disabled ||= selected === 0;
    document.getElementById('piece-right').disabled ||= selected === pieces.length - 1;
  }
  function refresh(fromInput = false) {
    if (fromInput) { pieces = piecesOf(input.value); selected = -1; }
    else input.value = pieces.join('');
    draw(); error.textContent = ''; links = undefined;
    try {
      const value = validateTemplate({ format: input.value, font: font.value, theme: theme.value });
      links = templateLinks(value); shared.value = links.web; open.href = links.app;
      input.removeAttribute('aria-invalid');
    } catch (failure) { error.textContent = failure.message; shared.value = ''; open.removeAttribute('href'); input.setAttribute('aria-invalid', 'true'); }
    open.setAttribute('aria-disabled', String(!links));
    document.getElementById('copy-format').disabled = !links;
    document.getElementById('copy-template').disabled = !links;
    updatePreview();
  }
  function add(piece) {
    const index = selected < 0 ? pieces.length : selected + 1;
    pieces.splice(index, 0, piece); selected = index; refresh();
  }
  function drop(event, index) {
    event.preventDefault(); event.stopPropagation();
    try {
      const raw = event.dataTransfer.getData('application/x-elevenclock-piece');
      if (raw.length > 128) return;
      const payload = JSON.parse(raw);
      if (Number.isInteger(payload.index) && payload.index >= 0 && payload.index < pieces.length) {
        const [piece] = pieces.splice(payload.index, 1);
        index -= payload.index < index ? 1 : 0;
        pieces.splice(index, 0, piece); selected = index;
      } else if (typeof payload.token === 'string' && payload.token in TOKENS) {
        pieces.splice(index, 0, `%${payload.token}`); selected = index;
      }
      refresh();
    } catch { /* Ignore external or malformed drag payloads. */ }
  }
  const palette = document.getElementById('token-palette');
  for (const [token, label] of Object.entries(TOKENS)) {
    const button = document.createElement('button');
    button.type = 'button'; button.draggable = true; button.textContent = `${label} %${token}`;
    button.addEventListener('click', () => add(`%${token}`));
    button.addEventListener('dragstart', event => event.dataTransfer.setData('application/x-elevenclock-piece', JSON.stringify({ token })));
    palette.append(button);
  }
  parts.addEventListener('dragover', event => event.preventDefault());
  parts.addEventListener('drop', event => drop(event, pieces.length));
  for (const [id, direction] of [['piece-left', -1], ['piece-right', 1]]) {
    document.getElementById(id).addEventListener('click', () => {
      const next = selected + direction;
      if (selected < 0 || next < 0 || next >= pieces.length) return;
      [pieces[selected], pieces[next]] = [pieces[next], pieces[selected]];
      selected = next; refresh(); status.textContent = 'Selected piece moved.';
      if (document.getElementById(id).disabled) parts.children[selected]?.firstChild.focus();
    });
  }
  document.getElementById('piece-remove').addEventListener('click', () => {
    if (selected >= 0) {
      pieces.splice(selected, 1); selected = Math.min(selected, pieces.length - 1); refresh();
      if (selected < 0) palette.firstChild.focus();
    }
  });
  document.getElementById('add-line').addEventListener('click', () => add('\n'));
  document.getElementById('add-literal').addEventListener('click', () => {
    const text = document.getElementById('literal-text');
    if (text.value) { add(text.value.replace(/%/g, '%%')); text.value = ''; }
  });
  input.addEventListener('input', () => refresh(true));
  font.addEventListener('input', () => refresh()); theme.addEventListener('change', () => refresh());
  document.querySelectorAll('.presets button').forEach(button => button.addEventListener('click', () => {
    input.value = button.dataset.format; refresh(true);
  }));
  async function copy(text, message) {
    try { await navigator.clipboard.writeText(text); status.textContent = message; }
    catch { status.textContent = 'Clipboard unavailable. Select the text and copy it manually.'; (text === input.value ? input : shared).focus(); (text === input.value ? input : shared).select(); }
  }
  document.getElementById('copy-format').addEventListener('click', () => copy(input.value, 'Format copied. Paste it into the app’s format editor.'));
  document.getElementById('copy-template').addEventListener('click', () => links && copy(links.web, 'Template link copied. It includes your format text, font size and theme.'));
  const loadTemplate = createTemplateLoader(
    () => ({ format: input.value, font: font.value, theme: theme.value }),
    template => { input.value = template.format; font.value = String(template.font); theme.value = template.theme; refresh(true); },
    () => window.confirm('Replace your current format and appearance edits with this shared template?'),
  );
  function loadHash() {
    if (!location.hash.startsWith('#template=')) return;
    try {
      if (!loadTemplate(WEB_PREFIX + location.hash.slice(1))) {
        status.textContent = 'Shared template not loaded. Your current edits were kept.';
        return;
      }
      document.getElementById('try').scrollIntoView();
      status.textContent = 'Shared template loaded. Review it here or open it in the app.';
    } catch (failure) { status.textContent = failure.message; }
  }
  window.addEventListener('hashchange', loadHash);
  refresh(true); loadHash();
}
