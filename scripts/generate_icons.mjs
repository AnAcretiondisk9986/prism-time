import fs from "node:fs";
import path from "node:path";
import zlib from "node:zlib";

function makeCrcTable() {
  const table = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) {
      c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    }
    table[n] = c >>> 0;
  }
  return table;
}

const CRC_TABLE = makeCrcTable();

function crc32(buf) {
  let c = 0xffffffff;
  for (let i = 0; i < buf.length; i++) {
    c = CRC_TABLE[(c ^ buf[i]) & 0xff] ^ (c >>> 8);
  }
  return (c ^ 0xffffffff) >>> 0;
}

function pngChunk(type, data) {
  const typeBuf = Buffer.from(type, "ascii");
  const lenBuf = Buffer.alloc(4);
  lenBuf.writeUInt32BE(data.length, 0);
  const crcInput = Buffer.concat([typeBuf, data]);
  const crcBuf = Buffer.alloc(4);
  crcBuf.writeUInt32BE(crc32(crcInput), 0);
  return Buffer.concat([lenBuf, typeBuf, data, crcBuf]);
}

function encodePng(width, height, rgba) {
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const ihdr = Buffer.alloc(13);
  ihdr.writeUInt32BE(width, 0);
  ihdr.writeUInt32BE(height, 4);
  ihdr[8] = 8; // bit depth
  ihdr[9] = 6; // RGBA
  ihdr[10] = 0;
  ihdr[11] = 0;
  ihdr[12] = 0;

  const stride = width * 4 + 1;
  const raw = Buffer.alloc(stride * height);
  for (let y = 0; y < height; y++) {
    raw[y * stride] = 0; // filter None
    rgba.copy(raw, y * stride + 1, y * width * 4, (y + 1) * width * 4);
  }
  const compressed = zlib.deflateSync(raw, { level: 9 });
  return Buffer.concat([
    signature,
    pngChunk("IHDR", ihdr),
    pngChunk("IDAT", compressed),
    pngChunk("IEND", Buffer.alloc(0)),
  ]);
}

function smoothstep(edge0, edge1, x) {
  const t = Math.max(0, Math.min(1, (x - edge0) / (edge1 - edge0)));
  return t * t * (3 - 2 * t);
}

function sdRoundedRect(px, py, rx, ry, r) {
  const qx = Math.abs(px) - (rx - r);
  const qy = Math.abs(py) - (ry - r);
  const ox = Math.max(qx, 0);
  const oy = Math.max(qy, 0);
  return Math.hypot(ox, oy) + Math.min(Math.max(qx, qy), 0) - r;
}

function sdSegment(px, py, ax, ay, bx, by) {
  const pax = px - ax;
  const pay = py - ay;
  const bax = bx - ax;
  const bay = by - ay;
  const h = Math.max(0, Math.min(1, (pax * bax + pay * bay) / (bax * bax + bay * bay)));
  return Math.hypot(pax - bax * h, pay - bay * h);
}

function blendPixel(dst, idx, r, g, b, a) {
  const srcA = Math.max(0, Math.min(1, a));
  if (srcA <= 0) return;
  const invA = 1 - srcA;
  const bgA = dst[idx + 3] / 255;
  const outA = srcA + bgA * invA;
  if (outA <= 1e-6) return;
  dst[idx] = Math.round((r * srcA + dst[idx] * bgA * invA) / outA);
  dst[idx + 1] = Math.round((g * srcA + dst[idx + 1] * bgA * invA) / outA);
  dst[idx + 2] = Math.round((b * srcA + dst[idx + 2] * bgA * invA) / outA);
  dst[idx + 3] = Math.round(outA * 255);
}

