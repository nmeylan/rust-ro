
function StAllCalc() {
    n_A_JobSet();

    if (n_A_JOB == 20) {
        if (SuperNoviceFullWeaponCHECK == 0 && eval(document.calcForm.A_skill9.value) == 1)
            SuperNoviceFullWeapon(1);
        else if (SuperNoviceFullWeaponCHECK == 1 && eval(document.calcForm.A_skill9.value) == 0)
            SuperNoviceFullWeapon(0);
    }
    n_A_BaseLV = eval(document.calcForm.A_BaseLV.value);
    n_A_JobLV = eval(document.calcForm.A_JobLV.value);

    n_A_STR = eval(document.calcForm.A_STR.value);
    n_A_AGI = eval(document.calcForm.A_AGI.value);
    n_A_VIT = eval(document.calcForm.A_VIT.value);
    n_A_DEX = eval(document.calcForm.A_DEX.value);
    n_A_INT = eval(document.calcForm.A_INT.value);
    n_A_LUK = eval(document.calcForm.A_LUK.value);
    SU_STR = n_A_STR;
    SU_AGI = n_A_AGI;
    SU_VIT = n_A_VIT;
    SU_DEX = n_A_DEX;
    SU_INT = n_A_INT;
    SU_LUK = n_A_LUK;

    n_A_WeaponType = eval(document.calcForm.A_WeaponType.value);

    n_A_Arrow = eval(document.calcForm.A_Arrow.value);
    n_A_Weapon1 = eval(document.calcForm.A_weapon1.value);

    n_A_WeaponLV = ItemOBJ[n_A_Weapon1][4];
    n_A_Weapon_ATK = ItemOBJ[n_A_Weapon1][3];

    n_A_Weapon2LV_upgradeBonusATK = 0;
    n_A_Weapon2LV_Minplus = 0;
    n_A_Weapon2LV_overUpgradeBonusATK = 0;
    n_A_Weapon2LV = 0;
    n_A_Weapon2_ATK = 0;
    n_A_Weapon2_RefinementLevel = 0;
    if (hasLeftHand) {

        if (targetStatsArray[19] != 5) {

            n_A_Weapon2 = eval(document.calcForm.A_weapon2.value);
            n_A_Weapon2LV = ItemOBJ[n_A_Weapon2][4];
            n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
            n_A_Weapon2_RefinementLevel = eval(document.calcForm.A_Weapon2_ATKplus.value);


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

    if (document.calcForm.A_weapon2) {
        n_A_Weapon2 = eval(document.calcForm.A_weapon2.value);
        n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
    } else {
        n_A_Weapon2_ATK = 0;
    }
    weaponRefinementLevel = eval(document.calcForm.A_Weapon_ATKplus.value);

    n_A_WeaponLV_upgradeBonusATK = 0;
    n_A_WeaponLV_Minplus = 0;
    n_A_WeaponLV_overUpgradeBonusATK = 0;
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
    n_A_HEAD_DEF_PLUS = eval(document.calcForm.A_HEAD_DEF_PLUS.value);
    n_A_BODY_DEF_PLUS = eval(document.calcForm.A_BODY_DEF_PLUS.value);
    n_A_LEFT_DEF_PLUS = eval(document.calcForm.A_LEFT_DEF_PLUS.value);
    n_A_SHOULDER_DEF_PLUS = eval(document.calcForm.A_SHOULDER_DEF_PLUS.value);
    n_A_SHOES_DEF_PLUS = eval(document.calcForm.A_SHOES_DEF_PLUS.value);
    n_A_DEFplus = n_A_HEAD_DEF_PLUS + n_A_BODY_DEF_PLUS + n_A_LEFT_DEF_PLUS + n_A_SHOULDER_DEF_PLUS + n_A_SHOES_DEF_PLUS;

    n_A_ActiveSkill = eval(document.calcForm.A_ActiveSkill.value);
    if (n_A_ActiveSkill > 100000)
        n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) / 100);

    n_A_ActiveSkillLV = eval(document.calcForm.A_ActiveSkillLV.value);
    n_A_SpeedPOT = eval(document.calcForm.A_SpeedPOT.value);

    n_A_Equip[0] = eval(document.calcForm.A_weapon1.value);
    if (hasLeftHand)
        n_A_Equip[1] = eval(document.calcForm.A_weapon2.value);
    else
        n_A_Equip[1] = 0;
    n_A_Equip[2] = eval(document.calcForm.A_head1.value);
    n_A_Equip[3] = eval(document.calcForm.A_head2.value);
    n_A_Equip[4] = eval(document.calcForm.A_head3.value);
    n_A_Equip[5] = eval(document.calcForm.A_left.value);
    n_A_Equip[6] = eval(document.calcForm.A_body.value);
    n_A_Equip[7] = eval(document.calcForm.A_shoulder.value);
    n_A_Equip[8] = eval(document.calcForm.A_shoes.value);
    n_A_Equip[9] = eval(document.calcForm.A_acces1.value);
    n_A_Equip[10] = eval(document.calcForm.A_acces2.value);

    SetEquip();

    n_A_card[0] = eval(document.calcForm.A_weapon1_card1.value);
    n_A_card[1] = eval(document.calcForm.A_weapon1_card2.value);
    n_A_card[2] = eval(document.calcForm.A_weapon1_card3.value);
    n_A_card[3] = eval(document.calcForm.A_weapon1_card4.value);
    if (hasLeftHand) {
        n_A_card[4] = eval(document.calcForm.A_weapon2_card1.value);
        n_A_card[5] = eval(document.calcForm.A_weapon2_card2.value);
        n_A_card[6] = eval(document.calcForm.A_weapon2_card3.value);
        n_A_card[7] = eval(document.calcForm.A_weapon2_card4.value);
    } else {
        n_A_card[4] = 0;
        n_A_card[5] = 0;
        n_A_card[6] = 0;
        n_A_card[7] = 0;
    }
    n_A_card[8] = eval(document.calcForm.A_head1_card.value);
    n_A_card[9] = eval(document.calcForm.A_head2_card.value);
    n_A_card[10] = eval(document.calcForm.A_left_card.value);
    n_A_card[11] = eval(document.calcForm.A_body_card.value);
    n_A_card[12] = eval(document.calcForm.A_shoulder_card.value);
    n_A_card[13] = eval(document.calcForm.A_shoes_card.value);
    n_A_card[14] = eval(document.calcForm.A_acces1_card.value);
    n_A_card[15] = eval(document.calcForm.A_acces2_card.value);

    SetCard();

    n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
    n_A_Weapon2_element = n_A_Weapon_element;


    if (n_A_Weapon_element == 0) {
        for (j = 0; ItemOBJ[n_A_Equip[0]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[n_A_Equip[0]][j + 11])
                n_A_Weapon_element = ItemOBJ[n_A_Equip[0]][j + 12];
        }
        for (j = 0; ItemOBJ[n_A_Equip[1]][j + 11] != 0; j += 2) {
            if (20 == ItemOBJ[n_A_Equip[1]][j + 11])
                n_A_Weapon2_element = ItemOBJ[n_A_Equip[1]][j + 12];
        }

        if (201 <= cardOBJ[n_A_card[0]][0] && cardOBJ[n_A_card[0]][0] <= 204)
            n_A_Weapon_element = cardOBJ[n_A_card[0]][0] - 200;
        if (201 <= cardOBJ[n_A_card[4]][0] && cardOBJ[n_A_card[4]][0] <= 204)
            n_A_Weapon2_element = cardOBJ[n_A_card[4]][0] - 200;

        if (n_A_WeaponType ==  WEAPON_TYPE_BOW || n_A_WeaponType ==  WEAPON_TYPE_HANDGUN || n_A_WeaponType ==  WEAPON_TYPE_RIFLE || n_A_WeaponType ==  WEAPON_TYPE_SHOTGUN || n_A_WeaponType ==  WEAPON_TYPE_GATLING_GUN || n_A_WeaponType ==  WEAPON_TYPE_GRENADE_LAUNCHER) {
            n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
        }
    }

    n_A_PassSkill = new Array();


    if (JobSkillPassOBJ[n_A_JOB][0] != 999) n_A_PassSkill[0] = eval(document.calcForm.A_skill0.value);
    if (JobSkillPassOBJ[n_A_JOB][1] != 999) n_A_PassSkill[1] = eval(document.calcForm.A_skill1.value);
    if (JobSkillPassOBJ[n_A_JOB][2] != 999) n_A_PassSkill[2] = eval(document.calcForm.A_skill2.value);
    if (JobSkillPassOBJ[n_A_JOB][3] != 999) n_A_PassSkill[3] = eval(document.calcForm.A_skill3.value);
    if (JobSkillPassOBJ[n_A_JOB][4] != 999) n_A_PassSkill[4] = eval(document.calcForm.A_skill4.value);
    if (JobSkillPassOBJ[n_A_JOB][5] != 999) n_A_PassSkill[5] = eval(document.calcForm.A_skill5.value);
    if (JobSkillPassOBJ[n_A_JOB][6] != 999) n_A_PassSkill[6] = eval(document.calcForm.A_skill6.value);
    if (JobSkillPassOBJ[n_A_JOB][7] != 999) n_A_PassSkill[7] = eval(document.calcForm.A_skill7.value);
    if (JobSkillPassOBJ[n_A_JOB][8] != 999) n_A_PassSkill[8] = eval(document.calcForm.A_skill8.value);
    if (JobSkillPassOBJ[n_A_JOB][9] != 999) n_A_PassSkill[9] = eval(document.calcForm.A_skill9.value);
    if (JobSkillPassOBJ[n_A_JOB][10] != 999) n_A_PassSkill[10] = eval(document.calcForm.A_skill10.value);
    if (JobSkillPassOBJ[n_A_JOB][11] != 999) n_A_PassSkill[11] = eval(document.calcForm.A_skill11.value);
    if (JobSkillPassOBJ[n_A_JOB][12] != 999) n_A_PassSkill[12] = eval(document.calcForm.A_skill12.value);
    if (JobSkillPassOBJ[n_A_JOB][13] != 999) n_A_PassSkill[13] = eval(document.calcForm.A_skill13.value);
    if (JobSkillPassOBJ[n_A_JOB][14] != 999) n_A_PassSkill[14] = eval(document.calcForm.A_skill14.value);


    if (n_SkillSW) {
        n_A_PassSkill2[0] = eval(document.calcForm.A2_Skill0.value);
        n_A_PassSkill2[1] = eval(document.calcForm.A2_Skill1.value);
        n_A_PassSkill2[2] = eval(document.calcForm.A2_Skill2.value);
        n_A_PassSkill2[3] = eval(document.calcForm.A2_Skill3.checked);
        n_A_PassSkill2[4] = eval(document.calcForm.A2_Skill4.value);
        n_A_PassSkill2[5] = eval(document.calcForm.A2_Skill5.checked);
        n_A_PassSkill2[6] = eval(document.calcForm.A2_Skill6.value);
        n_A_PassSkill2[7] = eval(document.calcForm.A2_Skill7.checked);
        n_A_PassSkill2[8] = eval(document.calcForm.A2_Skill8.value);
        n_A_PassSkill2[9] = eval(document.calcForm.A2_Skill9.value);
        n_A_PassSkill2[10] = eval(document.calcForm.A2_Skill10.value);
        n_A_PassSkill2[11] = eval(document.calcForm.A2_Skill11.checked);
        n_A_PassSkill2[12] = eval(document.calcForm.A2_Skill12.checked);
        n_A_PassSkill2[13] = eval(document.calcForm.A2_Skill13.value);
        n_A_PassSkill2[14] = eval(document.calcForm.A2_Skill14.value);
    }

    if (n_Skill3SW) {
        n_A_PassSkill3[0] = eval(document.calcForm.A3_Skill0_1.value);
        n_A_PassSkill3[1] = eval(document.calcForm.A3_Skill1_1.value);
        n_A_PassSkill3[2] = eval(document.calcForm.A3_Skill2_1.value);
        n_A_PassSkill3[3] = eval(document.calcForm.A3_Skill3_1.value);

        n_A_PassSkill3[5] = eval(document.calcForm.A3_Skill5_1.value);

        n_A_PassSkill3[7] = eval(document.calcForm.A3_Skill7.value);

        n_A_PassSkill3[9] = eval(document.calcForm.A3_Skill9.value);
        n_A_PassSkill3[10] = eval(document.calcForm.A3_Skill10.value);
        n_A_PassSkill3[11] = eval(document.calcForm.A3_Skill11.checked);
        if (n_A_PassSkill3[11]) {
            n_A_PassSkill3[12] = eval(document.calcForm.A3_Skill11_STR.value);
            n_A_PassSkill3[13] = eval(document.calcForm.A3_Skill11_AGI.value);
            n_A_PassSkill3[14] = eval(document.calcForm.A3_Skill11_VIT.value);
            n_A_PassSkill3[15] = eval(document.calcForm.A3_Skill11_INT.value);
            n_A_PassSkill3[16] = eval(document.calcForm.A3_Skill11_DEX.value);
            n_A_PassSkill3[17] = eval(document.calcForm.A3_Skill11_LUK.value);
        }

        if (n_A_PassSkill3[0]) {
            n_A_PassSkill3[20] = eval(document.calcForm.A3_Skill0_2.value);
            n_A_PassSkill3[30] = eval(document.calcForm.A3_Skill0_3.value);
        }
        if (n_A_PassSkill3[1]) {
            n_A_PassSkill3[21] = eval(document.calcForm.A3_Skill1_2.value);
            n_A_PassSkill3[31] = eval(document.calcForm.A3_Skill1_3.value);
        }
        if (n_A_PassSkill3[2]) {
            n_A_PassSkill3[22] = eval(document.calcForm.A3_Skill2_2.value);
            n_A_PassSkill3[29] = eval(document.calcForm.A3_Skill2_3.value);
            n_A_PassSkill3[32] = eval(document.calcForm.A3_Skill2_4.value);
        }
        if (n_A_PassSkill3[3]) {
            n_A_PassSkill3[23] = eval(document.calcForm.A3_Skill3_2.value);
            n_A_PassSkill3[33] = eval(document.calcForm.A3_Skill3_3.value);
        }
        if (n_A_PassSkill3[4]) {
            n_A_PassSkill3[24] = eval(document.calcForm.A3_Skill4_2.value);
            n_A_PassSkill3[34] = eval(document.calcForm.A3_Skill4_3.value);
        }
        if (n_A_PassSkill3[5]) {
            n_A_PassSkill3[25] = eval(document.calcForm.A3_Skill5_2.value);
            n_A_PassSkill3[35] = eval(document.calcForm.A3_Skill5_3.value);
        }
        if (n_A_PassSkill3[6]) {
            n_A_PassSkill3[26] = eval(document.calcForm.A3_Skill6_2.value);
            n_A_PassSkill3[36] = eval(document.calcForm.A3_Skill6_3.value);
        }

    }
    if (n_Skill4SW) {
        n_A_PassSkill3[40] = eval(document.calcForm.A3_Skill40.checked);
        n_A_PassSkill3[41] = eval(document.calcForm.A3_Skill41.value);
        n_A_PassSkill3[42] = eval(document.calcForm.A3_Skill42.value);
        n_A_PassSkill3[43] = eval(document.calcForm.A3_Skill43.value);
        n_A_PassSkill3[44] = eval(document.calcForm.A3_Skill44.value);
    }
    if (n_Skill5SW) {
        n_A_PassSkill5[0] = eval(document.calcForm.A5_Skill0.checked);
        n_A_PassSkill5[1] = eval(document.calcForm.A5_Skill1.checked);
        n_A_PassSkill5[2] = eval(document.calcForm.A5_Skill2.checked);
        n_A_PassSkill5[3] = eval(document.calcForm.A5_Skill3.checked);
        n_A_PassSkill5[4] = eval(document.calcForm.A5_Skill4.checked);
    }
    if (n_Skill6SW) {
        n_A_PassSkill6[0] = eval(document.calcForm.A6_Skill0.value);
        n_A_PassSkill6[1] = eval(document.calcForm.A6_Skill1.value);
        n_A_PassSkill6[2] = eval(document.calcForm.A6_Skill2.value);
        n_A_PassSkill6[3] = eval(document.calcForm.A6_Skill3.checked);
    }
    if (n_Skill7SW) {
        n_A_PassSkill7[0] = eval(document.calcForm.A7_Skill0.checked);
        n_A_PassSkill7[1] = eval(document.calcForm.A7_Skill1.checked);
        n_A_PassSkill7[2] = eval(document.calcForm.A7_Skill2.checked);
        n_A_PassSkill7[3] = eval(document.calcForm.A7_Skill3.value);
        n_A_PassSkill7[4] = eval(document.calcForm.A7_Skill4.value);
        n_A_PassSkill7[5] = eval(document.calcForm.A7_Skill5.value);
        n_A_PassSkill7[6] = eval(document.calcForm.A7_Skill6.value);
        n_A_PassSkill7[7] = eval(document.calcForm.A7_Skill7.value);
        n_A_PassSkill7[8] = eval(document.calcForm.A7_Skill8.value);
        n_A_PassSkill7[9] = eval(document.calcForm.A7_Skill9.checked);
        n_A_PassSkill7[10] = eval(document.calcForm.A7_Skill10.checked);
    }

    ClickB_Enemy();


    StPlusCalc();


    if (isNonRangeWeapon()) {
        baseATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        baseATK = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
    } else {
        baseATK_w = Math.round(Math.floor(n_A_DEX / 10) * Math.floor(n_A_DEX / 10));
        baseATK = n_A_DEX + baseATK_w + Math.floor(n_A_STR / 5) + Math.floor(n_A_LUK / 5);
    }


    w = StPlusCard(ATK);
    w += StPlusItem(ATK);

    if (SU_STR >= 80 && CardNumSearch("Giant Whisper"))
        w += 20;
    if (SU_STR >= 95 && EquipNumSearch("Doom Slayer"))
        w += 340;
    if (SU_STR >= 44 && EquipNumSearch("Holgren's Refining Hammer"))
        w += 44;
    if (EquipNumSearch("Mythical Lion Mask"))
        w += n_A_HEAD_DEF_PLUS * 2;

    if (n_A_PassSkill6[0] == 0 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Pasana") || n_A_Equip[6] == 428 || n_A_Equip[6] == 604))
        w += n_A_PassSkill6[1] * 10;

    if (n_A_PassSkill7[2])
        w += 10;
    if (n_A_PassSkill7[9])
        w += 20;

    if (SkillSearch("Last Stand"))
        w += 100;
    if (SkillSearch("Gatling Fever"))
        w += 20 + 10 * SkillSearch("Gatling Fever");

    if (n_A_PassSkill3[9])
        w += 25 + 25 * n_A_PassSkill3[9];


    baseATK += w;


    JobHP_A = new Array(0, 70, 50, 40, 50, 30, 40, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 150, 110, 75, 85, 55, 90, 110, 85, 90, 75, 75, 75, 90, 0, 0, 0, 0, 0, 0, 0, 70, 90, 75, 75, 84);
    JobHP_B = new Array(5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 0, 0, 0, 0, 0, 0, 0, 5, 6.5, 5, 3, 3.5);


    wHPSL = 0;
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
    for (i = 2; i <= n_A_BaseLV; i++) {
        w += Math.round(JobHP_A[n_A_JOB] * i / 100);
    }

    n_A_MaxHP = Math.floor((JobHP_B[n_A_JOB] * n_A_BaseLV) + 35 + w);


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
    if (eval(document.calcForm.isAdopted.checked))
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

    if (SkillSearch("Taekwon Ranker") && n_A_BaseLV >= 90)
        n_A_MaxHP *= 3;


    n_A_MaxHP += SkillSearch("Faith") * 200;
    bkHP = n_A_MaxHP;
    w = 0;

    w += StPlusItem(MAXHP);
    w += StPlusItem(VIT);


    w += StPlusCard(MAXHP);
    if (n_A_BODY_DEF_PLUS >= 9 && CardNumSearch("Apocalypse"))
        w += 800;

    //Temporary remover card code.
    if (CardNumSearch("Remover"))
        w -= n_A_BODY_DEF_PLUS * 40;

    if (n_A_Equip[8] == 536) {
        wHPVS = n_A_JobSearch();
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5)
            w += 5 * n_A_BaseLV;
    }

    n_A_MaxHP += w;

    w = 0;

    w += StPlusItem(MAXHP_PERCENTAGE);

    w += StPlusCard(MAXHP_PERCENTAGE);

    if (SU_VIT >= 80 && CardNumSearch("Giant Whisper"))
        w += 3;

    if (CardNumSearch("Aliot")) {
        if (n_A_JobSearch() == 1 || n_A_JobSearch() == 2 || n_A_JobSearch() == 6)
            w += 5;
    }
    if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch("Firelock Soldier"))
        w += 10;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus"))
        w += 4;
    if (n_A_PassSkill5[1])
        w += 100;
    if (EquipNumSearch("Variant Shoes"))
        w -= n_A_SHOES_DEF_PLUS;

    n_A_MaxHP = n_A_MaxHP * (100 + w) / 100;

    if (n_A_PassSkill6[0] == 1 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Swordfish") || n_A_Equip[6] == 429 || n_A_Equip[6] == 605)) {
        dHP = [5, 9, 12, 14, 15];
        n_A_MaxHP = n_A_MaxHP * (100 + dHP[n_A_PassSkill6[1] - 1]) / 100;
    }
    if (SkillSearch("Frenzy"))
        n_A_MaxHP *= 3;


    if (n_A_PassSkill3[3])
        n_A_MaxHP += (Math.floor(bkHP * (105 + n_A_PassSkill3[3] * 2 + n_A_PassSkill3[33] + Math.floor(n_A_PassSkill3[23] / 10)) / 100) - bkHP);


    n_A_MaxHP = Math.floor(n_A_MaxHP);


    if (n_A_MaxHP >= 100) {
        if (n_A_MaxHP >= 10000)
            myInnerHtml("A_MaxHP", " " + n_A_MaxHP, 0);
        else
            myInnerHtml("A_MaxHP", n_A_MaxHP, 0);
    } else
        myInnerHtml("A_MaxHP", " " + n_A_MaxHP, 0);


    JobSP_A = new Array(1, 2, 2, 5, 2, 6, 3, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 1, 3, 4, 8, 4, 9, 4, 4.7, 5, 4.7, 6, 6, 7, 4, 0, 0, 0, 0, 0, 0, 0, 2, 4.7, 9, 3.75, 3.75);


    wSPSL = 0;
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

    n_A_MaxSP = Math.floor(10 + n_A_BaseLV * JobSP_A[n_A_JOB] - wSPSL);

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
    if (eval(document.calcForm.isAdopted.checked))
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

    bkSP = n_A_MaxSP;

    if (SkillSearch("Taekwon Ranker") && n_A_BaseLV >= 90)
        n_A_MaxSP *= 3;

    w = 0;

    w += StPlusItem(MAXSP);
    w += StPlusItem(INT);

    w += StPlusCard(MAXSP);
    if (n_A_HEAD_DEF_PLUS >= 9 && n_A_card[8] == 298)
        w += 150;
    if (n_A_HEAD_DEF_PLUS <= 4 && n_A_card[8] == 179)
        w += 40;
    if (n_A_card[9] == 179)
        w += 40;

    if (SkillSearch("Kaina"))
        w += 30 * SkillSearch("Kaina");

    if (n_A_Equip[8] == 536) {
        wSPVS = n_A_JobSearch();
        if (wSPVS == 1 || wSPVS == 2 || wSPVS == 6)
            w += 2 * n_A_JobLV;
    }
    if (weaponRefinementLevel >= 9 && EquipNumSearch("Lich's Bone Wand"))
        w += 300;


    n_A_MaxSP += w;

    w = 0;

    w += StPlusItem(MAXSP_PERCENTAGE);

    w += StPlusCard(MAXSP_PERCENTAGE);
    if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch("Firelock Soldier"))
        w += 10;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus"))
        w += 4;

    if (CardNumSearch("Aliot")) {
        if (n_A_JobSearch() == 3 || n_A_JobSearch() == 4 || n_A_JobSearch() == 5)
            w += 5;
    }


    w += SkillSearch("Mediatio");

    w += SkillSearch("Soul Drain") * 2;
    if (n_A_PassSkill5[2])
        w += 100;
    if (EquipNumSearch("Variant Shoes"))
        w -= n_A_SHOES_DEF_PLUS;

    n_A_MaxSP = Math.floor(n_A_MaxSP * (100 + w) / 100);

    if (n_A_PassSkill3[6])
        n_A_MaxSP += (Math.floor(bkSP * (100 + n_A_PassSkill3[6] * 2 + n_A_PassSkill3[36] + Math.floor(n_A_PassSkill3[26] / 10)) / 100) - bkSP);


    if (n_A_MaxSP >= 100)
        myInnerHtml("A_MaxSP", n_A_MaxSP, 0);
    else
        myInnerHtml("A_MaxSP", " " + n_A_MaxSP, 0);


    n_A_DEF = StPlusItem(DEF);

    for (i = 2; i <= 10; i++) {
        n_A_DEF += ItemOBJ[n_A_Equip[i]][3];
    }

    n_A_DEF += StPlusCard(DEF);

    if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch("Arcluse"))
        n_A_DEF += 2;
    if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch("Goat"))
        n_A_DEF += 2;
    if (n_A_Equip[0] == 521) {
        if (weaponRefinementLevel <= 5)
            n_A_DEF += 2;
        else if (weaponRefinementLevel >= 9)
            n_A_DEF += 5;
        else
            n_A_DEF += 3;
    }
    if (EquipNumSearch("Gatekeeper-DD"))
        n_A_DEF += weaponRefinementLevel;
    if (EquipNumSearch("Variant Shoes"))
        n_A_DEF += n_A_SHOES_DEF_PLUS;
    if (EquipNumSearch("0") && n_A_JobSearch() == 1)
        n_A_DEF += 6;

    if (EquipNumSearch("0"))
        n_A_DEFplus -= (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

    n_A_totalDEF = n_A_DEF + Math.round(n_A_DEFplus * 7 / 10);

    if (StPlusItem(REDUCE_DEFENSE) + StPlusCard(REDUCE_DEFENSE))
        n_A_totalDEF = Math.floor(n_A_totalDEF / StPlusItem(REDUCE_DEFENSE));
    if (StPlusItem(LOWER_DEFENCE_PERCENTAGE) + StPlusCard(LOWER_DEFENCE_PERCENTAGE))
        n_A_totalDEF -= Math.floor(n_A_totalDEF * (StPlusItem(LOWER_DEFENCE_PERCENTAGE) + StPlusCard(LOWER_DEFENCE_PERCENTAGE)) / 100);

    if (SkillSearch("Spear Dynamo"))
        n_A_totalDEF = Math.floor(n_A_totalDEF * (1 - 0.05 * SkillSearch("Spear Dynamo")));


    if (SkillSearch("Mental Strength"))
        n_A_totalDEF = 90;

    if (SkillSearch("Frenzy"))
        n_A_totalDEF = 0;


    myInnerHtml("A_totalDEF", n_A_totalDEF, 0);


    n_A_VITDEF = new Array();
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
    if (SkillSearch("Auto Berserk")) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.45);
    } else {
        if (n_A_PassSkill2[12]) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.9);
        }
    }
    if (StPlusItem(REDUCE_DEFENSE)) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] / StPlusItem(24));
    }
    if (SkillSearch("Spear Dynamo")) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 - 0.05 * SkillSearch("Spear Dynamo")));
    }
    if (n_A_PassSkill2[4]) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 + 0.05 * n_A_PassSkill2[4]));
    }
    if (SkillSearch("Frenzy")) {
        for (i = 0; i <= 2; i++)
            n_A_VITDEF[i] = 0;
    }


    n_A_MDEF = StPlusItem(MDEF);


    n_A_MDEF += StPlusCard(MDEF);

    if (n_A_JobSearch() == 3)
        n_A_MDEF += CardNumSearch("Rideword");
    if (n_A_LEFT_DEF_PLUS >= 9 && CardNumSearch("Sting"))
        n_A_MDEF += 5;
    if (n_A_HEAD_DEF_PLUS <= 5 && n_A_card[8] == 213)
        n_A_MDEF += 5;
    if (n_A_card[9] == 213)
        n_A_MDEF += 5;
    if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch("Arcluse"))
        n_A_MDEF += 3;
    if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch("Goat"))
        n_A_MDEF += 5;
    if (n_A_SHOES_DEF_PLUS <= 5 && CardNumSearch("Megalith"))
        n_A_MDEF += 7;
    if (n_A_SHOULDER_DEF_PLUS <= 5 && CardNumSearch("Kappa"))
        n_A_MDEF += 8;
    if (EquipNumSearch("0"))
        n_A_MDEF += (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

    if (SkillSearch("Spear Dynamo"))
        n_A_MDEF += 1;
    if (SkillSearch("Endure"))
        n_A_MDEF += SkillSearch("Endure");


    if (SkillSearch("Mental Strength"))
        n_A_MDEF = 90;
    if (SkillSearch("Frenzy"))
        n_A_MDEF = 0;

    myInnerHtml("A_MDEF", n_A_MDEF, 0);


    n_A_HIT = n_A_BaseLV + n_A_DEX;


    n_A_HIT += StPlusItem(HIT);


    n_A_HIT += StPlusCard(HIT);

    if (EquipNumSearch("Jungle Carbine"))
        w -= Math.floor(SU_DEX / 3);


    n_A_HIT += 1 * SkillSearch("Vulture's Eye");
    n_A_HIT += 2 * SkillSearch("Weaponry Research");
    n_A_HIT += 3 * SkillSearch("True Sight");

    n_A_HIT += 10 * SkillSearch("Spear Dynamo");
    n_A_HIT += 1 * SkillSearch("Snake Eyes");
    if (SkillSearch("Gunslinger's Panic"))
        n_A_HIT -= 30;
    if (SkillSearch("Increase Accuracy"))
        n_A_HIT += 20;
    n_A_HIT += 2 * SkillSearch("Single Action");

    if (EquipNumSearch("Western Outlaw"))
        n_A_HIT += Math.floor(SU_AGI / 5);

    if (skillToUseName == "Rapid Smiting")
        n_A_HIT += 20;

    if (n_A_PassSkill5[4])
        n_A_HIT += 50;

    if (n_A_PassSkill7[0])
        n_A_HIT += 30;


    if (n_A_PassSkill3[4])
        n_A_HIT += n_A_PassSkill3[4] + Math.floor(n_A_PassSkill3[34] / 2) + Math.floor(n_A_PassSkill3[24] / 10);


    myInnerHtml("A_HIT", n_A_HIT, 0);


    n_A_FLEE = n_A_BaseLV + n_A_AGI;


    n_A_FLEE += StPlusItem(FLEE);


    n_A_FLEE += StPlusCard(FLEE);

    if (n_A_JobSearch() == 2 && CardNumSearch("Wanderer"))
        n_A_FLEE += 20;
    if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch("Ninetails"))
        n_A_FLEE += 20;
    if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch("Kavach Icarus"))
        n_A_FLEE += 10;
    if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch("Orc Baby"))
        n_A_FLEE += 5;

    if (n_A_PassSkill6[0] == 2 && n_A_PassSkill6[1] >= 1 && (CardNumSearch("Dokkebi") || n_A_Equip[6] == 430 || n_A_Equip[6] == 606))
        n_A_FLEE += n_A_PassSkill6[1] * 3;

    if (n_A_Equip[0] == 483)
        n_A_FLEE -= (n_A_BaseLV + SU_AGI);


    if (n_A_JOB == 8 || n_A_JOB == 14 || n_A_JOB == 22 || n_A_JOB == 28)
        n_A_FLEE += 4 * SkillSearch("Improve Dodge");
    else
        n_A_FLEE += 3 * SkillSearch("Improve Dodge");

    if (SkillSearch("Gunslinger's Panic"))
        n_A_FLEE += 30;
    if (SkillSearch("Gatling Fever"))
        n_A_FLEE -= 5 * SkillSearch("Gatling Fever");

    Mikiri = new Array(0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15);
    n_A_FLEE += Mikiri[SkillSearch("Flee")];


    if (n_A_JOB == 24)
        n_A_FLEE += Math.round(SkillSearch("Wind Walk") / 2);
    if (n_A_PassSkill2[9] && SkillSearch("Wind Walk") == 0)
        n_A_FLEE += Math.round(n_A_PassSkill2[9] / 2);


    if (SkillSearch("Close Confine"))
        n_A_FLEE += 10;


    if (SkillSearch("Lunar Protection"))
        n_A_FLEE += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);

    if (n_A_PassSkill5[4])
        n_A_FLEE += 50;

    if (n_A_PassSkill7[1])
        n_A_FLEE += 30;


    if (n_A_PassSkill3[0])
        n_A_FLEE += n_A_PassSkill3[0] + Math.floor(n_A_PassSkill3[30] / 2) + Math.floor(n_A_PassSkill3[20] / 10);

    if (SkillSearch("Frenzy"))
        n_A_FLEE /= 2;


    myInnerHtml("A_FLEE", n_A_FLEE, 0);


    n_A_LUCKY = 1 + n_A_LUK * 0.1;


    n_A_LUCKY += StPlusItem(PERFECT_DODGE);

    n_A_LUCKY += StPlusCard(PERFECT_DODGE);

    if (n_A_JobSearch() == 2)
        n_A_LUCKY += 5 * CardNumSearch("Wild Rose");

    if (n_A_JobSearch() == 1)
        n_A_LUCKY += 4 * CardNumSearch("Heater");
    if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch("Kavach Icarus"))
        n_A_LUCKY += 1;
    if (n_A_Equip[7] == 535) {
        wHPVS = n_A_JobSearch();
        if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5) {
            n_A_LUCKY += 5;
            n_A_LUCKY += n_A_SHOULDER_DEF_PLUS * 2;
        }
    }

    if (n_A_JobSearch() == 41 && EquipNumSearch("Hahoe Mask"))
        n_A_LUCKY += 2;

    n_A_LUCKY = Math.round(n_A_LUCKY * 10) / 10;


    myInnerHtml("A_LUCKY", n_A_LUCKY, 0);


    n_A_CRI = 1 + n_A_LUK * 0.3;


    n_A_CRI += StPlusItem(CRIT);

    w = 0;
    w += StPlusCard(CRIT);

    w += StPlusCard(CRITICAL_AGAINST_RACE_PERCENTAGE + targetStatsArray[TARGET_STAT_RACE]);

    if (CardNumSearch("Green Maiden"))
        w += n_A_SHOULDER_DEF_PLUS;
    if (n_A_JobSearch() == 2)
        w += 4 * CardNumSearch("Mobster");
    if (n_A_JobSearch() == 3) {
        if (targetStatsArray[TARGET_STAT_RACE] == 1 || targetStatsArray[TARGET_STAT_RACE] == 6)
            w += 9 * CardNumSearch("Fur Seal");
    }
    if (SU_LUK >= 80 && CardNumSearch("Giant Whisper"))
        w += 3;
    if (EquipNumSearch("Giant Encyclopedia"))
        w += Math.floor(SU_LUK / 5);
    if (EquipNumSearch("Sniping Suit"))
        w += Math.floor(SU_LUK / 5);
    if (EquipNumSearch("Sniping Suit *"))
        w += Math.floor(n_A_LUK / 5);

    if (EquipNumSearch("Sabath") && 90 <= targetStatsArray[TARGET_STAT_ELEMENT])
        w += 50;

    if (n_A_JobSearch() == 41 && EquipNumSearch("Bride Mask"))
        w += 5;
    if (EquipNumSearch("Heart Breaker"))
        w += weaponRefinementLevel;


    if (n_A_WeaponType ==  WEAPON_TYPE_BOW && n_A_Arrow == 15)
        w += 20;


    if (SkillSearch("Fury"))
        w += 7.5 + SkillSearch("Fury") * 2.5;
    if (SkillSearch("Fury (SuperNovice)"))
        w += 50;
    if (n_A_JOB == 24)
        w += SkillSearch("True Sight");
    n_A_CRI += w;

    if (n_A_PassSkill3[5])
        n_A_CRI += 10 + n_A_PassSkill3[5] + Math.floor(n_A_PassSkill3[35] / 2) + Math.floor(n_A_PassSkill3[25] / 10);


    if (n_A_WeaponType ==  WEAPON_TYPE_KATAR)
        n_A_CRI *= 2;

    n_A_CRI = Math.round(n_A_CRI * 10) / 10;


    myInnerHtml("A_CRI", n_A_CRI, 0);


    n_A_MATK = [0, 0, 0];

    w = Math.floor(n_A_INT / 7);
    n_A_MATK[0] = n_A_INT + w * w;


    w = Math.floor(n_A_INT / 5);
    n_A_MATK[2] = n_A_INT + w * w;

    w_MATK = 100;

    w_MATK += StPlusItem(MATK_PERCENTAGE);

    if (weaponRefinementLevel >= 9 && EquipNumSearch("Lich's Bone Wand"))
        w_MATK += 3;
    if (EquipNumSearch("Staff of Destruction"))
        w_MATK += Math.floor(weaponRefinementLevel / 2);
    if (EquipNumSearch("0") || EquipNumSearch("0"))
        w_MATK += weaponRefinementLevel;
    if (n_A_PassSkill6[2])
        w_MATK += 10;

    if (n_A_JobSearch() == 5 && CardNumSearch("0"))
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

    w_MATK += StPlusItem(MATK_BASED_ON_STAFF_PERCENTAGE);

    n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
    n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);


    myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);

    if (SkillSearch("Mystical Amplification")) {
        AmpMinMatkBK = n_A_MATK[0];
        AmpMaxMatkBK = n_A_MATK[2];
        n_A_MATK[0] = Math.floor(n_A_MATK[0] * (1 + 0.05 * SkillSearch("Mystical Amplification")));
        n_A_MATK[2] = Math.floor(n_A_MATK[2] * (1 + 0.05 * SkillSearch("Mystical Amplification")));

        myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);
        if (skillToUseName == "Stave Crasher") {
            n_A_MATK[0] = AmpMinMatkBK;
            n_A_MATK[2] = AmpMaxMatkBK;
        }
    }


    if (n_A_MATK[0] != n_A_MATK[2])
        n_A_MATK[2] -= 1;

    n_A_MATK[1] = (n_A_MATK[2] + n_A_MATK[0]) / 2;


    if (hasLeftHand == 1)
        wASPD = (200 - (JobASPD[n_A_JOB][n_A_WeaponType] + JobASPD[n_A_JOB][n_A_Weapon2Type]) / 2) * 1.4;
    else
        wASPD = 200 - JobASPD[n_A_JOB][n_A_WeaponType];


    if (hasLeftHand == 1 && n_A_WeaponType ==  WEAPON_TYPE_UNARMED && n_A_Weapon2Type != 0)
        wASPD = 200 - JobASPD[n_A_JOB][n_A_Weapon2Type];

    n_A_ASPD = 200 - wASPD + (Math.floor(wASPD * n_A_AGI * 4 / 100) + Math.floor(wASPD * n_A_DEX / 100)) / 10;

    if (n_A_Equip[0] == 47)
        n_A_ASPD += 2;


    if (SkillSearch("Cavalier Mastery") && (skillToUseName == "Basic Attack" || skillToUseName == "Martyr's Reconing"))
        n_A_ASPD -= (6 - SkillSearch("Cavalier Mastery")) * 10;

    n_A_ASPD += Math.round(SkillSearch("Single Action") / 2);


    w = 0;
    ASPDch = 0;
    if (n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SWORD && SkillSearch("Twohand Quicken")) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_WeaponType ==  WEAPON_TYPE_SWORD && SkillSearch("One Hand Quicken (Soul Linked)")) {
        w += 30;
        ASPDch = 1;
    }
    if (6 <= n_A_WeaponType && n_A_WeaponType <= 8 && SkillSearch("Andrenaline Rush")) {
        w += 30;
        ASPDch = 1;
    }
    if (ASPDch == 0 && SkillSearch("Full Andrenaline Rush")) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_WeaponType ==  WEAPON_TYPE_TWO_HANDED_SPEAR && SkillSearch("Spear Quicken")) {
        w += SkillSearch("Spear Quicken") + 20;
        ASPDch = 1;
    }
    if (EquipNumSearch("Western Outlaw"))
        w += Math.floor(SU_AGI / 5);
    if (n_A_Equip[0] == 484 && SU_STR >= 50)
        w += 5;
    if (SU_STR >= 95 && EquipNumSearch("Doom Slayer"))
        w -= 40;
    if (EquipNumSearch("Hurricane Fury"))
        w += (weaponRefinementLevel * 2);
    if (EquipNumSearch("Book of the Dead"))
        w += weaponRefinementLevel;
    if (SkillSearch("Frenzy"))
        w += 30;
    if (SkillSearch("Last Stand"))
        w += 20;
    if (SkillSearch("Gatling Fever"))
        w += 2 * SkillSearch("Gatling Fever");

    if (SkillSearch("Stellar Protection")) {
        ASPDch = 1;
        w += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);
    }

    if (SkillSearch("Solar, Lunar, and Stellar Shadow")) {
        ASPDch = 1;
        w += 3 * SkillSearch("Solar, Lunar, and Stellar Shadow");
    }
    if (ASPDch == 0 && n_A_WeaponType !=  WEAPON_TYPE_BOW && n_A_PassSkill2[6] == 2) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 1) {
        w += 25;
        ASPDch = 1;
    } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 3) {
        w += 30;
        ASPDch = 1;
    }
    if (n_A_PassSkill3[1] && n_A_WeaponType !=  WEAPON_TYPE_BOW && ASPDch == 0)
        w += 5 + n_A_PassSkill3[1] + Math.floor(n_A_PassSkill3[31] / 2) + Math.floor(n_A_PassSkill3[21] / 20);


    w += StPlusItem(ASPD_PERCENTAGE);
    w += StPlusCard(ASPD_PERCENTAGE);


    if (SkillSearch("Mental Strength"))
        w -= 25;


    if (n_A_SpeedPOT || SkillSearch("Deadly Poison (Consumed)")) {
        if (SkillSearch("Deadly Poison (Consumed)") == 0) {
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

    if (n_A_WeaponType ==  WEAPON_TYPE_BOOK && SkillSearch("Study"))
        n_A_ASPD += (200 - n_A_ASPD - (SkillSearch("Study") * 5 / 10)) * ((SkillSearch("Study") * 5 / 10) / 100);


    if (SkillSearch("Defending Aura"))
        n_A_ASPD -= (25 - SkillSearch("Defending Aura") * 5);

    if (n_A_ASPD > 190)
        n_A_ASPD = 190;


    n_A_ASPD *= 100;
    n_A_ASPD = Math.round(n_A_ASPD);
    n_A_ASPD /= 100;


    myInnerHtml("A_ASPD", n_A_ASPD, 0);

    n_A_ASPD = (200 - n_A_ASPD) / 50;

    if (isNonRangeWeapon()) {
        n_A_ATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10));
        n_A_ATK = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
    } else {
        n_A_ATK_w = Math.round(Math.floor(n_A_DEX / 10) * Math.floor(n_A_DEX / 10));
        n_A_ATK = n_A_DEX + n_A_ATK_w + Math.floor(n_A_STR / 5) + Math.floor(n_A_LUK / 5)
    }
    impositioMagnus = n_A_PassSkill2[2] * 5;
    ATK_LEFT = Math.floor(impositioMagnus + n_A_Weapon_ATK + n_A_Weapon2_ATK + n_A_ATK);
    ATK_RIGHT = Math.floor(n_A_WeaponLV_upgradeBonusATK + n_A_Weapon2LV_upgradeBonusATK);
    myInnerHtml("A_ATK_2", ATK_LEFT + "+" + ATK_RIGHT, 0),


        wDelay = 0;
    swDelay = 0;
    if (skillToUseName != "Basic Attack" && skillToUseName != "Martyr's Reconing") {
        wDelay = Math.floor(n_A_ASPD * 100) / 100;
        if (skillToUseName == "Envenom" || skillToUseName == "")
            wDelay = Math.floor(n_A_ASPD * 75) / 100;
        wA_ASPD = eval(document.calcForm.Conf01.value) / 100;
        if (wDelay < wA_ASPD)
            wDelay = wA_ASPD;
    }

    if (SkillSearch("Raging Trifecta Blow")) {
        w = 100 / (30 - SkillSearch("Raging Trifecta Blow"));
        n_A_ASPD += (n_A_ASPD - (1000 - n_A_AGI * 4 - n_A_DEX * 2) / 1000) / w;
        if (SkillSearch("<Font size=2>Add the delay time when attacking for triple attack</Font>"))
            n_A_ASPD += (0.3 / w);
    }


    n_A_CAST = 1 - n_A_DEX / 150;
    if (n_A_CAST < 0)
        n_A_CAST = 0;


    w = 100;
    if (n_A_JobSearch() == 5 && CardNumSearch("0"))
        w -= 15;
    if ((n_A_JOB == 18 || n_A_JOB == 32) && CardNumSearch("0"))
        w -= 15;
    if (EquipNumSearch("0") || EquipNumSearch("0"))
        w -= weaponRefinementLevel;
    if (n_A_card[8] == 177)
        w -= n_A_HEAD_DEF_PLUS;

    w += StPlusItem(CAST_TIME_PERCENTAGE);
    w += StPlusCard(CAST_TIME_PERCENTAGE);

    n_A_CAST *= w / 100;

    if (n_A_PassSkill2[13])
        n_A_CAST *= (100 - 15 * n_A_PassSkill2[13]) / 100;
    if (SkillSearch("Forsight"))
        n_A_CAST = n_A_CAST / 2;


    n_A_HPR = Math.floor(n_A_VIT / 5) + Math.floor(n_A_MaxHP / 200);
    if (n_A_HPR < 1)
        n_A_HPR = 1;
    w = 100;
    w += StPlusItem(HP_REGEN_PERCENTAGE);
    w += StPlusCard(HP_REGEN_PERCENTAGE);
    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling");

    if (n_A_JobSearch() == 41 && EquipNumSearch("Magistrate Hat"))
        w += 3;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus"))
        w += 5;

    n_A_HPR = Math.floor(n_A_HPR * w / 100);
    myInnerHtml("A_HPR", n_A_HPR, 0);


    n_A_SPR = Math.floor(n_A_INT / 6) + Math.floor(n_A_MaxSP / 100) + 1;

    w = 100;

    w += SkillSearch("Mediatio") * 3;

    w += StPlusItem(SP_REGEN_PERCENTAGE);
    w += StPlusCard(SP_REGEN_PERCENTAGE);

    if (SU_LUK >= 77)
        w += 100 * CardNumSearch("Arc Angeling");

    if (n_A_JobSearch() == 41 && EquipNumSearch("Ayam"))
        w += 3;
    if (n_A_LEFT_DEF_PLUS <= 4 && n_A_card[8] == 179)
        w += 5;
    if (n_A_card[9] == 179)
        w += 5;
    if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch("Gold Acidus"))
        w += 5;

    n_A_SPR = Math.floor(n_A_SPR * w / 100);

    if (n_A_INT >= 120)
        n_A_SPR += Math.floor((n_A_INT - 120) / 2) + 4;

    myInnerHtml("A_SPR", n_A_SPR, 0);

    KakutyouKansuu();
}

