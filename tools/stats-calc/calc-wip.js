function CalculateAllStats(FORM_DATA) {
    let {n_A_JOB, isRebirth} = n_A_JobSet(FORM_DATA);

    let n_A_PassSkill2 = new Array();
    for (let i = 0; i <= 15; i++)
        n_A_PassSkill2[i] = 0;


    let n_A_PassSkill3 = new Array();
    for (let i = 0; i <= 45; i++)
        n_A_PassSkill3[i] = 0;
    n_A_PassSkill3[20] = 100;
    n_A_PassSkill3[21] = 100;
    n_A_PassSkill3[22] = 130;
    n_A_PassSkill3[29] = 80;
    n_A_PassSkill3[23] = 100;
    n_A_PassSkill3[24] = 130;
    n_A_PassSkill3[25] = 50;
    n_A_PassSkill3[26] = 50;
    n_A_PassSkill3[30] = 10;
    n_A_PassSkill3[31] = 10;
    n_A_PassSkill3[32] = 10;
    n_A_PassSkill3[33] = 10;
    n_A_PassSkill3[34] = 10;
    n_A_PassSkill3[35] = 10;
    n_A_PassSkill3[36] = 10;


    let n_A_PassSkill5 = new Array();
    for (let i = 0; i <= 4; i++)
        n_A_PassSkill5[i] = 0;


    let n_A_PassSkill6 = new Array();
    for (let i = 0; i <= 3; i++)
        n_A_PassSkill6[i] = 0;


    let n_A_PassSkill7 = new Array();
    for (let i = 0; i <= 10; i++)
        n_A_PassSkill7[i] = 0;

    if (n_A_JOB == 20) {
        if (SuperNoviceFullWeaponCHECK == 0 && eval(FORM_DATA.A_PASSIVE_SKILL9) == 1)
            SuperNoviceFullWeapon(1);
        else if (SuperNoviceFullWeaponCHECK == 1 && eval(FORM_DATA.A_PASSIVE_SKILL9) == 0)
            SuperNoviceFullWeapon(0);
    }
    let n_A_BaseLV = eval(FORM_DATA.A_BaseLV);
    let n_A_JobLV = eval(FORM_DATA.A_JobLV);

    let n_A_STR = eval(FORM_DATA.A_STR);
    let n_A_AGI = eval(FORM_DATA.A_AGI);
    let n_A_VIT = eval(FORM_DATA.A_VIT);
    let n_A_DEX = eval(FORM_DATA.A_DEX);
    let n_A_INT = eval(FORM_DATA.A_INT);
    let n_A_LUK = eval(FORM_DATA.A_LUK);
    let SU_STR = n_A_STR;
    let SU_AGI = n_A_AGI;
    let SU_VIT = n_A_VIT;
    let SU_DEX = n_A_DEX;
    let SU_INT = n_A_INT;
    let SU_LUK = n_A_LUK;

    let n_A_WeaponType = eval(FORM_DATA.A_WeaponType);

    let n_A_Arrow = eval(FORM_DATA.A_Arrow);
    let n_A_Weapon1 = eval(FORM_DATA.A_weapon1);

    let n_A_WeaponLV = ItemOBJ[n_A_Weapon1][4];
    let n_A_Weapon_ATK = ItemOBJ[n_A_Weapon1][3];

    let n_A_Weapon2LV_upgradeBonusATK = 0;
    let n_A_Weapon2LV_Minplus = 0;
    let n_A_Weapon2LV_overUpgradeBonusATK = 0;
    let n_A_Weapon2LV = 0;
    let n_A_Weapon2_ATK = 0;
    let n_A_Weapon2_RefinementLevel = 0;
    if (hasLeftHand) {

        if (targetStatsArray[19] != 5) {

            n_A_Weapon2 = eval(FORM_DATA.A_weapon2);
            n_A_Weapon2LV = ItemOBJ[n_A_Weapon2][4];
            n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
            n_A_Weapon2_RefinementLevel = eval(FORM_DATA.A_Weapon2_ATKplus);


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
        }
    }

    if (FORM_DATA.A_weapon2) {
        n_A_Weapon2 = eval(FORM_DATA.A_weapon2);
        n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
    } else {
        n_A_Weapon2_ATK = 0;
    }
    let weaponRefinementLevel = eval(FORM_DATA.A_Weapon_ATKplus);

    let n_A_WeaponLV_upgradeBonusATK = 0;
    let n_A_WeaponLV_Minplus = 0;
    let n_A_WeaponLV_overUpgradeBonusATK = 0;
    if (n_A_WeaponLV == 1) {
        n_A_WeaponLV_upgradeBonusATK = weaponRefinementLevel * 2;
        if (weaponRefinementLevel >= 8) {
            n_A_WeaponLV_Minplus = 1;
            n_A_WeaponLV_overUpgradeBonusATK = 3 * (weaponRefinementLevel - 7);
        }
    } else if (n_A_WeaponLV == 2) {
        n_A_WeaponLV_upgradeBonusATK = weaponRefinementLevel * 3;
        if (weaponRefinementLevel >= 7) {
            n_A_WeaponLV_Minplus = 1;
            n_A_WeaponLV_overUpgradeBonusATK = 5 * (weaponRefinementLevel - 6);
        }
    } else if (n_A_WeaponLV == 3) {
        n_A_WeaponLV_upgradeBonusATK = weaponRefinementLevel * 5;
        if (weaponRefinementLevel >= 6) {
            n_A_WeaponLV_Minplus = 1;
            n_A_WeaponLV_overUpgradeBonusATK = 8 * (weaponRefinementLevel - 5);
        }
    } else if (n_A_WeaponLV == 4) {
        n_A_WeaponLV_upgradeBonusATK = weaponRefinementLevel * 7;
        if (weaponRefinementLevel >= 5) {
            n_A_WeaponLV_Minplus = 1;
            n_A_WeaponLV_overUpgradeBonusATK = 14 * (weaponRefinementLevel - 4);
        }
    }
    let n_A_HEAD_DEF_PLUS = eval(FORM_DATA.A_HEAD_DEF_PLUS);
    let n_A_BODY_DEF_PLUS = eval(FORM_DATA.A_BODY_DEF_PLUS);
    let n_A_LEFT_DEF_PLUS = eval(FORM_DATA.A_LEFT_DEF_PLUS);
    let n_A_SHOULDER_DEF_PLUS = eval(FORM_DATA.A_SHOULDER_DEF_PLUS);
    let n_A_SHOES_DEF_PLUS = eval(FORM_DATA.A_SHOES_DEF_PLUS);
    let n_A_DEFplus = n_A_HEAD_DEF_PLUS + n_A_BODY_DEF_PLUS + n_A_LEFT_DEF_PLUS + n_A_SHOULDER_DEF_PLUS + n_A_SHOES_DEF_PLUS;

    let n_A_ActiveSkill = eval(FORM_DATA.A_ActiveSkill);
    let skillToUseName = SkillOBJ[n_A_ActiveSkill][2];
    if (n_A_ActiveSkill > 100000)
        n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) / 100);

    let n_A_ActiveSkillLV = eval(FORM_DATA.A_ActiveSkillLV);
    let n_A_SpeedPOT = eval(FORM_DATA.A_SpeedPOT);
    let n_A_Equip = [];
    n_A_Equip[0] = eval(FORM_DATA.A_weapon1);
    if (hasLeftHand)
        n_A_Equip[1] = eval(FORM_DATA.A_weapon2);
    else
        n_A_Equip[1] = 0;
    n_A_Equip[2] = eval(FORM_DATA.A_head1);
    n_A_Equip[3] = eval(FORM_DATA.A_head2);
    n_A_Equip[4] = eval(FORM_DATA.A_head3);
    n_A_Equip[5] = eval(FORM_DATA.A_left);
    n_A_Equip[6] = eval(FORM_DATA.A_body);
    n_A_Equip[7] = eval(FORM_DATA.A_shoulder);
    n_A_Equip[8] = eval(FORM_DATA.A_shoes);
    n_A_Equip[9] = eval(FORM_DATA.A_acces1);
    n_A_Equip[10] = eval(FORM_DATA.A_acces2);

    SetEquip(n_A_Equip);

    let n_A_card = [];
    n_A_card[0] = eval(FORM_DATA.A_weapon1_card1);
    n_A_card[1] = eval(FORM_DATA.A_weapon1_card2);
    n_A_card[2] = eval(FORM_DATA.A_weapon1_card3);
    n_A_card[3] = eval(FORM_DATA.A_weapon1_card4);
    if (hasLeftHand) {
        n_A_card[4] = eval(FORM_DATA.A_weapon2_card1);
        n_A_card[5] = eval(FORM_DATA.A_weapon2_card2);
        n_A_card[6] = eval(FORM_DATA.A_weapon2_card3);
        n_A_card[7] = eval(FORM_DATA.A_weapon2_card4);
    } else {
        n_A_card[4] = 0;
        n_A_card[5] = 0;
        n_A_card[6] = 0;
        n_A_card[7] = 0;
    }
    n_A_card[8] = eval(FORM_DATA.A_head1_card);
    n_A_card[9] = eval(FORM_DATA.A_head2_card);
    n_A_card[10] = eval(FORM_DATA.A_left_card);
    n_A_card[11] = eval(FORM_DATA.A_body_card);
    n_A_card[12] = eval(FORM_DATA.A_shoulder_card);
    n_A_card[13] = eval(FORM_DATA.A_shoes_card);
    n_A_card[14] = eval(FORM_DATA.A_acces1_card);
    n_A_card[15] = eval(FORM_DATA.A_acces2_card);

    SetCard(n_A_card);

    let n_A_Weapon_element = eval(FORM_DATA.A_Weapon_element);
    let n_A_Weapon2_element = n_A_Weapon_element;


    if (n_A_Weapon_element == 0) {
        for (let j = 0; ItemOBJ[n_A_Equip[0]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[n_A_Equip[0]][j + 11])
                n_A_Weapon_element = ItemOBJ[n_A_Equip[0]][j + 12];
        }
        for (let j = 0; ItemOBJ[n_A_Equip[1]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[n_A_Equip[1]][j + 11])
                n_A_Weapon2_element = ItemOBJ[n_A_Equip[1]][j + 12];
        }

        if (201 <= cardOBJ[n_A_card[0]][0] && cardOBJ[n_A_card[0]][0] <= 204)
            n_A_Weapon_element = cardOBJ[n_A_card[0]][0] - 200;
        if (201 <= cardOBJ[n_A_card[4]][0] && cardOBJ[n_A_card[4]][0] <= 204)
            n_A_Weapon2_element = cardOBJ[n_A_card[4]][0] - 200;

        if (n_A_WeaponType == WEAPON_TYPE_BOW || n_A_WeaponType == WEAPON_TYPE_HANDGUN || n_A_WeaponType == WEAPON_TYPE_RIFLE || n_A_WeaponType == WEAPON_TYPE_SHOTGUN || n_A_WeaponType == WEAPON_TYPE_GATLING_GUN || n_A_WeaponType == WEAPON_TYPE_GRENADE_LAUNCHER) {
            n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
        }
    }

    let n_A_PassSkill = new Array();


    if (JobSkillPassOBJ[n_A_JOB][0] != 999) n_A_PassSkill[0] = eval(FORM_DATA.A_PASSIVE_SKILL0);
    if (JobSkillPassOBJ[n_A_JOB][1] != 999) n_A_PassSkill[1] = eval(FORM_DATA.A_PASSIVE_SKILL1);
    if (JobSkillPassOBJ[n_A_JOB][2] != 999) n_A_PassSkill[2] = eval(FORM_DATA.A_PASSIVE_SKILL2);
    if (JobSkillPassOBJ[n_A_JOB][3] != 999) n_A_PassSkill[3] = eval(FORM_DATA.A_PASSIVE_SKILL3);
    if (JobSkillPassOBJ[n_A_JOB][4] != 999) n_A_PassSkill[4] = eval(FORM_DATA.A_PASSIVE_SKILL4);
    if (JobSkillPassOBJ[n_A_JOB][5] != 999) n_A_PassSkill[5] = eval(FORM_DATA.A_PASSIVE_SKILL5);
    if (JobSkillPassOBJ[n_A_JOB][6] != 999) n_A_PassSkill[6] = eval(FORM_DATA.A_PASSIVE_SKILL6);
    if (JobSkillPassOBJ[n_A_JOB][7] != 999) n_A_PassSkill[7] = eval(FORM_DATA.A_PASSIVE_SKILL7);
    if (JobSkillPassOBJ[n_A_JOB][8] != 999) n_A_PassSkill[8] = eval(FORM_DATA.A_PASSIVE_SKILL8);
    if (JobSkillPassOBJ[n_A_JOB][9] != 999) n_A_PassSkill[9] = eval(FORM_DATA.A_PASSIVE_SKILL9);
    if (JobSkillPassOBJ[n_A_JOB][10] != 999) n_A_PassSkill[10] = eval(FORM_DATA.A_PASSIVE_SKILL10);
    if (JobSkillPassOBJ[n_A_JOB][11] != 999) n_A_PassSkill[11] = eval(FORM_DATA.A_PASSIVE_SKILL11);
    if (JobSkillPassOBJ[n_A_JOB][12] != 999) n_A_PassSkill[12] = eval(FORM_DATA.A_PASSIVE_SKILL12);
    if (JobSkillPassOBJ[n_A_JOB][13] != 999) n_A_PassSkill[13] = eval(FORM_DATA.A_PASSIVE_SKILL13);
    if (JobSkillPassOBJ[n_A_JOB][14] != 999) n_A_PassSkill[14] = eval(FORM_DATA.A_PASSIVE_SKILL14);


    if (n_SkillSW) {
        n_A_PassSkill2[0] = eval(FORM_DATA.A_SUPPORTIVE_SKILL0);
        n_A_PassSkill2[1] = eval(FORM_DATA.A_SUPPORTIVE_SKILL1);
        n_A_PassSkill2[2] = eval(FORM_DATA.A_SUPPORTIVE_SKILL2);
        n_A_PassSkill2[3] = FORM_DATA.A_SUPPORTIVE_SKILL3 === "on";
        n_A_PassSkill2[4] = eval(FORM_DATA.A_SUPPORTIVE_SKILL4);
        n_A_PassSkill2[5] = FORM_DATA.A_SUPPORTIVE_SKILL5 === "on";
        n_A_PassSkill2[6] = eval(FORM_DATA.A_SUPPORTIVE_SKILL6);
        n_A_PassSkill2[7] = FORM_DATA.A_SUPPORTIVE_SKILL7 === "on";
        n_A_PassSkill2[8] = eval(FORM_DATA.A_SUPPORTIVE_SKILL8);
        n_A_PassSkill2[9] = eval(FORM_DATA.A_SUPPORTIVE_SKILL9);
        n_A_PassSkill2[10] = eval(FORM_DATA.A_SUPPORTIVE_SKILL10);
        n_A_PassSkill2[11] = FORM_DATA.A_SUPPORTIVE_SKILL11 === "on";
        n_A_PassSkill2[12] = FORM_DATA.A_SUPPORTIVE_SKILL12 === "on";
        n_A_PassSkill2[13] = eval(FORM_DATA.A_SUPPORTIVE_SKILL13);
        n_A_PassSkill2[14] = eval(FORM_DATA.A_SUPPORTIVE_SKILL14);
    }

    if (n_Skill3SW) {
        n_A_PassSkill3[0] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_1);
        n_A_PassSkill3[1] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_1);
        n_A_PassSkill3[2] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_1);
        n_A_PassSkill3[3] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_1);

        n_A_PassSkill3[5] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_1);

        n_A_PassSkill3[7] = eval(FORM_DATA.A_PERFORMANCE_SKILL7);

        n_A_PassSkill3[9] = eval(FORM_DATA.A_PERFORMANCE_SKILL9);
        n_A_PassSkill3[10] = eval(FORM_DATA.A_PERFORMANCE_SKILL10);
        n_A_PassSkill3[11] = FORM_DATA.A_PERFORMANCE_SKILL11 === "on";
        if (n_A_PassSkill3[11]) {
            n_A_PassSkill3[12] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_STR);
            n_A_PassSkill3[13] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_AGI);
            n_A_PassSkill3[14] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_VIT);
            n_A_PassSkill3[15] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_INT);
            n_A_PassSkill3[16] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_DEX);
            n_A_PassSkill3[17] = eval(FORM_DATA.A_PERFORMANCE_SKILL11_LUK);
        }

        if (n_A_PassSkill3[0]) {
            n_A_PassSkill3[20] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_2);
            n_A_PassSkill3[30] = eval(FORM_DATA.A_PERFORMANCE_SKILL0_3);
        }
        if (n_A_PassSkill3[1]) {
            n_A_PassSkill3[21] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_2);
            n_A_PassSkill3[31] = eval(FORM_DATA.A_PERFORMANCE_SKILL1_3);
        }
        if (n_A_PassSkill3[2]) {
            n_A_PassSkill3[22] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_2);
            n_A_PassSkill3[29] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_3);
            n_A_PassSkill3[32] = eval(FORM_DATA.A_PERFORMANCE_SKILL2_4);
        }
        if (n_A_PassSkill3[3]) {
            n_A_PassSkill3[23] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_2);
            n_A_PassSkill3[33] = eval(FORM_DATA.A_PERFORMANCE_SKILL3_3);
        }
        if (n_A_PassSkill3[4]) {
            n_A_PassSkill3[24] = eval(FORM_DATA.A_PERFORMANCE_SKILL4_2);
            n_A_PassSkill3[34] = eval(FORM_DATA.A_PERFORMANCE_SKILL4_3);
        }
        if (n_A_PassSkill3[5]) {
            n_A_PassSkill3[25] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_2);
            n_A_PassSkill3[35] = eval(FORM_DATA.A_PERFORMANCE_SKILL5_3);
        }
        if (n_A_PassSkill3[6]) {
            n_A_PassSkill3[26] = eval(FORM_DATA.A_PERFORMANCE_SKILL6_2);
            n_A_PassSkill3[36] = eval(FORM_DATA.A_PERFORMANCE_SKILL6_3);
        }

    }
    if (n_Skill4SW) {
        n_A_PassSkill3[40] = FORM_DATA.A_PERFORMANCE_SKILL40 === "on";
        n_A_PassSkill3[41] = eval(FORM_DATA.A_PERFORMANCE_SKILL41);
        n_A_PassSkill3[42] = eval(FORM_DATA.A_PERFORMANCE_SKILL42);
        n_A_PassSkill3[43] = eval(FORM_DATA.A_PERFORMANCE_SKILL43);
        n_A_PassSkill3[44] = eval(FORM_DATA.A_PERFORMANCE_SKILL44);
    }
    if (n_Skill5SW) {
        n_A_PassSkill5[0] = FORM_DATA.A_BATTLECHANT_SKILL0 === "on";
        n_A_PassSkill5[1] = FORM_DATA.A_BATTLECHANT_SKILL1 === "on";
        n_A_PassSkill5[2] = FORM_DATA.A_BATTLECHANT_SKILL2 === "on";
        n_A_PassSkill5[3] = FORM_DATA.A_BATTLECHANT_SKILL3 === "on";
        n_A_PassSkill5[4] = FORM_DATA.A_BATTLECHANT_SKILL4 === "on";
    }
    if (n_Skill6SW) {
        n_A_PassSkill6[0] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL0);
        n_A_PassSkill6[1] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL1);
        n_A_PassSkill6[2] = eval(FORM_DATA.A_GROUND_SUPPORTIVE_SKILL2);
        n_A_PassSkill6[3] = FORM_DATA.A_GROUND_SUPPORTIVE_SKILL3 === "on";
    }
    if (n_Skill7SW) {
        n_A_PassSkill7[0] = FORM_DATA.A_FOOD_BOX_BONUS0 === "on";
        n_A_PassSkill7[1] = FORM_DATA.A_FOOD_BOX_BONUS1 === "on";
        n_A_PassSkill7[2] = FORM_DATA.A_FOOD_BOX_BONUS2 === "on";
        n_A_PassSkill7[3] = eval(FORM_DATA.A_FOOD_BOX_BONUS3);
        n_A_PassSkill7[4] = eval(FORM_DATA.A_FOOD_BOX_BONUS4);
        n_A_PassSkill7[5] = eval(FORM_DATA.A_FOOD_BOX_BONUS5);
        n_A_PassSkill7[6] = eval(FORM_DATA.A_FOOD_BOX_BONUS6);
        n_A_PassSkill7[7] = eval(FORM_DATA.A_FOOD_BOX_BONUS7);
        n_A_PassSkill7[8] = eval(FORM_DATA.A_FOOD_BOX_BONUS8);
        n_A_PassSkill7[9] = FORM_DATA.A_FOOD_BOX_BONUS9 === "on";
        n_A_PassSkill7[10] = FORM_DATA.A_FOOD_BOX_BONUS10 === "on";
    }

    let targetStats = CalculateEnemyStats(FORM_DATA, false, n_A_card);

    let wSPC_STR = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][0];
    let wSPC_AGI = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][1];
    let wSPC_VIT = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][2];
    let wSPC_INT = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][3];
    let wSPC_DEX = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][4];
    let wSPC_LUK = JobStatsBonusPerLevel[n_A_JOB][n_A_JobLV - 1][5];

    if (n_A_JOB == 0 && isRebirth) {
        TenNovSTR = [0, 0, 0, 0, 0, 0, 0, 1, 1, 1];
        TenNovAGI = [0, 0, 0, 0, 1, 1, 1, 1, 1, 1];
        TenNovVIT = [0, 0, 0, 0, 0, 1, 1, 1, 1, 1];
        TenNovINT = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1];
        TenNovDEX = [0, 0, 1, 1, 1, 1, 1, 1, 1, 1];
        TenNovLUK = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        wSPC_STR = TenNovSTR[n_A_JobLV - 1];
        wSPC_AGI = TenNovAGI[n_A_JobLV - 1];
        wSPC_VIT = TenNovVIT[n_A_JobLV - 1];
        wSPC_INT = TenNovINT[n_A_JobLV - 1];
        wSPC_DEX = TenNovDEX[n_A_JobLV - 1];
        wSPC_LUK = TenNovLUK[n_A_JobLV - 1];
    }


    let wSPCall = StPlusItem(ALL_STATS, n_A_Equip);
    wSPC_STR += StPlusItem(STR, n_A_Equip) + wSPCall;
    wSPC_AGI += StPlusItem(AGI, n_A_Equip) + wSPCall;
    wSPC_VIT += StPlusItem(VIT, n_A_Equip) + wSPCall;
    wSPC_VIT += StPlusItem(213, n_A_Equip);
    wSPC_INT += StPlusItem(INT, n_A_Equip) + wSPCall;
    wSPC_INT += StPlusItem(214, n_A_Equip);
    wSPC_DEX += StPlusItem(DEX, n_A_Equip) + wSPCall;
    wSPC_LUK += StPlusItem(LUK, n_A_Equip) + wSPCall;

    wSPC_DEX += SkillSearch("Owl's Eye", n_A_JOB, n_A_PassSkill);
    wSPC_STR += SkillSearch("Crazy Uproar", n_A_JOB, n_A_PassSkill) * 4;
    wSPC_STR += SkillSearch("Hilt Binding", n_A_JOB, n_A_PassSkill);
    wSPC_STR += SkillSearch("Ninja Aura", n_A_JOB, n_A_PassSkill);
    wSPC_INT += SkillSearch("Ninja Aura", n_A_JOB, n_A_PassSkill);
    if (SkillSearch("Dragonology", n_A_JOB, n_A_PassSkill))
        wSPC_INT += (Math.floor(SkillSearch("Dragonology", n_A_JOB, n_A_PassSkill) / 2) + 1);
    if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill)) {
        if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill) == 5) wSPC_STR += 16;
        if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill) == 4) wSPC_STR += 8;
        if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill) == 3) wSPC_STR += 4;
        if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill) == 2) wSPC_STR += 2;
        if (SkillSearch("Chase Walk", n_A_JOB, n_A_PassSkill) == 1) wSPC_STR += 1;
    }
    if (SkillSearch("Increase Accuracy", n_A_JOB, n_A_PassSkill)) {
        wSPC_DEX += 4;
        wSPC_AGI += 4;
    }

    let w = SkillSearch("Improve Concentration", n_A_JOB, n_A_PassSkill);
    if (w) {
        w += 102;
        wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * w / 100) - n_A_DEX;
        wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * w / 100) - n_A_AGI;
    } else if (n_A_PassSkill6[3]) {
        wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * 103 / 100) - n_A_DEX;
        wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * 103 / 100) - n_A_AGI;
    }

    wSPC_AGI += StPlusItem(212, n_A_Equip);
    wSPC_DEX += StPlusItem(215, n_A_Equip);
    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Magistrate Hat", n_A_Equip))
        wSPC_AGI += 1;
    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Ayam", n_A_Equip))
        wSPC_INT += 1;
    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Bride Mask", n_A_Equip))
        wSPC_LUK += 2;
    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Mythical Lion Mask", n_A_Equip))
        wSPC_DEX += 2;
    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Hahoe Mask", n_A_Equip))
        wSPC_LUK += 1;
    if (n_A_SHOES_DEF_PLUS >= 9 && EquipNumSearch("Black Leather Boots", n_A_Equip))
        wSPC_AGI += 2;

    wSPCall = StPlusCard(ALL_STATS, n_A_card);
    wSPC_STR += StPlusCard(STR, n_A_card) + wSPCall;
    wSPC_AGI += StPlusCard(AGI, n_A_card) + wSPCall;
    wSPC_VIT += StPlusCard(VIT, n_A_card) + wSPCall;
    wSPC_INT += StPlusCard(INT, n_A_card) + wSPCall;
    wSPC_DEX += StPlusCard(DEX, n_A_card) + wSPCall;
    wSPC_LUK += StPlusCard(LUK, n_A_card) + wSPCall;


    if (n_A_JobSearch(n_A_JOB) == 3)
        wSPC_INT += CardNumSearch("Rideword", n_A_card);
    if (CardNumSearch("Despero of Thanatos", n_A_card)) wSPC_INT += n_A_LEFT_DEF_PLUS;
    if (CardNumSearch("Green Maiden", n_A_card)) wSPC_LUK += n_A_SHOULDER_DEF_PLUS;
    if (CardNumSearch("Odium of Thanatos", n_A_card)) wSPC_AGI += n_A_SHOES_DEF_PLUS;
    if (n_A_card[8] == 180) wSPC_STR += n_A_HEAD_DEF_PLUS;

    if (CardNumSearch("Obsidian", n_A_card)) wSPC_VIT += Math.floor(SU_DEX / 18);
    if (CardNumSearch("Egnigem Cenia", n_A_card)) wSPC_STR += Math.floor(SU_INT / 18);
    if (CardNumSearch("Venatu", n_A_card)) wSPC_LUK += Math.floor(SU_AGI / 18);
    if (CardNumSearch("Ancient Mimic", n_A_card)) wSPC_AGI += Math.floor(SU_LUK / 18);
    if (CardNumSearch("Mistress of Shelter", n_A_card)) wSPC_INT += Math.floor(SU_STR / 18);
    if (CardNumSearch("Dame of Sentinel", n_A_card)) wSPC_DEX += Math.floor(SU_VIT / 18);


    if (CardNumSearch("Aliot", n_A_card)) {
        if (n_A_JobSearch(n_A_JOB) == 1 || n_A_JobSearch(n_A_JOB) == 2 || n_A_JobSearch(n_A_JOB) == 6)
            wSPC_STR += 2;
        if (n_A_JobSearch(n_A_JOB) == 3 || n_A_JobSearch(n_A_JOB) == 4 || n_A_JobSearch(n_A_JOB) == 5)
            wSPC_INT += 2;
    }

    wSPC_STR += n_A_PassSkill2[0];
    wSPC_INT += n_A_PassSkill2[0];
    wSPC_DEX += n_A_PassSkill2[0];
    if (n_A_PassSkill2[1] > 0)
        wSPC_AGI += n_A_PassSkill2[1] + 2;
    wSPC_LUK += (n_A_PassSkill2[3] * 30);
    if (n_A_JOB == 24 && SkillSearch("True Sight", n_A_JOB, n_A_PassSkill)) {
        wSPC_STR += 5;
        wSPC_AGI += 5;
        wSPC_VIT += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
        wSPC_LUK += 5;
    }


    if (SkillSearch("Sprint (STR + State)", n_A_JOB, n_A_PassSkill))
        wSPC_STR += 10;


    if (n_A_PassSkill3[40]) {
        wSPC_STR += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
    }
    wSPC_STR += n_A_PassSkill3[41];
    wSPC_VIT += n_A_PassSkill3[42];
    wSPC_AGI += n_A_PassSkill3[43];
    wSPC_DEX += n_A_PassSkill3[44];

    if (n_A_PassSkill5[0]) {
        wSPC_STR += 20;
        wSPC_AGI += 20;
        wSPC_VIT += 20;
        wSPC_DEX += 20;
        wSPC_INT += 20;
        wSPC_LUK += 20;
    }

    if (n_A_PassSkill6[2] == 1) {
        wSPC_STR += 3;
        wSPC_AGI += 3;
        wSPC_VIT += 3;
        wSPC_DEX += 3;
        wSPC_INT += 3;
        wSPC_LUK += 3;
    }
    if (n_A_PassSkill6[2] == 2) {
        wSPC_STR += 5;
        wSPC_AGI += 5;
        wSPC_VIT += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
        wSPC_LUK += 5;
    }


    if (n_A_PassSkill7[3])
        wSPC_STR += n_A_PassSkill7[3];
    if (n_A_PassSkill7[4])
        wSPC_AGI += n_A_PassSkill7[4];
    if (n_A_PassSkill7[5])
        wSPC_VIT += n_A_PassSkill7[5];
    if (n_A_PassSkill7[6])
        wSPC_INT += n_A_PassSkill7[6];
    if (n_A_PassSkill7[7])
        wSPC_DEX += n_A_PassSkill7[7];
    if (n_A_PassSkill7[8])
        wSPC_LUK += n_A_PassSkill7[8];

    if (n_A_PassSkill3[11]) {
        if (n_A_STR + wSPC_STR < 99) {
            if (n_A_STR + wSPC_STR + Math.floor(n_A_PassSkill3[12] / 2) < 99)
                wSPC_STR += Math.floor(n_A_PassSkill3[12] / 2);
            else
                wSPC_STR = (99 - n_A_STR);
        }
        if (n_A_AGI + wSPC_AGI < 99) {
            if (n_A_AGI + wSPC_AGI + Math.floor(n_A_PassSkill3[13] / 2) < 99)
                wSPC_AGI += Math.floor(n_A_PassSkill3[13] / 2);
            else
                wSPC_AGI = (99 - n_A_AGI);
        }
        if (n_A_VIT + wSPC_VIT < 99) {
            if (n_A_VIT + wSPC_VIT + Math.floor(n_A_PassSkill3[14] / 2) < 99)
                wSPC_VIT += Math.floor(n_A_PassSkill3[14] / 2);
            else
                wSPC_VIT = (99 - n_A_VIT);
        }
        if (n_A_INT + wSPC_INT < 99) {
            if (n_A_INT + wSPC_INT + Math.floor(n_A_PassSkill3[15] / 2) < 99)
                wSPC_INT += Math.floor(n_A_PassSkill3[15] / 2);
            else
                wSPC_INT = (99 - n_A_INT);
        }
        if (n_A_DEX + wSPC_DEX < 99) {
            if (n_A_DEX + wSPC_DEX + Math.floor(n_A_PassSkill3[16] / 2) < 99)
                wSPC_DEX += Math.floor(n_A_PassSkill3[16] / 2);
            else
                wSPC_DEX = (99 - n_A_DEX);
        }
        if (n_A_LUK + wSPC_LUK < 99) {
            if (n_A_LUK + wSPC_LUK + Math.floor(n_A_PassSkill3[17] / 2) < 99)
                wSPC_LUK += Math.floor(n_A_PassSkill3[17] / 2);
            else
                wSPC_LUK = (99 - n_A_LUK);
        }
    }

    n_A_STR += wSPC_STR;
    n_A_AGI += wSPC_AGI;
    n_A_VIT += wSPC_VIT;
    n_A_INT += wSPC_INT;
    n_A_DEX += wSPC_DEX;
    n_A_LUK += wSPC_LUK;

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

    let baseATK_w, baseATK;
    if (isNonRangeWeapon()) {
        baseATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        baseATK = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
    } else {
        baseATK_w = Math.round(Math.floor(n_A_DEX / 10) * Math.floor(n_A_DEX / 10));
        baseATK = n_A_DEX + baseATK_w + Math.floor(n_A_STR / 5) + Math.floor(n_A_LUK / 5);
    }


    w = StPlusCard(ATK, n_A_card);
    w += StPlusItem(ATK, n_A_Equip);

    if (SU_STR >= 80 && CardNumSearch("Giant Whisper", n_A_card))
        w += 20;
    if (SU_STR >= 95 && EquipNumSearch("Doom Slayer", n_A_Equip))
        w += 340;
    if (SU_STR >= 44 && EquipNumSearch("Holgren's Refining Hammer", n_A_Equip))
        w += 44;
    if (EquipNumSearch("Mythical Lion Mask", n_A_Equip))
        w += n_A_HEAD_DEF_PLUS * 2;

    if (n_A_PassSkill6[0] == 0 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Pasana", n_A_card) || n_A_Equip[6] == 428 || n_A_Equip[6] == 604))
        w += n_A_PassSkill6[1] * 10;

    if (n_A_PassSkill7[2])
        w += 10;
    if (n_A_PassSkill7[9])
        w += 20;

    if (SkillSearch("Last Stand", n_A_JOB, n_A_PassSkill))
        w += 100;
    if (SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill))
        w += 20 + 10 * SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill);

    if (n_A_PassSkill3[9])
        w += 25 + 25 * n_A_PassSkill3[9];


    baseATK += w;


    let JobHP_A = new Array(0, 70, 50, 40, 50, 30, 40, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 0, 0, 0, 0, 0, 0, 70, 90, 75, 75, 84);
    let JobHP_B = new Array(5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 5, 6.5, 5, 3, 3.5);


    let wHPSL = 0;
    if (n_A_JOB == 43) {
        if (n_A_BaseLV >= 70) {
            if (n_A_BaseLV <= 79)
                wHPSL = (n_A_BaseLV - 70) * 40;
            else if (n_A_BaseLV <= 84)
                wHPSL = (n_A_BaseLV - 80) * 50;
            else if (n_A_BaseLV <= 89)
                wHPSL = (n_A_BaseLV - 80) * 50 - 10;
            else if (n_A_BaseLV <= 92)
                wHPSL = (n_A_BaseLV - 90) * 50;
            else if (n_A_BaseLV <= 97)
                wHPSL = (n_A_BaseLV - 90) * 50 - 10;
            else if (n_A_BaseLV == 98) wHPSL = 375;
            else wHPSL = 4;
        }
    }

    w = 0;
    for (let i = 2; i <= n_A_BaseLV; i++) {
        w += Math.round(JobHP_A[n_A_JOB] * i / 100);
    }

    let n_A_MaxHP = Math.floor((JobHP_B[n_A_JOB] * n_A_BaseLV) + 35 + w);


    if (n_A_JOB == 44) {
        NinHP = new Array(131, 137, 144, 151, 159, 167, 175, 184, 193, 202, 212, 222, 232, 243, 254, 265, 277, 289, 301, 316, 331, 346, 364, 382, 400, 420, 440, 460, 482, 504, 526, 548, 572, 596, 620, 646, 672, 698, 726, 754, 784, 814, 844, 876, 908, 940, 975, 1010, 1100, 1140, 1180, 1220, 1260, 1300, 1340, 1385, 1430, 1475, 1520, 1565, 1615, 1665, 1715, 1765, 1815, 1880, 1935, 1990, 2045, 2100, 2160, 2200, 2280, 2340, 2400, 2460, 2520, 2580, 2640, 2705, 2770, 2835, 2900, 2965, 3030, 3100, 3170, 3240, 3310, 3380, 3455, 3530, 3605, 3680, 3760, 3840, 3920, 4000, 4080);
        n_A_MaxHP = NinHP[n_A_BaseLV - 1];
    }

    if (n_A_JOB == 45 && n_A_BaseLV >= 10) {
        GunHP = new Array(202, 212, 222, 232, 243, 254, 265, 277, 289, 301, 316, 331, 346, 364, 382, 400, 420, 440, 460, 490, 520, 550, 580, 610, 650, 680, 710, 740, 770, 800, 830, 860, 890, 920, 950, 990, 1020, 1050, 1080, 1110, 1140, 1180, 1230, 1280, 1330, 1395, 1455, 1515, 1575, 1635, 1695, 1760, 1820, 1885, 1950, 2015, 2080, 2145, 2210, 2275, 2340, 2410, 2480, 2550, 2620, 2690, 2760, 2830, 2900, 2970, 3040, 3115, 3190, 3265, 3340, 3415, 3490, 3565, 3640, 3715, 3790, 3870, 3950, 4030, 4110, 4190, 4270, 4350, 4430, 4510);
        n_A_MaxHP = GunHP[n_A_BaseLV - 10];
    }

    if (n_A_JOB == 20 && n_A_BaseLV == 99)
        n_A_MaxHP += 2000;

    if (isRebirth)
        n_A_MaxHP = Math.floor(n_A_MaxHP * 125 / 100);
    if (FORM_DATA.isAdopted == "on")
        n_A_MaxHP = Math.floor(n_A_MaxHP * 70 / 100);
    n_A_MaxHP = Math.floor((n_A_MaxHP - wHPSL) * (100 + n_A_VIT) / 100);


    if (n_A_JOB == 41 && n_A_BaseLV >= 70) {
        if (n_A_BaseLV <= 79)
            n_A_MaxHP = Math.floor((2127 + 10 * (n_A_BaseLV - 70)) * (100 + n_A_VIT) / 100);
        else if (n_A_BaseLV <= 89)
            n_A_MaxHP = Math.floor((2200 + 50 * (n_A_BaseLV - 80)) * (100 + n_A_VIT) / 100);
        else if (n_A_BaseLV <= 99)
            n_A_MaxHP = Math.floor((2700 + 50 * (n_A_BaseLV - 90)) * (100 + n_A_VIT) / 100);
    }

    if (n_A_JOB == 42 && n_A_BaseLV >= 70) {
        wKenseiHP = [3455, 3524, 3593, 3663, 3834, 3806, 3878, 3951, 4025, 4500];
        if (n_A_BaseLV <= 79)
            n_A_MaxHP = Math.floor((2670 + 10 * (n_A_BaseLV - 70)) * (100 + n_A_VIT) / 100);
        else if (n_A_BaseLV <= 89)
            n_A_MaxHP = Math.floor((3000 + 20 * (n_A_BaseLV - 80)) * (100 + n_A_VIT) / 100);
        else if (n_A_BaseLV <= 99)
            n_A_MaxHP = Math.floor(wKenseiHP[n_A_BaseLV - 90] * (100 + n_A_VIT) / 100);
    }

    if (SkillSearch("Taekwon Ranker", n_A_JOB, n_A_PassSkill) && n_A_BaseLV >= 90)
        n_A_MaxHP *= 3;


    n_A_MaxHP += SkillSearch("Faith", n_A_JOB, n_A_PassSkill) * 200;
    let bkHP = n_A_MaxHP;
    w = 0;

    w += StPlusItem(MAXHP, n_A_Equip);
    w += StPlusItem(VIT, n_A_Equip);


    w += StPlusCard(MAXHP, n_A_card);
    if (n_A_BODY_DEF_PLUS >= 9 && CardNumSearch("Apocalypse", n_A_card))
        w += 800;

    //Temporary remover card code.
    if (CardNumSearch("Remover", n_A_card))
        w -= n_A_BODY_DEF_PLUS * 40;

    if (n_A_Equip[8] == 536) {
        wHPVS = n_A_JobSearch(n_A_JOB);
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5)
            w += 5 * n_A_BaseLV;
    }

    n_A_MaxHP += w;

    w = 0;

    w += StPlusItem(MAXHP_PERCENTAGE, n_A_Equip);

    w += StPlusCard(MAXHP_PERCENTAGE, n_A_card);

    if (SU_VIT >= 80 && CardNumSearch("Giant Whisper", n_A_card))
        w += 3;

    if (CardNumSearch("Aliot", n_A_card)) {
        if (n_A_JobSearch(n_A_JOB) == 1 || n_A_JobSearch(n_A_JOB) == 2 || n_A_JobSearch(n_A_JOB) == 6)
            w += 5;
    }
    if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch("Firelock Soldier", n_A_card))
        w += 10;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus", n_A_card))
        w += 4;
    if (n_A_PassSkill5[1])
        w += 100;
    if (EquipNumSearch("Variant Shoes", n_A_Equip))
        w -= n_A_SHOES_DEF_PLUS;

    n_A_MaxHP = n_A_MaxHP * (100 + w) / 100;

    if (n_A_PassSkill6[0] == 1 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Swordfish", n_A_card) || n_A_Equip[6] == 429 || n_A_Equip[6] == 605)) {
        dHP = [5, 9, 12, 14, 15];
        n_A_MaxHP = n_A_MaxHP * (100 + dHP[n_A_PassSkill6[1] - 1]) / 100;
    }
    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill))
        n_A_MaxHP *= 3;


    if (n_A_PassSkill3[3])
        n_A_MaxHP += (Math.floor(bkHP * (105 + n_A_PassSkill3[3] * 2 + n_A_PassSkill3[33] + Math.floor(n_A_PassSkill3[23] / 10)) / 100) - bkHP);


    n_A_MaxHP = Math.floor(n_A_MaxHP);


    // if (n_A_MaxHP >= 100) {
    //     if (n_A_MaxHP >= 10000)
    //         myInnerHtml("A_MaxHP", " " + n_A_MaxHP, 0);
    //     else
    //         myInnerHtml("A_MaxHP", n_A_MaxHP, 0);
    // } else
    //     myInnerHtml("A_MaxHP", " " + n_A_MaxHP, 0);


    let JobSP_A = new Array(1, 2, 2, 5, 2, 6, 3, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 1, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 0, 0, 0, 0, 0, 0, 0, 2, 4.7, 9, 3.75, 3.75);


    let wSPSL = 0;
    if (n_A_JOB == 41 || n_A_JOB == 43) {
        if (n_A_BaseLV >= 70) {
            if (n_A_BaseLV < 80)
                wSPSL = (n_A_BaseLV - 70) * 4 + 5;
            else if (n_A_BaseLV < 90)
                wSPSL = (n_A_BaseLV - 80) * 4;
            else if (n_A_BaseLV < 93)
                wSPSL = (n_A_BaseLV - 90) * 4;
            else if (n_A_BaseLV < 99)
                wSPSL = (n_A_BaseLV - 90) * 4 - 10;
            else wSPSL = 1;
        }
    }

    let n_A_MaxSP = Math.floor(10 + n_A_BaseLV * JobSP_A[n_A_JOB] - wSPSL);

    if (n_A_JOB == 44) {
        if (n_A_BaseLV <= 20) n_A_MaxSP = 11 + n_A_BaseLV * 3;
        else if (n_A_BaseLV <= 40) n_A_MaxSP = 71 + (n_A_BaseLV - 20) * 4;
        else if (n_A_BaseLV <= 60) n_A_MaxSP = 151 + (n_A_BaseLV - 40) * 5;
        else if (n_A_BaseLV <= 80) n_A_MaxSP = 251 + (n_A_BaseLV - 60) * 6;
        else n_A_MaxSP = 370 + (n_A_BaseLV - 80) * 8;
    }

    if (n_A_JOB == 45) {
        if (n_A_BaseLV <= 25) n_A_MaxSP = 10 + n_A_BaseLV * 3;
        else if (n_A_BaseLV <= 35) n_A_MaxSP = 85 + (n_A_BaseLV - 25) * 4;
        else if (n_A_BaseLV <= 40) n_A_MaxSP = 126 + (n_A_BaseLV - 35) * 3;
        else if (n_A_BaseLV <= 50) n_A_MaxSP = 141 + (n_A_BaseLV - 40) * 4;
        else if (n_A_BaseLV <= 75) n_A_MaxSP = 181 + (n_A_BaseLV - 50) * 5;
        else if (n_A_BaseLV <= 78) n_A_MaxSP = 306 + (n_A_BaseLV - 75) * 6;
        else n_A_MaxSP = 330 + (n_A_BaseLV - 78) * 6;
    }

    if (isRebirth)
        n_A_MaxSP = Math.floor(n_A_MaxSP * 125 / 100);
    if (eval(FORM_DATA.isAdopted == "on"))
        n_A_MaxSP = Math.floor(n_A_MaxSP * 70 / 100);
    n_A_MaxSP = Math.floor(n_A_MaxSP * (100 + n_A_INT) / 100);


    if (n_A_JOB == 42 && n_A_BaseLV >= 70) {
        if (n_A_BaseLV <= 79)
            n_A_MaxSP = Math.floor((340 + 2 * (n_A_BaseLV - 70)) * (100 + n_A_INT) / 100);
        else if (n_A_BaseLV <= 89)
            n_A_MaxSP = Math.floor((385 + 2 * (n_A_BaseLV - 80)) * (100 + n_A_INT) / 100);
        else if (n_A_BaseLV <= 99)
            n_A_MaxSP = Math.floor((437 + 2 * (n_A_BaseLV - 90)) * (100 + n_A_INT) / 100);
    }

    let bkSP = n_A_MaxSP;

    if (SkillSearch("Taekwon Ranker", n_A_JOB, n_A_PassSkill) && n_A_BaseLV >= 90)
        n_A_MaxSP *= 3;

    w = 0;

    w += StPlusItem(MAXSP, n_A_Equip);
    w += StPlusItem(INT, n_A_Equip);

    w += StPlusCard(MAXSP, n_A_card);
    if (n_A_HEAD_DEF_PLUS >= 9 && n_A_card[8] == 298)
        w += 150;
    if (n_A_HEAD_DEF_PLUS <= 4 && n_A_card[8] == 179)
        w += 40;
    if (n_A_card[9] == 179)
        w += 40;

    if (SkillSearch("Kaina", n_A_JOB, n_A_PassSkill))
        w += 30 * SkillSearch("Kaina", n_A_JOB, n_A_PassSkill);

    if (n_A_Equip[8] == 536) {
        wSPVS = n_A_JobSearch(n_A_JOB);
        if (wSPVS == 1 || wSPVS == 2 || wSPVS == 6)
            w += 2 * n_A_JobLV;
    }
    if (weaponRefinementLevel >= 9 && EquipNumSearch("Lich's Bone Wand", n_A_Equip))
        w += 300;


    n_A_MaxSP += w;

    w = 0;

    w += StPlusItem(MAXSP_PERCENTAGE, n_A_Equip);

    w += StPlusCard(MAXSP_PERCENTAGE, n_A_card);
    if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch("Firelock Soldier", n_A_card))
        w += 10;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus", n_A_card))
        w += 4;

    if (CardNumSearch("Aliot", n_A_card)) {
        if (n_A_JobSearch(n_A_JOB) == 3 || n_A_JobSearch(n_A_JOB) == 4 || n_A_JobSearch(n_A_JOB) == 5)
            w += 5;
    }


    w += SkillSearch("Mediatio", n_A_JOB, n_A_PassSkill);

    w += SkillSearch("Soul Drain", n_A_JOB, n_A_PassSkill) * 2;
    if (n_A_PassSkill5[2])
        w += 100;
    if (EquipNumSearch("Variant Shoes", n_A_Equip))
        w -= n_A_SHOES_DEF_PLUS;

    n_A_MaxSP = Math.floor(n_A_MaxSP * (100 + w) / 100);

    if (n_A_PassSkill3[6])
        n_A_MaxSP += (Math.floor(bkSP * (100 + n_A_PassSkill3[6] * 2 + n_A_PassSkill3[36] + Math.floor(n_A_PassSkill3[26] / 10)) / 100) - bkSP);


    // if (n_A_MaxSP >= 100)
    //     myInnerHtml("A_MaxSP", n_A_MaxSP, 0);
    // else
    //     myInnerHtml("A_MaxSP", " " + n_A_MaxSP, 0);


    let n_A_DEF = StPlusItem(DEF, n_A_Equip);

    for (let i = 2; i <= 10; i++) {
        n_A_DEF += ItemOBJ[n_A_Equip[i]][3];
    }

    n_A_DEF += StPlusCard(DEF, n_A_card);

    if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch("Arcluse", n_A_card))
        n_A_DEF += 2;
    if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch("Goat", n_A_card))
        n_A_DEF += 2;
    if (n_A_Equip[0] == 521) {
        if (weaponRefinementLevel <= 5)
            n_A_DEF += 2;
        else if (weaponRefinementLevel >= 9)
            n_A_DEF += 5;
        else
            n_A_DEF += 3;
    }
    if (EquipNumSearch("Gatekeeper-DD", n_A_Equip))
        n_A_DEF += weaponRefinementLevel;
    if (EquipNumSearch("Variant Shoes", n_A_Equip))
        n_A_DEF += n_A_SHOES_DEF_PLUS;
    if (EquipNumSearch("0", n_A_Equip) && n_A_JobSearch(n_A_JOB) == 1)
        n_A_DEF += 6;

    if (EquipNumSearch("0", n_A_Equip))
        n_A_DEFplus -= (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

    let n_A_totalDEF = n_A_DEF + Math.round(n_A_DEFplus * 7 / 10);

    if (StPlusItem(REDUCE_DEFENSE, n_A_Equip) + StPlusCard(REDUCE_DEFENSE, n_A_card))
        n_A_totalDEF = Math.floor(n_A_totalDEF / StPlusItem(REDUCE_DEFENSE, n_A_Equip), n_A_Equip);
    if (StPlusItem(LOWER_DEFENCE_PERCENTAGE, n_A_Equip) + StPlusCard(LOWER_DEFENCE_PERCENTAGE, n_A_card))
        n_A_totalDEF -= Math.floor(n_A_totalDEF * (StPlusItem(LOWER_DEFENCE_PERCENTAGE, n_A_Equip) + StPlusCard(LOWER_DEFENCE_PERCENTAGE, n_A_card)) / 100, n_A_Equip);

    if (SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill))
        n_A_totalDEF = Math.floor(n_A_totalDEF * (1 - 0.05 * SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill)));


    if (SkillSearch("Mental Strength", n_A_JOB, n_A_PassSkill))
        n_A_totalDEF = 90;

    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill))
        n_A_totalDEF = 0;


    // myInnerHtml("A_totalDEF", n_A_totalDEF, 0);


    let n_A_VITDEF = new Array();
    n_A_VITDEF[0] = Math.floor(n_A_VIT * 0.5) + Math.floor(n_A_VIT * 0.3);
    n_A_VITDEF[2] = Math.floor(n_A_VIT * 0.5) + Math.floor(n_A_VIT * n_A_VIT / 150) - 1;
    if (n_A_VITDEF[2] > n_A_VITDEF[0]) {
        n_A_VITDEF[1] = (n_A_VITDEF[0] + n_A_VITDEF[2]) / 2;
    } else {
        n_A_VITDEF[1] = n_A_VITDEF[0];
        n_A_VITDEF[2] = n_A_VITDEF[0];
    }
    if (n_A_PassSkill3[9]) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] += 2 + 2 * n_A_PassSkill3[9];
    }
    if (SkillSearch("Auto Berserk", n_A_JOB, n_A_PassSkill)) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.45);
    } else {
        if (n_A_PassSkill2[12]) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.9);
        }
    }
    if (StPlusItem(REDUCE_DEFENSE, n_A_Equip)) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] / StPlusItem(24), n_A_Equip);
    }
    if (SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill)) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 - 0.05 * SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill)));
    }
    if (n_A_PassSkill2[4]) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 + 0.05 * n_A_PassSkill2[4]));
    }
    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill)) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = 0;
    }


    let n_A_MDEF = StPlusItem(MDEF, n_A_Equip);


    n_A_MDEF += StPlusCard(MDEF, n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 3)
        n_A_MDEF += CardNumSearch("Rideword", n_A_card);
    if (n_A_LEFT_DEF_PLUS >= 9 && CardNumSearch("Sting", n_A_card))
        n_A_MDEF += 5;
    if (n_A_HEAD_DEF_PLUS <= 5 && n_A_card[8] == 213)
        n_A_MDEF += 5;
    if (n_A_card[9] == 213)
        n_A_MDEF += 5;
    if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch("Arcluse", n_A_card))
        n_A_MDEF += 3;
    if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch("Goat", n_A_card))
        n_A_MDEF += 5;
    if (n_A_SHOES_DEF_PLUS <= 5 && CardNumSearch("Megalith", n_A_card))
        n_A_MDEF += 7;
    if (n_A_SHOULDER_DEF_PLUS <= 5 && CardNumSearch("Kappa", n_A_card))
        n_A_MDEF += 8;
    if (EquipNumSearch("0", n_A_Equip))
        n_A_MDEF += (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

    if (SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill))
        n_A_MDEF += 1;
    if (SkillSearch("Endure", n_A_JOB, n_A_PassSkill))
        n_A_MDEF += SkillSearch("Endure", n_A_JOB, n_A_PassSkill);


    if (SkillSearch("Mental Strength", n_A_JOB, n_A_PassSkill))
        n_A_MDEF = 90;
    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill))
        n_A_MDEF = 0;

    // myInnerHtml("A_MDEF", n_A_MDEF, 0);


    let n_A_HIT = n_A_BaseLV + n_A_DEX;


    n_A_HIT += StPlusItem(HIT, n_A_Equip);


    n_A_HIT += StPlusCard(HIT, n_A_card);

    if (EquipNumSearch("Jungle Carbine", n_A_Equip))
        w -= Math.floor(SU_DEX / 3);


    n_A_HIT += 1 * SkillSearch("Vulture's Eye", n_A_JOB, n_A_PassSkill);
    n_A_HIT += 2 * SkillSearch("Weaponry Research", n_A_JOB, n_A_PassSkill);
    n_A_HIT += 3 * SkillSearch("True Sight", n_A_JOB, n_A_PassSkill);

    n_A_HIT += 10 * SkillSearch("Spear Dynamo", n_A_JOB, n_A_PassSkill);
    n_A_HIT += 1 * SkillSearch("Snake Eyes", n_A_JOB, n_A_PassSkill);
    if (SkillSearch("Gunslinger's Panic", n_A_JOB, n_A_PassSkill))
        n_A_HIT -= 30;
    if (SkillSearch("Increase Accuracy", n_A_JOB, n_A_PassSkill))
        n_A_HIT += 20;
    n_A_HIT += 2 * SkillSearch("Single Action", n_A_JOB, n_A_PassSkill);

    if (EquipNumSearch("Western Outlaw", n_A_Equip))
        n_A_HIT += Math.floor(SU_AGI / 5);

    if (skillToUseName == "Rapid Smiting")
        n_A_HIT += 20;

    if (n_A_PassSkill5[4])
        n_A_HIT += 50;

    if (n_A_PassSkill7[0])
        n_A_HIT += 30;


    if (n_A_PassSkill3[4])
        n_A_HIT += n_A_PassSkill3[4] + Math.floor(n_A_PassSkill3[34] / 2) + Math.floor(n_A_PassSkill3[24] / 10);


    // myInnerHtml("A_HIT", n_A_HIT, 0);


    let n_A_FLEE = n_A_BaseLV + n_A_AGI;


    n_A_FLEE += StPlusItem(FLEE, n_A_Equip);


    n_A_FLEE += StPlusCard(FLEE, n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 2 && CardNumSearch("Wanderer", n_A_card))
        n_A_FLEE += 20;
    if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch("Ninetails", n_A_card))
        n_A_FLEE += 20;
    if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch("Kavach Icarus", n_A_card))
        n_A_FLEE += 10;
    if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch("Orc Baby", n_A_card))
        n_A_FLEE += 5;

    if (n_A_PassSkill6[0] == 2 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Dokkebi", n_A_card) || n_A_Equip[6] == 430 || n_A_Equip[6] == 606))
        n_A_FLEE += n_A_PassSkill6[1] * 3;

    if (n_A_Equip[0] == 483)
        n_A_FLEE -= (n_A_BaseLV + SU_AGI);


    if (n_A_JOB == 8 || n_A_JOB == 14 || n_A_JOB == 22 || n_A_JOB == 28)
        n_A_FLEE += 4 * SkillSearch("Improve Dodge", n_A_JOB, n_A_PassSkill);
    else
        n_A_FLEE += 3 * SkillSearch("Improve Dodge", n_A_JOB, n_A_PassSkill);

    if (SkillSearch("Gunslinger's Panic", n_A_JOB, n_A_PassSkill))
        n_A_FLEE += 30;
    if (SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill))
        n_A_FLEE -= 5 * SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill);

    let Mikiri = new Array(0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15);
    n_A_FLEE += Mikiri[SkillSearch("Flee", n_A_JOB, n_A_PassSkill)];


    if (n_A_JOB == 24)
        n_A_FLEE += Math.round(SkillSearch("Wind Walk", n_A_JOB, n_A_PassSkill) / 2);
    if (n_A_PassSkill2[9] && SkillSearch("Wind Walk", n_A_JOB, n_A_PassSkill) == 0)
        n_A_FLEE += Math.round(n_A_PassSkill2[9] / 2);


    if (SkillSearch("Close Confine", n_A_JOB, n_A_PassSkill))
        n_A_FLEE += 10;


    if (SkillSearch("Lunar Protection", n_A_JOB, n_A_PassSkill))
        n_A_FLEE += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);

    if (n_A_PassSkill5[4])
        n_A_FLEE += 50;

    if (n_A_PassSkill7[1])
        n_A_FLEE += 30;


    if (n_A_PassSkill3[0])
        n_A_FLEE += n_A_PassSkill3[0] + Math.floor(n_A_PassSkill3[30] / 2) + Math.floor(n_A_PassSkill3[20] / 10);

    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill))
        n_A_FLEE /= 2;


    // myInnerHtml("A_FLEE", n_A_FLEE, 0);


    let n_A_LUCKY = 1 + n_A_LUK * 0.1;


    n_A_LUCKY += StPlusItem(PERFECT_DODGE, n_A_Equip);

    n_A_LUCKY += StPlusCard(PERFECT_DODGE, n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 2)
        n_A_LUCKY += 5 * CardNumSearch("Wild Rose", n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 1)
        n_A_LUCKY += 4 * CardNumSearch("Heater", n_A_card);
    if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch("Kavach Icarus", n_A_card))
        n_A_LUCKY += 1;
    if (n_A_Equip[7] == 535) {
        wHPVS = n_A_JobSearch(n_A_JOB);
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5) {
            n_A_LUCKY += 5;
            n_A_LUCKY += n_A_SHOULDER_DEF_PLUS * 2;
        }
    }

    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Hahoe Mask", n_A_Equip))
        n_A_LUCKY += 2;

    n_A_LUCKY = Math.round(n_A_LUCKY * 10) / 10;


    // myInnerHtml("A_LUCKY", n_A_LUCKY, 0);


    let n_A_CRI = 1 + n_A_LUK * 0.3;


    n_A_CRI += StPlusItem(CRIT, n_A_Equip);

    w = 0;
    w += StPlusCard(CRIT, n_A_card);

    w += StPlusCard(CRITICAL_AGAINST_RACE_PERCENTAGE + targetStats.race, n_A_card);

    if (CardNumSearch("Green Maiden", n_A_card))
        w += n_A_SHOULDER_DEF_PLUS;
    if (n_A_JobSearch(n_A_JOB) == 2)
        w += 4 * CardNumSearch("Mobster", n_A_card);
    if (n_A_JobSearch(n_A_JOB) == 3) {
        if (targetStats.race == 1 || targetStats.race == 6)
            w += 9 * CardNumSearch("Fur Seal", n_A_card);
    }
    if (SU_LUK >= 80 && CardNumSearch("Giant Whisper", n_A_card))
        w += 3;
    if (EquipNumSearch("Giant Encyclopedia", n_A_Equip))
        w += Math.floor(SU_LUK / 5);
    if (EquipNumSearch("Sniping Suit", n_A_Equip))
        w += Math.floor(SU_LUK / 5);
    if (EquipNumSearch("Sniping Suit *", n_A_Equip))
        w += Math.floor(n_A_LUK / 5);

    if (EquipNumSearch("Sabath", n_A_Equip) && 90 <= targetStats.element)
        w += 50;

    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Bride Mask", n_A_Equip))
        w += 5;
    if (EquipNumSearch("Heart Breaker", n_A_Equip))
        w += weaponRefinementLevel;


    if (n_A_WeaponType == WEAPON_TYPE_BOW && n_A_Arrow == 15)
        w += 20;


    if (SkillSearch("Fury", n_A_JOB, n_A_PassSkill))
        w += 7.5 + SkillSearch("Fury", n_A_JOB, n_A_PassSkill) * 2.5;
    if (SkillSearch("Fury (SuperNovice)", n_A_JOB, n_A_PassSkill))
        w += 50;
    if (n_A_JOB == 24)
        w += SkillSearch("True Sight", n_A_JOB, n_A_PassSkill);
    n_A_CRI += w;

    if (n_A_PassSkill3[5])
        n_A_CRI += 10 + n_A_PassSkill3[5] + Math.floor(n_A_PassSkill3[35] / 2) + Math.floor(n_A_PassSkill3[25] / 10);


    if (n_A_WeaponType == WEAPON_TYPE_KATAR)
        n_A_CRI *= 2;

    n_A_CRI = Math.round(n_A_CRI * 10) / 10;


    // myInnerHtml("A_CRI", n_A_CRI, 0);


    let n_A_MATK = [0, 0, 0];

    w = Math.floor(n_A_INT / 7);
    n_A_MATK[0] = n_A_INT + w * w;


    w = Math.floor(n_A_INT / 5);
    n_A_MATK[2] = n_A_INT + w * w;

    let w_MATK = 100;

    w_MATK += StPlusItem(MATK_PERCENTAGE, n_A_Equip);

    if (weaponRefinementLevel >= 9 && EquipNumSearch("Lich's Bone Wand", n_A_Equip))
        w_MATK += 3;
    if (EquipNumSearch("Staff of Destruction", n_A_Equip))
        w_MATK += Math.floor(weaponRefinementLevel / 2);
    if (EquipNumSearch("0", n_A_Equip) || EquipNumSearch("0", n_A_Equip))
        w_MATK += weaponRefinementLevel;
    if (n_A_PassSkill6[2])
        w_MATK += 10;

    if (n_A_JobSearch(n_A_JOB) == 5 && CardNumSearch("0", n_A_card))
        w_MATK += 3;
    if (n_A_HEAD_DEF_PLUS >= 9 && n_A_card[8] == 177)
        w_MATK += 2;
    if (n_A_Equip[0] == 484 && SU_INT >= 70)
        w_MATK += 5;
    n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
    n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);

    if (n_A_PassSkill7[2]) {
        n_A_MATK[0] += 10;
        n_A_MATK[2] += 10;
    }
    if (n_A_PassSkill7[10]) {
        n_A_MATK[0] += 20;
        n_A_MATK[2] += 20;
    }

    w_MATK = 100;

    w_MATK += StPlusItem(MATK_BASED_ON_STAFF_PERCENTAGE, n_A_Equip);

    n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
    n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);


    // myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);

    if (SkillSearch("Mystical Amplification", n_A_JOB, n_A_PassSkill)) {
        AmpMinMatkBK = n_A_MATK[0];
        AmpMaxMatkBK = n_A_MATK[2];
        n_A_MATK[0] = Math.floor(n_A_MATK[0] * (1 + 0.05 * SkillSearch("Mystical Amplification", n_A_JOB, n_A_PassSkill)));
        n_A_MATK[2] = Math.floor(n_A_MATK[2] * (1 + 0.05 * SkillSearch("Mystical Amplification", n_A_JOB, n_A_PassSkill)));

        myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);
        if (skillToUseName == "Stave Crasher") {
            n_A_MATK[0] = AmpMinMatkBK;
            n_A_MATK[2] = AmpMaxMatkBK;
        }
    }


    if (n_A_MATK[0] != n_A_MATK[2])
        n_A_MATK[2] -= 1;

    n_A_MATK[1] = (n_A_MATK[2] + n_A_MATK[0]) / 2;

    let wASPD;

    if (hasLeftHand == 1)
        wASPD = (200 - (JobASPD[n_A_JOB][n_A_WeaponType] + JobASPD[n_A_JOB][n_A_Weapon2Type]) / 2) * 1.4;
    else
        wASPD = 200 - JobASPD[n_A_JOB][n_A_WeaponType];


    if (hasLeftHand == 1 && n_A_WeaponType == WEAPON_TYPE_UNARMED && n_A_Weapon2Type != 0)
        wASPD = 200 - JobASPD[n_A_JOB][n_A_Weapon2Type];

    let n_A_ASPD = 200 - wASPD + (Math.floor(wASPD * n_A_AGI * 4 / 100) + Math.floor(wASPD * n_A_DEX / 100)) / 10;

    if (n_A_Equip[0] == 47)
        n_A_ASPD += 2;


    if (SkillSearch("Cavalier Mastery", n_A_JOB, n_A_PassSkill) && (skillToUseName == "Basic Attack" || skillToUseName == "Martyr's Reconing"))
        n_A_ASPD -= (6 - SkillSearch("Cavalier Mastery", n_A_JOB, n_A_PassSkill)) * 10;

    n_A_ASPD += Math.round(SkillSearch("Single Action", n_A_JOB, n_A_PassSkill) / 2);


    w = 0;
    let ASPDch = 0;
    if (n_A_WeaponType == WEAPON_TYPE_TWO_HANDED_SWORD && SkillSearch("Twohand Quicken", n_A_JOB, n_A_PassSkill)) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_WeaponType == WEAPON_TYPE_SWORD && SkillSearch("One Hand Quicken (Soul Linked)", n_A_JOB, n_A_PassSkill)) {
        w += 30;
        ASPDch = 1;
    }
    if (6 <= n_A_WeaponType && n_A_WeaponType <= 8 && SkillSearch("Andrenaline Rush", n_A_JOB, n_A_PassSkill)) {
        w += 30;
        ASPDch = 1;
    }
    if (ASPDch == 0 && SkillSearch("Full Andrenaline Rush", n_A_JOB, n_A_PassSkill)) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_WeaponType == WEAPON_TYPE_TWO_HANDED_SPEAR && SkillSearch("Spear Quicken", n_A_JOB, n_A_PassSkill)) {
        w += SkillSearch("Spear Quicken", n_A_JOB, n_A_PassSkill) + 20;
        ASPDch = 1;
    }
    if (EquipNumSearch("Western Outlaw", n_A_Equip))
        w += Math.floor(SU_AGI / 5);
    if (n_A_Equip[0] == 484 && SU_STR >= 50)
        w += 5;
    if (SU_STR >= 95 && EquipNumSearch("Doom Slayer", n_A_Equip))
        w -= 40;
    if (EquipNumSearch("Hurricane Fury", n_A_Equip))
        w += (weaponRefinementLevel * 2);
    if (EquipNumSearch("Book of the Dead", n_A_Equip))
        w += weaponRefinementLevel;
    if (SkillSearch("Frenzy", n_A_JOB, n_A_PassSkill))
        w += 30;
    if (SkillSearch("Last Stand", n_A_JOB, n_A_PassSkill))
        w += 20;
    if (SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill))
        w += 2 * SkillSearch("Gatling Fever", n_A_JOB, n_A_PassSkill);

    if (SkillSearch("Stellar Protection", n_A_JOB, n_A_PassSkill)) {
        ASPDch = 1;
        w += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);
    }

    if (SkillSearch("Solar, Lunar, and Stellar Shadow", n_A_JOB, n_A_PassSkill)) {
        ASPDch = 1;
        w += 3 * SkillSearch("Solar, Lunar, and Stellar Shadow", n_A_JOB, n_A_PassSkill);
    }
    if (ASPDch == 0 && n_A_WeaponType != WEAPON_TYPE_BOW && n_A_PassSkill2[6] == 2) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 1) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 3) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_PassSkill3[1] && n_A_WeaponType != WEAPON_TYPE_BOW && ASPDch == 0)
        w += 5 + n_A_PassSkill3[1] + Math.floor(n_A_PassSkill3[31] / 2) + Math.floor(n_A_PassSkill3[21] / 20);


    w += StPlusItem(ASPD_PERCENTAGE, n_A_Equip);
    w += StPlusCard(ASPD_PERCENTAGE, n_A_card);


    if (SkillSearch("Mental Strength", n_A_JOB, n_A_PassSkill))
        w -= 25;


    if (n_A_SpeedPOT || SkillSearch("Deadly Poison (Consumed)", n_A_JOB, n_A_PassSkill)) {
        if (SkillSearch("Deadly Poison (Consumed)", n_A_JOB, n_A_PassSkill) == 0) {
            if (n_A_SpeedPOT == 1)
                w += 10;
            else if (n_A_SpeedPOT == 2)
                w += 15;
            else if (n_A_SpeedPOT == 3)
                w += 20;
        } else
            w += 20;
    }
    n_A_ASPD += (200 - n_A_ASPD) * (w / 100);

    if (n_A_WeaponType == WEAPON_TYPE_BOOK && SkillSearch("Study", n_A_JOB, n_A_PassSkill))
        n_A_ASPD += (200 - n_A_ASPD - (SkillSearch("Study") * 5 / 10)) * ((SkillSearch("Study", n_A_JOB, n_A_PassSkill) * 5 / 10) / 100);


    if (SkillSearch("Defending Aura", n_A_JOB, n_A_PassSkill))
        n_A_ASPD -= (25 - SkillSearch("Defending Aura", n_A_JOB, n_A_PassSkill) * 5);

    if (n_A_ASPD > 190)
        n_A_ASPD = 190;


    n_A_ASPD *= 100;
    n_A_ASPD = Math.round(n_A_ASPD);
    n_A_ASPD /= 100;


    // myInnerHtml("A_ASPD", n_A_ASPD, 0);

    n_A_ASPD = (200 - n_A_ASPD) / 50;
    let n_A_ATK_w, n_A_ATK;
    if (isNonRangeWeapon(n_A_WeaponType)) {
        n_A_ATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        n_A_ATK = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
    } else {
        n_A_ATK_w = Math.round(Math.floor(n_A_DEX / 10) * Math.floor(n_A_DEX / 10));
        n_A_ATK = n_A_DEX + n_A_ATK_w + Math.floor(n_A_STR / 5) + Math.floor(n_A_LUK / 5)
    }
    let impositioMagnus = n_A_PassSkill2[2] * 5;
    let ATK_LEFT = Math.floor(impositioMagnus + n_A_Weapon_ATK + n_A_Weapon2_ATK + n_A_ATK);
    let ATK_RIGHT = Math.floor(n_A_WeaponLV_upgradeBonusATK + n_A_Weapon2LV_upgradeBonusATK);
    // myInnerHtml("A_ATK_2", ATK_LEFT + "+" + ATK_RIGHT, 0),


    let wDelay = 0;
    swDelay = 0;
    if (skillToUseName != "Basic Attack" && skillToUseName != "Martyr's Reconing") {
        wDelay = Math.floor(n_A_ASPD * 100) / 100;
        if (skillToUseName == "Envenom" || skillToUseName == "")
            wDelay = Math.floor(n_A_ASPD * 75) / 100;
        wA_ASPD = eval(FORM_DATA.Conf01) / 100;
        if (wDelay < wA_ASPD)
            wDelay = wA_ASPD;
    }

    if (SkillSearch("Raging Trifecta Blow", n_A_JOB, n_A_PassSkill)) {
        w = 100 / (30 - SkillSearch("Raging Trifecta Blow", n_A_JOB, n_A_PassSkill));
        n_A_ASPD += (n_A_ASPD - (1000 - n_A_AGI * 4 - n_A_DEX * 2) / 1000) / w;
        if (SkillSearch("<Font size=2>Add the delay time when attacking for triple attack</Font>", n_A_JOB, n_A_PassSkill))
            n_A_ASPD += (0.3 / w);
    }


    let n_A_CAST = 1 - n_A_DEX / 150;
    if (n_A_CAST < 0)
        n_A_CAST = 0;


    w = 100;
    if (n_A_JobSearch(n_A_JOB) == 5 && CardNumSearch("0", n_A_card))
        w -= 15;
    if ((n_A_JOB == 18 || n_A_JOB == 32) && CardNumSearch("0", n_A_card))
        w -= 15;
    if (EquipNumSearch("0", n_A_Equip) || EquipNumSearch("0", n_A_Equip))
        w -= weaponRefinementLevel;
    if (n_A_card[8] == 177)
        w -= n_A_HEAD_DEF_PLUS;

    w += StPlusItem(CAST_TIME_PERCENTAGE, n_A_Equip);
    w += StPlusCard(CAST_TIME_PERCENTAGE, n_A_card);

    n_A_CAST *= w / 100;

    if (n_A_PassSkill2[13])
        n_A_CAST *= (100 - 15 * n_A_PassSkill2[13]) / 100;
    if (SkillSearch("Forsight", n_A_JOB, n_A_PassSkill))
        n_A_CAST = n_A_CAST / 2;


    let n_A_HPR = Math.floor(n_A_VIT / 5) + Math.floor(n_A_MaxHP / 200);
    if (n_A_HPR < 1)
        n_A_HPR = 1;
    w = 100;
    w += StPlusItem(HP_REGEN_PERCENTAGE, n_A_Equip);
    w += StPlusCard(HP_REGEN_PERCENTAGE, n_A_card);
    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling", n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Magistrate Hat", n_A_Equip))
        w += 3;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus", n_A_card))
        w += 5;

    n_A_HPR = Math.floor(n_A_HPR * w / 100);
    // myInnerHtml("A_HPR", n_A_HPR, 0);


    let n_A_SPR = Math.floor(n_A_INT / 6) + Math.floor(n_A_MaxSP / 100) + 1;

    w = 100;

    w += SkillSearch("Mediatio", n_A_JOB, n_A_PassSkill) * 3;

    w += StPlusItem(SP_REGEN_PERCENTAGE, n_A_Equip);
    w += StPlusCard(SP_REGEN_PERCENTAGE, n_A_card);

    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling", n_A_card);

    if (n_A_JobSearch(n_A_JOB) == 41 && EquipNumSearch("Ayam", n_A_Equip))
        w += 3;
    if (n_A_LEFT_DEF_PLUS <= 4 && n_A_card[8] == 179)
        w += 5;
    if (n_A_card[9] == 179)
        w += 5;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus", n_A_card))
        w += 5;

    n_A_SPR = Math.floor(n_A_SPR * w / 100);

    if (n_A_INT >= 120)
        n_A_SPR += Math.floor((n_A_INT - 120) / 2) + 4;

    // myInnerHtml("A_SPR", n_A_SPR, 0);

    // KakutyouKansuu();
}


