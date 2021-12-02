const submit = document.getElementById("submit");
const input = document.getElementById("input");
const selector = document.getElementById("day-selector");
const output_first = document.getElementById("output-first");
const output_second = document.getElementById("output-second");
const form = document.getElementById("form");
const toast = document.getElementById("toast");
const overlay = document.getElementById("overlay");

submit.disabled = true;

onwasm(() => {
  submit.disabled = false;
  overlay.style.display = "none";
  wasm.options().forEach((v, idx) => {
    const opt = document.createElement("option");
    opt.setAttribute("value", idx + 1);
    opt.innerText = `${idx + 1}: ${v}`;
    selector.appendChild(opt);
  });
});

toast.addEventListener("click", () => {
  toast.classList.remove("toast-show");
});

function error(err) {
  toast.innerText = err;
  toast.classList.add("toast-show");
  setTimeout(() => {
    toast.classList.remove("toast-show");
  }, 3000);
}

form.addEventListener("submit", ev => {
  ev.preventDefault();

  try {
    const { first, second } = window.wasm.run(input.value, +selector.value);

    output_first.value = first;
    output_second.value = second;
  } catch (err) {
    error(err);
    output_first.value = "";
    output_second.value = "";
  }
});
