#!/usr/bin/env node

// Regenerates the expected results in search-corpus.json.
//
// The corpus is the parity contract between www/search-core.js and
// ../src/search.rs. It carries its own fixture index and its own queries;
// this script only recomputes the expected results, using search-core.js as
// the reference implementation.
//
// Run it after a deliberate change to the search algorithm, then make the
// matching change in search.rs and check the diff. Do not run it to make a
// failing parity test pass: a diff here means the two implementations have
// drifted apart, which is the thing the corpus exists to catch.
//
// Usage: node gen-search-corpus.js
//
// Run it with `just gen-search-corpus`.

'use strict';

var path = require('path');
var fs = require('fs');

var repoRoot = path.resolve(__dirname, '..', '..', '..');
var core = require(path.join(repoRoot, 'www', 'search-core.js'));
var corpusPath = path.join(__dirname, 'search-corpus.json');
var corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));

corpus.cases = corpus.cases.map(function(testCase) {
    var results = core.performSearch(corpus.index, testCase.query).map(function(r) {
        return {
            id: r.entry.id,
            score: r.score,
            matchType: r.matchType,
            matchedText: r.matchedText === undefined ? null : r.matchedText,
        };
    });
    return { query: testCase.query, results: results };
});

fs.writeFileSync(corpusPath, JSON.stringify(corpus, null, 2) + '\n');
process.stdout.write('regenerated ' + corpus.cases.length + ' cases in ' + corpusPath + '\n');