function StPlusItem(nSTP2, n_A_Equip) {
    let wSTP2 = 0;
    for (let STP2i = 0; STP2i <= 20; STP2i++) {
        for (let STP2j = 0; ItemOBJ[n_A_Equip[STP2i]][STP2j + 11] != 0; STP2j += 2) {
            if (nSTP2 == ItemOBJ[n_A_Equip[STP2i]][STP2j + 11])
                wSTP2 += ItemOBJ[n_A_Equip[STP2i]][STP2j + 12];
        }
    }
    return wSTP2;
}


function StPlusCard(nSTP2, n_A_card) {
    let wSTP2 = 0;
    for (let STP2i = 0; STP2i <= 25; STP2i++) {
        for (let STP2j = 0; cardOBJ[n_A_card[STP2i]][STP2j + 4] != 0; STP2j += 2) {
            if (nSTP2 == cardOBJ[n_A_card[STP2i]][STP2j + 4])
                wSTP2 += cardOBJ[n_A_card[STP2i]][STP2j + 5];
        }
    }
    return wSTP2;
}

function CardNumSearch(nCNS, n_A_card) {
    let wCNS = 0;
    for (let CNSi = 0; CNSi <= 25; CNSi++) {
        let cardName = cardOBJ[n_A_card[CNSi]][2];
        if (nCNS === cardName)
            wCNS += 1;
    }
    return wCNS;
}

