//! Client-side search JavaScript generation.

/// Generate the client-side search JavaScript.
pub fn generate_search_js() -> String {
    SEARCH_JS.to_string()
}

/// The built-in search page template, rendered to `search/index.html`.
///
/// A collection can replace it with a `search.html` template of its own.
pub const SEARCH_PAGE_TEMPLATE: &str = SEARCH_PAGE;

const SEARCH_JS: &str = r#"/**
 * Folio Client-Side Search
 * Provides instant search functionality for static sites.
 */
(function() {
  'use strict';

  // Split a query into lowercase words, as the index splits documents.
  function words(text) {
    return text.toLowerCase()
      .split(/[^\p{L}\p{N}_'’]+/u)
      .filter(w => w.length >= 2);
  }

  // Search index class.
  //
  // There is no stemmer here. The index maps every word that appears in the
  // documents to the stem it was indexed under (`vocabulary`, for words that
  // differ from their stem), so a query word is looked up rather than stemmed,
  // and the browser and `folio` cannot disagree about a stem.
  class SearchIndex {
    constructor(data) {
      this.documents = data.documents || [];
      this.wordIndex = data.word_index || {};
      this.docLengths = data.doc_lengths || [];
      this.vocabulary = data.vocabulary || {};
      this.surfaceWords = Object.keys(this.vocabulary).concat(Object.keys(this.wordIndex));
    }

    // The stem a word in the documents was indexed under, if it was.
    stemOf(word) {
      if (Object.hasOwn(this.vocabulary, word)) return this.vocabulary[word];
      if (Object.hasOwn(this.wordIndex, word)) return word;
      return null;
    }

    search(query, maxResults = 20) {
      const queryWords = words(query);
      if (queryWords.length === 0) return [];

      // Weight per stem: whole words count fully. The last word may still be
      // being typed, so words it is the start of count for half.
      const weights = new Map();
      queryWords.forEach((word, i) => {
        const stem = this.stemOf(word);
        if (stem) weights.set(stem, 1);
        if (i === queryWords.length - 1) {
          for (const surface of this.surfaceWords) {
            if (surface !== word && surface.startsWith(word)) {
              const prefixStem = this.stemOf(surface);
              if (!weights.has(prefixStem)) weights.set(prefixStem, 0.5);
            }
          }
        }
      });

      const scores = new Map();
      const numDocs = this.documents.length;
      const avgDocLen = this.docLengths.reduce((a, b) => a + b, 0) / numDocs || 1;
      const k1 = 1.2;
      const b = 0.75;

      for (const [stem, weight] of weights) {
        const postings = this.wordIndex[stem] || [];
        const df = postings.length;
        const idf = Math.log((numDocs - df + 0.5) / (df + 0.5) + 1);

        for (const [docIdx, count] of postings) {
          const tf = count * weight;
          const docLen = this.docLengths[docIdx] || 1;
          const score = idf * (tf * (k1 + 1)) / (tf + k1 * (1 - b + b * docLen / avgDocLen));
          scores.set(docIdx, (scores.get(docIdx) || 0) + score);
        }
      }

      // Sort and return top results.
      return Array.from(scores.entries())
        .sort((a, b) => b[1] - a[1])
        .slice(0, maxResults)
        .map(([idx, score]) => ({
          ...this.documents[idx],
          score: Math.round(score * 100)
        }));
    }
  }

  // Global search instance.
  let searchIndex = null;

  // Initialize search.
  async function initSearch(indexUrl = '/search-index.json') {
    try {
      const response = await fetch(indexUrl);
      const data = await response.json();
      searchIndex = new SearchIndex(data);
      return true;
    } catch (e) {
      console.error('Failed to load search index:', e);
      return false;
    }
  }

  // Perform search.
  function search(query) {
    if (!searchIndex) {
      console.warn('Search index not loaded');
      return [];
    }
    return searchIndex.search(query);
  }

  // Debounce helper.
  function debounce(fn, delay) {
    let timeout;
    return function(...args) {
      clearTimeout(timeout);
      timeout = setTimeout(() => fn.apply(this, args), delay);
    };
  }

  // Render search results.
  function renderResults(results, container) {
    if (results.length === 0) {
      container.innerHTML = '<p class="no-results">No results found.</p>';
      return;
    }

    container.innerHTML = results.map(r => `
      <article class="search-result">
        <h3><a href="/${r.slug}/">${escapeHtml(r.title)}</a></h3>
        <p class="preview">${escapeHtml(r.content_preview)}</p>
        <p class="meta">
          ${r.tags.map(t => `<span class="tag">${escapeHtml(t)}</span>`).join(' ')}
        </p>
      </article>
    `).join('');
  }

  function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
  }

  // Auto-bind search form.
  function bindSearchForm(formSelector = '#search-form', resultsSelector = '#search-results') {
    const form = document.querySelector(formSelector);
    const results = document.querySelector(resultsSelector);
    if (!form || !results) return;

    const input = form.querySelector('input[type="search"], input[type="text"]');
    if (!input) return;

    const doSearch = debounce(() => {
      const query = input.value.trim();
      if (query.length < 2) {
        results.innerHTML = '';
        return;
      }
      const searchResults = search(query);
      renderResults(searchResults, results);
    }, 150);

    input.addEventListener('input', doSearch);
    form.addEventListener('submit', (e) => {
      e.preventDefault();
      doSearch();
    });
  }

  // Export API.
  window.FolioSearch = {
    init: initSearch,
    search: search,
    bind: bindSearchForm,
    renderResults: renderResults
  };

  // Auto-init on DOMContentLoaded.
  document.addEventListener('DOMContentLoaded', async () => {
    if (await initSearch()) {
      bindSearchForm();
    }
  });
})();
"#;

const SEARCH_PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Search - {{ site_title }}</title>
    <style>
        :root {
            --bg: #fff;
            --fg: #333;
            --fg-muted: #666;
            --accent: #0066cc;
            --code-bg: #f5f5f5;
            --border: #e0e0e0;
        }
        @media (prefers-color-scheme: dark) {
            :root {
                --bg: #1a1a1a;
                --fg: #e0e0e0;
                --fg-muted: #999;
                --accent: #6db3f2;
                --code-bg: #2d2d2d;
                --border: #444;
            }
        }
        * { box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            line-height: 1.6;
            max-width: 800px;
            margin: 0 auto;
            padding: 2rem;
            color: var(--fg);
            background: var(--bg);
        }
        h1 { color: var(--fg); }
        a { color: var(--accent); text-decoration: none; }
        a:hover { text-decoration: underline; }

        #search-form {
            margin: 2rem 0;
        }
        #search-form input {
            width: 100%;
            padding: 0.75rem 1rem;
            font-size: 1.1rem;
            border: 2px solid var(--border);
            border-radius: 4px;
            background: var(--bg);
            color: var(--fg);
        }
        #search-form input:focus {
            outline: none;
            border-color: var(--accent);
        }

        .search-result {
            padding: 1rem 0;
            border-bottom: 1px solid var(--border);
        }
        .search-result h3 {
            margin: 0 0 0.5rem 0;
        }
        .search-result .preview {
            color: var(--fg-muted);
            margin: 0.5rem 0;
        }
        .search-result .tag {
            display: inline-block;
            background: var(--code-bg);
            padding: 0.1rem 0.4rem;
            border-radius: 3px;
            font-size: 0.8rem;
            margin-right: 0.3rem;
        }
        .no-results {
            color: var(--fg-muted);
            font-style: italic;
        }

        header {
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 1px solid var(--border);
        }
        header nav a { font-weight: bold; font-size: 1.2rem; }
    </style>
</head>
<body>
    <header>
        <nav><a href="/">{{ site_title }}</a></nav>
    </header>
    <main>
        <h1>Search</h1>
        <form id="search-form">
            <input type="search" placeholder="Search documents..." autofocus>
        </form>
        <div id="search-results"></div>
    </main>
    <script src="/search.js" integrity="{{ integrity.search_js }}"></script>
</body>
</html>
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_search_js() {
        let js = generate_search_js();
        assert!(js.contains("FolioSearch"));
        assert!(js.contains("initSearch"));
        assert!(js.contains("vocabulary"));
        assert!(!js.contains("function stem"), "the browser must not stem on its own");
    }

    #[test]
    fn the_search_page_has_what_the_script_binds_to() {
        let html = SEARCH_PAGE_TEMPLATE;
        assert!(html.contains("search-form"));
        assert!(html.contains("search-results"));
        assert!(html.contains("search.js"));
    }
}
