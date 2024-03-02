<script>
    let { family, test, analysis, tail, allocRatio } = $props();

    import InputTemplate from './inputs/InputTemplate.svelte';
    import TTemplate from './inputs/TTemplate.svelte';
</script>

{#if family === "t"}
    <InputTemplate>
        <TTemplate test={test} analysis={analysis} bind:tail={tail} bind:allocRatio={allocRatio} />
    </InputTemplate>
{/if}


