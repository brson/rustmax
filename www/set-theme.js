function setTheme() {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.querySelector(':root').setAttribute("data-theme", "dark");
    } else {
        document.querySelector(':root').setAttribute("data-theme", "light");
    }
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
    setTheme();
});

window.addEventListener("load", setTheme);

