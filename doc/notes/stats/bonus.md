- bonus given by item/cards
- bonus given by passive skills
- bonus given by supportive skills

pre-re item scripts
```sql
with uniq_bonus as (
with bonus_only as (with bonus_with_value as (with cleaned_bonus as (with scripts
                                                                              as (select trim(both E'\n' from unnest(string_to_array(replace(replace(script, 'bonus_script', ''), 'autobonus', ''), ';'))) as arr
                                                                                  from item_db
                                                                                  where script is not null
                                                                                    and script like '%bonus%'
                                                                                    and script not like '/*%')
                                                                     select substring(arr, position('bonus' in arr)) as bonus
                                                                     from scripts s
                                                                     where arr like '%bonus%')
                                              select trim(substring(bonus, position('bonus' in bonus) + 6)) as bonus
                                              from cleaned_bonus)
                    select case when bonus like '%,%' then substring(bonus, 0, position(',' in bonus)) else bonus end as bonus
                    from bonus_with_value where bonus != '')
select distinct bonus as bonus from bonus_only)

select bonus, (select count(1) from item_db where script like '%' || bonus || '%') as occurence from uniq_bonus order by occurrence desc
;
```

+-----------------------------+---------+
|bonus                        |occurrence|
+-----------------------------+---------+
|bMdef                        |263      |
|bInt                         |237      |
|bAutoSpell                   |191      |
|bStr                         |188      |
|bDex                         |179      |
|bAgi                         |145      |
|bAtkEle                      |144      |
|bMatk                        |134      |
|bMaxHP                       |130      |
|bAddEff                      |130      |
|bAddRace                     |126      |
|bSubRace                     |125      |
|bMatkRate                    |125      |
|bMaxSP                       |113      |
|bLuk                         |110      |
|bSubEle                      |105      |
|bCritical                    |97       |
|bVit                         |96       |
|bUnbreakableWeapon           |94       |
|bFlee                        |92       |
|bDef                         |89       |
|bBaseAtk                     |79       |
|bAspd                        |79       |
|bResEff                      |79       |
|bAspdRate                    |78       |
|bAutoSpellWhenHit            |67       |
|bIgnoreDefRace               |64       |
|bAddMonsterDropItem          |60       |
|bHit                         |59       |
|bSkillAtk                    |58       |
|bAllStats                    |48       |
|bAddClass                    |46       |
|bIgnoreDefRaceRate           |42       |
|bCastrate                    |39       |
|bAddEle                      |38       |
|bMaxHPrate                   |33       |
|bFlee2                       |32       |
|bSPrecovRate                 |27       |
|bMaxSPrate                   |27       |
|bHPrecovRate                 |27       |
|bAddEffWhenHit               |26       |
|bExpAddRace                  |24       |
|bUseSPrate                   |22       |
|bCriticalAddRace             |20       |
|bAutoSpellOnSkill            |19       |
|bAddSize                     |18       |
|bDefEle                      |17       |
|bDelayRate                   |17       |
|bCritAtkRate                 |16       |
|bUnbreakableHelm             |16       |
|bHealPower                   |16       |
|bSPDrainValue                |15       |
|bSPGainRace                  |14       |
|bSpeedRate                   |14       |
|bMaxHPRate                   |13       |
|bMagicAddRace                |12       |
|bBreakArmorRate              |12       |
|bAddEff2                     |11       |
|bAddDamageClass              |10       |
|bHPLossRate                  |9        |
|bShortWeaponDamageReturn     |9        |
|bSplashRange                 |9        |
|bSubSize                     |9        |
|bIgnoreDefClass              |8        |
|bAddItemHealRate             |8        |
|bDoubleRate                  |7        |
|bAddEffOnSkill               |7        |
|bAddSkillBlow                |7        |
|bLongAtkDef                  |7        |
|bHPDrainRate                 |7        |
|bIgnoreMdefClassRate         |7        |
|bBreakWeaponRate             |7        |
|bUnbreakableArmor            |6        |
|bSPRegenRate                 |6        |
|bMaxSPRate                   |6        |
|bHPGainValue                 |6        |
|bSubClass                    |6        |
|bComaRace                    |6        |
|bAddRace2                    |6        |
|bAddItemGroupHealRate        |6        |
|bLongAtkRate                 |6        |
|bCastRate                    |5        |
|bAddDefMonster               |5        |
|bDef2Rate                    |5        |
|bDefRate                     |5        |
|bHPRegenRate                 |5        |
|bIgnoreMdefRaceRate          |5        |
|bNoCastCancel                |5        |
|bSPDrainRate                 |5        |
|bSPGainValue                 |5        |
|bAddMonsterDropItemGroup     |4        |
|bAtkRate                     |4        |
|bDefRatioAtkClass            |4        |
|bMagicDamageReturn           |4        |
|bNoKnockback                 |4        |
|bUnbreakableGarment          |3        |
|bSPDrainValueRace            |3        |
|bSPVanishRate                |3        |
|bNoGemStone                  |2        |
|bNoMagicDamage               |2        |
|bNoWalkDelay                 |2        |
|bPerfectHitRate              |2        |
|bMagicHPGainValue            |2        |
|bSPLossRate                  |2        |
|bMAtkRate                    |2        |
|bHealpower2                  |2        |
|bSkillHeal                   |2        |
|bGetZenyNum                  |2        |
|bCriticalLong                |2        |
|bComaClass                   |2        |
|bClassChange                 |2        |
|bAutoSpellwhenhit            |2        |
|bUnbreakableShield           |2        |
|bAddMonsterIdDropItem        |2        |
|bMatkrate                    |2        |
|bautospellonskill            |1        |
|bNoSizeFix                   |1        |
|bSubRace2                    |1        |
|bAGI                         |1        |
|buseSPRate                   |1        |
|bIntravision                 |1        |
|bHitRate                     |1        |
|bRestartFullRecover          |1        |
|bMagicSPGainValue            |1        |
|bSpeedAddRate                |1        |
|bNoRegen                     |1        |
|bExpAddClass                 |1        |
|bdex                         |1        |
|bUnbreakableShoes            |1        |
+-----------------------------+---------+

