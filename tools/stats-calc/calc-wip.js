function CalculateAllStats(FORM_DATA, targetStats) {
    let stats = {
        job: 0,
        baseLevel: 0,
        jobLevel: 0,
        str: 0,
        agi: 0,
        vit: 0,
        dex: 0,
        int: 0,
        luk: 0,
        bstr: 0, // A_STRp
        bagi: 0, // A_AGIp
        bvit: 0, // A_VITp
        bdex: 0, // A_DEXp
        bint: 0, // A_INTp
        bluk: 0, // A_LUKp
        maxHp: 0, // A_MaxHP
        maxSp: 0, // A_MaxSP
        def: 0,
        cast: 0,
        weaponType: 0,
        weapon1: 0,
        weapon1ATK: 0,
        weapon2: 0,
        weapon2ATK: 0,
        weapon1Element: 0,
        weapon2Element: 0,
        isRebirth: false,
        passiveSkills: [],
        supportiveSkills: [],
        performanceSkills: [],
        supportiveSkillsBattleChant: [],
        groundSupportiveSkills: [],
        foodBoxBonus: [],
        cards: [],
        equipments: [],
        arrow: null,
        weapon1Index: 0,
        weapon2Index: 0,
        shoulderDefPlus: 0,
        headDefPlus: 0,
        bodyDefPlus: 0,
        shieldHandDefPlus: 0,
        shoesHandDefPlus: 0,
        totalGearDefPlus: 0,
        speedPotion: false,
    }
    let {job, isRebirth} = n_A_JobSet(FORM_DATA);
    stats.job = job;
    stats.isRebirth = isRebirth;

    stats.supportiveSkills = new Array();
    for (let i = 0; i <= 15; i++)
        stats.supportiveSkills[i] = 0;


    stats.performanceSkills = new Array();
    for (let i = 0; i <= 45; i++)
        stats.performanceSkills[i] = 0;
    stats.performanceSkills[20] = 100;
    stats.performanceSkills[21] = 100;
    stats.performanceSkills[22] = 130;
    stats.performanceSkills[29] = 80;
    stats.performanceSkills[23] = 100;
    stats.performanceSkills[24] = 130;
    stats.performanceSkills[25] = 50;
    stats.performanceSkills[26] = 50;
    stats.performanceSkills[30] = 10;
    stats.performanceSkills[31] = 10;
    stats.performanceSkills[32] = 10;
    stats.performanceSkills[33] = 10;
    stats.performanceSkills[34] = 10;
    stats.performanceSkills[35] = 10;
    stats.performanceSkills[36] = 10;


    stats.supportiveSkillsBattleChant = new Array();
    for (let i = 0; i <= 4; i++)
        stats.supportiveSkillsBattleChant[i] = 0;


    stats.groundSupportiveSkills = new Array();
    for (let i = 0; i <= 3; i++)
        stats.groundSupportiveSkills[i] = 0;


    stats.foodBoxBonus = new Array();
    for (let i = 0; i <= 10; i++)
        stats.foodBoxBonus[i] = 0;

    if (stats.job == 20) {
        if (SuperNoviceFullWeaponCHECK == 0 && eval(FORM_DATA.A_PASSIVE_SKILL9) == 1)
            SuperNoviceFullWeapon(1);
        else if (SuperNoviceFullWeaponCHECK == 1 && eval(FORM_DATA.A_PASSIVE_SKILL9) == 0)
            SuperNoviceFullWeapon(0);
    }
    stats.baseLevel = eval(FORM_DATA.A_BaseLV);
    stats.jobLevel = eval(FORM_DATA.A_JobLV);

    stats.str = eval(FORM_DATA.A_STR);
    stats.agi = eval(FORM_DATA.A_AGI);
    stats.vit = eval(FORM_DATA.A_VIT);
    stats.dex = eval(FORM_DATA.A_DEX);
    stats.int = eval(FORM_DATA.A_INT);
    stats.luk = eval(FORM_DATA.A_LUK);
    let SU_STR = stats.str;
    let SU_AGI = stats.agi;
    let SU_VIT = stats.vit;
    let SU_DEX = stats.dex;
    let SU_INT = stats.int;
    let SU_LUK = stats.luk;

    stats.weaponType = eval(FORM_DATA.A_WeaponType);

    stats.arrow = eval(FORM_DATA.A_Arrow);
    stats.weapon1Index = eval(FORM_DATA.A_weapon1);

    stats.weapon1Lv = ItemOBJ[stats.weapon1Index][4];
    stats.weapon1ATK = ItemOBJ[stats.weapon1Index][3];

    stats.weapon2LV_upgradeBonusATK = 0;
    stats.weapon2LV_Minplus = 0;
    stats.weapon2LV_overUpgradeBonusATK = 0;
    stats.weapon2LV = 0;
    stats.weapon2_ATK = 0;
    stats.weapon2_RefinementLevel = 0;
    stats.cards = [];
    stats.cards[0] = eval(FORM_DATA.A_weapon1_card1);
    stats.cards[1] = eval(FORM_DATA.A_weapon1_card2);
    stats.cards[2] = eval(FORM_DATA.A_weapon1_card3);
    stats.cards[3] = eval(FORM_DATA.A_weapon1_card4);
    if (hasLeftHand) {
        stats.cards[4] = eval(FORM_DATA.A_weapon2_card1);
        stats.cards[5] = eval(FORM_DATA.A_weapon2_card2);
        stats.cards[6] = eval(FORM_DATA.A_weapon2_card3);
        stats.cards[7] = eval(FORM_DATA.A_weapon2_card4);
    } else {
        stats.cards[4] = 0;
        stats.cards[5] = 0;
        stats.cards[6] = 0;
        stats.cards[7] = 0;
    }
    stats.cards[8] = eval(FORM_DATA.A_head1_card);
    stats.cards[9] = eval(FORM_DATA.A_head2_card);
    stats.cards[10] = eval(FORM_DATA.A_left_card);
    stats.cards[11] = eval(FORM_DATA.A_body_card);
    stats.cards[12] = eval(FORM_DATA.A_shoulder_card);
    stats.cards[13] = eval(FORM_DATA.A_shoes_card);
    stats.cards[14] = eval(FORM_DATA.A_acces1_card);
    stats.cards[15] = eval(FORM_DATA.A_acces2_card);

    SetCard(stats);

    if (hasLeftHand) {

        if (!targetStats.isStaticPlant) {

            stats.weapon2Index = eval(FORM_DATA.A_weapon2);
            stats.weapon2LV = ItemOBJ[stats.weapon2Index][4];
            stats.weapon2_ATK = ItemOBJ[stats.weapon2Index][3];
            stats.weapon2_RefinementLevel = eval(FORM_DATA.A_Weapon2_ATKplus);


            if (stats.weapon2LV == 1) {
                stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 2;
                if (stats.weapon2_RefinementLevel >= 8) {
                    stats.weapon2LV_Minplus = 1;
                    stats.weapon2LV_overUpgradeBonusATK = 3 * (stats.weapon2_RefinementLevel - 7);
                }
            } else if (stats.weapon2LV == 2) {
                stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 3;
                if (stats.weapon2_RefinementLevel >= 7) {
                    stats.weapon2LV_Minplus = 1;
                    stats.weapon2LV_overUpgradeBonusATK = 5 * (stats.weapon2_RefinementLevel - 6);
                }
            } else if (stats.weapon2LV == 3) {
                stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 5;
                if (stats.weapon2_RefinementLevel >= 6) {
                    stats.weapon2LV_Minplus = 1;
                    stats.weapon2LV_overUpgradeBonusATK = 8 * (stats.weapon2_RefinementLevel - 5);
                }
            } else if (stats.weapon2LV == 4) {
                stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 7;
                if (stats.weapon2_RefinementLevel >= 5) {
                    stats.weapon2LV_Minplus = 1;
                    stats.weapon2LV_overUpgradeBonusATK = 14 * (stats.weapon2_RefinementLevel - 4);
                }
            }
        }
    }

    if (FORM_DATA.A_weapon2) {
        stats.weapon2Index = eval(FORM_DATA.A_weapon2);
        stats.weapon2_ATK = ItemOBJ[stats.weapon2Index][3];
    } else {
        stats.weapon2_ATK = 0;
    }
    let weaponRefinementLevel = eval(FORM_DATA.A_Weapon_ATKplus);

    stats.weapon1LV_upgradeBonusATK = 0;
    stats.weapon1LV_Minplus = 0;
    stats.weapon1LV_overUpgradeBonusATK = 0;
    if (stats.weapon1Lv == 1) {
        stats.weapon1LV_upgradeBonusATK = weaponRefinementLevel * 2;
        if (weaponRefinementLevel >= 8) {
            stats.weapon1LV_Minplus = 1;
            stats.weapon1LV_overUpgradeBonusATK = 3 * (weaponRefinementLevel - 7);
        }
    } else if (stats.weapon1Lv == 2) {
        stats.weapon1LV_upgradeBonusATK = weaponRefinementLevel * 3;
        if (weaponRefinementLevel >= 7) {
            stats.weapon1LV_Minplus = 1;
            stats.weapon1LV_overUpgradeBonusATK = 5 * (weaponRefinementLevel - 6);
        }
    } else if (stats.weapon1Lv == 3) {
        stats.weapon1LV_upgradeBonusATK = weaponRefinementLevel * 5;
        if (weaponRefinementLevel >= 6) {
            stats.weapon1LV_Minplus = 1;
            stats.weapon1LV_overUpgradeBonusATK = 8 * (weaponRefinementLevel - 5);
        }
    } else if (stats.weapon1Lv == 4) {
        stats.weapon1LV_upgradeBonusATK = weaponRefinementLevel * 7;
        if (weaponRefinementLevel >= 5) {
            stats.weapon1LV_Minplus = 1;
            stats.weapon1LV_overUpgradeBonusATK = 14 * (weaponRefinementLevel - 4);
        }
    }
    stats.headDefPlus = eval(FORM_DATA.A_HEAD_DEF_PLUS);
    stats.bodyDefPlus = eval(FORM_DATA.A_BODY_DEF_PLUS);
    stats.shieldHandDefPlus = eval(FORM_DATA.A_LEFT_DEF_PLUS);
    stats.shoulderDefPlus = eval(FORM_DATA.A_SHOULDER_DEF_PLUS);
    stats.shoesHandDefPlus = eval(FORM_DATA.A_SHOES_DEF_PLUS);
    stats.totalGearDefPlus = stats.headDefPlus + stats.bodyDefPlus + stats.shieldHandDefPlus + stats.shoulderDefPlus + stats.shoesHandDefPlus;

    stats.skillToUse = eval(FORM_DATA.A_ActiveSkill);
    stats.skillToUseName = SkillOBJ[stats.skillToUse][2];
    if (stats.skillToUse > 100000)
        stats.skillToUse = Math.floor((stats.skillToUse % 100000) / 100);

    stats.skillToUseLV = eval(FORM_DATA.A_ActiveSkillLV);
    stats.speedPotion = eval(FORM_DATA.A_SpeedPOT);

    stats.equipments[0] = eval(FORM_DATA.A_weapon1);
    if (hasLeftHand)
        stats.equipments[1] = eval(FORM_DATA.A_weapon2);
    else
        stats.equipments[1] = 0;
    stats.equipments[2] = eval(FORM_DATA.A_head1);
    stats.equipments[3] = eval(FORM_DATA.A_head2);
    stats.equipments[4] = eval(FORM_DATA.A_head3);
    stats.equipments[5] = eval(FORM_DATA.A_left);
    stats.equipments[6] = eval(FORM_DATA.A_body);
    stats.equipments[7] = eval(FORM_DATA.A_shoulder);
    stats.equipments[8] = eval(FORM_DATA.A_shoes);
    stats.equipments[9] = eval(FORM_DATA.A_acces1);
    stats.equipments[10] = eval(FORM_DATA.A_acces2);

    SetEquip(stats);


    stats.weapon1Element = eval(FORM_DATA.A_Weapon_element);
    stats.weapon2Element = stats.weapon1Element;


    if (stats.weapon1Element == 0) {
        for (let j = 0; ItemOBJ[stats.equipments[0]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[stats.equipments[0]][j + 11])
                stats.weapon1Element = ItemOBJ[stats.equipments[0]][j + 12];
        }
        for (let j = 0; ItemOBJ[stats.equipments[1]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[stats.equipments[1]][j + 11])
                stats.weapon2Element = ItemOBJ[stats.equipments[1]][j + 12];
        }

        if (201 <= cardOBJ[stats.cards[0]][0] && cardOBJ[stats.cards[0]][0] <= 204)
            stats.weapon1Element = cardOBJ[stats.cards[0]][0] - 200;
        if (201 <= cardOBJ[stats.cards[4]][0] && cardOBJ[stats.cards[4]][0] <= 204)
            stats.weapon2Element = cardOBJ[stats.cards[4]][0] - 200;

        if (stats.weaponType == WEAPON_TYPE_BOW || stats.weaponType == WEAPON_TYPE_HANDGUN || stats.weaponType == WEAPON_TYPE_RIFLE || stats.weaponType == WEAPON_TYPE_SHOTGUN || stats.weaponType == WEAPON_TYPE_GATLING_GUN || stats.weaponType == WEAPON_TYPE_GRENADE_LAUNCHER) {
            stats.weapon1Element = ArrowOBJ[stats.arrow][1];
        }
    }

    stats.passiveSkills = new Array();


    if (JobSkillPassOBJ[stats.job][0] != 999) stats.passiveSkills[0] = eval(FORM_DATA.A_PASSIVE_SKILL0);
    if (JobSkillPassOBJ[stats.job][1] != 999) stats.passiveSkills[1] = eval(FORM_DATA.A_PASSIVE_SKILL1);
    if (JobSkillPassOBJ[stats.job][2] != 999) stats.passiveSkills[2] = eval(FORM_DATA.A_PASSIVE_SKILL2);
    if (JobSkillPassOBJ[stats.job][3] != 999) stats.passiveSkills[3] = eval(FORM_DATA.A_PASSIVE_SKILL3);
    if (JobSkillPassOBJ[stats.job][4] != 999) stats.passiveSkills[4] = eval(FORM_DATA.A_PASSIVE_SKILL4);
    if (JobSkillPassOBJ[stats.job][5] != 999) stats.passiveSkills[5] = eval(FORM_DATA.A_PASSIVE_SKILL5);
    if (JobSkillPassOBJ[stats.job][6] != 999) stats.passiveSkills[6] = eval(FORM_DATA.A_PASSIVE_SKILL6);
    if (JobSkillPassOBJ[stats.job][7] != 999) stats.passiveSkills[7] = eval(FORM_DATA.A_PASSIVE_SKILL7);
    if (JobSkillPassOBJ[stats.job][8] != 999) stats.passiveSkills[8] = eval(FORM_DATA.A_PASSIVE_SKILL8);
    if (JobSkillPassOBJ[stats.job][9] != 999) stats.passiveSkills[9] = eval(FORM_DATA.A_PASSIVE_SKILL9);
    if (JobSkillPassOBJ[stats.job][10] != 999) stats.passiveSkills[10] = eval(FORM_DATA.A_PASSIVE_SKILL10);
    if (JobSkillPassOBJ[stats.job][11] != 999) stats.passiveSkills[11] = eval(FORM_DATA.A_PASSIVE_SKILL11);
    if (JobSkillPassOBJ[stats.job][12] != 999) stats.passiveSkills[12] = eval(FORM_DATA.A_PASSIVE_SKILL12);
    if (JobSkillPassOBJ[stats.job][13] != 999) stats.passiveSkills[13] = eval(FORM_DATA.A_PASSIVE_SKILL13);
    if (JobSkillPassOBJ[stats.job][14] != 999) stats.passiveSkills[14] = eval(FORM_DATA.A_PASSIVE_SKILL14);


    if (n_SkillSW) {
        stats.supportiveSkills[0] = eval(FORM_DATA.A_SUPPORTIVE_SKILL0);
        stats.supportiveSkills[1] = eval(FORM_DATA.A_SUPPORTIVE_SKILL1);
        stats.supportiveSkills[2] = eval(FORM_DATA.A_SUPPORTIVE_SKILL2);
        stats.supportiveSkills[3] = FORM_DATA.A_SUPPORTIVE_SKILL3 === "on";
        stats.supportiveSkills[4] = eval(FORM_DATA.A_SUPPORTIVE_SKILL4);
        stats.supportiveSkills[5] = FORM_DATA.A_SUPPORTIVE_SKILL5 === "on";
        stats.supportiveSkills[6] = eval(FORM_DATA.A_SUPPORTIVE_SKILL6);
        stats.supportiveSkills[7] = FORM_DATA.A_SUPPORTIVE_SKILL7 === "on";
        stats.supportiveSkills[8] = eval(FORM_DATA.A_SUPPORTIVE_SKILL8);
        stats.supportiveSkills[9] = eval(FORM_DATA.A_SUPPORTIVE_SKILL9);
        stats.supportiveSkills[10] = eval(FORM_DATA.A_SUPPORTIVE_SKILL10);
        stats.supportiveSkills[11] = FORM_DATA.A_SUPPORTIVE_SKILL11 === "on";
        stats.supportiveSkills[12] = FORM_DATA.A_SUPPORTIVE_SKILL12 === "on";
        stats.supportiveSkills[13] = eval(FORM_DATA.A_SUPPORTIVE_SKILL13);
        stats.supportiveSkills[14] = eval(FORM_DATA.A_SUPPORTIVE_SKILL14);
    }

    if (n_Skill3SW) {
        stats.performanceSkills[0] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_1);
        stats.performanceSkills[1] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_1);
        stats.performanceSkills[2] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_1);
        stats.performanceSkills[3] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_1);

        stats.performanceSkills[5] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_1);

        stats.performanceSkills[7] = eval(FORM_DATA.A_PERFORMANCE_SKILL7);

        stats.performanceSkills[9] = eval(FORM_DATA.A_PERFORMANCE_SKILL9);
        stats.performanceSkills[10] = eval(FORM_DATA.A_PERFORMANCE_SKILL10);
        stats.performanceSkills[11] = FORM_DATA.A_PERFORMANCE_SKILL11 === "on";
        if (stats.performanceSkills[11]) {
            stats.performanceSkills[12] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_STR);
            stats.performanceSkills[13] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_AGI);
            stats.performanceSkills[14] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_VIT);
            stats.performanceSkills[15] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_INT);
            stats.performanceSkills[16] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_DEX);
            stats.performanceSkills[17] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_LUK);
        }

        if (stats.performanceSkills[0]) {
            stats.performanceSkills[20] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_2);
            stats.performanceSkills[30] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_3);
        }
        if (stats.performanceSkills[1]) {
            stats.performanceSkills[21] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_2);
            stats.performanceSkills[31] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_3);
        }
        if (stats.performanceSkills[2]) {
            stats.performanceSkills[22] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_2);
            stats.performanceSkills[29] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_3);
            stats.performanceSkills[32] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_4);
        }
        if (stats.performanceSkills[3]) {
            stats.performanceSkills[23] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_2);
            stats.performanceSkills[33] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_3);
        }
        if (stats.performanceSkills[4]) {
            stats.performanceSkills[24] = eval(FORM_DATA.A_PERFORMANCE_SKILL4_2);
            stats.performanceSkills[34] = eval(FORM_DATA.A_PERFORMANCE_SKILL4_3);
        }
        if (stats.performanceSkills[5]) {
            stats.performanceSkills[25] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_2);
            stats.performanceSkills[35] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_3);
        }
        if (stats.performanceSkills[6]) {
            stats.performanceSkills[26] = eval(FORM_DATA.A_PERFORMANCE_SKILL6_2);
            stats.performanceSkills[36] = eval(FORM_DATA.A_PERFORMANCE_SKILL6_3);
        }

    }
    if (n_Skill4SW) {
        stats.performanceSkills[40] = FORM_DATA.A_PERFORMANCE_SKILL40 === "on";
        stats.performanceSkills[41] = eval(FORM_DATA.A_PERFORMANCE_SKILL41);
        stats.performanceSkills[42] = eval(FORM_DATA.A_PERFORMANCE_SKILL42);
        stats.performanceSkills[43] = eval(FORM_DATA.A_PERFORMANCE_SKILL43);
        stats.performanceSkills[44] = eval(FORM_DATA.A_PERFORMANCE_SKILL44);
    }
    if (n_Skill5SW) {
        stats.supportiveSkillsBattleChant[0] = FORM_DATA.A_BATTLECHANT_SKILL0 === "on";
        stats.supportiveSkillsBattleChant[1] = FORM_DATA.A_BATTLECHANT_SKILL1 === "on";
        stats.supportiveSkillsBattleChant[2] = FORM_DATA.A_BATTLECHANT_SKILL2 === "on";
        stats.supportiveSkillsBattleChant[3] = FORM_DATA.A_BATTLECHANT_SKILL3 === "on";
        stats.supportiveSkillsBattleChant[4] = FORM_DATA.A_BATTLECHANT_SKILL4 === "on";
    }
    if (n_Skill6SW) {
        stats.groundSupportiveSkills[0] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL0);
        stats.groundSupportiveSkills[1] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL1);
        stats.groundSupportiveSkills[2] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL2);
        stats.groundSupportiveSkills[3] = FORM_DATA.A_GROUND_SUPPORTIVE_SKILL3 === "on";
    }
    if (n_Skill7SW) {
        stats.foodBoxBonus[0] = FORM_DATA.A_FOOD_BOX_BONUS0 === "on";
        stats.foodBoxBonus[1] = FORM_DATA.A_FOOD_BOX_BONUS1 === "on";
        stats.foodBoxBonus[2] = FORM_DATA.A_FOOD_BOX_BONUS2 === "on";
        stats.foodBoxBonus[3] = eval(FORM_DATA.A_FOOD_BOX_BONUS3);
        stats.foodBoxBonus[4] = eval(FORM_DATA.A_FOOD_BOX_BONUS4);
        stats.foodBoxBonus[5] = eval(FORM_DATA.A_FOOD_BOX_BONUS5);
        stats.foodBoxBonus[6] = eval(FORM_DATA.A_FOOD_BOX_BONUS6);
        stats.foodBoxBonus[7] = eval(FORM_DATA.A_FOOD_BOX_BONUS7);
        stats.foodBoxBonus[8] = eval(FORM_DATA.A_FOOD_BOX_BONUS8);
        stats.foodBoxBonus[9] = FORM_DATA.A_FOOD_BOX_BONUS9 === "on";
        stats.foodBoxBonus[10] = FORM_DATA.A_FOOD_BOX_BONUS10 === "on";
    }


    let wSPC_STR = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][0];
    let wSPC_AGI = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][1];
    let wSPC_VIT = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][2];
    let wSPC_INT = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][3];
    let wSPC_DEX = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][4];
    let wSPC_LUK = JobStatsBonusPerLevel[stats.job][stats.jobLevel - 1][5];

    if (stats.job == 0 && isRebirth) {
        TenNovSTR = [0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        TenNovAGI = [0, 0, 0, 0, 1, 1, 1, 1, 1, 1];
        TenNovVIT = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        TenNovINT = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1];
        TenNovDEX = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        TenNovLUK = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        wSPC_STR = TenNovSTR[stats.jobLevel - 1];
        wSPC_AGI = TenNovAGI[stats.jobLevel - 1];
        wSPC_VIT = TenNovVIT[stats.jobLevel - 1];
        wSPC_INT = TenNovINT[stats.jobLevel - 1];
        wSPC_DEX = TenNovDEX[stats.jobLevel - 1];
        wSPC_LUK = TenNovLUK[stats.jobLevel - 1];
    }


    let wSPCall = StPlusItem(ALL_STATS, stats);
    wSPC_STR += StPlusItem(STR, stats) + wSPCall;
    wSPC_AGI += StPlusItem(AGI, stats) + wSPCall;
    wSPC_VIT += StPlusItem(VIT, stats) + wSPCall;
    wSPC_VIT += StPlusItem(213, stats);
    wSPC_INT += StPlusItem(INT, stats) + wSPCall;
    wSPC_INT += StPlusItem(214, stats);
    wSPC_DEX += StPlusItem(DEX, stats) + wSPCall;
    wSPC_LUK += StPlusItem(LUK, stats) + wSPCall;

    wSPC_DEX += SkillSearch("Owl's Eye", stats);
    wSPC_STR += SkillSearch("Crazy Uproar", stats) * 4;
    wSPC_STR += SkillSearch("Hilt Binding", stats);
    wSPC_STR += SkillSearch("Ninja Aura", stats);
    wSPC_INT += SkillSearch("Ninja Aura", stats);
    if (SkillSearch("Dragonology", stats))
        wSPC_INT += (Math.floor(SkillSearch("Dragonology", stats) / 2) + 1);
    if (SkillSearch("Chase Walk", stats)) {
        if (SkillSearch("Chase Walk", stats) == 5) wSPC_STR += 16;
        if (SkillSearch("Chase Walk", stats) == 4) wSPC_STR += 8;
        if (SkillSearch("Chase Walk", stats) == 3) wSPC_STR += 4;
        if (SkillSearch("Chase Walk", stats) == 2) wSPC_STR += 2;
        if (SkillSearch("Chase Walk", stats) == 1) wSPC_STR += 1;
    }
    if (SkillSearch("Increase Accuracy", stats)) {
        wSPC_DEX += 4;
        wSPC_AGI += 4;
    }

    let w = SkillSearch("Improve Concentration", stats);
    if (w) {
        w += 102;
        wSPC_DEX = Math.floor((stats.dex + wSPC_DEX) * w / 100) - stats.dex;
        wSPC_AGI = Math.floor((stats.agi + wSPC_AGI) * w / 100) - stats.agi;
    } else if (stats.groundSupportiveSkills[3]) {
        wSPC_DEX = Math.floor((stats.dex + wSPC_DEX) * 103 / 100) - stats.dex;
        wSPC_AGI = Math.floor((stats.agi + wSPC_AGI) * 103 / 100) - stats.agi;
    }

    wSPC_AGI += StPlusItem(212, stats);
    wSPC_DEX += StPlusItem(215, stats);
    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Magistrate Hat", stats))
        wSPC_AGI += 1;
    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Ayam", stats))
        wSPC_INT += 1;
    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Bride Mask", stats))
        wSPC_LUK += 2;
    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Mythical Lion Mask", stats))
        wSPC_DEX += 2;
    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Hahoe Mask", stats))
        wSPC_LUK += 1;
    if (stats.shoesHandDefPlus >= 9 && StPlusItem("Black Leather Boots", stats))
        wSPC_AGI += 2;

    wSPCall = StPlusCard(ALL_STATS, stats);
    wSPC_STR += StPlusCard(STR, stats) + wSPCall;
    wSPC_AGI += StPlusCard(AGI, stats) + wSPCall;
    wSPC_VIT += StPlusCard(VIT, stats) + wSPCall;
    wSPC_INT += StPlusCard(INT, stats) + wSPCall;
    wSPC_DEX += StPlusCard(DEX, stats) + wSPCall;
    wSPC_LUK += StPlusCard(LUK, stats) + wSPCall;


    if (n_A_JobSearch(stats.job) == 3)
        wSPC_INT += CardNumSearch("Rideword", stats);
    if (CardNumSearch("Despero of Thanatos", stats)) wSPC_INT += stats.shieldHandDefPlus;
    if (CardNumSearch("Green Maiden", stats)) wSPC_LUK += stats.shoulderDefPlus;
    if (CardNumSearch("Odium of Thanatos", stats)) wSPC_AGI += stats.shoesHandDefPlus;
    if (stats.cards[8] == 180) wSPC_STR += stats.headDefPlus;

    if (CardNumSearch("Obsidian", stats)) wSPC_VIT += Math.floor(SU_DEX / 18);
    if (CardNumSearch("Egnigem Cenia", stats)) wSPC_STR += Math.floor(SU_INT / 18);
    if (CardNumSearch("Venatu", stats)) wSPC_LUK += Math.floor(SU_AGI / 18);
    if (CardNumSearch("Ancient Mimic", stats)) wSPC_AGI += Math.floor(SU_LUK / 18);
    if (CardNumSearch("Mistress of Shelter", stats)) wSPC_INT += Math.floor(SU_STR / 18);
    if (CardNumSearch("Dame of Sentinel", stats)) wSPC_DEX += Math.floor(SU_VIT / 18);


    if (CardNumSearch("Aliot", stats)) {
        if (n_A_JobSearch(stats.job) == 1 || n_A_JobSearch(stats.job) == 2 || n_A_JobSearch(stats.job) == 6)
            wSPC_STR += 2;
        if (n_A_JobSearch(stats.job) == 3 || n_A_JobSearch(stats.job) == 4 || n_A_JobSearch(stats.job) == 5)
            wSPC_INT += 2;
    }

    wSPC_STR += stats.supportiveSkills[0];
    wSPC_INT += stats.supportiveSkills[0];
    wSPC_DEX += stats.supportiveSkills[0];
    if (stats.supportiveSkills[1] > 0)
        wSPC_AGI += stats.supportiveSkills[1] + 2;
    wSPC_LUK += (stats.supportiveSkills[3] * 30);
    if (stats.job == 24 && SkillSearch("True Sight", stats)) {
        wSPC_STR += 5;
        wSPC_AGI += 5;
        wSPC_VIT += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
        wSPC_LUK += 5;
    }


    if (SkillSearch("Sprint (STR + State)", stats))
        wSPC_STR += 10;


    if (stats.performanceSkills[40]) {
        wSPC_STR += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
    }
    wSPC_STR += stats.performanceSkills[41];
    wSPC_VIT += stats.performanceSkills[42];
    wSPC_AGI += stats.performanceSkills[43];
    wSPC_DEX += stats.performanceSkills[44];

    if (stats.supportiveSkillsBattleChant[0]) {
        wSPC_STR += 20;
        wSPC_AGI += 20;
        wSPC_VIT += 20;
        wSPC_DEX += 20;
        wSPC_INT += 20;
        wSPC_LUK += 20;
    }

    if (stats.groundSupportiveSkills[2] == 1) {
        wSPC_STR += 3;
        wSPC_AGI += 3;
        wSPC_VIT += 3;
        wSPC_DEX += 3;
        wSPC_INT += 3;
        wSPC_LUK += 3;
    }
    if (stats.groundSupportiveSkills[2] == 2) {
        wSPC_STR += 5;
        wSPC_AGI += 5;
        wSPC_VIT += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
        wSPC_LUK += 5;
    }


    if (stats.foodBoxBonus[3])
        wSPC_STR += stats.foodBoxBonus[3];
    if (stats.foodBoxBonus[4])
        wSPC_AGI += stats.foodBoxBonus[4];
    if (stats.foodBoxBonus[5])
        wSPC_VIT += stats.foodBoxBonus[5];
    if (stats.foodBoxBonus[6])
        wSPC_INT += stats.foodBoxBonus[6];
    if (stats.foodBoxBonus[7])
        wSPC_DEX += stats.foodBoxBonus[7];
    if (stats.foodBoxBonus[8])
        wSPC_LUK += stats.foodBoxBonus[8];

    if (stats.performanceSkills[11]) {
        if (stats.str + wSPC_STR < 99) {
            if (stats.str + wSPC_STR + Math.floor(stats.performanceSkills[12] / 2) < 99)
                wSPC_STR += Math.floor(stats.performanceSkills[12] / 2);
            else
                wSPC_STR = (99 - stats.str);
        }
        if (stats.agi + wSPC_AGI < 99) {
            if (stats.agi + wSPC_AGI + Math.floor(stats.performanceSkills[13] / 2) < 99)
                wSPC_AGI += Math.floor(stats.performanceSkills[13] / 2);
            else
                wSPC_AGI = (99 - stats.agi);
        }
        if (stats.vit + wSPC_VIT < 99) {
            if (stats.vit + wSPC_VIT + Math.floor(stats.performanceSkills[14] / 2) < 99)
                wSPC_VIT += Math.floor(stats.performanceSkills[14] / 2);
            else
                wSPC_VIT = (99 - stats.vit);
        }
        if (stats.int + wSPC_INT < 99) {
            if (stats.int + wSPC_INT + Math.floor(stats.performanceSkills[15] / 2) < 99)
                wSPC_INT += Math.floor(stats.performanceSkills[15] / 2);
            else
                wSPC_INT = (99 - stats.int);
        }
        if (stats.dex + wSPC_DEX < 99) {
            if (stats.dex + wSPC_DEX + Math.floor(stats.performanceSkills[16] / 2) < 99)
                wSPC_DEX += Math.floor(stats.performanceSkills[16] / 2);
            else
                wSPC_DEX = (99 - stats.dex);
        }
        if (stats.luk + wSPC_LUK < 99) {
            if (stats.luk + wSPC_LUK + Math.floor(stats.performanceSkills[17] / 2) < 99)
                wSPC_LUK += Math.floor(stats.performanceSkills[17] / 2);
            else
                wSPC_LUK = (99 - stats.luk);
        }
    }

    stats.str += wSPC_STR;
    stats.agi += wSPC_AGI;
    stats.vit += wSPC_VIT;
    stats.int += wSPC_INT;
    stats.dex += wSPC_DEX;
    stats.luk += wSPC_LUK;
    stats.bstr = wSPC_STR;
    stats.bagi = wSPC_AGI;
    stats.bvit = wSPC_VIT;
    stats.bint = wSPC_INT;
    stats.bdex = wSPC_DEX;
    stats.bluk = wSPC_LUK;


    // if (wSPC_STR >= 0)
    //     myInnerHtml("A_STRp", "+" + wSPC_STR, 0);
    // else
    //     myInnerHtml("A_STRp", wSPC_STR, 0);
    // if (wSPC_AGI >= 0)
    //     myInnerHtml("A_AGIp", "+" + wSPC_AGI, 0);
    // else
    //     myInnerHtml("A_AGIp", wSPC_AGI, 0);
    // if (wSPC_VIT >= 0)
    //     myInnerHtml("A_VITp", "+" + wSPC_VIT, 0);
    // else
    //     myInnerHtml("A_VITp", wSPC_VIT, 0);
    // if (wSPC_INT >= 0)
    //     myInnerHtml("A_INTp", "+" + wSPC_INT, 0);
    // else
    //     myInnerHtml("A_INTp", wSPC_INT, 0);
    // if (wSPC_DEX >= 0)
    //     myInnerHtml("A_DEXp", "+" + wSPC_DEX, 0);
    // else
    //     myInnerHtml("A_DEXp", wSPC_DEX, 0);
    // if (wSPC_LUK >= 0)
    //     myInnerHtml("A_LUKp", "+" + wSPC_LUK, 0);
    // else
    //     myInnerHtml("A_LUKp", wSPC_LUK, 0);

    let baseATK_w;

    if (isNonRangeWeapon()) {
        baseATK_w = Math.round(Math.floor(stats.str / 10) * Math.floor(stats.str / 10));
        stats.baseATK = stats.str + baseATK_w + Math.floor(stats.dex / 5) + Math.floor(stats.luk / 5);
    } else {
        baseATK_w = Math.round(Math.floor(stats.dex / 10) * Math.floor(stats.dex / 10));
        stats.baseATK = stats.dex + baseATK_w + Math.floor(stats.str / 5) + Math.floor(stats.luk / 5);
    }


    w = StPlusCard(ATK, stats);
    w += StPlusItem(ATK, stats);

    if (SU_STR >= 80 && CardNumSearch("Giant Whisper", stats))
        w += 20;
    if (SU_STR >= 95 && StPlusItem("Doom Slayer", stats))
        w += 340;
    if (SU_STR >= 44 && StPlusItem("Holgren's Refining Hammer", stats))
        w += 44;
    if (StPlusItem("Mythical Lion Mask", stats))
        w += stats.headDefPlus * 2;

    if (stats.groundSupportiveSkills[0] == 0 && stats.groundSupportiveSkills[1] >= 1 && (CardNumSearch("Pasana", stats) || stats.equipments[6] == 428 || stats.equipments[6] == 604))
        w += stats.groundSupportiveSkills[1] * 10;

    if (stats.foodBoxBonus[2])
        w += 10;
    if (stats.foodBoxBonus[9])
        w += 20;

    if (SkillSearch("Last Stand", stats))
        w += 100;
    if (SkillSearch("Gatling Fever", stats))
        w += 20 + 10 * SkillSearch("Gatling Fever", stats);

    if (stats.performanceSkills[9])
        w += 25 + 25 * stats.performanceSkills[9];


    stats.baseATK += w;


    let JobHP_A = new Array(0, 70, 50, 40, 50, 30, 40, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 0, 0, 0, 0, 0, 0, 70, 90, 75, 75, 84);
    let JobHP_B = new Array(5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 5, 6.5, 5, 3, 3.5);


    let wHPSL = 0;
    if (stats.job == 43) {
        if (stats.baseLevel >= 70) {
            if (stats.baseLevel <= 79)
                wHPSL = (stats.baseLevel - 70) * 40;
            else if (stats.baseLevel <= 84)
                wHPSL = (stats.baseLevel - 80) * 50;
            else if (stats.baseLevel <= 89)
                wHPSL = (stats.baseLevel - 80) * 50 - 10;
            else if (stats.baseLevel <= 92)
                wHPSL = (stats.baseLevel - 90) * 50;
            else if (stats.baseLevel <= 97)
                wHPSL = (stats.baseLevel - 90) * 50 - 10;
            else if (stats.baseLevel == 98) wHPSL = 375;
            else wHPSL = 4;
        }
    }

    w = 0;
    for (let i = 2; i <= stats.baseLevel; i++) {
        w += Math.round(JobHP_A[stats.job] * i / 100);
    }

    stats.maxHp = Math.floor((JobHP_B[stats.job] * stats.baseLevel) + 35 + w);


    if (stats.job == 44) {
        NinHP = new Array(131, 137, 144, 151, 159, 167, 175, 184, 193, 202, 212, 222, 232, 243, 254, 265, 277, 289, 301, 316, 331, 346, 364, 382, 400, 420, 440, 460, 482, 504, 526, 548, 572, 596, 620, 646, 672, 698, 726, 754, 784, 814, 844, 876, 908, 940, 975, 1010, 1100, 1140, 1180, 1220, 1260, 1300, 1340, 1385, 1430, 1475, 1520, 1565, 1615, 1665, 1715, 1765, 1815, 1880, 1935, 1990, 2045, 2100, 2160, 2200, 2280, 2340, 2400, 2460, 2520, 2580, 2640, 2705, 2770, 2835, 2900, 2965, 3030, 3100, 3170, 3240, 3310, 3380, 3455, 3530, 3605, 3680, 3760, 3840, 3920, 4000, 4080);
        stats.maxHp = NinHP[stats.baseLevel - 1];
    }

    if (stats.job == 45 && stats.baseLevel >= 10) {
        GunHP = new Array(202, 212, 222, 232, 243, 254, 265, 277, 289, 301, 316, 331, 346, 364, 382, 400, 420, 440, 460, 490, 520, 550, 580, 610, 650, 680, 710, 740, 770, 800, 830, 860, 890, 920, 950, 990, 1020, 1050, 1080, 1110, 1140, 1180, 1230, 1280, 1330, 1395, 1455, 1515, 1575, 1635, 1695, 1760, 1820, 1885, 1950, 2015, 2080, 2145, 2210, 2275, 2340, 2410, 2480, 2550, 2620, 2690, 2760, 2830, 2900, 2970, 3040, 3115, 3190, 3265, 3340, 3415, 3490, 3565, 3640, 3715, 3790, 3870, 3950, 4030, 4110, 4190, 4270, 4350, 4430, 4510);
        stats.maxHp = GunHP[stats.baseLevel - 10];
    }

    if (stats.job == 20 && stats.baseLevel == 99)
        stats.maxHp += 2000;

    if (isRebirth)
        stats.maxHp = Math.floor(stats.maxHp * 125 / 100);
    if (FORM_DATA.isAdopted == "on")
        stats.maxHp = Math.floor(stats.maxHp * 70 / 100);
    stats.maxHp = Math.floor((stats.maxHp - wHPSL) * (100 + stats.vit) / 100);


    if (stats.job == 41 && stats.baseLevel >= 70) {
        if (stats.baseLevel <= 79)
            stats.maxHp = Math.floor((2127 + 10 * (stats.baseLevel - 70)) * (100 + stats.vit) / 100);
        else if (stats.baseLevel <= 89)
            stats.maxHp = Math.floor((2200 + 50 * (stats.baseLevel - 80)) * (100 + stats.vit) / 100);
        else if (stats.baseLevel <= 99)
            stats.maxHp = Math.floor((2700 + 50 * (stats.baseLevel - 90)) * (100 + stats.vit) / 100);
    }

    if (stats.job == 42 && stats.baseLevel >= 70) {
        wKenseiHP = [3455, 3524, 3593, 3663, 3834, 3806, 3878, 3951, 4025, 4500];
        if (stats.baseLevel <= 79)
            stats.maxHp = Math.floor((2670 + 10 * (stats.baseLevel - 70)) * (100 + stats.vit) / 100);
        else if (stats.baseLevel <= 89)
            stats.maxHp = Math.floor((3000 + 20 * (stats.baseLevel - 80)) * (100 + stats.vit) / 100);
        else if (stats.baseLevel <= 99)
            stats.maxHp = Math.floor(wKenseiHP[stats.baseLevel - 90] * (100 + stats.vit) / 100);
    }

    if (SkillSearch("Taekwon Ranker", stats) && stats.baseLevel >= 90)
        stats.maxHp *= 3;


    stats.maxHp += SkillSearch("Faith", stats) * 200;
    let bkHP = stats.maxHp;
    w = 0;

    w += StPlusItem(MAXHP, stats);
    w += StPlusItem(VIT, stats);


    w += StPlusCard(MAXHP, stats);
    if (stats.bodyDefPlus >= 9 && CardNumSearch("Apocalypse", stats))
        w += 800;

    //Temporary remover card code.
    if (CardNumSearch("Remover", stats))
        w -= stats.bodyDefPlus * 40;

    if (stats.equipments[8] == 536) {
        wHPVS = n_A_JobSearch(stats.job);
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5)
            w += 5 * stats.baseLevel;
    }

    stats.maxHp += w;

    w = 0;

    w += StPlusItem(MAXHP_PERCENTAGE, stats);

    w += StPlusCard(MAXHP_PERCENTAGE, stats);

    if (SU_VIT >= 80 && CardNumSearch("Giant Whisper", stats))
        w += 3;

    if (CardNumSearch("Aliot", stats)) {
        if (n_A_JobSearch(stats.job) == 1 || n_A_JobSearch(stats.job) == 2 || n_A_JobSearch(stats.job) == 6)
            w += 5;
    }
    if (stats.shoesHandDefPlus >= 9 && CardNumSearch("Firelock Soldier", stats))
        w += 10;
    if (stats.shoesHandDefPlus <= 4 && CardNumSearch("Gold Acidus", stats))
        w += 4;
    if (stats.supportiveSkillsBattleChant[1])
        w += 100;
    if (StPlusItem("Variant Shoes", stats))
        w -= stats.shoesHandDefPlus;

    stats.maxHp = stats.maxHp * (100 + w) / 100;

    if (stats.groundSupportiveSkills[0] == 1 && stats.groundSupportiveSkills[1] >= 1 && (CardNumSearch("Swordfish", stats) || stats.equipments[6] == 429 || stats.equipments[6] == 605)) {
        dHP = [5, 9, 12, 14, 15];
        stats.maxHp = stats.maxHp * (100 + dHP[stats.groundSupportiveSkills[1] - 1]) / 100;
    }
    if (SkillSearch("Frenzy", stats))
        stats.maxHp *= 3;


    if (stats.performanceSkills[3])
        stats.maxHp += (Math.floor(bkHP * (105 + stats.performanceSkills[3] * 2 + stats.performanceSkills[33] + Math.floor(stats.performanceSkills[23] / 10)) / 100) - bkHP);


    stats.maxHp = Math.floor(stats.maxHp);


    // if (stats.maxHp >= 100) {
    //     if (stats.maxHp >= 10000)
    //         myInnerHtml("A_MaxHP", " " + stats.maxHp, 0);
    //     else
    //         myInnerHtml("A_MaxHP", stats.maxHp, 0);
    // } else
    //     myInnerHtml("A_MaxHP", " " + stats.maxHp, 0);


    let JobSP_A = new Array(1, 2, 2, 5, 2, 6, 3, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 1, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 0, 0, 0, 0, 0, 0, 0, 2, 4.7, 9, 3.75, 3.75);


    let wSPSL = 0;
    if (stats.job == 41 || stats.job == 43) {
        if (stats.baseLevel >= 70) {
            if (stats.baseLevel < 80)
                wSPSL = (stats.baseLevel - 70) * 4 + 5;
            else if (stats.baseLevel < 90)
                wSPSL = (stats.baseLevel - 80) * 4;
            else if (stats.baseLevel < 93)
                wSPSL = (stats.baseLevel - 90) * 4;
            else if (stats.baseLevel < 99)
                wSPSL = (stats.baseLevel - 90) * 4 - 10;
            else wSPSL = 1;
        }
    }

    stats.maxSp = Math.floor(10 + stats.baseLevel * JobSP_A[stats.job] - wSPSL);

    if (stats.job == 44) {
        if (stats.baseLevel <= 20) stats.maxSp = 11 + stats.baseLevel * 3;
        else if (stats.baseLevel <= 40) stats.maxSp = 71 + (stats.baseLevel - 20) * 4;
        else if (stats.baseLevel <= 60) stats.maxSp = 151 + (stats.baseLevel - 40) * 5;
        else if (stats.baseLevel <= 80) stats.maxSp = 251 + (stats.baseLevel - 60) * 6;
        else stats.maxSp = 370 + (stats.baseLevel - 80) * 8;
    }

    if (stats.job == 45) {
        if (stats.baseLevel <= 25) stats.maxSp = 10 + stats.baseLevel * 3;
        else if (stats.baseLevel <= 35) stats.maxSp = 85 + (stats.baseLevel - 25) * 4;
        else if (stats.baseLevel <= 40) stats.maxSp = 126 + (stats.baseLevel - 35) * 3;
        else if (stats.baseLevel <= 50) stats.maxSp = 141 + (stats.baseLevel - 40) * 4;
        else if (stats.baseLevel <= 75) stats.maxSp = 181 + (stats.baseLevel - 50) * 5;
        else if (stats.baseLevel <= 78) stats.maxSp = 306 + (stats.baseLevel - 75) * 6;
        else stats.maxSp = 330 + (stats.baseLevel - 78) * 6;
    }

    if (isRebirth)
        stats.maxSp = Math.floor(stats.maxSp * 125 / 100);
    if (eval(FORM_DATA.isAdopted == "on"))
        stats.maxSp = Math.floor(stats.maxSp * 70 / 100);
    stats.maxSp = Math.floor(stats.maxSp * (100 + stats.int) / 100);


    if (stats.job == 42 && stats.baseLevel >= 70) {
        if (stats.baseLevel <= 79)
            stats.maxSp = Math.floor((340 + 2 * (stats.baseLevel - 70)) * (100 + stats.int) / 100);
        else if (stats.baseLevel <= 89)
            stats.maxSp = Math.floor((385 + 2 * (stats.baseLevel - 80)) * (100 + stats.int) / 100);
        else if (stats.baseLevel <= 99)
            stats.maxSp = Math.floor((437 + 2 * (stats.baseLevel - 90)) * (100 + stats.int) / 100);
    }

    let bkSP = stats.maxSp;

    if (SkillSearch("Taekwon Ranker", stats) && stats.baseLevel >= 90)
        stats.maxSp *= 3;

    w = 0;

    w += StPlusItem(MAXSP, stats);
    w += StPlusItem(INT, stats);

    w += StPlusCard(MAXSP, stats);
    if (stats.headDefPlus >= 9 && stats.cards[8] == 298)
        w += 150;
    if (stats.headDefPlus <= 4 && stats.cards[8] == 179)
        w += 40;
    if (stats.cards[9] == 179)
        w += 40;

    if (SkillSearch("Kaina", stats))
        w += 30 * SkillSearch("Kaina", stats);

    if (stats.equipments[8] == 536) {
        wSPVS = n_A_JobSearch(stats.job);
        if (wSPVS == 1 || wSPVS == 2 || wSPVS == 6)
            w += 2 * stats.jobLevel;
    }
    if (weaponRefinementLevel >= 9 && StPlusItem("Lich's Bone Wand", stats))
        w += 300;


    stats.maxSp += w;

    w = 0;

    w += StPlusItem(MAXSP_PERCENTAGE, stats);

    w += StPlusCard(MAXSP_PERCENTAGE, stats);
    if (stats.shoesHandDefPlus >= 9 && CardNumSearch("Firelock Soldier", stats))
        w += 10;
    if (stats.shoesHandDefPlus <= 4 && CardNumSearch("Gold Acidus", stats))
        w += 4;

    if (CardNumSearch("Aliot", stats)) {
        if (n_A_JobSearch(stats.job) == 3 || n_A_JobSearch(stats.job) == 4 || n_A_JobSearch(stats.job) == 5)
            w += 5;
    }


    w += SkillSearch("Mediatio", stats);

    w += SkillSearch("Soul Drain", stats) * 2;
    if (stats.supportiveSkillsBattleChant[2])
        w += 100;
    if (StPlusItem("Variant Shoes", stats))
        w -= stats.shoesHandDefPlus;

    stats.maxSp = Math.floor(stats.maxSp * (100 + w) / 100);

    if (stats.performanceSkills[6])
        stats.maxSp += (Math.floor(bkSP * (100 + stats.performanceSkills[6] * 2 + stats.performanceSkills[36] + Math.floor(stats.performanceSkills[26] / 10)) / 100) - bkSP);


    // if (stats.maxSp >= 100)
    //     myInnerHtml("A_MaxSP", stats.maxSp, 0);
    // else
    //     myInnerHtml("A_MaxSP", " " + stats.maxSp, 0);


    stats.def = StPlusItem(DEF, stats);

    for (let i = 2; i <= 10; i++) {
        stats.def += ItemOBJ[stats.equipments[i]][3];
    }

    stats.def += StPlusCard(DEF, stats);

    if (stats.shieldHandDefPlus <= 5 && CardNumSearch("Arcluse", stats))
        stats.def += 2;
    if (stats.bodyDefPlus <= 5 && CardNumSearch("Goat", stats))
        stats.def += 2;
    if (stats.equipments[0] == 521) {
        if (weaponRefinementLevel <= 5)
            stats.def += 2;
        else if (weaponRefinementLevel >= 9)
            stats.def += 5;
        else
            stats.def += 3;
    }
    if (StPlusItem("Gatekeeper-DD", stats))
        stats.def += weaponRefinementLevel;
    if (StPlusItem("Variant Shoes", stats))
        stats.def += stats.shoesHandDefPlus;
    if (StPlusItem("0", stats) && n_A_JobSearch(stats.job) == 1)
        stats.def += 6;

    if (StPlusItem("0", stats))
        stats.totalGearDefPlus -= (stats.headDefPlus + stats.shieldHandDefPlus);

    stats.totalDef = stats.def + Math.round(stats.totalGearDefPlus * 7 / 10);

    if (StPlusItem(REDUCE_DEFENSE, stats) + StPlusCard(REDUCE_DEFENSE, stats))
        stats.totalDef = Math.floor(stats.totalDef / StPlusItem(REDUCE_DEFENSE, stats), stats);
    if (StPlusItem(LOWER_DEFENCE_PERCENTAGE, stats) + StPlusCard(LOWER_DEFENCE_PERCENTAGE, stats))
        stats.totalDef -= Math.floor(stats.totalDef * (StPlusItem(LOWER_DEFENCE_PERCENTAGE, stats) + StPlusCard(LOWER_DEFENCE_PERCENTAGE, stats)) / 100, stats);

    if (SkillSearch("Spear Dynamo", stats))
        stats.totalDef = Math.floor(stats.totalDef * (1 - 0.05 * SkillSearch("Spear Dynamo", stats)));


    if (SkillSearch("Mental Strength", stats))
        stats.totalDef = 90;

    if (SkillSearch("Frenzy", stats))
        stats.totalDef = 0;

    // myInnerHtml("A_totalDEF", stats.totalDef, 0);


    stats.vitDEF = new Array();
    stats.vitDEF[0] = Math.floor(stats.vit * 0.5) + Math.floor(stats.vit * 0.3);
    stats.vitDEF[2] = Math.floor(stats.vit * 0.5) + Math.floor(stats.vit * stats.vit / 150) - 1;
    if (stats.vitDEF[2] > stats.vitDEF[0]) {
        stats.vitDEF[1] = (stats.vitDEF[0] + stats.vitDEF[2]) / 2;
    } else {
        stats.vitDEF[1] = stats.vitDEF[0];
        stats.vitDEF[2] = stats.vitDEF[0];
    }
    if (stats.performanceSkills[9]) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] += 2 + 2 * stats.performanceSkills[9];
    }
    if (SkillSearch("Auto Berserk", stats)) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] = Math.floor(stats.vitDEF[i] * 0.45);
    } else {
        if (stats.supportiveSkills[12]) {
            for (i = 0; i <= 2; i++)
                stats.vitDEF[i] = Math.floor(stats.vitDEF[i] * 0.9);
        }
    }
    if (StPlusItem(REDUCE_DEFENSE, stats)) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] = Math.floor(stats.vitDEF[i] / StPlusItem(24), stats);
    }
    if (SkillSearch("Spear Dynamo", stats)) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] = Math.floor(stats.vitDEF[i] * (1 - 0.05 * SkillSearch("Spear Dynamo", stats)));
    }
    if (stats.supportiveSkills[4]) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] = Math.floor(stats.vitDEF[i] * (1 + 0.05 * stats.supportiveSkills[4]));
    }
    if (SkillSearch("Frenzy", stats)) {
        for (i = 0; i <= 2; i++)
            stats.vitDEF[i] = 0;
    }


    let n_A_MDEF = StPlusItem(MDEF, stats);


    n_A_MDEF += StPlusCard(MDEF, stats);

    if (n_A_JobSearch(stats.job) == 3)
        n_A_MDEF += CardNumSearch("Rideword", stats);
    if (stats.shieldHandDefPlus >= 9 && CardNumSearch("Sting", stats))
        n_A_MDEF += 5;
    if (stats.headDefPlus <= 5 && stats.cards[8] == 213)
        n_A_MDEF += 5;
    if (stats.cards[9] == 213)
        n_A_MDEF += 5;
    if (stats.shieldHandDefPlus <= 5 && CardNumSearch("Arcluse", stats))
        n_A_MDEF += 3;
    if (stats.bodyDefPlus <= 5 && CardNumSearch("Goat", stats))
        n_A_MDEF += 5;
    if (stats.shoesHandDefPlus <= 5 && CardNumSearch("Megalith", stats))
        n_A_MDEF += 7;
    if (stats.shoulderDefPlus <= 5 && CardNumSearch("Kappa", stats))
        n_A_MDEF += 8;
    if (StPlusItem("0", stats))
        n_A_MDEF += (stats.headDefPlus + stats.shieldHandDefPlus);

    if (SkillSearch("Spear Dynamo", stats))
        n_A_MDEF += 1;
    if (SkillSearch("Endure", stats))
        n_A_MDEF += SkillSearch("Endure", stats);


    if (SkillSearch("Mental Strength", stats))
        n_A_MDEF = 90;
    if (SkillSearch("Frenzy", stats))
        n_A_MDEF = 0;

    // myInnerHtml("A_MDEF", n_A_MDEF, 0);


    stats.hit = stats.baseLevel + stats.dex;


    stats.hit += StPlusItem(HIT, stats);


    stats.hit += StPlusCard(HIT, stats);

    if (StPlusItem("Jungle Carbine", stats))
        w -= Math.floor(SU_DEX / 3);


    stats.hit += 1 * SkillSearch("Vulture's Eye", stats);
    stats.hit += 2 * SkillSearch("Weaponry Research", stats);
    stats.hit += 3 * SkillSearch("True Sight", stats);

    stats.hit += 10 * SkillSearch("Spear Dynamo", stats);
    stats.hit += 1 * SkillSearch("Snake Eyes", stats);
    if (SkillSearch("Gunslinger's Panic", stats))
        stats.hit -= 30;
    if (SkillSearch("Increase Accuracy", stats))
        stats.hit += 20;
    stats.hit += 2 * SkillSearch("Single Action", stats);

    if (StPlusItem("Western Outlaw", stats))
        stats.hit += Math.floor(SU_AGI / 5);

    if (stats.skillToUseName == "Rapid Smiting")
        stats.hit += 20;

    if (stats.supportiveSkillsBattleChant[4])
        stats.hit += 50;

    if (stats.foodBoxBonus[0])
        stats.hit += 30;


    if (stats.performanceSkills[4])
        stats.hit += stats.performanceSkills[4] + Math.floor(stats.performanceSkills[34] / 2) + Math.floor(stats.performanceSkills[24] / 10);


    // myInnerHtml("A_HIT", stats.hit, 0);


    stats.flee = stats.baseLevel + stats.agi;
    stats.flee += StPlusItem(FLEE, stats);
    stats.flee += StPlusCard(FLEE, stats);

    if (n_A_JobSearch(stats.job) == 2 && CardNumSearch("Wanderer", stats))
        stats.flee += 20;
    if (stats.shoulderDefPlus >= 9 && CardNumSearch("Ninetails", stats))
        stats.flee += 20;
    if (stats.shoulderDefPlus <= 4 && CardNumSearch("Kavach Icarus", stats))
        stats.flee += 10;
    if (stats.shoulderDefPlus >= 9 && CardNumSearch("Orc Baby", stats))
        stats.flee += 5;

    if (stats.groundSupportiveSkills[0] == 2 && stats.groundSupportiveSkills[1] >= 1 && (CardNumSearch("Dokkebi", stats) || stats.equipments[6] == 430 || stats.equipments[6] == 606))
        stats.flee += stats.groundSupportiveSkills[1] * 3;

    if (stats.equipments[0] == 483)
        stats.flee -= (stats.baseLevel + SU_AGI);


    if (stats.job == 8 || stats.job == 14 || stats.job == 22 || stats.job == 28)
        stats.flee += 4 * SkillSearch("Improve Dodge", stats);
    else
        stats.flee += 3 * SkillSearch("Improve Dodge", stats);

    if (SkillSearch("Gunslinger's Panic", stats))
        stats.flee += 30;
    if (SkillSearch("Gatling Fever", stats))
        stats.flee -= 5 * SkillSearch("Gatling Fever", stats);

    let Mikiri = new Array(0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15);
    stats.flee += Mikiri[SkillSearch("Flee", stats)];


    if (stats.job == 24)
        stats.flee += Math.round(SkillSearch("Wind Walk", stats) / 2);
    if (stats.supportiveSkills[9] && SkillSearch("Wind Walk", stats) == 0)
        stats.flee += Math.round(stats.supportiveSkills[9] / 2);


    if (SkillSearch("Close Confine", stats))
        stats.flee += 10;


    if (SkillSearch("Lunar Protection", stats))
        stats.flee += Math.floor((stats.baseLevel + stats.luk + stats.dex) / 10);

    if (stats.supportiveSkillsBattleChant[4])
        stats.flee += 50;

    if (stats.foodBoxBonus[1])
        stats.flee += 30;


    if (stats.performanceSkills[0])
        stats.flee += stats.performanceSkills[0] + Math.floor(stats.performanceSkills[30] / 2) + Math.floor(stats.performanceSkills[20] / 10);

    if (SkillSearch("Frenzy", stats))
        stats.flee /= 2;


    // myInnerHtml("A_FLEE", stats.flee, 0);


    stats.perfectDodge = 1 + stats.luk * 0.1;


    stats.perfectDodge += StPlusItem(PERFECT_DODGE, stats);

    stats.perfectDodge += StPlusCard(PERFECT_DODGE, stats);

    if (n_A_JobSearch(stats.job) == 2)
        stats.perfectDodge += 5 * CardNumSearch("Wild Rose", stats);

    if (n_A_JobSearch(stats.job) == 1)
        stats.perfectDodge += 4 * CardNumSearch("Heater", stats);
    if (stats.shoulderDefPlus <= 4 && CardNumSearch("Kavach Icarus", stats))
        stats.perfectDodge += 1;
    if (stats.equipments[7] == 535) {
        wHPVS = n_A_JobSearch(stats.job);
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5) {
            stats.perfectDodge += 5;
            stats.perfectDodge += stats.shoulderDefPlus * 2;
        }
    }

    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Hahoe Mask", stats))
        stats.perfectDodge += 2;

    stats.perfectDodge = Math.round(stats.perfectDodge * 10) / 10;


    // myInnerHtml("A_LUCKY", stats.perfectDodge, 0);

    stats.crit = 1 + stats.luk * 0.3;

    stats.crit += StPlusItem(CRIT, stats);

    w = 0;
    w += StPlusCard(CRIT, stats);

    w += StPlusCard(CRITICAL_AGAINST_RACE_PERCENTAGE + targetStats.race, stats);

    if (CardNumSearch("Green Maiden", stats))
        w += stats.shoulderDefPlus;
    if (n_A_JobSearch(stats.job) == 2)
        w += 4 * CardNumSearch("Mobster", stats);
    if (n_A_JobSearch(stats.job) == 3) {
        if (targetStats.race == 1 || targetStats.race == 6)
            w += 9 * CardNumSearch("Fur Seal", stats);
    }
    if (SU_LUK >= 80 && CardNumSearch("Giant Whisper", stats))
        w += 3;
    if (StPlusItem("Giant Encyclopedia", stats))
        w += Math.floor(SU_LUK / 5);
    if (StPlusItem("Sniping Suit", stats))
        w += Math.floor(SU_LUK / 5);
    if (StPlusItem("Sniping Suit *", stats))
        w += Math.floor(stats.luk / 5);

    if (StPlusItem("Sabath", stats) && 90 <= targetStats.element)
        w += 50;

    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Bride Mask", stats))
        w += 5;
    if (StPlusItem("Heart Breaker", stats))
        w += weaponRefinementLevel;


    if (stats.weaponType == WEAPON_TYPE_BOW && stats.arrow == 15)
        w += 20;


    if (SkillSearch("Fury", stats))
        w += 7.5 + SkillSearch("Fury", stats) * 2.5;
    if (SkillSearch("Fury (SuperNovice)", stats))
        w += 50;
    if (stats.job == 24)
        w += SkillSearch("True Sight", stats);
    stats.crit += w;

    if (stats.performanceSkills[5])
        stats.crit += 10 + stats.performanceSkills[5] + Math.floor(stats.performanceSkills[35] / 2) + Math.floor(stats.performanceSkills[25] / 10);


    if (stats.weaponType == WEAPON_TYPE_KATAR)
        stats.crit *= 2;

    stats.crit = Math.round(stats.crit * 10) / 10;


    // myInnerHtml("A_CRI", stats.crit, 0);


    let n_A_MATK = [0, 0, 0];

    w = Math.floor(stats.int / 7);
    n_A_MATK[0] = stats.int + w * w;


    w = Math.floor(stats.int / 5);
    n_A_MATK[2] = stats.int + w * w;

    let w_MATK = 100;

    w_MATK += StPlusItem(MATK_PERCENTAGE, stats);

    if (weaponRefinementLevel >= 9 && StPlusItem("Lich's Bone Wand", stats))
        w_MATK += 3;
    if (StPlusItem("Staff of Destruction", stats))
        w_MATK += Math.floor(weaponRefinementLevel / 2);
    if (StPlusItem("0", stats) || EquipNumSearch("0", stats))
        w_MATK += weaponRefinementLevel;
    if (stats.groundSupportiveSkills[2])
        w_MATK += 10;

    if (n_A_JobSearch(stats.job) == 5 && CardNumSearch("0", stats))
        w_MATK += 3;
    if (stats.headDefPlus >= 9 && stats.cards[8] == 177)
        w_MATK += 2;
    if (stats.equipments[0] == 484 && SU_INT >= 70)
        w_MATK += 5;
    n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
    n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);

    if (stats.foodBoxBonus[2]) {
        n_A_MATK[0] += 10;
        n_A_MATK[2] += 10;
    }
    if (stats.foodBoxBonus[10]) {
        n_A_MATK[0] += 20;
        n_A_MATK[2] += 20;
    }

    w_MATK = 100;

    w_MATK += StPlusItem(MATK_BASED_ON_STAFF_PERCENTAGE, stats);

    n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
    n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);


    // myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);

    if (SkillSearch("Mystical Amplification", stats)) {
        AmpMinMatkBK = n_A_MATK[0];
        AmpMaxMatkBK = n_A_MATK[2];
        n_A_MATK[0] = Math.floor(n_A_MATK[0] * (1 + 0.05 * SkillSearch("Mystical Amplification", stats)));
        n_A_MATK[2] = Math.floor(n_A_MATK[2] * (1 + 0.05 * SkillSearch("Mystical Amplification", stats)));

        myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);
        if (stats.skillToUseName == "Stave Crasher") {
            n_A_MATK[0] = AmpMinMatkBK;
            n_A_MATK[2] = AmpMaxMatkBK;
        }
    }


    if (n_A_MATK[0] != n_A_MATK[2])
        n_A_MATK[2] -= 1;

    n_A_MATK[1] = (n_A_MATK[2] + n_A_MATK[0]) / 2;

    let wASPD;

    if (hasLeftHand == 1)
        wASPD = (200 - (JobASPD[stats.job][stats.weaponType] + JobASPD[stats.job][stats.weapon2IndexType]) / 2) * 1.4;
    else
        wASPD = 200 - JobASPD[stats.job][stats.weaponType];


    if (hasLeftHand == 1 && stats.weaponType == WEAPON_TYPE_UNARMED && stats.weapon2IndexType != 0)
        wASPD = 200 - JobASPD[stats.job][stats.weapon2IndexType];

    stats.aspd = 200 - wASPD + (Math.floor(wASPD * stats.agi * 4 / 100) + Math.floor(wASPD * stats.dex / 100)) / 10;

    if (stats.equipments[0] == 47)
        stats.aspd += 2;


    if (SkillSearch("Cavalier Mastery", stats) && (stats.skillToUseName == "Basic Attack" || stats.skillToUseName == "Martyr's Reconing", stats))
        stats.aspd -= (6 - SkillSearch("Cavalier Mastery", stats)) * 10;

    stats.aspd += Math.round(SkillSearch("Single Action", stats) / 2);


    w = 0;
    let ASPDch = 0;
    if (stats.weaponType == WEAPON_TYPE_TWO_HANDED_SWORD && SkillSearch("Twohand Quicken", stats)) {
        w += 30;
        ASPDch = 1;
    }
    if (stats.weaponType == WEAPON_TYPE_SWORD && SkillSearch("One Hand Quicken (Soul Linked)", stats)) {
        w += 30;
        ASPDch = 1;
    }
    if (6 <= stats.weaponType && stats.weaponType <= 8 && SkillSearch("Andrenaline Rush", stats)) {
        w += 30;
        ASPDch = 1;
    }
    if (ASPDch == 0 && SkillSearch("Full Andrenaline Rush", stats)) {
        w += 30;
        ASPDch = 1;
    }
    if (stats.weaponType == WEAPON_TYPE_TWO_HANDED_SPEAR && SkillSearch("Spear Quicken", stats)) {
        w += SkillSearch("Spear Quicken", stats) + 20;
        ASPDch = 1;
    }
    if (StPlusItem("Western Outlaw", stats))
        w += Math.floor(SU_AGI / 5);
    if (stats.equipments[0] == 484 && SU_STR >= 50)
        w += 5;
    if (SU_STR >= 95 && StPlusItem("Doom Slayer", stats))
        w -= 40;
    if (StPlusItem("Hurricane Fury", stats))
        w += (weaponRefinementLevel * 2);
    if (StPlusItem("Book of the Dead", stats))
        w += weaponRefinementLevel;
    if (SkillSearch("Frenzy", stats))
        w += 30;
    if (SkillSearch("Last Stand", stats))
        w += 20;
    if (SkillSearch("Gatling Fever", stats))
        w += 2 * SkillSearch("Gatling Fever", stats);

    if (SkillSearch("Stellar Protection", stats)) {
        ASPDch = 1;
        w += Math.floor((stats.baseLevel + stats.luk + stats.dex) / 10);
    }

    if (SkillSearch("Solar, Lunar, and Stellar Shadow", stats)) {
        ASPDch = 1;
        w += 3 * SkillSearch("Solar, Lunar, and Stellar Shadow", stats);
    }
    if (ASPDch == 0 && stats.weaponType != WEAPON_TYPE_BOW && stats.supportiveSkills[6] == 2) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= stats.weaponType && stats.weaponType <= 8 && stats.supportiveSkills[6] == 1) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= stats.weaponType && stats.weaponType <= 8 && stats.supportiveSkills[6] == 3) {
        w += 30;
        ASPDch = 1;
    }
    if (stats.performanceSkills[1] && stats.weaponType != WEAPON_TYPE_BOW && ASPDch == 0)
        w += 5 + stats.performanceSkills[1] + Math.floor(stats.performanceSkills[31] / 2) + Math.floor(stats.performanceSkills[21] / 20);


    w += StPlusItem(ASPD_PERCENTAGE, stats);
    w += StPlusCard(ASPD_PERCENTAGE, stats);


    if (SkillSearch("Mental Strength", stats))
        w -= 25;


    if (stats.speedPotion || SkillSearch("Deadly Poison (Consumed)", stats)) {
        if (SkillSearch("Deadly Poison (Consumed)", stats) == 0) {
            if (stats.speedPotion == 1)
                w += 10;
            else if (stats.speedPotion == 2)
                w += 15;
            else if (stats.speedPotion == 3)
                w += 20;
        } else
            w += 20;
    }
    stats.aspd += (200 - stats.aspd) * (w / 100);

    if (stats.weaponType == WEAPON_TYPE_BOOK && SkillSearch("Study", stats))
        stats.aspd += (200 - stats.aspd - (SkillSearch("Study", stats) * 5 / 10)) * ((SkillSearch("Study", stats) * 5 / 10) / 100);


    if (SkillSearch("Defending Aura", stats))
        stats.aspd -= (25 - SkillSearch("Defending Aura", stats) * 5);

    if (stats.aspd > 190)
        stats.aspd = 190;


    stats.aspd *= 100;
    stats.aspd = Math.round(stats.aspd);
    stats.aspd /= 100;


    // myInnerHtml("A_ASPD", stats.aspd, 0);

    stats.aspd = (200 - stats.aspd) / 50;
    let n_A_ATK_w, n_A_ATK;
    if (isNonRangeWeapon(stats.weaponType)) {
        n_A_ATK_w = Math.round(Math.floor(stats.str / 10) * Math.floor(stats.str / 10));
        n_A_ATK = stats.str + n_A_ATK_w + Math.floor(stats.dex / 5) + Math.floor(stats.luk / 5);
    } else {
        n_A_ATK_w = Math.round(Math.floor(stats.dex / 10) * Math.floor(stats.dex / 10));
        n_A_ATK = stats.dex + n_A_ATK_w + Math.floor(stats.str / 5) + Math.floor(stats.luk / 5)
    }
    let impositioMagnus = stats.supportiveSkills[2] * 5;
    let ATK_LEFT = Math.floor(impositioMagnus + stats.weapon1ATK + stats.weapon2_ATK + n_A_ATK);
    let ATK_RIGHT = Math.floor(stats.weapon1LV_upgradeBonusATK + stats.weapon2LV_upgradeBonusATK);
    // myInnerHtml("A_ATK_2", ATK_LEFT + "+" + ATK_RIGHT, 0),


    if (SkillSearch("Raging Trifecta Blow", stats)) {
        w = 100 / (30 - SkillSearch("Raging Trifecta Blow", stats));
        stats.aspd += (stats.aspd - (1000 - stats.agi * 4 - stats.dex * 2) / 1000) / w;
        if (SkillSearch("<Font size=2>Add the delay time when attacking for triple attack</Font>", stats))
            stats.aspd += (0.3 / w);
    }


    stats.cast = 1 - stats.dex / 150;
    if (stats.cast < 0)
        stats.cast = 0;


    w = 100;
    if (n_A_JobSearch(stats.job) == 5 && CardNumSearch("0", stats))
        w -= 15;
    if ((stats.job == 18 || stats.job == 32) && CardNumSearch("0", stats))
        w -= 15;
    if (StPlusItem("0", stats) || EquipNumSearch("0", stats))
        w -= weaponRefinementLevel;
    if (stats.cards[8] == 177)
        w -= stats.headDefPlus;

    w += StPlusItem(CAST_TIME_PERCENTAGE, stats);
    w += StPlusCard(CAST_TIME_PERCENTAGE, stats);

    stats.cast *= w / 100;

    if (stats.supportiveSkills[13])
        stats.cast *= (100 - 15 * stats.supportiveSkills[13]) / 100;
    if (SkillSearch("Forsight", stats))
        stats.cast = stats.cast / 2;


    let n_A_HPR = Math.floor(stats.vit / 5) + Math.floor(stats.maxHp / 200);
    if (n_A_HPR < 1)
        n_A_HPR = 1;
    w = 100;
    w += StPlusItem(HP_REGEN_PERCENTAGE, stats);
    w += StPlusCard(HP_REGEN_PERCENTAGE, stats);
    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling", stats);

    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Magistrate Hat", stats))
        w += 3;
    if (stats.shoesHandDefPlus <= 4 && CardNumSearch("Gold Acidus", stats))
        w += 5;

    n_A_HPR = Math.floor(n_A_HPR * w / 100);
    // myInnerHtml("A_HPR", n_A_HPR, 0);


    let n_A_SPR = Math.floor(stats.int / 6) + Math.floor(stats.maxSp / 100) + 1;

    w = 100;

    w += SkillSearch("Mediatio", stats) * 3;

    w += StPlusItem(SP_REGEN_PERCENTAGE, stats);
    w += StPlusCard(SP_REGEN_PERCENTAGE, stats);

    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling", stats);

    if (n_A_JobSearch(stats.job) == 41 && StPlusItem("Ayam", stats))
        w += 3;
    if (stats.shieldHandDefPlus <= 4 && stats.cards[8] == 179)
        w += 5;
    if (stats.cards[9] == 179)
        w += 5;
    if (stats.shoesHandDefPlus <= 4 && CardNumSearch("Gold Acidus", stats))
        w += 5;

    n_A_SPR = Math.floor(n_A_SPR * w / 100);

    if (stats.int >= 120)
        n_A_SPR += Math.floor((stats.int - 120) / 2) + 4;

    // myInnerHtml("A_SPR", n_A_SPR, 0);

    // KakutyouKansuu();
    return stats;
}


