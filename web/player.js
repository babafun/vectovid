// Shim that prefers a WASM implementation from core/ (wasm-pack output),
// falling back to the moved JS in `../core/js/player.js`.

// WASM-assisted web player.
// Tries to load `core/pkg/vectovid_core.js` and uses wasm helper `frame_interval_ms(fps)` if present.

const svgContainer = document.getElementById('svgContainer');
const vvfInput = document.getElementById('vvfInput');
const vvfStatus = document.getElementById('vvfStatus');
const audio = document.getElementById('audio');

let frames = [];
let fps = 12;
let frameCount = 0;
let currentFrame = 0;
let intervalId = null;

function loadScript(url) {
  return new Promise((resolve, reject) => {
    const s = document.createElement('script');
    s.src = url;
    s.onload = () => resolve();
    s.onerror = () => reject(new Error('Failed to load ' + url));
    document.head.appendChild(s);
  });
}

async function ensureWasm() {
  try {
    if (!window.vectovid_core) await loadScript('../core/pkg/vectovid_core.js');
  } catch (e) {
    console.debug('WASM core not available:', e);
  }
}

vvfInput.addEventListener('change', async () => {
  const file = vvfInput.files[0];
  if (!file) return;

  vvfStatus.textContent = '📦 Loading .vvf...';
  const zip = await JSZip.loadAsync(file);

  const metaData = await zip.file('meta.json').async('string');
  const meta = JSON.parse(metaData);
  fps = meta.fps;
  frameCount = meta.frameCount;

  frames = [];
  for (let i = 0; i < frameCount; i++) {
    const filename = `frames/${String(i).padStart(3, '0')}.svg`;
    const svgText = await zip.file(filename).async('string');
    frames.push(svgText);
  }

  if (meta.hasAudio && zip.file(meta.audioFile)) {
    const audioBlob = await zip.file(meta.audioFile).async('blob');
    const audioUrl = URL.createObjectURL(audioBlob);
    audio.src = audioUrl;
  }
  audio.hidden = true;

  vvfStatus.textContent = `✅ Loaded ${frameCount} frames at ${fps} fps. Ready to play!`;
  showFrame(0);
});

function showFrame(index) {
  svgContainer.innerHTML = frames[index] || '';
}

async function playVVF() {
  if (frames.length === 0) return;
  if (intervalId) return;

  await ensureWasm();

  currentFrame = 0;
  if (audio?.currentTime) audio.currentTime = 0;
  audio?.play?.();

  let intervalMs = 1000 / fps;
  try {
    if (window.vectovid_core && window.vectovid_core.frame_interval_ms) {
      intervalMs = window.vectovid_core.frame_interval_ms(fps);
    }
  } catch (e) {
    console.debug('WASM frame interval helper not available, using JS timing', e);
  }

  intervalId = setInterval(() => {
    showFrame(currentFrame);
    currentFrame = (currentFrame + 1) % frameCount;
  }, intervalMs);
}

function pauseVVF() {
  clearInterval(intervalId);
  intervalId = null;
  audio?.pause?.();
}

window.playVVF = playVVF;
window.pauseVVF = pauseVVF;

