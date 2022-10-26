Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._r_dt(1, 2, 1);
    document.getElementById("txtField").value = x;
}
