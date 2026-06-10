```
╔══════════════════════════════════════════╗
║   🌱  i d l e _ g a r d e n  🌱         ║
║   ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~       ║
║   the conservation idle game             ║
╚══════════════════════════════════════════╝
generations. ecosystems. compost.
```

## 🧭 Navigation
- [What is this?](#-what-is-this)
- [The Three Resources](#-the-three-resources)
- [The Garden](#-the-garden)
- [The Subsystems](#-the-subsystems-each-is-its-own-game)
- [Generations & Property](#-generations--property--two-decoupled-axes)
- [Game Loop](#-game-loop)
- [Design Principles](#-design-principles)
- [Concept Roadmap](#-concept-roadmap-high-level-not-committed)
- [QoL Backlog](#-qol-backlog)
- [Inspired By](#-inspired-by)

---

> **Status:** concept phase
> **Genre:** idle / incremental / simulation
> **Inspiration:** NGU Idle, Unnamed Space Idle, [idle_fish](../idle_fish) (predecessor)

---

## 🌍 What is this?

An idle game about **backyard conservation**. You tend a garden across multiple generations of the same family, each one inheriting what the last built. The garden hosts a collection of small ecosystems — a pond, a beehive, a bird house, a compost heap — each one its own little simulation with its own mechanics. Manage them well and your **Conservation Rating** goes up. Save up enough money, move to a bigger property. Pass the garden down whenever you want. Your kid finds your notebook and keeps going.

---

## 🎯 The Three Resources

Most idle games orbit two or three core resources. `idle_garden` has three, and they operate on different timescales:

| Resource | Loop | What it's for |
|---|---|---|
| 💰 **Money** | Short (minutes) | Buys upgrades within a subsystem. Also the gate for buying the next property. Sources: garage sales, honey, eggs, surplus produce. |
| 🌳 **Conservation** | Medium (hours) | The *unlocked surface area* of the game. New subsystems, new slots, new mechanics. This is the main thing you're playing for in any given session. |
| 📜 **Legacy** | Long (generations) | Carries between rebirths. Permanent bonuses, family knowledge, ancestral specialisations. |

Breadth being a resource is the important one. You're not just stacking multipliers — you're unlocking *new things to do*. Every meaningful progression milestone gives the player a new toy.

---

## 🏡 The Garden

The main view is a pixel-art garden. It is also the menu. There are no tab buttons — you click on the pond to enter the pond, the beehive to enter the beehive, the shed to enter the shop. The garden visually reflects everything you've unlocked: empty plot at the start, gradually filling with structures, plants, wildlife, paths, fences, weather effects.

```
   ┌─────────────────────────────────────────┐
   │   🌳         ☁                    🦋   │
   │      🏠   ╔══════╗                      │
   │           ║ 🐝🐝 ║      🌻🌻🌻         │  ← bee tab
   │           ╚══════╝      🌻🌻🌻         │  ← flower tab
   │                                          │
   │     🪺                ░░░░░░             │  ← bird tab
   │   ┌─────┐           ░  🐠  ░             │  ← pond tab
   │   │ 🥕  │           ░░░░░░░             │
   │   │veg  │              🐜                │  ← veg tab, ant tab
   │   └─────┘                                │
   │                                          │
   └─────────────────────────────────────────┘
```

The garden has a finite number of **slots** per property tier. You don't unlock everything at the council flat — you make choices about what to focus on, and saving up for the next house gives you more room.

---

## 🔬 The Subsystems (each is its own game)

The core design principle: **every subsystem has a different verb.**, variety is our main priority, each system should feel individual.

| Subsystem | Verb | What you're actually doing | Status |
|---|---|---|---|
| 🐠 **Pond** | *Balance* | Water chemistry, fish wellness, ecosystem stability | Ported from idle_fish |
| 🐝 **Beehive** | *Build* | Place comb cells on a hex grid, queen at centre, route foragers | Concept |
| 🪺 **Bird house** | *Attract* | Match house design + food + location to attract specific species | Concept |
| 🪱 **Compost** | *Layer* | Stack materials with different decay rates, produce graded soil | Concept |
| 🌻 **Flower bed** | *Schedule* | Plan seasonal bloom cycles so pollinators always have food | Concept |
| 🐜 **Ant nest** | *Grow* | Tunnel expansion idle, assign roles, the most NGU-flavoured one | Concept |
| 🥕 **Veg patch** | *Rotate* | Crop rotation, soil depletion, yield curves | Concept |

Each subsystem is **self-contained**. You can sit down on a Sunday afternoon, open `beehive/`, and just work on bee simulation code without thinking about the rest of the game.
Each subsystem should feed into 1 of the 3 core resource in some way, to allow garden builds for specific things.

### Subsystem integration contract

This is a hard architectural rule for sanity reasons (a direct lesson from `idle_fish`):

**Adding a new subsystem must not require editing shared enums, central match arms, or core game files.** Each subsystem registers itself through a small stable interface — something like:

```
trait Subsystem {
    fn tick(&mut self, dt) -> SubsystemOutput;
    fn ui(&mut self, ctx);
    fn conservation_contribution(&self) -> f64;
    fn produces(&self) -> Vec<ResourceFlow>;
    fn consumes(&self) -> Vec<ResourceFlow>;
}
```

…or whatever the right shape ends up being. The point is: a new subsystem is a new file in a new directory that *plugs in*, not a new arm in seven enums scattered across the codebase. Building Bird House on a Sunday should not involve touching Beehive code.

This also makes the project a nicer thing to come back to after a week of day-job work, which is the whole point.

### Interconnection

Subsystems consume and produce each other's outputs:

```
   compost ──soil──► veg patch ──surplus──► money
      ▲                  │
      │                  └──flowers──► flower bed ◄──pollinators── beehive
      │                                    │                            │
   organic                                 └────nectar───────────────────┘
   matter                                                       │
      │                                                         ▼
   veg/flowers ─────────────────────────────────────────────► honey ──► money

   pond ──irrigation──► veg patch
   pond ──insects──► bird house
   bird house ──pest control──► veg patch
```

The point is that *upgrading your compost makes your veg better which makes your money go up which lets you afford a better beehive which pollinates more flowers which attracts more butterflies which raises your Conservation Rating*.

Because of the integration contract above, these flows are declared by each subsystem (`produces` / `consumes`) rather than hardcoded into a central place. New subsystems opt into the flow graph; they don't extend a master switch statement.

---

## 👴 Generations & Property — Two Decoupled Axes

There are **two separate progression axes** that interact but don't lock to each other:

### Axis 1: Generations (player-driven, low friction)

Rebirth whenever you want. There is no Conservation Rating threshold, no countdown, no death timer. The player chooses to retire when it feels right.

When you retire:
- Permanent structures and mature plants stay (the oak Grandad planted is still there)
- Active simulations reset (the bees move on, the pond is drained, a new generation of fish)
- You unlock a **Legacy bonus** based on what you focused on this generation
- A new family member is generated (procedural name + a short procedurally-written life story summary)
- The family tree gains an entry

Generations are *cheap*. Many of them can happen in the same property. A player who likes the rhythm of cycling through retirements can run through five or six generations in the same council flat before they ever upgrade. A player who hates rebirthing can play the same character indefinitely.

```
   👴 Walter (gen 1) ───► loved bees. Permanent: +10% honey yield.
        │  (still in council flat)
        ▼
   👩 Susan (gen 2) ────► expanded the pond. Permanent: +1 pond slot.
        │  (still in council flat)
        ▼
   🧑 Mark (gen 3) ─────► saved like hell. Bought the semi.
        │  (NOW at suburban semi)
        ▼
   👧 You (gen 4) ──────► ?
```

### Axis 2: Property (money-gated, big milestone)

The actual *long-arc* gate. Buying the next property costs serious money — the kind of money you save up across multiple generations. Each property unlocks:

- More slots in the garden
- New subsystem *types* (woodland tier, wetland tier, etc.)
- A new pixel-art background
- A bigger horizon for the family story

| Property tier | Slots | New subsystem types unlocked |
|---|---|---|
| Council flat balcony | 3 | Flower box, herb pot, bird feeder |
| Suburban semi | 6 | + Beehive, compost, small veg patch |
| Cottage with garden | 9 | + Pond, ant nest, full veg rotation |
| Smallholding | 12 | + Woodland edge, wild meadow, hedgerow |
| Rewilded acreage | 15 | + Wetland, orchard, dry stone wall |
| … | … | … |

Money carries across generations (it's an inheritance), so saving for the next property is a deliberate long-term goal that survives retirements. The "we finally bought the cottage" moment is the dramatic family milestone, not Walter's death.

### Axis 3 (placeholder): Family Line Reset

Eventually there's room for a *bigger* rebirth — wipe the whole family line, restart from scratch with some persistent meta-bonus. Not designing it yet. Mentioning it so the structure leaves room for it later.

### What carries / what resets

| | Generation rebirth | Property purchase | Family line reset (future) |
|---|---|---|---|
| Money | Mostly (inheritance) | Spent on the house | Reset |
| Legacy bonuses | Keep + add new one | Keep all | Convert to meta-bonus, then reset |
| Property tier | Keep | **Upgrade** | Reset to flat |
| Active subsystem state | Reset | Mostly reset | Reset |
| Permanent structures (mature trees, etc.) | Keep | Keep + new slots | Reset |
| Family tree | Append new entry | Append milestone | Archive, start new line |

---

## 🔁 Game Loop

```
Open the game: see your garden. Some subsystems running, some empty slots.
         │
         ▼
Click into a subsystem → play the mini-game → outputs flow into other systems
         │
         ▼
Earn money → upgrade subsystems → unlock breadth → new things appear in the garden
         │
         ▼
Conservation Rating climbs → milestones → cosmetic + mechanical unlocks
         │
         ▼
   ┌─────┴──────────────────┐
   │                        │
   ▼                        ▼
Retire whenever         Save up for next property
(Legacy bonus,          (huge milestone,
 family story,           more slots,
 fresh sims)             new subsystem types)
   │                        │
   └──────────┬─────────────┘
              │
              └──────► repeat, with more land each time
```

---

## 💡 Design Principles

A few rules to keep yourself honest as features get added:

1. **Every subsystem must have its own verb.** If a new subsystem is just "balance the meter," it doesn't ship.
2. **Subsystems must plug in, not patch in.** No editing shared enums or central match arms to add one. If it requires that, the architecture is wrong.
3. **Subsystems must be self-contained at the file level too.** You should be able to work on one without opening files belonging to another.
4. **Interconnections must map to real ecology.** Bees pollinate flowers, compost feeds veg. Don't invent mechanical links that don't pass the smell test — they're what make the theme work.
5. **Breadth over depth on the first pass.** Better to have six shallow subsystems that all *work* than one polished one and five broken ones. (This is the opposite of how `idle_fish` ended up.)
6. **Generation passes should feel emotional, not bureaucratic.** The family story is the moment players bond with the game. Spend disproportionate effort there.
7. **Generations are free, properties are earned.** Don't gate rebirth. Gate the house.

---

## 📋 Concept Roadmap (high-level, not committed)

Written ground-up in Rust with minimal external dependencies where reasonable. Engine before content, architecture before game design.
 
### Phase 0 — Engine foundations
- [x] Project skeleton, workspace layout, minimal dependency choices
- [x] Decide directory structure (subsystems live in their own dirs from day one)
- [x] Core `GameState` struct sketched out with stub fields for known systems
- [x] Other scaffolding structs that need to be initially present
### Phase 1 — Save/load
- [x] Port save/load from `idle_fish`
- [x] Fix the missing-file panic (graceful new-game fallback)
- [x] Round-trip test: save → load → state matches
### Phase 2 — Page architecture & navigation
- [x] Page trait / interface (whatever shape ends up right)
- [x] Page registry & swapping mechanism
- [x] Garden view as the "home" page with clickable hotspots
- [x] Stub pages for at least two subsystems to prove swap works
- [x] Back-to-garden navigation pattern
### Phase 3 — Running game loop
- [x] Main loop scaffolding
- [ ] Tick rate decision (real-time vs accumulator-based)
- [x] Pause / resume hooks
- [x] Offline catch-up handler (with the offline-cap limit from idle_fish backlog)
### Phase 4 — Tick loop & subsystem contract
- [x] Subsystem plug-in interface (the integration contract — trait shape decided here)
- [x] Tick dispatch: GameState tells each registered subsystem to advance
- [x] Resource flow declaration (`produces` / `consumes`)
- [x] Confirm: registering a stub subsystem requires zero edits to shared enums
### Phase 5 — Game design pass
- [ ] Subsystem-by-subsystem design docs (verb, inputs, outputs, win/fail conditions)
- [ ] Resource flow graph for Council Flat tier
- [ ] Conservation Rating formula draft
- [ ] Economy curve sketch (income vs costs across early game)
### Phase 6 — First real subsystem (Pond)
- [ ] Port pond from `idle_fish` into the new contract
- [ ] Pond outputs flow into GameState via `produces`
- [ ] Garden visual reflects unlocked pond
### Phase 7 — Prove the plug-in model (Beehive + Bird house)
- [ ] Beehive (build-verb prototype)
- [ ] Bird house (attract-verb prototype)
- [ ] First real interconnection: bees ↔ flowers
- [ ] Confirm adding bird house didn't require touching pond or bee code
### Phase 8 — Generational layer
- [ ] Retirement flow (player-triggered)
- [ ] Procedural family-story generator
- [ ] Family tree view
- [ ] Legacy bonus system
- [ ] Inheritance carry-over rules
### Phase 9 — Property progression
- [x] Property purchase mechanic (money gate)
- [x] Council flat → suburban semi transition
- [ ] More slots, new subsystem types unlocked at semi tier
### Phase 10 — Council Flat MVP polish
- [ ] Remaining subsystems for flat tier
- [ ] Conservation Rating calculation finalised
- [ ] Balance pass
- [ ] First playable release
### Phase 11 — Beyond
- [ ] Further property tiers
- [ ] Family line reset (the meta-prestige)
- [ ] Pixel art polish
- [ ] Whatever the simulation gremlin in the dev's brain wants to build that week

---

## 🛠 QoL Backlog

### 🌻 Bed Subsystem
- [ ] **Harvest all button** — single click to harvest all spots that are ready rather than clicking each one individually
- [ ] **Sticky selection** — selected item stays selected until either a different item is chosen or the player runs out of that item, so planting a full bed of grass seeds doesn't require re-clicking after every spot

---

## 🙏 Inspired By

- **NGU Idle** — for the "your resources gate different things" framing and feature-unlock-as-progression
- **Unnamed Space Idle** — for interconnected subsystems where everything feeds everything else
- **Stardew Valley** — for the "tending a place across time" feeling
- **idle_fish** — for the cautionary tale, the pond code, and the lesson about plug-in architecture

---

*plant trees you won't sit under* 🌳
