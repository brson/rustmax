// hide transitive crate dependencies from sidebar

window.addEventListener("load", () => {
    console.log("hiding irrelevant crates from sidebar");

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
});

// Must be kept in sync with crates/rmx/Cargo.toml
const crateSet = new Set([
    'ahash',
    'anyhow',
    'backtrace',
    'base64',
    'bindgen',
    'bitflags',
    'blake3',
    'byteorder',
    'bytes',
    'cc',
    'cfg_if',
    'chrono',
    'clap',
    'console',
    'ctrlc',
    'crossbeam',
    'cxx',
    'cxx_build',
    'derive_more',
    'dialoguer',
    'env_logger',
    'extension_trait',
    'futures',
    'http',
    'hex',
    'hyper',
    'indicatif',
    'itertools',
    'jiff',
    'json5',
    'libc',
    'log',
    'mime',
    'nom',
    'num_bigint',
    'num_cpus',
    'num_enum',
    'proc_macro2',
    'proptest',
    'quote',
    'rand',
    'rand_chacha',
    'rand_pcg',
    'rayon',
    'regex',
    'reqwest',
    'rustyline',
    'serde',
    'serde_json',
    'sha2',
    'socket2',
    'static_assertions',
    'syn',
    'tempfile',
    'tera',
    'termcolor',
    'thiserror',
    'tokio',
    'toml',
    'unicode_segmentation',
    'url',
    'walkdir',
    'xshell',
]);

