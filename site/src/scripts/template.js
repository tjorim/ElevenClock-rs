// Version 1 presentation contract shared with clock-core::template.
export const WEB_PREFIX = 'https://tjorim.github.io/ElevenClock-rs/#';
export const APP_PREFIX = 'elevenclock://template?';
export const TOKENS = {
  H: 'Hour (24h)', I: 'Hour (12h)', M: 'Minute', S: 'Second', p: 'AM/PM',
  Y: 'Year', y: 'Short year', G: 'ISO week-year', g: 'Short ISO year',
  V: 'ISO week', u: 'Weekday number', m: 'Month number', d: 'Day',
  a: 'Short weekday', A: 'Weekday', b: 'Short month', B: 'Month', '%': 'Percent sign',
};
export function splitFormat(format) {
  return format.match(/%[\s\S]?|\n|[^%\n]+/g) || [];
}
export function validateTemplate(value) {
  const format = value.format.replace(/\r\n/g, '\n');
  if (!format || format.split('\n').some(line => !line)) throw Error('Each line needs some content.');
  if (format.split('\n').length > 2) throw Error('Use at most two lines.');
  if ([...format].length > 256) throw Error('Use at most 256 characters.');
  if (/[\u0000-\u0009\u000B-\u001F\u007F-\u009F]/u.test(format)) throw Error('Control characters are not supported.');
  let maximum = 0;
  for (const piece of splitFormat(format)) {
    if (piece.startsWith('%')) {
      if (!(piece.slice(1) in TOKENS)) throw Error(`Unsupported token: ${piece}`);
      maximum += ({ A: 9, B: 9, a: 3, b: 3, Y: 4, G: 4, '%': 1, u: 1 })[piece[1]] ?? 2;
    } else maximum += [...piece].length;
  }
  if (maximum > 512) throw Error('The rendered clock would be too long.');
  const font = Number(value.font);
  if (!Number.isInteger(font) || font < 6 || font > 24) throw Error('Choose a size from 6 to 24 pt.');
  if (!['system', 'light', 'dark'].includes(value.theme)) throw Error('Choose a supported theme.');
  return { format, font, theme: value.theme };
}
const encode = text => encodeURIComponent(text).replace(/[!'()*]/g, c => `%${c.charCodeAt(0).toString(16).toUpperCase()}`);
export function templateLinks(value) {
  const t = validateTemplate(value);
  const query = `1&format=${encode(t.format)}&font=${t.font}&theme=${t.theme}`;
  return { web: `${WEB_PREFIX}template=${query}`, app: `${APP_PREFIX}v=${query}` };
}
export function parseTemplateLink(link) {
  if (new TextEncoder().encode(link).length > 4096) throw Error('This template link is too long.');
  let query, version;
  if (link.startsWith(WEB_PREFIX)) { query = link.slice(WEB_PREFIX.length); version = 'template'; }
  else if (link.startsWith(APP_PREFIX)) { query = link.slice(APP_PREFIX.length); version = 'v'; }
  else if (link.startsWith('elevenclock://template/?')) { query = link.slice('elevenclock://template/?'.length); version = 'v'; }
  else throw Error('Use an ElevenClock template link.');
  const values = Object.create(null);
  for (const part of query.split('&')) {
    const index = part.indexOf('=');
    if (index < 0) throw Error('Invalid template link.');
    const key = part.slice(0, index), raw = part.slice(index + 1);
    if (![version, 'format', 'font', 'theme'].includes(key) || key in values || !/^(?:[A-Za-z0-9_.~+\-]|%[\da-f]{2})*$/i.test(raw)) throw Error('Invalid template fields.');
    values[key] = decodeURIComponent(raw.replace(/\+/g, ' '));
  }
  if (values[version] !== '1' || !/^[+]?\d+$/.test(values.font ?? '') || values.format === undefined) throw Error('Invalid or unsupported template version.');
  return validateTemplate(values);
}

// A shared link may replace only an unchanged draft or one the user agrees to
// discard. Validate before prompting or touching the current controls.
export function createTemplateLoader(read, apply, confirm) {
  let baseline = JSON.stringify(read());
  return link => {
    const template = parseTemplateLink(link);
    if (JSON.stringify(read()) !== baseline && !confirm()) return false;
    apply(template);
    baseline = JSON.stringify(read());
    return true;
  };
}
