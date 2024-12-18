<script lang="ts">
    import { onMount } from "svelte";

    const draculaTheme = {
        background: "#282a36",
        foreground: "#f8f8f2",
        comment: "#6272a4",
        selectionBackground: "#44475a",
        cyan: "#8be9fd",
        green: "#50fa7b",
        orange: "#ffb86c",
        pink: "#ff79c6",
        purple: "#bd93f9",
        red: "#ff5555",
        yellow: "#f1fa8c",
    };
    let blinker = "|";

    onMount(async () => {
        function blink() {
            blinker = blinker == "|" ? "&nbsp" : "|";
            setTimeout(blink, 1000);
        }
        blink();
    });
    function rand(min: number, max: number) {
  min = Math.ceil(min);
  max = Math.floor(max); 
  return Math.floor(Math.random() * (max - min + 1)) + min;
}


    const greetings = [`if emacs can exist, why can't this`,
        `did you mean vscode`
    ]
</script>

<div style="background-color: {draculaTheme.background};" class="h-screen font-kode text-[#f8f8f2]">
    <div class="w-1/2 mx-auto pt-10 p-4  text-center">
        <img src="/logo.png" alt="icon" class="mx-auto w-20% h-auto" />
        <div class="text-2xl m-4">
            > wisp_<span class="text-md">&nbsp;{@html blinker}</span>
        </div>
        <div>{greetings[rand(0, greetings.length)]}</div>
        <div class="h-[.1rem] m-4 bg-slate-600"></div>
        <div class="text-left px-4">
            <div class="text-2xl py-2">Commands</div>
            <ul class="text-sm">
                <li>[<i class="bi bi-command"></i>] Open Command Palette: <span class="bg-slate-800 rounded-sm px-2 float-right">Cntrl + Shift + P</span></li>
                <li>[<i class="bi bi-folder2-open"></i>] Open File: <span class="bg-slate-800 rounded-sm px-2 float-right">Cntrl + O</span></li>
                <li>[<i class="bi bi-file-earmark-plus"></i>] Create new File: <span class="bg-slate-800 rounded-sm px-2 float-right">Cntrl + N</span></li>
                <li>[<i class="bi bi-floppy2-fill"></i>] Save current File: <span class="bg-slate-800 rounded-sm px-2 float-right">Cntrl + S</span></li>
            </ul>
        </div>
    </div>
</div> 