function StPlusItem(nSTP2, stats) {
    let wSTP2 = 0;
    for (let STP2i = 0; STP2i <= 20; STP2i++) {
        for (let STP2j = 0; ItemOBJ[stats.equipments[STP2i]][STP2j + 11] != 0; STP2j += 2) {
            if (nSTP2 == ItemOBJ[stats.equipments[STP2i]][STP2j + 11])
                wSTP2 += ItemOBJ[stats.equipments[STP2i]][STP2j + 12];
        }
    }
    return wSTP2;
}


function StPlusCard(nSTP2, stats) {
    let wSTP2 = 0;
    for (let STP2i = 0; STP2i <= 25; STP2i++) {
        for (let STP2j = 0; cardOBJ[stats.cards[STP2i]][STP2j + 4] != 0; STP2j += 2) {
            if (nSTP2 == cardOBJ[stats.cards[STP2i]][STP2j + 4])
                wSTP2 += cardOBJ[stats.cards[STP2i]][STP2j + 5];
        }
    }
    return wSTP2;
}

function CardNumSearch(nCNS, stats) {
    let wCNS = 0;
    for (let CNSi = 0; CNSi <= 25; CNSi++) {
        let cardName = cardOBJ[stats.cards[CNSi]][2];
        if (nCNS === cardName)
            wCNS += 1;
    }
    return wCNS;
}