function renderPrismIcon(size) {
  const rgba = Buffer.alloc(size * size * 4);
  const aa = 1.35 / size;

  for (let py = 0; py < size; py++) {
    for (let px = 0; px < size; px++) {
      // Normalized coords in [-0.5, 0.5]
      const x = (px + 0.5) / size - 0.5;
      const y = (py + 0.5) / size - 0.5;
      const idx = (py * size + px) * 4;

      // Squircle / rounded tile background
      const dBg = sdRoundedRect(x, y, 0.44, 0.44, 0.115);
      const bgAlpha = 1 - smoothstep(-aa, aa, dBg);
      if (bgAlpha <= 0) continue;

      // Subtle architectural obsidian-indigo gradient
      const gMix = Math.max(0, Math.min(1, (x + y + 0.88) / 1.76));
      const bgR = 17 + gMix * 11;
      const bgG = 20 + gMix * 13;
      const bgB = 28 + gMix * 20;
      blendPixel(rgba, idx, bgR, bgG, bgB, bgAlpha);

      // Subtle inner rim highlight
      const rimDist = Math.abs(dBg + 0.012);
      const rimAlpha = (1 - smoothstep(0, 0.012, rimDist)) * 0.16 * bgAlpha;
      blendPixel(rgba, idx, 255, 255, 255, rimAlpha);

      // Soft ambient prism glow at upper-left
      const glow = Math.exp(-((x + 0.08) ** 2 + (y + 0.1) ** 2) * 10.5) * 0.22 * bgAlpha;
      blendPixel(rgba, idx, 96, 132, 255, glow);

      // Outer precision time-arc ring (270 deg sweep, open at bottom-right)
      const r = Math.hypot(x, y);
      const angle = Math.atan2(y, x); // [-pi, pi]
      const ringD = Math.abs(r - 0.285) - 0.022;
      let ringCov = 1 - smoothstep(-aa, aa, ringD);
      // Leave clean architectural gap around angle [0.25, 1.05] rad
      const inGap = angle > 0.22 && angle < 1.02;
      if (ringCov > 0 && !inGap) {
        const arcMix = (Math.sin(angle * 1.1 + 0.8) + 1) * 0.5;
        const rr = 79 + arcMix * (45 - 79);
        const rg = 107 + arcMix * (212 - 107);
        const rb = 237 + arcMix * (191 - 237);
        blendPixel(rgba, idx, rr, rg, rb, ringCov * bgAlpha);
      }

      // Inner faceted prism diamond/triangle geometry
      const triTop = [0, -0.17];
      const triRight = [0.145, 0.11];
      const triLeft = [-0.145, 0.11];
      const d1 = sdSegment(x, y, triTop[0], triTop[1], triRight[0], triRight[1]);
      const d2 = sdSegment(x, y, triRight[0], triRight[1], triLeft[0], triLeft[1]);
      const d3 = sdSegment(x, y, triLeft[0], triLeft[1], triTop[0], triTop[1]);
      const dInner = sdSegment(x, y, triTop[0], triTop[1], 0.02, 0.11);
      const dPrism = Math.min(d1, d2, d3);
      const prismStroke = 1 - smoothstep(0.015 - aa, 0.015 + aa, dPrism);
      if (prismStroke > 0) {
        blendPixel(rgba, idx, 245, 247, 252, prismStroke * bgAlpha);
      }
      const innerStroke = 1 - smoothstep(0.01 - aa, 0.01 + aa, dInner);
      if (innerStroke > 0) {
        blendPixel(rgba, idx, 145, 180, 255, innerStroke * 0.85 * bgAlpha);
      }

      // Crisp focus dot in the gap
      const dotDist = Math.hypot(x - 0.23, y - 0.165);
      const dotCov = 1 - smoothstep(0.032 - aa, 0.032 + aa, dotDist);
      if (dotCov > 0) {
        blendPixel(rgba, idx, 45, 212, 191, dotCov * bgAlpha);
      }
    }
  }

  return encodePng(size, size, rgba);
}

function encodeIco(frames) {
  // Each frame: { size, png }
  const count = frames.length;
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // reserved
  header.writeUInt16LE(1, 2); // type 1 = ICO
  header.writeUInt16LE(count, 4);

  const dirEntries = [];
  let offset = 6 + count * 16;

  for (const frame of frames) {
    const entry = Buffer.alloc(16);
    entry[0] = frame.size >= 256 ? 0 : frame.size;
    entry[1] = frame.size >= 256 ? 0 : frame.size;
    entry[2] = 0; // color count
    entry[3] = 0; // reserved
    entry.writeUInt16LE(1, 4); // planes
    entry.writeUInt16LE(32, 6); // bpp
    entry.writeUInt32LE(frame.png.length, 8);
    entry.writeUInt32LE(offset, 12);
    dirEntries.push(entry);
    offset += frame.png.length;
  }

  return Buffer.concat([header, ...dirEntries, ...frames.map((f) => f.png)]);
}

const outDir = path.resolve("src-tauri", "icons");
fs.mkdirSync(outDir, { recursive: true });

const png32 = renderPrismIcon(32);
const png128 = renderPrismIcon(128);
const png256 = renderPrismIcon(256);
const ico = encodeIco([
  { size: 32, png: png32 },
  { size: 256, png: png256 },
]);

fs.writeFileSync(path.join(outDir, "32x32.png"), png32);
fs.writeFileSync(path.join(outDir, "128x128.png"), png128);
fs.writeFileSync(path.join(outDir, "icon.png"), png256);
fs.writeFileSync(path.join(outDir, "icon.ico"), ico);

console.log("Generated Tauri icons in", outDir);
