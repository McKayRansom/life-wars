
# Lifewars

Lifewars is a RTS game where you fight using Cellular Automatons.
Discover and save interesting patterns and strategically deploy them to win battles!

## Features
- Performant CA algorithms: Simulate huge battles at high speed
- Fully featured Editors: Create patterns and save them
- Support for outside file formats: .RLA, plaintext, and apgcode compatible to export and share your designs elsewhere, or import designs from LifeWiki or elsewhere


## Required features:
- [ ] Pattern hotbar with keys 1-9 (move speed to shift or faster/slower buttons?)
- [ ] AI Pattern library easy/medium/hard
- [ ] Winning/losing (Resign, victory screen with stats and graph?, global win/loss stats?)
- [ ] Pattern cost/resource rethink, Respawn timer OR fixed counts

## Bugs:
- [ ] Rotation broken, just use other code
- [ ] Ui Selected color broken (why is macroquad's UI SO BAD!!!)

## Completed features
- [x] Life "Star wars" simulation with factions, some perf opt and lots of research into that
- [x] Save designs in lots of different formats
- [x] Functional editor
- [x] Reasonable pattern placement with view preview centered on cursor


## Future directions
- Custom patterns
- Run searches for patterns
- Multiplayer!
- Strategic game mode or huge maps


## Thoughts after first playtest 3/20/25:
- AI deploying REAL patterns is awesome! Really adds a ton
- Need an easier way to switch patterns... Hotbar?
- zooming is a bit clunky, minimap?
- Maybe you can only place patterns so far from your side? Or based on if you are winning?

Maybe features:
- Achevements or tracking of wins on easy/medium/hard
- Achievements on Pattern finding? (is this a different game???)

## Thoughts on population 3/22/25:

Specifying a pattern cost seems exteremly difficult and easy to cheese by pattern collisions

Alternatives:
- Max pop count, use it wisely. Replicators are automatically a large commmitment of troops. Basic ship very little
- Cost associated with birthing a cell that is returned upon death (similar to above?)
- Max pop AND placement 'energy' cost?
- Some hybrid approach: There should be a cost to lay down a pattern to avoid HIGH-APM requirement AND there should be a limited POP to avoid replicator abuse
- Simplest possible approach: 
 - You get 1 placement every X seconds (this stacks if you don't use it)
 - This makes every placement very strategic
 - Encourages large or replicator placements in safe locations
 - Encourages wild patterns (maybe this is okay?)


Avoid High-APM requirement:
- Slow speed?
- Small maps?
- Limited pattern placement!?
- But also placing individual ships to block attacks is fun

## Thoughts on 3/3/25:
- Deploying fighters/bombers/frigates/dreadnaughts was the most fun (not just putting down death stars willy nilly)
  - Axe custom patterns (for MVP, revisit later)
  - Just have those, limited number OR recharge timer (try both see what feels good)
  - Smaller maps will probably work better so you can take down those frigates, large maps can feel overwhelming but could also be fun
  - Toolbar and # shortcuts for deploying
- 