function EquipNumSearch(nENS, stats) {
    let wENS = 0;
    for (let ENSi = 0; ENSi <= 20; ENSi++) {
        let itemName = ItemOBJ[stats.equipments[ENSi]][8];
        if (nENS === itemName)
            wENS += 1;
    }
    return wENS;
}


function n_A_JobSet(FORM_DATA) {
    let job = eval(FORM_DATA.A_JOB);
    let isRebirth = 0;
    if (21 <= job && job <= 40) {
        let isRebirth = 1;
        if (34 <= job && job <= 40)
            job -= 34;
    }
    return {job, isRebirth}
}

function SetEquip(stats) {
    const start = Date.now();

    for (let SEi = 11; SEi <= 20; SEi++)
        stats.equipments[SEi] = 736;

    let w_SE_num = 11;
    let w_SE_ch = 0;
    for (let SEk = 0; SEk <= SE_MAXnum; SEk++) {
        for (let SEj = 1; w_SE[SEk][SEj] != "NULL" && (w_SE_ch == 1 || (w_SE_ch == 0 && SEj == 1)); SEj++) {
            w_SE_ch = 0;
            for (let SEi = 0; SEi <= 10 && w_SE_ch == 0; SEi++) {
                if (stats.equipments[SEi] == w_SE[SEk][SEj])
                    w_SE_ch = 1;
            }
        }
        if (w_SE_ch == 1) {
            stats.equipments[w_SE_num] = w_SE[SEk][0];
            w_SE_num++;
        }
    }
}

function SetCard(stats) {
    for (let SEi = 16; SEi <= 25; SEi++)
        stats.cards[SEi] = 0;

    let w_SE_num = 16;
    let w_SE_ch = 0;
    for (let SEk = 0; SEk <= SC_MAXnum; SEk++) {
        for (let SEj = 1; w_SC[SEk][SEj] != "NULL" && (w_SE_ch == 1 || (w_SE_ch == 0 && SEj == 1)); SEj++) {
            w_SE_ch = 0;
            for (let SEi = 0; SEi <= 15 && w_SE_ch == 0; SEi++) {
                if (stats.cards[SEi] == w_SC[SEk][SEj])
                    w_SE_ch = 1;
            }
        }
        if (w_SE_ch == 1) {
            stats.cards[w_SE_num] = w_SC[SEk][0];
            w_SE_num++;
        }
    }
}

function SkillSearch(n, stats) {
    for (let k = 0; k <= 14; k++) {
        let passiveSkillToUseName;
        if (JobSkillPassOBJ[stats.job][k] != 999)
            passiveSkillToUseName = SkillOBJ[JobSkillPassOBJ[stats.job][k]][2];
        if (passiveSkillToUseName === n) {
            return stats.passiveSkills[k];
        }
    }
    return 0;
}


function tPlusDamCut(stats, targetStats, wPDC, InWarOfEmperium) {
    if (wBTw1 == 0) {
        if (targetStats.lexAeterna && wLAch == 0)//LA
            wPDC *= 2;
        if (targetStats.fiberLock && stats.weapon1Element == 3)//ƒXƒpƒCƒ_ƒEƒFƒu
            wPDC *= 2;
        let baizok = [110, 114, 117, 119, 120];
        if (stats.groundSupportiveSkills[0] == 0 && stats.groundSupportiveSkills[1] >= 1 && stats.weapon1Element == 3)//‰Î
            wPDC = Math.floor(wPDC * baizok[stats.groundSupportiveSkills[1] - 1] / 100);
        if (stats.groundSupportiveSkills[0] == 1 && stats.groundSupportiveSkills[1] >= 1 && stats.weapon1Element == 1)//…
            wPDC = Math.floor(wPDC * baizok[stats.groundSupportiveSkills[1] - 1] / 100);
        if (stats.groundSupportiveSkills[0] == 2 && stats.groundSupportiveSkills[1] >= 1 && stats.weapon1Element == 4)//•—
            wPDC = Math.floor(wPDC * baizok[stats.groundSupportiveSkills[1] - 1] / 100);
    }

    return wPDC;
}

function tPlusEnemyClick(InWarOfEmperium) {
    if (InWarOfEmperium) {
        n_B = new Array();
        for (i = 0; i <= 26; i++)
            n_B[i] = MonsterOBJ[document.calcForm.B_Enemy.value][i];

        document.calcForm.B_LV.value = n_B[5];
        document.calcForm.B_AGI.value = n_B[8];
        document.calcForm.B_VIT.value = n_B[7];
        document.calcForm.B_INT.value = n_B[9];
        document.calcForm.B_LUK.value = n_B[11];
        document.calcForm.B_DEF.value = n_B[14];
        document.calcForm.B_MDEF.value = n_B[15];
    }
}

function tPlusTaiseiSyokia() {
}

function tPlusLucky(wPL, InWarOfEmperium) {
    if (InWarOfEmperium) {
        w = eval(document.calcForm.B_TAISEI6.value);
        w += (targetStatsArray[TARGET_STAT_LUK] / 10);

        w = wPL * (100 - w) / 100;
        return w;
    } else {
        return wPL;
    }
}

function tPlusAG(InWarOfEmperium) {
    if (InWarOfEmperium) {
        if (n_Enekyori != 2) {
            wPAG = w_AG[eval(document.calcForm.B_TAISEI10.value)];
            w_Maxatk *= (wPAG / 100);
            w_Minatk *= (wPAG / 100);
            w_Aveatk *= (wPAG / 100);
        }
    }
}


// TODO rename
function n_A_JobSearch(stats) {

    if (stats.job <= 6)
        return stats.job;
    if (stats.job == 20)
        return 0;
    if (stats.job == 7 || stats.job == 13 || stats.job == 21 || stats.job == 27)
        return 1;
    if (stats.job == 8 || stats.job == 14 || stats.job == 22 || stats.job == 28)
        return 2;
    if (stats.job == 9 || stats.job == 15 || stats.job == 23 || stats.job == 29)
        return 3;
    if (stats.job == 10 || stats.job == 16 || stats.job == 17 || stats.job == 24 || stats.job == 30 || stats.job == 31)
        return 4;
    if (stats.job == 11 || stats.job == 18 || stats.job == 25 || stats.job == 32)
        return 5;
    if (stats.job == 12 || stats.job == 19 || stats.job == 26 || stats.job == 33)
        return 6;
    if (stats.job == 41 || stats.job == 42 || stats.job == 43)
        return 41;
    return 7;
}


function isNonRangeWeapon(weaponType) {
    return weaponType != WEAPON_TYPE_BOW && weaponType != WEAPON_TYPE_INSTRUMENT && weaponType != WEAPON_TYPE_WHIP && weaponType != WEAPON_TYPE_HANDGUN && weaponType != WEAPON_TYPE_RIFLE && weaponType != WEAPON_TYPE_SHOTGUN && weaponType != WEAPON_TYPE_GATLING_GUN && weaponType != WEAPON_TYPE_GRENADE_LAUNCHER;
}

function isRangedWeapon(weaponType) {
    return !(isNonRangeWeapon(weaponType));
}


