function idle(c) {
  if ("requestIdleCallback" in window) {
    window.requestIdleCallback(c);
  } else {
    c();
  }
}
idle(() => {
  import("/pkg/website.js").then((mod) => {
    mod.default("/pkg/website.wasm").then(() => {
      for (let e of document.querySelectorAll("leptos-island")) {
        let l = e.dataset.component;
        mod["_island_" + l](e);
      }
      mod.hydrate();
    });
  });
});
