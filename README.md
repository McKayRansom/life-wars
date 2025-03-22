
# Lifewars

Lifewars is a RTS game where you fight using Cellular Automatons.
Discover and save interesting patterns and strategically deploy them to win battles!

## Features
- Performant CA algorithms: Simulate huge battles at high speed
- Fully featured Editors: Create patterns and save them
- Support for outside file formats: .RLA, plaintext, and apgcode compatible to export and share your designs elsewhere, or import designs from LifeWiki or elsewhere

## Future directions
- Run searches for patterns
- Multiplayer!
- Strategic game mode or huge maps


## Thoughts after first playtest 3/20/24:
- AI deploying REAL patterns is awesome! Really adds a ton
- Need an easier way to switch patterns... Hotbar?
- zooming is a bit clunky, minimap?
- Maybe you can only place patterns so far from your side? Or based on if you are winning?

## Required features:
-[ ] Pattern hotbar
-[ ] AI Pattern library easy/medium/hard
-[ ] PatternQOL:
 -[ ] Rotate
 -[ ] View preview
 -[ ] Preview direction and/or impact
 -[ ] Canonicalize! (Remove whitespace at least)
 -[ ] Center pattery on cursor
- [ ] Winning/losing
 - [ ] Resign?
 - [ ] Stats (pop history?)
 - [ ] Global stats win/losses 
- [ ] Pattern cost/resource rethink
- [ ] Don't allow patterns to be placed on (enemys OR empty space only?)

Maybe features:
- Achevements or tracking of wins on easy/medium/hard
- Achievements on Pattern finding?

## Thoughts on population 3/22/24:

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
