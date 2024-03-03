<script>
    let { getOutput, family, test, analysis, n, alpha, power, es, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors, df } = $props();

    // Derived states for disabling inputs
    let nEnabled = $derived.by(() => analysis === 'n');
    let alphaEnabled = $derived.by(() => analysis === "alpha");
    let powerEnabled = $derived.by(() => analysis === "power");
    let esEnabled = $derived.by(() => analysis === "es");

    /**
     * Handles the click event on the calculate button. This function calls `getOutput`
     * with the current form values, waits for the calculation result, and updates the
     * appropriate component state based on the type of analysis specified in the result.
     *
     * It's an asynchronous function due to potential delays in calculation,
     * especially when interfacing with WebAssembly or external APIs.
     *
     * Upon successfully updating the values, this function triggers an animation
     * on the element(s) that display the calculation result to visually indicate
     * the update.
     *
     * @async
     * @returns {Promise<void>} A promise that resolves once the state has been updated with the calculation results.
     */
    async function handleCalculate() {
        const result = await getOutput(test, analysis, n, alpha, power, es, tail, allocRatio, k, p, q, m, rho, epsilon, nPredictors, df);
        const id = Object.keys(result)[0]; // 'n', 'es', 'power', 'alpha'
        const value = result[id];

        // Update the corresponding state based on the id
        if (id === 'n') n = value;
        else if (id === 'es') es = value;
        else if (id === 'power') power = value;
        else if (id === 'alpha') alpha = value;

        // Animate the changed element
        animateElements([id]);
    }

    /**
     * Resets statistical analysis parameters to their default values and applies
     * an animation to these fields to indicate that they have been reset.
     * This function is typically used to clear the input fields in the UI and
     * set them back to a predefined starting point.
     *
     * @returns {void}
     */
    function handleReset() {
        n = 50;
        alpha = 0.05;
        power = 0.95;
        es = 0.5;

        // Animate all inputs that have been reset
        animateElements(['n', 'alpha', 'power', 'es']);
    }

    /**
     * Applies an animation effect to a list of elements identified by their DOM IDs. This function
     * is used to highlight changes in input fields by animating their borders.
     *
     * @param {string[]} ids - An array of element IDs to which the animation will be applied.
     */
    function animateElements(ids) {
        const highlightBorder = [
            { border: '1px solid var(--favicon-red)' }
        ];
        const highlightTiming = {
            duration: 400,
            iterations: 1,
        };

        ids.forEach(id => {
            const element = document.querySelector(`#${id}`);
            if (element) {
                element.animate(highlightBorder, highlightTiming);
            }
        });
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
    <button class="resetBtn" onclick={handleReset}>Reset</button>
    <button class="calculateBtn" on:click={handleCalculate}>Calculate</button>
  </div>
</div>