function StPlusCalc() {
    n_A_JobSet();
    n_A_JobLV = eval(document.calcForm.A_JobLV.value);

    wSPC_STR = JobBOBJ[n_A_JOB][n_A_JobLV - 1][0];
    wSPC_AGI = JobBOBJ[n_A_JOB][n_A_JobLV - 1][1];
    wSPC_VIT = JobBOBJ[n_A_JOB][n_A_JobLV - 1][2];
    wSPC_INT = JobBOBJ[n_A_JOB][n_A_JobLV - 1][3];
    wSPC_DEX = JobBOBJ[n_A_JOB][n_A_JobLV - 1][4];
    wSPC_LUK = JobBOBJ[n_A_JOB][n_A_JobLV - 1][5];

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


    wSPCall = StPlusItem(ALL_STATS);
    wSPC_STR += StPlusItem(STR) + wSPCall;
    wSPC_AGI += StPlusItem(AGI) + wSPCall;
    wSPC_VIT += StPlusItem(VIT) + wSPCall;
    wSPC_VIT += StPlusItem(213);
    wSPC_INT += StPlusItem(INT) + wSPCall;
    wSPC_INT += StPlusItem(214);
    wSPC_DEX += StPlusItem(DEX) + wSPCall;
    wSPC_LUK += StPlusItem(LUK) + wSPCall;

    wSPC_DEX += SkillSearch("Owl's Eye");
    wSPC_STR += SkillSearch("Crazy Uproar") * 4;
    wSPC_STR += SkillSearch("Hilt Binding");
    wSPC_STR += SkillSearch("Ninja Aura");
    wSPC_INT += SkillSearch("Ninja Aura");
    if (SkillSearch("Dragonology"))
        wSPC_INT += (Math.floor(SkillSearch("Dragonology") / 2) + 1);
    if (SkillSearch("Chase Walk")) {
        if (SkillSearch("Chase Walk") == 5) wSPC_STR += 16;
        if (SkillSearch("Chase Walk") == 4) wSPC_STR += 8;
        if (SkillSearch("Chase Walk") == 3) wSPC_STR += 4;
        if (SkillSearch("Chase Walk") == 2) wSPC_STR += 2;
        if (SkillSearch("Chase Walk") == 1) wSPC_STR += 1;
    }
    if (SkillSearch("Increase Accuracy")) {
        wSPC_DEX += 4;
        wSPC_AGI += 4;
    }

    w = SkillSearch("Improve Concentration");
    if (w) {
        w += 102;
        wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * w / 100) - n_A_DEX;
        wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * w / 100) - n_A_AGI;
    } else if (n_A_PassSkill6[3]) {
        wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * 103 / 100) - n_A_DEX;
        wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * 103 / 100) - n_A_AGI;
    }

    wSPC_AGI += StPlusItem(212);
    wSPC_DEX += StPlusItem(215);
    if (n_A_JobSearch() == 41 && EquipNumSearch("Magistrate Hat"))
        wSPC_AGI += 1;
    if (n_A_JobSearch() == 41 && EquipNumSearch("Ayam"))
        wSPC_INT += 1;
    if (n_A_JobSearch() == 41 && EquipNumSearch("Bride Mask"))
        wSPC_LUK += 2;
    if (n_A_JobSearch() == 41 && EquipNumSearch("Mythical Lion Mask"))
        wSPC_DEX += 2;
    if (n_A_JobSearch() == 41 && EquipNumSearch("Hahoe Mask"))
        wSPC_LUK += 1;
    if (n_A_SHOES_DEF_PLUS >= 9 && EquipNumSearch("Black Leather Boots"))
        wSPC_AGI += 2;

    wSPCall = StPlusCard(ALL_STATS);
    wSPC_STR += StPlusCard(STR) + wSPCall;
    wSPC_AGI += StPlusCard(AGI) + wSPCall;
    wSPC_VIT += StPlusCard(VIT) + wSPCall;
    wSPC_INT += StPlusCard(INT) + wSPCall;
    wSPC_DEX += StPlusCard(DEX) + wSPCall;
    wSPC_LUK += StPlusCard(LUK) + wSPCall;


    if (n_A_JobSearch() == 3)
        wSPC_INT += CardNumSearch("Rideword");
    if (CardNumSearch("Despero of Thanatos")) wSPC_INT += n_A_LEFT_DEF_PLUS;
    if (CardNumSearch("Green Maiden")) wSPC_LUK += n_A_SHOULDER_DEF_PLUS;
    if (CardNumSearch("Odium of Thanatos")) wSPC_AGI += n_A_SHOES_DEF_PLUS;
    if (n_A_card[8] == 180) wSPC_STR += n_A_HEAD_DEF_PLUS;

    if (CardNumSearch("Obsidian")) wSPC_VIT += Math.floor(SU_DEX / 18);
    if (CardNumSearch("Egnigem Cenia")) wSPC_STR += Math.floor(SU_INT / 18);
    if (CardNumSearch("Venatu")) wSPC_LUK += Math.floor(SU_AGI / 18);
    if (CardNumSearch("Ancient Mimic")) wSPC_AGI += Math.floor(SU_LUK / 18);
    if (CardNumSearch("Mistress of Shelter")) wSPC_INT += Math.floor(SU_STR / 18);
    if (CardNumSearch("Dame of Sentinel")) wSPC_DEX += Math.floor(SU_VIT / 18);


    if (CardNumSearch("Aliot")) {
        if (n_A_JobSearch() == 1 || n_A_JobSearch() == 2 || n_A_JobSearch() == 6)
            wSPC_STR += 2;
        if (n_A_JobSearch() == 3 || n_A_JobSearch() == 4 || n_A_JobSearch() == 5)
            wSPC_INT += 2;
    }

    wSPC_STR += n_A_PassSkill2[0];
    wSPC_INT += n_A_PassSkill2[0];
    wSPC_DEX += n_A_PassSkill2[0];
    if (n_A_PassSkill2[1] > 0)
        wSPC_AGI += n_A_PassSkill2[1] + 2;
    wSPC_LUK += (n_A_PassSkill2[3] * 30);
    if (n_A_JOB == 24 && SkillSearch("True Sight")) {
        wSPC_STR += 5;
        wSPC_AGI += 5;
        wSPC_VIT += 5;
        wSPC_DEX += 5;
        wSPC_INT += 5;
        wSPC_LUK += 5;
    }


    if (SkillSearch("Sprint (STR + State)"))
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

    if (wSPC_STR >= 0)
        myInnerHtml("A_STRp", "+" + wSPC_STR, 0);
    else
        myInnerHtml("A_STRp", wSPC_STR, 0);
    if (wSPC_AGI >= 0)
        myInnerHtml("A_AGIp", "+" + wSPC_AGI, 0);
    else
        myInnerHtml("A_AGIp", wSPC_AGI, 0);
    if (wSPC_VIT >= 0)
        myInnerHtml("A_VITp", "+" + wSPC_VIT, 0);
    else
        myInnerHtml("A_VITp", wSPC_VIT, 0);
    if (wSPC_INT >= 0)
        myInnerHtml("A_INTp", "+" + wSPC_INT, 0);
    else
        myInnerHtml("A_INTp", wSPC_INT, 0);
    if (wSPC_DEX >= 0)
        myInnerHtml("A_DEXp", "+" + wSPC_DEX, 0);
    else
        myInnerHtml("A_DEXp", wSPC_DEX, 0);
    if (wSPC_LUK >= 0)
        myInnerHtml("A_LUKp", "+" + wSPC_LUK, 0);
    else
        myInnerHtml("A_LUKp", wSPC_LUK, 0);
}


function StPlusItem(nSTP2) {
    wSTP2 = 0;
    for (STP2i = 0; STP2i <= 20; STP2i++) {
        for (STP2j = 0; ItemOBJ[n_A_Equip[STP2i]][STP2j + 11] != 0; STP2j += 2) {
            if (nSTP2 == ItemOBJ[n_A_Equip[STP2i]][STP2j + 11])
                wSTP2 += ItemOBJ[n_A_Equip[STP2i]][STP2j + 12];
        }
    }
    return wSTP2;
}


function StPlusCard(nSTP2) {
    wSTP2 = 0;
    for (STP2i = 0; STP2i <= 25; STP2i++) {
        for (STP2j = 0; cardOBJ[n_A_card[STP2i]][STP2j + 4] != 0; STP2j += 2) {
            if (nSTP2 == cardOBJ[n_A_card[STP2i]][STP2j + 4])
                wSTP2 += cardOBJ[n_A_card[STP2i]][STP2j + 5];
        }
    }
    return wSTP2;
}