function EquipNumSearch(nENS, n_A_Equip) {
    let wENS = 0;
    for (let ENSi = 0; ENSi <= 20; ENSi++) {
        let itemName = ItemOBJ[n_A_Equip[ENSi]][8];
        if (nENS === itemName)
            wENS += 1;
    }
    return wENS;
}


function n_A_JobSet(FORM_DATA) {
    let n_A_JOB = eval(FORM_DATA.A_JOB);
    let isRebirth = 0;
    if (21 <= n_A_JOB && n_A_JOB <= 40) {
        let isRebirth = 1;
        if (34 <= n_A_JOB && n_A_JOB <= 40)
            n_A_JOB -= 34;
    }
    return {n_A_JOB, isRebirth}
}

function SetEquip(n_A_Equip) {
    const start = Date.now();

    for (let SEi = 11; SEi <= 20; SEi++)
        n_A_Equip[SEi] = 736;

    let w_SE_num = 11;
    let w_SE_ch = 0;
    for (let SEk = 0; SEk <= SE_MAXnum; SEk++) {
        for (let SEj = 1; w_SE[SEk][SEj] != "NULL" && (w_SE_ch == 1 || (w_SE_ch == 0 && SEj == 1)); SEj++) {
            w_SE_ch = 0;
            for (let SEi = 0; SEi <= 10 && w_SE_ch == 0; SEi++) {
                if (n_A_Equip[SEi] == w_SE[SEk][SEj])
                    w_SE_ch = 1;
            }
        }
        if (w_SE_ch == 1) {
            n_A_Equip[w_SE_num] = w_SE[SEk][0];
            w_SE_num++;
        }
    }
}

