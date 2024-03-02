<script>
    let { family, test, analysis, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors } = $props();

    import InputTemplate from './inputs/InputTemplate.svelte';
    import TTemplate from './inputs/TTemplate.svelte';
    import FTemplate from './inputs/FTemplate.svelte';
</script>

{#if family === "t"}
    <InputTemplate>
        <TTemplate test={test} bind:analysis={analysis} bind:tail={tail} bind:allocRatio={allocRatio} />
    </InputTemplate>
{:else if family === "f"}
    <InputTemplate>
        <FTemplate test={test} bind:analysis={analysis} bind:k={k} bind:p={p} bind:q={q} bind:m={m} bind:rho={rho} bind:epsilon={epsilon} bind:nPredictors={nPredictors} />
    </InputTemplate>
{/if}


