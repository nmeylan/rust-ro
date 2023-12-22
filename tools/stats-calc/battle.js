

function calc() {
    StAllCalc();


    sizeModifier = weaponsize[n_A_WeaponType][targetStatsArray[TARGET_STAT_SIZE]];
    if (SkillSearch("Cavalier Mastery")) {
        if ((n_A_WeaponType ==  WEAPON_TYPE_SPEAR || n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SPEAR) && targetStatsArray[TARGET_STAT_SIZE] == 1)
            sizeModifier = 1;
    }
    if (SkillSearch("Weapon Perfection") || n_A_PassSkill2[7])
        sizeModifier = 1;

    if (cardOBJ[n_A_card[0]][0] == 32 || cardOBJ[n_A_card[1]][0] == 32 || cardOBJ[n_A_card[2]][0] == 32 || cardOBJ[n_A_card[3]][0] == 32 || cardOBJ[n_A_card[4]][0] == 32 || cardOBJ[n_A_card[5]][0] == 32 || cardOBJ[n_A_card[6]][0] == 32 || cardOBJ[n_A_card[7]][0] == 32)
        sizeModifier = 1;


    impositioMagnus = n_A_PassSkill2[2] * 5;


    hitRate = n_A_HIT + 80 - (n_B_FLEE);
    if (SkillSearch("Weaponry Research"))
        hitRate = Math.floor(hitRate * (100 + 2 * SkillSearch("Weaponry Research")) / 100);
    if (skillToUseName == "Pierce" || skillToUseName == "Bash") {
        hitRate *= 1 + n_A_ActiveSkillLV * 0.05;
    }
    if ((skillToUseName == "Sonic Blow" || skillToUseName == "Sonic Blow (Soul Linked)") && SkillSearch("Sonic Acceleration")) {
        hitRate *= 1.5;
    }
    if (skillToUseName == "Magnum Break") {
        hitRate *= 1 + n_A_ActiveSkillLV * 0.1;
    }
    if (skillToUseName == "Sharp Shooting (Temp)") {
        hitRate *= (1 + n_A_ActiveSkillLV * 0.1);
    }
    if (skillToUseName == "Counter Kick") {
        hitRate = 100;
    }
    if (skillToUseName == "Shield Boomerang (SoulLinked)") {
        hitRate = 100;
    }
    if (SkillSearch("Solar, Lunar, and Stellar Union")) {
        hitRate = 100;
    }
    if (hitRate > 100) {
        hitRate = 100;
    } else if (hitRate < 5) {
        hitRate = 5;
    }
    if (StPlusItem(INCREASE_HIT_PERCENTAGE) + StPlusCard(INCREASE_HIT_PERCENTAGE))
        hitRate = hitRate + (100 - hitRate) * (StPlusItem(INCREASE_HIT_PERCENTAGE) + StPlusCard(INCREASE_HIT_PERCENTAGE)) / 100;

    hitRate = Math.floor(hitRate * 100) / 100;
    myInnerHtml("BattleHIT", hitRate, 0);

    if (skillToUseName == "Sharp Shooting (Temp)") {
        n_A_CRI += 20;
    }
    criticalRate = n_A_CRI - targetStatsArray[TARGET_STAT_LUK] * 0.2 - 0.1;
    if (TargetStatusFlags[8])
        criticalRate *= 2;
    if (criticalRate < 0) {
        criticalRate = 0;
    } else if (criticalRate > 100) {
        criticalRate = 100;
    }
    myInnerHtml("CRInum", Math.round(criticalRate * 10) / 10 + SubName[0], 0);


    tripleAttackChanceRate = 0;
    if (SkillSearch("Raging Trifecta Blow")) // Ragin trifecta blow
        tripleAttackChanceRate = 30 - SkillSearch("Raging Trifecta Blow");


    doubleAttackChanceRate = SkillSearch("Double Attack") * 5;
    if (n_A_WeaponType !=  WEAPON_TYPE_DAGGER)
        doubleAttackChanceRate = 0;
    if (cardOBJ[n_A_card[0]][0] == 43 || cardOBJ[n_A_card[1]][0] == 43 || cardOBJ[n_A_card[2]][0] == 43 || cardOBJ[n_A_card[3]][0] == 43 || cardOBJ[n_A_card[4]][0] == 43 || cardOBJ[n_A_card[5]][0] == 43 || cardOBJ[n_A_card[6]][0] == 43 || cardOBJ[n_A_card[7]][0] == 43) {
        if (SkillSearch("Double Attack") > 1)
            doubleAttackChanceRate = SkillSearch("Double Attack") * 5;
        else
            doubleAttackChanceRate = 5;
    }
    if (ItemOBJ[n_A_Equip[2]][0] == 570) {
        if (SkillSearch("Double Attack") > 1)
            doubleAttackChanceRate = SkillSearch("Double Attack") * 5;
        else
            doubleAttackChanceRate = 10;
    }
    if (ItemOBJ[n_A_Equip[0]][0] == 399 || ItemOBJ[n_A_Equip[1]][0] == 399)
        doubleAttackChanceRate = 25;
    if (n_A_WeaponType ==  WEAPON_TYPE_HANDGUN)
        doubleAttackChanceRate = SkillSearch("Single Action") * 5;

    hitRateDoubleAttack = hitRate;
    if (doubleAttackChanceRate != 0 && n_A_WeaponType !=  WEAPON_TYPE_HANDGUN) {
        hitRateDoubleAttack = hitRateDoubleAttack * (100 + SkillSearch("Double Attack")) / 100;
        if (hitRateDoubleAttack >= 100)
            hitRateDoubleAttack = 100;
    }

    w998A = 100 - tripleAttackChanceRate;
    w998B = tripleAttackChanceRate * hitRate / 100;
    w998C = tripleAttackChanceRate - w998B;
    w998D = w998A * doubleAttackChanceRate / 100;
    w998E = w998D * hitRateDoubleAttack / 100;
    w998F = w998D - w998E;
    w998G = (100 - tripleAttackChanceRate - w998D) * criticalRate / 100;
    w998H = 100 - tripleAttackChanceRate - w998D - w998G;
    w998I = w998H * hitRate / 100;
    w998J = w998H - w998I;
    w998K = w998B + w998E + w998G + w998I;
    w998L = 100 - w998K;


    w_FLEE = n_A_FLEE + 20 - (n_B_HIT);
    if (w_FLEE > 95) {
        w_FLEE = 95;
    } else if (w_FLEE < 5) {
        w_FLEE = 5;
    }
    if (InWarOfEmperium == 0)
        myInnerHtml("BattleFLEE", w_FLEE, 0);

    n_A_workDEX = Math.floor(n_A_DEX * (1 + (n_A_WeaponLV - 1) * 0.2));

    n_A_DMG = [0, 0, 0];
    weaponAttack = [0, 0, 0];


    if (n_A_workDEX >= n_A_Weapon_ATK || SkillSearch("Power Maximize")) // 155 = power maximize
        weaponAttack[2] = n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);
    else
        weaponAttack[2] = n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK - 1 + impositioMagnus) * sizeModifier);

    if (isRangedWeapon())
        weaponAttack[2] += Math.floor((ArrowOBJ[n_A_Arrow][0] - 1) * sizeModifier);


    if (isRangedWeapon()) {
        w1 = n_A_WeaponLV_overUpgradeBonusATK + Math.floor(n_A_Weapon_ATK * n_A_Weapon_ATK / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);
        w2 = n_A_WeaponLV_overUpgradeBonusATK + Math.floor(n_A_Weapon_ATK * n_A_workDEX / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);

        w = Math.floor((ArrowOBJ[n_A_Arrow][0] - 1) * sizeModifier);
        w1 += w;
        w2 += w;
        if (w1 > w2) w1 = w2;
        if (weaponAttack[2] < w1) weaponAttack[2] = w1;
    }


    if (isRangedWeapon()) {
        weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_Weapon_ATK / 100 + impositioMagnus) * sizeModifier);
        w = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_workDEX / 100 + impositioMagnus) * sizeModifier);
        if (weaponAttack[0] > w) weaponAttack[0] = w;
    } else {
        if (n_A_workDEX >= n_A_Weapon_ATK)
            weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);
        else {

            if (SkillSearch("Power Maximize"))
                n_A_workDEX = n_A_Weapon_ATK;
            weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_workDEX + impositioMagnus) * sizeModifier);
        }
    }


    weaponAttack[1] = (weaponAttack[0] + weaponAttack[2]) / 2;
    n_A_DMG[0] = baseATK + weaponAttack[0];
    n_A_DMG[1] = baseATK + weaponAttack[1];
    n_A_DMG[2] = baseATK + weaponAttack[2];
    myInnerHtml("BaseAttackCalc", baseATK, 0);
    myInnerHtml("MinWeaponAttackCalc", weaponAttack[0], 0);
    myInnerHtml("AvgWeaponAttackCalc", weaponAttack[1], 0);
    myInnerHtml("MaxWeaponAttackCalc", weaponAttack[2], 0);

    n_Enekyori = 0;
    n_A_CriATK = [0, 0, 0];
    n_A_CriATK[1] = baseATK + (n_A_WeaponLV_Minplus + n_A_WeaponLV_overUpgradeBonusATK) / 2 + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);
    n_A_CriATK[0] = baseATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);
    n_A_CriATK[2] = baseATK + n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);


    if (n_A_WeaponType ==  WEAPON_TYPE_BOW) {
        n_Enekyori = 1;
        n_A_CriATK[1] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
        n_A_CriATK[0] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
        n_A_CriATK[2] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
    }


    BK_n_A_DMG = [0, 0, 0];
    BK_n_A_DMG[2] = n_A_DMG[2];
    BK_n_A_DMG[0] = n_A_DMG[0];
    BK_n_A_DMG[1] = n_A_DMG[1];

    ApplyATKBonusPercentage();
    ApplySkillModifier(1, 1);

    wCriTyuu = 1;
    n_A_CriATK[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[1], 10);
    n_A_CriATK[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[0], 10);
    n_A_CriATK[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[2], 10);
    wCriTyuu = 0;


    n_A_EDP_DMG = [0, 0, 0];
    n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2], 2);
    n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0], 0);
    n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1], 1);

    if (n_A_WeaponType ==  WEAPON_TYPE_KATAR) {
        wk = Math.floor(n_A_CriATK[1] * (0.01 + SkillSearch("Double Attack") * 0.02));
        wk2 = Math.floor((n_A_CriATK[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch("Double Attack") * 0.02));
        if (n_A_WeaponLV_Minplus == n_A_WeaponLV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2]) {
            myInnerHtml("CRIATK", (n_A_CriATK[1] + wk2 + n_A_EDP_DMG[1]) + "(" + (n_A_CriATK[1] + n_A_EDP_DMG[1]) + "+" + wk2 + ")", 0);
        } else {
            w1 = Math.floor((n_A_CriATK[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch("Double Attack") * 0.02));
            w2 = Math.floor((n_A_CriATK[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch("Double Attack") * 0.02));
            myInnerHtml("CRIATK", (n_A_CriATK[0] + w1 + n_A_EDP_DMG[0]) + " ~ " + (n_A_CriATK[2] + w2 + n_A_EDP_DMG[2]) + "(" + (n_A_CriATK[0] + n_A_EDP_DMG[0]) + " ~ " + (n_A_CriATK[2] + n_A_EDP_DMG[2]) + "+" + w1 + " ~ " + w2 + ")", 0);
        }
        n_A_CriATK[1] += wk;
    } else {
        if (n_A_WeaponLV_Minplus == n_A_WeaponLV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2])
            myInnerHtml("CRIATK", n_A_CriATK[1] + n_A_EDP_DMG[1], 0);
        else
            myInnerHtml("CRIATK", (n_A_CriATK[0] + n_A_EDP_DMG[0]) + " ~ " + (n_A_CriATK[2] + n_A_EDP_DMG[2]), 0);
    }


    n_A_CriATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
    n_A_CriATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
    n_A_CriATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

    BattleCalc999();
}


function ApplyMasteryAndWeaponryResearchAndDMGLevel(w_atk, w_2) {

    
    if (w_2 == 10)
        w_atk += n_A_WeaponLV_upgradeBonusATK;
    else
        w_atk = BattleCalc4(w_atk, w_2, 0);

    if (w_atk < 1) w_atk = 1;


    if (n_A_WeaponType ==  WEAPON_TYPE_DAGGER || n_A_WeaponType ==  WEAPON_TYPE_SWORD) w_atk += 4 * SkillSearch(" Sword Mastery");
    else if (n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SWORD) w_atk += 4 * SkillSearch("Two-Hand Sword Mastery");
    else if (n_A_WeaponType ==  WEAPON_TYPE_SPEAR || n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SPEAR) {
        if (SkillSearch("Cavalier Mastery") == 0)
            w_atk += 4 * SkillSearch("Spear Mastery");
        else
            w_atk += 5 * SkillSearch("Spear Mastery");

    } else if (n_A_WeaponType ==  WEAPON_TYPE_KATAR) w_atk += 3 * SkillSearch("Katar Mastery");
    else if (n_A_WeaponType ==  WEAPON_TYPE_MACE) w_atk += 3 * SkillSearch("Mace Mastery");
    else if (n_A_WeaponType ==  WEAPON_TYPE_KNUCKLE || n_A_WeaponType ==  WEAPON_TYPE_UNARMED) w_atk += 3 * SkillSearch("Iron Fist");
    else if (n_A_WeaponType ==  WEAPON_TYPE_INSTRUMENT) w_atk += 3 * SkillSearch("Music Lessons");
    else if (n_A_WeaponType ==  WEAPON_TYPE_WHIP) w_atk += 3 * SkillSearch("Dance Lessons");
    else if (n_A_WeaponType ==  WEAPON_TYPE_BOOK) w_atk += 3 * SkillSearch("Study");
    else if (n_A_WeaponType ==  WEAPON_TYPE_AXE || n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_AXE) w_atk += 3 * SkillSearch("Axe Mastery");

    if (n_A_WeaponType ==  WEAPON_TYPE_UNARMED && SkillSearch("Sprint (Unarmed Mastery)"))
        w_atk += 10 * SkillSearch("Sprint (Unarmed Mastery)");

    if (n_A_PassSkill3[10] && n_A_WeaponLV == 4)
        w_atk += 50 + 25 * n_A_PassSkill3[10];


    if (targetStatsArray[TARGET_STAT_RACE] == 6 || (90 <= targetStatsArray[TARGET_STAT_ELEMENT] && targetStatsArray[TARGET_STAT_ELEMENT] <= 99)) {
        if (SkillSearch("Demon Bane"))
            w_atk += Math.floor((3 + 5 / 100 * n_A_BaseLV) * SkillSearch("Demon Bane"));
    }
    if (targetStatsArray[TARGET_STAT_RACE] == 2 || targetStatsArray[TARGET_STAT_RACE] == 4) {
        w_atk += 4 * SkillSearch("Beast Bane");
        if (SkillSearch("Hunter Spirit (Soul Link)"))
            w_atk += n_A_STR;
    }

    w_atk = ApplyWeaponryResearchAndDMGLevel(w_atk);

    return Math.floor(w_atk);
}


