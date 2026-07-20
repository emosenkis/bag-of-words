const picker = document.querySelector("[data-font-picker]");

if (picker) {
  const input = picker.querySelector('input[name="fontFamily"]');
  const choices = picker.querySelectorAll("[data-font]");

  function selectFont(id) {
    input.value = id;
    for (const choice of choices) {
      const selected = choice.dataset.font === id;
      choice.setAttribute("aria-pressed", String(selected));
    }
  }

  for (const choice of choices) {
    choice.addEventListener("click", () => selectFont(choice.dataset.font));
  }
}
