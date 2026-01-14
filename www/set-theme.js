// Toggle highlight.js theme based on system preference.
// Color scheme is handled by CSS via prefers-color-scheme.
(function() {
    function setHljsTheme() {
        var dark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        var darkTheme = document.getElementById('hljs-dark-theme');
        var lightTheme = document.getElementById('hljs-light-theme');
        if (darkTheme) darkTheme.disabled = !dark;
        if (lightTheme) lightTheme.disabled = dark;
    }

    // Run immediately if DOM is ready, otherwise wait.
    if (document.getElementById('hljs-dark-theme')) {
        setHljsTheme();
    } else {
        document.addEventListener('DOMContentLoaded', setHljsTheme);
    }

    // Listen for system theme changes.
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', setHljsTheme);
})();