function CalculateEnemyStats(FORM_DATA, InWarOfEmperium) {
    // TARGET_STAT_MOB_INDEX = 0;
    // TARGET_STAT_RACE = 2;
    // TARGET_STAT_ELEMENT = 3;
    // TARGET_STAT_SIZE = 4;
    // TARGET_STAT_LEVEL = 5;
    // TARGET_STAT_HP = 6;
    // TARGET_STAT_VIT = 7;
    // TARGET_STAT_AGI = 8;
    // TARGET_STAT_INT = 9;
    // TARGET_STAT_DEX = 10;
    // TARGET_STAT_LUK = 11;
    // TARGET_STAT_ATK = 12;
    // TARGET_STAT_ATK2 = 13;
    // TARGET_STAT_DEF = 14;
    // TARGET_STAT_MDEF = 15;
    // TARGET_STAT_MDEF = 23;
    // TARGET_STAT_MDEF = 24;
    // TARGET_STAT_MDEF = 25;
    // TARGET_STAT_MDEF = 26;
    // TARGET_STAT_MDEF = 27;

    let mob = MonsterOBJ[eval(FORM_DATA.B_Enemy)];
    let targetStats = {
        mobIndex: mob[0],
        race: mob[2],
        element: mob[3],
        size: mob[4],
        level: mob[5],
        hp: mob[6],
        vit: mob[7],
        agi: mob[8],
        int: mob[9],
        dex: mob[10],
        luk: mob[11],
        atk: mob[12],
        atk2: mob[13],
        def: mob[14],
        mdef: mob[15],
        exp: mob[16],
        jobExp: mob[17],
        isMvp: mob[19] == 1,
        isStaticPlant: mob[19] == 5,
        isNormal: mob[19] == 0,
        rangeAttack: mob[20],
        flee: 0,
        hit: 0,
        mdef2: 0,
        def2Min: 0,
        def2Avg: 0,
        def2Max: 0,
    };

    let def2Min, def2Max, def2Avg, mdef2, hit, flee;
    if (InWarOfEmperium) {
        targetStats.element = eval(FORM_DATA.B_element);
        targetStats.level = eval(FORM_DATA.B_LV);
        targetStats.vit = eval(FORM_DATA.B_VIT);
        targetStats.agi = eval(FORM_DATA.B_AGI);
        targetStats.int = eval(FORM_DATA.B_INT);
        targetStats.luk = eval(FORM_DATA.B_LUK);
        targetStats.def = eval(FORM_DATA.B_DEF);
        targetStats.mdef = eval(FORM_DATA.B_MDEF);

        let JobHP_A = new Array(0, 0.7, 0.5, 0.4, 0.5, 0.3, 0.4, 1.5, 1.1, 0.75, 0.85, 0.55, 0.9, 1.1, 0.85, 0.9, 0.75, 0.75, 0.75, 0.9, 0, 1.5, 1.1, 0.75, 0.85, 0.55, 0.9, 1.1, 0.85, 0.9, 0.75, 0.75, 0.75, 0.9);
        let JobHP_B = new Array(5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5);


        let w = 0;
        for (let i = 2; i <= targetStats.level; i++)
            w += Math.round(JobHP_A[targetStatsArray[1]] * i);
        w = (JobHP_B[targetStatsArray[1]] * targetStats.level) + 35 + w;

        if (targetStatsArray[1] > 20)
            w = Math.floor(w * 125 / 100);
        targetStats.hp = Math.floor(w * (100 + targetStats.vit) / 100);
        targetStats.hp += eval(document.calcForm.B_TAISEI11.value);
        targetStats.hp = Math.floor(targetStats.hp * (100 + eval(document.calcForm.B_TAISEI12.value)) / 100);
        myInnerHtml("B_HP", targetStats.hp, 0);


        n_B_DEF2[2] = Math.floor(targetStats.vit * 0.5) + Math.floor(targetStats.vit * 0.3);
        n_B_DEF2[0] = Math.floor(targetStats.vit * 0.5) + Math.floor(targetStats.vit * targetStats.vit / 150) - 1;
        if (n_B_DEF2[2] > n_B_DEF2[0])
            n_B_DEF2[0] = n_B_DEF2[2];
        w = eval(document.calcForm.B_TAISEI4.value);
        if (w) {
            n_B_DEF2[2] *= (1 + 0.05 * w);
            n_B_DEF2[0] *= (1 + 0.05 * w);
        }
        n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2);
    } else {

        def2Min = targetStats.vit;
        def2Max = targetStats.vit + (Math.floor(targetStats.vit / 20) * Math.floor(targetStats.vit / 20) - 1);
        if (def2Min > def2Max)
            def2Max = def2Min;
    }
    mdef2 = Math.floor(targetStats.vit / 2) + targetStats.vit;
    hit = targetStats.level + targetStats.dex;
    flee = targetStats.level + targetStats.agi;
    let targetStatusFlag = {
        provoke: 0,
        quagmire: 0,
        poison: false,
        blind: false,
        frozen: false,
        blessing: false,
        lexAerterna: false,
        stun: false,
        sleep: false,
        stone: false,
        curse: false,
        agilityDown: eval(FORM_DATA.TargetStatusFlag10),
        signumCrucis: eval(FORM_DATA.TargetStatusFlag11),
        divestWeapon: false,
        divestWeapon: false,
        divestShield: false,
        divestArmor: false,
        divestHelm: false,
        fiberLock: false,
        mindBreaker: 0,
        slowGrace: false,
        downTtempo: false,
        powerUp: false,
        agilityUp: false,
        eska: false,
        eske: false,
        monsterChangeElement: false,
        sageChangeElement: false,
        flying: 0,
    };
    if (MonsterStats) {
        targetStatusFlag = {
            provoke: eval(FORM_DATA.TargetStatusFlag0),
            quagmire: eval(FORM_DATA.TargetStatusFlag1),
            poison: FORM_DATA.TargetStatusFlag2 == "on",
            blind: FORM_DATA.TargetStatusFlag3 == "on",
            frozen: FORM_DATA.TargetStatusFlag4 == "on",
            blessing: FORM_DATA.TargetStatusFlag5 == "on",
            lexAerterna: FORM_DATA.TargetStatusFlag6 == "on",
            stun: FORM_DATA.TargetStatusFlag7 == "on",
            sleep: FORM_DATA.TargetStatusFlag8 == "on",
            stone: FORM_DATA.TargetStatusFlag9 == "on",
            curse: FORM_DATA.TargetStatusFlag10 == "on",
            agilityDown: eval(FORM_DATA.TargetStatusFlag10),
            signumCrucis: eval(FORM_DATA.TargetStatusFlag11),
            divestWeapon: FORM_DATA.TargetStatusFlag12 == "on",
            divestWeapon: FORM_DATA.TargetStatusFlag13 == "on",
            divestShield: FORM_DATA.TargetStatusFlag14 == "on",
            divestArmor: FORM_DATA.TargetStatusFlag15 == "on",
            divestHelm: FORM_DATA.TargetStatusFlag16 == "on",
            fiberLock: FORM_DATA.TargetStatusFlag17 == "on",
            mindBreaker: eval(FORM_DATA.TargetStatusFlag18),
            slowGrace: FORM_DATA.TargetStatusFlag19 == "on",
            downTtempo: FORM_DATA.TargetStatusFlag20 == "on",
            powerUp: FORM_DATA.TargetStatusFlag21 == "on",
            agilityUp: FORM_DATA.TargetStatusFlag22 == "on",
            eska: FORM_DATA.TargetStatusFlag23 == "on",
            eske: FORM_DATA.TargetStatusFlag24 == "on",
            monsterChangeElement: eval(FORM_DATA.TargetStatusFlag25),
            sageChangeElement: eval(FORM_DATA.TargetStatusFlag26),
            flying: eval(FORM_DATA.TargetStatusFlag27),
        };
    }

    if (targetStatusFlag.monsterChangeElement)
        targetStats.element = targetStatusFlag.monsterChangeElement
    if (targetStatusFlag.sageChangeElement)
        targetStats.element = targetStatusFlag.sageChangeElement * 10 + (targetStats.element % 10);

    if (targetStatusFlag.powerUp) {
        targetStats.atk1 = targetStats.atk1 * 3;
        targetStats.atk2 = targetStats.atk2 * 3;
        targetStats.dex = targetStats.dex * 3;
    }
    if (targetStatusFlag.agilityUp)
        targetStats.agi = targetStats.agi * 3;

    if (targetStatusFlag.quagmire) {
        if (InWarOfEmperium) {
            w2 = targetStatusFlag.quagmire * 5;
            w = Math.floor(targetStats.agi / 4);
        } else {
            w2 = targetStatusFlag.quagmire * 10
            w = Math.floor(targetStats.agi / 2);
        }
        if (w > w2)
            targetStats.agi -= w2;
        else
            targetStats.agi -= w;
        if (InWarOfEmperium)
            w = Math.floor(targetStats.dex / 4);
        else
            w = Math.floor(targetStats.dex / 2);
        if (w > w2)
            targetStats.dex -= w2;
        else
            targetStats.dex -= w;
    }

    if (targetStats.isNormal) {
        if (targetStatusFlag.frozen && (targetStats.race == 6 || targetStats.element >= 90)) {
            targetStats.dex = targetStats.dex - Math.floor(targetStats.dex / 2);
            targetStats.int = targetStats.int - Math.floor(targetStats.int / 2);
        }
        if (targetStatusFlag.curse) {
            targetStats.luk = 0;
            targetStats.atk1 -= Math.floor(targetStats.atk1 * 25 / 100);
            targetStats.atk2 -= Math.floor(targetStats.atk2 * 25 / 100);
        }
        if (targetStatusFlag.agilityDown) {
            targetStats.agi -= (targetStatusFlag.agilityDown + 2);
            if (targetStats.agi < 0)
                targetStats.agi = 0;
        }
    }

    if (targetStatusFlag.divestArmor && InWarOfEmperium == 0)
        targetStats.vit -= Math.floor(targetStats.vit * 40 / 100);
    if (targetStatusFlag.divestHelm && InWarOfEmperium == 0)
        targetStats.int -= Math.floor(targetStats.int * 40 / 100);

    if (InWarOfEmperium == 0) {

        targetStats.def2Min = targetStats.vit;
        targetStats.def2Max = targetStats.vit + (Math.floor(targetStats.vit / 20) * Math.floor(targetStats.vit / 20) - 1);
        if (targetStats.def2Min > targetStats.def2Max)
            targetStats.def2Max = targetStats.def2Min;
    }
    targetStats.mdef2 = Math.floor(targetStats.vit / 2) + targetStats.int;
    targetStats.hit = targetStats.level + targetStats.dex;
    targetStats.flee = targetStats.level + targetStats.agi;

    let xiATK = 0;
    let xiDEF = 0;
    if (targetStats.isNormal) {
        if (targetStatusFlag.provoke != 0 && targetStats.element < 90) {
            xiDEF += 5 + targetStatusFlag.provoke * 5;
            xiATK += 2 + targetStatusFlag.provoke * 3;
        }
        if (targetStatusFlag.poison) {
            targetStats.def -= Math.floor(targetStats.def * 25 / 100);
            targetStats.def2Min -= Math.floor(targetStats.def2Min * 25 / 100);
            targetStats.def2Max -= Math.floor(targetStats.def2Max * 25 / 100);
        }
        if (targetStatusFlag.blind) {
            targetStats.hit -= 25;
            if (targetStats.hit < 1)
                targetStats.hit = 1;
            targetStats.flee -= Math.floor(targetStats.flee * 25 / 100);
        }
    }
    if (InWarOfEmperium == 0) {
        if (targetStatusFlag.eske) {
            xiDEF += 50;
            xiATK += 300;
        }
        if (targetStatusFlag.flying)
            xiDEF += 5 * targetStatusFlag.flying;
    }
    if (xiDEF > 100)
        xiDEF = 100;
    if (InWarOfEmperium == 0)
        targetStats.def -= Math.floor(targetStats.def * xiDEF / 100);
    targetStats.def2Min -= Math.floor(targetStats.def2Min * xiDEF / 100);
    targetStats.def2Max -= Math.floor(targetStats.def2Max * xiDEF / 100);
    targetStats.atk1 += Math.floor(targetStats.atk1 * xiATK / 100);
    targetStats.atk2 += Math.floor(targetStats.atk2 * xiATK / 100);

    if (targetStatusFlag.divestWeapon && InWarOfEmperium == 0) {
        targetStats.atk1 -= Math.floor(targetStats.atk1 * 25 / 100);
        targetStats.atk2 -= Math.floor(targetStats.atk2 * 25 / 100);
    }
    if (targetStatusFlag.divestShield && InWarOfEmperium == 0)
        targetStats.def -= Math.floor(targetStats.def * 15 / 100);
    if (targetStats.isNormal) {
        if (targetStatusFlag.fiberLock) {
            targetStats.flee -= 50;
            if (targetStats.flee < 1)
                targetStats.flee = 1;
        }

        if (targetStatusFlag.mindBreaker && targetStats.element < 90)
            targetStats.mdef2 -= Math.floor(targetStats.mdef2 * (targetStatusFlag.mindBreaker * 12) / 100);
        if (targetStatusFlag.frozen && targetStats.race != 1) {
            targetStats.element = 11;
            targetStats.def -= Math.floor(targetStats.def * 50 / 100);
            targetStats.def2Min -= Math.floor(targetStats.def2Min * 50 / 100);
            targetStats.def2Max -= Math.floor(targetStats.def2Max * 50 / 100);
            targetStats.mdef += Math.floor(targetStats.mdef * 25 / 100);
            targetStats.flee = -19;
        }
        if (targetStatusFlag.stun || targetStatusFlag.sleep)
            targetStats.flee = -19;
        if (targetStatusFlag.stone && targetStats.race != 1) {
            targetStats.element = 21;
            targetStats.def -= Math.floor(targetStats.def * 50 / 100);
            targetStats.def2Min -= Math.floor(targetStats.def2Min * 50 / 100);
            targetStats.def2Max -= Math.floor(targetStats.def2Max * 50 / 100);
            targetStats.mdef += Math.floor(targetStats.mdef * 25 / 100);
            targetStats.flee = -19;
        }
    }

    if (InWarOfEmperium == 0) {
        if (targetStatusFlag.eska) {
            targetStats.def2Max += 90;
            targetStats.mdef2 = 90;
        }
    }
    if (targetStatusFlag.downTempo) {
        targetStats.def = 0;
        targetStats.def2Min = 0;
        targetStats.def2Max = 0;
    }
    if (targetStatusFlag.signumCrucis && (targetStats.race == 6 || targetStats.element >= 90))
        targetStats.def -= Math.floor(targetStats.def * (10 + targetStatusFlag.signumCrucis * 4) / 100);


    // TODO move exp reward somewhere else
    // if (InWarOfEmperium == 0) {
    //     let w1_Exp = StPlusCard(120 + targetStats.race, stats);
    //     w1_Exp += StPlusItem(120 + targetStats.race, stats);
    //     if (n_A_JobSearch() == 3 && CardNumSearch(452) && (targetStats.race == 1 || targetStats.race == 6))
    //         w1_Exp += 5;
    //     if (targetStats.race == 2 && n_A_JobSearch() == 4 && CardNumSearch(453))
    //         w1_Exp += 5;
    //     if (w1_Exp != 0) {
    //         targetStats.exp = Math.floor(targetStats.exp * (100 + w1_Exp) / 100);
    //         targetStats.jobExp = Math.floor(targetStats.jobExp * (100 + w1_Exp) / 100);
    //     }
    //     if (targetStats.isNormal) {
    //         let mrKingRichManSong = null;
    //         if (n_Skill3SW)
    //             mrKingRichManSong = eval(FORM_DATA.A_PERFORMANCE_SKILL8);
    //         if (mrKingRichManSong) {
    //             targetStats.exp = Math.floor(targetStats.exp * (125 + 11 * mrKingRichManSong) / 100);
    //             targetStats.jobExp = Math.floor(targetStats.jobExp * (125 + 11 * mrKingRichManSong) / 100);
    //         }
    //     }
    // }

    if (InWarOfEmperium == 0) {
        // myInnerHtml("B_AA", " + ", 0);
        // myInnerHtml("B_AB", " + ", 0);
        let wIJ = [6, 12, 13, 21, 22, 14, 15, 23, 25];
        let wIJ2 = [16, 17];
        let wFront = "<Font color='BLUE'><B>";
        let wFront2 = "<Font color='RED'><B>";
        let wBack = "</B></Font>";

        // for (i = 0; i <= 8; i++) {
        //     wIJstr = targetStatsArray[wIJ[i]];
        //     if (targetStatsArray[wIJ[i]] < n_B2[wIJ[i]])
        //         wIJstr = wFront + targetStatsArray[wIJ[i]] + wBack;
        //     if (targetStatsArray[wIJ[i]] > n_B2[wIJ[i]])
        //         wIJstr = wFront2 + targetStatsArray[wIJ[i]] + wBack;
        //     myInnerHtml("B_" + wIJ[i], wIJstr, 0);
        // }
        // for (i = 0; i <= 1; i++) {
        //     wIJstr = targetStatsArray[wIJ2[i]];
        //     if (targetStatsArray[wIJ2[i]] < n_B2[wIJ2[i]])
        //         wIJstr = wFront2 + targetStatsArray[wIJ2[i]] + wBack;
        //     if (targetStatsArray[wIJ2[i]] > n_B2[wIJ2[i]])
        //         wIJstr = wFront + targetStatsArray[wIJ2[i]] + wBack;
        //     myInnerHtml("B_" + wIJ2[i], wIJstr, 0);
        // }
        //
        // myInnerHtml("B_2", RaceOBJ[targetStats.race], 0);
        // w = Math.floor(targetStats.element / 10);
        // if (targetStats.element != n_B2[3])
        //     myInnerHtml("B_3", wFront2 + (elementOBJ[w] + targetStats.element % 10) + wBack, 0);
        // else
        //     myInnerHtml("B_3", (elementOBJ[w] + targetStats.element % 10), 0);
        // myInnerHtml("B_4", SizeOBJ[targetStats.size], 0);
    } else {
        // targetStats.flee += eval(document.calcForm.B_TAISEI7.value);
        // n_Ses = document.calcForm.B_Ses.checked;
        // if (n_Ses) {
        //     targetStats.flee = Math.floor(targetStats.flee * 0.8);
        // }
    }
    //
    // n_B_DEF2 = [0, 0, 0];
    // n_B_DEF2[2] = targetStats.def2Min;
    // n_B_DEF2[0] = targetStats.def2Max;
    // n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2);
    // n_B_MDEF2 = targetStats.mdef2;
    // targetStats.hit = targetStats.hit;
    // targetStats.flee = targetStats.flee;
    return {
        ...targetStats,
        ...targetStatusFlag

    }
}

function CalculateBattle(stats, targetStats, InWarOfEmperium) {
    let battleResult = {
        battleHit: 0, // BattleHit
        battleFlee: 0, // BattleFlee
        critChance: 0, // CRInum
        critAtk: [],  // CRIATK
        baseAttackCalc: 0,  // BaseAttackCalc
        minWeaponAttackCalc: 0, // MinWeaponAttackCalc
        maxWeaponAttackCalc: 0,  // AvgWeaponAttackCalc
        avgWeaponAttackCalc: 0,  // MaxWeaponAttackCalc
        bonusSub: "", // bSUB
        bonusSubName: "", // bSUBname
        bonusSub2: "", // bSUB2
        bonusSub2Name: "", // bSUB2name
        critAtkName: "", // CRIATKname
        critChanceName: "", // CRInumname
        atk00: "", // ATK_00
        atk02: "", // ATK_02
        atk01: "", // ATK_01
        battleHit: 0, // BattleHIT
        dps: 0, // DPS
        minAtkNum: 0, // MinATKnum
        maxAtkNum: 0, // MaxATKnum
        avgAtkNum: 0, // AveATKnum
        battleTime: 0, // BattleTime
        averageReceivedDamage: 0, // AverageReceivedDamageIncludingDodge
    }
    let sizeModifier = weaponsize[stats.weaponType][targetStats.size];
    if (SkillSearch("Cavalier Mastery", stats)) {
        if ((stats.weaponType == WEAPON_TYPE_SPEAR || stats.weaponType == WEAPON_TYPE_TWO_HANDED_SPEAR) && targetStats.size == 1)
            sizeModifier = 1;
    }
    if (SkillSearch("Weapon Perfection", stats) || stats.supportiveSkills[7])
        sizeModifier = 1;

    if (cardOBJ[stats.cards[0]][0] == 32 || cardOBJ[stats.cards[1]][0] == 32 || cardOBJ[stats.cards[2]][0] == 32 || cardOBJ[stats.cards[3]][0] == 32 || cardOBJ[stats.cards[4]][0] == 32 || cardOBJ[stats.cards[5]][0] == 32 || cardOBJ[stats.cards[6]][0] == 32 || cardOBJ[stats.cards[7]][0] == 32)
        sizeModifier = 1;


    let impositioMagnus = stats.supportiveSkills[2] * 5;


    let hitRate = stats.hit + 80 - (targetStats.flee);
    if (SkillSearch("Weaponry Research", stats))
        hitRate = Math.floor(hitRate * (100 + 2 * SkillSearch("Weaponry Research", stats)) / 100);
    if (stats.skillToUseName == "Pierce" || stats.skillToUseName == "Bash") {
        hitRate *= 1 + stats.skillToUseLV * 0.05;
    }
    if ((stats.skillToUseName == "Sonic Blow" || stats.skillToUseName == "Sonic Blow (Soul Linked)") && SkillSearch("Sonic Acceleration", stats)) {
        hitRate *= 1.5;
    }
    if (stats.skillToUseName == "Magnum Break") {
        hitRate *= 1 + stats.skillToUseLV * 0.1;
    }
    if (stats.skillToUseName == "Sharp Shooting (Temp)") {
        hitRate *= (1 + stats.skillToUseLV * 0.1);
    }
    if (stats.skillToUseName == "Counter Kick") {
        hitRate = 100;
    }
    if (stats.skillToUseName == "Shield Boomerang (SoulLinked)") {
        hitRate = 100;
    }
    if (SkillSearch("Solar, Lunar, and Stellar Union", stats)) {
        hitRate = 100;
    }
    if (hitRate > 100) {
        hitRate = 100;
    } else if (hitRate < 5) {
        hitRate = 5;
    }
    if (StPlusItem("INCREASE_HIT_PERCENTAGE", stats) + StPlusCard("INCREASE_HIT_PERCENTAGE", stats))
        hitRate = hitRate + (100 - hitRate) * (StPlusItem("INCREASE_HIT_PERCENTAGE", stats) + StPlusCard("INCREASE_HIT_PERCENTAGE", stats)) / 100;

    hitRate = Math.floor(hitRate * 100) / 100;
    battleResult.battleHit = hitRate;
    // myInnerHtml("BattleHIT", hitRate, 0);

    if (stats.skillToUseName == "Sharp Shooting (Temp)") {
        stats.crit += 20;
    }
    let criticalRate = stats.crit - targetStats.luk * 0.2 - 0.1;
    if (targetStats.sleep)
        criticalRate *= 2;
    if (criticalRate < 0) {
        criticalRate = 0;
    } else if (criticalRate > 100) {
        criticalRate = 100;
    }
    battleResult.critChance = Math.round(criticalRate * 10) / 10 + SubName[0];
    // myInnerHtml("CRInum", Math.round(criticalRate * 10) / 10 + SubName[0], 0);


    let tripleAttackChanceRate = 0;
    if (SkillSearch("Raging Trifecta Blow", stats)) // Ragin trifecta blow
        tripleAttackChanceRate = 30 - SkillSearch("Raging Trifecta Blow", stats);


    let doubleAttackChanceRate = SkillSearch("Double Attack", stats) * 5;
    if (stats.weaponType != WEAPON_TYPE_DAGGER)
        doubleAttackChanceRate = 0;
    if (cardOBJ[stats.cards[0]][0] == 43 || cardOBJ[stats.cards[1]][0] == 43 || cardOBJ[stats.cards[2]][0] == 43 || cardOBJ[stats.cards[3]][0] == 43 || cardOBJ[stats.cards[4]][0] == 43 || cardOBJ[stats.cards[5]][0] == 43 || cardOBJ[stats.cards[6]][0] == 43 || cardOBJ[stats.cards[7]][0] == 43) {
        if (SkillSearch("Double Attack", stats) > 1)
            doubleAttackChanceRate = SkillSearch("Double Attack", stats) * 5;
        else
            doubleAttackChanceRate = 5;
    }
    if (ItemOBJ[stats.equipments[2]][0] == 570) {
        if (SkillSearch("Double Attack", stats) > 1)
            doubleAttackChanceRate = SkillSearch("Double Attack", stats) * 5;
        else
            doubleAttackChanceRate = 10;
    }
    if (ItemOBJ[stats.equipments[0]][0] == 399 || ItemOBJ[stats.equipments[1]][0] == 399)
        doubleAttackChanceRate = 25;
    if (stats.weaponType == WEAPON_TYPE_HANDGUN)
        doubleAttackChanceRate = SkillSearch("Single Action", stats) * 5;

    let hitRateDoubleAttack = hitRate;
    if (doubleAttackChanceRate != 0 && stats.weaponType != WEAPON_TYPE_HANDGUN) {
        hitRateDoubleAttack = hitRateDoubleAttack * (100 + SkillSearch("Double Attack", stats)) / 100;
        if (hitRateDoubleAttack >= 100)
            hitRateDoubleAttack = 100;
    }

    let w998A = 100 - tripleAttackChanceRate;
    let w998B = tripleAttackChanceRate * hitRate / 100;
    let w998C = tripleAttackChanceRate - w998B;
    let w998D = w998A * doubleAttackChanceRate / 100;
    let w998E = w998D * hitRateDoubleAttack / 100;
    let w998F = w998D - w998E;
    let w998G = (100 - tripleAttackChanceRate - w998D) * criticalRate / 100;
    let w998H = 100 - tripleAttackChanceRate - w998D - w998G;
    let w998I = w998H * hitRate / 100;
    let w998J = w998H - w998I;
    let w998K = w998B + w998E + w998G + w998I;
    let w998L = 100 - w998K;


    stats.flee = stats.flee + 20 - (targetStats.hit);
    if (stats.flee > 95) {
        stats.flee = 95;
    } else if (stats.flee < 5) {
        stats.flee = 5;
    }
    if (InWarOfEmperium == 0) ;
    // myInnerHtml("BattleFLEE", stats.flee, 0);
    battleResult.battleFlee = stats.flee;

    let workDex = Math.floor(stats.dex * (1 + (stats.weapon1Lv - 1) * 0.2));

    let n_A_DMG = [0, 0, 0];
    let weaponAttack = [0, 0, 0];


    if (workDex >= stats.weapon1ATK || SkillSearch("Power Maximize", stats)) // 155 = power maximize
        weaponAttack[2] = stats.weapon1LV_overUpgradeBonusATK + Math.floor((stats.weapon1ATK + impositioMagnus) * sizeModifier);
    else
        weaponAttack[2] = stats.weapon1LV_overUpgradeBonusATK + Math.floor((stats.weapon1ATK - 1 + impositioMagnus) * sizeModifier);

    if (isRangedWeapon(stats.weaponType))
        weaponAttack[2] += Math.floor((ArrowOBJ[stats.arrow][0] - 1) * sizeModifier);


    if (isRangedWeapon(stats.weaponType)) {
        w1 = stats.weapon1LV_overUpgradeBonusATK + Math.floor(stats.weapon1ATK * stats.weapon1ATK / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);
        w2 = stats.weapon1LV_overUpgradeBonusATK + Math.floor(stats.weapon1ATK * workDex / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);

        w = Math.floor((ArrowOBJ[stats.arrow][0] - 1) * sizeModifier);
        w1 += w;
        w2 += w;
        if (w1 > w2) w1 = w2;
        if (weaponAttack[2] < w1) weaponAttack[2] = w1;
    }


    if (isRangedWeapon(stats.weaponType)) {
        weaponAttack[0] = stats.weapon1LV_Minplus + Math.floor((stats.weapon1ATK * stats.weapon1ATK / 100 + impositioMagnus) * sizeModifier);
        w = stats.weapon1LV_Minplus + Math.floor((stats.weapon1ATK * workDex / 100 + impositioMagnus) * sizeModifier);
        if (weaponAttack[0] > w) weaponAttack[0] = w;
    } else {
        if (workDex >= stats.weapon1ATK)
            weaponAttack[0] = stats.weapon1LV_Minplus + Math.floor((stats.weapon1ATK + impositioMagnus) * sizeModifier);
        else {

            if (SkillSearch("Power Maximize", stats))
                workDex = stats.weapon1ATK;
            weaponAttack[0] = stats.weapon1LV_Minplus + Math.floor((workDex + impositioMagnus) * sizeModifier);
        }
    }


    weaponAttack[1] = (weaponAttack[0] + weaponAttack[2]) / 2;
    n_A_DMG[0] = stats.baseATK + weaponAttack[0];
    n_A_DMG[1] = stats.baseATK + weaponAttack[1];
    n_A_DMG[2] = stats.baseATK + weaponAttack[2];
    battleResult.baseAttackCalc = stats.baseATK;
    battleResult.minWeaponAttackCalc = weaponAttack[0];
    battleResult.maxWeaponAttackCalc = weaponAttack[2];
    battleResult.avgWeaponAttackCalc = weaponAttack[1];
    // myInnerHtml("BaseAttackCalc", stats.baseATK, 0);
    // myInnerHtml("MinWeaponAttackCalc", weaponAttack[0], 0);
    // myInnerHtml("AvgWeaponAttackCalc", weaponAttack[1], 0);
    // myInnerHtml("MaxWeaponAttackCalc", weaponAttack[2], 0);

    n_Enekyori = 0;
    stats.critATK = [0, 0, 0];
    stats.critATK[1] = stats.baseATK + (stats.weapon1LV_Minplus + stats.weapon1LV_overUpgradeBonusATK) / 2 + Math.floor((stats.weapon1ATK + impositioMagnus) * sizeModifier);
    stats.critATK[0] = stats.baseATK + stats.weapon1LV_Minplus + Math.floor((stats.weapon1ATK + impositioMagnus) * sizeModifier);
    stats.critATK[2] = stats.baseATK + stats.weapon1LV_overUpgradeBonusATK + Math.floor((stats.weapon1ATK + impositioMagnus) * sizeModifier);


    if (stats.weaponType == WEAPON_TYPE_BOW) {
        n_Enekyori = 1;
        stats.critATK[1] += Math.floor((ArrowOBJ[stats.arrow][0]) * sizeModifier);
        stats.critATK[0] += Math.floor((ArrowOBJ[stats.arrow][0]) * sizeModifier);
        stats.critATK[2] += Math.floor((ArrowOBJ[stats.arrow][0]) * sizeModifier);
    }


    let BK_n_A_DMG = [0, 0, 0];
    BK_n_A_DMG[2] = n_A_DMG[2];
    BK_n_A_DMG[0] = n_A_DMG[0];
    BK_n_A_DMG[1] = n_A_DMG[1];

    ApplyATKBonusPercentage(stats, n_A_DMG);
    ApplySkillModifier(stats, n_A_DMG, 1, 1);

    wCriTyuu = 1;
    stats.critATK[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[1], 10, InWarOfEmperium);
    stats.critATK[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[0], 10, InWarOfEmperium);
    stats.critATK[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[2], 10, InWarOfEmperium);
    wCriTyuu = 0;


    let n_A_EDP_DMG = [0, 0, 0];
    n_A_EDP_DMG[2] = BattleCalcEDP(stats, targetStats, n_A_DMG[2], 2);
    n_A_EDP_DMG[0] = BattleCalcEDP(stats, targetStats, n_A_DMG[0], 0);
    n_A_EDP_DMG[1] = BattleCalcEDP(stats, targetStats, n_A_DMG[1], 1);

    if (stats.weaponType == WEAPON_TYPE_KATAR) {
        wk = Math.floor(stats.critATK[1] * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
        wk2 = Math.floor((stats.critATK[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
        if (stats.weapon1LV_Minplus == stats.weapon1LV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2]) {
            battleResult.critAtk[0] = stats.critATK[1] + wk2 + n_A_EDP_DMG[1];
            // myInnerHtml("CRIATK", (stats.critATK[1] + wk2 + n_A_EDP_DMG[1]) + "(" + (stats.critATK[1] + n_A_EDP_DMG[1]) + "+" + wk2 + ")", 0);
        } else {
            w1 = Math.floor((stats.critATK[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            w2 = Math.floor((stats.critATK[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            battleResult.critAtk[0] = stats.critATK[0] + w1 + n_A_EDP_DMG[0];
            battleResult.critAtk[1] = stats.critATK[2] + w2 + n_A_EDP_DMG[2];
            // myInnerHtml("CRIATK", (stats.critATK[0] + w1 + n_A_EDP_DMG[0]) + " ~ " + (stats.critATK[2] + w2 + n_A_EDP_DMG[2]) + "(" + (stats.critATK[0] + n_A_EDP_DMG[0]) + " ~ " + (stats.critATK[2] + n_A_EDP_DMG[2]) + "+" + w1 + " ~ " + w2 + ")", 0);
        }
        stats.critATK[1] += wk;
    } else {
        if (stats.weapon1LV_Minplus == stats.weapon1LV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2]) {
            battleResult.critAtk[0] = stats.critATK[1] + n_A_EDP_DMG[1];
            //     myInnerHtml("CRIATK", stats.critATK[1] + n_A_EDP_DMG[1], 0);
        } else {
            battleResult.critAtk[0] = stats.critATK[0] + n_A_EDP_DMG[0];
            battleResult.critAtk[1] = stats.critATK[2] + n_A_EDP_DMG[2];
            //     myInnerHtml("CRIATK", (stats.critATK[0] + n_A_EDP_DMG[0]) + " ~ " + (stats.critATK[2] + n_A_EDP_DMG[2]), 0);
        }
    }


    stats.critATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
    stats.critATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
    stats.critATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

    let battleResult999 = BattleCalc999(stats, targetStats, InWarOfEmperium, hitRate, criticalRate, n_A_DMG, n_A_EDP_DMG, w998B, w998E, w998G, w998I, w998L);
    battleResult = Object.assign(battleResult, battleResult999);
    return battleResult;
}

function HitEDPplus(wBCEDPp) {
    if (wBCEDPp <= 0)
        return 0;
    if (element[targetStatsArray[TARGET_STAT_ELEMENT]][n_A_Weapon_element] <= 0)
        return 0;
    wBCEDPpDA = 1;
    if (n_A_ActiveSkill == 0)
        wBCEDPpDA = (100 + w998E) / 100;

    wBCEDPch = 1;
    wBCEDPpHOSI = ApplyWeaponryResearchAndDMGLevel(0);
    wBCEDPch = 0;
    if (wBCEDPpHOSI >= 1) {
        www = hitRate;

        if (SkillSearch(266))
            wBCEDPpHOSI = Math.floor((wBCEDPpHOSI * element[targetStatsArray[TARGET_STAT_ELEMENT]][5]) / 4);
        if (n_A_PassSkill2[11])
            wBCEDPpHOSI = Math.floor((wBCEDPpHOSI * element[targetStatsArray[TARGET_STAT_ELEMENT]][3]) / 5);
    } else
        www = w998K * hitRate / 100;

    if (n_A_WeaponType == WEAPON_TYPE_KATAR && n_A_ActiveSkill == 0)
        wBCEDPp = Math.floor(wBCEDPp * (1.01 + SkillSearch(13) * 0.02));


    if (n_A_ActiveSkill == 0) {
        wBCEDPp *= wBCEDPpDA;
        wBCEDPpHOSI *= wBCEDPpDA;
    }
    return (wBCEDPp * www / 100) + (wBCEDPpHOSI * (100 - hitRate) / 100);
}

function ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, w_atk, w_2, InWarOfEmperium) {
    if (w_2 == 10)
        w_atk += stats.weapon1LV_upgradeBonusATK;
    else
        w_atk = BattleCalc4(stats, targetStats, w_atk, w_2, 0);

    if (w_atk < 1) w_atk = 1;


    if (stats.weaponType == WEAPON_TYPE_DAGGER || stats.weaponType == WEAPON_TYPE_SWORD) w_atk += 4 * SkillSearch(" Sword Mastery", stats);
    else if (stats.weaponType == WEAPON_TYPE_TWO_HANDED_SWORD) w_atk += 4 * SkillSearch("Two-Hand Sword Mastery", stats);
    else if (stats.weaponType == WEAPON_TYPE_SPEAR || stats.weaponType == WEAPON_TYPE_TWO_HANDED_SPEAR) {
        if (SkillSearch("Cavalier Mastery", stats) == 0)
            w_atk += 4 * SkillSearch("Spear Mastery", stats);
        else
            w_atk += 5 * SkillSearch("Spear Mastery", stats);

    } else if (stats.weaponType == WEAPON_TYPE_KATAR) w_atk += 3 * SkillSearch("Katar Mastery", stats);
    else if (stats.weaponType == WEAPON_TYPE_MACE) w_atk += 3 * SkillSearch("Mace Mastery", stats);
    else if (stats.weaponType == WEAPON_TYPE_KNUCKLE || stats.weaponType == WEAPON_TYPE_UNARMED) w_atk += 3 * SkillSearch("Iron Fist", stats);
    else if (stats.weaponType == WEAPON_TYPE_INSTRUMENT) w_atk += 3 * SkillSearch("Music Lessons", stats);
    else if (stats.weaponType == WEAPON_TYPE_WHIP) w_atk += 3 * SkillSearch("Dance Lessons", stats);
    else if (stats.weaponType == WEAPON_TYPE_BOOK) w_atk += 3 * SkillSearch("Study", stats);
    else if (stats.weaponType == WEAPON_TYPE_AXE || stats.weaponType == WEAPON_TYPE_TWO_HANDED_AXE) w_atk += 3 * SkillSearch("Axe Mastery", stats);

    if (stats.weaponType == WEAPON_TYPE_UNARMED && SkillSearch("Sprint (Unarmed Mastery)", stats))
        w_atk += 10 * SkillSearch("Sprint (Unarmed Mastery)", stats);

    if (stats.performanceSkills[10] && stats.weapon1Lv == 4)
        w_atk += 50 + 25 * stats.performanceSkills[10];


    if (targetStats.race == 6 || (90 <= targetStats.element && targetStats.element <= 99)) {
        if (SkillSearch("Demon Bane", stats))
            w_atk += Math.floor((3 + 5 / 100 * stats.baseLevel) * SkillSearch("Demon Bane", stats));
    }
    if (targetStats.race == 2 || targetStats.race == 4) {
        w_atk += 4 * SkillSearch("Beast Bane", stats);
        if (SkillSearch("Hunter Spirit (Soul Link)", stats))
            w_atk += stats.str;
    }

    w_atk = ApplyWeaponryResearchAndDMGLevel(stats, targetStats, w_atk, InWarOfEmperium);

    return Math.floor(w_atk);
}


function ApplyWeaponryResearchAndDMGLevel(stats, targetStats, w999, InWarOfEmperium) {

    let w999_AB = 0;
    if (w999 > 0)
        w999_AB = 1;


    w999 += 2 * SkillSearch("Weaponry Research", stats);


    if (wBCEDPch == 0)
        w999 = w999 * element[targetStats.element][stats.weapon1Element];


    if (stats.weaponType == WEAPON_TYPE_UNARMED && SkillSearch("Sprint (Unarmed Mastery)", stats))
        if (stats.skillToUseName == "Tornado Kick" || stats.skillToUseName == "Heel Drop" || stats.skillToUseName == "Roundouse" || stats.skillToUseName == "Counter Kick")
            w999 += 10 * SkillSearch("Sprint (Unarmed Mastery)", stats);


    if (stats.job == 15 || stats.job == 29)
        w999 += 3 * SkillSearch("Summon Spirit Sphere", stats);
    else
        w999 += 3 * stats.supportiveSkills[10];

    w999 += 3 * SkillSearch("Coin Flip", stats);


    if (stats.weaponType != WEAPON_TYPE_UNARMED && w999_AB == 1)
        w999 += 20 * SkillSearch("Aura Blade", stats);


    if (wBCEDPch == 0) {
        if (stats.skillToUseName == "Envenom" || stats.skillToUseName == "")
            w999 += 15 * stats.skillToUseLV;
        if (stats.skillToUseName == "Poison React (Counter)" && (targetStats.element < 50 || 60 <= targetStats.element))
            w999 += 75;
    }
    if (stats.skillToUseName == "Magical Bullet")
        w999 += Math.floor(n_A_MATK[b] * (100 - targetStatsArray[TARGET_STAT_MDEF]) / 100 - n_B_MDEF2);
    if (stats.skillToUseName == "Gunslinger Mine")
        w999 += stats.skillToUseLV * 50;


    if (cardOBJ[stats.cards[0]][0] == 106 && cardOBJ[stats.cards[1]][0] == 106 && cardOBJ[stats.cards[2]][0] == 106) {
        w999 += 40;
    } else {
        for (let i = 0; i <= 2; i++) {
            if (cardOBJ[stats.cards[i]][0] == 106)
                w999 += 5;
        }
    }
    if (stats.cards[3] == 106)
        w999 += 10;


    if (stats.skillToUseName == "Throw Dagger") {
        w999 += SyurikenOBJ[eval(document.calcForm.SkillSubNum.value)][0];
        w999 += 3 * SkillSearch("Dagger Throwing Practice", stats);
        w999 += 4 * stats.skillToUseLV;
    }

    if (stats.skillToUseName == "Throw Kunai")
        w999 += KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][0] * 3;

    w999 = BaiCI(stats, targetStats, w999, InWarOfEmperium);


    if (stats.skillToUseName == "Back Stab" && stats.weaponType == WEAPON_TYPE_BOW)
        w999 = Math.floor(w999 / 2);


    if (hasLeftHand && stats.skillToUseName == "Basic Attack") {

        if (stats.weaponType != WEAPON_TYPE_UNARMED)
            w999 = Math.floor(w999 * (50 + SkillSearch("Righthand Mastery", stats) * 10) / 100);
    }


    if (targetStats.isStaticPlant)
        return 1;

    if (stats.skillToUseName == "Magical Bullet")
        w999 = w999 * element[targetStats.element][8];
    if (stats.skillToUseName == "Gunslinger Mine")
        w999 = w999 * element[targetStats.element][0];

    return w999;
}


function BaiCI(stats, targetStats, wBaiCI, InWarOfEmperium) {


    if (wBCEDPch == 0 && not_use_card == 0) {

        let w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_RACE_PERCENTAGE + targetStats.race, stats);
        w1 += StPlusItem(INCREASE_DAMAGE_RACE_PERCENTAGE + targetStats.race, stats);
        if (targetStats.race == 6) {
            if (ArrowOBJ[stats.arrow][2] == "Holy Arrow")
                w1 += 5;
        }
        if (targetStats.race == 9 && SkillSearch("Dragonology", stats))
            w1 += SkillSearch("Dragonology", stats) * 4;

        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_ELEMENT_PERCENTAGE + Math.floor(targetStats.element / 10), stats);
        w1 += StPlusItem(INCREASE_DAMAGE_ELEMENT_PERCENTAGE + Math.floor(targetStats.element / 10), stats);
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        w1 = 0;
        w1 += StPlusCard(INCREASE_DAMAGE_AGAINST_SIZE_PERCENTAGE + targetStats.size, stats);
        w1 += StPlusItem(INCREASE_DAMAGE_AGAINST_SIZE_PERCENTAGE + targetStats.size, stats);
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        if (n_Enekyori == 1) {
            if (TyouEnkakuSousa3dan != -1) {
                w1 = 0;
                w1 += StPlusCard("REDUCE_DEFENSE_PERCENTAGE", stats);
                w1 += StPlusItem("REDUCE_DEFENSE_PERCENTAGE", stats);
                wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);
            }
        }


        w1 = 0;
        if (targetStats.isMvp) {
            w1 += StPlusCard("INCREASE_DAMAGE_AGAINST_BOSS_PERCENTAGE", stats);
            w1 += StPlusItem("INCREASE_DAMAGE_AGAINST_BOSS_PERCENTAGE", stats);
        }
        if (EquipNumSearch("The Sign", stats))
            w1 += EquipNumSearch("The Sign", stats) * 5;
        if (CardNumSearch("Turtle General", stats))
            w1 += CardNumSearch("Turtle General", stats) * 20;
        if (CardNumSearch("Valkyrie Randgris", stats))
            w1 += CardNumSearch("Valkyrie Randgris", stats) * 10;
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);


        if (wCriTyuu == 1 && stats.skillToUseName != "Sharp Shooting (Temp)")
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard("CRITICAL_DAMAGE_PERCENTAGE", stats)) / 100);


        if (108 <= targetStats.mobIndex && targetStats.mobIndex <= 115 || targetStats.mobIndex == 319)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard("INCREASE_DAMAGE_GOBLIN_PERCENTAGE", stats)) / 100);

        if (116 <= targetStats.mobIndex && targetStats.mobIndex <= 120)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard("INCREASE_DAMAGE_KOBOLD_PERCENTAGE", stats)) / 100);

        if (49 <= targetStats.mobIndex && targetStats.mobIndex <= 52 || 55 == targetStats.mobIndex || 221 == targetStats.mobIndex)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard("INCREASE_DAMAGE_ORC_PERCENTAGE", stats)) / 100);

        if (106 == targetStats.mobIndex || 152 == targetStats.mobIndex || 308 == targetStats.mobIndex || 32 == targetStats.mobIndex)
            wBaiCI = Math.floor(wBaiCI * (100 + StPlusCard("INCREASE_DAMAGE_GOLEM_PERCENTAGE", stats)) / 100);


        wBaiCI = Math.floor(wBaiCI * (100 + StPlusItem(1000 + targetStats.mobIndex, stats) + StPlusCard(1000 + targetStats.mobIndex, stats)) / 100);


        if (EquipNumSearch("Burning Bow", stats) && stats.arrow == 2)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Freezing Bow", stats) && stats.arrow == 5)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Earthen Bow", stats) && stats.arrow == 4)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Gale Bow", stats) && stats.arrow == 6)
            wBaiCI = wBaiCI * 125 / 100;

        if (EquipNumSearch("Orc Archer's Bow", stats) && stats.arrow == 10)
            wBaiCI = wBaiCI * 150 / 100;

        if (SkillSearch("Frenzy", stats))
            wBaiCI = wBaiCI * 2;
        if (SkillSearch("Enchant Deadly Poison", stats))
            wBaiCI = Math.floor(wBaiCI * (150 + 50 * SkillSearch("Enchant Deadly Poison", stats)) / 100);
        if (stats.skillToUseName == "Poison React (Counter)" && (50 <= targetStats.element && targetStats.element < 60))
            wBaiCI = Math.floor(wBaiCI * (100 + 30 * stats.skillToUseLV) / 100);


        if (stats.weaponType == WEAPON_TYPE_KATAR && SkillSearch("Advanced Katar Mastery", stats) && stats.skillToUseName != "Soul Destroyer")
            wBaiCI = Math.floor(wBaiCI * (110 + 2 * SkillSearch("Advanced Katar Mastery", stats)) / 100);

        w1 = 0;
        if (InWarOfEmperium == 0) {
            if (SkillSearch("Stellar Wrath", stats) && SkillSearch("Solar, Lunar, and Stellar Miracle", stats))
                w1 += (stats.baseLevel + stats.str + stats.luk + stats.dex) / (12 - SkillSearch("Stellar Wrath", stats) * 3);
            else if (SkillSearch("Stellar Wrath", stats) && targetStats.size == 2 && targetStats.hp >= 20000)
                w1 += (stats.baseLevel + stats.str + stats.luk + stats.dex) / (12 - SkillSearch("Stellar Wrath", stats) * 3);
            else if (SkillSearch("Solar Wrath", stats) && targetStats.size == 0)
                w1 += (stats.baseLevel + stats.luk + stats.dex) / (12 - SkillSearch("Solar Wrath", stats) * 3);
            else if (SkillSearch("Lunar Wrath", stats) && targetStats.size == 1 && targetStats.hp >= 5000)
                w1 += (stats.baseLevel + stats.luk + stats.dex) / (12 - SkillSearch("Lunar Wrath", stats) * 3);
        } else {
            if (SkillSearch("Stellar Wrath", stats)) {
                w1 += (stats.baseLevel + stats.str + stats.luk + stats.dex) / (12 - SkillSearch("Stellar Wrath", stats) * 3);
            } else {
                if (SkillSearch("Solar Wrath", stats)) {
                    w1 += (stats.baseLevel + stats.luk + stats.dex) / (12 - SkillSearch("Solar Wrath", stats) * 3);
                } else {
                    if (SkillSearch("Lunar Wrath", stats))
                        w1 += (stats.baseLevel + stats.luk + stats.dex) / (12 - SkillSearch("Lunar Wrath", stats) * 3);
                }
            }
        }
        wBaiCI = Math.floor(wBaiCI * (100 + w1) / 100);
    }

    wBaiCI = Math.floor(tPlusDamCut(stats, targetStats, wBaiCI, InWarOfEmperium));

    let w1 = 0;

    if (stats.skillToUseName == "Bash")
        if (stats.shoesHandDefPlus >= 9 && CardNumSearch("Freezer"))
            w1 += 10;

    if (TyouEnkakuSousa3dan == -1 && EquipNumSearch("Barrage Fist", stats))
        w1 += 15;

    if ((stats.skillToUseName == "Sonic Blow" || stats.skillToUseName == "Sonic Blow (Soul Linked)") && SkillSearch("Sonic Acceleration", stats))
        w1 += 10;

    wBaiCI = wBaiCI * (100 + StPlusItem(5000 + stats.skillToUse, stats) + StPlusCard(5000 + stats.skillToUse, stats) + w1) / 100;

    return wBaiCI;
}


