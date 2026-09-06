// Topic search UI.
//
// Core search logic is in search-core.js,
// loaded as a separate <script> before this file.
(function() {
    'use strict';

    let searchIndex = null;
    let searchInput = null;
    let searchResults = null;
    let isLoading = false;
    let selectedIndex = -1;

    // Load search index on first focus.
    async function loadSearchIndex() {
        if (searchIndex || isLoading) return;
        isLoading = true;

        try {
            const response = await fetch('search-index.json');
            if (!response.ok) {
                console.warn('Failed to load search index:', response.status);
                return;
            }
            searchIndex = await response.json();
        } catch (e) {
            console.warn('Error loading search index:', e);
        } finally {
            isLoading = false;
        }
    }

    // Escape text for interpolation into HTML.
    //
    // Topic names and aliases are Rust source fragments, so they contain
    // markup characters: the alias "Box<dyn Error>" is parsed as a tag and
    // silently swallowed if it goes into innerHTML unescaped.
    function escapeHtml(text) {
        return String(text)
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;');
    }

    // Render search results grouped by category.
    function renderSearchResults(results) {
        if (!searchResults) return;

        if (results.length === 0) {
            searchResults.innerHTML = '';
            searchResults.classList.remove('visible');
            return;
        }

        // Group by category, in the order the categories first appear.
        //
        // Results arrive sorted by score, so a category's first result is its
        // best one, and grouping this way keeps the highest-scoring result at
        // the top of the dropdown. A fixed category order would bury it.
        const groups = new Map();
        for (const result of results) {
            const cat = result.entry.category;
            if (!groups.has(cat)) groups.set(cat, []);
            groups.get(cat).push(result);
        }

        let html = '';
        for (const [cat, items] of groups) {
            html += `<div class="search-category">${escapeHtml(cat)}</div>`;
            for (const { entry, matchedText } of items) {
                const href = entry.path || '#';
                const matchInfo = formatMatchInfo(matchedText);
                const matchHtml = matchInfo
                    ? `<span class="search-match-info">${escapeHtml(matchInfo)}</span>`
                    : '';
                html += `<a class="search-result" href="${escapeHtml(href)}">
                    <span class="search-result-name">${escapeHtml(entry.name)}${matchHtml}</span>
                    <span class="search-result-brief">${escapeHtml(entry.brief)}</span>
                </a>`;
            }
        }

        searchResults.innerHTML = html;
        searchResults.classList.add('visible');
        selectedIndex = -1;
    }

    // Get all result links in the dropdown.
    function getResultLinks() {
        if (!searchResults) return [];
        return searchResults.querySelectorAll('.search-result');
    }

    // Update the visual highlight on the selected result.
    function updateSelection() {
        const links = getResultLinks();
        for (let i = 0; i < links.length; i++) {
            links[i].classList.toggle('search-result-selected', i === selectedIndex);
        }
        if (selectedIndex >= 0 && selectedIndex < links.length) {
            links[selectedIndex].scrollIntoView({ block: 'nearest' });
        }
    }

    // Initialize search when DOM is ready.
    function setupSearch() {
        searchInput = document.getElementById('search-input');
        searchResults = document.getElementById('search-results');

        if (!searchInput || !searchResults) return;

        // Load index on first focus.
        searchInput.addEventListener('focus', () => {
            loadSearchIndex();
        });

        // Perform search on input.
        let debounceTimer = null;
        searchInput.addEventListener('input', () => {
            clearTimeout(debounceTimer);
            debounceTimer = setTimeout(() => {
                const query = searchInput.value.trim();
                if (query.length < 2) {
                    renderSearchResults([]);
                    return;
                }
                const results = performSearch(searchIndex, query);
                renderSearchResults(results);
            }, 100);
        });

        // Keyboard navigation: up/down/enter/escape.
        searchInput.addEventListener('keydown', (e) => {
            const links = getResultLinks();
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                if (links.length > 0) {
                    selectedIndex = Math.min(selectedIndex + 1, links.length - 1);
                    updateSelection();
                }
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                if (links.length > 0) {
                    selectedIndex = Math.max(selectedIndex - 1, -1);
                    updateSelection();
                }
            } else if (e.key === 'Enter') {
                if (selectedIndex >= 0 && selectedIndex < links.length) {
                    e.preventDefault();
                    links[selectedIndex].click();
                }
            } else if (e.key === 'Escape') {
                searchInput.value = '';
                renderSearchResults([]);
                searchInput.blur();
            }
        });

        // Close when clicking outside.
        document.addEventListener('click', (e) => {
            const container = document.getElementById('search-container');
            if (container && !container.contains(e.target)) {
                renderSearchResults([]);
            }
        });

        // Ctrl+K to focus search.
        document.addEventListener('keydown', (e) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                searchInput.focus();
                loadSearchIndex();
            }
        });
    }

    // Run setup when DOM is ready.
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setupSearch);
    } else {
        // DOM already ready, wait a bit for htmx to load the search container.
        setTimeout(setupSearch, 100);
    }

    // Also setup after htmx swaps in new content.
    document.addEventListener('htmx:afterSwap', (e) => {
        if (e.detail.target.querySelector('#search-input')) {
            setupSearch();
        }
    });
})();
