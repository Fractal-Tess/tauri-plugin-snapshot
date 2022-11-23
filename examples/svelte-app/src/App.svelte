<script lang="ts">
  import { snapshotViewport } from 'tauri-plugin-screen-shot-api';

  let response = '';
  let img: HTMLImageElement;

  function updateResponse(returnValue) {
    response +=
      `[${new Date().toLocaleTimeString()}]` +
      (typeof returnValue === 'string'
        ? returnValue
        : JSON.stringify(returnValue)) +
      '<br>';
  }

  async function _execute() {
    const typedArray = await snapshotViewport({
      capture: { highlighted: true },
      path: 'Path',
    });
    console.log(typedArray);
    const imgBlob = new Blob([typedArray], { type: 'image/png' });
    const imageUrl = window.webkitURL.createObjectURL(imgBlob);
    img.src = imageUrl;
  }
</script>

<div>
  <button on:click={_execute}>Execute</button>
  <div>{@html response}</div>
</div>
<img
  bind:this={img}
  alt=""
  style="border: 3px solid red;"
  width="512"
  height="512"
/>