function BattleCalc3(stats, targetStats, w998, InWarOfEmperium, w998B, w998E, w998G, w998I, w998L) {
    let wBC3_3dan = w998B * TyouEnkakuSousa3dan;
    let wBC3_DA = w998E * w998 * 2;
    let wBC3_Cri = w998G * stats.critATK[1];
    let wBC3_Normal = w998I * w998;
    let wBC3_Miss = w998L * ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium);

    let wBC3_X = (wBC3_3dan + wBC3_DA + wBC3_Cri + wBC3_Normal + wBC3_Miss) / 100;

    return tPlusLucky(wBC3_X, InWarOfEmperium);
}


function BattleCalc3left(w998) {

    wBC3L2 = 0;
    for (i = 4; i <= 7; i++) {
        if (cardOBJ[stats.cards[i]][0] == 106)
            wBC3L2 += 5;
    }

    wBC3_Normal = w998 * hitRate / 100;
    wBC3_Miss = wBC3L2 * (100 - hitRate) / 100;

    wBC3_X = wBC3_Normal + wBC3_Miss;

    wBC3_X = tPlusDamCut(stats, targetStats, wBC3_X, InWarOfEmperium);

    return tPlusLucky(wBC3_X);
}


function BattleCalc4(stats, targetStats, wBC4, wBC4_2, wBC4_3) {

    if (wBC4_3 === undefined || wBC4_3 == 0)
        wBC4_3 = stats.weapon1LV_upgradeBonusATK;
    else
        wBC4_3 = stats.weapon2LV_upgradeBonusATK;
    let n_B_DEF2 = [targetStats.def2Max, targetStats.def2Avg, targetStats.def2Min]
    if ((StPlusItem("WEAPON_ATK_INCREASE_ON_TARGET_DEFENSE", stats) + StPlusCard("WEAPON_ATK_INCREASE_ON_TARGET_DEFENSE", stats)) == 0 || stats.skillToUseName == "Stave Crasher") {
        if (stats.skillToUseName == "Wounding Shot")
            return wBC4 + wBC4_3;
        if (StPlusItem("BYPASS_DEFENSE_ON_RACE", stats) == targetStats.race && targetStats.race != 0)
            return wBC4 + wBC4_3;
        if (StPlusItem("BYPASS_DEFENSE_ON_RACE", stats) == 99 && targetStats.isNormal)
            return wBC4 + wBC4_3;
        if (SkillSearch("Solar, Lunar, and Stellar Union", stats))
            return wBC4 + wBC4_3;
        if (CardNumSearch("Samurai Specter", stats) && targetStats.isNormal)
            return wBC4 + wBC4_3;
        wBC4 = Math.floor(wBC4 * (100 - targetStats.def) / 100) - n_B_DEF2[wBC4_2] + wBC4_3;
    } else {
        if (wBC4_2 == 0) {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[2] + targetStats.def) / 100) + wBC4_3;
        } else if (wBC4_2 == 1) {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[1] + targetStats.def) / 100) + wBC4_3;
        } else {
            wBC4 = Math.floor(wBC4 * (n_B_DEF2[0] + targetStats.def) / 100) + wBC4_3;
        }
    }
    return wBC4;
}


function BattleCalcEDP(stats, targetStats, wBCEDP, wBCEDP2) {

    if (wBCEDP <= 0)
        return 0;
    if (element[targetStats.element][stats.weapon1Element] <= 0 && ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) == 0)
        return 0;

    if (stats.skillToUseName == "Sand Attack" || stats.skillToUseName == "Soul Destroyer" || stats.skillToUseName == "Venom Splasher" || stats.skillToUseName == "Meteor Assault" || stats.skillToUseName == "Bomb")
        return 0;
    let wBCEDPch = 1;
    let wBCEDPx = 0;
    let wBCEDPy = 0;
    if (SkillSearch("Enchant Deadly Poison", stats)) {
        wBCEDPx = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, wBCEDP, wBCEDP2, InWarOfEmperium);
        wBCEDPx = Math.floor((wBCEDPx * element[targetStats.element][5]) / 4);
    }
    if (stats.supportiveSkills[11]) {
        wBCEDPy = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, wBCEDP, wBCEDP2, InWarOfEmperium);
        wBCEDPy = Math.floor((wBCEDPy * element[targetStats.element][3]) / 5);
    }
    wBCEDPch = 0;
    return wBCEDPx + wBCEDPy;
}