function SetCard(n_A_card) {
    for (let SEi = 16; SEi <= 25; SEi++)
        n_A_card[SEi] = 0;

    let w_SE_num = 16;
    let w_SE_ch = 0;
    for (let SEk = 0; SEk <= SC_MAXnum; SEk++) {
        for (let SEj = 1; w_SC[SEk][SEj] != "NULL" && (w_SE_ch == 1 || (w_SE_ch == 0 && SEj == 1)); SEj++) {
            w_SE_ch = 0;
            for (let SEi = 0; SEi <= 15 && w_SE_ch == 0; SEi++) {
                if (n_A_card[SEi] == w_SC[SEk][SEj])
                    w_SE_ch = 1;
            }
        }
        if (w_SE_ch == 1) {
            n_A_card[w_SE_num] = w_SC[SEk][0];
            w_SE_num++;
        }
    }
}

function SkillSearch(n, n_A_JOB, n_A_PassSkill) {
    for (let k = 0; k <= 14; k++) {
        let passiveSkillToUseName;
        if (JobSkillPassOBJ[n_A_JOB][k] != 999)
            passiveSkillToUseName = SkillOBJ[JobSkillPassOBJ[n_A_JOB][k]][2];
        if (passiveSkillToUseName === n) {
            return n_A_PassSkill[k];
        }
    }
    return 0;
}

