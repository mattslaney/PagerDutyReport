import init, { alert_rs, process_csv } from "./pkg/wasm.js";

init().then(() => {
  alert_rs("WASM loaded");
});

const fi = document.getElementById("file_input");
fi.addEventListener("input", handleFile);
function handleFile(e) {
  const target = e.target;
  const file = target.files[0];

  if (file) {
    const reader = new FileReader();

    reader.onload = function(event) {
      const contents = event.target.result;
      process_csv(contents);
    };

    reader.onerror = function(event) {
      console.error("File could not be read! Code " + event.target.error.code);
    };

    reader.readAsText(file);
  }
}

