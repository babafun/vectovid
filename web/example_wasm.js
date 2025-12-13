document.getElementById('pack').addEventListener('click', async () => {
  const status = document.getElementById('status');
  if (!window.vectovid_core || !window.vectovid_core.pack_vvf) {
    status.textContent = 'WASM packer not loaded. Run `wasm-pack build` in core/ first.';
    return;
  }

  const fps = parseInt(document.getElementById('fps').value) || 12;
  const svgFiles = Array.from(document.getElementById('svgInput').files);
  const audioFile = document.getElementById('audioInput').files[0] || null;

  if (svgFiles.length === 0) {
    status.textContent = 'Please select SVG frames.';
    return;
  }

  status.textContent = 'Reading frames...';
  const frames = [];
  for (const f of svgFiles) frames.push(await f.text());

  let audioBytes = null;
  if (audioFile) {
    const ab = await audioFile.arrayBuffer();
    audioBytes = new Uint8Array(ab);
  }

  status.textContent = 'Packing via wasm...';
  try {
    const bytes = window.vectovid_core.pack_vvf(fps, frames, audioBytes);
    const blob = new Blob([bytes], { type: 'application/octet-stream' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'example.vvf';
    a.click();
    status.textContent = 'Done.';
  } catch (e) {
    status.textContent = 'Error: ' + String(e);
  }
});
