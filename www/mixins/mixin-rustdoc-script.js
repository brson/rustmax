// hide transitive crate dependencies from sidebar

window.addEventListener("load", async () => {

    const cratePath = guessCrateJsonPath();
    console.log(`guessing crate.json at ${cratePath}`);

    try {
        const response = await fetch(cratePath);
        if (!response.ok) {
            console.log(`unable to load crate.json: ${response.status}`);
            return;
        }

        const crates = await response.json();
        console.log(`crates: ${crates}`);
        hideCrates(crates);
    } catch (error) {
        console.log(`fetch error: ${error}`);
    }

    //hideCrates();
});

function guessCrateJsonPath() {
    // Presumably `rootPath` is set by rustdoc
    console.assert(window.rootPath != null);
    return `${window.rootPath}/crates.json`;
}

function hideCrates(crates) {
    console.log("hiding irrelevant crates from sidebar");

    const crateSet = new Set(crates);

    var visibleTotal = 0;
    var hiddenTotal = 0;

    const query = "div#rustdoc-modnav ul.crate > li";
    const items = document.querySelectorAll(query);

    for (const item of items) {
        const link = item.querySelector("a");
        const itemName = link.innerText;

        const keep = crateSet.has(itemName);

        if (keep) {
            visibleTotal += 1;
        } else {
            item.style.display = "none";
            hiddenTotal += 1;
        }
    }

    console.log(`visible ${visibleTotal}; hidden ${hiddenTotal}`);
}

