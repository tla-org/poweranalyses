<script>
    let { family, test, analysis } = $props();

    import InputTemplate from './inputs/InputTemplate.svelte';
    import TTemplate from './inputs/TTemplate.svelte';
</script>

{#if family == "t"}
    <InputTemplate>
        <TTemplate {test} {analysis} />
    </InputTemplate>
{/if}