function BattleCalc999(stats, targetStats, InWarOfEmperium, hitRate, criticalRate, n_A_DMG, n_A_EDP_DMG, w998B, w998E, w998G, w998I, w998L) {
    let skillModifier = 1;
    let wCast = 0;
    let wDelay = 0;
    let swDelay = 0;
    let battleResult = {
        // bonusSub: "", // bSUB
        // bonusSubName: "", // bSUBname
        // bonusSub2: "", // bSUB2
        // bonusSub2Name: "", // bSUB2name
        // critAtk: [],  // CRIATK
        // critChance: 0, // CRInum
        // critAtkName: "", // CRIATKname
        // critChanceName: "", // CRInumname
        // atk00: "", // ATK_00
        // atk02: "", // ATK_02
        // atk01: "", // ATK_01
        // battleHit: 0, // BattleHIT
        // dps: 0, // DPS
        // minAtkNum: 0, // MinATKnum
        // maxAtkNum: 0, // MaxATKnum
        // avgAtkNum: 0, // AveATKnum
        // battleTime: 0, // BattleTime
        // averageReceivedDamage: 0, // AverageReceivedDamageIncludingDodge
    };

    if (stats.skillToUseName != "Basic Attack" && stats.skillToUseName != "Martyr's Reconing") {
        wDelay = Math.floor(stats.aspd * 100) / 100;
        if (stats.skillToUseName == "Envenom" || stats.skillToUseName == "")
            wDelay = Math.floor(stats.aspd * 75) / 100;
        let minimumDelayBetweenActiveSkills = 10;
        // wA_ASPD = eval(FORM_DATA.Conf01) / 100;
        let wA_ASPD = eval(minimumDelayBetweenActiveSkills) / 100;
        if (wDelay < wA_ASPD)
            wDelay = wA_ASPD;
    }

    let hitCount = 1;
    let isRangedAttack = 0;

    let finalDamages = [0, 0, 0];
    let not_use_card = 0;
    let cast_kotei = 0;


    // myInnerHtml("bSUBname", "", 0);
    // myInnerHtml("bSUB", "", 0);
    // myInnerHtml("bSUB2name", "", 0);
    // myInnerHtml("bSUB2", "", 0);


    if (stats.skillToUseName != "Basic Attack" && stats.skillToUseName != "Sharp Shooting (Temp)" && (stats.skillToUseName != "Poison React (Counter)" || (targetStats.element < 50 && 60 <= targetStats.element))) {
        // myInnerHtml("CRIATK", "", 0);
        // myInnerHtml("CRInum", "", 0);
        // myInnerHtml("CRIATKname", "", 0);
        // myInnerHtml("CRInumname", "", 0);
    }

    if ((stats.weaponType == WEAPON_TYPE_BOW || stats.weaponType == WEAPON_TYPE_HANDGUN || stats.weaponType == WEAPON_TYPE_RIFLE || stats.weaponType == WEAPON_TYPE_SHOTGUN || stats.weaponType == WEAPON_TYPE_GATLING_GUN || stats.weaponType == WEAPON_TYPE_GRENADE_LAUNCHER) && stats.skillToUseName === "Basic Attack")
        isRangedAttack = 1;


    if (stats.skillToUseName === "Basic Attack" || (stats.skillToUseName === "Poison React (Counter)" && (50 <= targetStats.element && targetStats.element < 60))) {
        // myInnerHtml("CRIATKname", SubName[3], 0);
        // myInnerHtml("CRInumname", SubName[4], 0);
        battleResult.critAtkName = SubName[3];
        battleResult.critChanceName = SubName[4];

        if (stats.skillToUseName === "Poison React (Counter)") {
            n_SpSkill = 1;
            if (stats.weaponType != WEAPON_TYPE_KATAR)
                ;
            battleResult.bonusSub = "Damage Shown with 2x right hand damage";
            // myInnerHtml("bSUB", '<Font size="2"><B>Damage Shown with 2x right hand damage.</B></Font>', 0);
        }

        if (hasLeftHand) {

            if (!targetStats.isStaticPlant) {
                TyouEnkakuSousa3dan = 0;

                stats.weapon2Index = eval(document.calcForm.A_weapon2.value);
                stats.weapon2LV = ItemOBJ[stats.weapon2Index][4];
                stats.weapon2_ATK = ItemOBJ[stats.weapon2Index][3];
                stats.weapon2_RefinementLevel = eval(document.calcForm.A_Weapon2_ATKplus.value);


                stats.weapon2LV_upgradeBonusATK = 0;
                stats.weapon2LV_Minplus = 0;
                stats.weapon2LV_overUpgradeBonusATK = 0;
                if (stats.weapon2LV == 1) {
                    stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 2;
                    if (stats.weapon2_RefinementLevel >= 8) {
                        stats.weapon2LV_Minplus = 1;
                        stats.weapon2LV_overUpgradeBonusATK = 3 * (stats.weapon2_RefinementLevel - 7);
                    }
                } else if (stats.weapon2LV == 2) {
                    stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 3;
                    if (stats.weapon2_RefinementLevel >= 7) {
                        stats.weapon2LV_Minplus = 1;
                        stats.weapon2LV_overUpgradeBonusATK = 5 * (stats.weapon2_RefinementLevel - 6);
                    }
                } else if (stats.weapon2LV == 3) {
                    stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 5;
                    if (stats.weapon2_RefinementLevel >= 6) {
                        stats.weapon2LV_Minplus = 1;
                        stats.weapon2LV_overUpgradeBonusATK = 8 * (stats.weapon2_RefinementLevel - 5);
                    }
                } else if (stats.weapon2LV == 4) {
                    stats.weapon2LV_upgradeBonusATK = stats.weapon2_RefinementLevel * 7;
                    if (stats.weapon2_RefinementLevel >= 5) {
                        stats.weapon2LV_Minplus = 1;
                        stats.weapon2LV_overUpgradeBonusATK = 14 * (stats.weapon2_RefinementLevel - 4);
                    }
                }

                workDex = Math.floor(stats.dex * (1 + (stats.weapon2LV - 1) * 0.2));

                if (workDex >= stats.weapon2_ATK)
                    w_left_Maxatk = stats.baseATK + stats.weapon2LV_overUpgradeBonusATK + Math.floor((stats.weapon2_ATK + impositioMagnus) * sizeModifier);
                else
                    w_left_Maxatk = stats.baseATK + stats.weapon2LV_overUpgradeBonusATK + Math.floor((stats.weapon2_ATK - 1 + impositioMagnus) * sizeModifier);

                w_left_Maxatk = BattleCalc4(stats, targetStats, w_left_Maxatk * skillModifier, 2, 1);

                if (w_left_Maxatk < 1) w_left_Maxatk = 1;
                w_left_Maxatk = Math.floor(w_left_Maxatk * element[targetStats.element][stats.weapon2Element]);


                w_left_star = 0;
                if (stats.cards[4] == 106 && stats.cards[5] == 106 && stats.cards[6] == 106) {
                    w_left_star += 40;
                } else {
                    for (i = 4; i <= 6; i++) {
                        if (cardOBJ[stats.cards[i]][0] == 106)
                            w_left_star += 5;
                    }
                }
                if (stats.cards[7] == 106)
                    w_left_star += 10;
                w_left_Maxatk += w_left_star;
                w_left_Maxatk = w_left_Maxatk * (3 + SkillSearch("Lefthand Mastery", stats)) / 10;
                w_left_Maxatk = Math.floor(w_left_Maxatk);


                if (workDex > stats.weapon2_ATK)
                    workDex = stats.weapon2_ATK;
                w_left_Minatk = stats.baseATK + stats.weapon2LV_Minplus + Math.floor((workDex + impositioMagnus) * sizeModifier);
                w_left_Minatk = BattleCalc4(stats, targetStats, w_left_Minatk * skillModifier, 0, 1);

                if (w_left_Minatk < 1) w_left_Minatk = 1;
                w_left_Minatk = Math.floor(w_left_Minatk * element[targetStats.element][stats.weapon2Element]);
                w_left_Minatk += w_left_star;
                w_left_Minatk *= (0.3 + SkillSearch("Lefthand Mastery", stats) / 10);
                w_left_Minatk = Math.floor(w_left_Minatk);

                w_left_Aveatk = (w_left_Maxatk + w_left_Minatk) / 2;
            } else {
                w_left_Maxatk = 1;
                w_left_Minatk = 1;
                w_left_Aveatk = 1;
            }

            ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);


            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[2], 2, InWarOfEmperium);
            // myInnerHtml("ATK_02", finalDamages[2] + n_A_EDP_DMG[2] + "(" + w_left_Maxatk + ")", 0);
            battleResult.atk02 = finalDamages[2] + n_A_EDP_DMG[2] + "(" + w_left_Maxatk + ")";

            finalDamages[2] = BattleCalc3(stats, targetStats, finalDamages[2], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[2] += BattleCalc3left(w_left_Maxatk);
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[0], 0, InWarOfEmperium);
            // myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0] + "(" + w_left_Minatk + ")", 0);
            battleResult.atk00 = finalDamages[0] + n_A_EDP_DMG[0] + "(" + w_left_Minatk + ")";

            finalDamages[0] = BattleCalc3(stats, targetStats, finalDamages[0], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[0] += BattleCalc3left(w_left_Minatk);
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[1], 1, InWarOfEmperium);
            // myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1] + "(" + w_left_Aveatk + ")", 0);
            battleResult.atk01 = finalDamages[1] + n_A_EDP_DMG[1] + "(" + w_left_Aveatk + ")";

            finalDamages[1] = BattleCalc3(stats, targetStats, finalDamages[1], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[1] += BattleCalc3left(w_left_Aveatk);
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            let battleVariousResult = BattleVariousResults(stats, targetStats, 0, 0, finalDamages, InWarOfEmperium);
            battleResult = Object.assign(battleResult, battleVariousResult);
        } else if (stats.weaponType == WEAPON_TYPE_KATAR) {
            ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[2], 2, InWarOfEmperium);
            wk = Math.floor(finalDamages[2] * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            wk2 = Math.floor((finalDamages[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            // myInnerHtml("ATK_02", (finalDamages[2] + wk2 + n_A_EDP_DMG[2]) + "(" + (finalDamages[2] + n_A_EDP_DMG[2]) + "+" + wk2 + ")", 0);
            battleResult.atk02 = (finalDamages[2] + wk2 + n_A_EDP_DMG[2]) + "(" + (finalDamages[2] + n_A_EDP_DMG[2]) + "+" + wk2 + ")";

            finalDamages[2] += wk;


            finalDamages[2] = BattleCalc3(stats, targetStats, finalDamages[2], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[0], 0, InWarOfEmperium);
            wk = Math.floor(finalDamages[0] * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            wk2 = Math.floor((finalDamages[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            // myInnerHtml("ATK_00", (finalDamages[0] + wk2 + n_A_EDP_DMG[0]) + "(" + (finalDamages[0] + n_A_EDP_DMG[0]) + "+" + wk2 + ")", 0);
            battleResult.atk00 = (finalDamages[0] + wk2 + n_A_EDP_DMG[0]) + "(" + (finalDamages[0] + n_A_EDP_DMG[0]) + "+" + wk2 + ")";
            finalDamages[0] += wk;

            finalDamages[0] = BattleCalc3(stats, targetStats, finalDamages[0], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[1], 1, InWarOfEmperium);
            wk = Math.floor(finalDamages[1] * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            wk2 = Math.floor((finalDamages[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch("Double Attack", stats) * 0.02));
            // myInnerHtml("ATK_01", (finalDamages[1] + wk2 + n_A_EDP_DMG[1]) + "(" + (finalDamages[1] + n_A_EDP_DMG[1]) + "+" + wk2 + ")", 0);
            battleResult.atk01 = (finalDamages[1] + wk2 + n_A_EDP_DMG[1]) + "(" + (finalDamages[1] + n_A_EDP_DMG[1]) + "+" + wk2 + ")";
            finalDamages[1] += wk;

            finalDamages[1] = BattleCalc3(stats, targetStats, finalDamages[1], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            let battleVariousResult = BattleVariousResults(stats, targetStats, 0, 0, finalDamages, InWarOfEmperium);
            battleResult = Object.assign(battleResult, battleVariousResult);
        } else {
            let wTAKA = 0;
            if (stats.weaponType == WEAPON_TYPE_BOW && SkillSearch("Blitz Beat") && stats.skillToUseName != "Sharp Shooting (Temp)", stats) {
                // myInnerHtml("bSUBname", "Bird Damage (Atk Rate))", 0);
                battleResult.bonusSubName = "Bird Damage (Atk Rate))";
                wBTw1 = Math.floor((stats.jobLevel - 1) / 10 + 1);
                if (wBTw1 > 5) wBTw1 = 5;
                let wBTw2 = SkillSearch("Blitz Beat", stats);
                if (wBTw2 < wBTw1)
                    wBTw1 = wBTw2;
                let wBT = 80 + Math.floor(stats.dex / 10) * 2 + Math.floor(stats.int / 2) * 2 + SkillSearch("Steel Crow", stats) * 6;
                wBT = Math.floor(wBT * element[targetStats.element][0]);
                wBT = tPlusDamCut(stats, targetStats, wBT, InWarOfEmperium);
                let wBTw3 = Math.round((1 + stats.luk * 0.3) * 100) / 100;
                if (targetStats.mobIndex == 44)
                    wBT = 0;
                // myInnerHtml("bSUB", wBT * wBTw1 + "(" + wBTw3 + "%)", 0);
                battleResult.bonusSub = wBT * wBTw1 + "(" + wBTw3 + "%)";
                wBT = wBT * wBTw1 * wBTw3 / 100;
                wBT = wBT * (hitRate + ((100 - hitRate) * criticalRate / 100)) / 100;
                wBTw1 = 0;
                wTAKA = Math.round(wBT * 100) / 100;
            }
            TyouEnkakuSousa3dan = 0;

            if (SkillSearch("Raging Trifecta Blow", stats)) {
                TyouEnkakuSousa3dan = -1;

                // myInnerHtml("bSUBname", "Trifecta Damage", 0);
                battleResult.bonusSubName = "Trifecta Damage";
                san1 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[0] * (skillModifier + wBC3_3danAtkBairitu, InWarOfEmperium), 0) / 3) * 3;
                san2 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[1] * (skillModifier + wBC3_3danAtkBairitu, InWarOfEmperium), 1) / 3) * 3;
                san3 = Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[2] * (skillModifier + wBC3_3danAtkBairitu, InWarOfEmperium), 2) / 3) * 3;
                // myInnerHtml("bSUB", san1 + " ~ " + san3, 0);
                // myInnerHtml("bSUB2name", "Trifecta Rate", 0);
                // myInnerHtml("bSUB2", 30 - SkillSearch("Raging Trifecta Blow", stats) + "%", 0);
                battleResult.bonusSub = san1 + " ~ " + san3;
                battleResult.bonusSub2Name = "Trifecta Rate";
                battleResult.bonusSub2 = 30 - SkillSearch("Raging Trifecta Blow", stats) + "%";
                TyouEnkakuSousa3dan = 0;
            }

            ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);

            finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[2], 2, InWarOfEmperium);
            if (SkillSearch("Raging Trifecta Blow", stats))
                TyouEnkakuSousa3dan = san3;
            battleResult.atk02 = (finalDamages[2] + n_A_EDP_DMG[2]) + "";
            // myInnerHtml("ATK_02", (finalDamages[2] + n_A_EDP_DMG[2]), 0);


            finalDamages[2] = BattleCalc3(stats, targetStats, finalDamages[2], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[2] += wTAKA;
            finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

            finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[0], 0, InWarOfEmperium);
            // myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0], 0);
            battleResult.atk00 = (finalDamages[0] + n_A_EDP_DMG[0]) + "";
            if (SkillSearch("Raging Trifecta Blow", stats))
                TyouEnkakuSousa3dan = san1;

            finalDamages[0] = BattleCalc3(stats, targetStats, finalDamages[0], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[0] += wTAKA;
            finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

            finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[1], 1, InWarOfEmperium);
            // myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1], 0);
            battleResult.atk01 = (finalDamages[1] + n_A_EDP_DMG[1]) + "";
            if (SkillSearch("Raging Trifecta Blow", stats))
                TyouEnkakuSousa3dan = san2;

            finalDamages[1] = BattleCalc3(stats, targetStats, finalDamages[1], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
            finalDamages[1] += wTAKA;
            finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

            let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
            battleResult = Object.assign(battleResult, castAndDelayBattleResult);
            let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
            battleResult = Object.assign(battleResult, battleVariousResult);
        }
        return;
    } else if (stats.skillToUseName === "Sharp Shooting (Temp)") {
        isRangedAttack = 1;
        battleResult.critAtkName = "Defence Bypassing Damage";
        battleResult.critChanceName = "Chance to Bypass Defence";
        // myInnerHtml("CRIATKname", "Defence Bypassing Damage", 0);
        // myInnerHtml("CRInumname", "Chance to Bypass Defence", 0);

        skillModifier += (1 + 0.5 * stats.skillToUseLV);
        wCast = 2 * stats.cast;
        wDelay = 1.5;
        swDelay = 1;


        stats.critATK[1] = n_A_DMG[1];
        stats.critATK[0] = n_A_DMG[0];
        stats.critATK[2] = n_A_DMG[2];

        ApplySkillModifier(stats, n_A_DMG, skillModifier, 1);

        wCriTyuu = 1;
        stats.critATK[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[1], 10, InWarOfEmperium);
        stats.critATK[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[0], 10, InWarOfEmperium);
        stats.critATK[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, stats.critATK[2], 10, InWarOfEmperium);
        wCriTyuu = 0;


        n_A_EDP_DMG[2] = BattleCalcEDP(stats, targetStats, n_A_DMG[2], 0);
        n_A_EDP_DMG[0] = BattleCalcEDP(stats, targetStats, n_A_DMG[0], 2);
        n_A_EDP_DMG[1] = BattleCalcEDP(stats, targetStats, n_A_DMG[1], 3);

        // myInnerHtml("CRIATK", (stats.critATK[0] + n_A_EDP_DMG[0]) + " ~ " + (stats.critATK[2] + n_A_EDP_DMG[2]), 0);
        battleResult.critAtk[0] = stats.critATK[0] + n_A_EDP_DMG[0];
        battleResult.critAtk[1] = stats.critATK[2] + n_A_EDP_DMG[2];


        stats.critATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
        stats.critATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
        stats.critATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);

        finalDamages[2] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[2], 2, InWarOfEmperium);
        // myInnerHtml("ATK_02", (finalDamages[2] + n_A_EDP_DMG[2]), 0);
        battleResult.atk02 = (finalDamages[2] + n_A_EDP_DMG[2]) + "";


        finalDamages[2] = BattleCalc3(stats, targetStats, finalDamages[2], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
        finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

        finalDamages[0] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[0], 0, InWarOfEmperium);
        // myInnerHtml("ATK_00", finalDamages[0] + n_A_EDP_DMG[0], 0);
        battleResult.atk00 = (finalDamages[0] + n_A_EDP_DMG[0]) + "";

        finalDamages[0] = BattleCalc3(stats, targetStats, finalDamages[0], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
        finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

        finalDamages[1] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[1], 1, InWarOfEmperium);
        // myInnerHtml("ATK_01", finalDamages[1] + n_A_EDP_DMG[1], 0);
        battleResult.atk01 = (finalDamages[1] + n_A_EDP_DMG[1]) + "";

        finalDamages[1] = BattleCalc3(stats, targetStats, finalDamages[1], InWarOfEmperium, w998B, w998E, w998G, w998I, w998L);
        finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        return;
    }

    // 6 -> bash
    // 7 -> magnum break
    let activeSkillIds = [6, 7, 19, 41, 44, 65, 71, 72, 73, 83, 84, 158, 161, 169, 171, 188, 189, 199, 207, 248, 260, 261, 264, 288, 289, 290, 292, 302, 303, 326, 331, 333, 335, 337, 339, 382, 388, 348, 349, 350, 419, 423, 428, 429, 430, 431, 432, 434, 435, 436, 437, "NULL"];
    let activeSkillInUseIndex;
    let wActiveHitNum = 0;
    for (activeSkillInUseIndex = 0; activeSkillIds[activeSkillInUseIndex] != stats.skillToUse && activeSkillIds[activeSkillInUseIndex] != "NULL"; activeSkillInUseIndex++) ;
    if (stats.skillToUse === activeSkillIds[activeSkillInUseIndex]) {
        wActiveHitNum = 1;
        if (stats.skillToUseName === "Bash")
            skillModifier += stats.skillToUseLV * 0.3;
        else if (stats.skillToUseName === "Solar Heat" || stats.skillToUseName === "Lunar Heat" || stats.skillToUseName === "Stellar Heat") {
            //Heat

            wDelay = 0.1;

        } else if (stats.skillToUseName === "Magnum Break") {
            skillModifier += stats.skillToUseLV * 0.2;
            stats.weapon1Element = 3;
            wDelay = 2;
            swDelay = 1;
        } else if (stats.skillToUseName === "Sand Attack") {
            not_use_card = 1;
            skillModifier += 0.3;
            stats.weapon1Element = 2;
        } else if (stats.skillToUseName === "Arrow Shower") {
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 0.05 - 0.25;
            wDelay = 1;
            swDelay = 1;
        } else if (stats.skillToUseName === "Arrow Repel") {
            isRangedAttack = 1;
            wCast = 1.5;
            skillModifier += 0.5;
        } else if (stats.skillToUseName === "Mammonite")
            skillModifier += stats.skillToUseLV * 0.5;
        else if (stats.skillToUseName === "Spear Stab") {
            skillModifier += stats.skillToUseLV * 0.2;
            isRangedAttack = 1;
        } else if (stats.skillToUseName === "Grimtooth") {
            if (stats.skillToUseLV >= 3)
                isRangedAttack = 1;
            skillModifier += 0.2 * stats.skillToUseLV;


        } else if (stats.skillToUseName === "Smite") {
            skillModifier += stats.skillToUseLV * 0.2;

        } else if (stats.skillToUseName === "Holy Cross") {
            skillModifier += stats.skillToUseLV * 0.35;
            stats.weapon1Element = 6;
        } else if (stats.skillToUseName === "Sightless Mind")
            skillModifier += stats.skillToUseLV * 0.4;
        else if (stats.skillToUseName === "Spear Boomerang") {
            skillModifier += stats.skillToUseLV * 0.5;
            wDelay = 1;
            swDelay = 1;
            isRangedAttack = 1;
        } else if (stats.skillToUseName === "Brandish Spear") {
            w = (1 + stats.skillToUseLV * 0.2);
            if (stats.skillToUseLV == 10) skillModifier += 4.625;
            else if (stats.skillToUseLV >= 7) skillModifier += w + w / 2 + w / 4 - 1;
            else if (stats.skillToUseLV >= 4) skillModifier += w + w / 2 - 1;
            else skillModifier += w - 1;
            wCast = 0.7;
        } else if (stats.skillToUseName === "Sonic Blow" || stats.skillToUseName === "Sonic Blow (Soul Linked)") {
            wActiveHitNum = 8;
            skillModifier += stats.skillToUseLV * 0.5 + 2;
            if (stats.skillToUseName === "Sonic Blow (Soul Linked)" && InWarOfEmperium == 0)
                skillModifier *= 2;
            if (stats.skillToUseName === "Sonic Blow (Soul Linked)" && InWarOfEmperium == 1)
                skillModifier *= 1.25;
            wDelay = 2;
            swDelay = 2;
        } else if (stats.skillToUseName === "Back Stab") {
            skillModifier += stats.skillToUseLV * 0.4 + 2;
            wDelay = 0.5;
            swDelay = 1;
            w_HIT = 100;
            // myInnerHtml("BattleHIT", 100, 0);
            battleResult.battleHit = 100;
        } else if (stats.skillToUseName === "Raging Quadruple Blow") {
            wActiveHitNum = 4;
            skillModifier += 0.5 + stats.skillToUseLV * 0.5;
            n_SpSkill = 1;
        } else if (stats.skillToUseName === "Raging Thrust") {
            skillModifier += 1.4 + stats.skillToUseLV * 0.6;
            n_SpSkill = 1;
        } else if (stats.skillToUseName === "Melody Strike" || stats.skillToUseName === "Slinging Arrow") {
            wCast = 1.5;
            skillModifier += (stats.skillToUseLV * 0.4 - 0.4);
            stats.weapon1Element = ArrowOBJ[stats.arrow][1];
            if (eval(document.calcForm.A_Weapon_element.value) != 0)
                stats.weapon1Element = eval(document.calcForm.A_Weapon_element.value);
            isRangedAttack = 1;
        } else if (stats.skillToUseName === "Bomb") {
            not_use_card = 1;
            stats.weapon1Element = 3;
            n_SpSkill = 1;
            wCast = 1;
            skillModifier += stats.skillToUseLV * 0.2;
            w_HIT = 100;
            // myInnerHtml("BattleHIT", 100, 0);
            battleResult.battleHit = 100;
        } else if (stats.skillToUseName === "Traumatic Blow") {
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 0.4;
            wDelay = 0.5;
            swDelay = 1;
        } else if (stats.skillToUseName === "Vital Strike") {
            isRangedAttack = 1;
            skillModifier += (stats.skillToUseLV * 0.1 - 0.5);
            if (stats.skillToUseLV > 5)
                wDelay = 1;
            else
                wDelay = 0.8;
            swDelay = 1;
        } else if (stats.skillToUseName === "Meteor Assault") {
            not_use_card = 1;
            skillModifier += (stats.skillToUseLV * 0.4 - 0.6);
            wCast = 0.5;
            wDelay = 0.5;
            swDelay = 1;
        } else if (stats.skillToUseName === "Raging Palm Strike") {
            skillModifier += (1 + stats.skillToUseLV);
            wDelay = 0.3;
            swDelay = 1;
        } else if (stats.skillToUseName === "Glacier Fist") {
            n_SpSkill = 1;
            skillModifier += stats.skillToUseLV - 0.6;


        } else if (stats.skillToUseName === "Chain Crush Combo") {
            n_SpSkill = 1;
            skillModifier += (3 + stats.skillToUseLV);
            if (stats.skillToUseLV > 6) wDelay = 1;
            else wDelay = 0.8;
            swDelay = 1;
        } else if (stats.skillToUseName === "Arrow Vulcan") {
            wActiveHitNum = 9;
            skillModifier += 1 + stats.skillToUseLV;
            stats.weapon1Element = ArrowOBJ[stats.arrow][1];
            if (eval(document.calcForm.A_Weapon_element.value) != 0)
                stats.weapon1Element = eval(document.calcForm.A_Weapon_element.value);
            isRangedAttack = 1;
            wCast = 1.8 + stats.skillToUseLV * 0.2;
            if (stats.skillToUseLV >= 6) wDelay = 1;
            else wDelay = 0.8;
            wDelay = 3;
            swDelay = 2;
        } else if (stats.skillToUseName === "Tomahawk Throwing") {
            isRangedAttack = 1;
            not_use_card = 1;
            stats.weapon1Element = 4;
        } else if (stats.skillToUseName === "Pulse Strike (Temp)") {
            skillModifier += (stats.skillToUseLV - 1) * 1;
        } else if (stats.skillToUseName === "High Speed Cart Ram") {
            not_use_card = 1;
            skillModifier += Math.floor((eval(document.calcForm.SkillSubNum.value) / (16 - stats.skillToUseLV) / 100 - 1) * 100) / 100;
        } else if (stats.skillToUseName === "Excruciating Palm") {
            not_use_card = 1;
            skillModifier += 2;


        } else if (stats.skillToUseName === "Tornado Kick" || stats.skillToUseName === "Heel Drop") {
            n_SpSkill = 1;
            skillModifier += (0.6 + stats.skillToUseLV * 0.2);
        } else if (stats.skillToUseName === "Roundouse" || stats.skillToUseName === "Counter Kick") {
            n_SpSkill = 1;
            skillModifier += (0.9 + stats.skillToUseLV * 0.3);
            if (stats.skillToUseName === "Counter Kick")
                wActiveHitNum = 3;
        } else if (stats.skillToUseName === "Flying Kick (Normal)") {
            n_SpSkill = 1;
            skillModifier += (-0.7 + stats.skillToUseLV * 0.1);
        } else if (stats.skillToUseName === "Bull's Eye") {
            not_use_card = 1;
            wCast = 0.5;
            isRangedAttack = 1;
            wActiveHitNum = 5;
            if (targetStats.race == 2 || targetStats.race == 7)
                skillModifier += 4;
        } else if (stats.skillToUseName === "Magical Bullet") {
            isRangedAttack = 1;
            stats.weapon1Element = 8;
            not_use_card = 1;
        } else if (stats.skillToUseName === "Trigger Happy Shot") {
            isRangedAttack = 1;
            wActiveHitNum = 5;
            skillModifier += stats.skillToUseLV * 0.5 + 4;
            wDelay = 1.7;
            swDelay = 1;
        } else if (stats.skillToUseName === "Desperado (Single Hit)") {
            isRangedAttack = 0;
            skillModifier += stats.skillToUseLV * 0.5 - 0.5;
            wDelay = 1;
            swDelay = 1;
        } else if (stats.skillToUseName === "Tracking") {
            wCast = 1 + 0.2 * stats.skillToUseLV;
            cast_kotei = 1;
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 1 + 1;
            wDelay = 1;
            swDelay = 1;
            w_HIT = hitRate * 5 + 5;
            if (w_HIT > 100)
                w_HIT = 100;
        } else if (stats.skillToUseName === "Disarm") {
            wCast = 2;
            isRangedAttack = 1;
        } else if (stats.skillToUseName === "Wounding Shot") {
            wCast = 1.5;
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 0.2;
            wDelay = 0;
            swDelay = 1;
            w_HIT = 100;
        } else if (stats.skillToUseName === "Crowd Control Shot") {
            cast_kotei = 1;
            wCast = 1;
            isRangedAttack = 0;
            skillModifier += stats.skillToUseLV * 0.5;
            wDelay = 1;
            swDelay = 2
            w_HIT = 100;
        } else if (stats.skillToUseName === "Full Blast") {
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 1 + 2;
            wDelay = 1 + stats.skillToUseLV * 0.2;
            swDelay = 1;
        } else if (stats.skillToUseName === "Spread Shot") {
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 0.2 - 0.2;
            wDelay = "(Unknown)";
            swDelay = 1;
        } else if (stats.skillToUseName === "Gunslinger Mine") {
            isRangedAttack = 1;
            not_use_card = 1;
            wCast = 1;
            wDelay = 1;
            swDelay = 1;
            w_HIT = 100;
        }


        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);


        n_A_EDP_DMG[0] = BattleCalcEDP(stats, targetStats, n_A_DMG[0], 0);
        n_A_EDP_DMG[1] = BattleCalcEDP(stats, targetStats, n_A_DMG[1], 1);
        n_A_EDP_DMG[2] = BattleCalcEDP(stats, targetStats, n_A_DMG[2], 2);

        if (cast_kotei == 0)
            wCast = wCast * stats.cast;

        let finalDamagesCopy = [0, 0, 0];
        for (let b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            if (wActiveHitNum > 1)
                finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = (finalDamagesCopy[0] + n_A_EDP_DMG[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1] + n_A_EDP_DMG[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2] + n_A_EDP_DMG[2]) + "";

        if (cast_kotei == 0) {
            let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
            battleResult = Object.assign(battleResult, castAndDelayBattleResult);
            let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
            battleResult = Object.assign(battleResult, battleVariousResult);
        }
    } else if (stats.skillToUseName === "Stave Crasher") {
        isRangedAttack = 1;
        wCast = 0.3;
        wDelay = 0.3;
        swDelay = 1;
        n_A_DMG[2] = n_A_MATK[2];
        n_A_DMG[0] = n_A_MATK[0];
        n_A_DMG[1] = (n_A_MATK[0] + n_A_MATK[2]) / 2;


        for (b = 0; b <= 2; b++)
            n_A_EDP_DMG[b] = BattleCalcEDP(stats, targetStats, n_A_DMG[b], b);

        wCast = wCast * stats.cast;

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + (ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) + stats.weapon1LV_upgradeBonusATK) * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = (finalDamagesCopy[0] + n_A_EDP_DMG[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1] + n_A_EDP_DMG[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2] + n_A_EDP_DMG[2]) + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Double Strafe" || stats.skillToUseName === "Pierce" || stats.skillToUseName === "Throw Spirit Spheres (# Hits = # of Spirit Spheres)" || stats.skillToUseName === "Bowling Bash" || stats.skillToUseName === "Triple Action" || stats.skillToUseName === "Beast Strafing") {
        var isBowlingBash = false;
        if (stats.skillToUseName === "Double Strafe") { // double strafe
            isRangedAttack = 1;
            skillModifier += stats.skillToUseLV * 0.1 - 0.1;
            hitCount = 2;
        } else if (stats.skillToUseName === "Pierce") {
            skillModifier += stats.skillToUseLV * 0.1;
            hitCount = targetStats.size + 1;
        } else if (stats.skillToUseName === "Bowling Bash") {
            skillModifier += stats.skillToUseLV * 0.4;
            wCast = 0.7 * stats.cast;
            hitCount = 2;
            if (stats.skillToUseLV == 1)
                hitCount = 1;
            isBowlingBash = true;
            if (targetStats.lexAeterna == 1) {
                hitCount = 3;
                if (stats.skillToUseLV == 1)
                    hitCount = 2;
            }
        } else if (stats.skillToUseName === "Throw Spirit Spheres (# Hits = # of Spirit Spheres)") {
            skillModifier += stats.skillToUseLV * 0.5;
            if (stats.job == 15 || stats.job == 29)
                w = SkillSearch("Summon Spirit Sphere", stats);
            else
                w = stats.supportiveSkills[10];
            if (w > stats.skillToUseLV) {
                w = stats.skillToUseLV;
            }
            hitCount = w;
            wCast = (1 + w) * stats.cast;
            wDelay = 0.5;
            swDelay = 1;
            isRangedAttack = 1;
        } else if (stats.skillToUseName === "Triple Action") {
            isRangedAttack = 1;
            skillModifier += 0.5;
            hitCount = 3;
        } else if (stats.skillToUseName === "Beast Strafing") {
            n_SpSkill = 1;
            isRangedAttack = 1;
            skillModifier += stats.str * 0.08 - 0.5;
            hitCount = 2;
        }


        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] += n_A_EDP_DMG[b];
            if (stats.skillToUseName === "Beast Strafing" && targetStats.race != 2 && targetStats.race != 4)
                finalDamages[b] = 0;
            finalDamagesCopy[b] = finalDamages[b];
            // if (targetStats.lexAeterna == 0 || !isBowlingBash)
            //     myInnerHtml("ATK_0" + b, finalDamages[b] * hitCount + "(" + finalDamages[b] + SubName[8] + hitCount + "hit)", 0);
            // else
            //     myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] * 2 + " + " + finalDamages[b] + ")", 0);
            finalDamages[b] -= n_A_EDP_DMG[b];
            finalDamages[b] *= hitCount;
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * hitCount * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]) * hitCount;
        }
        if (targetStats.lexAeterna == 0 || !isBowlingBash) {
            battleResult.atk00 = (finalDamagesCopy[0] * hitCount + "(" + finalDamagesCopy[0] + SubName[8] + hitCount + "hit)") + "";
            battleResult.atk01 = (finalDamagesCopy[1] * hitCount + "(" + finalDamagesCopy[1] + SubName[8] + hitCount + "hit)") + "";
            battleResult.atk02 = (finalDamagesCopy[2] * hitCount + "(" + finalDamagesCopy[2] + SubName[8] + hitCount + "hit)") + "";
        } else {
            battleResult.atk00 = finalDamagesCopy[0] * 3 + "(" + finalDamagesCopy[0] * 2 + " + " + finalDamagesCopy[0] + ")";
            battleResult.atk01 = finalDamagesCopy[1] * 3 + "(" + finalDamagesCopy[1] * 2 + " + " + finalDamagesCopy[1] + ")";
            battleResult.atk02 = finalDamagesCopy[2] * 3 + "(" + finalDamagesCopy[2] * 2 + " + " + finalDamagesCopy + ")";
        }

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Blitz Beat" || stats.skillToUseName === "Falcon Eyes") {
        stats.weapon1Element = 0;
        isRangedAttack = 1;
        wBT = 80 + Math.floor(stats.dex / 10) * 2 + Math.floor(stats.int / 2) * 2 + SkillSearch("Steel Crow", stats) * 6;
        if (stats.skillToUseName === "Falcon Eyes") {
            wBT = Math.floor(wBT * (150 + 70 * stats.skillToUseLV) / 100);
            wBT = Math.floor(wBT * element[targetStats.element][0]);
            wBT = tPlusDamCut(stats, targetStats, wBT, InWarOfEmperium);
            wBT *= 5;
            wCast = 1 * stats.cast;
            wDelay = 3;
        } else {
            wBT = Math.floor(wBT * element[targetStats.element][0]);
            wBT = tPlusDamCut(stats, targetStats, wBT, InWarOfEmperium);
            wBT *= stats.skillToUseLV;
            wCast = 1.5 * stats.cast;
            wDelay = 1;
        }
        swDelay = 1;
        // myInnerHtml("ATK_02", wBT, 0);
        // myInnerHtml("ATK_00", wBT, 0);
        // myInnerHtml("ATK_01", wBT, 0);
        battleResult.atk00 = wBT;
        battleResult.atk01 = wBT;
        battleResult.atk02 = wBT;
        finalDamages[0] = finalDamages[2] = finalDamages[1] = wBT;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Envenom" || (stats.skillToUseName === "Poison React (Counter)" && (targetStats.element < 50 || 60 <= targetStats.element))) {
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
        stats.weapon1Element = 5;


        n_A_EDP_DMG[2] = BattleCalcEDP(stats, targetStats, n_A_DMG[2], 2);
        n_A_EDP_DMG[0] = BattleCalcEDP(stats, targetStats, n_A_DMG[0], 0);
        n_A_EDP_DMG[1] = BattleCalcEDP(stats, targetStats, n_A_DMG[1], 1);

        wINV = Math.floor(ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * element[targetStats.element][5]);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][5]);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + wINV * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = (finalDamagesCopy[0] + n_A_EDP_DMG[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1] + n_A_EDP_DMG[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2] + n_A_EDP_DMG[2]) + "";

        // myInnerHtml("bSUBname", '<Font color="#0000FF">Poison Damage</Font>', 0);
        // myInnerHtml("bSUB", '<Font color="#0000FF">' + wINV + "</Font>", 0);
        battleResult.bonusSubName = "Poison Damage";
        battleResult.bonusSub = wINV + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Shield Boomerang" || stats.skillToUseName === "Shield Boomerang (SoulLinked)") {
        isRangedAttack = 1;
        stats.weapon1Element = 0;
        wDelay = 0.7;
        if (stats.skillToUseName === "Shield Boomerang (SoulLinked)")
            wDelay = 0.35;
        swDelay = 1;
        wSBr = stats.shieldHandDefPlus * 4;

        skillModifier2 = (1 + stats.skillToUseLV * 0.3);
        if (stats.skillToUseName === "Shield Boomerang (SoulLinked)")
            skillModifier2 *= 2;

        baseATK_w = Math.round(Math.floor(stats.str / 10) * Math.floor(stats.str / 10));
        stats.baseATK = stats.str + baseATK_w + Math.floor(stats.dex / 5) + Math.floor(stats.luk / 5);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = stats.baseATK * skillModifier + ItemOBJ[stats.equipments[5]][6] + wSBr;
            finalDamages[b] = Math.floor(Math.floor(finalDamages[b] * (100 - targetStats.def) / 100 - n_B_DEF2[b]) * skillModifier2);
            finalDamages[b] = BaiCI(stats, targetStats, finalDamages[b], InWarOfEmperium);
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate) / 100;
        }
        battleResult.atk00 = (finalDamagesCopy[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2]) + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Rapid Smiting") {
        isRangedAttack = 1;
        stats.weapon1Element = 0;
        wCast = 1 * stats.cast;
        wDelay = 1;
        swDelay = 1;
        wSBr = stats.shieldHandDefPlus;
        wSC = ItemOBJ[stats.equipments[5]][6];

        skillModifier2 = (1 + stats.skillToUseLV * 0.3);

        baseATK_w = Math.round(Math.floor(stats.str / 10) * Math.floor(stats.str / 10));
        stats.baseATK = stats.str + baseATK_w + Math.floor(stats.dex / 5) + Math.floor(stats.luk / 5);
        stats.baseATK = stats.baseATK * skillModifier + wSC + wSBr * 4;

        wSC -= 100;
        if (wSC < 0)
            wSC = 0;
        wSC2 = [0, 0, 0];
        wSC2[2] = 100 + wSC + (wSBr * 2) * wSBr;
        wSC2[1] = 100 + (wSC + (wSBr * 2) * wSBr) / 2;
        wSC2[0] = 100

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = (stats.baseATK * (100 - targetStats.def) / 100 - n_B_DEF2[b]) * skillModifier2;
            finalDamages[b] += wSC2[b];
            finalDamages[b] = BaiCI(stats, targetStats, finalDamages[b], InWarOfEmperium);
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] * 5 + "(" + finalDamages[b] + SubName[8] + "5hit)", 0);
            finalDamages[b] *= 5;
            finalDamages[b] = (finalDamages[b] * hitRate) / 100;
        }
        battleResult.atk00 = (finalDamagesCopy[0] * 5 + "(" + finalDamagesCopy[0] + SubName[8] + "5hit)") + "";
        battleResult.atk01 = (finalDamagesCopy[1] * 5 + "(" + finalDamagesCopy[1] + SubName[8] + "5hit)") + "";
        battleResult.atk02 = (finalDamagesCopy[2] * 5 + "(" + finalDamagesCopy[2] + SubName[8] + "5hit)") + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Clashing Spiral") {
        isRangedAttack = 1;
        if (stats.skillToUseLV == 5)
            wCast = 1 * stats.cast;
        else
            wCast = (0.1 + 0.2 * stats.skillToUseLV) * stats.cast;
        wDelay = 1 + 0.2 * stats.skillToUseLV;
        swDelay = 1;

        wSPP = Math.floor(stats.str / 10);
        finalDamages[2] = wSPP * wSPP + ItemOBJ[stats.equipments[0]][6] * 0.8 * (1 + 0.5 * stats.skillToUseLV);
        wSPP = 1.25 - (targetStats.size * 0.25);
        finalDamages[2] = Math.floor(finalDamages[2] * wSPP + stats.weapon1LV_upgradeBonusATK);
        finalDamages[2] = finalDamages[2] * element[targetStats.element][stats.weapon1Element];
        finalDamages[2] = BaiCI(stats, targetStats, finalDamages[2], InWarOfEmperium);
        // myInnerHtml("ATK_00", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        // myInnerHtml("ATK_01", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        // myInnerHtml("ATK_02", finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)", 0);
        battleResult.atk00 = finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)";
        battleResult.atk01 = finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)";
        battleResult.atk02 = finalDamages[2] * 5 + "(" + finalDamages[2] + SubName[8] + 5 + "hit)";
        finalDamages[2] *= 5;
        wSPP2 = stats.weapon1LV_upgradeBonusATK * element[targetStats.element][stats.weapon1Element];
        wSPP2 = BaiCI(stats, targetStats, wSPP2, InWarOfEmperium);
        wSPP2 = tPlusDamCut(stats, targetStats, wSPP2, InWarOfEmperium);
        finalDamages[2] = finalDamages[2] * hitRate / 100 + wSPP2 * 5 * (100 - hitRate) / 100;


        finalDamages[0] = finalDamages[1] = finalDamages[2];

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Venom Splasher") {
        not_use_card = 1;
        n_SpSkill = 1;
        wCast = 1 * stats.cast;

        if (targetStats.isNormal) {

            skillModifier += (400 + 50 * stats.skillToUseLV + 20 * eval(document.calcForm.SkillSubNum.value)) / 100;
            ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);


            let finalDamagesCopy = [0, 0, 0];
            for (b = 0; b <= 2; b++) {
                finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
                finalDamages[b] = Math.floor(finalDamages[b]);
                finalDamagesCopy[b] = finalDamages[b];
                // myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            }
            battleResult.atk00 = (finalDamagesCopy[0]) + "";
            battleResult.atk01 = (finalDamagesCopy[1]) + "";
            battleResult.atk02 = (finalDamagesCopy[2]) + "";
        }
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Soul Destroyer") {
        not_use_card = 1;
        isRangedAttack = 1;
        wCast = 0.5 * stats.cast;
        wDelay = 0.8 + 0.2 * stats.skillToUseLV;
        swDelay = 1;

        w_SBr = new Array();
        w = stats.int * 5 * stats.skillToUseLV;
        w_SBr[2] = w + 1000 - Math.floor((targetStats.def + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[2]) / 2);
        w_SBr[1] = w + 750 - Math.floor((targetStats.def + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[1]) / 2);
        w_SBr[0] = w + 500 - Math.floor((targetStats.def + targetStatsArray[TARGET_STAT_MDEF] + n_B_MDEF2 + n_B_DEF2[0]) / 2);
        for (i = 0; i <= 2; i++)
            w_SBr[i] = tPlusDamCut(stats, targetStats, w_SBr[i], InWarOfEmperium);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] *= stats.skillToUseLV;
            // myInnerHtml("ATK_0" + b, finalDamages[b] + w_SBr[b] + "(" + finalDamages[b] + "+" + w_SBr[b] + ")", 0);
            finalDamagesCopy[b] = finalDamages[b];
            finalDamages[b] = ((finalDamages[b] + w_SBr[b]) * hitRate + (ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) + w_SBr[b]) * (100 - hitRate)) / 100;
        }
        battleResult.atk00 = finalDamagesCopy[0] + w_SBr[0] + "(" + finalDamagesCopy[0] + "+" + w_SBr[0] + ")";
        battleResult.atk01 = finalDamagesCopy[1] + w_SBr[1] + "(" + finalDamagesCopy[1] + "+" + w_SBr[1] + ")";
        battleResult.atk02 = finalDamagesCopy[2] + w_SBr[2] + "(" + finalDamagesCopy[2] + "+" + w_SBr[2] + ")";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Grand Cross") {

        // myInnerHtml("CRIATKname", '<Font color="#FF0000">HP Casting Cost</Font>', 0);
        // myInnerHtml("CRIATK", '<Font color="#FF0000">' + Math.floor(stats.maxHp / 5) + "</Font>", 0);
        //
        // myInnerHtml("CRInumname", '<Font color="#FF0000">Reflect Damage</Font>', 0);
        battleResult.critAtkName = "HP Casting Cost";
        battleResult.critAtk[0] = Math.floor(stats.maxHp / 5);
        battleResult.critChanceName = "Reflect Damage";


        wGXhito = 100 - StPlusCard("DAMAGE_INC_DEC_RACE_DEMIHUMAN_PERCENTAGE", stats);
        wGXhito -= StPlusItem("DAMAGE_INC_DEC_RACE_DEMIHUMAN_PERCENTAGE", stats);

        wGXsei = 100 - SkillSearch("Faith", stats) * 5;
        wGXsei -= StPlusCard("DAMAGE_INC_DEC_ELEMENT_HOLY_PERCENTAGE", stats);
        wGXsei -= StPlusItem("DAMAGE_INC_DEC_ELEMENT_HOLY_PERCENTAGE", stats);

        wGXen = StPlusCard("RESISTANCE_RANGE_ATTACK_PERCENTAGE", stats);
        wGXen += StPlusItem("RESISTANCE_RANGE_ATTACK_PERCENTAGE", stats);


        work_A_VITDEF = [0, 0, 0];
        work_A_VITDEF[0] = stats.vitDEF[2];
        work_A_VITDEF[1] = stats.vitDEF[1];
        work_A_VITDEF[2] = stats.vitDEF[0];
        stats.intMDEF = stats.int + Math.floor(stats.vit / 2);

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BK_n_A_DMG[b] * (100 - stats.def) / 100 - work_A_VITDEF[b] + stats.weapon1LV_upgradeBonusATK;
            finalDamages[b] = Math.floor(finalDamages[b] * (skillModifier + stats.skillToUseLV * 0.4));

            w = n_A_MATK[b] * (100 - n_A_MDEF) / 100 - stats.intMDEF;
            w = Math.floor(w * (stats.skillToUseLV * 0.4 + 1));

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
        battleResult.critChance = finalDamages[0] + "~3hit ~ " + finalDamages[1] + "~3hit";
        // myInnerHtml("CRInum", '<Font color="#FF0000">' + finalDamages[0] + "�~3hit ~ " + finalDamages[1] + "�~3hit</Font>", 0);


        isRangedAttack = 2;
        stats.weapon1Element = 6;
        wCast = 3 * stats.cast;
        wDelay = 1.5;
        swDelay = 1;

        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BK_n_A_DMG[b] * (100 - targetStats.def) / 100 - n_B_DEF2[b] + stats.weapon1LV_upgradeBonusATK;
            finalDamages[b] *= skillModifier + stats.skillToUseLV * 0.4;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][6]);
            w = n_A_MATK[b] * (100 - targetStatsArray[TARGET_STAT_MDEF]) / 100 - n_B_MDEF2;
            w *= (stats.skillToUseLV * 0.4 + 1);
            w = Math.floor(w * element[targetStats.element][6]);
            finalDamages[b] = tPlusDamCut(stats, targetStats, Math.floor((w + finalDamages[b], InWarOfEmperium) * element[targetStats.element][6]));
            if (finalDamages[b] < 1) finalDamages[b] = 1;
            if (60 <= targetStats.element && targetStats.element <= 69) finalDamages[b] = 0;
        }


        let finalDamagesCopy = [0, 0, 0];
        if (targetStats.lexAeterna == 0) {
            for (b = 0; b <= 2; b++) {
                // myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] + SubName[8] + "3hit)", 0);
                finalDamagesCopy[b] = finalDamages[b];
                finalDamages[b] *= 3;
            }
            battleResult.atk00 = finalDamagesCopy[0] * 3 + "(" + finalDamagesCopy[0] + SubName[8] + "3hit)";
            battleResult.atk01 = finalDamagesCopy[1] * 3 + "(" + finalDamagesCopy[1] + SubName[8] + "3hit)";
            battleResult.atk02 = finalDamagesCopy[2] * 3 + "(" + finalDamagesCopy[2] + SubName[8] + "3hit)";
        } else {
            for (b = 0; b <= 2; b++) {
                // myInnerHtml("ATK_0" + b, finalDamages[b] * 4 + "(" + finalDamages[b] * 2 + " + " + finalDamages[b] + SubName[8] + "2hit)", 0);
                finalDamagesCopy[b] = finalDamages[b];
                finalDamages[b] *= 4;
            }
            battleResult.atk00 = finalDamagesCopy[0] * 4 + "(" + finalDamagesCopy[0] * 2 + " + " + finalDamagesCopy[0] + SubName[8] + "2hit)";
            battleResult.atk01 = finalDamagesCopy[1] * 4 + "(" + finalDamagesCopy[1] * 2 + " + " + finalDamagesCopy[1] + SubName[8] + "2hit)";
            battleResult.atk02 = finalDamagesCopy[2] * 4 + "(" + finalDamagesCopy[2] * 2 + " + " + finalDamagesCopy[2] + SubName[8] + "2hit)";
        }
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);

        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Cart Revolution") {
        wCR = 100;

        if (SkillSearch("Maximum Power-Thust", stats)) {
            wCR += 20 * SkillSearch("Maximum Power-Thust", stats);
        } else {
            if (SkillSearch("Power-Thrust", stats))
                wCR += SkillSearch("Power-Thrust", stats) * 5;
            if (SkillSearch("Power-Thrust", stats) == 0 && stats.supportiveSkills[8])
                wCR += stats.supportiveSkills[8] * 5 / 10;
        }
        CR_n_A_DMG = [0, 0, 0];

        CRbai = eval(document.calcForm.SkillSubNum.value) / 8000;
        for (b = 0; b <= 2; b++)
            CR_n_A_DMG[b] = Math.floor(n_A_DMG[b] * wCR / 100);

        skillModifier += 0.5;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] += Math.floor(ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, CR_n_A_DMG[b], b, InWarOfEmperium) * CRbai);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b], 0);

            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * 2 * (100 - hitRate)) / 100;
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
        }
        battleResult.atk00 = (finalDamagesCopy[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2]) + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, 0, 0, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Gloria Domini") {
        finalDamages[2] = 500 + 300 * stats.skillToUseLV;
        if (n_Ses)
            finalDamages[2] = Math.floor(finalDamages[2] * 0.6);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        // myInnerHtml("ATK_02", finalDamages[2], 0);
        // myInnerHtml("ATK_00", finalDamages[0], 0);
        // myInnerHtml("ATK_01", finalDamages[1], 0);
        battleResult.atk00 = (finalDamages[0]) + "";
        battleResult.atk01 = (finalDamages[1]) + "";
        battleResult.atk02 = (finalDamages[2]) + "";

        wCast = (1.5 + 0.5 * stats.skillToUseLV) * stats.cast;
        wDelay = 1.5 + stats.skillToUseLV * 0.5;
        swDelay = 1;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Martyr's Reconing") {
        stats.weapon1Element = 0;
        finalDamages[2] = Math.floor(stats.maxHp * 0.09 * (0.9 + 0.1 * stats.skillToUseLV));
        finalDamages[2] = BaiCI(stats, targetStats, finalDamages[2], InWarOfEmperium);
        finalDamages[2] = Math.floor(finalDamages[2] * element[targetStats.element][0]);
        // myInnerHtml("ATK_02", finalDamages[2], 0);
        // myInnerHtml("ATK_00", finalDamages[2], 0);
        // myInnerHtml("ATK_01", finalDamages[2], 0);
        battleResult.atk00 = (finalDamages[2]) + "";
        battleResult.atk01 = (finalDamages[2]) + "";
        battleResult.atk02 = (finalDamages[2]) + "";
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);

        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Occult Impaction") {
        stats.weapon1Element = 0;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
        skillModifier += stats.skillToUseLV * 0.75;


        work_B_DEF2 = [0, 0, 0];
        work_B_DEF2[0] = n_B_DEF2[2];
        work_B_DEF2[1] = n_B_DEF2[1];
        work_B_DEF2[2] = n_B_DEF2[0];

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor(Math.floor(BK_n_A_DMG[b] * skillModifier) * (work_B_DEF2[b] + targetStats.def) / 50);
            finalDamages[b] = BaiCI(stats, targetStats, finalDamages[b], InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            // myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            finalDamagesCopy[b] = finalDamages[b];
        }
        battleResult.atk00 = (finalDamagesCopy[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2]) + "";

        wCast = 1 * stats.cast;
        wDelay = 0.5;
        swDelay = 1;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Guillotine Fist" || stats.skillToUseName === "Guillotine Fist (MaxSP-1)") {
        stats.weapon1Element = 0;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
        if (stats.skillToUseName === "Guillotine Fist")
            skillModifier += 7 + eval(document.calcForm.SkillSubNum.value) / 10;
        else
            skillModifier += 7 + (stats.maxSp - 1) / 10;
        wASYU = 250 + stats.skillToUseLV * 150;

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor(BK_n_A_DMG[b] * skillModifier) + wASYU;
            finalDamages[b] = BaiCI(stats, targetStats, finalDamages[b], InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            // myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            finalDamagesCopy[b] = finalDamages[b];
        }
        battleResult.atk00 = (finalDamagesCopy[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2]) + "";

        wCast = (4.5 - 0.5 * stats.skillToUseLV) * stats.cast;
        wDelay = 3.5 - 0.5 * stats.skillToUseLV;
        swDelay = 1;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Throw Dagger") {
        isRangedAttack = 1;
        not_use_card = 1;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            // myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamagesCopy[b] = finalDamages[b];
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * element[targetStats.element][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = (finalDamagesCopy[0] + n_A_EDP_DMG[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1] + n_A_EDP_DMG[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2] + n_A_EDP_DMG[2]) + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Throw Kunai") {
        isRangedAttack = 1;
        not_use_card = 1;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);


        if (eval(document.calcForm.A_Weapon_element.value) == 0 && stats.weaponType != WEAPON_TYPE_UNARMED && StPlusCard(20) == 0)
            stats.weapon1Element = KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][1];

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] * 3 + "(" + finalDamages[b] + SubName[8] + "3hit)", 0);
            finalDamages[b] = (finalDamages[b] * 3 * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * element[targetStats.element][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = finalDamagesCopy[0] * 3 + "(" + finalDamagesCopy[0] + SubName[8] + "3hit)";
        battleResult.atk01 = finalDamagesCopy[1] * 3 + "(" + finalDamagesCopy[1] + SubName[8] + "3hit)";
        battleResult.atk02 = finalDamagesCopy[2] * 3 + "(" + finalDamagesCopy[2] + SubName[8] + "3hit)";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Throw Huuma Shuriken") {
        skillModifier += (stats.skillToUseLV * 1.5 + 0.5);
        isRangedAttack = 1;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
        wCast = 3 * stats.cast;
        wDelay = 3;
        swDelay = 1;
        wActiveHitNum = 2 + Math.round(stats.skillToUseLV / 2);

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = ApplyMasteryAndWeaponryResearchAndDMGLevel(stats, targetStats, n_A_DMG[b], b, InWarOfEmperium);
            finalDamages[b] = Math.floor(finalDamages[b] * element[targetStats.element][0]);
            if (wActiveHitNum > 1)
                finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
            finalDamagesCopy[b] = finalDamages[b];
            // myInnerHtml("ATK_0" + b, finalDamages[b] + n_A_EDP_DMG[b], 0);
            finalDamages[b] = (finalDamages[b] * hitRate + ApplyWeaponryResearchAndDMGLevel(stats, targetStats, 0, InWarOfEmperium) * element[targetStats.element][0] * (100 - hitRate)) / 100;
            finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
        }
        battleResult.atk00 = (finalDamagesCopy[0] + n_A_EDP_DMG[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1] + n_A_EDP_DMG[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2] + n_A_EDP_DMG[2]) + "";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
    } else if (stats.skillToUseName === "Final Strike" || stats.skillToUseName === "Final Strike (MaxHP-1)") {
        stats.weapon1Element = 0;
        isRangedAttack = 1;
        ApplySkillModifier(stats, n_A_DMG, skillModifier, 0);
        if (stats.skillToUseName === "Final Strike")
            w_1senHP = eval(document.calcForm.SkillSubNum.value);
        else
            w_1senHP = stats.maxHp - 1;

        finalDamages[0] = (stats.str + stats.skillToUseLV) * 40 + w_1senHP * (stats.baseLevel / 100) * stats.skillToUseLV / 10;
        finalDamages[0] = finalDamages[0] * (100 - targetStats.def) / 100;
        finalDamages[0] = BaiCI(stats, targetStats, finalDamages[0], InWarOfEmperium);
        finalDamages[0] = Math.floor(finalDamages[0] * element[targetStats.element][0]);

        finalDamages[2] = finalDamages[1] = finalDamages[0];
        battleResult.atk00 = finalDamages[0];
        battleResult.atk01 = finalDamages[2];
        battleResult.atk02 = finalDamages[2];

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Acid Terror") {
        isRangedAttack = 1;
        stats.weapon1Element = 0;
        skillModifier = (50 + stats.skillToUseLV * 50) / 100;

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = Math.floor((BK_n_A_DMG[b] - n_B_DEF2[b]) * skillModifier);
            finalDamages[b] = tPlusDamCut(stats, targetStats, Math.floor(finalDamages[b] * element[targetStats.element][0], InWarOfEmperium));
            // myInnerHtml("ATK_0" + b, finalDamages[b], 0);
            finalDamagesCopy[b] = finalDamages[b];
        }
        battleResult.atk00 = (finalDamagesCopy[0]) + "";
        battleResult.atk01 = (finalDamagesCopy[1]) + "";
        battleResult.atk02 = (finalDamagesCopy[2]) + "";

        wCast = 1 * stats.cast;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Acid Demonstration") {
        isRangedAttack = 1;
        stats.weapon1Element = 0;
        hitCount = stats.skillToUseLV;

        wAD = 0.7 * stats.int * stats.int * targetStatsArray[TARGET_STAT_VIT] / (stats.int + targetStatsArray[TARGET_STAT_VIT]);
        finalDamages[2] = Math.floor(wAD);
        finalDamages[2] = tPlusDamCut(stats, targetStats, Math.floor(finalDamages[2] * element[targetStats.element][0], InWarOfEmperium));
        if (InWarOfEmperium == 1)
            finalDamages[2] = Math.floor(finalDamages[2] / 2);
        // myInnerHtml("ATK_02", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        // myInnerHtml("ATK_00", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        // myInnerHtml("ATK_01", finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)", 0);
        battleResult.atk00 = finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)";
        battleResult.atk01 = finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)";
        battleResult.atk02 = finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)";
        finalDamages[2] *= hitCount;
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        wCast = 1 * stats.cast;
        wDelay = 1;
        swDelay = 1;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);
        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Land Mine" || stats.skillToUseName === "Blast Mine" || stats.skillToUseName === "Claymore Trap") {
        n_SpSkill = 1;
        if (stats.skillToUseName === "Land Mine") {
            stats.weapon1Element = 2;
            finalDamages[2] = Math.floor((75 + stats.dex) * (1 + stats.int / 100) * stats.skillToUseLV * element[targetStats.element][2]);
        } else if (stats.skillToUseName === "Blast Mine") {
            stats.weapon1Element = 4;
            finalDamages[2] = Math.floor((50 + stats.dex / 2) * (1 + stats.int / 100) * stats.skillToUseLV * element[targetStats.element][4]) * eval(document.calcForm.SkillSubNum.value);
        } else if (stats.skillToUseName === "Claymore Trap") {
            stats.weapon1Element = 3;
            finalDamages[2] = Math.floor((75 + stats.dex / 2) * (1 + stats.int / 100) * stats.skillToUseLV * element[targetStats.element][3]) * eval(document.calcForm.SkillSubNum.value);
        }

        finalDamages[2] = tPlusDamCut(stats, targetStats, finalDamages[2], InWarOfEmperium);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        // myInnerHtml("ATK_02", finalDamages[2], 0);
        // myInnerHtml("ATK_00", finalDamages[0], 0);
        // myInnerHtml("ATK_01", finalDamages[1], 0);
        battleResult.atk00 = finalDamages[0];
        battleResult.atk01 = finalDamages[1];
        battleResult.atk02 = finalDamages[2];
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, 0, 0, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Heal") {
        stats.weapon1Element = 6;
        wDelay = 1;
        swDelay = 1;
        isRangedAttack = 2;
        finalDamages[2] = HealCalc(stats.skillToUseLV, 0);
        finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[targetStats.element][6]);
        if (targetStats.element < 90) {
            finalDamages[2] = 0;
        }

        finalDamages[2] = tPlusDamCut(stats, targetStats, finalDamages[2], InWarOfEmperium);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        // myInnerHtml("ATK_02", finalDamages[2], 0);
        // myInnerHtml("ATK_00", finalDamages[0], 0);
        // myInnerHtml("ATK_01", finalDamages[1], 0);
        battleResult.atk00 = finalDamages[0];
        battleResult.atk01 = finalDamages[1];
        battleResult.atk02 = finalDamages[2];

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, 0, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Sanctuary") {
        stats.weapon1Element = 6;
        n_SpSkill = 1;
        isRangedAttack = 2;
        if (stats.skillToUseLV <= 6)
            finalDamages[2] = 100 * stats.skillToUseLV;
        else
            finalDamages[2] = 777;
        finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[targetStats.element][6]);
        if (targetStats.element < 90 && targetStats.race != 6)
            finalDamages[2] = 0;
        if (targetStats.race != 6 && targetStats.race != 1)
            finalDamages[2] = 0;

        w_HEAL_BAI = 100;
        if (EquipNumSearch("Staff of Recovery", stats))
            w_HEAL_BAI += Math.floor(weaponRefinementLevel * 1.5)
        if (CardNumSearch("White Lady"))
            w_HEAL_BAI += 30 * CardNumSearch("White Lady");
        finalDamages[2] = Math.floor(finalDamages[2] * w_HEAL_BAI / 100);

        finalDamages[2] = tPlusDamCut(stats, targetStats, finalDamages[2], InWarOfEmperium);
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];
        // myInnerHtml("ATK_02", finalDamages[2], 0);
        // myInnerHtml("ATK_00", finalDamages[0], 0);
        // myInnerHtml("ATK_01", finalDamages[1], 0);
        battleResult.atk00 = finalDamages[0];
        battleResult.atk01 = finalDamages[1];
        battleResult.atk02 = finalDamages[2];

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, 0, 0, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Turn Undead") {
        stats.weapon1Element = 6;
        isRangedAttack = 2;
        if (targetStats.element < 90) {
            w = 0;
            finalDamages[2] = 0;
            finalDamages[0] = 0;
            finalDamages[1] = 0;
        } else {
            if (targetStatsArray[19] != 1) {
                w = (20 * stats.skillToUseLV + stats.baseLevel + stats.int + stats.luk) / 1000;
                finalDamages[2] = targetStats.hp;
            } else {
                w = 0;
                finalDamages[2] = 0;
            }
            finalDamages[0] = stats.baseLevel + stats.int + stats.skillToUseLV * 10;
            finalDamages[0] = Math.floor(finalDamages[0] * element[targetStats.element][6]);
            finalDamages[1] = Math.round((targetStats.hp * w + finalDamages[0] * (100 - w) / 100));
        }
        // myInnerHtml("ATK_02", Math.floor(finalDamages[2] * element[targetStats.element][6]) + "(Success Rate " + Math.round(w * 10000) / 100 + "%)", 0);
        // myInnerHtml("ATK_00", finalDamages[0] + "(Failure Damage)", 0);
        // myInnerHtml("ATK_01", finalDamages[1] + "(Certain One Hit Kill HP)", 0);
        battleResult.atk00 = finalDamages[0] + "(Failure Damage)";
        battleResult.atk01 = finalDamages[1] + "(Certain One Hit Kill HP)";
        battleResult.atk02 = Math.floor(finalDamages[2] * element[targetStats.element][6]) + "(Success Rate " + Math.round(w * 10000) / 100 + "%)";

        wCast = 1 * stats.cast;
        wDelay = 3;
        swDelay = 1;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else if (stats.skillToUseName === "Gravity Field") {
        stats.weapon1Element = 0;
        n_SpSkill = 1;
        isRangedAttack = 2;
        hitCount = 4 + stats.skillToUseLV;
        finalDamages[2] = 200 + 200 * stats.skillToUseLV;

        finalDamages[2] = Math.floor(finalDamages[2]);

        wStrG = finalDamages[2] * hitCount + "(" + finalDamages[2] + " x " + hitCount + "hit)"
        // myInnerHtml("ATK_02", wStrG, 0);
        // myInnerHtml("ATK_00", wStrG, 0);
        // myInnerHtml("ATK_01", wStrG, 0);
        battleResult.atk00 = wStrG;
        battleResult.atk01 = wStrG;
        battleResult.atk02 = wStrG;

        finalDamages[2] = finalDamages[2] * hitCount;
        finalDamages[0] = finalDamages[2];
        finalDamages[1] = finalDamages[2];

        wCast = 5 * stats.cast;
        swDelay = 1;
        wDelay = 2;
        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    } else {
        isRangedAttack = 2;
        swDelay = 1;
        skillModifier = 1;
        if (stats.skillToUseName === "Fire Bolt") {
            stats.weapon1Element = 3;
            hitCount = stats.skillToUseLV;
            wCast = 0.7 * stats.skillToUseLV;
            wDelay = 0.8 + stats.skillToUseLV * 0.2;
        } else if (stats.skillToUseName === "Cold Bolt") {
            stats.weapon1Element = 1;
            hitCount = stats.skillToUseLV;
            wCast = 0.7 * stats.skillToUseLV;
            wDelay = 0.8 + stats.skillToUseLV * 0.2;
        } else if (stats.skillToUseName === "Lightning Bolt") {
            stats.weapon1Element = 4;
            hitCount = stats.skillToUseLV;
            wCast = 0.7 * stats.skillToUseLV;
            wDelay = 0.8 + stats.skillToUseLV * 0.2;
        } else if (stats.skillToUseName === "Fire Ball") {
            stats.weapon1Element = 3;
            if (stats.skillToUseLV <= 5) {
                wCast = 1.5;
                wDelay = 1.5;
            } else {
                wCast = 1;
                wDelay = 1;
            }
            skillModifier = 0.7 + stats.skillToUseLV * 0.1;
        } else if (stats.skillToUseName === "Fire Wall") {
            stats.weapon1Element = 3;
            hitCount = 4 + stats.skillToUseLV;
            wCast = 2.15 - (stats.skillToUseLV * 0.15);
            wDelay = 0.1;
            skillModifier = 0.5;
        } else if (stats.skillToUseName === "Frost Diver") {
            stats.weapon1Element = 1;
            wCast = 0.8;
            wDelay = 1.5;
            skillModifier = 1 + stats.skillToUseLV * 0.1;
        } else if (stats.skillToUseName === "Thunder Storm") {
            stats.weapon1Element = 4;
            hitCount = stats.skillToUseLV;
            wCast = 1 * stats.skillToUseLV;
            wDelay = 2;
            skillModifier = 0.8;
        } else if (stats.skillToUseName === "Napalm Beat") {
            stats.weapon1Element = 8;
            wCast = 0.5;
            if (stats.skillToUseLV == 10)
                wDelay = 0.5;
            else if (stats.skillToUseLV == 9)
                wDelay = 0.6;
            else if (stats.skillToUseLV == 8)
                wDelay = 0.7;
            else if (stats.skillToUseLV >= 6)
                wDelay = 0.8;
            else if (stats.skillToUseLV >= 4)
                wDelay = 0.9;
            else
                wDelay = 1;
            skillModifier = 0.7 + stats.skillToUseLV * 0.1;
        } else if (stats.skillToUseName === "Soul Strike") {
            stats.weapon1Element = 8;
            hitCount = Math.round(stats.skillToUseLV / 2);
            wCast = 0.5;
            if (stats.skillToUseLV % 2 == 0)
                wDelay = 0.8 + stats.skillToUseLV / 2 * 0.2;
            else
                wDelay = 1 + (stats.skillToUseLV + 1) / 2 * 0.2;
        } else if (stats.skillToUseName === "Fire Pillar") {
            stats.weapon1Element = 3;
            hitCount = stats.skillToUseLV + 2;
            wCast = 3.3 - (0.3 * stats.skillToUseLV);
            wDelay = 1;
            skillModifier = 0.2;
        } else if (stats.skillToUseName === "Sightrasher") {
            stats.weapon1Element = 3;
            wCast = 0.7;
            wDelay = 2;
            skillModifier = 1 + stats.skillToUseLV * 0.2;
        } else if (stats.skillToUseName === "Meteor Storm") {
            stats.weapon1Element = 3;
            hitCount = Math.round(stats.skillToUseLV / 2) * (Math.floor(stats.skillToUseLV / 2) + 2);
            wCast = 15;
            wDelay = Math.floor(stats.skillToUseLV / 2) * 1 + 2;
        } else if (stats.skillToUseName === "Jupitel Thunder") {
            stats.weapon1Element = 4;
            hitCount = stats.skillToUseLV + 2;
            wCast = 2 + stats.skillToUseLV * 0.5;
            wDelay = 0.01;
        } else if (stats.skillToUseName === "Lord of Vermillion") {
            stats.weapon1Element = 4;
            hitCount = 4;
            wCast = 15.5 - stats.skillToUseLV * 0.5;
            wDelay = 5;
            skillModifier = 0.8 + stats.skillToUseLV * 0.2;
        } else if (stats.skillToUseName === "Water Ball" || stats.skillToUseName === "Water Ball") {
            swDelay = 2;
            stats.weapon1Element = 1;
            if (stats.skillToUseLV >= 4)
                hitCount = 25
            else if (stats.skillToUseLV >= 2)
                hitCount = 9;
            wCast = stats.skillToUseLV;
            skillModifier = 1 + stats.skillToUseLV * 0.3;
            wDelay = 0.1 * hitCount;
        } else if (stats.skillToUseName === "Frost Nova") {
            skillModifier = 0.66 + stats.skillToUseLV * 0.066;
            stats.weapon1Element = 1;
            wCast = 6 - Math.floor((stats.skillToUseLV - 1) / 2) * 0.5;
            wDelay = 1;
        } else if (stats.skillToUseName === "Storm Gust") {
            stats.weapon1Element = 1;
            hitCount = eval(document.calcForm.SkillSubNum.value);
            wCast = 5 + stats.skillToUseLV;
            wDelay = 5;
            skillModifier = 1 + stats.skillToUseLV * 0.4;
        } else if (stats.skillToUseName === "Earth Spike" || stats.skillToUseName === "Heaven's Drive") {
            stats.weapon1Element = 2;
            hitCount = stats.skillToUseLV;
            if (stats.skillToUseName === "Earth Spike") {
                wCast = stats.skillToUseLV * 0.7;
                wDelay = 0.8 + stats.skillToUseLV * 0.2;
            } else {
                wCast = stats.skillToUseLV;
                wDelay = 1;
            }
        } else if (stats.skillToUseName === "Napalm Vulcan") {
            hitCount = stats.skillToUseLV;
            stats.weapon1Element = 8;
            wCast = 1;
            wDelay = 1;
            skillModifier = 0.7 + stats.skillToUseLV * 0.1;
        } else if (stats.skillToUseName === "Holy Light" || stats.skillToUseName === "Holy Light (Soul Linked)") {
            stats.weapon1Element = 6;
            wCast = 2;
            skillModifier = 1.25;
            if (stats.skillToUseName === "Holy Light (Soul Linked)")
                skillModifier *= 5;
            wDelay = 0.01;
        } else if (stats.skillToUseName === "Magnus Exorcismus") {
            n_SpSkill = 1;
            stats.weapon1Element = 6;
            hitCount = stats.skillToUseLV;
            wCast = 15;
            wDelay = 4;
            if (targetStats.race != 6 && targetStats.element < 90) {
                n_A_MATK[2] = 0;
                n_A_MATK[0] = 0;
                n_A_MATK[1] = 0;
            }
        } else if (stats.skillToUseName === "Estin") {
            stats.weapon1Element = eval(document.calcForm.A_Weapon_element.value);
            wCast = 0.1;
            wDelay = 0.5;
            if (targetStats.size == 0)
                skillModifier = stats.skillToUseLV * 0.1;
            else
                skillModifier = 0.01;
            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (stats.skillToUseName === "Estun") {
            stats.weapon1Element = eval(document.calcForm.A_Weapon_element.value);
            wCast = 0.1;
            wDelay = 0.5;

            skillModifier = stats.skillToUseLV * 0.05;


            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (stats.skillToUseName === "Esma") {
            stats.weapon1Element = eval(document.calcForm.A_Weapon_element.value);
            n_SpSkill = 1;
            hitCount = stats.skillToUseLV;
            wCast = 2;
            wDelay = 0.5;
            skillModifier = 0.4 + stats.baseLevel / 100;
            if (InWarOfEmperium == 1)
                skillModifier = 0;
        } else if (stats.skillToUseName === "Flaming Petals") {
            stats.weapon1Element = 3;
            skillModifier = 0.9;
            hitCount = stats.skillToUseLV;
            wCast = 0.7 * stats.skillToUseLV;
            wDelay = 0.01;
        } else if (stats.skillToUseName === "Blaze Shield") {
            stats.weapon1Element = 3;
            skillModifier = 0.5;
            hitCount = Math.round(stats.skillToUseLV / 2) + 4;
            wCast = 6.5 - 0.5 * stats.skillToUseLV;
            wDelay = 1;
            n_SpSkill = 1;
        } else if (stats.skillToUseName === "Exploding Dragon") {
            stats.weapon1Element = 3;
            skillModifier = 1.5 + stats.skillToUseLV * 1.5;
            hitCount = 1;
            wCast = 3;
            wDelay = 3;
        } else if (stats.skillToUseName === "Freezing Spear") {
            stats.weapon1Element = 1;
            skillModifier = 1;
            hitCount = stats.skillToUseLV + 2;
            wCast = stats.skillToUseLV * 0.7;
            wDelay = 0.01;
        } else if (stats.skillToUseName === "Snow Flake Draft") {
            stats.weapon1Element = 1;
            skillModifier = 1.0 + stats.skillToUseLV * 0.5;
            hitCount = 1;
            wCast = 3;
            wDelay = 3;
        } else if (stats.skillToUseName === "Wind Blade") {
            stats.weapon1Element = 4;
            skillModifier = 1.0;
            hitCount = Math.floor(stats.skillToUseLV / 2) + 1;
            wCast = Math.floor(stats.skillToUseLV / 2) + 1;
            wDelay = 1;
        } else if (stats.skillToUseName === "Lightning Jolt") {
            stats.weapon1Element = 4;
            skillModifier = 1.6 + 0.4 * stats.skillToUseLV;
            hitCount = 1;
            wCast = 4;
            wDelay = 0.01;

        } else if (stats.skillToUseName === "First Wind") {
            stats.weapon1Element = 4;
            skillModifier = 1.0 + stats.skillToUseLV * 1.0;
            hitCount = 1;
            wCast = 4;
            wDelay = 0.01;
        }

        wCast *= stats.cast;

        let finalDamagesCopy = [0, 0, 0];
        for (b = 0; b <= 2; b++) {
            finalDamages[b] = BattleMagicCalc(n_A_MATK[b] * skillModifier);
            // myInnerHtml("ATK_0" + b, finalDamages[b] * hitCount + "(" + finalDamages[b] + SubName[8] + hitCount + "hit)", 0);
            finalDamagesCopy[b] = finalDamages[b];
            finalDamages[b] *= hitCount;
        }
        battleResult.atk00 = finalDamages[0] * hitCount + "(" + finalDamages[0] + SubName[8] + hitCount + "hit)";
        battleResult.atk01 = finalDamages[1] * hitCount + "(" + finalDamages[1] + SubName[8] + hitCount + "hit)";
        battleResult.atk02 = finalDamages[2] * hitCount + "(" + finalDamages[2] + SubName[8] + hitCount + "hit)";

        let castAndDelayBattleResult = CastAndDelay(stats, wCast, wDelay, swDelay);
        battleResult = Object.assign(battleResult, castAndDelayBattleResult);

        let battleVariousResult = BattleVariousResults(stats, targetStats, wCast, wDelay, finalDamages, InWarOfEmperium);
        battleResult = Object.assign(battleResult, battleVariousResult);
        // myInnerHtml("BattleHIT", 100, 0);
        battleResult.battleHit = 100;
    }
    return battleResult;
}


function ApplyATKBonusPercentage(stats, n_A_DMG) {
    let wA01 = 100;

    ;
    if (stats.skillToUseName != "Occult Impaction" && stats.skillToUseName != "Guillotine Fist" && stats.skillToUseName != "Guillotine Fist (MaxSP-1)") {
        if (SkillSearch("Auto Berserk", stats))
            wA01 += 32;
        else if (stats.supportiveSkills[12])
            wA01 += 5;


        if (SkillSearch("Spear Dynamo", stats))
            wA01 += SkillSearch("Spear Dynamo", stats) * 5;
        if (SkillSearch("True Sight", stats))
            wA01 += SkillSearch("True Sight", stats) * 2;
        if (stats.supportiveSkillsBattleChant[3])
            wA01 += 100;
        if (stats.groundSupportiveSkills[2])
            wA01 += 10;
        if (StPlusItem("ATK_PERCENTAGE", stats))
            wA01 += StPlusItem("ATK_PERCENTAGE", stats);
    }
    stats.critATK[2] = stats.critATK[2] * wA01 / 100;
    stats.critATK[0] = stats.critATK[0] * wA01 / 100;
    stats.critATK[1] = stats.critATK[1] * wA01 / 100;
    n_A_DMG[2] = n_A_DMG[2] * wA01 / 100;
    n_A_DMG[0] = n_A_DMG[0] * wA01 / 100;
    n_A_DMG[1] = n_A_DMG[1] * wA01 / 100;
}


function ApplySkillModifier(stats, n_A_DMG, skillModifier, isCrit) {
    let wA02 = skillModifier * 100;
    if (SkillSearch("Maximum Power-Thust", stats)) {
        wA02 += 20 * SkillSearch("Maximum Power-Thust", stats);
    } else {
        if (SkillSearch("Power-Thrust", stats))
            wA02 += SkillSearch("Power-Thrust", stats) * 5;
        if (SkillSearch("Power-Thrust", stats) == 0 && stats.supportiveSkills[8])
            wA02 += stats.supportiveSkills[8] * 5 / 5;
    }
    if (SkillSearch("Kihop", stats)) {
        wA02 += 2 * SkillSearch("Kihop") * SkillSearch("Party Members (Kihop Bonus", stats);
    }

    if (isCrit == 0) {
        n_A_DMG[2] = Math.floor(n_A_DMG[2] * wA02 / 100);
        n_A_DMG[0] = Math.floor(n_A_DMG[0] * wA02 / 100);
        n_A_DMG[1] = Math.floor(n_A_DMG[1] * wA02 / 100);
    } else {
        stats.critATK[1] = Math.floor(stats.critATK[1] * wA02 / 100);
        stats.critATK[0] = Math.floor(stats.critATK[0] * wA02 / 100);
        stats.critATK[2] = Math.floor(stats.critATK[2] * wA02 / 100);
    }
}


function HealCalc(HealLv, HealType) {
    wHeal = Math.floor((stats.baseLevel + stats.int) / 8) * (HealLv * 8 + 4);
    wHealBAI = 100;
    wHealBAI += SkillSearch("Mediatio", stats) * 2;
    if (EquipNumSearch("Staff of Recovery", stats))
        wHealBAI += Math.floor(weaponRefinementLevel * 1.5)
    if (CardNumSearch("White Lady"))
        wHealBAI += 30 * CardNumSearch("White Lady");
    if (HealType == 0)
        if (EquipNumSearch("0") || EquipNumSearch("0", stats))
            wHealBAI += 50;
    wHeal = Math.floor(wHeal * wHealBAI / 100);
    return wHeal;
}


function BattleVariousResults(stats, targetStats, cast, afterCastDelay, finalDamages, InWarOfEmperium) {
    let battleResult = {
        // dps: 0, // DPS
        // minAtkNum: 0, // MinATKnum
        // maxAtkNum: 0, // MaxATKnum
        // avgAtkNum: 0, // AveATKnum
        // battleTime: 0, // BattleTime
        // averageReceivedDamage: 0, // AverageReceivedDamageIncludingDodge
    }
    let w;
    if (afterCastDelay == 0)
        w = 1 / (cast + stats.aspd) * finalDamages[1];
    else
        w = 1 / (cast + afterCastDelay) * finalDamages[1];
    w *= 100;
    w = Math.round(w);
    w /= 100;

    if (n_SpSkill) {
        // myInnerHtml("DPS", "Special", 0);
    } else {
        // myInnerHtml("DPS", w, 0);
        battleResult.dps = w;
    }

    if (targetStats.mobIndex == 44 && stats.skillToUseName != "Basic Attack") {
        for (let i = 0; i <= 2; i++) {
            finalDamages[i] = 0;
            // myInnerHtml("ATK_0" + i, 0, 0);
        }
        battleResult.atk00 = 0;
        battleResult.atk01 = 0;
        battleResult.atk02 = 0;
    }

    tPlusAG(InWarOfEmperium);
    w = targetStats.hp;
    let i;
    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[2];
    }
    if (i < 1000) {
        // myInnerHtml("MinATKnum", i, 0);
        battleResult.minAtkNum = i;
    } else {
        // myInnerHtml("MinATKnum", SubName[5], 0);
        battleResult.minAtkNum = SubName[5];
    }
    w = targetStats.hp;

    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[0];
    }
    if (i < 1000) {
        battleResult.maxAtkNum = i;
        // myInnerHtml("MaxATKnum", i, 0);
    } else {
        battleResult.maxAtkNum = SubName[5];
        // myInnerHtml("MaxATKnum", SubName[5], 0);
    }
    w = targetStats.hp;

    for (i = 0; 0 < w && i < 1000; i++) {
        w -= finalDamages[1];
    }

    if (InWarOfEmperium == 0) {
        // if (i < 1000) {
        //     myInnerHtml("AtkBaseExp", Math.round(targetStatsArray[16] / i) + "Exp", 0);
        //     myInnerHtml("AtkJobExp", Math.round(targetStatsArray[17] / i) + "Exp", 0);
        // } else {
        //     myInnerHtml("AtkBaseExp", SubName[7], 0);
        //     myInnerHtml("AtkJobExp", SubName[7], 0);
        // }
    }
    if (i < 1000) {
        battleResult.avgAtkNum = i;
        // myInnerHtml("AveATKnum", i, 0);

        let n_AveATKnum = i;


        if (afterCastDelay == 0)

            w = (cast + stats.aspd) * n_AveATKnum;
        else
            w = (cast + afterCastDelay) * n_AveATKnum;
        w = Math.floor(w * 100) / 100;

        if (n_SpSkill) {
            // myInnerHtml("BattleTime", "Special", 0);
            battleResult.battleTime = "special";
        } else {
            // myInnerHtml("BattleTime", w + "s", 0);
            battleResult.battleTime = w + "s";
        }
    } else {
        battleResult.avgAtkNum = SubName[5];
        battleResult.battleTime = SubName[6];
        // myInnerHtml("AveATKnum", SubName[5], 0);
        // myInnerHtml("BattleTime", SubName[6], 0);
    }


    n_SpSkill = 0

    if (InWarOfEmperium == 0) {
        let damageReceived = CalculateDamageReceived(stats, targetStats);
        battleResult = Object.assign(battleResult, damageReceived);
        w = damageReceived.avgDamageReceived;

        w = Math.round(w * (100 - stats.perfectDodge)) / 100;
        w = Math.round(w * (100 - stats.flee)) / 100;
        if (SkillSearch("Guard", stats)) {
            w = Math.round(w * w_AG[SkillSearch("Guard", stats)]) / 100;
        }
        if (stats.weaponType == WEAPON_TYPE_TWO_HANDED_SWORD && SkillSearch("Parrying", stats)) {
            w = Math.round(w * (80 - SkillSearch("Parrying", stats) * 3)) / 100;
        }
        if (SkillSearch("Counter Instinct", stats)) {
            w = Math.round(w * (100 - SkillSearch("Counter Instinct", stats) * 7.5)) / 100;
        }
        // myInnerHtml("AverageReceivedDamageIncludingDodge", w + "Damage", 0);
        battleResult.averageReceivedDamage = w;

    }
    return battleResult;
}

