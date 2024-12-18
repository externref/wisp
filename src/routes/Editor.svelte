<script lang="ts">
    import Title from "./Title.svelte";
    import CodeMirror from "svelte-codemirror-editor";
    import { extensionToLanguageConfig } from "../editorUtils";
    export let filedata: {
        name: string,
        code: string,
        path: string
    } 
    import { dracula } from "thememirror";   
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";
    let value = filedata.code; 

    function saveToLocal(e: KeyboardEvent){
        if (e.ctrlKey && e.key=="s"){
            e.preventDefault()
            invoke("write_to_file", {path: filedata.path, contents: value})
        }
    }

    onMount(() => {
        window.addEventListener("keydown", saveToLocal);
        return () => {
            window.removeEventListener("keydown", saveToLocal);
        };
    });
</script>
<Title {filedata}></Title>
<CodeMirror
    bind:value
    lang={extensionToLanguageConfig[filedata.name.split(".")[filedata.name.split(".").length-1] as keyof typeof extensionToLanguageConfig]() }  
    theme={dracula}     
    styles={{
        "&": {
            width: "100%",
            height: "95.5vh",
        },
    }}
/>
