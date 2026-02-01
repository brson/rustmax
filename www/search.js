// Topic search functionality
(function() {
    'use strict';

    let searchIndex = null;
    let searchInput = null;
    let searchResults = null;
    let isLoading = false;

    // Fuzzy scoring: returns 0-1, higher is better match.
    function fuzzyScore(query, target) {
        query = query.toLowerCase();
        target = target.toLowerCase();

        // Exact match.
        if (target === query) return 1.0;

        // Prefix match (strong).
        if (target.startsWith(query)) return 0.9;

        // Word-start match ("ws" matches "web server").
        const words = target.split(/[\s\-_]+/);
        const queryChars = query.split('');
        let wordIdx = 0, charIdx = 0;
        while (wordIdx < words.length && charIdx < queryChars.length) {
            if (words[wordIdx].startsWith(queryChars[charIdx])) charIdx++;
            wordIdx++;
        }
        if (charIdx === queryChars.length) return 0.8;

        // Substring match.
        if (target.includes(query)) return 0.6;

        // Character sequence match (all chars appear in order).
        let ti = 0;
        for (const c of query) {
            ti = target.indexOf(c, ti);
            if (ti === -1) return 0;
            ti++;
        }
        return 0.3;
    }

    // Category weights for ranking.
    const categoryWeights = {
        'crate': 1.5,
        'book': 1.3,
        'std': 1.1,
    };

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

    // Perform search and return ranked results.
    function performSearch(query) {
        if (!searchIndex || !query.trim()) return [];

        const results = [];
        const seen = new Set();

        for (const entry of searchIndex) {
            const score = fuzzyScore(query, entry.searchable);
            if (score < 0.2) continue;

            // Apply category weight.
            const categoryWeight = categoryWeights[entry.category] || 1.0;
            const finalScore = score * categoryWeight;

            // Deduplicate by id.
            if (seen.has(entry.id)) continue;
            seen.add(entry.id);

            results.push({ entry, score: finalScore });
        }

        // Sort by score descending.
        results.sort((a, b) => b.score - a.score);

        return results.slice(0, 20);
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
        for (const { entry, score } of results) {
            const cat = entry.category;
            if (!groups[cat]) groups[cat] = [];
            groups[cat].push({ entry, score });
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
            for (const { entry } of items) {
                const href = entry.path || '#';
                html += `<a class="search-result" href="${href}">
                    <span class="search-result-name">${entry.name}</span>
                    <span class="search-result-brief">${entry.brief}</span>
                </a>`;
            }
        }

        searchResults.innerHTML = html;
        searchResults.classList.add('visible');
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
                const results = performSearch(query);
                renderSearchResults(results);
            }, 100);
        });

        // Close on escape.
        searchInput.addEventListener('keydown', (e) => {
            if (e.key === 'Escape') {
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
