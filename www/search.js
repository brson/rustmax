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

    // Render search results grouped by category.
    function renderSearchResults(results) {
        if (!searchResults) return;

        if (results.length === 0) {
            searchResults.innerHTML = '';
            searchResults.classList.remove('visible');
            return;
        }

        // Group by category.
        const groups = {};
        for (const result of results) {
            const cat = result.entry.category;
            if (!groups[cat]) groups[cat] = [];
            groups[cat].push(result);
        }

        // Render order: crate, book, std.
        const order = ['crate', 'book', 'std'];
        const sortedCats = Object.keys(groups).sort((a, b) => {
            const ai = order.indexOf(a);
            const bi = order.indexOf(b);
            return (ai === -1 ? 99 : ai) - (bi === -1 ? 99 : bi);
        });

        let html = '';
        for (const cat of sortedCats) {
            const items = groups[cat];
            html += `<div class="search-category">${cat}</div>`;
            for (const { entry, matchedText } of items) {
                const href = entry.path || '#';
                const matchInfo = formatMatchInfo(matchedText);
                const matchHtml = matchInfo
                    ? `<span class="search-match-info">${matchInfo}</span>`
                    : '';
                html += `<a class="search-result" href="${href}">
                    <span class="search-result-name">${entry.name}${matchHtml}</span>
                    <span class="search-result-brief">${entry.brief}</span>
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
