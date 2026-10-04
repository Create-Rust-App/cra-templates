import init, { greet, add } from "../../pkg/wasm_starter.js";

const greetingEl = document.querySelector("#greeting");
const sumEl = document.querySelector("#sum");

try {
  await init();
  const greeting = greet("World");
  const sum = add(2, 3);
  console.log(greeting);
  console.log(`2 + 3 = ${sum}`);
  if (greetingEl) greetingEl.textContent = greeting;
  if (sumEl) sumEl.textContent = `2 + 3 = ${sum}`;
} catch (error) {
  console.error("Failed to load WASM package. Run `wasm-pack build --target web --out-dir pkg` first.", error);
  if (greetingEl) greetingEl.textContent = "WASM package missing — build pkg/ first.";
}
