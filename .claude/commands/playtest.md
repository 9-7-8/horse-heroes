---
description: Give the owner up to four in-game tests of built work, record her results, then end the session
argument-hint: "[issue number or a subject, optional]"
---

Read `procedures/playtest.txt` and follow it exactly; do not work from
memory of it. It picks up to four things that are built and wait only on
her trying them (the next release's first, menus first), gives her checked
steps for each, asks her results with the question tool, records them on
the issues, closes what passes, files a bug for what fails, and then runs
`procedures/end-session.txt` without being asked.

Tests only, never design decisions (those are /answers). It records what
she saw and fixes nothing.

If anything follows this line it is the owner's steer for this run (an issue
number, or a subject); it narrows the pick to that: $ARGUMENTS
