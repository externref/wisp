<script lang="ts">
    export let filedata: {
        name: string,
        code: string
    };
</script>

<div class="bg-[#282a36] text-[#f8f8f2] px-2 border-b-2 w-full grid grid-flow-col font-kode h-[4.5vh]">
    <div class="text-center pt-[3px]">{filedata.name}</div>
</div>