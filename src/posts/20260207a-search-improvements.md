---
title: "Search improvements"
category: news
summary: "Keyboard navigation, CLI search, improved matching, and other fixes."
---

Lots of search improvements since the initial launch last week.

**Keyboard navigation** -
Arrow keys now move through search results
and enter navigates to the selected result.

**CLI search** -
`rustmax search <query>` runs the same search algorithm
from the command line, outputting results as TOML.
Running the CLI outside the repo mostly doesn't actually work though.

**Better matching** -
The search algorithm and metadata is improved in many ways,
and should be minimally useful now.

## Other changes

- Updated lockfile past cargo audit vulnerabilities in `bytes` and `time` crates
- Enabled heading IDs in the rustmax-rustdoc markdown renderer, fixing broken anchor links
- Updated template and guide examples to use `profile-portable`.
  It is my prefered default profile now.
- Reorganized guide.md with usage first, reference lists last

