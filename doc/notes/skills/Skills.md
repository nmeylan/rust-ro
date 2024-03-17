5 category of skills:

- Active: cloaking, quagmire, 
- Passive: axe mastery, fatal blow
- Offensive: bash, vulcan arrow
- Performance: songs
- Supportive: blessing, twohand quicken, heal


Skill can have modifier bonus from another skill:
- bash + fatal blow

Skills can have prerequisite skill
-  asura strike: fury state + spirit sphere

Skills can have modifier bonus from items:
- musical strike + oriental lute

Skills can have modifier bonus from cards:
- double strafe + dragon tail card


# Notes

## Behavior
shieldboomerang: 
- shield refinement should be use after applying modifier for cards and element
- does not consider star crumb
- ignore weapon refinement bonus

investigate:
- ignore weapon refinement bonus

guillotine fist:
- ignore weapon refinement bonus

rapid smith:
- ignore weapon refinement bonus
- shield refinement should be use after applying modifier for cards and element

## Misc
Rely on item and card script to add bonuses, compute bonus on skill use

Chain of command:
- Ensure requirements are fulfilled
  - map allows skill usage
  - player state allows skill usage
  - target is targetable
  - area is targetable
  - player has the skill assigned
  - player has the skill level assigned
  - player fulfill skill requirement (level, weapon, ammo, spirit sphere, etc...) <- per skill
  - 
- Offensive: Skill damage calculation
  - Consider weapon
  - Consider buff
  - Consider passive
  - Consider card
- Offensive: Damage target
- Performance
  - Apply buff to area
- Supportive
  - Apply buff to target
- Skill effect
- Client update
  - self
  - target
  - area

# to add to skill config
- dmgAtk
- dmgAtk per level
- dmgMatk
- dmgMatk per level
- dmgInt
- dmgInt per level
- chance
- chance per level
- heal
- heal per level
- accuracy per level
- knockback
- knockback per level
- aoe size
- aoe size per level
- aoe splash
- aoe splash per level
- blind chance
- blind chance per level
- freeze chance
- freeze chance per level
- item break change chance
- item break change  per level
- stun chance
- stun chance per level
- poison chance
- poison chance per level
- snare chance
- snare chance per level
- block chance
- block chance per level
- reflect chance
- reflect chance per level
- divest chance
- divest chance per level
- hp loss percentage
- hp loss percentage per level
- consume item on failure
- acd on failure

- damage bonus (soul link, skill known e.g: sonic acceleration)
- damage malus (if woe,)

# to add in rust trait
- dmg formula

