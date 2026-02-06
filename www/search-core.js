// Core search logic shared between browser and CLI.
//
// In the browser, loaded as a <script> before search.js,
// making these functions available as globals.
// In Node.js, loaded via require() for CLI search.

'use strict';

// Match types.
var MATCH_EXACT = 'exact';
var MATCH_PREFIX = 'prefix';
var MATCH_SUBSTRING = 'substring';

// Check match type for a query against a target string.
// Returns { type, score } or null if no match.
function getMatch(query, target) {
    var q = query.toLowerCase();
    var t = target.toLowerCase();

    if (t === q) return { type: MATCH_EXACT, score: 1.0 };
    if (t.startsWith(q)) return { type: MATCH_PREFIX, score: 0.9 };
    if (t.includes(q)) return { type: MATCH_SUBSTRING, score: 0.6 };

    return null;
}

// Find best match for query against an entry.
// Returns { score, matchedText, matchType } or null.
function findMatch(query, entry) {
    var name = entry.name;
    var searchable = entry.searchable;

    // Try matching name first.
    var nameMatch = getMatch(query, name);
    if (nameMatch && nameMatch.score >= 0.6) {
        return { score: nameMatch.score, matchedText: null, matchType: nameMatch.type };
    }

    // Try matching the full searchable text.
    var fullMatch = getMatch(query, searchable);
    if (!fullMatch) return null;

    // Find which alias matched (if not the name).
    var aliasText = searchable.slice(name.length).trim();
    if (aliasText) {
        var aliases = aliasText.split(/\s+/);
        var bestAlias = null;
        var bestAliasScore = 0;
        for (var i = 0; i < aliases.length; i++) {
            var m = getMatch(query, aliases[i]);
            if (m && m.score > bestAliasScore) {
                bestAliasScore = m.score;
                bestAlias = aliases[i];
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
var categoryWeights = {
    'crate': 1.5,
    'book': 1.3,
    'std': 1.1,
};

// Perform search and return ranked results.
function performSearch(searchIndex, query) {
    if (!searchIndex || !query.trim()) return [];

    var results = [];
    var seen = {};

    for (var i = 0; i < searchIndex.length; i++) {
        var entry = searchIndex[i];
        var match = findMatch(query, entry);
        if (!match || match.score < 0.2) continue;

        // Apply category weight.
        var categoryWeight = categoryWeights[entry.category] || 1.0;
        var finalScore = match.score * categoryWeight;

        // Deduplicate by id.
        if (seen[entry.id]) continue;
        seen[entry.id] = true;

        results.push({
            entry: entry,
            score: finalScore,
            matchedText: match.matchedText,
            matchType: match.matchType,
        });
    }

    // Sort by score descending.
    results.sort(function(a, b) { return b.score - a.score; });

    return results.slice(0, 20);
}

// Format match explanation.
function formatMatchInfo(matchedText) {
    if (matchedText) {
        return 'aka "' + matchedText + '"';
    }
    return null;
}

// Node.js exports.
if (typeof module !== 'undefined' && module.exports) {
    module.exports = { performSearch: performSearch, formatMatchInfo: formatMatchInfo };
}
