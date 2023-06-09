import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// Wrap wasm-bindgen exports (the `generate` function) to add time measurement.
function wrapExports({ generate }) {
  return () => {
    const start = performance.now();
    const data = generate();
    const time = performance.now() - start;
    return {
      // Little perf boost to transfer data to the main thread w/o copying.
      data: Comlink.transfer(data, [data.buffer]),
      time
    };
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      // const singleThread = await import('./pkg/wasm_bindgen_rayon_demo.js');
      // await singleThread.default();
      // return wrapExports(singleThread);
    })(),
    (async () => {
      // If threads are unsupported in this browser, skip this handler.
      if (!(await threads())) return;
      const multiThread = await import(
        './pkg-parallel/wasm_bindgen_rayon_demo.js'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});