// TODO rename
function n_A_JobSearch(n_A_JOB) {

    if (n_A_JOB <= 6)
        return n_A_JOB;
    if (n_A_JOB == 20)
        return 0;
    if (n_A_JOB == 7 || n_A_JOB == 13 || n_A_JOB == 21 || n_A_JOB == 27)
        return 1;
    if (n_A_JOB == 8 || n_A_JOB == 14 || n_A_JOB == 22 || n_A_JOB == 28)
        return 2;
    if (n_A_JOB == 9 || n_A_JOB == 15 || n_A_JOB == 23 || n_A_JOB == 29)
        return 3;
    if (n_A_JOB == 10 || n_A_JOB == 16 || n_A_JOB == 17 || n_A_JOB == 24 || n_A_JOB == 30 || n_A_JOB == 31)
        return 4;
    if (n_A_JOB == 11 || n_A_JOB == 18 || n_A_JOB == 25 || n_A_JOB == 32)
        return 5;
    if (n_A_JOB == 12 || n_A_JOB == 19 || n_A_JOB == 26 || n_A_JOB == 33)
        return 6;
    if (n_A_JOB == 41 || n_A_JOB == 42 || n_A_JOB == 43)
        return 41;
    return 7;
}


function isNonRangeWeapon(n_A_WeaponType) {
    return n_A_WeaponType != WEAPON_TYPE_BOW && n_A_WeaponType != WEAPON_TYPE_INSTRUMENT && n_A_WeaponType != WEAPON_TYPE_WHIP && n_A_WeaponType != WEAPON_TYPE_HANDGUN && n_A_WeaponType != WEAPON_TYPE_RIFLE && n_A_WeaponType != WEAPON_TYPE_SHOTGUN && n_A_WeaponType != WEAPON_TYPE_GATLING_GUN && n_A_WeaponType != WEAPON_TYPE_GRENADE_LAUNCHER;
}