function ApplyWeaponryResearchAndDMGLevel(w999) {

    
    w999_AB = 0;
    if (w999 > 0)
        w999_AB = 1;


    w999 += 2 * SkillSearch("Weaponry Research");


    if (wBCEDPch == 0)
        w999 = w999 * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element];


    if (n_A_WeaponType ==  WEAPON_TYPE_UNARMED && SkillSearch("Sprint (Unarmed Mastery)"))
        if (skillToUseName == "Tornado Kick" || skillToUseName == "Heel Drop" || skillToUseName == "Roundouse" || skillToUseName == "Counter Kick")
            w999 += 10 * SkillSearch("Sprint (Unarmed Mastery)");


    if (n_A_JOB == 15 || n_A_JOB == 29)
        w999 += 3 * SkillSearch("Summon Spirit Sphere");
    else
        w999 += 3 * n_A_PassSkill2[10];

    w999 += 3 * SkillSearch("Coin Flip");


    if (n_A_WeaponType !=  WEAPON_TYPE_UNARMED && w999_AB == 1)
        w999 += 20 * SkillSearch("Aura Blade");


    if (wBCEDPch == 0) {
        if (skillToUseName == "Envenom" || skillToUseName == "")
            w999 += 15 * n_A_ActiveSkillLV;
        if (skillToUseName == "Poison React (Counter)" && (targetStatsArray[TARGET_STAT_ELEMENT] < 50 || 60 <= targetStatsArray[TARGET_STAT_ELEMENT]))
            w999 += 75;
    }
    if (skillToUseName == "Magical Bullet")
        w999 += Math.floor(n_A_MATK[b] * (100 - targetStatsArray[TARGET_STAT_MDEF]) / 100 - n_B_MDEF2);
    if (skillToUseName == "Gunslinger Mine")
        w999 += n_A_ActiveSkillLV * 50;


    if (cardOBJ[n_A_card[0]][0] == 106 && cardOBJ[n_A_card[1]][0] == 106 && cardOBJ[n_A_card[2]][0] == 106) {
        w999 += 40;
    } else {
        for (i = 0; i <= 2; i++) {
            if (cardOBJ[n_A_card[i]][0] == 106)
                w999 += 5;
        }
    }
    if (n_A_card[3] == 106)
        w999 += 10;


    if (skillToUseName == "Throw Dagger") {
        w999 += SyurikenOBJ[eval(document.calcForm.SkillSubNum.value)][0];
        w999 += 3 * SkillSearch("Dagger Throwing Practice");
        w999 += 4 * n_A_ActiveSkillLV;
    }

    if (skillToUseName == "Throw Kunai")
        w999 += KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][0] * 3;

    w999 = BaiCI(w999);


    if (skillToUseName == "Back Stab" && n_A_WeaponType ==  WEAPON_TYPE_BOW)
        w999 = Math.floor(w999 / 2);


    if (hasLeftHand && skillToUseName == "Basic Attack") {

        if (n_A_WeaponType !=  WEAPON_TYPE_UNARMED)
            w999 = Math.floor(w999 * (50 + SkillSearch("Righthand Mastery") * 10) / 100);
    }


    if (targetStatsArray[19] == 5)
        return 1;

    if (skillToUseName == "Magical Bullet")
        w999 = w999 * element[targetStatsArray[TARGET_STAT_ELEMENT]][8];
    if (skillToUseName == "Gunslinger Mine")
        w999 = w999 * element[targetStatsArray[TARGET_STAT_ELEMENT]][0];

    return w999;
}


function BaiCI(wBaiCI) {

    
    if (wBCEDPch == 0 && not_use_card == 0) {

        w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_RACE_PERCENTAGE + targetStatsArray[TARGET_STAT_RACE]);
        w1 += StPlusItem(INCREASE_DAMAGE_RACE_PERCENTAGE + targetStatsArray[TARGET_STAT_RACE]);
        if (targetStatsArray[TARGET_STAT_RACE] == 6) {
            if (ArrowOBJ[n_A_Arrow][2] == "Holy Arrow")
                w1 += 5;
        }
        if (targetStatsArray[TARGET_STAT_RACE] == 9 && SkillSearch("Dragonology"))
            w1 += SkillSearch("Dragonology") * 4;

        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_ELEMENT_PERCENTAGE + Math.floor(targetStatsArray[TARGET_STAT_ELEMENT] / 10));
        w1 += StPlusItem(INCREASE_DAMAGE_ELEMENT_PERCENTAGE + Math.floor(targetStatsArray[TARGET_STAT_ELEMENT] / 10));
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_AGAINST_SIZE_PERCENTAGE + targetStatsArray[TARGET_STAT_SIZE]);
        w1 += StPlusItem(INCREASE_DAMAGE_AGAINST_SIZE_PERCENTAGE + targetStatsArray[TARGET_STAT_SIZE]);
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        if (n_Enekyori == 1) {
            if (TyouEnkakuSousa3dan != -1) {
                w1 = 0;
                w1 += StPlusCard(REDUCE_DEFENSE_PERCENTAGE);
                w1 += StPlusItem(REDUCE_DEFENSE_PERCENTAGE);
                wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);
            }
        }


        w1 = 0;
        if (targetStatsArray[19] == 1) {
            w1 += StPlusCard(INCREASE_DAMAGE_AGAINST_BOSS_PERCENTAGE);
            w1 += StPlusItem(INCREASE_DAMAGE_AGAINST_BOSS_PERCENTAGE);
        }
        if (EquipNumSearch("The Sign"))
            w1 += EquipNumSearch("The Sign") * 5;
        if (CardNumSearch("Turtle General"))
            w1 += CardNumSearch("Turtle General") * 20;
        if (CardNumSearch("Valkyrie Randgris"))
            w1 += CardNumSearch("Valkyrie Randgris") * 10;
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        if (wCriTyuu == 1 && skillToUseName != "Sharp Shooting (Temp)")
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard(CRITICAL_DAMAGE_PERCENTAGE)) / 100);


        if (108 <= targetStatsArray[TARGET_STAT_MOB_INDEX] && targetStatsArray[TARGET_STAT_MOB_INDEX] <= 115 || targetStatsArray[TARGET_STAT_MOB_INDEX] == 319)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard(INCREASE_DAMAGE_GOBLIN_PERCENTAGE)) / 100);

        if (116 <= targetStatsArray[TARGET_STAT_MOB_INDEX] && targetStatsArray[TARGET_STAT_MOB_INDEX] <= 120)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard(INCREASE_DAMAGE_KOBOLD_PERCENTAGE)) / 100);

        if (49 <= targetStatsArray[TARGET_STAT_MOB_INDEX] && targetStatsArray[TARGET_STAT_MOB_INDEX] <= 52 || 55 == targetStatsArray[TARGET_STAT_MOB_INDEX] || 221 == targetStatsArray[TARGET_STAT_MOB_INDEX])
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard(INCREASE_DAMAGE_ORC_PERCENTAGE)) / 100);

        if (106 == targetStatsArray[TARGET_STAT_MOB_INDEX] || 152 == targetStatsArray[TARGET_STAT_MOB_INDEX] || 308 == targetStatsArray[TARGET_STAT_MOB_INDEX] || 32 == targetStatsArray[TARGET_STAT_MOB_INDEX])
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard(INCREASE_DAMAGE_GOLEM_PERCENTAGE)) / 100);


        wBaiCI = Math.floor(wBaiCI * (100 + StPlusItem(1000 + targetStatsArray[TARGET_STAT_MOB_INDEX]) + StPlusCard(1000 + targetStatsArray[TARGET_STAT_MOB_INDEX])) / 100);


        if (EquipNumSearch("Burning Bow") && n_A_Arrow == 2)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Freezing Bow") && n_A_Arrow == 5)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Earthen Bow") && n_A_Arrow == 4)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Gale Bow") && n_A_Arrow == 6)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Orc Archer's Bow") && n_A_Arrow == 10)
            wBaiCI = wBaiCI * 150 / 100;

        if (SkillSearch("Frenzy"))
            wBaiCI = wBaiCI * 2;
        if (SkillSearch("Enchant Deadly Poison"))
            wBaiCI = Math.floor(wBaiCI * (150 + 50 * SkillSearch("Enchant Deadly Poison")) / 100);
        if (skillToUseName == "Poison React (Counter)" && (50 <= targetStatsArray[TARGET_STAT_ELEMENT] && targetStatsArray[TARGET_STAT_ELEMENT] < 60))
            wBaiCI = Math.floor(wBaiCI * (100 + 30 * n_A_ActiveSkillLV) / 100);


        if (n_A_WeaponType ==  WEAPON_TYPE_KATAR && SkillSearch("Advanced Katar Mastery") && skillToUseName != "Soul Destroyer")
            wBaiCI = Math.floor(wBaiCI * (110 + 2 * SkillSearch("Advanced Katar Mastery")) / 100);

        w1 = 0;
        if (InWarOfEmperium == 0) {
            if (SkillSearch("Stellar Wrath") && SkillSearch("Solar, Lunar, and Stellar Miracle"))
                w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Stellar Wrath") * 3);
            else if (SkillSearch("Stellar Wrath") && targetStatsArray[TARGET_STAT_SIZE] == 2 && targetStatsArray[TARGET_STAT_HP] >= 20000)
                w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Stellar Wrath") * 3);
            else if (SkillSearch("Solar Wrath") && targetStatsArray[TARGET_STAT_SIZE] == 0)
                w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Solar Wrath") * 3);
            else if (SkillSearch("Lunar Wrath") && targetStatsArray[TARGET_STAT_SIZE] == 1 && targetStatsArray[TARGET_STAT_HP] >= 5000)
                w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Lunar Wrath") * 3);
        } else {
            if (SkillSearch("Stellar Wrath")) {
                w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Stellar Wrath") * 3);
            } else {
                if (SkillSearch("Solar Wrath")) {
                    w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Solar Wrath") * 3);
                } else {
                    if (SkillSearch("Lunar Wrath"))
                        w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch("Lunar Wrath") * 3);
                }
            }
        }
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);
    }

    wBaiCI = Math.floor(tPlusDamCut(wBaiCI));

    w1 = 0;

    if (skillToUseName == "Bash")
        if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch("Freezer"))
            w1 += 10;

    if (TyouEnkakuSousa3dan == -1 && EquipNumSearch("Barrage Fist"))
        w1 += 15;

    if ((skillToUseName == "Sonic Blow" || skillToUseName == "Sonic Blow (Soul Linked)") && SkillSearch("Sonic Acceleration"))
        w1 += 10;

    wBaiCI = wBaiCI * (100 + StPlusItem(5000 + n_A_ActiveSkill) + StPlusCard(5000 + n_A_ActiveSkill) + w1) / 100;

    return wBaiCI;
}


function BattleCalc3(w998) {
    wBC3_3dan = w998B * TyouEnkakuSousa3dan;
    wBC3_DA = w998E * w998 * 2;
    wBC3_Cri = w998G * n_A_CriATK[1];
    wBC3_Normal = w998I * w998;
    wBC3_Miss = w998L * ApplyWeaponryResearchAndDMGLevel(0);

    wBC3_X = (wBC3_3dan + wBC3_DA + wBC3_Cri + wBC3_Normal + wBC3_Miss) / 100;

    return tPlusLucky(wBC3_X);
}


function BattleCalc3left(w998) {

    wBC3L2 = 0;
    for (i = 4; i <= 7; i++) {
        if (cardOBJ[n_A_card[i]][0] == 106)
            wBC3L2 += 5;
    }

    wBC3_Normal = w998 * hitRate / 100;
    wBC3_Miss = wBC3L2 * (100 - hitRate) / 100;

    wBC3_X = wBC3_Normal + wBC3_Miss;

    wBC3_X = tPlusDamCut(wBC3_X);

    return tPlusLucky(wBC3_X);
}


function SkillSearch(n) {
    for (k = 0; k <= 14; k++) {
        let passiveSkillToUseName;
        if (JobSkillPassOBJ[n_A_JOB][k] != 999)
            passiveSkillToUseName = SkillOBJ[JobSkillPassOBJ[n_A_JOB][k]][2];
        if (passiveSkillToUseName === n) {
            return n_A_PassSkill[k];
        }
    }
    return 0;
}