function CalculateDamageReceived(stats, targetStats) {
    let battleResult = {
        // avgDamageReceived: 0, // AverageReceivedDamage
        // minDamageReceived: 0,
        // maxDamageReceived: 0,
    }

    let w_HiDam = new Array();
    let wBHD = targetStats.atk2;
    w_HiDam[0] = targetStats.atk;
    w_HiDam[1] = (targetStats.atk * 5 + wBHD) / 6;
    w_HiDam[2] = (targetStats.atk * 4 + wBHD * 2) / 6;
    w_HiDam[3] = (targetStats.atk + wBHD) / 2;
    w_HiDam[4] = (targetStats.atk * 2 + wBHD * 4) / 6;
    w_HiDam[5] = (targetStats.atk + wBHD * 5) / 6;
    w_HiDam[6] = wBHD;
    if (targetStats.atk == targetStats.atk2)
        w_HiDam[6] = wBHD - 1;

    w_HiDam[0] = w_HiDam[0] * (100 - stats.totalDef) / 100 - stats.vitDEF[2];
    w_HiDam[1] = w_HiDam[1] * (100 - stats.totalDef) / 100 - stats.vitDEF[2];
    w_HiDam[2] = w_HiDam[2] * (100 - stats.totalDef) / 100 - stats.vitDEF[2];
    w_HiDam[3] = w_HiDam[3] * (100 - stats.totalDef) / 100 - stats.vitDEF[1];
    w_HiDam[4] = w_HiDam[4] * (100 - stats.totalDef) / 100 - stats.vitDEF[0];
    w_HiDam[5] = w_HiDam[5] * (100 - stats.totalDef) / 100 - stats.vitDEF[0];
    w_HiDam[6] = w_HiDam[6] * (100 - stats.totalDef) / 100 - stats.vitDEF[0];


    if (SkillSearch("Divine Protection", stats) && (targetStats.element >= 90 || targetStats.race == 6)) {
        wBHD = Math.floor((3 + 4 / 100 * stats.baseLevel) * SkillSearch("Divine Protection", stats));
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= wBHD;
    }


    if (SkillSearch("Solar Protection", stats)) {
        wBHD = Math.floor((stats.baseLevel + stats.luk + stats.dex) / 2);
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= wBHD;
    }


    wBHD = StPlusCard("DAMAGE_INC_DEC_ELEMENT_NEUTRAL_PERCENTAGE", stats);
    wBHD += StPlusItem("DAMAGE_INC_DEC_ELEMENT_NEUTRAL_PERCENTAGE", stats);
    if (EquipNumSearch("0", stats) || EquipNumSearch("0", stats))
        wBHD += stats.shoulderDefPlus * 3;
    if (SkillSearch("Skin Tempering", stats))
        wBHD += SkillSearch("Skin Tempering", stats);
    if (stats.shoulderDefPlus >= 9 && CardNumSearch("Orc Baby", stats))
        wBHD += 5;
    if (wBHD != 0) {
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }

    if (SkillSearch("Energy Coat", stats)) {
        wBHD = 6 * SkillSearch("Energy Coat", stats);
        for (i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    wBHD = 0;
    if (CardNumSearch("0", stats) && n_A_JobSearch(stats) == 3 && (targetStats.race == 6 || targetStats.race == 1))
        wBHD += 30;
    if (stats.supportiveSkills[14] && targetStats.race == 6 && stats.job != 13 && stats.job != 27)
        wBHD += stats.supportiveSkills[14] * 5;
    if (targetStats.race == 9 && SkillSearch("Dragonology", stats))
        wBHD += SkillSearch("Dragonology", stats) * 4;
    wBHD += StPlusCard(DAMAGE_INC_DEC_RACE_PERCENTAGE + targetStats.race, stats);
    wBHD += StPlusItem(DAMAGE_INC_DEC_RACE_PERCENTAGE + targetStats.race, stats);
    if (wBHD != 0) {
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    wBHD = 0;
    wBHD += StPlusCard(DAMAGE_INC_DEC_SIZE_PERCENTAGE + targetStats.size, stats);
    wBHD += StPlusItem(DAMAGE_INC_DEC_SIZE_PERCENTAGE + targetStats.size, stats);
    if (targetStats.size == 1) {
        if (EquipNumSearch("Hurricane Fury", stats))
            wBHD += weaponRefinementLevel;
    }

    if (wBHD != 0) {
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    if (targetStats.isNormal) {
        wBHD = StPlusCard("NORMAL_ATTACK_PERCENTAGE", stats);
        wBHD += StPlusItem("NORMAL_ATTACK_PERCENTAGE", stats);
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    }


    if (targetStats.rangeAttack) {
        wBHD = StPlusCard("RESISTANCE_RANGE_ATTACK_PERCENTAGE", stats);
        wBHD += StPlusItem("RESISTANCE_RANGE_ATTACK_PERCENTAGE", stats);
        if (SkillSearch("Gunslinger's Panic", stats))
            wBHD += 20;
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);

        if (SkillSearch("Defending Aura", stats)) {
            wBHD = 5 + 15 * SkillSearch("Defending Aura", stats);
            for (let i = 0; i <= 6; i++)
                w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
        }
    }


    if (targetStats.isMvp && CardNumSearch("Alice", stats)) {
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] -= Math.floor(w_HiDam[i] * 40 / 100);

    }


    if (targetStats.mobIndex == 275 && CardNumSearch("Bongun", stats)) {
        wBHD = 100 * CardNumSearch("Bongun", stats);
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD / 100);

    }


    wBHD = StPlusCard(3000 + targetStats.mobIndex, stats);
    wBHD += StPlusItem(3000 + targetStats.mobIndex, stats);
    for (let i = 0; i <= 6; i++)
        w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);


    if (EquipNumSearch("Strong Shield", stats)) {
        wBHD = 20;
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD / 100);

    }

    for (let i = 0; i <= 6; i++) {
        if (w_HiDam[i] < 1)
            w_HiDam[i] = 1;
        w_HiDam[i] = Math.floor(w_HiDam[i] * 100) / 100;
    }

    if (stats.supportiveSkills[5])
        for (let i = 0; i <= 6; i++)
            w_HiDam[i] = Math.floor(w_HiDam[i] / 2);

    w_HiDam[0] = Math.floor(w_HiDam[0]);
    w_HiDam[6] = Math.floor(w_HiDam[6]);


    wBHD = 0;
    for (let i = 0; i <= 6; i++)
        wBHD += w_HiDam[i];
    wBHD = Math.round(wBHD / 7);
    battleResult.avgDamageReceived = wBHD;
    battleResult.minDamageReceived = w_HiDam[0];
    battleResult.maxDamageReceived = w_HiDam[6];
    // myInnerHtml("AverageReceivedDamage", wBHD + " (" + w_HiDam[0] + " ~ " + w_HiDam[6] + ")", 0);

    return battleResult;
}

