function setTheme() {
    const darkTheme = document.getElementById('hljs-dark-theme');
    const lightTheme = document.getElementById('hljs-light-theme');

    if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
        document.querySelector(':root').setAttribute("data-theme", "dark");
        if (darkTheme) darkTheme.disabled = false;
        if (lightTheme) lightTheme.disabled = true;
    } else {
        document.querySelector(':root').setAttribute("data-theme", "light");
        if (darkTheme) darkTheme.disabled = true;
        if (lightTheme) lightTheme.disabled = false;
    }
}

window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', event => {
    setTheme();
});

window.addEventListener("load", setTheme);

