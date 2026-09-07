---
title: state
tags: cyb, core
alias: cyb state, state access, state-query, proof-router
crystal-type: reference
crystal-domain: cyb
---
# state — how the robot reads external truth

part of the [[cyb/anatomy|anatomy]] (senses layer — the robot's verified sight into other networks). the notes below are the working doctrine for cyb's access to external network state.

## the task

build a browser whose access to state is better than a wallet's and better than RPC.
core in Rust. not the provider's truth — local verification. where verification is impossible, that fact is visible.

## the principle

every answer to a query is a value **plus a tier**:

- **T0** — we verified consensus and proof ourselves
- **T1** — we verified the anchor on a T0 chain ourselves; execution/leaf is conditional
- **T3** — data from an RPC, no proof

the tier never hides. T0 and T1 are the product. T3 is an admission with a badge — not "network support".

## two axes, not seven "states"

the mess came from putting LN, RGB, names and CIDs next to "blockchains". these are different axes.

**consensus family** — how the tip is fixed:

| family | tip | already exists |
|---|---|---|
| Bitcoin UTXO | headers + filters / utreexo | rust-bitcoin, Floresta, BDK |
| Ethereum | beacon sync committee | Helios, Alloy, revm |
| OP-Stack | L2 state against L1 output | Helios opstack, op-reth as option |
| the rest (Tron, BNB, Solana, Arb/Orbit…) | no rust-light yet | T3 adapter only |

**object** — what is asked of that tip:

coin · channel · CSV contract · account+storage · name · blob.

the family gives the way to prove. the object gives the shape of the query.
a name and a CID are not "more chains": they are pointers that resolve into an object on a family.

## state-query

one IR for everything. external RPCs and p2p collapse into it.

```
Query  { family, loc, object, at? }
Answer { value, proof?, tier, at }
```

`loc` — network or anchor (btc, eth, base, alice.btc, cid).
`object` — outpoint, account, slot, seal, invoice, zonefile, blob.
no tier in the answer — the answer is invalid.

## proof-router

not a pile of clients in the UI. a router: object + family → verification method.

examples: compact filter / utreexo leaf; EIP-1186 account+storage; OP output root; RGB consignment; DNSSEC (BIP-353); BNS zonefile against the Stacks→BTC anchor.

no route with a proof → T3 only, or refusal.
a new protocol = a new route, not a new architecture.

## take, write, refuse

**take** verification, don't reinvent chains: rust-bitcoin, miniscript, BDK, Floresta, LDK, rgb-std, Helios (+opstack), Alloy, revm, BNS/BIP-353 resolution, content by CID.

**write** the glue: the IR, the router, one proof store, the tier badge, name→object, CSV as an object on the Bitcoin family.

**refuse**: no own Core, EVM, Agave, Trongrid.
TAP/LND (Go) — outside the process, never in the core.
bitcoin forks and the museum (Omni, XCP, inscriptions) — an object decoder, not a new family, until a consensus module exists.

## the criterion

the address bar opens an object and shows why it can be trusted.
a T0/T1 verification can be repeated without our server.
best access on the market is not the number of networks — it is the maximal share of queries closed by a proof, on one rust core.
