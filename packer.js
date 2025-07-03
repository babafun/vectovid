async function createVVF() {
  const fps = parseInt(document.getElementById("fps").value);
  const svgFiles = Array.from(document.getElementById("svgInput").files);
  const audioFile = document.getElementById("audioInput").files[0] || null;
  const vvfStatus = document.getElementById("vvfStatus");

  if (svgFiles.length === 0) {
    vvfStatus.textContent = "❌ Please upload at least one SVG frame.";
    return;
  }

  vvfStatus.textContent = "⚙️ Packing VVF...";

  const zip = new JSZip();
  const frames = zip.folder("frames");

  svgFiles.sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }));

  for (let i = 0; i < svgFiles.length; i++) {
    const file = svgFiles[i];
    const content = await file.text();
    const filename = String(i).padStart(3, '0') + '.svg';
    frames.file(filename, content);
  }

  let hasAudio = false;
  if (audioFile) {
    const audioData = await audioFile.arrayBuffer();
    zip.file("audio.mp3", audioData);
    hasAudio = true;
  }

  const meta = {
    version: "1.0",
    fps,
    frameCount: svgFiles.length,
    width: 400,
    height: 300,
    hasAudio,
    audioFile: hasAudio ? "audio.mp3" : null
  };
  zip.file("meta.json", JSON.stringify(meta, null, 2));

  const blob = await zip.generateAsync({ type: "blob" });
  const url = URL.createObjectURL(blob);

  const a = document.createElement("a");
  a.href = url;
  a.download = "my_video.vvf";
  a.click();

  vvfStatus.textContent = "✅ Done! VVF file downloaded.";
}