function BattleCalc4(wBC4, wBC4_2, wBC4_3) {
    
    if (wBC4_3 == 0)
        wBC4_3 = n_A_WeaponLV_upgradeBonusATK;
    else
        wBC4_3 = n_A_Weapon2LV_upgradeBonusATK;
    if ((StPlusItem(WEAPON_ATK_INCREASE_ON_TARGET_DEFENSE) + StPlusCard(WEAPON_ATK_INCREASE_ON_TARGET_DEFENSE)) == 0 || skillToUseName == "Stave Crasher") {
        if (skillToUseName == "Wounding Shot")
            return wBC4 + wBC4_3;
        if (StPlusItem(BYPASS_DEFENSE_ON_RACE) == targetStatsArray[TARGET_STAT_RACE] && targetStatsArray[TARGET_STAT_RACE] != 0)
            return wBC4 + wBC4_3;
        if (StPlusItem(BYPASS_DEFENSE_ON_RACE) == 99 && targetStatsArray[19] == 0)
            return wBC4 + wBC4_3;
        if (SkillSearch("Solar, Lunar, and Stellar Union"))
            return wBC4 + wBC4_3;
        if (CardNumSearch("Samurai Specter") && targetStatsArray[19] == 0)
            return wBC4 + wBC4_3;
        wBC4 = Math.floor(wBC4 * (100 - targetStatsArray[TARGET_STAT_DEF]) / 100) - n_B_DEF2[wBC4_2] + wBC4_3;
    } else {
        if (wBC4_2 == 0) {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[2] + targetStatsArray[TARGET_STAT_DEF]) / 100) + wBC4_3;
        } else if (wBC4_2 == 1) {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[1] + targetStatsArray[TARGET_STAT_DEF]) / 100) + wBC4_3;
        } else {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[0] + targetStatsArray[TARGET_STAT_DEF]) / 100) + wBC4_3;
        }
    }
    return wBC4;
}


function BattleCalcEDP(wBCEDP, wBCEDP2) {
    
    if (wBCEDP <= 0)
        return 0;
    if (element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element] <= 0 && ApplyWeaponryResearchAndDMGLevel(0) == 0)
        return 0;

    if (skillToUseName == "Sand Attack" || skillToUseName == "Soul Destroyer" || skillToUseName == "Venom Splasher" || skillToUseName == "Meteor Assault" || skillToUseName == "Bomb")
        return 0;
    wBCEDPch = 1;
    wBCEDPx = 0;
    wBCEDPy = 0;
    if (SkillSearch("Enchant Deadly Poison")) {
        wBCEDPx = ApplyMasteryAndWeaponryResearchAndDMGLevel(wBCEDP, wBCEDP2);
        wBCEDPx = Math.floor((wBCEDPx * element[targetStatsArray[TARGET_STAT_ELEMENT]][5]) / 4);
    }
    if (n_A_PassSkill2[11]) {
        wBCEDPy = ApplyMasteryAndWeaponryResearchAndDMGLevel(wBCEDP, wBCEDP2);
        wBCEDPy = Math.floor((wBCEDPy * element[targetStatsArray[TARGET_STAT_ELEMENT]][3]) / 5);
    }
    wBCEDPch = 0;
    return wBCEDPx + wBCEDPy;
}

