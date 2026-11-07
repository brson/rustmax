// rustmax code browser.
//
// Symbol data and the search index are JS files that call back into
// this script, instead of JSON fetched with XHR, so that the site
// works when opened from `file://` URLs.

(function () {
    'use strict';

    const root = document.body.dataset.root;
    const assets = root + '_rmx/';
    const SHARD_SIZE = 256;

    function escapeHtml(s) {
        return s.replace(/[&<>"]/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
    }

    function loadScript(src) {
        const script = document.createElement('script');
        script.src = src;
        document.head.appendChild(script);
        return script;
    }

    // Symbol data.

    const shards = new Map();
    const shardResolvers = new Map();

    window.rmxSymbols = function (n, data) {
        shardResolvers.get(n)(data);
    };

    function loadShard(n) {
        if (!shards.has(n)) {
            shards.set(n, new Promise((resolve, reject) => {
                shardResolvers.set(n, resolve);
                loadScript(assets + 'sym/' + n + '.js').onerror = reject;
            }));
        }
        return shards.get(n);
    }

    function symbol(id) {
        return loadShard(Math.floor(id / SHARD_SIZE)).then(shard => shard[id]);
    }

    // Hover cards.

    const card = document.getElementById('card');
    let showTimer = null;
    let hideTimer = null;
    let cardFor = null;

    function linkOrText(text, href) {
        if (!href) return escapeHtml(text);
        return '<a href="' + escapeHtml(root + href) + '">' + escapeHtml(text) + '</a>';
    }

    // A titled list of locations, grouped by file.
    function locationsHtml(title, locations) {
        if (!locations.length) return '';
        let html = '<h3 class="rb-section">' + escapeHtml(title) + ' (' + locations.length + ')</h3>';
        let currentFile = null;
        for (const [path, line, snippet] of locations) {
            if (path !== currentFile) {
                if (currentFile !== null) html += '</ol>';
                html += '<h4><a href="' + escapeHtml(root + path) + '.html">' + escapeHtml(path) + '</a></h4><ol>';
                currentFile = path;
            }
            html += '<li><a href="' + escapeHtml(root + path) + '.html#L' + line + '">'
                + '<span class="rb-lnum">' + line + '</span><span>' + escapeHtml(snippet) + '</span></a></li>';
        }
        return html + '</ol>';
    }

    function cardHtml(id, s) {
        let meta = '';
        if (s.k) meta += '<span class="rb-kind">' + escapeHtml(s.k) + '</span>';
        if (s.pkg) meta += '<span>' + escapeHtml(s.pkg) + '</span>';
        if (s.def) meta += '<a href="' + escapeHtml(root + s.def) + '">definition</a>';
        if (s.url) meta += '<a href="' + escapeHtml(s.url) + '">docs</a>';
        meta += '<button data-refs="' + id + '">' + s.refs.length + ' references</button>';
        if (s.imps.length) meta += '<button data-refs="' + id + '">' + s.imps.length + ' implementations</button>';
        if (s.impls.length) meta += '<button data-refs="' + id + '">' + s.impls.length + ' impls</button>';
        let html = '<div class="rb-meta">' + meta + '</div>';
        if (s.implements) html += '<div class="rb-path">implements ' + linkOrText(s.implements.p, s.implements.def) + '</div>';
        html += '<div class="rb-path">' + escapeHtml(s.p) + '</div>';
        if (s.sig) html += '<pre><code>' + s.sig + '</code></pre>';
        if (s.doc) html += '<div class="rb-doc">' + s.doc + '</div>';
        return html;
    }

    function placeCard(el) {
        const rect = el.getBoundingClientRect();
        card.hidden = false;
        const width = card.offsetWidth;
        const left = Math.max(4, Math.min(rect.left, document.documentElement.clientWidth - width - 8));
        card.style.left = (left + window.scrollX) + 'px';
        card.style.top = (rect.bottom + window.scrollY + 4) + 'px';
    }

    function showCard(el) {
        const id = Number(el.dataset.s);
        cardFor = el;
        symbol(id).then(s => {
            if (cardFor !== el || !s) return;
            card.innerHTML = cardHtml(id, s);
            placeCard(el);
        });
    }

    function hideCardSoon() {
        clearTimeout(showTimer);
        clearTimeout(hideTimer);
        hideTimer = setTimeout(() => {
            card.hidden = true;
            cardFor = null;
        }, 300);
    }

    // Locals.

    let locals = [];
    const localsEl = document.getElementById('locals');
    if (localsEl) locals = JSON.parse(localsEl.textContent);

    function highlightLocal(n, on) {
        document.querySelectorAll('a[data-l="' + n + '"]').forEach(a => a.classList.toggle('hl', on));
    }

    document.addEventListener('mouseover', e => {
        const local = e.target.closest('a[data-l]');
        if (local) {
            const info = locals[Number(local.dataset.l)];
            if (info && info.sig && !local.title) local.title = info.sig;
            highlightLocal(local.dataset.l, true);
            return;
        }
        const el = e.target.closest('a[data-s]');
        if (el) {
            clearTimeout(hideTimer);
            clearTimeout(showTimer);
            showTimer = setTimeout(() => showCard(el), 350);
        }
    });

    document.addEventListener('mouseout', e => {
        const local = e.target.closest('a[data-l]');
        if (local) highlightLocal(local.dataset.l, false);
        if (e.target.closest('a[data-s]')) hideCardSoon();
    });

    card.addEventListener('mouseenter', () => clearTimeout(hideTimer));
    card.addEventListener('mouseleave', hideCardSoon);

    // References panel.

    const panel = document.getElementById('refs');

    function showRefs(id) {
        symbol(id).then(s => {
            if (!s) return;
            let html = '<button class="rb-close" title="Close (Esc)">close</button>';
            html += '<h2>' + escapeHtml(s.p) + '</h2>';
            let meta = s.k ? escapeHtml(s.k) : '';
            if (s.def) meta += ', <a href="' + escapeHtml(root + s.def) + '">definition</a>';
            if (s.url) meta += ', <a href="' + escapeHtml(s.url) + '">docs</a>';
            if (s.implements) meta += ', implements ' + linkOrText(s.implements.p, s.implements.def);
            html += '<div class="rb-kind">' + meta + '</div>';
            html += locationsHtml('Implementations', s.imps);
            html += locationsHtml('Impls', s.impls);
            html += locationsHtml('References', s.refs);
            if (!s.refs.length && !s.imps.length && !s.impls.length) html += '<p>No references.</p>';
            panel.innerHTML = html;
            panel.hidden = false;
            panel.scrollTop = 0;
        });
    }

    document.addEventListener('click', e => {
        const def = e.target.closest('a.d[data-s]');
        if (def) {
            e.preventDefault();
            showRefs(Number(def.dataset.s));
            return;
        }
        const button = e.target.closest('button[data-refs]');
        if (button) {
            card.hidden = true;
            showRefs(Number(button.dataset.refs));
            return;
        }
        if (e.target.closest('#refs .rb-close')) panel.hidden = true;
    });

    // Search.

    const input = document.getElementById('search');
    const results = document.getElementById('search-results');
    let index = null;
    let indexRequested = false;
    let active = 0;

    window.rmxSearchIndex = function (entries) {
        index = entries.map(e => ({
            name: e[0], path: e[1], kind: e[2], href: e[3],
            lname: e[0].toLowerCase(), lpath: e[1].toLowerCase(),
        }));
        runSearch();
    };

    function requestIndex() {
        if (!indexRequested) {
            indexRequested = true;
            loadScript(assets + 'search.js');
        }
    }

    function rank(entry, q, pathQuery) {
        if (pathQuery) return entry.lpath.includes(q) ? 3 : -1;
        if (entry.lname === q) return 0;
        if (entry.lname.startsWith(q)) return 1;
        if (entry.lname.includes(q)) return 2;
        if (entry.lpath.includes(q)) return 3;
        return -1;
    }

    function runSearch() {
        const q = input.value.trim().toLowerCase();
        if (!index || !q) {
            results.hidden = true;
            return;
        }
        const pathQuery = q.includes('::') || q.includes('/');
        const matches = [];
        for (const entry of index) {
            const r = rank(entry, q, pathQuery);
            if (r >= 0) matches.push([r, entry]);
        }
        matches.sort((a, b) => a[0] - b[0] || a[1].path.length - b[1].path.length);
        active = 0;
        results.innerHTML = matches.slice(0, 50).map(([, e], i) =>
            '<li' + (i === 0 ? ' class="active"' : '') + '><a href="' + escapeHtml(root + e.href) + '">'
            + '<span class="rb-name">' + escapeHtml(e.name) + '</span> <span class="rb-kind">' + escapeHtml(e.kind) + '</span>'
            + '<span class="rb-path">' + escapeHtml(e.path) + '</span></a></li>'
        ).join('') || '<li><a>No results</a></li>';
        results.hidden = false;
    }

    function setActive(n) {
        const items = results.querySelectorAll('li');
        if (!items.length) return;
        items[active].classList.remove('active');
        active = (n + items.length) % items.length;
        items[active].classList.add('active');
        items[active].scrollIntoView({ block: 'nearest' });
    }

    input.addEventListener('focus', () => { requestIndex(); runSearch(); });
    input.addEventListener('input', runSearch);
    input.addEventListener('blur', () => setTimeout(() => { results.hidden = true; }, 200));
    input.addEventListener('keydown', e => {
        if (e.key === 'ArrowDown') { setActive(active + 1); e.preventDefault(); }
        else if (e.key === 'ArrowUp') { setActive(active - 1); e.preventDefault(); }
        else if (e.key === 'Enter') {
            const link = results.querySelectorAll('li a')[active];
            if (link && link.href) window.location.href = link.href;
        } else if (e.key === 'Escape') {
            input.blur();
        }
    });

    // Outline, open or closed as last left.

    const outline = document.querySelector('.rb-outline');
    if (outline) {
        if (localStorage.getItem('rb-outline') === 'closed') outline.open = false;
        outline.addEventListener('toggle', () => {
            localStorage.setItem('rb-outline', outline.open ? 'open' : 'closed');
        });
    }

    document.addEventListener('keydown', e => {
        if (e.target === input) return;
        if (e.key === 'o' && outline && !e.ctrlKey && !e.metaKey && !e.altKey) {
            outline.open = !outline.open;
            return;
        }
        if (e.key === '/' || e.key === 's') {
            input.focus();
            e.preventDefault();
        } else if (e.key === 'Escape') {
            panel.hidden = true;
            card.hidden = true;
        }
    });
})();
