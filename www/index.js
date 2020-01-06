import { main } from "tigerust";
import beautify from "json-beautify";

document.getElementById("compile-button").addEventListener("click", e => {
  const code = document.getElementById("code").value;
  const ast = main(code);

  document.getElementById("compiled-ast").textContent = beautify(ast, null, 2, 80);
});
