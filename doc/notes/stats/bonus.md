- bonus given by item/cards
- bonus given by passive skills
- bonus given by supportive skills

pre-re item scripts
```
with d as (
with c as (with b as (with a as (select unnest(string_to_array(script, ';')) as s
from item_db
where script ilike '%bonus%')
select unnest(string_to_array(s, ',')) as s1
from a
where s ilike '%bonus%')
select substring(s1, position('bonus' in s1) + 6) as s2
from b
where s1 ilike '%bonus%')
select trim(substring(s2,position('bonus' in s2))) as s3  from c)
select distinct(s3) from d;
;
```


+------------------------------+
|bAGI                          |
|bAddClass                     |
|bAddDamageClass               |
|bAddDefMonster                |
|bAddEff                       |
|bAddEff2                      |
|bAddEffOnSkill                |
|bAddEffWhenHit                |
|bAddEle                       |
|bAddItemGroupHealRate         |
|bAddItemHealRate              |
|bAddMonsterDropItem           |
|bAddMonsterDropItemGroup      |
|bAddMonsterIdDropItem         |
|bAddRace                      |
|bAddRace2                     |
|bAddSize                      |
|bAddSkillBlow                 |
|bAgi                          |
|bAllStats                     |
|bAspd                         |
|bAspdRate                     |
|bAtkEle                       |
|bAtkRate                      |
|bAutoSpell                    |
|bAutoSpellOnSkill             |
|bAutoSpellWhenHit             |
|bAutoSpellwhenhit             |
|bBaseAtk                      |
|bBreakArmorRate               |
|bBreakWeaponRate              |
|bCastRate                     |
|bCastrate                     |
|bClassChange                  |
|bComaClass                    |
|bComaRace                     |
|bCritAtkRate                  |
|bCritical                     |
|bCriticalAddRace              |
|bCriticalLong                 |
|bDef                          |
|bDef2Rate                     |
|bDefEle                       |
|bDefRate                      |
|bDefRatioAtkClass             |
|bDelayRate                    |
|bDex                          |
|bDoubleRate                   |
|bExpAddClass                  |
|bExpAddRace                   |
|bFlee                         |
|bFlee2                        |
|bGetZenyNum                   |
|bHPDrainRate                  |
|bHPGainValue                  |
|bHPLossRate                   |
|bHPRegenRate                  |
|bHPrecovRate                  |
|bHealPower                    |
|bHealpower2                   |
|bHit                          |
|bHitRate                      |
|bIgnoreDefClass               |
|bIgnoreDefRace                |
|bIgnoreDefRaceRate            |
|bIgnoreMdefClassRate          |
|bIgnoreMdefRaceRate           |
|bInt                          |
|bIntravision                  |
|bLongAtkDef                   |
|bLongAtkRate                  |
|bLuk                          |
|bMAtkRate                     |
|bMagicAddRace                 |
|bMagicDamageReturn            |
|bMagicHPGainValue             |
|bMagicSPGainValue             |
|bMatk                         |
|bMatkRate                     |
|bMatkrate                     |
|bMaxHP                        |
|bMaxHPRate                    |
|bMaxHPrate                    |
|bMaxSP                        |
|bMaxSPRate                    |
|bMaxSPrate                    |
|bMdef                         |
|bNoCastCancel                 |
|bNoGemStone                   |
|bNoKnockback                  |
|bNoMagicDamage                |
|bNoRegen                      |
|bNoSizeFix                    |
|bNoWalkDelay                  |
|bPerfectHitRate               |
|bResEff                       |
|bRestartFullRecover           |
|bSPDrainRate                  |
|bSPDrainValue                 |
|bSPDrainValueRace             |
|bSPGainRace                   |
|bSPGainValue                  |
|bSPLossRate                   |
|bSPRegenRate                  |
|bSPVanishRate                 |
|bSPrecovRate                  |
|bShortWeaponDamageReturn      |
|bSkillAtk                     |
|bSkillHeal                    |
|bSpeedAddRate                 |
|bSpeedRate                    |
|bSplashRange                  |
|bStr                          |
|bSubClass                     |
|bSubEle                       |
|bSubRace                      |
|bSubRace2                     |
|bSubSize                      |
|bUnbreakableArmor             |
|bUnbreakableGarment           |
|bUnbreakableHelm              |
|bUnbreakableShield            |
|bUnbreakableShoes             |
|bUnbreakableWeapon            |
|bUseSPrate                    |
|bVit                          |
|bautospellonskill             |
|bdex                          |
+------------------------------+
