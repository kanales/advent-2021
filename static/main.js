const get = id => document.getElementById(id);
const submit = get("submit"),
  input = get("input"),
  selector = get("day-selector"),
  output_first = get("output-first"),
  output_second = get("output-second"),
  form = get("form"),
  toast = get("toast"),
  advent_link = get("advent-link"),
  overlay = get("overlay");

submit.disabled = true;
onwasm(() => {
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

const ADVENT_BASE = `https://adventofcode.com/2021`;
selector.addEventListener("change", () => {
  if (selector.value == 0) {
    submit.disabled = true;

    return;
  }
  submit.disabled = false;

  const link = `${ADVENT_BASE}/day/${selector.value}`;
  advent_link.setAttribute("href", link);
  advent_link.innerText = link;
});

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
