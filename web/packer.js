// Shim that prefers a WASM implementation from core/ (wasm-pack output),
// falling back to the moved JS in `../core/js/packer.js`.

function loadScript(url) {
  return new Promise((resolve, reject) => {
    const s = document.createElement('script');
    s.src = url;
    s.onload = () => resolve();
    s.onerror = () => reject(new Error('Failed to load ' + url));
    document.head.appendChild(s);
  });
}

async function initPacker() {
  // Try wasm-pack output first (common path `core/pkg/<crate>.js`)
  try {
    await loadScript('../core/pkg/vectovid_core.js');
    if (window.vectovid_core && window.vectovid_core.pack_vvf) {
      // Full wasm packer available: call it with frames array and optional audio bytes
      window.createVVF = async function() {
        const fps = parseInt(document.getElementById('fps').value);
        const svgFiles = Array.from(document.getElementById('svgInput').files);
        const audioFile = document.getElementById('audioInput').files[0] || null;
        const vvfStatus = document.getElementById('vvfStatus');

        if (svgFiles.length === 0) {
          vvfStatus.textContent = '❌ Please upload at least one SVG frame.';
          return;
        }

        vvfStatus.textContent = '⚙️ Packing VVF (wasm)...';

        // read frame texts
        svgFiles.sort((a, b) => a.name.localeCompare(b.name, undefined, { numeric: true }));
        const framesArray = [];
        for (let i = 0; i < svgFiles.length; i++) framesArray.push(await svgFiles[i].text());

        // read optional audio bytes
        let audioBytes = null;
        if (audioFile) {
          const buf = await audioFile.arrayBuffer();
          audioBytes = new Uint8Array(buf);
        }

        // call wasm packer
        try {
          const zipped = window.vectovid_core.pack_vvf(fps, framesArray, audioBytes);
          // wasm-bindgen returns Uint8Array; create blob
          const blob = new Blob([zipped], { type: 'application/octet-stream' });
          const url = URL.createObjectURL(blob);
          const a = document.createElement('a');
          a.href = url;
          a.download = 'my_video.vvf';
          a.click();
          vvfStatus.textContent = '✅ Done! VVF file downloaded.';
        } catch (err) {
          vvfStatus.textContent = '❌ Packer wasm error: ' + String(err);
          console.error(err);
        }
      };
      return;
    }
  } catch (e) {
    console.debug('WASM packer not found', e);
  }

  // Fallback: load the moved JS copy
  try {
    await loadScript('../core/js/packer.js');
    if (window.createVVF_core_js) {
      window.createVVF = window.createVVF_core_js;
    }
  } catch (e) {
    console.error('No packer implementation available:', e);
  }
}

initPacker();
