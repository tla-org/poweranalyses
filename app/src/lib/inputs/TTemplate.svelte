<script>
    let { test, analysis } = $props();
    let allocRatio = $state(1);
</script>

{#if test === "oneSampleTTest"}
    <tr>
        <td>Tail(s):</td>
        <td>
            <select id="tail">
                <option value="1">One tail</option>
                <option value="2">Two tails</option>
            </select>
        </td>
    </tr>

{:else if test === "independentSamplesTTest"}
    <tr>
        <td>Tail(s):</td>
        <td>
            <select id="tail">
                <option value="1">One tail</option>
                <option value="2">Two tails</option>
            </select>
        </td>
    </tr>
    <tr>
        <td>Allocation ratio N2/N1</td>
        <td>
            <input id="allocRatio" type="number" min="0.01" max="100" step="0.1" bind:value={allocRatio}>
        </td>
    </tr>

{/if}

