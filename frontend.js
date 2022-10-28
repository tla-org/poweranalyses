Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._some_r();
    document.getElementById("n").textContent = 1 + parseFloat(x).toFixed(2);
}
