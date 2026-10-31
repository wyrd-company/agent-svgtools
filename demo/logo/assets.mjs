// Builds the brand SVGs in docs/user/assets from the marks that gen.mjs draws.
// usage: node assets.mjs <marks-dir> <assets-dir> <scratch-dir>
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const [marks, assets, scratch] = process.argv.slice(2);
if (!marks || !assets || !scratch) {
  console.error("usage: node assets.mjs <marks-dir> <assets-dir> <scratch-dir>");
  process.exit(1);
}
mkdirSync(scratch, { recursive: true });

const THEMES = {
  light: { page: "#fafafa", ink: "#111113", ink2: "#4a4a52", accent: "#0b4fa8", mark: "owl.svg" },
  dark: { page: "#0b0b0c", ink: "#ededef", ink2: "#a6a6af", accent: "#6aa5ff", mark: "owl-dark.svg" },
};

const read = (file) => readFileSync(join(marks, file), "utf8");
const w = (dir, name, body) => writeFileSync(join(dir, name), body);

// The mark's viewBox and inner markup, without its title, for nesting in a larger image.
function inner(file) {
  const svg = read(file);
  const viewBox = svg.match(/viewBox="([^"]+)"/)[1];
  const body = svg
    .replace(/^<svg[^>]*>\n?/, "")
    .replace(/<\/svg>\s*$/, "")
    .replace(/\s*<title[^>]*>.*?<\/title>/, "");
  return { viewBox, body };
}

// Logos: the full mark as drawn.
w(assets, "logo.svg", read("owl.svg"));
w(assets, "logo-dark.svg", read("owl-dark.svg"));

// Favicons: the flat mark with its cream keyline, which holds on light and dark tabs.
const favicon = read("owl-small-dark.svg");
w(assets, "favicon.svg", favicon);
w(assets, "favicon-16.svg", favicon.replace("<svg ", '<svg width="16" height="16" '));

// Apple touch icon source: the full mark on an opaque dark square. The platform rounds it.
{
  const m = inner("owl-dark.svg");
  w(scratch, "apple-touch-icon.svg", `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 180 180" width="180" height="180">
  <rect width="180" height="180" fill="${THEMES.dark.page}"/>
  <svg x="18" y="18" width="144" height="144" viewBox="${m.viewBox}">${m.body}</svg>
</svg>
`);
}

// Social images: mark on the left, name and tagline on the right.
const NAME = "agent-svgtools";
const LINES = ["Close the loop for agents", "authoring SVG from images."];
function social(file, W, H, mode) {
  const t = THEMES[mode];
  const m = inner(t.mark);
  const x = (n) => n + (W - 1200) / 2;
  const y = (n) => n + (H - 630) / 2;
  w(assets, file, `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 ${W} ${H}" width="${W}" height="${H}">
  <title>${NAME}: ${LINES.join(" ")}</title>
  <rect id="background" width="${W}" height="${H}" fill="${t.page}"/>
  <svg id="mark" x="${x(70)}" y="${y(115)}" width="400" height="400" viewBox="${m.viewBox}">${m.body}</svg>
  <g id="text" font-family="Geist, DejaVu Sans, Helvetica, Arial, sans-serif">
    <text x="${x(520)}" y="${y(280)}" font-family="Geist Mono, DejaVu Sans Mono, ui-monospace, monospace" font-size="68" font-weight="700" letter-spacing="-1" fill="${t.ink}">${NAME}</text>
    <rect x="${x(526)}" y="${y(314)}" width="104" height="8" rx="4" fill="${t.accent}"/>
    <text x="${x(520)}" y="${y(385)}" font-size="36" fill="${t.ink2}">${LINES[0]}</text>
    <text x="${x(520)}" y="${y(433)}" font-size="36" fill="${t.ink2}">${LINES[1]}</text>
  </g>
</svg>
`);
}
social("social-image.svg", 1200, 630, "light");
social("social-image-dark.svg", 1200, 630, "dark");
social("social-preview.svg", 1280, 640, "light");
social("social-preview-dark.svg", 1280, 640, "dark");
