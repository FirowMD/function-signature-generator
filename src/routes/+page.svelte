<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let textCode = $state("");
  let textHexPattern = $state("");

  async function generateFunctionSignature(event: Event) {
    event.preventDefault();
    textHexPattern = await invoke("generate_function_signature", { code: textCode });
  }
</script>

<main class="flex flex-col space-y-4 h-screen p-4 overflow-hidden">
  <div class="flex card flex-col justify-center items-center  variant-glass-secondary h-12 p-4">
    <h1 class="text-2xl font-bold text-indigo-300">Function Signature Generator</h1>
  </div>
  <div class="flex flex-col space-y-4 justify-center items-center variant-glass-secondary h-[calc(100%-3rem)] w-full p-2">
    <textarea class="textarea resize-none variant-soft-secondary h-[calc(100%-3rem)]" rows="4" placeholder="Paste code from Cheat Engine" bind:value={textCode}>
    </textarea>
    <button type="button" class="btn variant-ghost-primary w-full h-12" onclick={generateFunctionSignature}>Generate</button>
    <textarea class="textarea resize-none variant-ringed-secondary h-[calc(100%-3rem)]" rows="4" placeholder="Hex pattern will be generated here" bind:value={textHexPattern}>
    </textarea>
  </div>
</main>
