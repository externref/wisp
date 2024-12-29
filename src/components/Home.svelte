<script lang="ts">
    import type { ThemeSchema } from "$lib/interfaces";
    import { onMount } from "svelte";

    export let theme: ThemeSchema;
    let blinker = "|"
    onMount(async () => {
        function blink() {
            blinker = blinker == "|" ? "&nbsp" : "|";
            setTimeout(blink, 1000);
        }
        blink();
    });
  
</script>

<div style="background-color: {theme.background};" class="h-screen font-kode text-[#f8f8f2]">
    <div class="w-1/2 mx-auto pt-10 p-4  text-center">
        <img src="/logo.png" alt="icon" class="mx-auto w-20% h-auto" />
        <div class="text-2xl m-4">
            > wisp_<span class="text-md">&nbsp;{@html blinker}</span>
        </div>
        <div>lighter than your mom</div>
        <div class="h-[.1rem] m-4 bg-slate-600"></div>
        <div class="text-left px-4">
            <div class="text-2xl py-2"> Getting Started</div>
            
        </div>
    </div>
</div> 