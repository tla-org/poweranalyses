<script>
    import Output from './Output.svelte';
    let family = $state("t");
    let test = $state("oneSampleTTest");
    let analysis = $state("n");
    // let n = $state(107);
    // let alpha = $state(0.05);
    // let power = $state(0.95);
    // let es = $state(0.5);

    const options = {
        t: [
        { id: null, text: "Correlation: Point biseral model", enabled: false, value: 1 },
        { id: null, text: "Linear bivariate regression: One group, size of slope", enabled: false, value: 2 },
        { id: null, text: "Linear bivariate regression: Two groups, difference between intercepts", enabled: false, value: 3 },
        { id: null, text: "Linear bivariate regression: Two groups, difference between slopes", enabled: false, value: 4 },
        { id: null, text: "Linear multiple regression: Fixed model, single regression coefficient", enabled: false, value: 5 },
        { id: "dependentSamplesTTest", text: "Means: Difference between two dependent means (matched pairs)", enabled: false, value: 6 },
        { id: "independentSamplesTTest", text: "Means: Difference between two independent means (two groups)", enabled: true, value: 7 },
        { id: "oneSampleTTest", text: "Means: Difference from constant (one sample case)", enabled: true, value: 8 },
        { id: null, text: "Means: Wilcoxon signed-rank test (matched pairs)", enabled: false, value: 9 },
        { id: null, text: "Means: Wilcoxon signed-rank test (one sample case)", enabled: false, value: 10 },
        { id: null, text: "Means: Wilcoxon-Mann-Whitney test (two groups)", enabled: false, value: 11 },
        { id: null, text: "Generic t test", enabled: false, value: 12 },
        ],
        f: [
            { id: "ANCOVA", text: "ANCOVA: Fixed effects, main effects, and interactions", enabled: true, value: 1 },
        ],
        chi: [
            { id: "goodnessOfFitChisqTest", text: "Goodness-of-fit tests: Contingency tables", enabled: true, value: 1 },
        ],
    };

    // TODO: Comment me out
    $effect(() => {
		console.log("Family changed to ", family);
	});

    $effect(() => {
		console.log("Test changed to ", test);
	});

    $effect(() => {
		console.log("Analysis changed to ", analysis);
	});
</script>

<div class="dropdowns border">
  <label for="family">
    Test family:<br>
    <select class="drop-down" id="family" bind:value={family} name="type">
      <option disabled value="exact">Exact</option>
      <option value="f">F tests</option>
      <option value="t">t tests</option>
      <option value="chi">χ² tests</option>
      <option disabled value="z">z tests</option>
    </select><br>
  </label>
  <label for="test">
    Statistical test:<br>
    <select class="drop-down" id="test" bind:value={test} name="type">
      {#each options[family] as {id, text, value, enabled} (id ?? text)}
        <option value={id ?? text} disabled={!enabled}>{text}</option>
      {/each}
    </select><br>
  </label>
  <label for="analysis">
    Type of power analysis:<br>
    <select class="drop-down" id="analysis" bind:value={analysis} name="type">
      <option value="n">Compute required sample size (a priori)</option>
      <option value="alpha">Compute required α (criterion)</option>
      <option value="power">Compute achieved power (post hoc)</option>
      <option value="es">Compute required effect size (sensitivity)</option>
    </select>
  </label>
  <br>
  <br>
</div>

<div class="numbers">
<Output {family} {test} {analysis} />
</div>
