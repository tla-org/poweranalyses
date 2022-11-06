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
            addSelectOption(testSelector, "Correlation: Point biseral model", true, 1);
            addSelectOption(testSelector, "Linear bivariate regression: One group, size of slope", false, 2);
            addSelectOption(testSelector, "Linear bivariate regression: Two groups, difference between intercepts", false, 3);
            break;
        case 4: // χ2 tests
            break;
        case 5: // z tests
            break;
        default:
            console.log("Unexpected familySelector.value");
    }
    console.log(testSelector);
    return 1;
}

Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._some_r();
    document.getElementById("n").textContent = 1 + parseFloat(x).toFixed(2);
    familyChanged();
}
