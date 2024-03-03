<script>
    let { getOutput, family, test, analysis, n, alpha, power, es, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors, df } = $props();

    // Derived states for disabling inputs
    let nEnabled = $derived.by(() => analysis === 'n');
    let alphaEnabled = $derived.by(() => analysis === "alpha");
    let powerEnabled = $derived.by(() => analysis === "power");
    let esEnabled = $derived.by(() => analysis === "es");

    function reset() {
        n = 50;
        alpha = 0.05;
        power = 0.95;
        es = 0.5;
    }

    // NOTE: Debug purposes only. Comment out when not needed.
    $inspect(n, alpha, power, es)
</script>

<div class="output border">
  <table id="output" class="center">
    <tbody class="center">
      <tr>
        <td>Sample size:</td>
        <td class="result">
          <input id="n" type="number" min="2" max="99999" step="5" bind:value={n} disabled={nEnabled}>
        </td>
      </tr>
      <tr>
        <td>α err prob:</td>
        <td class="result">
          <input id="alpha" type="number" min="0.01" max="0.99" step="0.01" bind:value={alpha} disabled={alphaEnabled}>
        </td>
      </tr>
      <tr>
        <td>Power (1-β err prob):</td>
        <td class="result">
          <input id="power" type="number" min="0" max="0.99" step="0.01" bind:value={power} disabled={powerEnabled}>
        </td>
      </tr>
      <tr>
        <td>Effect size:</td>
        <td class="result">
          <input id="es" type="number" min="0" max="100" step="0.05" bind:value={es} disabled={esEnabled}>
        </td>
      </tr>
    </tbody>
  </table>
  <div id="error" class="center error">
  </div>
  <div class="center">
    <button class="resetBtn" onclick={reset}>Reset</button>
    <button class="calculateBtn" on:click={() => getOutput(test, analysis, n, alpha, power, es, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors, df)}>Calculate</button>
  </div>
</div>

