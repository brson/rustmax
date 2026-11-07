//! Layered layout of the workspace crate graph, rendered as SVG.
//!
//! Packages are placed in rows so that every package sits above the packages it
//! depends on, then each row is reordered a few times by the mean position of
//! its neighbors to reduce crossings. Dev-dependencies are drawn but do not
//! affect the layering, since they may point back up the graph.

use crate::highlight::escape;
use std::fmt::Write;

pub struct Node {
    pub name: String,
    pub href: String,
}

/// `from` depends on `to`.
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub dev: bool,
}

const CHAR_WIDTH: f64 = 7.8;
const NODE_HEIGHT: f64 = 28.0;
const NODE_PADDING: f64 = 12.0;
const H_GAP: f64 = 24.0;
const V_GAP: f64 = 64.0;
const MARGIN: f64 = 8.0;
const ORDERING_SWEEPS: usize = 8;

/// Node rectangles, as `(x, y, width)`, and the total width and height.
pub struct Layout {
    pub boxes: Vec<(f64, f64, f64)>,
    pub width: f64,
    pub height: f64,
}

pub fn layout(nodes: &[Node], edges: &[Edge]) -> Layout {
    let n = nodes.len();

    // Longest path to a sink over non-dev edges. Cargo forbids cycles among
    // those, so this settles within `n` rounds.
    let mut depth = vec![0usize; n];
    for _ in 0..n {
        let mut changed = false;
        for e in edges.iter().filter(|e| !e.dev && e.from != e.to) {
            if depth[e.from] < depth[e.to] + 1 {
                depth[e.from] = depth[e.to] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let max_depth = depth.iter().copied().max().unwrap_or(0);
    let mut rows: Vec<Vec<usize>> = vec![Vec::new(); max_depth + 1];
    let mut by_name: Vec<usize> = (0..n).collect();
    by_name.sort_by(|a, b| nodes[*a].name.cmp(&nodes[*b].name));
    for i in by_name {
        rows[max_depth - depth[i]].push(i);
    }

    let widths: Vec<f64> = nodes
        .iter()
        .map(|node| node.name.chars().count() as f64 * CHAR_WIDTH + 2.0 * NODE_PADDING)
        .collect();
    let row_width = |row: &[usize]| {
        row.iter().map(|i| widths[*i]).sum::<f64>() + H_GAP * row.len().saturating_sub(1) as f64
    };
    let total_width = rows.iter().map(|r| row_width(r)).fold(0.0, f64::max);

    let mut x = vec![0.0; n];
    let place = |row: &[usize], x: &mut [f64]| {
        let mut left = MARGIN + (total_width - row_width(row)) / 2.0;
        for i in row {
            x[*i] = left;
            left += widths[*i] + H_GAP;
        }
    };
    for row in &rows {
        place(row, &mut x);
    }

    let mut neighbors: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in edges.iter().filter(|e| e.from != e.to) {
        neighbors[e.from].push(e.to);
        neighbors[e.to].push(e.from);
    }
    for sweep in 0..ORDERING_SWEEPS {
        let order: Vec<usize> = if sweep % 2 == 0 {
            (0..rows.len()).collect()
        } else {
            (0..rows.len()).rev().collect()
        };
        for r in order {
            let center = |i: usize| x[i] + widths[i] / 2.0;
            let mut keyed: Vec<(f64, usize)> = rows[r]
                .iter()
                .map(|&i| {
                    let ns = &neighbors[i];
                    let key = if ns.is_empty() {
                        center(i)
                    } else {
                        ns.iter().map(|&j| center(j)).sum::<f64>() / ns.len() as f64
                    };
                    (key, i)
                })
                .collect();
            keyed.sort_by(|a, b| a.0.total_cmp(&b.0));
            rows[r] = keyed.into_iter().map(|(_, i)| i).collect();
            place(&rows[r], &mut x);
        }
    }

    let mut boxes = vec![(0.0, 0.0, 0.0); n];
    for (r, row) in rows.iter().enumerate() {
        let y = MARGIN + r as f64 * (NODE_HEIGHT + V_GAP);
        for &i in row {
            boxes[i] = (x[i], y, widths[i]);
        }
    }
    Layout {
        boxes,
        width: total_width + 2.0 * MARGIN,
        height: rows.len() as f64 * (NODE_HEIGHT + V_GAP) - V_GAP + 2.0 * MARGIN,
    }
}

pub fn render_svg(nodes: &[Node], edges: &[Edge]) -> String {
    let layout = layout(nodes, edges);
    let mut svg = String::new();
    let _ = write!(
        svg,
        "<svg class=\"rb-graph\" xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" viewBox=\"0 0 {w:.0} {h:.0}\">",
        w = layout.width,
        h = layout.height,
    );
    svg.push_str(
        "<defs><marker id=\"rb-arrow\" viewBox=\"0 0 10 10\" refX=\"10\" refY=\"5\" markerWidth=\"7\" markerHeight=\"7\" orient=\"auto-start-reverse\"><path d=\"M 0 0 L 10 5 L 0 10 z\"/></marker></defs>",
    );
    for e in edges.iter().filter(|e| e.from != e.to) {
        let (fx, fy, fw) = layout.boxes[e.from];
        let (tx, ty, tw) = layout.boxes[e.to];
        let (x1, x2) = (fx + fw / 2.0, tx + tw / 2.0);
        // Leave from the side facing the target.
        let (y1, y2) = if ty > fy { (fy + NODE_HEIGHT, ty) } else { (fy, ty + NODE_HEIGHT) };
        let bend = (y2 - y1) / 2.0;
        let class = if e.dev { "rb-edge rb-dev" } else { "rb-edge" };
        let _ = write!(
            svg,
            "<path class=\"{class}\" d=\"M {x1:.1} {y1:.1} C {x1:.1} {c1:.1} {x2:.1} {c2:.1} {x2:.1} {y2:.1}\" marker-end=\"url(#rb-arrow)\"/>",
            c1 = y1 + bend,
            c2 = y2 - bend,
        );
    }
    for (node, (x, y, w)) in nodes.iter().zip(&layout.boxes) {
        let _ = write!(
            svg,
            "<a href=\"{href}\"><rect class=\"rb-node\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{w:.1}\" height=\"{NODE_HEIGHT}\" rx=\"4\"/><text x=\"{tx:.1}\" y=\"{ty:.1}\">{name}</text></a>",
            href = escape(&node.href),
            tx = x + w / 2.0,
            ty = y + NODE_HEIGHT / 2.0,
            name = escape(&node.name),
        );
    }
    svg.push_str("</svg>");
    svg
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nodes(names: &[&str]) -> Vec<Node> {
        names.iter().map(|n| Node { name: n.to_string(), href: format!("{n}/") }).collect()
    }

    #[test]
    fn dependents_sit_above_dependencies() {
        let nodes = nodes(&["app", "lib", "core", "tests"]);
        let edges = [
            Edge { from: 0, to: 1, dev: false },
            Edge { from: 1, to: 2, dev: false },
            Edge { from: 0, to: 2, dev: false },
            // A dev edge pointing back up must not change the layering.
            Edge { from: 2, to: 3, dev: true },
            Edge { from: 3, to: 0, dev: false },
        ];
        let layout = layout(&nodes, &edges);
        let y = |i: usize| layout.boxes[i].1;
        assert!(y(3) < y(0));
        assert!(y(0) < y(1));
        assert!(y(1) < y(2));
    }

    #[test]
    fn boxes_in_a_row_do_not_overlap() {
        let nodes = nodes(&["a", "bbbbbbbb", "cc", "top"]);
        let edges = [
            Edge { from: 3, to: 0, dev: false },
            Edge { from: 3, to: 1, dev: false },
            Edge { from: 3, to: 2, dev: false },
        ];
        let layout = layout(&nodes, &edges);
        let mut row: Vec<(f64, f64, f64)> = layout.boxes[..3].to_vec();
        row.sort_by(|a, b| a.0.total_cmp(&b.0));
        for pair in row.windows(2) {
            assert!(pair[0].0 + pair[0].2 <= pair[1].0);
        }
        assert!(row.iter().all(|b| b.0 + b.2 <= layout.width));
    }

    #[test]
    fn svg_escapes_names() {
        let nodes = nodes(&["a<b"]);
        let svg = render_svg(&nodes, &[]);
        assert!(svg.contains("a&lt;b"));
    }
}
