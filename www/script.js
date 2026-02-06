// Initialize after HTMX loads content.
// Note: Syntax highlighting is pre-rendered by syntect, no client-side hljs needed.
document.addEventListener("htmx:afterSettle", function(evt) {
    setupExampleButtons();

    loadBuildInfo();
});

function setupExampleButtons() {
    const exampleButtons = document.querySelectorAll(".example-button");
    for (const button of exampleButtons) {
        // Skip if already initialized to avoid duplicate listeners
        if (button.dataset.initialized === 'true') {
            continue;
        }
        button.dataset.initialized = 'true';

        const name = button.dataset.name;
        const exampleRow = document.querySelector(`#example-row-${name}`);

        // Restore saved state
        const isExpanded = getExpandedState(name);
        if (isExpanded) {
            exampleRow.classList.add("example-row-visible");
            button.innerText = "📖";
        }

        button.addEventListener("click", () => {
            exampleRow.classList.toggle("example-row-visible");
            const isVisible = exampleRow.classList.contains("example-row-visible");
            if (isVisible) {
                button.innerText = "📖";
                saveExpandedState(name, true);
            } else {
                button.innerText = "📘";
                saveExpandedState(name, false);
            }
        });
    }
}

function getExpandedState(name) {
    try {
        const saved = localStorage.getItem('rustmax-expanded-examples');
        if (saved) {
            const expandedExamples = JSON.parse(saved);
            return expandedExamples[name] === true;
        }
    } catch (e) {
        console.log('Error reading expanded state:', e);
    }
    return false;
}

function saveExpandedState(name, isExpanded) {
    try {
        const saved = localStorage.getItem('rustmax-expanded-examples');
        let expandedExamples = {};
        if (saved) {
            expandedExamples = JSON.parse(saved);
        }

        if (isExpanded) {
            expandedExamples[name] = true;
        } else {
            delete expandedExamples[name];
        }

        localStorage.setItem('rustmax-expanded-examples', JSON.stringify(expandedExamples));
    } catch (e) {
        console.log('Error saving expanded state:', e);
    }
}

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

