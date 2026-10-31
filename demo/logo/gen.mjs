// Draws the agent-svgtools owl mark in four forms.
// Geometry is in the pixel space of demo/targets/owl.png, so any part can be
// scored against the reference with `agent-svgtools judge`. The face axis is x=241.
//
// usage: node gen.mjs <out-dir>
// writes owl.svg, owl-dark.svg (full mark) and owl-small.svg, owl-small-dark.svg (flat, for sizes below 48px).
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const AX = 241;
const VB = { light: "86 -29 310 310", dark: "86 -29 310 310", small: "92 -22 298 298", "small-dark": "88 -26 306 306" };

// Mirror absolute path data (M L C Q H V Z) across the axis.
const mirrorPath = (d) => {
  const toks = d.match(/[A-Za-z]|-?[\d.]+/g);
  let cmd = "", i = 0;
  const o = [];
  for (const t of toks) {
    if (/[A-Za-z]/.test(t)) { cmd = t; i = 0; o.push(t); continue; }
    let v = parseFloat(t);
    if (cmd === "H" || (cmd !== "V" && i % 2 === 0)) v = 2 * AX - v;
    o.push(String(+v.toFixed(2)));
    i++;
  }
  return o.join(" ");
};

const C = {
  ink: "#1e120e",
  brown: "#7a4e40",
  brownLight: "#97654f",
  brownDark: "#54342a",
  keyline: "#f3e9da",
};

// Head: left half drawn counter-clockwise from the crown to the chin; the right half is its mirror.
const HEAD_L = [
  [[224, 26], [208, 29], [196, 34]],
  [[170, 30], [140, 24], [114, 4]],
  [[120, 26], [134, 48], [150, 60]],
  [[124, 84], [106, 120], [104, 160]],
  [[103, 190], [118, 216], [156, 232]],
  [[184, 248], [214, 256], [241, 256]],
];
const HEAD = (() => {
  const start = [241, 26];
  const f = (p) => `${p[0]} ${p[1]}`;
  const m = (p) => [2 * AX - p[0], p[1]];
  let d = `M${f(start)}`;
  for (const [a, b, e] of HEAD_L) d += `C${f(a)} ${f(b)} ${f(e)}`;
  // Reverse and mirror back up the right side.
  const pts = [start, ...HEAD_L.map((s) => s[2])];
  for (let k = HEAD_L.length - 1; k >= 0; k--) {
    const [a, b] = HEAD_L[k];
    d += `C${f(m(b))} ${f(m(a))} ${f(m(pts[k]))}`;
  }
  return d + "Z";
})();

// Facial disc: the inner edge tapers from a wide forehead to a narrow bridge at the beak.
const DISC_L = "M226 60C202 46 160 50 138 76C120 98 114 130 116 160C118 196 138 224 172 232C200 238 222 230 236 214C240 196 239 176 238 156C237 124 236 92 232 74C230 66 229 62 226 60Z";
const DISC_R = mirrorPath(DISC_L);
const TUFT_L = "M150 56C140 42 130 26 122 14M170 38C158 30 144 24 132 18";
const PATCH = (cx) => `M${cx} 94C${cx + 17} 94 ${cx + 29} 106 ${cx + 29} 124C${cx + 29} 146 ${cx + 17} 158 ${cx} 158C${cx - 17} 158 ${cx - 29} 146 ${cx - 29} 124C${cx - 29} 106 ${cx - 17} 94 ${cx} 94Z`;

const eye = (side, cx, px, k = 1.12, dy = 0, rot = 0) => `<g id="${side}-eye" transform="translate(${cx} ${126 + dy}) rotate(${rot}) scale(${k}) translate(${-cx} -126)">
    <path id="${side}-iris" d="${PATCH(cx)}" fill="url(#iris-fill)"/>
    <path id="${side}-lid-shadow" d="M${cx - 34} 88H${cx + 34}V118H${cx - 34}Z" fill="url(#lid-shade)" clip-path="url(#${side}-iris-clip)"/>
    <circle id="${side}-eye-rim" cx="${px + 1.5}" cy="130.5" r="20" fill="#fff8ec"/>
    <circle id="${side}-pupil" cx="${px - 1}" cy="127.5" r="19" fill="url(#pupil-fill)"/>
    <circle id="${side}-catchlight" cx="${px - 7}" cy="121" r="4.6" fill="#fff"/>
  </g>`;

