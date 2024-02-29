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
    import Footer from './lib/Footer.svelte';
    import TestType from './lib/TestType.svelte';
</script>

<div class="content">
    <div class="title center">
        <img class="favicon" src="/src/assets/favicon.png" alt="favicon" />
          PowerAnalyses.org Beta
    </div>

    <TestType />

    Got the following response from the back end: {getOutput().n}
    Count: {count}
<Footer />
</div>

