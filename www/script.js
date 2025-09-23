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

    // Load build info
    loadBuildInfo();
});

// Initialize highlight.js after HTMX loads content
document.addEventListener("htmx:afterSettle", function(evt) {
    // Highlight all code blocks in the dynamically loaded content
    document.querySelectorAll('pre code').forEach((block) => {
        hljs.highlightElement(block);
    });
});

async function loadBuildInfo() {
    try {
        const response = await fetch('build-info.json');
        if (response.ok) {
            const buildInfo = await response.json();
            const commitShaElement = document.getElementById('commit-sha');
            if (commitShaElement && buildInfo.commit_sha) {
                const shortSha = buildInfo.commit_sha.substring(0, 8);
                commitShaElement.textContent = `${shortSha}`;
            }
        }
    } catch (error) {
        console.log('Build info not available:', error);
    }
}