function draw(theme) {
  const vb = VB[theme];
  const full = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${vb}" role="img" aria-labelledby="title">
  <title id="title">agent-svgtools</title>
  <defs>
    <radialGradient id="head-fill" cx="200" cy="40" r="260" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="${C.brownLight}"/>
      <stop offset=".5" stop-color="${C.brown}"/>
      <stop offset="1" stop-color="${C.brownDark}"/>
    </radialGradient>
    <radialGradient id="disc-left-fill" cx="160" cy="96" r="160" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="#fbf9f3"/>
      <stop offset=".55" stop-color="#efebe0"/>
      <stop offset="1" stop-color="#d6ccb3"/>
    </radialGradient>
    <radialGradient id="disc-right-fill" cx="290" cy="96" r="170" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="#f3f1ea"/>
      <stop offset=".55" stop-color="#e3dfd3"/>
      <stop offset="1" stop-color="#c7bda4"/>
    </radialGradient>
    <linearGradient id="disc-top-shade" x1="0" y1="50" x2="0" y2="100" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="#5a3a2c" stop-opacity=".14"/>
      <stop offset="1" stop-color="#5a3a2c" stop-opacity="0"/>
    </linearGradient>
    <linearGradient id="iris-fill" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#ff8414"/>
      <stop offset="1" stop-color="#e2650a"/>
    </linearGradient>
    <linearGradient id="lid-shade" x1="0" y1="88" x2="0" y2="118" gradientUnits="userSpaceOnUse">
      <stop offset="0" stop-color="#a8460a" stop-opacity=".6"/>
      <stop offset="1" stop-color="#a8460a" stop-opacity="0"/>
    </linearGradient>
    <radialGradient id="pupil-fill" cx=".45" cy=".4" r=".6">
      <stop offset="0" stop-color="#2a1b16"/>
      <stop offset=".75" stop-color="#0e0907"/>
      <stop offset="1" stop-color="#0b0706"/>
    </radialGradient>
    <linearGradient id="beak-fill" x1="0" y1="0" x2="1" y2="1">
      <stop offset="0" stop-color="#f7c23c"/>
      <stop offset="1" stop-color="#d48f16"/>
    </linearGradient>
    <clipPath id="disc-left-clip"><path d="${DISC_L}"/></clipPath>
    <clipPath id="disc-right-clip"><path d="${DISC_R}"/></clipPath>
    <clipPath id="left-iris-clip"><path d="${PATCH(188)}"/></clipPath>
    <clipPath id="right-iris-clip"><path d="${PATCH(294)}"/></clipPath>
  </defs>
${theme === "dark" ? `  <path id="keyline" d="${HEAD}" fill="${C.keyline}" stroke="${C.keyline}" stroke-width="14" stroke-linejoin="round"/>\n` : ""}
  <path id="head" d="${HEAD}" fill="url(#head-fill)" stroke="${C.ink}" stroke-width="6" stroke-linejoin="round"/>
  <g id="tuft-feathers" fill="none" stroke="${C.brownDark}" stroke-width="2.5" stroke-linecap="round" opacity=".6">
    <path d="${TUFT_L}"/>
    <path d="${mirrorPath(TUFT_L)}"/>
  </g>

  <g id="facial-disc">
    <g id="left-disc">
      <path d="${DISC_L}" fill="url(#disc-left-fill)"/>
      <path d="${DISC_L}" fill="none" stroke="url(#disc-top-shade)" stroke-width="14" clip-path="url(#disc-left-clip)"/>
    </g>
    <g id="right-disc">
      <path d="${DISC_R}" fill="url(#disc-right-fill)"/>
      <path d="${DISC_R}" fill="none" stroke="url(#disc-top-shade)" stroke-width="14" clip-path="url(#disc-right-clip)"/>
    </g>
  </g>

  ${eye("left", 188, 190, 1.12, 0, -2)}
  ${eye("right", 294, 292, 1.16, -3, 6)}

  <g id="beak">
    <path d="M224 158C224 151 234 147 241 147C248 147 258 151 258 158C258 172 250 188 243 197C242 199 240 199 239 197C232 188 224 172 224 158Z" fill="url(#beak-fill)" stroke="${C.brownDark}" stroke-width="3" stroke-linejoin="round"/>
    <path id="beak-highlight" d="M230 158C230 154 235 152 239 152C236 160 234 168 236 176C232 170 230 164 230 158Z" fill="#ffe08a" opacity=".7"/>
  </g>
</svg>
`;

  const small = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="${vb}" role="img" aria-labelledby="title">
  <title id="title">agent-svgtools</title>
${theme === "small-dark" ? `  <path id="keyline" d="${HEAD}" fill="${C.keyline}" stroke="${C.keyline}" stroke-width="22" stroke-linejoin="round"/>\n` : ""}  <path id="head" d="${HEAD}" fill="${C.brown}" stroke="${C.ink}" stroke-width="12" stroke-linejoin="round"/>
  <path id="left-disc" d="${DISC_L}" fill="#f1efe7"/>
  <path id="right-disc" d="${DISC_R}" fill="#e4e2da"/>
  <ellipse id="left-iris" cx="186" cy="126" rx="34" ry="37" fill="#f2700c"/>
  <ellipse id="right-iris" cx="296" cy="126" rx="34" ry="37" fill="#f2700c"/>
  <circle id="left-pupil" cx="187" cy="130" r="24" fill="#0b0706"/>
  <circle id="right-pupil" cx="295" cy="130" r="24" fill="#0b0706"/>
  <circle cx="179" cy="122" r="7" fill="#fff"/>
  <circle cx="287" cy="122" r="7" fill="#fff"/>
  <path id="beak" d="M218 160C218 151 232 147 241 147C250 147 264 151 264 160C264 176 252 194 241 206C230 194 218 176 218 160Z" fill="#eaa823" stroke="${C.ink}" stroke-width="5" stroke-linejoin="round"/>
</svg>
`;

  return theme.startsWith("small") ? small : full;
}

const outDir = process.argv[2];
if (!outDir) {
  console.error("usage: node gen.mjs <out-dir>");
  process.exit(1);
}
mkdirSync(outDir, { recursive: true });
for (const [theme, file] of [["light", "owl.svg"], ["dark", "owl-dark.svg"], ["small", "owl-small.svg"], ["small-dark", "owl-small-dark.svg"]]) {
  writeFileSync(join(outDir, file), draw(theme));
}
