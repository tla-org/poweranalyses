<script>
    let { test, analysis, df } = $props();

    // NOTE: Debug purposes only. Comment out when not needed.
    $inspect(df)
</script>

{#if test === "goodnessOfFitChisqTest"}
    <tr>
        <td>Df:</td>
        <td>
            <input id="df" type="number" min=1 max=1000 step=1 bind:value={df}>
        </td>
    </tr>
{/if}