function BattleMagicCalc(wBMC) {


    wBMC_MDEF = targetStatsArray[TARGET_STAT_MDEF];
    wMDEF_w = 0;
    if (EquipNumSearch("Staff of Piercing", stats))
        wMDEF_w += 10 + weaponRefinementLevel;
    if (targetStats.isNormal && CardNumSearch("High Wizard Kathryne (MVP)"))
        wMDEF_w += 100;
    if (targetStats.isMvp && CardNumSearch("Vesper"))
        wMDEF_w += 30 * CardNumSearch("Vesper");
    if (wMDEF_w > 100)
        wMDEF_w = 100;
    if (wMDEF_w != 0) {
        wBMC_MDEF = wBMC_MDEF - Math.floor(wBMC_MDEF * wMDEF_w / 100);
        n_B_MDEF2 = n_B_MDEF2 - Math.floor(n_B_MDEF2 * wMDEF_w / 100);
    }
    if (stats.skillToUseName == "Fire Pillar")
        wBMC2 = Math.floor(wBMC + 50);
    else
        wBMC2 = Math.floor(wBMC * (100 - wBMC_MDEF) / 100 - n_B_MDEF2);
    if (wBMC2 < 1) wBMC2 = 1;
    if (stats.skillToUseName == "Magnus Exorcismus") {
        if (targetStats.race != 6 && targetStats.element < 90) {
            wBMC2 = 0;
        }
    }

    wBMC2 = Math.floor(wBMC2 * element[targetStats.element][stats.weapon1Element]);
    if (90 <= targetStats.element && (stats.skillToUseName == "Soul Strike" || stats.skillToUseName == ""))
        wBMC2 = Math.floor(wBMC2 * (1 + 0.05 * stats.skillToUseLV));


    if (targetStats.race == 9 && SkillSearch("Dragonology", stats))
        wBMC2 = wBMC2 * (100 + SkillSearch("Dragonology", stats) * 2) / 100;

    if (targetStats.race == 8)
        wBMC2 = wBMC2 * (100 + CardNumSearch("Dolor of Thanatos") * 10) / 100;

    if (targetStats.race == 6)
        wBMC2 = wBMC2 * (100 + CardNumSearch("Skeggiold") * 2) / 100;

    wBMC2 = tPlusDamCut(stats, targetStats, wBMC2, InWarOfEmperium);


    wBMC2 = wBMC2 * (100 + StPlusItem(5000 + stats.skillToUse) + StPlusCard(5000 + stats.skillToUse)) / 100;

    wBMC2 = Math.floor(wBMC2);

    return wBMC2;
}

function CastAndDelay(stats, wCast, wDelay, swDelay) {
    let battleResult = {
        // bonusSub: "", // bSUB
        // bonusSubName: "", // bSUBname
        // bonusSub2: "", // bSUB2name
        // bonusSub2Name: "", // bSUB2name
        // cast: 0,
        // afterCastDelay: 0,
    }
    if (wCast != 0) {
        let wCAD = stats.performanceSkills[2];
        if (wCAD != 0) {
            wCAD = wCAD * 3 + stats.performanceSkills[32] + Math.floor(stats.performanceSkills[22] / 10);
            wCast *= (100 - wCAD) / 100;
        }
        battleResult.bonusSub = Math.floor(wCast * 100) / 100 + SubName[1];
        battleResult.bonusSubName = SubName[9];
        // myInnerHtml("bSUBname", SubName[9], 0);
        // myInnerHtml("bSUB", Math.floor(wCast * 100) / 100 + SubName[1], 0);
    }
    if (wDelay != 0) {
        if (swDelay == 1) {
            wCAD = stats.performanceSkills[2];
            if (wDelay != "(Unknown)") {
                wDelay = Math.floor(wDelay * (100 - (StPlusCard(ACD_PERCENTAGE, stats) + StPlusItem(ACD_PERCENTAGE, stats)))) / 100;
                if (wCAD != 0) {
                    if (wCAD == 10)
                        wCAD2 = 5;
                    else
                        wCAD2 = 3;
                    wCAD = wCAD * wCAD2 + stats.performanceSkills[32] * 2 + Math.floor(stats.performanceSkills[29] / 5);
                    wDelay *= (100 - wCAD) / 100;
                }
                wDelay = Math.floor(wDelay * 100) / 100;
                let minimumDelayBetweenActiveSkills = 10;
                if (wCast + wDelay < minimumDelayBetweenActiveSkills / 100)
                    wDelay = minimumDelayBetweenActiveSkills / 100 - wCast;
            }
            // myInnerHtml("bSUB2name", "Delay (Fixed Type)", 0);
            // myInnerHtml("bSUB2", Math.floor(wDelay * 100) / 100 + "s", 0);
            battleResult.bonusSub2 = Math.floor(wDelay * 100) / 100 + "s";
            battleResult.bonusSub2Name = "Delay (Fixed Type)";
            return;
        }
        if (swDelay == 2) {
            battleResult.bonusSub2 = wDelay + "s";
            battleResult.bonusSub2Name = "Delay(Motion Type)";
            // myInnerHtml("bSUB2name", "Delay(Motion Type)", 0);
            // myInnerHtml("bSUB2", wDelay + "s", 0);
        } else {
            if (n_SpSkill != 1) {
                if (wDelay != "(�s��)")
                    wDelay = Math.floor(wDelay * 100) / 100;
                battleResult.bonusSub2 = wDelay + "s";
                battleResult.bonusSub2Name = "Delay(Attack Speed Type)";
                // myInnerHtml("bSUB2name", "Delay(Attack Speed Type)", 0);
                // myInnerHtml("bSUB2", wDelay + "s", 0);
            }
        }
    }
    battleResult.cast = wCast;
    battleResult.afterCastDelay = wDelay;
    return battleResult;
}


export {
    CalculateAllStats,
    CalculateEnemyStats,
    CalculateBattle
}