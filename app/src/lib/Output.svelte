<script>
    let { family, test, analysis } = $props;

    let n = $state(50);
    let alpha = $state(0.05);
    let power = $state(0.95);
    let es = $state(0.5);
</script>

<div class="output border">
  <table id="output" class="center">
    <tbody class="center">
      <tr>
        <td>Sample size:</td>
        <td class="result">
          <input id="n" type="number" min="2" max="99999" step="5" bind:value={n} disable={analysis === "n"}>
        </td>
      </tr>
      <tr>
        <td>α err prob:</td>
        <td class="result">
          <input id="alpha" type="number" value="0.05" min="0.01" max="0.99" step="0.01">
        </td>
      </tr>
      <tr>
        <td>Power (1-β err prob):</td>
        <td class="result">
          <input id="power" type="number" value="0.95" min="0" max="0.99" step="0.01">
        </td>
      </tr>
      <tr>
        <td>Effect size:</td>
        <td class="result">
          <input id="es" type="number" value="0.5" min="0" max="100" step="0.05">
        </td>
      </tr>
    </tbody>
  </table>
  <div id="error" class="center error">
  </div>
  <div class="center">
    <button class="resetBtn">Reset</button>
    <button class="calculateBtn">Calculate</button>
  </div>
</div>

