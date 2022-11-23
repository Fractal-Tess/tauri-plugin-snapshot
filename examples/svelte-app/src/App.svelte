<script lang="ts">
  import {
    snapshotArea,
    snapshotDocument,
    snapshotViewport,
  } from 'tauri-plugin-screen-shot-api';

  let response = '';
  let img: HTMLImageElement;
  let savePath: HTMLInputElement;

  async function area() {
    const typedArray = await snapshotArea({
      x: 10,
      y: 10,
      width: 20,
      height: 20,
    });
    const imgBlob = new Blob([typedArray], { type: 'image/png' });
    const imageUrl = window.webkitURL.createObjectURL(imgBlob);
    img.src = imageUrl;
  }

  async function view() {
    const typedArray = await snapshotViewport({
      capture: {
        highlighted: true,
        transparentBackground: false,
      },
    });
    const imgBlob = new Blob([typedArray], { type: 'image/png' });
    const imageUrl = window.webkitURL.createObjectURL(imgBlob);
    img.src = imageUrl;
  }
  async function doc() {
    const typedArray = await snapshotDocument();
    const imgBlob = new Blob([typedArray], { type: 'image/png' });
    const imageUrl = window.webkitURL.createObjectURL(imgBlob);
    img.src = imageUrl;
  }
  async function docWithSavePath() {
    if (!savePath.value) {
      alert('save path cannot be empty');
      // return
    }

    const typedArray = await snapshotDocument({
      save: {
        path: savePath.value,
        overwrite: false,
      },
    });
    const imgBlob = new Blob([typedArray], { type: 'image/png' });
    const imageUrl = window.webkitURL.createObjectURL(imgBlob);
    img.src = imageUrl;
  }
</script>

<div>
  <input bind:this={savePath} type="text" placeholder="save path" />
</div>
<div>
  <button on:click={area}>Area</button>
  <button on:click={view}>Viewport</button>
  <button on:click={doc}>Document</button>
  <button on:click={docWithSavePath}>Document & SavePath</button>
  <div>{@html response}</div>
</div>
<img
  bind:this={img}
  alt=""
  style="border: 3px solid red;"
  width="512"
  height="512"
/>
