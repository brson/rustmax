window.addEventListener("load", () => {
    const exampleButtons = document.querySelectorAll(".example-button");
    for (const button of exampleButtons) {
        const name = button.dataset.name;
        const exampleRow = document.querySelector(`#example-row-${name}`);

        button.addEventListener("click", () => {
            exampleRow.classList.toggle("example-row-visible");
            if (exampleRow.classList.contains("example-row-visible")) {
                button.innerText = "-";
            } else {
                button.innerText = "+";
            }
        });
    }
});