function BattleCalc999() {
    skillModifier = 1;
    wCast = 0;

    hitCount = 1;
    isRangedAttack = 0;

    finalDamages = [0, 0, 0];
    not_use_card = 0;
    cast_kotei = 0;


    myInnerHtml("bSUBname", "", 0);
    myInnerHtml("bSUB", "", 0);
    myInnerHtml("bSUB2name", "", 0);
    myInnerHtml("bSUB2", "", 0);


    
    if (skillToUseName != "Basic Attack" && skillToUseName != "Sharp Shooting (Temp)" && (skillToUseName != "Poison React (Counter)" || (targetStatsArray[TARGET_STAT_ELEMENT] < 50 && 60 <= targetStatsArray[TARGET_STAT_ELEMENT]))) {
        myInnerHtml("CRIATK", "", 0);
        myInnerHtml("CRInum", "", 0);
        myInnerHtml("CRIATKname", "", 0);
        myInnerHtml("CRInumname", "", 0);
    }

    if ((n_A_WeaponType ==  WEAPON_TYPE_BOW || n_A_WeaponType ==  WEAPON_TYPE_HANDGUN || n_A_WeaponType ==  WEAPON_TYPE_RIFLE || n_A_WeaponType ==  WEAPON_TYPE_SHOTGUN || n_A_WeaponType ==  WEAPON_TYPE_GATLING_GUN || n_A_WeaponType ==  WEAPON_TYPE_GRENADE_LAUNCHER) && skillToUseName === "Basic Attack")
        isRangedAttack = 1;


    if (skillToUseName === "Basic Attack" || (skillToUseName === "Poison React (Counter)" && (50 <= targetStatsArray[TARGET_STAT_ELEMENT] && targetStatsArray[TARGET_STAT_ELEMENT] < 60))) {
        myInnerHtml("CRIATKname", SubName[3], 0);
        myInnerHtml("CRInumname", SubName[4], 0);

        if (skillToUseName === "Poison React (Counter)") {
            n_SpSkill = 1;
            if (n_A_WeaponType !=  WEAPON_TYPE_KATAR)
                myInnerHtml("bSUB", '<Font size="2"><B>Damage Shown with 2x right hand damage.</B></Font>', 0);
        }

        if (hasLeftHand) {

            if (targetStatsArray[19] != 5) {
                TyouEnkakuSousa3dan = 0;

                n_A_Weapon2 = eval(document.calcForm.A_weapon2.value);
                n_A_Weapon2LV = ItemOBJ[n_A_Weapon2][4];
                n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
                n_A_Weapon2_RefinementLevel = eval(document.calcForm.A_Weapon2_ATKplus.value);


                n_A_Weapon2LV_upgradeBonusATK = 0;
                n_A_Weapon2LV_Minplus = 0;
                n_A_Weapon2LV_overUpgradeBonusATK = 0;
                if (n_A_Weapon2LV == 1) {
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 2;
                    if (n_A_Weapon2_RefinementLevel >= 8) {
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 3 * (n_A_Weapon2_RefinementLevel - 7);
                    }
                } else if (n_A_Weapon2LV == 2) {
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 3;
                    if (n_A_Weapon2_RefinementLevel >= 7) {
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 5 * (n_A_Weapon2_RefinementLevel - 6);
                    }
                } else if (n_A_Weapon2LV == 3) {
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 5;
                    if (n_A_Weapon2_RefinementLevel >= 6) {
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 8 * (n_A_Weapon2_RefinementLevel - 5);
                    }
                } else if (n_A_Weapon2LV == 4) {
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 7;
                    if (n_A_Weapon2_RefinementLevel >= 5) {
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 14 * (n_A_Weapon2_RefinementLevel - 4);
                    }
                }

                n_A_workDEX = Math.floor(n_A_DEX * (1 + (n_A_Weapon2LV - 1) * 0.2));

                if (n_A_workDEX >= n_A_Weapon2_ATK)
                    w_left_Maxatk = baseATK + n_A_Weapon2LV_overUpgradeBonusATK + Math.floor((n_A_Weapon2_ATK + impositioMagnus) * sizeModifier);
                else
                    w_left_Maxatk = baseATK + n_A_Weapon2LV_overUpgradeBonusATK + Math.floor((n_A_Weapon2_ATK - 1 + impositioMagnus) * sizeModifier);

                w_left_Maxatk = BattleCalc4(w_left_Maxatk * skillModifier, 2, 1);

                if (w_left_Maxatk < 1) w_left_Maxatk = 1;
                w_left_Maxatk = Math.floor(w_left_Maxatk * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon2_element]);


                w_left_star = 0;
                if (n_A_card[4] == 106 && n_A_card[5] == 106 && n_A_card[6] == 106) {
                    w_left_star += 40;
                } else {
                    for (i = 4; i <= 6; i++) {
                        if (cardOBJ[n_A_card[i]][0] == 106)
                            w_left_star += 5;
                    }
                }
                if (n_A_card[7] == 106)
                    w_left_star += 10;
                w_left_Maxatk += w_left_star;
                w_left_Maxatk = w_left_Maxatk * (3 + SkillSearch("Lefthand Mastery")) / 10;
                w_left_Maxatk = Math.floor(w_left_Maxatk);


                if (n_A_workDEX > n_A_Weapon2_ATK)
                    n_A_workDEX = n_A_Weapon2_ATK;
                w_left_Minatk = baseATK + n_A_Weapon2LV_Minplus + Math.floor((n_A_workDEX + impositioMagnus) * sizeModifier);
                w_left_Minatk = BattleCalc4(w_left_Minatk * skillModifier, 0, 1);

                if (w_left_Minatk < 1) w_left_Minatk = 1;
                w_left_Minatk = Math.floor(w_left_Minatk * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon2_element]);
                w_left_Minatk += w_left_star;
                w_left_Minatk *= (0.3 + SkillSearch("Lefthand Mastery") / 10);
                w_left_Minatk = Math.floor(w_left_Minatk);

                w_left_Aveatk = (w_left_Maxatk + w_left_Minatk) / 2;
            } else {
                w_left_Maxatk = 1;
                w_left_Minatk = 1;
                w_left_Aveatk = 1;
            }

            ApplySkillModifier(skillModifier, 0);


            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[2], 2);
            myInnerHtml("ATK_02", finalDamages[2] + n_A_EDP_DMG[2] + "(" + w_left_Maxatk + ")", 0);


            finalDamages[2] = BattleCalc3(finalDamages[2]);
            finalDamages[2] += BattleCalc3left(w_left_Maxatk);
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[0], 0);
            myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0] + "(" + w_left_Minatk + ")", 0);

            finalDamages[0] = BattleCalc3(finalDamages[0]);
            finalDamages[0] += BattleCalc3left(w_left_Minatk);
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[1], 1);
            myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1] + "(" + w_left_Aveatk + ")", 0);

            finalDamages[1] = BattleCalc3(finalDamages[1]);
            finalDamages[1] += BattleCalc3left(w_left_Aveatk);
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            BattleVariousResults(0, 0);
        } else if (n_A_WeaponType ==  WEAPON_TYPE_KATAR) {
            ApplySkillModifier(skillModifier, 0);
            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[2], 2);
            wk = Math.floor(finalDamages[2] * (0.01 + SkillSearch("Double Attack") * 0.02));
            wk2 = Math.floor((finalDamages[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch("Double Attack") * 0.02));
            myInnerHtml("ATK_02", (finalDamages[2] + wk2 + n_A_EDP_DMG[2]) + "(" + (finalDamages[2] + n_A_EDP_DMG[2]) + "+" + wk2 + ")", 0);
            finalDamages[2] += wk;


            finalDamages[2] = BattleCalc3(finalDamages[2]);
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[0], 0);
            wk = Math.floor(finalDamages[0] * (0.01 + SkillSearch("Double Attack") * 0.02));
            wk2 = Math.floor((finalDamages[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch("Double Attack") * 0.02));
            myInnerHtml("ATK_00", (finalDamages[0] + wk2 + n_A_EDP_DMG[0]) + "(" + (finalDamages[0] + n_A_EDP_DMG[0]) + "+" + wk2 + ")", 0);
            finalDamages[0] += wk;

            finalDamages[0] = BattleCalc3(finalDamages[0]);
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[1], 1);
            wk = Math.floor(finalDamages[1] * (0.01 + SkillSearch("Double Attack") * 0.02));
            wk2 = Math.floor((finalDamages[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch("Double Attack") * 0.02));
            myInnerHtml("ATK_01", (finalDamages[1] + wk2 + n_A_EDP_DMG[1]) + "(" + (finalDamages[1] + n_A_EDP_DMG[1]) + "+" + wk2 + ")", 0);
            finalDamages[1] += wk;

            finalDamages[1] = BattleCalc3(finalDamages[1]);
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            BattleVariousResults(0, 0);
        } else {
            wTAKA = BattleTAKA();
            TyouEnkakuSousa3dan = 0;

            if (SkillSearch("Raging Trifecta Blow")) {
                TyouEnkakuSousa3dan = -1;

                myInnerHtml("bSUBname", "Trifecta Damage", 0);
                san1 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[0] * (skillModifier + wBC3_3danAtkBairitu), 0) / 3) * 3;
                san2 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[1] * (skillModifier + wBC3_3danAtkBairitu), 1) / 3) * 3;
                san3 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[2] * (skillModifier + wBC3_3danAtkBairitu), 2) / 3) * 3;
                myInnerHtml("bSUB", san1 + " ~ " + san3, 0);
                myInnerHtml("bSUB2name", "Trifecta Rate", 0);
                myInnerHtml("bSUB2", 30 - SkillSearch("Raging Trifecta Blow") + "%", 0);
                TyouEnkakuSousa3dan = 0;
            }

            ApplySkillModifier(skillModifier, 0);

            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[2], 2);
            if (SkillSearch("Raging Trifecta Blow"))
                TyouEnkakuSousa3dan = san3;
            myInnerHtml("ATK_02", (finalDamages[2] + n_A_EDP_DMG[2]), 0);


            finalDamages[2] = BattleCalc3(finalDamages[2]);
            finalDamages[2] += wTAKA;
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[0], 0);
            myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0], 0);
            if (SkillSearch("Raging Trifecta Blow"))
                TyouEnkakuSousa3dan = san1;

            finalDamages[0] = BattleCalc3(finalDamages[0]);
            finalDamages[0] += wTAKA;
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[1], 1);
            myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1], 0);
            if (SkillSearch("Raging Trifecta Blow"))
                TyouEnkakuSousa3dan = san2;

            finalDamages[1] = BattleCalc3(finalDamages[1]);
            finalDamages[1] += wTAKA;
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            CastAndDelay();
            BattleVariousResults(wCast, wDelay);
        }
        return;
    } else if (skillToUseName === "Sharp Shooting (Temp)") {
        isRangedAttack = 1;
        myInnerHtml("CRIATKname", "Defence Bypassing Damage", 0);
        myInnerHtml("CRInumname", "Chance to Bypass Defence", 0);

        skillModifier += (1 + 0.5 * n_A_ActiveSkillLV);
        wCast = 2 * n_A_CAST;
        wDelay = 1.5;
        swDelay = 1;


        n_A_CriATK[1] = n_A_DMG[1];
        n_A_CriATK[0] = n_A_DMG[0];
        n_A_CriATK[2] = n_A_DMG[2];

        ApplySkillModifier(skillModifier, 1);

        wCriTyuu = 1;
        n_A_CriATK[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[1], 10);
        n_A_CriATK[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[0], 10);
        n_A_CriATK[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_CriATK[2], 10);
        wCriTyuu = 0;


        n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2], 0);
        n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0], 2);
        n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1], 3);

        myInnerHtml("CRIATK", (n_A_CriATK[0] + n_A_EDP_DMG[0]) + " ~ " + (n_A_CriATK[2] + n_A_EDP_DMG[2]), 0);


        n_A_CriATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
        n_A_CriATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
        n_A_CriATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

        ApplySkillModifier(skillModifier, 0);

        finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[2], 2);
        myInnerHtml("ATK_02", (finalDamages[2] + n_A_EDP_DMG[2]), 0);


        finalDamages[2] = BattleCalc3(finalDamages[2]);
        finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

        finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[0], 0);
        myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0], 0);

        finalDamages[0] = BattleCalc3(finalDamages[0]);
        finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

        finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[1], 1);
        myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1], 0);

        finalDamages[1] = BattleCalc3(finalDamages[1]);
        finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        return;
    }

    // 6 -> bash
    // 7 -> magnum break
    w_ActS = [6, 7, 19, 41, 44, 65, 71, 72, 73, 83, 84, 158, 161, 169, 171, 188, 189, 199, 207, 248, 260, 261, 264, 288, 289, 290, 292, 302, 303, 326, 331, 333, 335, 337, 339, 382, 388, 348, 349, 350, 419, 423, 428, 429, 430, 431, 432, 434, 435, 436, 437, "NULL"];
    for (iw = 0; w_ActS[iw] != n_A_ActiveSkill && w_ActS[iw] != "NULL"; iw++) ;
    if (n_A_ActiveSkill === w_ActS[iw]) {
        wActiveHitNum = 1;
        if (skillToUseName === "Bash")
            skillModifier += n_A_ActiveSkillLV * 0.3;
        else if (skillToUseName === "Solar Heat" || skillToUseName === "Lunar Heat" || skillToUseName === "Stellar Heat") {
            //Heat

            wDelay = 0.1;

        } else if (skillToUseName === "Magnum Break") {
            skillModifier += n_A_ActiveSkillLV * 0.2;
            n_A_Weapon_element = 3;
            wDelay = 2;
            swDelay = 1;
        } else if (skillToUseName === "Sand Attack") {
            not_use_card = 1;
            skillModifier += 0.3;
            n_A_Weapon_element = 2;
        } else if (skillToUseName === "Arrow Shower") {
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 0.05 - 0.25;
            wDelay = 1;
            swDelay = 1;
        } else if (skillToUseName === "Arrow Repel") {
            isRangedAttack = 1;
            wCast = 1.5;
            skillModifier += 0.5;
        } else if (skillToUseName === "Mammonite")
            skillModifier += n_A_ActiveSkillLV * 0.5;
        else if (skillToUseName === "Spear Stab") {
            skillModifier += n_A_ActiveSkillLV * 0.2;
            isRangedAttack = 1;
        } else if (skillToUseName === "Grimtooth") {
            if (n_A_ActiveSkillLV >= 3)
                isRangedAttack = 1;
            skillModifier += 0.2 * n_A_ActiveSkillLV;


        } else if (skillToUseName === "Smite") {
            skillModifier += n_A_ActiveSkillLV * 0.2;

        } else if (skillToUseName === "Holy Cross") {
            skillModifier += n_A_ActiveSkillLV * 0.35;
            n_A_Weapon_element = 6;
        } else if (skillToUseName === "Sightless Mind")
            skillModifier += n_A_ActiveSkillLV * 0.4;
        else if (skillToUseName === "Spear Boomerang") {
            skillModifier += n_A_ActiveSkillLV * 0.5;
            wDelay = 1;
            swDelay = 1;
            isRangedAttack = 1;
        } else if (skillToUseName === "Brandish Spear") {
            w = (1 + n_A_ActiveSkillLV * 0.2);
            if (n_A_ActiveSkillLV == 10) skillModifier += 4.625;
            else if (n_A_ActiveSkillLV >= 7) skillModifier += w + w / 2 + w / 4 - 1;
            else if (n_A_ActiveSkillLV >= 4) skillModifier += w + w / 2 - 1;
            else skillModifier += w - 1;
            wCast = 0.7;
        } else if (skillToUseName === "Sonic Blow" || skillToUseName === "Sonic Blow (Soul Linked)") {
            wActiveHitNum = 8;
            skillModifier += n_A_ActiveSkillLV * 0.5 + 2;
            if (skillToUseName === "Sonic Blow (Soul Linked)" && InWarOfEmperium == 0)
                skillModifier *= 2;
            if (skillToUseName === "Sonic Blow (Soul Linked)" && InWarOfEmperium == 1)
                skillModifier *= 1.25;
            wDelay = 2;
            swDelay = 2;
        } else if (skillToUseName === "Back Stab") {
            skillModifier += n_A_ActiveSkillLV * 0.4 + 2;
            wDelay = 0.5;
            swDelay = 1;
            w_HIT = 100;
            myInnerHtml("BattleHIT", 100, 0);
        } else if (skillToUseName === "Raging Quadruple Blow") {
            wActiveHitNum = 4;
            skillModifier += 0.5 + n_A_ActiveSkillLV * 0.5;
            n_SpSkill = 1;
        } else if (skillToUseName === "Raging Thrust") {
            skillModifier += 1.4 + n_A_ActiveSkillLV * 0.6;
            n_SpSkill = 1;
        } else if (skillToUseName === "Melody Strike" || skillToUseName === "Slinging Arrow") {
            wCast = 1.5;
            skillModifier += (n_A_ActiveSkillLV * 0.4 - 0.4);
            n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
            if (eval(document.calcForm.A_Weapon_element.value) != 0)
                n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
            isRangedAttack = 1;
        } else if (skillToUseName === "Bomb") {
            not_use_card = 1;
            n_A_Weapon_element = 3;
            n_SpSkill = 1;
            wCast = 1;
            skillModifier += n_A_ActiveSkillLV * 0.2;
            w_HIT = 100;
            myInnerHtml("BattleHIT", 100, 0);
        } else if (skillToUseName === "Traumatic Blow") {
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 0.4;
            wDelay = 0.5;
            swDelay = 1;
        } else if (skillToUseName === "Vital Strike") {
            isRangedAttack = 1;
            skillModifier += (n_A_ActiveSkillLV * 0.1 - 0.5);
            if (n_A_ActiveSkillLV > 5)
                wDelay = 1;
            else
                wDelay = 0.8;
            swDelay = 1;
        } else if (skillToUseName === "Meteor Assault") {
            not_use_card = 1;
            skillModifier += (n_A_ActiveSkillLV * 0.4 - 0.6);
            wCast = 0.5;
            wDelay = 0.5;
            swDelay = 1;
        } else if (skillToUseName === "Raging Palm Strike") {
            skillModifier += (1 + n_A_ActiveSkillLV);
            wDelay = 0.3;
            swDelay = 1;
        } else if (skillToUseName === "Glacier Fist") {
            n_SpSkill = 1;
            skillModifier += n_A_ActiveSkillLV - 0.6;


        } else if (skillToUseName === "Chain Crush Combo") {
            n_SpSkill = 1;
            skillModifier += (3 + n_A_ActiveSkillLV);
            if (n_A_ActiveSkillLV > 6) wDelay = 1;
            else wDelay = 0.8;
            swDelay = 1;
        } else if (skillToUseName === "Arrow Vulcan") {
            wActiveHitNum = 9;
            skillModifier += 1 + n_A_ActiveSkillLV;
            n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
            if (eval(document.calcForm.A_Weapon_element.value) != 0)
                n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
            isRangedAttack = 1;
            wCast = 1.8 + n_A_ActiveSkillLV * 0.2;
            if (n_A_ActiveSkillLV >= 6) wDelay = 1;
            else wDelay = 0.8;
            wDelay = 3;
            swDelay = 2;
        } else if (skillToUseName === "Tomahawk Throwing") {
            isRangedAttack = 1;
            not_use_card = 1;
            n_A_Weapon_element = 4;
        } else if (skillToUseName === "Pulse Strike (Temp)") {
            skillModifier += (n_A_ActiveSkillLV - 1) * 1;
        } else if (skillToUseName === "High Speed Cart Ram") {
            not_use_card = 1;
            skillModifier += Math.floor((eval(document.calcForm.SkillSubNum.value) / (16 - n_A_ActiveSkillLV) / 100 - 1) * 100) / 100;
        } else if (skillToUseName === "Excruciating Palm") {
            not_use_card = 1;
            skillModifier += 2;


        } else if (skillToUseName === "Tornado Kick" || skillToUseName === "Heel Drop") {
            n_SpSkill = 1;
            skillModifier += (0.6 + n_A_ActiveSkillLV * 0.2);
        } else if (skillToUseName === "Roundouse" || skillToUseName === "Counter Kick") {
            n_SpSkill = 1;
            skillModifier += (0.9 + n_A_ActiveSkillLV * 0.3);
            if (skillToUseName === "Counter Kick")
                wActiveHitNum = 3;
        } else if (skillToUseName === "Flying Kick (Normal)") {
            n_SpSkill = 1;
            skillModifier += (-0.7 + n_A_ActiveSkillLV * 0.1);
        } else if (skillToUseName === "Bull's Eye") {
            not_use_card = 1;
            wCast = 0.5;
            isRangedAttack = 1;
            wActiveHitNum = 5;
            if (targetStatsArray[TARGET_STAT_RACE] == 2 || targetStatsArray[TARGET_STAT_RACE] == 7)
                skillModifier += 4;
        } else if (skillToUseName === "Magical Bullet") {
            isRangedAttack = 1;
            n_A_Weapon_element = 8;
            not_use_card = 1;
        } else if (skillToUseName === "Trigger Happy Shot") {
            isRangedAttack = 1;
            wActiveHitNum = 5;
            skillModifier += n_A_ActiveSkillLV * 0.5 + 4;
            wDelay = 1.7;
            swDelay = 1;
        } else if (skillToUseName === "Desperado (Single Hit)") {
            isRangedAttack = 0;
            skillModifier += n_A_ActiveSkillLV * 0.5 - 0.5;
            wDelay = 1;
            swDelay = 1;
        } else if (skillToUseName === "Tracking") {
            wCast = 1 + 0.2 * n_A_ActiveSkillLV;
            cast_kotei = 1;
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 1 + 1;
            wDelay = 1;
            swDelay = 1;
            w_HIT = hitRate * 5 + 5;
            if (w_HIT > 100)
                w_HIT = 100;
        } else if (skillToUseName === "Disarm") {
            wCast = 2;
            isRangedAttack = 1;
        } else if (skillToUseName === "Wounding Shot") {
            wCast = 1.5;
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 0.2;
            wDelay = 0;
            swDelay = 1;
            w_HIT = 100;
        } else if (skillToUseName === "Crowd Control Shot") {
            cast_kotei = 1;
            wCast = 1;
            isRangedAttack = 0;
            skillModifier += n_A_ActiveSkillLV * 0.5;
            wDelay = 1;
            swDelay = 2
            w_HIT = 100;
        } else if (skillToUseName === "Full Blast") {
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 1 + 2;
            wDelay = 1 + n_A_ActiveSkillLV * 0.2;
            swDelay = 1;
        } else if (skillToUseName === "Spread Shot") {
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 0.2 - 0.2;
            wDelay = "(Unknown)";
            swDelay = 1;
        } else if (skillToUseName === "Gunslinger Mine") {
            isRangedAttack = 1;
            not_use_card = 1;
            wCast = 1;
            wDelay = 1;
            swDelay = 1;
            w_HIT = 100;
        }


        ApplySkillModifier(skillModifier, 0);


        n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0], 0);
        n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1], 1);
        n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2], 2);

        if (cast_kotei == 0)
            wCast = wCast * n_A_CAST;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            if (wActiveHitNum > 1)
                finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
            myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        if (cast_kotei == 0)
            CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Stave Crasher") {
        isRangedAttack = 1;
        wCast = 0.3;
        wDelay = 0.3;
        swDelay = 1;
        n_A_DMG[2] = n_A_MATK[2];
        n_A_DMG[0] = n_A_MATK[0];
        n_A_DMG[1] = (n_A_MATK[0] + n_A_MATK[2]) / 2;


        for (b = 0; b <= 2; b++)
            n_A_EDP_DMG[b] = BattleCalcEDP(n_A_DMG[b], b);

        wCast = wCast * n_A_CAST;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + (ApplyWeaponryResearchAndDMGLevel(0) + n_A_WeaponLV_upgradeBonusATK) * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    }

    else if (skillToUseName === "Double Strafe" || skillToUseName === "Pierce" || skillToUseName === "Throw Spirit Spheres (# Hits = # of Spirit Spheres)" || skillToUseName === "Bowling Bash" || skillToUseName === "Triple Action" || skillToUseName === "Beast Strafing") {
        var isBowlingBash = false;
        if (skillToUseName === "Double Strafe") { // double strafe
            isRangedAttack = 1;
            skillModifier += n_A_ActiveSkillLV * 0.1 - 0.1;
            hitCount = 2;
        } else if (skillToUseName === "Pierce") {
            skillModifier += n_A_ActiveSkillLV * 0.1;
            hitCount = targetStatsArray[TARGET_STAT_SIZE] + 1;
        } else if (skillToUseName === "Bowling Bash") {
            skillModifier += n_A_ActiveSkillLV * 0.4;
            wCast = 0.7 * n_A_CAST;
            hitCount = 2;
            if (n_A_ActiveSkillLV == 1)
                hitCount = 1;
            isBowlingBash = true;
            if (TargetStatusFlags[6] == 1) {
                hitCount = 3;
                if (n_A_ActiveSkillLV == 1)
                    hitCount = 2;
            }
        } else if (skillToUseName === "Throw Spirit Spheres (# Hits = # of Spirit Spheres)") {
            skillModifier += n_A_ActiveSkillLV * 0.5;
            if (n_A_JOB == 15 || n_A_JOB == 29)
                w = SkillSearch("Summon Spirit Sphere");
            else
                w = n_A_PassSkill2[10];
            if (w > n_A_ActiveSkillLV) {
                w = n_A_ActiveSkillLV;
            }
            hitCount = w;
            wCast = (1 + w) * n_A_CAST;
            wDelay = 0.5;
            swDelay = 1;
            isRangedAttack = 1;
        } else if (skillToUseName === "Triple Action") {
            isRangedAttack = 1;
            skillModifier += 0.5;
            hitCount = 3;
        } else if (skillToUseName === "Beast Strafing") {
            n_SpSkill = 1;
            isRangedAttack = 1;
            skillModifier += n_A_STR * 0.08 - 0.5;
            hitCount = 2;
        }


        ApplySkillModifier(skillModifier, 0);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] += n_A_EDP_DMG[b];
            if (skillToUseName === "Beast Strafing" && targetStatsArray[TARGET_STAT_RACE] != 2 && targetStatsArray[TARGET_STAT_RACE] != 4)
                finalDamages[b] = 0;
            if (TargetStatusFlags[6] == 0 || !isBowlingBash)
                myInnerHtml("ATK_0" + b, finalDamages[b] * hitCount + "(" + finalDamages[b] + SubName[8] + hitCount + "hit)", 0);
            else
                myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] * 2 + " + " + finalDamages[b] + ")", 0);
            finalDamages[b] -= n_A_EDP_DMG[b];
            finalDamages[b] *= hitCount;
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * hitCount * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]) * hitCount;
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Blitz Beat" || skillToUseName === "Falcon Eyes") {
        n_A_Weapon_element = 0;
        isRangedAttack = 1;
        wBT = 80 + Math.floor(n_A_DEX / 10) * 2 + Math.floor(n_A_INT / 2) * 2 + SkillSearch("Steel Crow") * 6;
        if (skillToUseName === "Falcon Eyes") {
            wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) / 100);
            wBT = Math.floor(wBT * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            wBT = tPlusDamCut(wBT);
            wBT *= 5;
            wCast = 1 * n_A_CAST;
            wDelay = 3;
        } else {
            wBT = Math.floor(wBT * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            wBT = tPlusDamCut(wBT);
            wBT *= n_A_ActiveSkillLV;
            wCast = 1.5 * n_A_CAST;
            wDelay = 1;
        }
        swDelay = 1;
        myInnerHtml("ATK_02", wBT, 0);
        myInnerHtml("ATK_00", wBT, 0);
        myInnerHtml("ATK_01", wBT, 0);
        finalDamages[0] = finalDamages[2] = finalDamages[1] = wBT;
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Envenom" || (skillToUseName === "Poison React (Counter)" && (targetStatsArray[TARGET_STAT_ELEMENT] < 50 || 60 <= targetStatsArray[TARGET_STAT_ELEMENT]))) {
        ApplySkillModifier(skillModifier, 0);
        n_A_Weapon_element = 5;


        n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2], 2);
        n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0], 0);
        n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1], 1);

        wINV = Math.floor(ApplyWeaponryResearchAndDMGLevel(0) * element[targetStatsArray[TARGET_STAT_ELEMENT]][5]);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][5]);
            myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + wINV * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        myInnerHtml("bSUBname", '<Font color="#0000FF">Poison Damage</Font>', 0);
        myInnerHtml("bSUB", '<Font color="#0000FF">' + wINV + "</Font>", 0);

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Shield Boomerang" || skillToUseName === "Shield Boomerang (SoulLinked)") {
        isRangedAttack = 1;
        n_A_Weapon_element = 0;
        wDelay = 0.7;
        if (skillToUseName === "Shield Boomerang (SoulLinked)")
            wDelay = 0.35;
        swDelay = 1;
        wSBr = n_A_LEFT_DEF_PLUS * 4;

        skillModifier2 = (1 + n_A_ActiveSkillLV * 0.3);
        if (skillToUseName === "Shield Boomerang (SoulLinked)")
            skillModifier2 *= 2;

        baseATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        baseATK = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = baseATK * skillModifier + ItemOBJ[n_A_Equip[5]][6] + wSBr;
            finalDamages[b] = Math.floor(Math.floor(finalDamages[b] * (100 - targetStatsArray[TARGET_STAT_DEF]) / 100 - n_B_DEF2[b]) * skillModifier2);
            finalDamages[b] = BaiCI(finalDamages[b]);
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate) / 100;
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Rapid Smiting") {
        isRangedAttack = 1;
        n_A_Weapon_element = 0;
        wCast = 1 * n_A_CAST;
        wDelay = 1;
        swDelay = 1;
        wSBr = n_A_LEFT_DEF_PLUS;
        wSC = ItemOBJ[n_A_Equip[5]][6];

        skillModifier2 = (1 + n_A_ActiveSkillLV * 0.3);

        baseATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        baseATK = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
        baseATK = baseATK * skillModifier + wSC + wSBr * 4;

        wSC -= 100;
        if (wSC < 0)
            wSC = 0;
        wSC2 = [0, 0, 0];
        wSC2[2] = 100 + wSC + (wSBr * 2) * wSBr;
        wSC2[1] = 100 + (wSC + (wSBr * 2) * wSBr) / 2;
        wSC2[0] = 100

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = (baseATK * (100 - targetStatsArray[TARGET_STAT_DEF]) / 100 - n_B_DEF2[b]) * skillModifier2;
            finalDamages[b] += wSC2[b];
            finalDamages[b] = BaiCI(finalDamages[b]);
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b] * 5 + "(" + finalDamages[b] + SubName[8] + "5hit)", 0);
            finalDamages[b] *= 5;
            finalDamages[b] = (finalDamages[b] * hitRate) / 100;
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Clashing Spiral") {
        isRangedAttack = 1;
        if (n_A_ActiveSkillLV == 5)
            wCast = 1 * n_A_CAST;
        else
            wCast = (0.1 + 0.2 * n_A_ActiveSkillLV) * n_A_CAST;
        wDelay = 1 + 0.2 * n_A_ActiveSkillLV;
        swDelay = 1;

        wSPP = Math.floor(n_A_STR / 10);
        finalDamages[2] = wSPP * wSPP + ItemOBJ[n_A_Equip[0]][6] * 0.8 * (1 + 0.5 * n_A_ActiveSkillLV);
        wSPP = 1.25 - (targetStatsArray[TARGET_STAT_SIZE] * 0.25);
        finalDamages[2] = Math.floor(finalDamages[2] * wSPP + n_A_WeaponLV_upgradeBonusATK);
        finalDamages[2] = finalDamages[2] * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element];
        finalDamages[2] = BaiCI(finalDamages[2]);
        myInnerHtml("ATK_00", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        myInnerHtml("ATK_01", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        myInnerHtml("ATK_02", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        finalDamages[2] *= 5;
        wSPP2 = n_A_WeaponLV_upgradeBonusATK * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element];
        wSPP2 = BaiCI(wSPP2);
        wSPP2 = tPlusDamCut(wSPP2);
        finalDamages[2] = finalDamages[2] * hitRate / 100 + wSPP2 * 5 * (100 - hitRate) / 100;


        finalDamages[0] = finalDamages[1] = finalDamages[2];

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Venom Splasher") {
        not_use_card = 1;
        n_SpSkill = 1;
        wCast = 1 * n_A_CAST;

        if (targetStatsArray[19] == 0) {

            skillModifier += (400 + 50 * n_A_ActiveSkillLV + 20 * eval(document.calcForm.SkillSubNum.value)) / 100;
            ApplySkillModifier(skillModifier, 0);


            for (b = 0; b <= 2; b++) {
                finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
                finalDamages[b] = Math.floor(finalDamages[b]);
                myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            }
        }
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Soul Destroyer") {
        not_use_card = 1;
        isRangedAttack = 1;
        wCast = 0.5 * n_A_CAST;
        wDelay = 0.8 + 0.2 * n_A_ActiveSkillLV;
        swDelay = 1;

        w_SBr = new Array();
        w = n_A_INT * 5 * n_A_ActiveSkillLV;
        w_SBr[2] = w + 1000 - Math.floor((targetStatsArray[TARGET_STAT_DEF] + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[2]) / 2);
        w_SBr[1] = w + 750 - Math.floor((targetStatsArray[TARGET_STAT_DEF] + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[1]) / 2);
        w_SBr[0] = w + 500 - Math.floor((targetStatsArray[TARGET_STAT_DEF] + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[0]) / 2);
        for (i = 0; i <= 2; i++)
            w_SBr[i] = tPlusDamCut(w_SBr[i]);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] *= n_A_ActiveSkillLV;
            myInnerHtml("ATK_0" + b, finalDamages[b] + w_SBr[b] + "(" + finalDamages[b] + "+" + w_SBr[b] + ")", 0);
            finalDamages[b] = ((finalDamages[b] + w_SBr[b]) * hitRate + (ApplyWeaponryResearchAndDMGLevel(0) + w_SBr[b]) * (100 - hitRate)) / 100;
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Grand Cross") {

        myInnerHtml("CRIATKname", '<Font color="#FF0000">HP Casting Cost</Font>', 0);
        myInnerHtml("CRIATK", '<Font color="#FF0000">' + Math.floor(n_A_MaxHP / 5) + "</Font>", 0);

        myInnerHtml("CRInumname", '<Font color="#FF0000">Reflect Damage</Font>', 0);


        wGXhito = 100 - StPlusCard(DAMAGE_INC_DEC_RACE_DEMIHUMAN_PERCENTAGE);
        wGXhito -= StPlusItem(DAMAGE_INC_DEC_RACE_DEMIHUMAN_PERCENTAGE);

        wGXsei = 100 - SkillSearch("Faith") * 5;
        wGXsei -= StPlusCard(DAMAGE_INC_DEC_ELEMENT_HOLY_PERCENTAGE);
        wGXsei -= StPlusItem(DAMAGE_INC_DEC_ELEMENT_HOLY_PERCENTAGE);

        wGXen = StPlusCard(RESISTANCE_RANGE_ATTACK_PERCENTAGE);
        wGXen += StPlusItem(RESISTANCE_RANGE_ATTACK_PERCENTAGE);


        work_A_VITDEF = [0, 0, 0];
        work_A_VITDEF[0] = n_A_VITDEF[2];
        work_A_VITDEF[1] = n_A_VITDEF[1];
        work_A_VITDEF[2] = n_A_VITDEF[0];
        n_A_INTMDEF = n_A_INT + Math.floor(n_A_VIT / 2);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BK_n_A_DMG[b] * (100 - n_A_DEF) / 100 - work_A_VITDEF[b] + n_A_WeaponLV_upgradeBonusATK;
            finalDamages[b] = Math.floor(finalDamages[b] * (skillModifier + n_A_ActiveSkillLV * 0.4));

            w = n_A_MATK[b] * (100 - n_A_MDEF) / 100 - n_A_INTMDEF;
            w = Math.floor(w * (n_A_ActiveSkillLV * 0.4 + 1));

            finalDamages[b] += w;
            finalDamages[b] = Math.floor(finalDamages[b] * wGXhito / 100);
            finalDamages[b] = Math.floor(finalDamages[b] * wGXsei / 100);
            finalDamages[b] = Math.floor(finalDamages[b] * (100 - wGXen) / 100);

            if (CardNumSearch("Bathory"))
                finalDamages[b] = Math.floor(finalDamages[b] * 125 / 100);

            if (CardNumSearch("Evil Druid"))
                finalDamages[b] = Math.floor(finalDamages[b] * 150 / 100);

            if (CardNumSearch("Angeling") || CardNumSearch("0"))
                finalDamages[b] = Math.floor(finalDamages[b] * 0 / 100);
            finalDamages[b] = Math.floor(finalDamages[b] / 2);
        }
        myInnerHtml("CRInum", '<Font color="#FF0000">' + finalDamages[0] + "�~3hit ~ " + finalDamages[1] + "�~3hit</Font>", 0);


        isRangedAttack = 2;
        n_A_Weapon_element = 6;
        wCast = 3 * n_A_CAST;
        wDelay = 1.5;
        swDelay = 1;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BK_n_A_DMG[b] * (100 - targetStatsArray[TARGET_STAT_DEF]) / 100 - n_B_DEF2[b] + n_A_WeaponLV_upgradeBonusATK;
            finalDamages[b] *= skillModifier + n_A_ActiveSkillLV * 0.4;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]);
            w = n_A_MATK[b] * (100 - targetStatsArray[TARGET_STAT_MDEF]) / 100 - n_B_MDEF2;
            w *= (n_A_ActiveSkillLV * 0.4 + 1);
            w = Math.floor(w * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]);
            finalDamages[b] = tPlusDamCut(Math.floor((w + finalDamages[b]) * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]));
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            if (60 <= targetStatsArray[TARGET_STAT_ELEMENT] && targetStatsArray[TARGET_STAT_ELEMENT] <= 69) finalDamages[b] = 0;
        }


        if (TargetStatusFlags[6] == 0) {
            for (b = 0; b <= 2; b++) {
                myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] + SubName[8] + "3hit)", 0);
                finalDamages[b] *= 3;
            }
        } else {
            for (b = 0; b <= 2; b++) {
                myInnerHtml("ATK_0" + b, finalDamages[b] * 4 + "(" + finalDamages[b] * 2 + " + " + finalDamages[b] + SubName[8] + "2hit)", 0);
                finalDamages[b] *= 4;
            }
        }
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);

        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Cart Revolution") {
        wCR = 100;

        if (SkillSearch("Maximum Power-Thust")) {
            wCR += 20 * SkillSearch("Maximum Power-Thust");
        } else {
            if (SkillSearch("Power-Thrust"))
                wCR += SkillSearch("Power-Thrust") * 5;
            if (SkillSearch("Power-Thrust") == 0 && n_A_PassSkill2[8])
                wCR += n_A_PassSkill2[8] * 5 / 10;
        }
        CR_n_A_DMG = [0, 0, 0];

        CRbai = eval(document.calcForm.SkillSubNum.value) / 8000;
        for (b = 0; b <= 2; b++)
            CR_n_A_DMG[b] = Math.floor(n_A_DMG[b] * wCR / 100);

        skillModifier += 0.5;
        ApplySkillModifier(skillModifier, 0);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] += Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(CR_n_A_DMG[b], b) * CRbai);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);

            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * 2 * (100 - hitRate)) / 100;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
        }

        CastAndDelay();
        BattleVariousResults(0, 0);
    } else if (skillToUseName === "Gloria Domini") {
        finalDamages[2] = 500 + 300 * n_A_ActiveSkillLV;
        if (n_Ses)
            finalDamages[2] = Math.floor(finalDamages[2] * 0.6);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        myInnerHtml("ATK_02", finalDamages[2], 0);
        myInnerHtml("ATK_00", finalDamages[0], 0);
        myInnerHtml("ATK_01", finalDamages[1], 0);

        wCast = (1.5 + 0.5 * n_A_ActiveSkillLV) * n_A_CAST;
        wDelay = 1.5 + n_A_ActiveSkillLV * 0.5;
        swDelay = 1;
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Martyr's Reconing") {
        n_A_Weapon_element = 0;
        finalDamages[2] = Math.floor(n_A_MaxHP * 0.09 * (0.9 + 0.1 * n_A_ActiveSkillLV));
        finalDamages[2] = BaiCI(finalDamages[2]);
        finalDamages[2] = Math.floor(finalDamages[2] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
        myInnerHtml("ATK_02", finalDamages[2], 0);
        myInnerHtml("ATK_00", finalDamages[2], 0);
        myInnerHtml("ATK_01", finalDamages[2], 0);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);

        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Occult Impaction") {
        n_A_Weapon_element = 0;
        ApplySkillModifier(skillModifier, 0);
        skillModifier += n_A_ActiveSkillLV * 0.75;


        work_B_DEF2 = [0, 0, 0];
        work_B_DEF2[0] = n_B_DEF2[2];
        work_B_DEF2[1] = n_B_DEF2[1];
        work_B_DEF2[2] = n_B_DEF2[0];

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor(Math.floor(BK_n_A_DMG[b] * skillModifier) * (work_B_DEF2[b] + targetStatsArray[TARGET_STAT_DEF]) / 50);
            finalDamages[b] = BaiCI(finalDamages[b]);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);
        }

        wCast = 1 * n_A_CAST;
        wDelay = 0.5;
        swDelay = 1;
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Guillotine Fist" || skillToUseName === "Guillotine Fist (MaxSP-1)") {
        n_A_Weapon_element = 0;
        ApplySkillModifier(skillModifier, 0);
        if (skillToUseName === "Guillotine Fist")
            skillModifier += 7 + eval(document.calcForm.SkillSubNum.value) / 10;
        else
            skillModifier += 7 + (n_A_MaxSP - 1) / 10;
        wASYU = 250 + n_A_ActiveSkillLV * 150;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor(BK_n_A_DMG[b] * skillModifier) + wASYU;
            finalDamages[b] = BaiCI(finalDamages[b]);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);
        }

        wCast = (4.5 - 0.5 * n_A_ActiveSkillLV) * n_A_CAST;
        wDelay = 3.5 - 0.5 * n_A_ActiveSkillLV;
        swDelay = 1;
        CastAndDelay();

        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Throw Dagger") {
        isRangedAttack = 1;
        not_use_card = 1;
        ApplySkillModifier(skillModifier, 0);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * element[targetStatsArray[TARGET_STAT_ELEMENT]][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Throw Kunai") {
        isRangedAttack = 1;
        not_use_card = 1;
        ApplySkillModifier(skillModifier, 0);


        if (eval(document.calcForm.A_Weapon_element.value) == 0 && n_A_WeaponType !=  WEAPON_TYPE_UNARMED && StPlusCard(20) == 0)
            n_A_Weapon_element = KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][1];

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] + SubName[8] + "3hit)", 0);
            finalDamages[b] = (finalDamages[b] * 3 * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * element[targetStatsArray[TARGET_STAT_ELEMENT]][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Throw Huuma Shuriken") {
        skillModifier += (n_A_ActiveSkillLV * 1.5 + 0.5);
        isRangedAttack = 1;
        ApplySkillModifier(skillModifier, 0);
        wCast = 3 * n_A_CAST;
        wDelay = 3;
        swDelay = 1;
        wActiveHitNum = 2 + Math.round(n_A_ActiveSkillLV / 2);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(n_A_DMG[b], b);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
            if (wActiveHitNum > 1)
                finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
            myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(0) * element[targetStatsArray[TARGET_STAT_ELEMENT]][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }

        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
    } else if (skillToUseName === "Final Strike" || skillToUseName === "Final Strike (MaxHP-1)") {
        n_A_Weapon_element = 0;
        isRangedAttack = 1;
        ApplySkillModifier(skillModifier, 0);
        if (skillToUseName === "Final Strike")
            w_1senHP = eval(document.calcForm.SkillSubNum.value);
        else
            w_1senHP = n_A_MaxHP - 1;

        finalDamages[0] = (n_A_STR + n_A_ActiveSkillLV) * 40 + w_1senHP * (n_A_BaseLV / 100) * n_A_ActiveSkillLV / 10;
        finalDamages[0] = finalDamages[0] * (100 - targetStatsArray[TARGET_STAT_DEF]) / 100;
        finalDamages[0] = BaiCI(finalDamages[0]);
        finalDamages[0] = Math.floor(finalDamages[0] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);

        finalDamages[2] = finalDamages[1] = finalDamages[0];
        for (b = 0; b <= 2; b++)
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);

        CastAndDelay();

        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Acid Terror") {
        isRangedAttack = 1;
        n_A_Weapon_element = 0;
        skillModifier = (50 + n_A_ActiveSkillLV * 50) / 100;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor((BK_n_A_DMG[b] - n_B_DEF2[b]) * skillModifier);
            finalDamages[b] = tPlusDamCut(Math.floor(finalDamages[b] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]));
            myInnerHtml("ATK_0" + b, finalDamages[b], 0);
        }

        wCast = 1 * n_A_CAST;
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Acid Demonstration") {
        isRangedAttack = 1;
        n_A_Weapon_element = 0;
        hitCount = n_A_ActiveSkillLV;

        wAD = 0.7 * n_A_INT * n_A_INT * targetStatsArray[TARGET_STAT_VIT] / (n_A_INT + targetStatsArray[TARGET_STAT_VIT]);
        finalDamages[2] = Math.floor(wAD);
        finalDamages[2] = tPlusDamCut(Math.floor(finalDamages[2] * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]));
        if (InWarOfEmperium == 1)
            finalDamages[2] = Math.floor(finalDamages[2] / 2);
        myInnerHtml("ATK_02", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        myInnerHtml("ATK_00", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        myInnerHtml("ATK_01", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        finalDamages[2] *= hitCount;
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        wCast = 1 * n_A_CAST;
        wDelay = 1;
        swDelay = 1;
        CastAndDelay();
        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Land Mine" || skillToUseName === "Blast Mine" || skillToUseName === "Claymore Trap") {
        n_SpSkill = 1;
        if (skillToUseName === "Land Mine") {
            n_A_Weapon_element = 2;
            finalDamages[2] = Math.floor((75 + n_A_DEX) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * element[targetStatsArray[TARGET_STAT_ELEMENT]][2]);
        } else if (skillToUseName === "Blast Mine") {
            n_A_Weapon_element = 4;
            finalDamages[2] = Math.floor((50 + n_A_DEX / 2) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * element[targetStatsArray[TARGET_STAT_ELEMENT]][4]) * eval(document.calcForm.SkillSubNum.value);
        } else if (skillToUseName === "Claymore Trap") {
            n_A_Weapon_element = 3;
            finalDamages[2] = Math.floor((75 + n_A_DEX / 2) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * element[targetStatsArray[TARGET_STAT_ELEMENT]][3]) * eval(document.calcForm.SkillSubNum.value);
        }

        finalDamages[2] = tPlusDamCut(finalDamages[2]);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        myInnerHtml("ATK_02", finalDamages[2], 0);
        myInnerHtml("ATK_00", finalDamages[0], 0);
        myInnerHtml("ATK_01", finalDamages[1], 0);

        CastAndDelay();

        BattleVariousResults(0, 0);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Heal") {
        n_A_Weapon_element = 6;
        wDelay = 1;
        swDelay = 1;
        isRangedAttack = 2;
        finalDamages[2] = HealCalc(n_A_ActiveSkillLV, 0);
        finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]);
        if (targetStatsArray[TARGET_STAT_ELEMENT] < 90) {
            finalDamages[2] = 0;
        }

        finalDamages[2] = tPlusDamCut(finalDamages[2]);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        myInnerHtml("ATK_02", finalDamages[2], 0);
        myInnerHtml("ATK_00", finalDamages[0], 0);
        myInnerHtml("ATK_01", finalDamages[1], 0);

        CastAndDelay();

        BattleVariousResults(0, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Sanctuary") {
        n_A_Weapon_element = 6;
        n_SpSkill = 1;
        isRangedAttack = 2;
        if (n_A_ActiveSkillLV <= 6)
            finalDamages[2] = 100 * n_A_ActiveSkillLV;
        else
            finalDamages[2] = 777;
        finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]);
        if (targetStatsArray[TARGET_STAT_ELEMENT] < 90 && targetStatsArray[TARGET_STAT_RACE] != 6)
            finalDamages[2] = 0;
        if (targetStatsArray[TARGET_STAT_RACE] != 6 && targetStatsArray[TARGET_STAT_RACE] != 1)
            finalDamages[2] = 0;

        w_HEAL_BAI = 100;
        if (EquipNumSearch("Staff of Recovery"))
            w_HEAL_BAI += Math.floor(weaponRefinementLevel * 1.5)
        if (CardNumSearch("White Lady"))
            w_HEAL_BAI += 30 * CardNumSearch("White Lady");
        finalDamages[2] = Math.floor(finalDamages[2] * w_HEAL_BAI / 100);

        finalDamages[2] = tPlusDamCut(finalDamages[2]);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        myInnerHtml("ATK_02", finalDamages[2], 0);
        myInnerHtml("ATK_00", finalDamages[0], 0);
        myInnerHtml("ATK_01", finalDamages[1], 0);

        CastAndDelay();

        BattleVariousResults(0, 0);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Turn Undead") {
        n_A_Weapon_element = 6;
        isRangedAttack = 2;
        if (targetStatsArray[TARGET_STAT_ELEMENT] < 90) {
            w = 0;
            finalDamages[2] = 0;
            finalDamages[0] = 0;
            finalDamages[1] = 0;
        } else {
            if (targetStatsArray[19] != 1) {
                w = (20 * n_A_ActiveSkillLV + n_A_BaseLV + n_A_INT + n_A_LUK) / 1000;
                finalDamages[2] = targetStatsArray[TARGET_STAT_HP];
            } else {
                w = 0;
                finalDamages[2] = 0;
            }
            finalDamages[0] = n_A_BaseLV + n_A_INT + n_A_ActiveSkillLV * 10;
            finalDamages[0] = Math.floor(finalDamages[0] * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]);
            finalDamages[1] = Math.round((targetStatsArray[TARGET_STAT_HP] * w + finalDamages[0] * (100 - w) / 100));
        }
        myInnerHtml("ATK_02", Math.floor(finalDamages[2] * element[targetStatsArray[TARGET_STAT_ELEMENT]][6]) + "(Success Rate " + Math.round(w * 10000) / 100 + "%)", 0);
        myInnerHtml("ATK_00", finalDamages[0] + "(Failure Damage)", 0);
        myInnerHtml("ATK_01", finalDamages[1] + "(Certain One Hit Kill HP)", 0);

        wCast = 1 * n_A_CAST;
        wDelay = 3;
        swDelay = 1;
        CastAndDelay();

        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else if (skillToUseName === "Gravity Field") {
        n_A_Weapon_element = 0;
        n_SpSkill = 1;
        isRangedAttack = 2;
        hitCount = 4 + n_A_ActiveSkillLV;
        finalDamages[2] = 200 + 200 * n_A_ActiveSkillLV;

        finalDamages[2] = Math.floor(finalDamages[2]);

        wStrG = finalDamages[2] * hitCount + "(" + finalDamages[2] + " x " + hitCount + "hit)"
        myInnerHtml("ATK_02", wStrG, 0);
        myInnerHtml("ATK_00", wStrG, 0);
        myInnerHtml("ATK_01", wStrG, 0);

        finalDamages[2] = finalDamages[2] * hitCount;
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        wCast = 5 * n_A_CAST;
        swDelay = 1;
        wDelay = 2;
        CastAndDelay();

        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    } else {
        isRangedAttack = 2;
        swDelay = 1;
        skillModifier = 1;
        if (skillToUseName === "Fire Bolt") {
            n_A_Weapon_element = 3;
            hitCount = n_A_ActiveSkillLV;
            wCast = 0.7 * n_A_ActiveSkillLV;
            wDelay = 0.8 + n_A_ActiveSkillLV * 0.2;
        } else if (skillToUseName === "Cold Bolt") {
            n_A_Weapon_element = 1;
            hitCount = n_A_ActiveSkillLV;
            wCast = 0.7 * n_A_ActiveSkillLV;
            wDelay = 0.8 + n_A_ActiveSkillLV * 0.2;
        } else if (skillToUseName === "Lightning Bolt") {
            n_A_Weapon_element = 4;
            hitCount = n_A_ActiveSkillLV;
            wCast = 0.7 * n_A_ActiveSkillLV;
            wDelay = 0.8 + n_A_ActiveSkillLV * 0.2;
        } else if (skillToUseName === "Fire Ball") {
            n_A_Weapon_element = 3;
            if (n_A_ActiveSkillLV <= 5) {
                wCast = 1.5;
                wDelay = 1.5;
            } else {
                wCast = 1;
                wDelay = 1;
            }
            skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
        } else if (skillToUseName === "Fire Wall") {
            n_A_Weapon_element = 3;
            hitCount = 4 + n_A_ActiveSkillLV;
            wCast = 2.15 - (n_A_ActiveSkillLV * 0.15);
            wDelay = 0.1;
            skillModifier = 0.5;
        } else if (skillToUseName === "Frost Diver") {
            n_A_Weapon_element = 1;
            wCast = 0.8;
            wDelay = 1.5;
            skillModifier = 1 + n_A_ActiveSkillLV * 0.1;
        } else if (skillToUseName === "Thunder Storm") {
            n_A_Weapon_element = 4;
            hitCount = n_A_ActiveSkillLV;
            wCast = 1 * n_A_ActiveSkillLV;
            wDelay = 2;
            skillModifier = 0.8;
        } else if (skillToUseName === "Napalm Beat") {
            n_A_Weapon_element = 8;
            wCast = 0.5;
            if (n_A_ActiveSkillLV == 10)
                wDelay = 0.5;
            else if (n_A_ActiveSkillLV == 9)
                wDelay = 0.6;
            else if (n_A_ActiveSkillLV == 8)
                wDelay = 0.7;
            else if (n_A_ActiveSkillLV >= 6)
                wDelay = 0.8;
            else if (n_A_ActiveSkillLV >= 4)
                wDelay = 0.9;
            else
                wDelay = 1;
            skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
        } else if (skillToUseName === "Soul Strike") {
            n_A_Weapon_element = 8;
            hitCount = Math.round(n_A_ActiveSkillLV / 2);
            wCast = 0.5;
            if (n_A_ActiveSkillLV % 2 == 0)
                wDelay = 0.8 + n_A_ActiveSkillLV / 2 * 0.2;
            else
                wDelay = 1 + (n_A_ActiveSkillLV + 1) / 2 * 0.2;
        } else if (skillToUseName === "Fire Pillar") {
            n_A_Weapon_element = 3;
            hitCount = n_A_ActiveSkillLV + 2;
            wCast = 3.3 - (0.3 * n_A_ActiveSkillLV);
            wDelay = 1;
            skillModifier = 0.2;
        } else if (skillToUseName === "Sightrasher") {
            n_A_Weapon_element = 3;
            wCast = 0.7;
            wDelay = 2;
            skillModifier = 1 + n_A_ActiveSkillLV * 0.2;
        } else if (skillToUseName === "Meteor Storm") {
            n_A_Weapon_element = 3;
            hitCount = Math.round(n_A_ActiveSkillLV / 2) * (Math.floor(n_A_ActiveSkillLV / 2) + 2);
            wCast = 15;
            wDelay = Math.floor(n_A_ActiveSkillLV / 2) * 1 + 2;
        } else if (skillToUseName === "Jupitel Thunder") {
            n_A_Weapon_element = 4;
            hitCount = n_A_ActiveSkillLV + 2;
            wCast = 2 + n_A_ActiveSkillLV * 0.5;
            wDelay = 0.01;
        } else if (skillToUseName === "Lord of Vermillion") {
            n_A_Weapon_element = 4;
            hitCount = 4;
            wCast = 15.5 - n_A_ActiveSkillLV * 0.5;
            wDelay = 5;
            skillModifier = 0.8 + n_A_ActiveSkillLV * 0.2;
        } else if (skillToUseName === "Water Ball" || skillToUseName === "Water Ball") {
            swDelay = 2;
            n_A_Weapon_element = 1;
            if (n_A_ActiveSkillLV >= 4)
                hitCount = 25
            else if (n_A_ActiveSkillLV >= 2)
                hitCount = 9;
            wCast = n_A_ActiveSkillLV;
            skillModifier = 1 + n_A_ActiveSkillLV * 0.3;
            wDelay = 0.1 * hitCount;
        } else if (skillToUseName === "Frost Nova") {
            skillModifier = 0.66 + n_A_ActiveSkillLV * 0.066;
            n_A_Weapon_element = 1;
            wCast = 6 - Math.floor((n_A_ActiveSkillLV - 1) / 2) * 0.5;
            wDelay = 1;
        } else if (skillToUseName === "Storm Gust") {
            n_A_Weapon_element = 1;
            hitCount = eval(document.calcForm.SkillSubNum.value);
            wCast = 5 + n_A_ActiveSkillLV;
            wDelay = 5;
            skillModifier = 1 + n_A_ActiveSkillLV * 0.4;
        } else if (skillToUseName === "Earth Spike" || skillToUseName === "Heaven's Drive") {
            n_A_Weapon_element = 2;
            hitCount = n_A_ActiveSkillLV;
            if (skillToUseName === "Earth Spike") {
                wCast = n_A_ActiveSkillLV * 0.7;
                wDelay = 0.8 + n_A_ActiveSkillLV * 0.2;
            } else {
                wCast = n_A_ActiveSkillLV;
                wDelay = 1;
            }
        } else if (skillToUseName === "Napalm Vulcan") {
            hitCount = n_A_ActiveSkillLV;
            n_A_Weapon_element = 8;
            wCast = 1;
            wDelay = 1;
            skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
        } else if (skillToUseName === "Holy Light" || skillToUseName === "Holy Light (Soul Linked)") {
            n_A_Weapon_element = 6;
            wCast = 2;
            skillModifier = 1.25;
            if (skillToUseName === "Holy Light (Soul Linked)")
                skillModifier *= 5;
            wDelay = 0.01;
        } else if (skillToUseName === "Magnus Exorcismus") {
            n_SpSkill = 1;
            n_A_Weapon_element = 6;
            hitCount = n_A_ActiveSkillLV;
            wCast = 15;
            wDelay = 4;
            if (targetStatsArray[TARGET_STAT_RACE] != 6 && targetStatsArray[TARGET_STAT_ELEMENT] < 90) {
                n_A_MATK[2] = 0;
                n_A_MATK[0] = 0;
                n_A_MATK[1] = 0;
            }
        } else if (skillToUseName === "Estin") {
            n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
            wCast = 0.1;
            wDelay = 0.5;
            if (targetStatsArray[TARGET_STAT_SIZE] == 0)
                skillModifier = n_A_ActiveSkillLV * 0.1;
            else
                skillModifier = 0.01;
            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (skillToUseName === "Estun") {
            n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
            wCast = 0.1;
            wDelay = 0.5;

            skillModifier = n_A_ActiveSkillLV * 0.05;


            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (skillToUseName === "Esma") {
            n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
            n_SpSkill = 1;
            hitCount = n_A_ActiveSkillLV;
            wCast = 2;
            wDelay = 0.5;
            skillModifier = 0.4 + n_A_BaseLV / 100;
            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (skillToUseName === "Flaming Petals") {
            n_A_Weapon_element = 3;
            skillModifier = 0.9;
            hitCount = n_A_ActiveSkillLV;
            wCast = 0.7 * n_A_ActiveSkillLV;
            wDelay = 0.01;
        } else if (skillToUseName === "Blaze Shield") {
            n_A_Weapon_element = 3;
            skillModifier = 0.5;
            hitCount = Math.round(n_A_ActiveSkillLV / 2) + 4;
            wCast = 6.5 - 0.5 * n_A_ActiveSkillLV;
            wDelay = 1;
            n_SpSkill = 1;
        } else if (skillToUseName === "Exploding Dragon") {
            n_A_Weapon_element = 3;
            skillModifier = 1.5 + n_A_ActiveSkillLV * 1.5;
            hitCount = 1;
            wCast = 3;
            wDelay = 3;
        } else if (skillToUseName === "Freezing Spear") {
            n_A_Weapon_element = 1;
            skillModifier = 1;
            hitCount = n_A_ActiveSkillLV + 2;
            wCast = n_A_ActiveSkillLV * 0.7;
            wDelay = 0.01;
        } else if (skillToUseName === "Snow Flake Draft") {
            n_A_Weapon_element = 1;
            skillModifier = 1.0 + n_A_ActiveSkillLV * 0.5;
            hitCount = 1;
            wCast = 3;
            wDelay = 3;
        } else if (skillToUseName === "Wind Blade") {
            n_A_Weapon_element = 4;
            skillModifier = 1.0;
            hitCount = Math.floor(n_A_ActiveSkillLV / 2) + 1;
            wCast = Math.floor(n_A_ActiveSkillLV / 2) + 1;
            wDelay = 1;
        } else if (skillToUseName === "Lightning Jolt") {
            n_A_Weapon_element = 4;
            skillModifier = 1.6 + 0.4 * n_A_ActiveSkillLV;
            hitCount = 1;
            wCast = 4;
            wDelay = 0.01;

        } else if (skillToUseName === "First Wind") {
            n_A_Weapon_element = 4;
            skillModifier = 1.0 + n_A_ActiveSkillLV * 1.0;
            hitCount = 1;
            wCast = 4;
            wDelay = 0.01;
        }

        wCast *= n_A_CAST;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BattleMagicCalc(n_A_MATK[b] * skillModifier);
            myInnerHtml("ATK_0" + b, finalDamages[b] * hitCount + "(" + finalDamages[b] + SubName[8] + hitCount + "hit)", 0);
            finalDamages[b] *= hitCount;
        }

        CastAndDelay();

        BattleVariousResults(wCast, wDelay);
        myInnerHtml("BattleHIT", 100, 0);
    }
}


function ApplyATKBonusPercentage() {
    wA01 = 100;
    
    ;
    if (skillToUseName != "Occult Impaction" && skillToUseName != "Guillotine Fist" && skillToUseName != "Guillotine Fist (MaxSP-1)") {
        if (SkillSearch("Auto Berserk"))
            wA01 += 32;
        else if (n_A_PassSkill2[12])
            wA01 += 5;


        if (SkillSearch("Spear Dynamo"))
            wA01 += SkillSearch("Spear Dynamo") * 5;
        if (SkillSearch("True Sight"))
            wA01 += SkillSearch("True Sight") * 2;
        if (n_A_PassSkill5[3])
            wA01 += 100;
        if (n_A_PassSkill6[2])
            wA01 += 10;
        if (StPlusItem(ATK_PERCENTAGE))
            wA01 += StPlusItem(ATK_PERCENTAGE);
    }
    n_A_CriATK[2] = n_A_CriATK[2] * wA01 / 100;
    n_A_CriATK[0] = n_A_CriATK[0] * wA01 / 100;
    n_A_CriATK[1] = n_A_CriATK[1] * wA01 / 100;
    n_A_DMG[2] = n_A_DMG[2] * wA01 / 100;
    n_A_DMG[0] = n_A_DMG[0] * wA01 / 100;
    n_A_DMG[1] = n_A_DMG[1] * wA01 / 100;
}


function ApplySkillModifier(skillModifier, isCrit) {
    wA02 = skillModifier * 100;
    if (SkillSearch("Maximum Power-Thust")) {
        wA02 += 20 * SkillSearch("Maximum Power-Thust");
    } else {
        if (SkillSearch("Power-Thrust"))
            wA02 += SkillSearch("Power-Thrust") * 5;
        if (SkillSearch("Power-Thrust") == 0 && n_A_PassSkill2[8])
            wA02 += n_A_PassSkill2[8] * 5 / 5;
    }
    if (SkillSearch("Kihop")) {
        wA02 += 2 * SkillSearch("Kihop") * SkillSearch("Party Members (Kihop Bonus");
    }

    if (isCrit == 0) {
        n_A_DMG[2] = Math.floor(n_A_DMG[2] * wA02 / 100);
        n_A_DMG[0] = Math.floor(n_A_DMG[0] * wA02 / 100);
        n_A_DMG[1] = Math.floor(n_A_DMG[1] * wA02 / 100);
    } else {
        n_A_CriATK[1] = Math.floor(n_A_CriATK[1] * wA02 / 100);
        n_A_CriATK[0] = Math.floor(n_A_CriATK[0] * wA02 / 100);
        n_A_CriATK[2] = Math.floor(n_A_CriATK[2] * wA02 / 100);
    }
}


function BattleTAKA() {
    if (n_A_WeaponType ==  WEAPON_TYPE_BOW && SkillSearch("Blitz Beat") && skillToUseName != "Sharp Shooting (Temp)") {
        myInnerHtml("bSUBname", "Bird Damage (Atk Rate))", 0);
        wBTw1 = Math.floor((n_A_JobLV - 1) / 10 + 1);
        if (wBTw1 > 5) wBTw1 = 5;
        wBTw2 = SkillSearch("Blitz Beat");
        if (wBTw2 < wBTw1)
            wBTw1 = wBTw2;
        wBT = 80 + Math.floor(n_A_DEX / 10) * 2 + Math.floor(n_A_INT / 2) * 2 + SkillSearch("Steel Crow") * 6;
        wBT = Math.floor(wBT * element[targetStatsArray[TARGET_STAT_ELEMENT]][0]);
        wBT = tPlusDamCut(wBT);
        wBTw3 = Math.round((1 + n_A_LUK * 0.3) * 100) / 100;
        if (targetStatsArray[TARGET_STAT_MOB_INDEX] == 44)
            wBT = 0;
        myInnerHtml("bSUB", wBT * wBTw1 + "(" + wBTw3 + "%)", 0);
        wBT = wBT * wBTw1 * wBTw3 / 100;
        wBT = wBT * (hitRate + ((100 - hitRate) * criticalRate / 100)) / 100;
        wBTw1 = 0;
        return Math.round(wBT * 100) / 100;
    } else
        return 0;
}


function HealCalc(HealLv, HealType) {
    wHeal = Math.floor((n_A_BaseLV + n_A_INT) / 8) * (HealLv * 8 + 4);
    wHealBAI = 100;
    wHealBAI += SkillSearch("Mediatio") * 2;
    if (EquipNumSearch("Staff of Recovery"))
        wHealBAI += Math.floor(weaponRefinementLevel * 1.5)
    if (CardNumSearch("White Lady"))
        wHealBAI += 30 * CardNumSearch("White Lady");
    if (HealType == 0)
        if (EquipNumSearch("0") || EquipNumSearch("0"))
            wHealBAI += 50;
    wHeal = Math.floor(wHeal * wHealBAI / 100);
    return wHeal;
}

function BattleVariousResults(cast, afterCastDelay) {

    if (afterCastDelay == 0)
        w = 1 / (cast + n_A_ASPD) * finalDamages[1];
    else
        w = 1 / (cast + afterCastDelay) * finalDamages[1];
    w *= 100;
    w = Math.round(w);
    w /= 100;

    if (n_SpSkill)
        myInnerHtml("DPS", "Special", 0);
    else
        myInnerHtml("DPS", w, 0);
    
    if (targetStatsArray[TARGET_STAT_MOB_INDEX] == 44 && skillToUseName != "Basic Attack") {
        for (i = 0; i <= 2; i++) {
            finalDamages[i] = 0;
            myInnerHtml("ATK_0" + i, 0, 0);
        }
    }

    tPlusAG();
    w = targetStatsArray[TARGET_STAT_HP];
    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[2];
    }
    if (i < 1000)
        myInnerHtml("MinATKnum", i, 0);
    else
        myInnerHtml("MinATKnum", SubName[5], 0);
    w = targetStatsArray[TARGET_STAT_HP];
    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[0];
    }
    if (i < 1000)
        myInnerHtml("MaxATKnum", i, 0);
    else
        myInnerHtml("MaxATKnum", SubName[5], 0);
    w = targetStatsArray[TARGET_STAT_HP];
    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[1];
    }

    if (InWarOfEmperium == 0) {
        if (i < 1000) {
            myInnerHtml("AtkBaseExp", Math.round(targetStatsArray[16] / i) + "Exp", 0);
            myInnerHtml("AtkJobExp", Math.round(targetStatsArray[17] / i) + "Exp", 0);
        } else {
            myInnerHtml("AtkBaseExp", SubName[7], 0);
            myInnerHtml("AtkJobExp", SubName[7], 0);
        }
    }
    if (i < 1000) {
        myInnerHtml("AveATKnum", i, 0);

        n_AveATKnum = i;


        if (afterCastDelay == 0)

            w = (cast + n_A_ASPD) * n_AveATKnum;
        else
            w = (cast + afterCastDelay) * n_AveATKnum;
        w = Math.floor(w * 100) / 100;

        if (n_SpSkill)
            myInnerHtml("BattleTime", "Special", 0);
        else
            myInnerHtml("BattleTime", w + "s", 0);
    } else {
        myInnerHtml("AveATKnum", SubName[5], 0);
        myInnerHtml("BattleTime", SubName[6], 0);
    }


    n_SpSkill = 0

    if (InWarOfEmperium == 0) {
        w = CalculateDamageReceived();

        w = Math.round(w * (100 - n_A_LUCKY)) / 100;
        w = Math.round(w * (100 - w_FLEE)) / 100;
        if (SkillSearch("Guard")) {
            w = Math.round(w * w_AG[SkillSearch("Guard")]) / 100;
        }
        if (n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SWORD && SkillSearch("Parrying")) {
            w = Math.round(w * (80 - SkillSearch("Parrying") * 3)) / 100;
        }
        if (SkillSearch("Counter Instinct")) {
            w = Math.round(w * (100 - SkillSearch("Counter Instinct") * 7.5)) / 100;
        }
        myInnerHtml("AverageReceivedDamageIncludingDodge", w + "Damage", 0);

    }
}

