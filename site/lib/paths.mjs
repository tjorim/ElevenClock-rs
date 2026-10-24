// Kept independent of Astro so config, Markdown transforms and tests share routing.
import { existsSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

export const base = '/ElevenClock-rs/';
export const origin = 'https://tjorim.github.io';
export const repository = 'https://github.com/tjorim/ElevenClock-rs';
export const siteUrl = (route = '') => base + route.replace(/^\/+/, '');

const root = fileURLToPath(new URL('../../', import.meta.url));

// Markdown outside docs/ that is published too. Their routes sit under docs/ so the
// documentation navigation covers them; the files themselves never move.
const special = {
  'README.md': 'docs/guide/',
  'CODE_OF_CONDUCT.md': 'docs/code-of-conduct/',
  'crates/clock-core/FORMAT.md': 'docs/format/',
  'site/README.md': 'docs/website/',
};

// The route a repository Markdown file is published at, or undefined if it is not published.
export function documentRoute(source) {
  if (source in special) return special[source];
  if (!source.startsWith('docs/') || !source.endsWith('.md')) return undefined;
  // Bundled third-party notices are only linked from the index, not published.
  if (source.startsWith('docs/dependency-notices/') && source !== 'docs/dependency-notices/README.md') {
    return undefined;
  }
  const slug = source.slice(5, -3).replace(/(^|\/)README$/, '$1');
  return `docs/${slug}${slug && !slug.endsWith('/') ? '/' : ''}`;
}

export const publishedSources = (sources) => sources.filter(source => documentRoute(source) !== undefined);

// Images and samples kept next to the documentation are copied by scripts/prepare-site.mjs.
const isStatic = target => target.startsWith('assets/') || /^docs\/design\/images\//.test(target);

// Resolve relative URLs from the Markdown source, never from its published URL.
export function rewriteUrl(url, source) {
  if (!url || /^(?:[a-z][a-z\d+.-]*:|\/\/|#)/i.test(url)) return url;
  const [, pathname, suffix = ''] = url.match(/^([^?#]*)(.*)$/);
  if (!pathname) return url;
  const target = path.posix.normalize(pathname.startsWith('/')
    ? pathname.slice(1) : path.posix.join(path.posix.dirname(source), pathname)).replace(/\/$/, '');
  const file = path.join(root, target);
  const kind = existsSync(file) && statSync(file).isDirectory() ? 'tree' : 'blob';
  // A folder links to its README page when it has one.
  const index = kind === 'tree' && existsSync(path.join(file, 'README.md')) ? documentRoute(`${target}/README.md`) : undefined;
  const route = documentRoute(target) ?? index;
  if (route !== undefined) return siteUrl(route) + suffix;
  if (kind === 'blob' && isStatic(target)) return siteUrl(target) + suffix;
  // Other repository files and folders remain available from their source.
  return `${repository}/${kind}/main/${target}${suffix}`;
}

// Every published Markdown source in the working tree, as repository-relative paths.
export function listSources() {
  const found = Object.keys(special).filter(file => existsSync(path.join(root, file)));
  const walk = (dir) => {
    for (const entry of readdirSync(path.join(root, dir), { withFileTypes: true })) {
      const relative = `${dir}/${entry.name}`;
      if (entry.isDirectory()) walk(relative);
      else if (entry.name.endsWith('.md')) found.push(relative);
    }
  };
  walk('docs');
  return publishedSources(found).sort();
}