function CalculateEnemyStats(FORM_DATA, InWarOfEmperium, n_A_card) {
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
        atk1: mob[12],
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
        def2Min:0,
        def2Avg:0,
        def2Max:0,
    };

    let def2Min, def2Max , def2Avg , mdef2 , hit , flee;
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
        targetStatsArray[TARGET_STAT_HP] = Math.floor(w * (100 + targetStats.vit) / 100);
        targetStatsArray[TARGET_STAT_HP] += eval(document.calcForm.B_TAISEI11.value);
        targetStatsArray[TARGET_STAT_HP] = Math.floor(targetStatsArray[TARGET_STAT_HP] * (100 + eval(document.calcForm.B_TAISEI12.value)) / 100);
        myInnerHtml("B_HP", targetStatsArray[TARGET_STAT_HP], 0);


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
        provokeLevel: 0,
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
            provokeLevel: eval(FORM_DATA.TargetStatusFlag0),
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
        targetStats.element = targetStatusFlag.sageChangeElement  * 10 + (targetStats.element % 10);

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


    if (InWarOfEmperium == 0) {
        let w1_Exp = StPlusCard(120 + targetStats.race, n_A_card);
        w1_Exp += StPlusItem(120 + targetStats.race, n_A_card);
        if (n_A_JobSearch() == 3 && CardNumSearch(452) && (targetStats.race == 1 || targetStats.race == 6))
            w1_Exp += 5;
        if (targetStats.race == 2 && n_A_JobSearch() == 4 && CardNumSearch(453))
            w1_Exp += 5;
        if (w1_Exp != 0) {
            targetStats.exp = Math.floor(targetStats.exp * (100 + w1_Exp) / 100);
            targetStats.jobExp = Math.floor(targetStats.jobExp * (100 + w1_Exp) / 100);
        }
        if (targetStats.isNormal) {
            let mrKingRichManSong = null;
            if (n_Skill3SW)
                mrKingRichManSong = eval(FORM_DATA.A_PERFORMANCE_SKILL8);
            if (mrKingRichManSong) {
                targetStats.exp = Math.floor(targetStats.exp * (125 + 11 * mrKingRichManSong) / 100);
                targetStats.jobExp = Math.floor(targetStats.jobExp * (125 + 11 * mrKingRichManSong) / 100);
            }
        }
    }

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
        // myInnerHtml("B_4", SizeOBJ[targetStatsArray[TARGET_STAT_SIZE]], 0);
    } else {
        n_B_FLEE += eval(document.calcForm.B_TAISEI7.value);
        n_Ses = document.calcForm.B_Ses.checked;
        if (n_Ses) {
            n_B_FLEE = Math.floor(n_B_FLEE * 0.8);
        }
    }
    //
    // n_B_DEF2 = [0, 0, 0];
    // n_B_DEF2[2] = targetStats.def2Min;
    // n_B_DEF2[0] = targetStats.def2Max;
    // n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2);
    // n_B_MDEF2 = targetStats.mdef2;
    // n_B_HIT = targetStats.hit;
    // n_B_FLEE = targetStats.flee;
    return {
        ...targetStats,
        ...targetStatusFlag

    }
}


export {
    CalculateAllStats,
}