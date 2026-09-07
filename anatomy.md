---
title: anatomy
tags: cyb, core
alias: robot anatomy, anatomy of the robot, cyb anatomy
crystal-type: reference
crystal-domain: cyb
---
# anatomy — the source of truth

the robot is one organism with **20 parts in six layers**. every surface — the app's worlds, the docs, [cyb.ai](https://cyb.ai), the crates — names things by this page. a page that cannot be traced to a part is either legacy or component matter.

## I · identity — who

| part | is | today |
|---|---|---|
| **name** | the NFT resolver — the robot's name resolves through the graph ([[moon-passport]] lineage), owned like a token, not set in a config | ports with the soft3 genesis |
| **avatar** | the robot's visualization — its model, the rendered creature others see and you recognize | to grow; the robot world's creature is its seed |
| **soul** | the main config: one file defining processing when the robot is asked — which model, which dialect, what it may do unasked | `~/cyb/soul` to define; soma settings are its embryo |

## II · mind — what thinks

| part | is | today |
|---|---|---|
| **soma** | mind and model: local inference, weights, silicon | live — soma kernel + honeycrisp, `? q` in com |
| **brain** | the rendered graph | live as the world currently named *graph* (mir, 100+ fps) → renames to **brain** |
| **memory** | particles rendered as a file system — table or tiles, tap to read | half-live: tap-to-read pages + [[cyb/root/fs|fs]] become its spec; a projection of brain, one key flips brain ⇄ memory |

## III · senses and speech — what perceives and says

| part | is | today |
|---|---|---|
| **com** | the commander — the one line where the user types; the mouth of the robot, reads [[cyb/anatomy|soul]] on every ask | live (com world); becomes omnipresent chrome, not a world |
| **now** | the context indicator — which particle the robot stands on, shown top-left | to build; chrome element |
| **sense** | the messenger — interaction with other neurons, particles, robots | seed exists (`money_to_sense`, notices); grows into the robot's inbox/outbox |
| **voice** | speech — the robot heard and speaking | to grow |
| **vision** | sight — camera, screen, world | to grow |

## IV · economy — what it owns

| part | is | today |
|---|---|---|
| **sigma** | not a wallet: the sum of all tokens in possession, and the management of neurons (identities) | live (sigma world); gains the neurons screen; the word *wallet* is banned |
| **vault** | secrets and sleeping neurons: keys, mnemonics, TOTP | live — XChaCha20 under mnemonic + TOTP; sigma spends, vault holds and signs |

## V · time — what it did and will do

| part | is | today |
|---|---|---|
| **log** | the history of every interaction | live in substance: the durable tape `~/cyb/graph.log` *is* the log |
| **plan** | the schedule: standing orders, deferred intents | seed exists (mining standing order); generalizes |
| **time** | one screen: log ← **now** → plan, the present in the middle | to build; the flagship view |

## VI · substrate — what it stands on

| part | is | today |
|---|---|---|
| **body** | the physical body: silicon, sensors, energy, mining — telemetry and resources of the machine the robot lives on | live (default world: telemetry, erga child, PUSSY/day) |
| **cell** | an organ-extension: a live-loaded program that grows the robot a new ability | live (rune cell runtime). ⚠ distinct from the protocol's [[cell]] (a 4D particle group) — the robot grows *cells as organs* |
| **radio** | the physical layer of communication with external networks; [[sense]] speaks over radio | live — the cyber-radio transport; the wire obeys the graph: follow/antenna/socket are cyberlinks |
| **state** | verified perception of external networks: every answer is a value **plus a tier** (T0 proof-verified · T1 anchor-verified · T3 unproven RPC, badged); one query IR, a proof-router, local verification over provider truth | doctrine written — [[cyb/state|state]] |

## deliberately absent

**core.** we looked for a hidden lower layer and found none. the robot's immortality needs no extra organ — it is the backup: *name + soul + vault seed + log*. carry those four to any machine and the robot resurrects. the `cyb-core` crate keeps its name as code, not as anatomy.

## banned words

*wallet* → [[cyb/anatomy|sigma]] · *oracle, portal, old brain* → legacy (the JS era) · *graph (as a world name)* → brain · *face* → avatar

## alignment phases

1. **docs** — [[cyb/root/robot|robot]], [[cyb/root/spec|spec]], [[cyb/root/product|product]], [[cyb/root/os|os]] restate themselves as expansions of parts; anything untraceable goes legacy (per [[restructure]])
2. **app** — world *graph* → *brain*; com becomes chrome; *now* indicator; *time* world from tape + standing orders
3. **landing** — cyb.ai lists the anatomy, one line per part
4. **code** — crates and modules adopt part names; `~/cyb/soul` file is born

the parts are the contract. the order of growth is the roadmap's business — but the names are settled here.
