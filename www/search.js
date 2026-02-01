// Topic search functionality
(function() {
    'use strict';

    let searchIndex = null;
    let searchInput = null;
    let searchResults = null;
    let isLoading = false;

    // Match types in priority order.
    const MATCH_EXACT = 'exact';
    const MATCH_PREFIX = 'prefix';
    const MATCH_WORD_START = 'initials';
    const MATCH_SUBSTRING = 'substring';
    const MATCH_SEQUENCE = 'fuzzy';

    // Check match type for a query against a target string.
    // Returns { type, score } or null if no match.
    function getMatch(query, target) {
        const q = query.toLowerCase();
        const t = target.toLowerCase();

        if (t === q) return { type: MATCH_EXACT, score: 1.0 };
        if (t.startsWith(q)) return { type: MATCH_PREFIX, score: 0.9 };

        // Word-start match.
        const words = t.split(/[\s\-_]+/);
        const queryChars = q.split('');
        let wordIdx = 0, charIdx = 0;
        while (wordIdx < words.length && charIdx < queryChars.length) {
            if (words[wordIdx].startsWith(queryChars[charIdx])) charIdx++;
            wordIdx++;
        }
        if (charIdx === queryChars.length) return { type: MATCH_WORD_START, score: 0.8 };

        if (t.includes(q)) return { type: MATCH_SUBSTRING, score: 0.6 };

        // Character sequence match.
        let ti = 0;
        for (const c of q) {
            ti = t.indexOf(c, ti);
            if (ti === -1) return null;
            ti++;
        }
        return { type: MATCH_SEQUENCE, score: 0.3 };
    }

    // Find best match for query against an entry.
    // Returns { score, matchedText, matchType } or null.
    function findMatch(query, entry) {
        // Parse searchable into name and aliases.
        // Format: "name alias1 alias2 ..." but we need to check each part.
        // Since we don't have structured data, try name first, then search for alias matches.
        const name = entry.name;
        const searchable = entry.searchable;

        // Try matching name first.
        const nameMatch = getMatch(query, name);
        if (nameMatch && nameMatch.score >= 0.6) {
            return { score: nameMatch.score, matchedText: null, matchType: nameMatch.type };
        }

        // Try matching the full searchable text.
        const fullMatch = getMatch(query, searchable);
        if (!fullMatch) return null;

        // Find which alias matched (if not the name).
        // Extract aliases: everything after the name in searchable.
        const aliasText = searchable.slice(name.length).trim();
        if (aliasText) {
            const aliases = aliasText.split(/\s+/);
            // Find the best matching alias.
            let bestAlias = null;
            let bestAliasScore = 0;
            for (const alias of aliases) {
                const m = getMatch(query, alias);
                if (m && m.score > bestAliasScore) {
                    bestAliasScore = m.score;
                    bestAlias = alias;
                }
            }
            if (bestAlias && bestAliasScore >= fullMatch.score * 0.8) {
                return { score: fullMatch.score, matchedText: bestAlias, matchType: fullMatch.type };
            }
        }

        // Matched via combined text.
        return { score: fullMatch.score, matchedText: null, matchType: fullMatch.type };
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
            const match = findMatch(query, entry);
            if (!match || match.score < 0.2) continue;

            // Apply category weight.
            const categoryWeight = categoryWeights[entry.category] || 1.0;
            const finalScore = match.score * categoryWeight;

            // Deduplicate by id.
            if (seen.has(entry.id)) continue;
            seen.add(entry.id);

            results.push({
                entry,
                score: finalScore,
                matchedText: match.matchedText,
                matchType: match.matchType,
            });
        }

        // Sort by score descending.
        results.sort((a, b) => b.score - a.score);

        return results.slice(0, 20);
    }

    // Format match explanation.
    function formatMatchInfo(matchedText, matchType) {
        if (matchedText) {
            // Matched an alias.
            return `aka "${matchedText}"`;
        }
        if (matchType === MATCH_WORD_START) {
            return 'initials';
        }
        return null;
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
            for (const { entry, matchedText, matchType } of items) {
                const href = entry.path || '#';
                const matchInfo = formatMatchInfo(matchedText, matchType);
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
