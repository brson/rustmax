#!/usr/bin/env node

// Checks www/search-core.js against the shared parity corpus.
//
// The Rust port in ../src/search.rs checks itself against the same file,
// so if both this and `cargo test -p rustmax-cli` pass, the browser and the
// CLI agree on every case in the corpus.
//
// Usage: node search-parity.js
//
// Run it with `just test-search-parity`, which skips when node is absent.

'use strict';

var path = require('path');
var fs = require('fs');

var repoRoot = path.resolve(__dirname, '..', '..', '..');
var core = require(path.join(repoRoot, 'www', 'search-core.js'));
var corpusPath = path.join(__dirname, 'search-corpus.json');
var corpus = JSON.parse(fs.readFileSync(corpusPath, 'utf8'));

// Matches SCORE_TOLERANCE in search.rs.
var SCORE_TOLERANCE = 1e-9;

var failures = [];

function describe(rows) {
    return rows.map(function(r) {
        return r.id + '/' + r.matchType + '/' + (r.matchedText === null ? '-' : r.matchedText);
    }).join(', ');
}

corpus.cases.forEach(function(testCase) {
    var got = core.performSearch(corpus.index, testCase.query).map(function(r) {
        return {
            id: r.entry.id,
            score: r.score,
            matchType: r.matchType,
            matchedText: r.matchedText === undefined ? null : r.matchedText,
        };
    });
    var want = testCase.results;

    if (describe(got) !== describe(want)) {
        failures.push('query ' + JSON.stringify(testCase.query) + '\n' +
                      '  got  : ' + describe(got) + '\n' +
                      '  want : ' + describe(want));
        return;
    }

    for (var i = 0; i < want.length; i++) {
        if (Math.abs(got[i].score - want[i].score) >= SCORE_TOLERANCE) {
            failures.push('query ' + JSON.stringify(testCase.query) +
                          ': score for ' + want[i].id +
                          ' is ' + got[i].score + ', corpus says ' + want[i].score);
        }
    }
});

if (failures.length > 0) {
    process.stderr.write(failures.join('\n\n') + '\n\n');
    process.stderr.write(failures.length + ' of ' + corpus.cases.length + ' cases diverged.\n');
    process.stderr.write('search-core.js and src/search.rs must stay at parity.\n');
    process.exit(1);
}

process.stdout.write(corpus.cases.length + ' cases match the parity corpus.\n');