function CalculateDamageReceived() {

    
    w_HiDam = new Array();
    wBHD = targetStatsArray[TARGET_STAT_ATK2];
    w_HiDam[0] = targetStatsArray[TARGET_STAT_ATK];
    w_HiDam[1] = (targetStatsArray[TARGET_STAT_ATK] * 5 + wBHD) / 6;
    w_HiDam[2] = (targetStatsArray[TARGET_STAT_ATK] * 4 + wBHD * 2) / 6;
    w_HiDam[3] = (targetStatsArray[TARGET_STAT_ATK] + wBHD) / 2;
    w_HiDam[4] = (targetStatsArray[TARGET_STAT_ATK] * 2 + wBHD * 4) / 6;
    w_HiDam[5] = (targetStatsArray[TARGET_STAT_ATK] + wBHD * 5) / 6;
    w_HiDam[6] = wBHD;
    if (targetStatsArray[TARGET_STAT_ATK] == targetStatsArray[TARGET_STAT_ATK2])
        w_HiDam[6] = wBHD - 1;

    w_HiDam[0] = w_HiDam[0] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2];
    w_HiDam[1] = w_HiDam[1] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2];
    w_HiDam[2] = w_HiDam[2] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2];
    w_HiDam[3] = w_HiDam[3] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[1];
    w_HiDam[4] = w_HiDam[4] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0];
    w_HiDam[5] = w_HiDam[5] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0];
    w_HiDam[6] = w_HiDam[6] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0];


    if (SkillSearch("Divine Protection") && (targetStatsArray[TARGET_STAT_ELEMENT] >= 90 || targetStatsArray[TARGET_STAT_RACE] == 6)) {
        wBHD = Math.floor((3 + 4 / 100 * n_A_BaseLV) * SkillSearch("Divine Protection"));
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= wBHD;
    }


    if (SkillSearch("Solar Protection")) {
        wBHD = Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 2);
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= wBHD;
    }


    wBHD = StPlusCard(DAMAGE_INC_DEC_ELEMENT_NEUTRAL_PERCENTAGE);
    wBHD += StPlusItem(DAMAGE_INC_DEC_ELEMENT_NEUTRAL_PERCENTAGE);
    if (EquipNumSearch("0") || EquipNumSearch("0"))
        wBHD += n_A_SHOULDER_DEF_PLUS * 3;
    if (SkillSearch("Skin Tempering"))
        wBHD += SkillSearch("Skin Tempering");
    if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch("Orc Baby"))
        wBHD += 5;
    if (wBHD != 0) {
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }

    if (SkillSearch("Energy Coat")) {
        wBHD = 6 * SkillSearch("Energy Coat");
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    wBHD = 0;
    if (CardNumSearch("0") && n_A_JobSearch() == 3 && (targetStatsArray[TARGET_STAT_RACE] == 6 || targetStatsArray[TARGET_STAT_RACE] == 1))
        wBHD += 30;
    if (n_A_PassSkill2[14] && targetStatsArray[TARGET_STAT_RACE] == 6 && n_A_JOB != 13 && n_A_JOB != 27)
        wBHD += n_A_PassSkill2[14] * 5;
    if (targetStatsArray[TARGET_STAT_RACE] == 9 && SkillSearch("Dragonology"))
        wBHD += SkillSearch("Dragonology") * 4;
    wBHD += StPlusCard(DAMAGE_INC_DEC_RACE_PERCENTAGE + targetStatsArray[TARGET_STAT_RACE]);
    wBHD += StPlusItem(DAMAGE_INC_DEC_RACE_PERCENTAGE + targetStatsArray[TARGET_STAT_RACE]);
    if (wBHD != 0) {
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    wBHD = 0;
    wBHD += StPlusCard(DAMAGE_INC_DEC_SIZE_PERCENTAGE + targetStatsArray[TARGET_STAT_SIZE]);
    wBHD += StPlusItem(DAMAGE_INC_DEC_SIZE_PERCENTAGE + targetStatsArray[TARGET_STAT_SIZE]);
    if (targetStatsArray[TARGET_STAT_SIZE] == 1) {
        if (EquipNumSearch("Hurricane Fury"))
            wBHD += weaponRefinementLevel;
    }

    if (wBHD != 0) {
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    if (targetStatsArray[19] == 0) {
        wBHD = StPlusCard(NORMAL_ATTACK_PERCENTAGE);
        wBHD += StPlusItem(NORMAL_ATTACK_PERCENTAGE);
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    if (targetStatsArray[20]) {
        wBHD = StPlusCard(RESISTANCE_RANGE_ATTACK_PERCENTAGE);
        wBHD += StPlusItem(RESISTANCE_RANGE_ATTACK_PERCENTAGE);
        if (SkillSearch("Gunslinger's Panic"))
            wBHD += 20;
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);

        if (SkillSearch("Defending Aura")) {
            wBHD = 5 + 15 * SkillSearch("Defending Aura");
            for (i = 0; i <= 6; i++)
                w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
        }
    }


    if (targetStatsArray[19] == 1 && CardNumSearch("Alice")) {
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * 40 / 100);

    }


    if (targetStatsArray[TARGET_STAT_MOB_INDEX] == 275 && CardNumSearch("Bongun")) {
        wBHD = 100 * CardNumSearch("Bongun");
        for (i = 0; i <= 6; i++)
            w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD / 100);

    }


    wBHD = StPlusCard(3000 + targetStatsArray[TARGET_STAT_MOB_INDEX]);
    wBHD += StPlusItem(3000 + targetStatsArray[TARGET_STAT_MOB_INDEX]);
    for (i = 0; i <= 6; i++)
        w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);


    if (EquipNumSearch("Strong Shield")) {
        wBHD = 20;
        for (i = 0; i <= 6; i++)
            w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD / 100);

    }

    for (i = 0; i <= 6; i++) {
        if (w_HiDam[i] < 1)
            w_HiDam[i] = 1;
        w_HiDam[i] = Math.floor(w_HiDam[i] * 100) / 100;
    }

    if (n_A_PassSkill2[5])
        for (i = 0; i <= 6; i++)
            w_HiDam[i] = Math.floor(w_HiDam[i] / 2);

    w_HiDam[0] = Math.floor(w_HiDam[0]);
    w_HiDam[6] = Math.floor(w_HiDam[6]);


    wBHD = 0;
    for (i = 0; i <= 6; i++)
        wBHD += w_HiDam[i];
    wBHD = Math.round(wBHD / 7);
    myInnerHtml("AverageReceivedDamage", wBHD + " (" + w_HiDam[0] + " ~ " + w_HiDam[6] + ")", 0);

    return wBHD;
}

