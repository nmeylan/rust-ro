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