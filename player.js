let frames = [];
let fps = 12;
let frameCount = 0;
let currentFrame = 0;
let intervalId = null;
let audio = null;

const svgContainer = document.getElementById("svgContainer");
const vvfInput = document.getElementById("vvfInput");
const vvfStatus = document.getElementById("vvfStatus");
audio = document.getElementById("audio");

vvfInput.addEventListener("change", async () => {
  const file = vvfInput.files[0];
  if (!file) return;

  vvfStatus.textContent = "📦 Loading .vvf...";
  const zip = await JSZip.loadAsync(file);

  const metaData = await zip.file("meta.json").async("string");
  const meta = JSON.parse(metaData);
  fps = meta.fps;
  frameCount = meta.frameCount;

  frames = [];
  for (let i = 0; i < frameCount; i++) {
    const filename = `frames/${String(i).padStart(3, '0')}.svg`;
    const svgText = await zip.file(filename).async("string");
    frames.push(svgText);
  }

  if (meta.hasAudio && zip.file(meta.audioFile)) {
    const audioBlob = await zip.file(meta.audioFile).async("blob");
    const audioUrl = URL.createObjectURL(audioBlob);
    audio.src = audioUrl;
  }
  audio.hidden = true;

  vvfStatus.textContent = `✅ Loaded ${frameCount} frames at ${fps} fps. Ready to play!`;
  showFrame(0);
});

function showFrame(index) {
  svgContainer.innerHTML = frames[index];
}

function playVVF() {
  if (frames.length === 0) return;
  if (intervalId) return;
  currentFrame = 0;
  audio?.currentTime && (audio.currentTime = 0);
  audio?.play?.();
  intervalId = setInterval(() => {
    showFrame(currentFrame);
    currentFrame = (currentFrame + 1) % frameCount;
  }, 1000 / fps);
}

function pauseVVF() {
  clearInterval(intervalId);
  intervalId = null;
  audio?.pause?.();
}

window.playVVF = playVVF;
window.pauseVVF = pauseVVF;
