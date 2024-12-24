function setTheme() {
    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.querySelector(':root').setAttribute("data-theme", "dark");
        document.querySelector(':root').classList.remove("light");
        document.querySelector(':root').classList.add("navy");
    } else {
        document.querySelector(':root').setAttribute("data-theme", "light");
        document.querySelector(':root').classList.remove("navy");
        document.querySelector(':root').classList.add("light");
    }
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
    setTheme();
});

window.addEventListener("load", setTheme);
