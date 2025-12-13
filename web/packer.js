// Web packer using JSZip (pure JavaScript ZIP library).
// No WASM needed for packing in the browser.

window.createVVF = async function() {
  const fps = parseInt(document.getElementById('fps').value);
  const svgFiles = Array.from(document.getElementById('svgInput').files);
  const audioFile = document.getElementById('audioInput').files[0] || null;
  const vvfStatus = document.getElementById('vvfStatus');

  if (svgFiles.length === 0) {
    vvfStatus.textContent = '❌ Please upload at least one SVG frame.';
    return;
  }

  vvfStatus.textContent = '⚙️ Packing VVF...';

  try {
    // Sort frames by filename (numeric-aware sort)
    svgFiles.sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }));

    // Create ZIP archive
    const zip = new JSZip();
    const framesFolder = zip.folder('frames');

    // Add each SVG frame
    for (let i = 0; i < svgFiles.length; i++) {
      const svgText = await svgFiles[i].text();
      framesFolder.file(String(i).padStart(3, '0') + '.svg', svgText);
    }

    // Add audio if provided
    if (audioFile) {
      const audioBytes = await audioFile.arrayBuffer();
      zip.file('audio.mp3', audioBytes);
    }

    // Create metadata JSON
    const meta = {
      version: '1.0',
      fps: fps,
      frameCount: svgFiles.length,
      width: 400,
      height: 300,
      hasAudio: !!audioFile,
      audioFile: audioFile ? 'audio.mp3' : null
    };
    zip.file('meta.json', JSON.stringify(meta, null, 2));

    // Generate and download VVF file
    const blob = await zip.generateAsync({ type: 'blob' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'my_video.vvf';
    a.click();
    URL.revokeObjectURL(url);

    vvfStatus.textContent = '✅ Done! VVF file downloaded.';
  } catch (err) {
    vvfStatus.textContent = '❌ Packer error: ' + String(err);
    console.error(err);
  }
};
