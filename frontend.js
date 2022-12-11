function removeAllSelectOptions(selector) {
    while (selector.options.length > 0) {
        selector.remove(0);
    }
}

function addSelectOption(selector, text, enabled, value) {
    let option = new Option(text, value);
    option.disabled = !enabled;
    selector.add(option, undefined);
}

function getElementById(id) {
    const elem = document.getElementById(id);
    assert(elem != null);
    return elem;
}

function readString(id) {
    const elem = getElementById(id);
    return elem.value;
}

// Update the "Statistical test" options based on the "Test family" setting.
function familyChanged() {
    const family = readString("family");
    const testSelector = document.getElementById("test");
    removeAllSelectOptions(testSelector);
    if (family == "exact") {
        addSelectOption(testSelector, "Correlation: Bivariate normal model", true, 1);
        addSelectOption(testSelector, "Linear multiple regression: Random model", false, 2);
        addSelectOption(testSelector, "Proportion: Difference from constant (binomial test, one sample case)", false, 3);
        addSelectOption(testSelector, "Proportions: Inequality, two dependent groups (McNemar)", false, 4);
        addSelectOption(testSelector, "Proportions: Inequality, two independent groups (Fisher's exact test)", false, 5);
        addSelectOption(testSelector, "Proportions: Inequality, two independent groups (unconditional)", false, 6);
        addSelectOption(testSelector, "Proportions: Inequality (offset), two independent groups (unconditional)", false, 7);
        addSelectOption(testSelector, "Proportions: Sign test (binomial test)", false, 8);
    } else if (family == "f") {
        addSelectOption(testSelector, "ANCOVA: Fixed effects, main effects, and interactions", true, 1);
        addSelectOption(testSelector, "ANOVA: Fixed effects, omnibus, one-way", false, 2);
        addSelectOption(testSelector, "ANOVA: Fixed effects, special, main effects, and interactions", false, 3);
        addSelectOption(testSelector, "ANOVA: Repeated measures, between factors", false, 4);
        addSelectOption(testSelector, "ANOVA: Repeated measures, within factors", false, 5);
        addSelectOption(testSelector, "ANOVA: Repeated measures, within-between interaction", false, 6);
        addSelectOption(testSelector, "Hotellings T²: One group mean vector", false, 7);
        addSelectOption(testSelector, "Hotellings T²: Two group mean vector", false, 8);
        addSelectOption(testSelector, "MANOVA: Global effects", false, 9);
        addSelectOption(testSelector, "MANOVA: Special effects and interactions", false, 10);
        addSelectOption(testSelector, "MANOVA: Repeated measures, between factors", false, 11);
        addSelectOption(testSelector, "MANOVA: Repeated measures, within factors", false, 12);
        addSelectOption(testSelector, "MANOVA: Repeated measures, within-between interaction", false, 13);
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, R² deviation from zero", false, 14);
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, R² increase", false, 15);
        addSelectOption(testSelector, "Variance: Test of equality (two sample case)", false, 16);
        addSelectOption(testSelector, "Generic F test", false, 17);
    } else if (family == "t") {
        addSelectOption(testSelector, "Correlation: Point biseral model", false, 1);
        addSelectOption(testSelector, "Linear bivariate regression: One group, size of slope", false, 2);
        addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between intercepts", false, 3);
        addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between slopes", false, 4);
        addSelectOption(testSelector, "Linear multiple regression: Fixed model, single regression coefficient", false, 5);
        addSelectOption(testSelector, "Means: Difference between two dependent means (matched pairs)", false, 6);
        addSelectOption(testSelector, "Means: Difference between two independent means (two groups)", false, 7);
        addSelectOption(testSelector, "Means: Difference from constant (one sample case)", true, 8);
        addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (matched pairs)", false, 9);
        addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (one sample case)", false, 10);
        addSelectOption(testSelector, "Means: Wilcoxon-Mann-Whitney test (two groups)", false, 11);
        addSelectOption(testSelector, "Generic t test", false, 12);
    } else if (family == "chi") {
        addSelectOption(testSelector, "Goodness-of-fit tests: Contingency tables", true, 1);
        addSelectOption(testSelector, "Variance: Difference from constant (one sample case)", true, 2);
        addSelectOption(testSelector, "Generic χ² test", false, 3);
    } else if (family == "z") {
        addSelectOption(testSelector, "Correlation: Tetrachoric model", false, 1);
        addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (common index)", true, 2);
        addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (no common index)", true, 3);
        addSelectOption(testSelector, "Correlations: Two independent Pearson r's", true, 4);
        addSelectOption(testSelector, "Logistic regression", true, 5);
        addSelectOption(testSelector, "Poisson regression", false, 6);
        addSelectOption(testSelector, "Proportions: Difference between two independent proportions", false, 7);
        addSelectOption(testSelector, "Generic z test", false, 8);
    }
    analysisChanged();
    return;
}

