function writeToPtr(ptr, text) {
  const buffer = Module.HEAPU8.buffer;
  const view = new Uint8Array(buffer, ptr, 1024);
  const encoder = new TextEncoder();
  const with_stop = text + "<END>";
  view.set(encoder.encode(with_stop));
}

function readFromPtr(ptr) {
  const buffer = Module.HEAPU8.buffer;
  const view = new Uint8Array(buffer, ptr, 1024);
  const length = view.findIndex((byte) => byte === 0);
  const decoder = new TextDecoder();

  return decoder.decode(new Uint8Array(buffer, ptr, length));
}

function testme() {
  // create a test JSON with n, alpha, power, es, analysis, test, and the input fields
  const json = JSON.stringify({
    test: "oneSampleTTest",
    tail: "1".toString(),
    analysis: "n",
    n: "50",
    alpha: "0.05",
    power: "0.95",
    es: "0.5",
  });
  console.log(`Sending the following json to the back end: ${json}`);
  const json2 =
    '{"test":"oneSampleTTest", "tail":"1", "analysis":"n", "n":50, "alpha":0.05,"power":0.95,"es":0.5}';
  const ptr = Module._alloc();
  writeToPtr(ptr, json2);
  Module._calculatePower(ptr);
  const returned = readFromPtr(ptr);
  Module._dealloc(ptr);
  console.log(`Received the following json from the back end: ${returned}`);
  const result = JSON.parse(returned);
}

function webAssemblySupport() {
  try {
    if (
      typeof WebAssembly === "object" &&
      typeof WebAssembly.instantiate === "function"
    ) {
      const module = new WebAssembly.Module(
        Uint8Array.of(0x0, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00)
      );
      if (module instanceof WebAssembly.Module) {
        return new WebAssembly.Instance(module) instanceof WebAssembly.Instance;
      }
    }
  } catch (e) {}
  return false;
}

if (!webAssemblySupport()) {
  document.body.innerHTML = `
        <br>
        <center>
        This site only works with WebAssembly. Enable WebAssembly in your browser to continue.
        </center>
        `;
} else {
  Module["onRuntimeInitialized"] = function () {
    console.log("Loading of the poweranalyses.wasm library succeeded.");
  };
}
