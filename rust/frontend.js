console.log("foo");

Module['onRuntimeInitialized'] = function() {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
    var x = Module._foobar();
    document.getElementById("txtField").value = x;
}
