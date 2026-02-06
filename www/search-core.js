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
// Substring matches require a word boundary: the match position must
// be at the start of the target or preceded by a non-alphanumeric character.
// This prevents false positives like "time" matching inside "runtime".
function getMatch(query, target) {
    var q = query.toLowerCase();
    var t = target.toLowerCase();

    if (t === q) return { type: MATCH_EXACT, score: 1.0 };
    if (t.startsWith(q)) return { type: MATCH_PREFIX, score: 0.9 };

    // Word-boundary substring match.
    var pos = 0;
    while (pos < t.length) {
        var idx = t.indexOf(q, pos);
        if (idx === -1) break;
        // Accept if at start of string or preceded by a non-alphanumeric char.
        if (idx === 0 || !/[a-z0-9]/.test(t[idx - 1])) {
            return { type: MATCH_SUBSTRING, score: 0.6 };
        }
        pos = idx + 1;
    }

    return null;
}

// Find best match for query against an entry.
// Returns { score, matchedText, matchType } or null.
//
// Aliases in the searchable field are pipe-separated:
//   "name|alias one|alias two|..."
// Each alias is matched individually so multi-word aliases work correctly
// and cross-alias false positives are eliminated.
function findMatch(query, entry) {
    var parts = entry.searchable.split('|');
    var name = parts[0];

    var bestScore = 0;
    var bestText = null;
    var bestType = null;

    // Try matching name.
    var nameMatch = getMatch(query, name);
    if (nameMatch) {
        bestScore = nameMatch.score;
        bestType = nameMatch.type;
        // matchedText stays null for name matches since the name is already displayed.
    }

    // Try matching each alias, keep the best.
    for (var i = 1; i < parts.length; i++) {
        var alias = parts[i];
        if (!alias) continue;
        var m = getMatch(query, alias);
        if (m && m.score > bestScore) {
            bestScore = m.score;
            bestText = alias;
            bestType = m.type;
        }
    }

    if (bestScore > 0) {
        return { score: bestScore, matchedText: bestText, matchType: bestType };
    }

    return null;
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
