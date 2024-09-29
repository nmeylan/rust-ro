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

magic rod:
- block enemy skill and receive percentage of sp consumed by enemy to use the blocked skill -> we need to get sp cost of the skill

BD_SIEGFRIED:
- should include resistance from element

BD_LULLABY:
- sleep chance formula

KN_SPEARMASTERY:
- riding bonus 

KN_CAVALIERMASTERY:
- aspd penalty reduction while riding (like performance + music lesson)

AS_RIGHT:
- atk penatly reduction on right hand while dual weilding (like performance + music lesson)
AS_LEFT :
- atk penatly reduction on left hand while dual weilding (like performance + music lesson)

BS_HILTBINDING:
- increase duration of Adrenaline Rush, Power Thrust, and Weapon Perfection by 10%.

TF_MISS:
- movement for assassin only

WZ_QUAGMIRE:
- Mitigate effect of some skills (improve concentration, one hand quicken, increase agi...) while in the zone
- Max 3 active quagmire per wizard -> unit should have a limit

MO_EXTREMITYFIST:
- Guillotine Fist may be cast once within a short window after Raging Thrust or Chain Crush Combo. When cast this way, the Spirit Sphere catalyst is reduced by 2 and 4 respectively and the cast time is removed. The window base duration is 1.3 seconds, and is decreased by AGI and DEX.

## Skill bonus
- negate penalty (music lesson negate speed penalty when using performance skill)
## Misc
Rely on item and card script to add bonuses, compute bonus on skill use

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


Skill db struct:
- BonusesToSelf
- BonusesToTarget
- BonusesToParty
- BonusesToSkill (for offensive skill)

Skill bonus config:

Should we have:
- conditional on skill active (fatal blow stun chance bonus when SM_BASH used, music lesson speed bonus when Performance skill use)
- conditional on skill known (SM_BASH stun chance bonus when fatal blow known, Performance speed bonus when music lesson known)
- conditional on class (TF_MISS speed bonus for assassin)
- conditional on wear weapon