+-----------------------------------+
|bonus                              |
+-----------------------------------+
|bonus                              |
|bonus = max(getskilllv("TF_DOUBLE")|
|bonus bAGI                         |
|bonus bAddItemHealRate             |
|bonus bAgi                         |
|bonus bAllStats                    |
|bonus bAspd                        |
|bonus bAspdRate                    |
|bonus bAtkEle                      |
|bonus bAtkRate                     |
|bonus bBaseAtk                     |
|bonus bBreakArmorRate              |
|bonus bBreakWeaponRate             |
|bonus bCastRate                    |
|bonus bCastrate                    |
|bonus bClassChange                 |
|bonus bCritAtkRate                 |
|bonus bCritical                    |
|bonus bCriticalLong                |
|bonus bDef                         |
|bonus bDef2Rate                    |
|bonus bDefEle                      |
|bonus bDefRate                     |
|bonus bDefRatioAtkClass            |
|bonus bDelayRate                   |
|bonus bDex                         |
|bonus bDoubleRate                  |
|bonus bFlee                        |
|bonus bFlee2                       |
|bonus bHPDrainRate                 |
|bonus bHPGainValue                 |
|bonus bHPrecovRate                 |
|bonus bHealPower                   |
|bonus bHealpower2                  |
|bonus bHit                         |
|bonus bHitRate                     |
|bonus bIgnoreDefClass              |
|bonus bIgnoreDefRace               |
|bonus bInt                         |
|bonus bIntravision                 |
|bonus bLongAtkDef                  |
|bonus bLongAtkRate                 |
|bonus bLuk                         |
|bonus bMAtkRate                    |
|bonus bMagicDamageReturn           |
|bonus bMagicHPGainValue            |
|bonus bMagicSPGainValue            |
|bonus bMatk                        |
|bonus bMatkRate                    |
|bonus bMatkrate                    |
|bonus bMaxHP                       |
|bonus bMaxHPRate                   |
|bonus bMaxHPrate                   |
|bonus bMaxSP                       |
|bonus bMaxSPRate                   |
|bonus bMaxSPrate                   |
|bonus bMdef                        |
|bonus bNoCastCancel                |
|bonus bNoGemStone                  |
|bonus bNoKnockback                 |
|bonus bNoMagicDamage               |
|bonus bNoRegen                     |
|bonus bNoSizeFix                   |
|bonus bNoWalkDelay                 |
|bonus bPerfectHitRate              |
|bonus bRestartFullRecover          |
|bonus bSPDrainValue                |
|bonus bSPGainValue                 |
|bonus bSPrecovRate                 |
|bonus bShortWeaponDamageReturn     |
|bonus bSpeedAddRate                |
|bonus bSpeedRate                   |
|bonus bSplashRange                 |
|bonus bStr                         |
|bonus bUnbreakableArmor            |
|bonus bUnbreakableGarment          |
|bonus bUnbreakableHelm             |
|bonus bUnbreakableShield           |
|bonus bUnbreakableShoes            |
|bonus bUnbreakableWeapon           |
|bonus bUseSPrate                   |
|bonus bVit                         |
|bonus bdex                         |
|bonus buseSPRate                   |
|bonus2 bAddClass                   |
|bonus2 bAddDamageClass             |
|bonus2 bAddDefMonster              |
|bonus2 bAddEff                     |
|bonus2 bAddEff2                    |
|bonus2 bAddEffWhenHit              |
|bonus2 bAddEle                     |
|bonus2 bAddItemGroupHealRate       |
|bonus2 bAddItemHealRate            |
|bonus2 bAddMonsterDropItem         |
|bonus2 bAddMonsterDropItemGroup    |
|bonus2 bAddRace                    |
|bonus2 bAddRace2                   |
|bonus2 bAddSize                    |
|bonus2 bAddSkillBlow               |
|bonus2 bCastrate                   |
|bonus2 bComaClass                  |
|bonus2 bComaRace                   |
|bonus2 bCriticalAddRace            |
|bonus2 bExpAddClass                |
|bonus2 bExpAddRace                 |
|bonus2 bGetZenyNum                 |
|bonus2 bHPDrainRate                |
|bonus2 bHPLossRate                 |
|bonus2 bHPRegenRate                |
|bonus2 bIgnoreDefRaceRate          |
|bonus2 bIgnoreMdefClassRate        |
|bonus2 bIgnoreMdefRaceRate         |
|bonus2 bMagicAddRace               |
|bonus2 bResEff                     |
|bonus2 bSPDrainRate                |
|bonus2 bSPDrainValueRace           |
|bonus2 bSPGainRace                 |
|bonus2 bSPLossRate                 |
|bonus2 bSPRegenRate                |
|bonus2 bSPVanishRate               |
|bonus2 bSkillAtk                   |
|bonus2 bSkillHeal                  |
|bonus2 bSubClass                   |
|bonus2 bSubEle                     |
|bonus2 bSubRace                    |
|bonus2 bSubRace2                   |
|bonus2 bSubSize                    |
|bonus3 bAddEff                     |
|bonus3 bAddEffOnSkill              |
|bonus3 bAddEffWhenHit              |
|bonus3 bAddMonsterDropItem         |
|bonus3 bAddMonsterIdDropItem       |
|bonus3 bAutoSpell                  |
|bonus3 bAutoSpellWhenHit           |
|bonus3 bAutoSpellwhenhit           |
|bonus3 bSubEle                     |
|bonus4 bAutoSpell                  |
|bonus4 bAutoSpellOnSkill           |
|bonus4 bAutoSpellWhenHit           |
|bonus4 bautospellonskill           |
|bonus5 bAutoSpell                  |
|bonus5 bAutoSpellOnSkill           |
|bonus5 bAutoSpellWhenHit           |
|bonus5 bAutoSpellwhenhit           |
+-----------------------------------+


```sql
select item_db.name_aegis, item_db.script from item_db where script like '%(%';
```
