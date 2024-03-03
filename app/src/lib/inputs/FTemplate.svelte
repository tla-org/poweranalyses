<script>
    let { test, analysis, k, p, q, m, rho, epsilon, nPredictors } = $props();

    /**
     * Sanitizes numerical input values to ensure they meet specific criteria, such as being positive numbers.
     * This function can be used to preprocess user inputs in form fields that are expected to be positive.
     *
     * @param {number} value - The input value to be sanitized.
     * @param {number} [defaultValue=0] - A default value to return if the input does not meet the criteria. Optional.
     * @returns {number} - The sanitized input value, ensuring it conforms to the defined criteria (e.g., positive number).
     */
    function sanitizeInt(value, defaultValue = 0) {
        if (value < 0) {
            console.warn(`Input value ${value} is negative, resetting to ${defaultValue}.`);
            return defaultValue;
        }
        return value;
    }

    /**
     * Sanitizes numerical input values to ensure they meet specific criteria, such as being positive numbers.
     * This function can be used to preprocess user inputs in form fields that are expected to be positive.
     *
     * @param {number} value - The input value to be sanitized.
     * @param {number} [defaultValue=0.5] - A default value to return if the input does not meet the criteria. Optional.
     * @returns {number} - The sanitized input value, ensuring it conforms to the defined criteria (e.g., positive number).
     */
    function sanitizeFloat(value, defaultValue = 0.5) {
        if (value < 0) {
            console.warn(`Input value ${value} is negative, resetting to ${defaultValue}.`);
            return defaultValue;
        } else if (value > 1) {
            console.warn(`Input value ${value} is greater than 1, resetting to ${defaultValue}.`);
            return defaultValue;
        }
        return value;
    }

    $effect(() => {
        // Sanitize input values
        k = sanitizeInt(k, 1);
        p = sanitizeInt(p, 2);
        q = sanitizeInt(q, 1);
        m = sanitizeInt(m, 2);
        rho = sanitizeFloat(rho, 0.5);
        epsilon = sanitizeFloat(epsilon, 1);
        nPredictors = sanitizeInt(nPredictors, 1);
    });

    // NOTE: Debug purposes only. Comment out when not needed.
    $inspect(k, p, q, m, rho, epsilon, nPredictors)
</script>

{#if test === "ANCOVA"}
    <tr>
        <td>Numerator df:</td>
        <td>
            <input id="q" type="number" min=1 max=1000 step=1 bind:value={q}>
        </td>
    </tr>
    <tr>
        <td>Number of groups:</td>
        <td><input id="k" type="number" min=1 max=1000 step=1 bind:value={k}>
        </td>
    </tr>
    <tr>
        <td>Number of covariates:</td>
        <td><input id="p" type="number" min=2 max=1000 step=1 bind:value={p}>
        </td>
    </tr>

{:else if test === "oneWayANOVA"}
    <tr>
        <td>Number of groups:</td>
        <td>
            <input id="k" type="number" min=1 max=1000 step=1 bind:value={k}>
        </td>
    </tr>

{:else if test === "twoWayANOVA"}
    <tr>
        <td>Numerator df:</td>
        <td>
            <input id="q" type="number" min=1 max=1000 step=1 bind:value={q}>
        </td>
    </tr>
    <tr>
        <td>Number of groups:</td>
        <td>
            <input id="k" type="number" min=1 max=1000 step=1 bind:value={k}>
        </td>
    </tr>

{:else if test === "betweenRepeatedANOVA"}
    <tr>
        <td>Number of groups:</td>
        <td>
            <input id="k" type="number" min=0 max=1000 step=1 bind:value={k}>
        </td>
    </tr>
    <tr>
        <td>Number of measurement:</td>
        <td>
            <input id="m" type="number" min=2 max=1000 step=1 bind:value={m}>
        </td>
    </tr>
    <tr>
        <td>Corr among rep measures:</td>
        <td>
            <input id="rho" type="number" min=0 max=1 step=0.1 bind:value={rho}>
        </td>
    </tr>

{:else if test === "withinRepeatedANOVA"}
    <tr>
        <td>Number of groups:</td>
        <td>
            <input id="k" type="number" min=0 max=1000 step=1 bind:value={k}>
        </td>
    </tr>
    <tr>
        <td>Number of measurement:</td>
        <td>
            <input id="m" type="number" min=2 max=1000 step=1 bind:value={m}>
        </td>
    </tr>
    <tr>
        <td>Corr among rep measures:</td>
        <td>
            <input id="rho" type="number" min=0 max=1 step=0.1 bind:value={rho}>
        </td>
    </tr>
    <tr>
        <td>Nonsphericity correction ε:</td>
        <td>
            <input id="epsilon" type="number" min=0 max=1 step=0.1 bind:value={epsilon}>
        </td>
    </tr>

{:else if test === "withinBetweenRepeatedANOVA"}
    <tr>
        <td>Number of groups:</td>
        <td>
            <input id="k" type="number" min=0 max=1000 step=1 bind:value={k}>
        </td>
    </tr>
    <tr>
        <td>Number of measurement:</td>
        <td>
            <input id="m" type="number" min=2 max=1000 step=1 bind:value={m}>
        </td>
    </tr>
    <tr>
        <td>Corr among rep measures:</td>
        <td>
            <input id="rho" type="number" min=0 max=1 step=0.1 bind:value={rho}>
        </td>
    </tr>
    <tr>
        <td>Nonsphericity correction ε:</td>
        <td>
            <input id="epsilon" type="number" min=0 max=1 step=0.1 bind:value={epsilon}>
        </td>
    </tr>

{:else if test === "deviationFromZeroMultipleRegression"}
    <tr>
        <td>Number of predictors:</td>
        <td>
            <input id="nPredictors" type="number" min=0 max=1000 step=1 bind:value={nPredictors}>
        </td>
    </tr>

{:else if test === "increaseMultipleRegression"}
    <tr>
        <td>Number of tested predictors:</td>
        <td>
            <input id="q" type="number" min=0 max=1000 step=1 bind:value={q}>
        </td>
    </tr>
    <tr>
        <td>Total number of predictors:</td>
        <td>
            <input id="p" type="number" min=0 max=1000 step=1 bind:value={p}>
        </td>
    </tr>

{/if}

