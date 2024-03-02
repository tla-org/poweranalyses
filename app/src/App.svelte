<script context="module">
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
        const length = view.findIndex(byte => byte === 0);
        const decoder = new TextDecoder();

        return decoder.decode(new Uint8Array(buffer, ptr, length));
    }

    /** Update the output area by calculating the numbers via WebAssembly. */
    function getOutput() {
        const state = {"analysis":"n","test":"ANCOVA","n":107,"alpha":0.05,"power":0.95,"es":0.5,"q":"10","k":"5","p":"2"};
        const json = JSON.stringify(state);
        console.log(`Sending the following json to the back end: ${json}`);

        const ptr = Module._alloc();
        writeToPtr(ptr, json);
        Module._calculatePower(ptr);
        const returned = readFromPtr(ptr);
        Module._dealloc(ptr);
        console.log(`Received the following json from the back end: ${returned}`);
        const result = JSON.parse(returned);
        const id = Object.keys(result)[0];

        return result;
    }
    let count = $state(0);
</script>

<script>
    // General components
    let family = $state("t");
    let test = $state("oneSampleTTest");
    let analysis = $state("n");

    // Analysis
    let n = $state(50);
    let alpha = $state(0.05);
    let power = $state(0.95);
    let es = $state(0.5);

    // T-test stuff
    let tail = $state("1");
    let allocRatio = $state(1);

    import Footer from './lib/Footer.svelte';
    import Options from './lib/Options.svelte';
</script>

<div class="content">
    <div class="title center">
        <img class="favicon" src="/src/assets/favicon.png" alt="favicon" />
          PowerAnalyses.org Beta
    </div>

    <Options family={family} test={test} analysis={analysis} n={n} alpha={alpha} power={power} es={es} tail={tail} allocRatio={allocRatio} />

    <!-- TODO: remove me  -->
    Got the following response from the back end: {getOutput().n}
    Count: {count}
<Footer />
</div>

