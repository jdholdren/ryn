# RYN

**ASCII Terminal Roguelike · Rust · Solo Dev**

---

## Setting

It's the late 1990s in a small Midwest town. Strip malls, cornfields, wood-paneled basements, and dial-up internet. Everything is normal — except it isn't. Androids work at gas stations. Aliens turn up in cornfields. Nobody makes a big deal about it. The weird has always been here; people just don't talk about it much.

*The twist: Earthbound-style "ordinary town, extraordinary strangeness" — but set in the flat, lonely geography of the rural Midwest, where the emptiness makes every strange thing feel bigger.*

## Story

You play as Ryn, a high schooler who walks out of after-school detention to find the world has changed. Something happened while you were stuck in that room, and now each area of town is controlled by a boss — a local who has captured a small alien and is using its power. The aliens aren't the enemy. The people holding them hostage are.

Your job is to defeat each boss, free the alien, and figure out what happened while you were writing "I will not talk in class" fifty times on a chalkboard. Dying is part of the cycle. Each death resets the world but not your knowledge — the environments, enemy configurations, and boss powers shuffle with each run. The story unfolds across deaths, not in spite of them.

## Design Goals

| Goal | Why It Matters |
|------|----------------|
| **The weird is mundane** | Sci-fi elements should feel lived-in, not shocking. An android at the diner is Tuesday. This grounds the tone and makes the genuinely strange moments land harder. |
| **Combat is a moment, not a grind** | Earthbound-style menu combat as full-screen encounters. Each fight should feel like an event. Weak enemies auto-resolve. Boss fights are setpieces. |
| **Lore is discovered, not delivered** | Short fragments tied to objects and places. A journal collects them. The player reconstructs the story themselves. No exposition dumps. |
| **Death teaches** | Each run reveals something new — a lore fragment, a boss pattern, a shortcut. The roguelike loop is the narrative structure, not a punishment. |
| **Restraint over ambition** | A focused game that does a few things well. Every feature must serve the core loop of explore, fight, discover, die, learn. |

## Core Mechanics

**Exploration:** Top-down ASCII roguelike grid. The player navigates procedurally generated areas of the town — the school, main street, the mall, the cornfields. Each area leads to a boss. The map, enemy placement, and lore fragments shuffle each run.

**Combat:** Earthbound-style full-screen takeover. The player selects from Attack, Special, Items, Guard, and Run. Enemies are drawn in ASCII art. Descriptions are wry and specific to the Midwest setting. Boss powers change each run based on the alien they've captured.

**Lore:** Found on objects — inscriptions, journals, screens, overheard conversations. Stored in a journal with categories that emerge from the world. Fragments are short (a paragraph or two). Ambient flavor text in the message log for atmosphere.

**Death cycle:** Dying returns Ryn to detention. The world re-shuffles. Certain knowledge and journal entries persist. The story advances through accumulated discovery across runs.

## Development Milestones

| # | Milestone | What's In It |
|---|-----------|-------------|
| 1 | **Walking Around** | `@` moves on a hardcoded map. Walls block movement. Terminal renders with crossterm. Message line at top, stats at bottom. |
| 2 | **Something to Fight** | One enemy type that chases you. Bump to enter Earthbound-style combat screen. HP, damage, death. Dying resets the map. |
| 3 | **A Reason to Explore** | Procedural map generation. Lore objects you can examine. A two-line message log. Items (health potion equivalent). |
| 4 | **The First Boss** | One complete area with a boss encounter. Boss has a captured alien that changes its power set. Victory frees the alien. |
| 5 | **The Loop** | Death returns to detention. World reshuffles. Journal persists across runs. Second area and boss. The game has a loop. |

## Fallback Designs

If Earthbound-style combat proves too complex early on, fall back to bump-to-attack with a combat log. The full-screen battle system can be layered in later. If procedural generation is too slow to get right, start with handcrafted maps and add generation once the core loop is solid. If the journal system stalls development, lore can live entirely in the message log until a dedicated screen is justified.

## Not In This Game

No branching dialogue trees. No crafting system. No multiplayer. No persistent upgrades between runs (knowledge is the upgrade). No overworld map — areas connect directly. These may sound cool, but each one dilutes focus. A game that includes everything is about nothing.
