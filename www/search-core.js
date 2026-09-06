// Core search logic shared between browser and CLI.
//
// In the browser, loaded as a <script> before search.js,
// making these functions available as globals.
// In Node.js, loaded via require() for the parity test.
//
// crates/rustmax-cli/src/search.rs is a port of this file and must stay at
// parity with it. See botdocs/search.md.

'use strict';

// Match types, best first.
var MATCH_EXACT = 'exact';
var MATCH_PREFIX = 'prefix';
var MATCH_SUBSTRING = 'substring';

var matchScores = {
    'exact': 1.0,
    'prefix': 0.9,
    'substring': 0.6,
};

// Ordering over match types, for picking the better or worse of two.
var matchRanks = {
    'exact': 3,
    'prefix': 2,
    'substring': 1,
};

// Check how a query matches a target string.
// Returns a match type, or null if there is no match.
//
// Substring matches require a word boundary: the match position must be at
// the start of the target or preceded by a non-alphanumeric character.
// This prevents false positives like "time" matching inside "runtime".
function getMatch(query, target) {
    var q = query.toLowerCase();
    var t = target.toLowerCase();

    if (t === q) return MATCH_EXACT;
    if (t.startsWith(q)) return MATCH_PREFIX;

    // An empty query is a prefix of everything, so it never gets this far.
    // Guarding anyway keeps the scan below from spinning.
    if (q.length === 0) return null;

    var pos = 0;
    while (pos < t.length) {
        var idx = t.indexOf(q, pos);
        if (idx === -1) break;
        // Accept if at start of string or preceded by a non-alphanumeric char.
        if (idx === 0 || !/[a-z0-9]/.test(t[idx - 1])) {
            return MATCH_SUBSTRING;
        }
        pos = idx + 1;
    }

    return null;
}

// Split text into tokens on ASCII whitespace.
//
// Deliberately ASCII-only rather than a \s regex: the Rust port has to
// split identically, and the two languages disagree about the edges of
// Unicode whitespace.
function splitTokens(text) {
    var tokens = [];
    var current = '';
    for (var i = 0; i < text.length; i++) {
        var c = text.charAt(i);
        if (c === ' ' || c === '\t' || c === '\n' || c === '\r') {
            if (current.length > 0) {
                tokens.push(current);
                current = '';
            }
        } else {
            current += c;
        }
    }
    if (current.length > 0) tokens.push(current);
    return tokens;
}

// Work out the forms of a query that get matched against the index.
function prepareQuery(query) {
    var tokens = splitTokens(query);

    // Single-character tokens are dropped from token matching. They match
    // most of the index, so "read a file" would otherwise hinge on "a".
    var searchTokens = [];
    for (var i = 0; i < tokens.length; i++) {
        if (tokens[i].length >= 2) searchTokens.push(tokens[i]);
    }

    return {
        // The query as typed, trimmed with runs of whitespace collapsed.
        normalized: tokens.join(' '),
        // The same with the spaces taken out, so "hash map" finds "HashMap".
        joined: tokens.join(''),
        tokens: searchTokens,
        multiWord: tokens.length > 1,
    };
}

// Match every token somewhere in the entry, each possibly in a different
// alias. Returns the weakest of the per-token match types, or null if any
// token matches nothing.
//
// Scoring by the weakest token means a scattered match can never outrank a
// literal match of the same quality.
function matchAllTokens(tokens, parts) {
    var worst = null;

    for (var i = 0; i < tokens.length; i++) {
        var bestForToken = null;
        for (var j = 0; j < parts.length; j++) {
            if (j > 0 && !parts[j]) continue;
            var m = getMatch(tokens[i], parts[j]);
            if (m !== null && (bestForToken === null || matchRanks[m] > matchRanks[bestForToken])) {
                bestForToken = m;
            }
        }
        if (bestForToken === null) return null;
        if (worst === null || matchRanks[bestForToken] < matchRanks[worst]) {
            worst = bestForToken;
        }
    }

    return worst;
}

// Find best match for a prepared query against an entry.
// Returns { score, matchedText, matchType } or null.
//
// Aliases in the searchable field are pipe-separated:
//   "name|alias one|alias two|..."
// Each alias is matched individually so multi-word aliases work correctly
// and cross-alias false positives are eliminated.
//
// Three ways to match are tried, best result wins. Ties go to whichever
// was tried first, so a literal match beats a spaces-removed one, which
// beats a scattered token match.
function findMatch(prepared, entry) {
    var parts = entry.searchable.split('|');
    var best = null;

    function consider(matchType, matchedText) {
        if (matchType === null) return;
        var score = matchScores[matchType];
        if (best === null || score > best.score) {
            best = { score: score, matchedText: matchedText, matchType: matchType };
        }
    }

    // matchedText stays null for name matches since the name is displayed.
    function considerAll(query) {
        consider(getMatch(query, parts[0]), null);
        for (var i = 1; i < parts.length; i++) {
            if (!parts[i]) continue;
            consider(getMatch(query, parts[i]), parts[i]);
        }
    }

    considerAll(prepared.normalized);

    if (prepared.multiWord) {
        considerAll(prepared.joined);
    }

    // No single alias explains a scattered match, so it reports no alias.
    if (prepared.tokens.length >= 2) {
        consider(matchAllTokens(prepared.tokens, parts), null);
    }

    return best;
}

// Category weights for ranking.
// Standard library modules rank highest since they are the foundational
// building blocks. Crates extend them, and books explain them.
//
// These only break ties between equally good matches. The weights stay
// within a ratio of 1.0/0.9, the smallest gap between two match types, so
// a weight can never promote a worse match over a better one: an exact
// match on a book still beats a prefix match on a std module.
var categoryWeights = {
    'std': 1.10,
    'crate': 1.05,
    'book': 1.00,
};

// Perform search and return ranked results.
function performSearch(searchIndex, query) {
    if (!searchIndex) return [];

    var prepared = prepareQuery(query);
    if (prepared.normalized.length === 0) return [];

    var results = [];
    var seen = {};

    for (var i = 0; i < searchIndex.length; i++) {
        var entry = searchIndex[i];
        var match = findMatch(prepared, entry);
        if (!match) continue;

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

    // Sort by score descending. Array.sort is stable, so equal scores keep
    // the index's own order, which the Rust port relies on too.
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
