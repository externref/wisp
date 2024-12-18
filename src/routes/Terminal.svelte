<script lang="ts">
    import { onMount } from "svelte";
    import { Terminal } from "@xterm/xterm";

    let showTerminal: boolean = false;
    let terminalDiv: HTMLDivElement;
    let terminal: Terminal;

    function handleKeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.key === "t") {
            event.preventDefault();
            showTerminal = !showTerminal;
            if (showTerminal) {
                setTimeout(() => terminalDiv.focus(), 0);
            }
        }
    }
    function initialiseTerminal() {
        terminal = new Terminal();
        terminal.open(terminalDiv);
        terminal.write("> ");
    }
    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
        initialiseTerminal(); terminal.onData((data)=>{
        console.log(data)
    })
        return () => {
            window.removeEventListener("keydown", handleKeydown);
        };
    });
</script>

<div
    class="{showTerminal
        ? `block`
        : `hidden`} fixed inset-0 bg-black bg-opacity-50 z-50"
>
    <div bind:this={terminalDiv}></div>
</div>
