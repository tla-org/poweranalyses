Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._some_r();
    document.getElementById("txtField").value = x;
}
