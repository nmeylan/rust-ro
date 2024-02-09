Character have statuses that contains its the current state like (str, agi, hp, max hp, matk, etc...)

Character Status comes from multiple sources:

* Allocated status point (str, agi,vit, dex, int, luk)
* Bonus from Item with static script (e.g: wearing Glove[1]: bonus bDex,1;, with Zerom card bonus bDex,3;)
* Bonus from item with dynamic script (e.g: venatu card bonus bLuk,readparam(bAgi)/18;)
* Temporary bonus from Item script (e.g: berserk potion sc_start SC_ASPDPOTION2,1800000,0;)
* Bonus from supportive skills (e.g: Blessing)
* Bonus from passive skills
* Bonus from performance skills
* Formula (hit, flee, crit, aspd, etc...)
* Combination of formula + bonus

When a character is involved inside the game loop, when character is attacked or do attack for example, we need to known its statuses.

When used within the game loop we don't want the character state to be mutable. In addition we want the character state to reflect its actual state, with all statuses filled (total str including bonuses, weapon element, armor element, state like stun or frozen).

To do that we will take a "snapshot" of the character state at the beginning of the game loop.

so we have:
* the **status**: contains base stats (str, agi, vit... ) assigned by the player, wear items with their refinement, slotted card, buff, debuff, state (frozen, stun...)
* the **status snapshot**: contains the calculated stats value (total str, agi, vit) using the formula and computing all items + card + buff +... etc

The snapshot is then use in battle/skill calculation, the snapshot is valid only for the current game loop iteration and recreated at each iteration when the character is involved.

