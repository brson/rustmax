// Build the sidebar's module tree from the crate's sidebar-tree.js.
//
// The list carries the page's path to the output root in `data-root`, which
// prefixes every link, and the path of the module to mark in `data-current`.
(function() {
    var list = document.querySelector('.sidebar .module-tree');
    var tree = window.rmxSidebarTree;
    if (!list || !tree) return;
    var root = list.dataset.root;
    var current = list.dataset.current;

    function build(node) {
        var li = document.createElement('li');
        if (current && node[2] === current) li.className = 'current';
        var a = document.createElement('a');
        a.href = root + node[1];
        a.textContent = node[0];
        li.appendChild(a);
        if (node[3].length) {
            var ul = document.createElement('ul');
            node[3].forEach(function(child) { ul.appendChild(build(child)); });
            li.appendChild(ul);
        }
        return li;
    }

    list.appendChild(build(tree));
})();
