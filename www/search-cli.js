#!/usr/bin/env node

// CLI wrapper for the search-core.js search logic.
//
// Usage: node search-cli.js <index-path> <query>
//
// Reads the pre-processed search-index.json,
// runs the same search algorithm as the website,
// and outputs results as JSON to stdout.

'use strict';

var path = require('path');
var fs = require('fs');
var core = require(path.join(__dirname, 'search-core.js'));

var args = process.argv.slice(2);
if (args.length < 2) {
    process.stderr.write('usage: node search-cli.js <index-path> <query>\n');
    process.exit(1);
}

var indexPath = args[0];
var query = args.slice(1).join(' ');

var indexData = fs.readFileSync(indexPath, 'utf8');
var searchIndex = JSON.parse(indexData);
var results = core.performSearch(searchIndex, query);

// Output as JSON array.
process.stdout.write(JSON.stringify(results) + '\n');