function BattleMagicCalc(wBMC) {
    
    
    wBMC_MDEF = targetStatsArray[TARGET_STAT_MDEF];
    wMDEF_w = 0;
    if (EquipNumSearch("Staff of Piercing"))
        wMDEF_w += 10 + weaponRefinementLevel;
    if (targetStatsArray[19] == 0 && CardNumSearch("High Wizard Kathryne (MVP)"))
        wMDEF_w += 100;
    if (targetStatsArray[19] == 1 && CardNumSearch("Vesper"))
        wMDEF_w += 30 * CardNumSearch("Vesper");
    if (wMDEF_w > 100)
        wMDEF_w = 100;
    if (wMDEF_w != 0) {
        wBMC_MDEF = wBMC_MDEF - Math.floor(wBMC_MDEF * wMDEF_w / 100);
        n_B_MDEF2 = n_B_MDEF2 - Math.floor(n_B_MDEF2 * wMDEF_w / 100);
    }
    if (skillToUseName == "Fire Pillar")
        wBMC2 = Math.floor(wBMC + 50);
    else
        wBMC2 = Math.floor(wBMC * (100 - wBMC_MDEF) / 100 - n_B_MDEF2);
    if (wBMC2 < 1) wBMC2 = 1;
    if (skillToUseName == "Magnus Exorcismus") {
        if (targetStatsArray[TARGET_STAT_RACE] != 6 && targetStatsArray[TARGET_STAT_ELEMENT] < 90) {
            wBMC2 = 0;
        }
    }

    wBMC2 = Math.floor(wBMC2 * element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element]);
    if (90 <= targetStatsArray[TARGET_STAT_ELEMENT] && (skillToUseName == "Soul Strike" || skillToUseName == ""))
        wBMC2 = Math.floor(wBMC2 * (1 + 0.05 * n_A_ActiveSkillLV));


    if (targetStatsArray[TARGET_STAT_RACE] == 9 && SkillSearch("Dragonology"))
        wBMC2 = wBMC2 * (100 + SkillSearch("Dragonology") * 2) / 100;

    if (targetStatsArray[TARGET_STAT_RACE] == 8)
        wBMC2 = wBMC2 * (100 + CardNumSearch("Dolor of Thanatos") * 10) / 100;

    if (targetStatsArray[TARGET_STAT_RACE] == 6)
        wBMC2 = wBMC2 * (100 + CardNumSearch("Skeggiold") * 2) / 100;

    wBMC2 = tPlusDamCut(wBMC2);


    wBMC2 = wBMC2 * (100 + StPlusItem(5000 + n_A_ActiveSkill) + StPlusCard(5000 + n_A_ActiveSkill)) / 100;

    wBMC2 = Math.floor(wBMC2);

    return wBMC2;
}