function removeAllTableRows(table) {
    while (table.rows.length > 0) {
        table.deleteRow(0);
    }
}

/** Add an option to a table. */
function addTableOption(table, description, element) {
    var row = table.insertRow(table.rows.length);
    var left = row.insertCell(0);
    // Using innerHTML over textContent to allow formatting such as italic.
    left.innerHTML = description.concat(":");
    var right = row.insertCell(1);
    right.innerHTML = element;
    return null;
}

/** Return an input element for floats with element `id`. */
function floatInputElement(id, defaultValue, step) {
    return `<input id="${id}" type="number" value="${defaultValue}" onchange="updateOutput()" min="0" max="999999" step="${step}">`;
}

function floatOutputElement(id, value) {
    return `<span id="${id}">${value}</span>`;
}

function disableOutputElement(id) {
    const elem = getElementById(id);
    elem.disabled = true;
    return null;
}

function enableOutputElement(id) {
    const elem = getElementById(id);
    elem.disabled = false;
    return null;
}

/** Update the input and output area based on the "Type of power analysis" setting. */
function analysisChanged() {
    var inputTable = document.getElementById("input");
    removeAllTableRows(inputTable);
    const family = readString("family");
    if (family == "exact") {
    } else if (family == "f") {
    } else if (family == "t") {
        addTableOption(inputTable, "Tail(s)", "<select onchange='updateOutput()' id='tail'><option value=1>One tail</option><option value=2>Two tails</option></select>");
    } else if (family == "chi") {
    } else if (family == "z") {
    }

    enableOutputElement("n");
    enableOutputElement("alpha");
    enableOutputElement("power");
    enableOutputElement("es");

    const analysis = readString("analysis");
    if (analysis == "n") {
        disableOutputElement("n");
    } else if (analysis == "alpha") {
        disableOutputElement("alpha");
    } else if (analysis == "power") {
        disableOutputElement("power");
    } else if (analysis == "es") {
        disableOutputElement("es");
    }

    updateOutput();

    return null;
}

function readFloat(id) {
    const elem = getElementById(id);
    return parseFloat(elem.value);
}

const highlightBorder = [
    { border: '1px var(--favicon-red) solid' }
];

const highlightTiming = {
    duration: 400,
    iterations: 1,
}

function setFloat(id, value) {
    const elem = getElementById(id);
    elem.animate(highlightBorder, highlightTiming);
    elem.value = value;
    return null;
}

function readInt(id) {
    const elem = getElementById(id);
    return parseInt(elem.value);
}

function tail() {
    return readInt("tail");
}

function alpha() {
    return readFloat("alpha");
}

function power() {
    return readFloat("power");
}

function es() {
    return readFloat("es");
}

function n() {
    return readFloat("n");
}

function restrictFloat(id) {
    const elem = getElementById(id);
    const value = elem.value;
    if (elem.max < elem.value) {
        elem.value = elem.max;
    }
}

/** Enforce that input numbers are within the HTML specified values. */
function restrictInput() {
    restrictFloat("alpha");
}

function setError(text) {
    const elem = getElementById("error");
    elem.innerText = text;
    return null;
}

function handleError(value) {
    if (value == -111 || value == -111.0) {
        setError("Unable to find a solution for given input.");
    }
    return null;
}

function setOutput(id, out) {
    handleError(out);
    setFloat(id, out);
    return null;
}

/** Update the output area by calculating the numbers via WebAssembly. */
function updateOutput() {
    setError("");
    restrictInput();

    const family = readString("family");
    if (family == "exact") {
    } else if (family == "f") {
    } else if (family == "t") {
        const analysis = readString("analysis");
        if (analysis == "n") {
            setOutput("n", Module._oneSampleTTestN(tail(), alpha(), power(), es()));
        } else if (analysis == "alpha") {
            setOutput("alpha", Module._oneSampleTTestAlpha(tail(), n(), power(), es()));
        } else if (analysis == "power") {
            setOutput("power", Module._oneSampleTTestAlpha(tail(), n(), alpha(), es()));
        } else if (analysis == "es") {
        }
    }

    return null;
}

/** Reset the numbers in the output area. */
function resetOutput() {
    setFloat("n", 100);
    setFloat("alpha", 0.05);
    setFloat("power", 0.95);
    setFloat("es", 0.5);
    updateOutput();
}

Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._some_r();
    document.getElementById("n").textContent = 1 + parseFloat(x).toFixed(2);
    familyChanged();
    analysisChanged();
    updateOutput();
}
