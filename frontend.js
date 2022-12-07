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

// Update the "Statistical test" options based on the "Test family" setting.
function familyChanged() {
    const familySelector = document.getElementById("family");
    const familyValue = parseInt(familySelector.value);
    const testSelector = document.getElementById("test");
    removeAllSelectOptions(testSelector);
    switch (familyValue) {
        // Using integers since those are easier to pass to WebAssembly.
        case 1: // Exact
            addSelectOption(testSelector, "Correlation: Bivariate normal model", true, 1);
            addSelectOption(testSelector, "Linear multiple regression: Random model", false, 2);
            addSelectOption(testSelector, "Proportion: Difference from constant (binomial test, one sample case)", false, 3);
            addSelectOption(testSelector, "Proportions: Inequality, two dependent groups (McNemar)", false, 4);
            addSelectOption(testSelector, "Proportions: Inequality, two independent groups (Fisher's exact test)", false, 5);
            addSelectOption(testSelector, "Proportions: Inequality, two independent groups (unconditional)", false, 6);
            addSelectOption(testSelector, "Proportions: Inequality (offset), two independent groups (unconditional)", false, 7);
            addSelectOption(testSelector, "Proportions: Sign test (binomial test)", false, 8);
            break;
        case 2: // F tests
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
            break;
        case 3: // t tests
            addSelectOption(testSelector, "Correlation: Point biseral model", false, 1);
            addSelectOption(testSelector, "Linear bivariate regression: One group, size of slope", false, 2);
            addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between intercepts", false, 3);
            addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between slopes", false, 4);
            addSelectOption(testSelector, "Linear multiple regression: Fixed model, single regression coefficient", false, 5);
            addSelectOption(testSelector, "Means: Difference between two dependent means (matched pairs)", false, 6);
            addSelectOption(testSelector, "Means: Difference between two independent means (two groups)", true, 7);
            addSelectOption(testSelector, "Means: Difference from constant (one sample case)", true, 8);
            addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (matched pairs)", false, 9);
            addSelectOption(testSelector, "Means: Wilcoxon signed-rank test (one sample case)", false, 10);
            addSelectOption(testSelector, "Means: Wilcoxon-Mann-Whitney test (two groups)", false, 11);
            addSelectOption(testSelector, "Generic t test", false, 12);
            break;
        case 4: // χ² tests
            addSelectOption(testSelector, "Goodness-of-fit tests: Contingency tables", true, 1);
            addSelectOption(testSelector, "Variance: Difference from constant (one sample case)", true, 2);
            addSelectOption(testSelector, "Generic χ² test", false, 3);
            break;
        case 5: // z tests
            addSelectOption(testSelector, "Correlation: Tetrachoric model", false, 1);
            addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (common index)", true, 2);
            addSelectOption(testSelector, "Correlations: Two dependent Pearson r's (no common index)", true, 3);
            addSelectOption(testSelector, "Correlations: Two independent Pearson r's", true, 4);
            addSelectOption(testSelector, "Logistic regression", true, 5);
            addSelectOption(testSelector, "Poisson regression", false, 6);
            addSelectOption(testSelector, "Proportions: Difference between two independent proportions", false, 7);
            addSelectOption(testSelector, "Generic z test", false, 8);
            break;
        default:
            console.log("Unexpected familySelector.value");
    }
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
    return `<input id="${id}" type="number" value="${defaultValue}" min="0" max="999999" step="${step}">`;
}

function readInt(id) {
    const elem = document.getElementById(id);
    assert(elem != null);
    return parseInt(elem.value);
}

function floatOutputElement(id, value) {
    return `<span id="${id}">${value}</span>`;
}

/** Update the input area based on the "Type of power analysis" setting. */
function analysisChanged() {
    const analysisSelector = document.getElementById("analysis");
    // The number order is the same as the selector.
    const familyValue = readInt("family");
    var inputTable = document.getElementById("input");
    removeAllTableRows(inputTable);
    switch (familyValue) {
        case 3: // t tests
            addTableOption(inputTable, "Tail(s)", "<select id='tail'><option value=1>One tail</option><option value=2>Two tails</option></select>");
            break;
        default:
    }
    // Having the number as an integer makes it easy to pass it to WebAssembly.
    const analysisValue = readInt("analysis");
    // The numbers match the order of the elements in the "Type of power analysis" box.
    if (analysisValue != 1) {
        addTableOption(inputTable, "Total sample size", floatInputElement("n", 100, 5));
    }
    if (analysisValue != 2) {
        addTableOption(inputTable, "α err prob", floatInputElement("alpha", 0.05, 0.05));
    }
    if (analysisValue != 3) {
        addTableOption(inputTable, "Power (1-β err prob)", floatInputElement("alpha", 0.95, 0.05));
    }
    if (analysisValue != 4) {
        addTableOption(inputTable, "Effect size <i>d</i>", floatInputElement("es", 0.5, 0.1));
    }

    var outputTable = document.getElementById("output");
    removeAllTableRows(outputTable);
    addTableOption(outputTable, "Noncentrality parameter λ", floatOutputElement("lambda", 3));
    addTableOption(outputTable, "Critical t", floatOutputElement("t", 3));
    addTableOption(outputTable, "Df", floatOutputElement("df", 3));

    if (analysisValue == 1) {
        addTableOption(outputTable, "Total sample size", "3");
        addTableOption(outputTable, "Actual power", "3");
    }
    if (analysisValue == 2) {
        addTableOption(outputTable, "α err prob", "3");
    }
    if (analysisValue == 3) {
        addTableOption(outputTable, "Power (1-β err prob)", "3");
    }
    if (analysisValue == 4) {
        addTableOption(outputTable, "Effect size <i>dz</i>", "3");
    }

    return null;
}

function readFloat(id) {
    const elem = document.getElementById(id);
    assert(elem != null);
    return parseFloat(elem.value);
}

function setFloat(id, value) {
    const elem = document.getElementById(id);
    assert(elem != null);
    elem.innerText = value;
    return null;
}

/** Update the output area by calculating the numbers via WebAssembly. */
function updateOutput() {
    console.log(readFloat("n"));
    var out = Module._add_ten(readFloat("n"));
    setFloat("df", out);
    return null;
}

Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._some_r();
    document.getElementById("n").textContent = 1 + parseFloat(x).toFixed(2);
    familyChanged();
    analysisChanged();
    updateOutput();
}
