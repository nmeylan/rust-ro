var _____WB$wombat$assign$function_____ = function (name) {
    return (self._wb_wombat && self._wb_wombat.local_init && self._wb_wombat.local_init(name)) || self[name];
};
if (!self.__WB_pmw) {
    self.__WB_pmw = function (obj) {
        this.__WB_source = obj;
        return this;
    }
}

function isNonRangeWeapon() {
    return n_A_WeaponType != 10 && n_A_WeaponType != 14 && n_A_WeaponType != 15 && n_A_WeaponType != 17 && n_A_WeaponType != 18 && n_A_WeaponType != 19 && n_A_WeaponType != 20 && n_A_WeaponType != 21;
}

{
    let window = _____WB$wombat$assign$function_____("window");
    let self = _____WB$wombat$assign$function_____("self");
    let document = _____WB$wombat$assign$function_____("document");
    let location = _____WB$wombat$assign$function_____("location");
    let top = _____WB$wombat$assign$function_____("top");
    let parent = _____WB$wombat$assign$function_____("parent");
    let frames = _____WB$wombat$assign$function_____("frames");
    let opener = _____WB$wombat$assign$function_____("opener");

    myInnerHtml("PR1", "", 0);
    myInnerHtml("set", '<A Href="../other/set.html" target="_blank">Description</A>', 0);
    myInnerHtml("DELHTML", ' <Font size=2><A Href="del.html" target="migi">Delete Save Data</A></Font>', 0);


    for (i = 1; i <= 99; i++) {
        document.calcForm.A_BaseLV.options[i - 1] = new Option(i, i);
    }

    for (i = 1; i <= 99; i++) {
        document.calcForm.A_STR.options[i - 1] = new Option(i, i);
    }
    for (i = 1; i <= 99; i++) {
        document.calcForm.A_AGI.options[i - 1] = new Option(i, i);
    }
    for (i = 1; i <= 99; i++) {
        document.calcForm.A_VIT.options[i - 1] = new Option(i, i);
    }
    for (i = 1; i <= 99; i++) {
        document.calcForm.A_INT.options[i - 1] = new Option(i, i);
    }
    for (i = 1; i <= 99; i++) {
        document.calcForm.A_DEX.options[i - 1] = new Option(i, i);
    }
    for (i = 1; i <= 99; i++) {
        document.calcForm.A_LUK.options[i - 1] = new Option(i, i);
    }

    function StCalc(nSC) {
        n_A_STR = eval(document.calcForm.A_STR.value);
        n_A_AGI = eval(document.calcForm.A_AGI.value);
        n_A_VIT = eval(document.calcForm.A_VIT.value);
        n_A_DEX = eval(document.calcForm.A_DEX.value);
        n_A_INT = eval(document.calcForm.A_INT.value);
        n_A_LUK = eval(document.calcForm.A_LUK.value);

        StPoint = 0;
        for (i = 2; i <= n_A_STR; i++)
            StPoint += StCalc2(i);
        for (i = 2; i <= n_A_AGI; i++)
            StPoint += StCalc2(i);
        for (i = 2; i <= n_A_VIT; i++)
            StPoint += StCalc2(i);
        for (i = 2; i <= n_A_INT; i++)
            StPoint += StCalc2(i);
        for (i = 2; i <= n_A_DEX; i++)
            StPoint += StCalc2(i);
        for (i = 2; i <= n_A_LUK; i++)
            StPoint += StCalc2(i);

        n_A_BaseLV = eval(document.calcForm.A_BaseLV.value);

        n_A_JobSet();
        if (isRebirth)
            statusPoint = 100;
        else
            statusPoint = 48;

        if (nSC == 1 || document.calcForm.BLVauto.checked == 0) {
            for (i = 1; i < n_A_BaseLV; i++)
                statusPoint += Math.floor((i) / 5) + 3;
        } else {
            for (i = 1; StPoint > statusPoint && i < 99; i++)
                statusPoint += Math.floor((i) / 5) + 3;
        }
        if (i > 99) i = 99;
        document.calcForm.A_BaseLV.value = i;
        myInnerHtml("A_STPOINT", statusPoint - StPoint, 0);
    }

    function StCalc2(nSC2) {
        return Math.floor((nSC2 - 2) / 10) + 2;
    }

    function SuperNoviceFullWeapon(nSNFW) {
        if (nSNFW == 1) {
            SuperNoviceFullWeaponCHECK = 1;
            JobASPD[20][7] = 120;
        } else {
            SuperNoviceFullWeaponCHECK = 0;
            JobASPD[20][7] = 0;
        }

        for (i = 21; i >= 0; i--)
            document.calcForm.A_WeaponType.options[i] = null;
        j = 0;
        for (i = 0; i <= 21; i++) {
            if (JobASPD[20][i] != 0) {
                document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i], i);
                j++;
            }
        }
        ClickWeaponType(0);
        WeaponSet();
        WeaponSet2();
    }

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
        if(hasLeftHand){

            if(n_B[19] != 5){

                n_A_Weapon2 = eval(document.calcForm.A_weapon2.value);
                n_A_Weapon2LV = ItemOBJ[n_A_Weapon2][4];
                n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
                n_A_Weapon2_RefinementLevel = eval(document.calcForm.A_Weapon2_ATKplus.value);


                if(n_A_Weapon2LV == 1){
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 2;
                    if(n_A_Weapon2_RefinementLevel >= 8){
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 3 * (n_A_Weapon2_RefinementLevel - 7);
                    }
                }else if(n_A_Weapon2LV == 2){
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 3;
                    if(n_A_Weapon2_RefinementLevel >= 7){
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 5 * (n_A_Weapon2_RefinementLevel - 6);
                    }
                }else if(n_A_Weapon2LV == 3){
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 5;
                    if(n_A_Weapon2_RefinementLevel >= 6){
                        n_A_Weapon2LV_Minplus = 1;
                        n_A_Weapon2LV_overUpgradeBonusATK = 8 * (n_A_Weapon2_RefinementLevel - 5);
                    }
                }else if(n_A_Weapon2LV == 4){
                    n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 7;
                    if(n_A_Weapon2_RefinementLevel >= 5){
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

            if (n_A_WeaponType == 10 || n_A_WeaponType == 17 || n_A_WeaponType == 18 || n_A_WeaponType == 19 || n_A_WeaponType == 20 || n_A_WeaponType == 21) {
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


        w = StPlusCard(17);
        w += StPlusCalc2(17);

        if (SU_STR >= 80 && CardNumSearch(267))
            w += 20;
        if (SU_STR >= 95 && EquipNumSearch(621))
            w += 340;
        if (SU_STR >= 44 && EquipNumSearch(625))
            w += 44;
        if (EquipNumSearch(676))
            w += n_A_HEAD_DEF_PLUS * 2;

        if (n_A_PassSkill6[0] == 0 && n_A_PassSkill6[1] >= 1 && (CardNumSearch(130) || n_A_Equip[6] == 428 || n_A_Equip[6] == 604))
            w += n_A_PassSkill6[1] * 10;

        if (n_A_PassSkill7[2])
            w += 10;
        if (n_A_PassSkill7[9])
            w += 20;

        if (SkillSearch(420))
            w += 100;
        if (SkillSearch(433))
            w += 20 + 10 * SkillSearch(433);

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

        if (SkillSearch(345) && n_A_BaseLV >= 90)
            n_A_MaxHP *= 3;


        n_A_MaxHP += SkillSearch(156) * 200;
        bkHP = n_A_MaxHP;
        w = 0;

        w += StPlusCalc2(13);
        w += StPlusCalc2(3);


        w += StPlusCard(13);
        if (n_A_BODY_DEF_PLUS >= 9 && CardNumSearch(225))
            w += 800;

        //Temporary remover card code.
        if (CardNumSearch(186))
            w -= n_A_BODY_DEF_PLUS * 40;

        if (n_A_Equip[8] == 536) {
            wHPVS = n_A_JobSearch();
            if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5)
                w += 5 * n_A_BaseLV;
        }

        n_A_MaxHP += w;

        w = 0;

        w += StPlusCalc2(15);

        w += StPlusCard(15);

        if (SU_VIT >= 80 && CardNumSearch(267))
            w += 3;

        if (CardNumSearch(405)) {
            if (n_A_JobSearch() == 1 || n_A_JobSearch() == 2 || n_A_JobSearch() == 6)
                w += 5;
        }
        if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch(304))
            w += 10;
        if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch(407))
            w += 4;
        if (n_A_PassSkill5[1])
            w += 100;
        if (EquipNumSearch(715))
            w -= n_A_SHOES_DEF_PLUS;

        n_A_MaxHP = n_A_MaxHP * (100 + w) / 100;

        if (n_A_PassSkill6[0] == 1 && n_A_PassSkill6[1] >= 1 && (CardNumSearch(128) || n_A_Equip[6] == 429 || n_A_Equip[6] == 605)) {
            dHP = [5, 9, 12, 14, 15];
            n_A_MaxHP = n_A_MaxHP * (100 + dHP[n_A_PassSkill6[1] - 1]) / 100;
        }
        if (SkillSearch(258))
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

        if (SkillSearch(345) && n_A_BaseLV >= 90)
            n_A_MaxSP *= 3;

        w = 0;

        w += StPlusCalc2(14);
        w += StPlusCalc2(4);

        w += StPlusCard(14);
        if (n_A_HEAD_DEF_PLUS >= 9 && n_A_card[8] == 298)
            w += 150;
        if (n_A_HEAD_DEF_PLUS <= 4 && n_A_card[8] == 179)
            w += 40;
        if (n_A_card[9] == 179)
            w += 40;

        if (SkillSearch(372))
            w += 30 * SkillSearch(372);

        if (n_A_Equip[8] == 536) {
            wSPVS = n_A_JobSearch();
            if (wSPVS == 1 || wSPVS == 2 || wSPVS == 6)
                w += 2 * n_A_JobLV;
        }
        if (weaponRefinementLevel >= 9 && EquipNumSearch(642))
            w += 300;


        n_A_MaxSP += w;

        w = 0;

        w += StPlusCalc2(16);

        w += StPlusCard(16);
        if (n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch(304))
            w += 10;
        if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch(407))
            w += 4;

        if (CardNumSearch(405)) {
            if (n_A_JobSearch() == 3 || n_A_JobSearch() == 4 || n_A_JobSearch() == 5)
                w += 5;
        }


        w += SkillSearch(269);

        w += SkillSearch(274) * 2;
        if (n_A_PassSkill5[2])
            w += 100;
        if (EquipNumSearch(715))
            w -= n_A_SHOES_DEF_PLUS;

        n_A_MaxSP = Math.floor(n_A_MaxSP * (100 + w) / 100);

        if (n_A_PassSkill3[6])
            n_A_MaxSP += (Math.floor(bkSP * (100 + n_A_PassSkill3[6] * 2 + n_A_PassSkill3[36] + Math.floor(n_A_PassSkill3[26] / 10)) / 100) - bkSP);


        if (n_A_MaxSP >= 100)
            myInnerHtml("A_MaxSP", n_A_MaxSP, 0);
        else
            myInnerHtml("A_MaxSP", " " + n_A_MaxSP, 0);


        n_A_DEF = StPlusCalc2(18);

        for (i = 2; i <= 10; i++) {
            n_A_DEF += ItemOBJ[n_A_Equip[i]][3];
        }

        n_A_DEF += StPlusCard(18);

        if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch(222))
            n_A_DEF += 2;
        if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch(283))
            n_A_DEF += 2;
        if (n_A_Equip[0] == 521) {
            if (weaponRefinementLevel <= 5)
                n_A_DEF += 2;
            else if (weaponRefinementLevel >= 9)
                n_A_DEF += 5;
            else
                n_A_DEF += 3;
        }
        if (EquipNumSearch(658))
            n_A_DEF += weaponRefinementLevel;
        if (EquipNumSearch(715))
            n_A_DEF += n_A_SHOES_DEF_PLUS;
        if (EquipNumSearch(742) && n_A_JobSearch() == 1)
            n_A_DEF += 6;

        if (EquipNumSearch(764))
            n_A_DEFplus -= (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

        n_A_totalDEF = n_A_DEF + Math.round(n_A_DEFplus * 7 / 10);

        if (StPlusCalc2(24) + StPlusCard(24))
            n_A_totalDEF = Math.floor(n_A_totalDEF / StPlusCalc2(24));
        if (StPlusCalc2(85) + StPlusCard(85))
            n_A_totalDEF -= Math.floor(n_A_totalDEF * (StPlusCalc2(85) + StPlusCard(85)) / 100);

        if (SkillSearch(256))
            n_A_totalDEF = Math.floor(n_A_totalDEF * (1 - 0.05 * SkillSearch(256)));


        if (SkillSearch(196))
            n_A_totalDEF = 90;

        if (SkillSearch(258))
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
        if (SkillSearch(12)) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.45);
        } else {
            if (n_A_PassSkill2[12]) {
                for (i = 0; i <= 2; i++)
                    n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * 0.9);
            }
        }
        if (StPlusCalc2(24)) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] / StPlusCalc2(24));
        }
        if (SkillSearch(256)) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 - 0.05 * SkillSearch(256)));
        }
        if (n_A_PassSkill2[4]) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = Math.floor(n_A_VITDEF[i] * (1 + 0.05 * n_A_PassSkill2[4]));
        }
        if (SkillSearch(258)) {
            for (i = 0; i <= 2; i++)
                n_A_VITDEF[i] = 0;
        }


        n_A_MDEF = StPlusCalc2(19);


        n_A_MDEF += StPlusCard(19);

        if (n_A_JobSearch() == 3)
            n_A_MDEF += CardNumSearch(383);
        if (n_A_LEFT_DEF_PLUS >= 9 && CardNumSearch(310))
            n_A_MDEF += 5;
        if (n_A_HEAD_DEF_PLUS <= 5 && n_A_card[8] == 213)
            n_A_MDEF += 5;
        if (n_A_card[9] == 213)
            n_A_MDEF += 5;
        if (n_A_LEFT_DEF_PLUS <= 5 && CardNumSearch(222))
            n_A_MDEF += 3;
        if (n_A_BODY_DEF_PLUS <= 5 && CardNumSearch(283))
            n_A_MDEF += 5;
        if (n_A_SHOES_DEF_PLUS <= 5 && CardNumSearch(381))
            n_A_MDEF += 7;
        if (n_A_SHOULDER_DEF_PLUS <= 5 && CardNumSearch(258))
            n_A_MDEF += 8;
        if (EquipNumSearch(764))
            n_A_MDEF += (n_A_HEAD_DEF_PLUS + n_A_LEFT_DEF_PLUS);

        if (SkillSearch(256))
            n_A_MDEF += 1;
        if (SkillSearch(9))
            n_A_MDEF += SkillSearch(9);


        if (SkillSearch(196))
            n_A_MDEF = 90;
        if (SkillSearch(258))
            n_A_MDEF = 0;

        myInnerHtml("A_MDEF", n_A_MDEF, 0);


        n_A_HIT = n_A_BaseLV + n_A_DEX;


        n_A_HIT += StPlusCalc2(8);


        n_A_HIT += StPlusCard(8);

        if (EquipNumSearch(656))
            w -= Math.floor(SU_DEX / 3);


        n_A_HIT += 1 * SkillSearch(39);
        n_A_HIT += 2 * SkillSearch(148);
        n_A_HIT += 3 * SkillSearch(270);

        n_A_HIT += 10 * SkillSearch(256);
        n_A_HIT += 1 * SkillSearch(426);
        if (SkillSearch(421))
            n_A_HIT -= 30;
        if (SkillSearch(422))
            n_A_HIT += 20;
        n_A_HIT += 2 * SkillSearch(425);

        if (EquipNumSearch(654))
            n_A_HIT += Math.floor(SU_AGI / 5);

        if (n_A_ActiveSkill == 324)
            n_A_HIT += 20;

        if (n_A_PassSkill5[4])
            n_A_HIT += 50;

        if (n_A_PassSkill7[0])
            n_A_HIT += 30;


        if (n_A_PassSkill3[4])
            n_A_HIT += n_A_PassSkill3[4] + Math.floor(n_A_PassSkill3[34] / 2) + Math.floor(n_A_PassSkill3[24] / 10);


        myInnerHtml("A_HIT", n_A_HIT, 0);


        n_A_FLEE = n_A_BaseLV + n_A_AGI;


        n_A_FLEE += StPlusCalc2(9);


        n_A_FLEE += StPlusCard(9);

        if (n_A_JobSearch() == 2 && CardNumSearch(295))
            n_A_FLEE += 20;
        if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch(271))
            n_A_FLEE += 20;
        if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch(401))
            n_A_FLEE += 10;
        if (n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch(403))
            n_A_FLEE += 5;

        if (n_A_PassSkill6[0] == 2 && n_A_PassSkill6[1] >= 1 && (CardNumSearch(131) || n_A_Equip[6] == 430 || n_A_Equip[6] == 606))
            n_A_FLEE += n_A_PassSkill6[1] * 3;

        if (n_A_Equip[0] == 483)
            n_A_FLEE -= (n_A_BaseLV + SU_AGI);


        if (n_A_JOB == 8 || n_A_JOB == 14 || n_A_JOB == 22 || n_A_JOB == 28)
            n_A_FLEE += 4 * SkillSearch(14);
        else
            n_A_FLEE += 3 * SkillSearch(14);

        if (SkillSearch(421))
            n_A_FLEE += 30;
        if (SkillSearch(433))
            n_A_FLEE -= 5 * SkillSearch(433);

        Mikiri = new Array(0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15);
        n_A_FLEE += Mikiri[SkillSearch(191)];


        if (n_A_JOB == 24)
            n_A_FLEE += Math.round(SkillSearch(273) / 2);
        if (n_A_PassSkill2[9] && SkillSearch(273) == 0)
            n_A_FLEE += Math.round(n_A_PassSkill2[9] / 2);


        if (SkillSearch(383))
            n_A_FLEE += 10;


        if (SkillSearch(356))
            n_A_FLEE += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);

        if (n_A_PassSkill5[4])
            n_A_FLEE += 50;

        if (n_A_PassSkill7[1])
            n_A_FLEE += 30;


        if (n_A_PassSkill3[0])
            n_A_FLEE += n_A_PassSkill3[0] + Math.floor(n_A_PassSkill3[30] / 2) + Math.floor(n_A_PassSkill3[20] / 10);

        if (SkillSearch(258))
            n_A_FLEE /= 2;


        myInnerHtml("A_FLEE", n_A_FLEE, 0);


        n_A_LUCKY = 1 + n_A_LUK * 0.1;


        n_A_LUCKY += StPlusCalc2(11);

        n_A_LUCKY += StPlusCard(11);

        if (n_A_JobSearch() == 2)
            n_A_LUCKY += 5 * CardNumSearch(391);

        if (n_A_JobSearch() == 1)
            n_A_LUCKY += 4 * CardNumSearch(354);
        if (n_A_SHOULDER_DEF_PLUS <= 4 && CardNumSearch(401))
            n_A_LUCKY += 1;
        if (n_A_Equip[7] == 535) {
            wHPVS = n_A_JobSearch();
            if (wHPVS == 3 || wHPVS == 4 || wHPVS == 5) {
                n_A_LUCKY += 5;
                n_A_LUCKY += n_A_SHOULDER_DEF_PLUS * 2;
            }
        }

        if (n_A_JobSearch() == 41 && EquipNumSearch(678))
            n_A_LUCKY += 2;

        n_A_LUCKY = Math.round(n_A_LUCKY * 10) / 10;


        myInnerHtml("A_LUCKY", n_A_LUCKY, 0);


        n_A_CRI = 1 + n_A_LUK * 0.3;


        n_A_CRI += StPlusCalc2(10);

        w = 0;
        w += StPlusCard(10);

        w += StPlusCard(110 + n_B[2]);

        if (CardNumSearch(402))
            w += n_A_SHOULDER_DEF_PLUS;
        if (n_A_JobSearch() == 2)
            w += 4 * CardNumSearch(328);
        if (n_A_JobSearch() == 3) {
            if (n_B[2] == 1 || n_B[2] == 6)
                w += 9 * CardNumSearch(253);
        }
        if (SU_LUK >= 80 && CardNumSearch(267))
            w += 3;
        if (EquipNumSearch(640))
            w += Math.floor(SU_LUK / 5);
        if (EquipNumSearch(689))
            w += Math.floor(SU_LUK / 5);
        if (EquipNumSearch(762))
            w += Math.floor(n_A_LUK / 5);

        if (EquipNumSearch(416) && 90 <= n_B[3])
            w += 50;

        if (n_A_JobSearch() == 41 && EquipNumSearch(675))
            w += 5;
        if (EquipNumSearch(623))
            w += weaponRefinementLevel;


        if (n_A_WeaponType == 10 && n_A_Arrow == 15)
            w += 20;


        if (SkillSearch(195))
            w += 7.5 + SkillSearch(195) * 2.5;
        if (SkillSearch(253))
            w += 50;
        if (n_A_JOB == 24)
            w += SkillSearch(270);
        n_A_CRI += w;

        if (n_A_PassSkill3[5])
            n_A_CRI += 10 + n_A_PassSkill3[5] + Math.floor(n_A_PassSkill3[35] / 2) + Math.floor(n_A_PassSkill3[25] / 10);


        if (n_A_WeaponType == 11)
            n_A_CRI *= 2;

        n_A_CRI = Math.round(n_A_CRI * 10) / 10;


        myInnerHtml("A_CRI", n_A_CRI, 0);


        n_A_MATK = [0, 0, 0];

        w = Math.floor(n_A_INT / 7);
        n_A_MATK[0] = n_A_INT + w * w;


        w = Math.floor(n_A_INT / 5);
        n_A_MATK[2] = n_A_INT + w * w;

        w_MATK = 100;

        w_MATK += StPlusCalc2(89);

        if (weaponRefinementLevel >= 9 && EquipNumSearch(642))
            w_MATK += 3;
        if (EquipNumSearch(646))
            w_MATK += Math.floor(weaponRefinementLevel / 2);
        if (EquipNumSearch(737) || EquipNumSearch(769))
            w_MATK += weaponRefinementLevel;
        if (n_A_PassSkill6[2])
            w_MATK += 10;

        if (n_A_JobSearch() == 5 && CardNumSearch(454))
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

        w_MATK += StPlusCalc2(88);

        n_A_MATK[0] = Math.floor(n_A_MATK[0] * w_MATK / 100);
        n_A_MATK[2] = Math.floor(n_A_MATK[2] * w_MATK / 100);


        myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);

        if (SkillSearch(276)) {
            AmpMinMatkBK = n_A_MATK[0];
            AmpMaxMatkBK = n_A_MATK[2];
            n_A_MATK[0] = Math.floor(n_A_MATK[0] * (1 + 0.05 * SkillSearch(276)));
            n_A_MATK[2] = Math.floor(n_A_MATK[2] * (1 + 0.05 * SkillSearch(276)));

            myInnerHtml("A_MATK", n_A_MATK[0] + " ~ " + n_A_MATK[2], 0);
            if (n_A_ActiveSkill == 275) {
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


        if (hasLeftHand == 1 && n_A_WeaponType == 0 && n_A_Weapon2Type != 0)
            wASPD = 200 - JobASPD[n_A_JOB][n_A_Weapon2Type];

        n_A_ASPD = 200 - wASPD + (Math.floor(wASPD * n_A_AGI * 4 / 100) + Math.floor(wASPD * n_A_DEX / 100)) / 10;

        if (n_A_Equip[0] == 47)
            n_A_ASPD += 2;


        if (SkillSearch(78) && (n_A_ActiveSkill == 0 || n_A_ActiveSkill == 284))
            n_A_ASPD -= (6 - SkillSearch(78)) * 10;

        n_A_ASPD += Math.round(SkillSearch(425) / 2);


        w = 0;
        ASPDch = 0;
        if (n_A_WeaponType == 3 && SkillSearch(74)) {
            w += 30;
            ASPDch = 1;
        }
        if (n_A_WeaponType == 2 && SkillSearch(386)) {
            w += 30;
            ASPDch = 1;
        }
        if (6 <= n_A_WeaponType && n_A_WeaponType <= 8 && SkillSearch(152)) {
            w += 30;
            ASPDch = 1;
        }
        if (ASPDch == 0 && SkillSearch(389)) {
            w += 30;
            ASPDch = 1;
        }
        if (n_A_WeaponType == 5 && SkillSearch(166)) {
            w += SkillSearch(166) + 20;
            ASPDch = 1;
        }
        if (EquipNumSearch(654))
            w += Math.floor(SU_AGI / 5);
        if (n_A_Equip[0] == 484 && SU_STR >= 50)
            w += 5;
        if (SU_STR >= 95 && EquipNumSearch(621))
            w -= 40;
        if (EquipNumSearch(624))
            w += (weaponRefinementLevel * 2);
        if (EquipNumSearch(641))
            w += weaponRefinementLevel;
        if (SkillSearch(258))
            w += 30;
        if (SkillSearch(420))
            w += 20;
        if (SkillSearch(433))
            w += 2 * SkillSearch(433);

        if (SkillSearch(357)) {
            ASPDch = 1;
            w += Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 10);
        }

        if (SkillSearch(361)) {
            ASPDch = 1;
            w += 3 * SkillSearch(361);
        }
        if (ASPDch == 0 && n_A_WeaponType != 10 && n_A_PassSkill2[6] == 2) {
            w += 25;
            ASPDch = 1;
        } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 1) {
            w += 25;
            ASPDch = 1;
        } else if (ASPDch == 0 && 6 <= n_A_WeaponType && n_A_WeaponType <= 8 && n_A_PassSkill2[6] == 3) {
            w += 30;
            ASPDch = 1;
        }
        if (n_A_PassSkill3[1] && n_A_WeaponType != 10 && ASPDch == 0)
            w += 5 + n_A_PassSkill3[1] + Math.floor(n_A_PassSkill3[31] / 2) + Math.floor(n_A_PassSkill3[21] / 20);


        w += StPlusCalc2(12);
        w += StPlusCard(12);


        if (SkillSearch(196))
            w -= 25;


        if (n_A_SpeedPOT || SkillSearch(323)) {
            if (SkillSearch(323) == 0) {
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

        if (n_A_WeaponType == 12 && SkillSearch(224))
            n_A_ASPD += (200 - n_A_ASPD - (SkillSearch(224) * 5 / 10)) * ((SkillSearch(224) * 5 / 10) / 100);


        if (SkillSearch(165))
            n_A_ASPD -= (25 - SkillSearch(165) * 5);

        if (n_A_ASPD > 190)
            n_A_ASPD = 190;


        n_A_ASPD *= 100;
        n_A_ASPD = Math.round(n_A_ASPD);
        n_A_ASPD /= 100;


        myInnerHtml("A_ASPD", n_A_ASPD, 0);

        n_A_ASPD = (200 - n_A_ASPD) / 50;

        if (isNonRangeWeapon()) {
            n_A_ATK_w=Math.round(Math.floor(n_A_STR/10)*Math.floor(n_A_STR/10));
            n_A_ATK=n_A_STR+n_A_ATK_w+Math.floor(n_A_DEX/5)+Math.floor(n_A_LUK/5);
        } else {
            n_A_ATK_w=Math.round(Math.floor(n_A_DEX/10)*Math.floor(n_A_DEX/10));
            n_A_ATK=n_A_DEX+n_A_ATK_w+Math.floor(n_A_STR/5)+Math.floor(n_A_LUK/5)
        }
        impositioMagnus = n_A_PassSkill2[2] *5;
        ATK_LEFT = Math.floor(impositioMagnus+n_A_Weapon_ATK+n_A_Weapon2_ATK+n_A_ATK);
        ATK_RIGHT = Math.floor(n_A_WeaponLV_upgradeBonusATK+n_A_Weapon2LV_upgradeBonusATK);
        myInnerHtml("A_ATK_2",ATK_LEFT+"+"+ATK_RIGHT,0),


        wDelay = 0;
        swDelay = 0;
        if (n_A_ActiveSkill != 0 && n_A_ActiveSkill != 284) {
            wDelay = Math.floor(n_A_ASPD * 100) / 100;
            if (n_A_ActiveSkill == 17 || n_A_ActiveSkill == 307)
                wDelay = Math.floor(n_A_ASPD * 75) / 100;
            wA_ASPD = eval(document.calcForm.Conf01.value) / 100;
            if (wDelay < wA_ASPD)
                wDelay = wA_ASPD;
        }

        if (SkillSearch(187)) {
            w = 100 / (30 - SkillSearch(187));
            n_A_ASPD += (n_A_ASPD - (1000 - n_A_AGI * 4 - n_A_DEX * 2) / 1000) / w;
            if (SkillSearch(301))
                n_A_ASPD += (0.3 / w);
        }


        n_A_CAST = 1 - n_A_DEX / 150;
        if (n_A_CAST < 0)
            n_A_CAST = 0;


        w = 100;
        if (n_A_JobSearch() == 5 && CardNumSearch(454))
            w -= 15;
        if ((n_A_JOB == 18 || n_A_JOB == 32) && CardNumSearch(460))
            w -= 15;
        if (EquipNumSearch(750) || EquipNumSearch(770))
            w -= weaponRefinementLevel;
        if (n_A_card[8] == 177)
            w -= n_A_HEAD_DEF_PLUS;

        w += StPlusCalc2(73);
        w += StPlusCard(73);

        n_A_CAST *= w / 100;

        if (n_A_PassSkill2[13])
            n_A_CAST *= (100 - 15 * n_A_PassSkill2[13]) / 100;
        if (SkillSearch(322))
            n_A_CAST = n_A_CAST / 2;


        n_A_HPR = Math.floor(n_A_VIT / 5) + Math.floor(n_A_MaxHP / 200);
        if (n_A_HPR < 1)
            n_A_HPR = 1;
        w = 100;
        w += StPlusCalc2(75);
        w += StPlusCard(75);
        if (SU_LUK >= 77)
            w += 100 * CardNumSearch(221);

        if (n_A_JobSearch() == 41 && EquipNumSearch(672))
            w += 3;
        if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch(407))
            w += 5;

        n_A_HPR = Math.floor(n_A_HPR * w / 100);
        myInnerHtml("A_HPR", n_A_HPR, 0);


        n_A_SPR = Math.floor(n_A_INT / 6) + Math.floor(n_A_MaxSP / 100) + 1;

        w = 100;

        w += SkillSearch(269) * 3;

        w += StPlusCalc2(76);
        w += StPlusCard(76);

        if (SU_LUK >= 77)
            w += 100 * CardNumSearch(221);

        if (n_A_JobSearch() == 41 && EquipNumSearch(673))
            w += 3;
        if (n_A_LEFT_DEF_PLUS <= 4 && n_A_card[8] == 179)
            w += 5;
        if (n_A_card[9] == 179)
            w += 5;
        if (n_A_SHOES_DEF_PLUS <= 4 && CardNumSearch(407))
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


        wSPCall = StPlusCalc2(7);
        wSPC_STR += StPlusCalc2(1) + wSPCall;
        wSPC_AGI += StPlusCalc2(2) + wSPCall;
        wSPC_VIT += StPlusCalc2(3) + wSPCall;
        wSPC_VIT += StPlusCalc2(213);
        wSPC_INT += StPlusCalc2(4) + wSPCall;
        wSPC_INT += StPlusCalc2(214);
        wSPC_DEX += StPlusCalc2(5) + wSPCall;
        wSPC_LUK += StPlusCalc2(6) + wSPCall;

        wSPC_DEX += SkillSearch(38);
        wSPC_STR += SkillSearch(68) * 4;
        wSPC_STR += SkillSearch(146);
        wSPC_STR += SkillSearch(404);
        wSPC_INT += SkillSearch(404);
        if (SkillSearch(234))
            wSPC_INT += (Math.floor(SkillSearch(234) / 2) + 1);
        if (SkillSearch(286)) {
            if (SkillSearch(286) == 5) wSPC_STR += 16;
            if (SkillSearch(286) == 4) wSPC_STR += 8;
            if (SkillSearch(286) == 3) wSPC_STR += 4;
            if (SkillSearch(286) == 2) wSPC_STR += 2;
            if (SkillSearch(286) == 1) wSPC_STR += 1;
        }
        if (SkillSearch(422)) {
            wSPC_DEX += 4;
            wSPC_AGI += 4;
        }

        w = SkillSearch(42);
        if (w) {
            w += 102;
            wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * w / 100) - n_A_DEX;
            wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * w / 100) - n_A_AGI;
        } else if (n_A_PassSkill6[3]) {
            wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * 103 / 100) - n_A_DEX;
            wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * 103 / 100) - n_A_AGI;
        }

        wSPC_AGI += StPlusCalc2(212);
        wSPC_DEX += StPlusCalc2(215);
        if (n_A_JobSearch() == 41 && EquipNumSearch(672))
            wSPC_AGI += 1;
        if (n_A_JobSearch() == 41 && EquipNumSearch(673))
            wSPC_INT += 1;
        if (n_A_JobSearch() == 41 && EquipNumSearch(675))
            wSPC_LUK += 2;
        if (n_A_JobSearch() == 41 && EquipNumSearch(676))
            wSPC_DEX += 2;
        if (n_A_JobSearch() == 41 && EquipNumSearch(678))
            wSPC_LUK += 1;
        if (n_A_SHOES_DEF_PLUS >= 9 && EquipNumSearch(717))
            wSPC_AGI += 2;

        wSPCall = StPlusCard(7);
        wSPC_STR += StPlusCard(1) + wSPCall;
        wSPC_AGI += StPlusCard(2) + wSPCall;
        wSPC_VIT += StPlusCard(3) + wSPCall;
        wSPC_INT += StPlusCard(4) + wSPCall;
        wSPC_DEX += StPlusCard(5) + wSPCall;
        wSPC_LUK += StPlusCard(6) + wSPCall;


        if (n_A_JobSearch() == 3)
            wSPC_INT += CardNumSearch(383);
        if (CardNumSearch(173)) wSPC_INT += n_A_LEFT_DEF_PLUS;
        if (CardNumSearch(402)) wSPC_LUK += n_A_SHOULDER_DEF_PLUS;
        if (CardNumSearch(406)) wSPC_AGI += n_A_SHOES_DEF_PLUS;
        if (n_A_card[8] == 180) wSPC_STR += n_A_HEAD_DEF_PLUS;

        if (CardNumSearch(185)) wSPC_VIT += Math.floor(SU_DEX / 18);
        if (CardNumSearch(187)) wSPC_STR += Math.floor(SU_INT / 18);
        if (CardNumSearch(189)) wSPC_LUK += Math.floor(SU_AGI / 18);
        if (CardNumSearch(191)) wSPC_AGI += Math.floor(SU_LUK / 18);
        if (CardNumSearch(196)) wSPC_INT += Math.floor(SU_STR / 18);
        if (CardNumSearch(197)) wSPC_DEX += Math.floor(SU_VIT / 18);


        if (CardNumSearch(405)) {
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
        if (n_A_JOB == 24 && SkillSearch(270)) {
            wSPC_STR += 5;
            wSPC_AGI += 5;
            wSPC_VIT += 5;
            wSPC_DEX += 5;
            wSPC_INT += 5;
            wSPC_LUK += 5;
        }


        if (SkillSearch(379))
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


    function StPlusCalc2(nSTP2) {
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

    function sort(work) {
        for (i = 1; work[i] != "EOF"; i++) {
            for (k = i; k > 0; k--) {
                if (ItemOBJ[work[k - 1]][8] > ItemOBJ[work[k]][8]) {
                    work_backup = work[k - 1];
                    work[k - 1] = work[k];
                    work[k] = work_backup;
                }
            }
        }
        return work;
    }

    function WeaponSet() {
        n_A_JobSet();
        n_A_WeaponType = eval(document.calcForm.A_WeaponType.value);
        for (i = 50; i >= 0; i--) {
            document.calcForm.A_weapon1.options[i] = null;
        }

        work = new Array();
        j = 0;
        for (i = 0; i <= ItemMax; i++) {
            if (ItemOBJ[i][1] == n_A_WeaponType && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                work[j] = i;
                j++;
            } else if (ItemOBJ[i][4] == 4 && ItemOBJ[i][1] == n_A_WeaponType && SuperNoviceFullWeaponCHECK) {
                work[j] = i;
                j++;
            }
        }
        work[j] = "EOF";


        work = sort(work);
        for (i = 0; i < j; i++)
            document.calcForm.A_weapon1.options[i] = new Option(ItemOBJ[work[i]][8], ItemOBJ[work[i]][0]);

    }

    function WeaponSetLeft() {
        n_A_JobSet();
        n_A_Weapon2Type = eval(document.calcForm.A_Weapon2Type.value);
        for (i = 50; i >= 0; i--) {
            document.calcForm.A_weapon2.options[i] = null;
        }
        work = new Array();
        j = 0;
        for (i = 0; i <= ItemMax; i++) {
            if (ItemOBJ[i][1] == n_A_Weapon2Type && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                work[j] = i;
                j++;
            }
        }
        work[j] = "EOF";
        work = sort(work);
        for (i = 0; i < j; i++)
            document.calcForm.A_weapon2.options[i] = new Option(ItemOBJ[work[i]][8], ItemOBJ[work[i]][0]);

    }


    function WeaponSet2() {
        n_A_JobSet();
        for (i = 120; i >= 0; i--) {
            document.calcForm.A_head1.options[i] = null;
        }
        for (i = 40; i >= 0; i--) {
            document.calcForm.A_head2.options[i] = null;
            document.calcForm.A_head3.options[i] = null;
            document.calcForm.A_left.options[i] = null;
            document.calcForm.A_body.options[i] = null;
            document.calcForm.A_shoulder.options[i] = null;
            document.calcForm.A_shoes.options[i] = null;
            document.calcForm.A_acces1.options[i] = null;
            document.calcForm.A_acces2.options[i] = null;
        }
        workB = new Array();
        for (i = 0; i <= 7; i++)
            workB[i] = new Array();
        wsj = new Array();
        for (i = 0; i <= 7; i++)
            wsj[i] = 0;
        for (i = 0; i <= ItemMax; i++) {
            if (ItemOBJ[i][1] == 50 && (JobEquipItemSearch(ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
                workB[0][wsj[0]] = i;
                wsj[0]++;
            } else if (ItemOBJ[i][1] == 51 && (JobEquipItemSearch(ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
                workB[1][wsj[1]] = i;
                wsj[1]++;
            } else if (ItemOBJ[i][1] == 52 && (JobEquipItemSearch(ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
                workB[2][wsj[2]] = i;
                wsj[2]++;
            } else if (ItemOBJ[i][1] == 61 && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                workB[3][wsj[3]] = i;
                wsj[3]++;
            } else if (ItemOBJ[i][1] == 60 && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                workB[4][wsj[4]] = i;
                wsj[4]++;
            } else if (ItemOBJ[i][1] == 62 && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                workB[5][wsj[5]] = i;
                wsj[5]++;
            } else if (ItemOBJ[i][1] == 63 && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                workB[6][wsj[6]] = i;
                wsj[6]++;
            } else if (ItemOBJ[i][1] == 64 && JobEquipItemSearch(ItemOBJ[i][2]) == 1) {
                workB[7][wsj[7]] = i;
                wsj[7]++;
            }
        }
        for (i = 0; i <= 7; i++)
            workB[i][wsj[i]] = "EOF";

        for (m = 0; m <= 7; m++)
            workB[m] = sort(workB[m]);

        for (i = 0; i < wsj[0]; i++) {
            z = workB[0][i];
            document.calcForm.A_head1.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[1]; i++) {
            z = workB[1][i];
            document.calcForm.A_head2.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[2]; i++) {
            z = workB[2][i];
            document.calcForm.A_head3.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[3]; i++) {
            z = workB[3][i];
            document.calcForm.A_left.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[4]; i++) {
            z = workB[4][i];
            document.calcForm.A_body.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[5]; i++) {
            z = workB[5][i];
            document.calcForm.A_shoulder.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[6]; i++) {
            z = workB[6][i];
            document.calcForm.A_shoes.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
        for (i = 0; i < wsj[7]; i++) {
            z = workB[7][i];
            document.calcForm.A_acces1.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
            document.calcForm.A_acces2.options[i] = new Option(ItemOBJ[z][8], ItemOBJ[z][0]);
        }
    }

    function JobEquipItemSearch(nJEIS) {
        if (isRebirth == 0) {
            if (ItemOBJ[i][11] == 200)
                return 0;
        }
        for (nJEISi = 0; JobEquipItemOBJ[n_A_JOB][nJEISi] != 999; nJEISi++) {
            if (JobEquipItemOBJ[n_A_JOB][nJEISi] == nJEIS)
                return 1;
        }
        return 0;
    }


    function n_A_JobSet() {
        n_A_JOB = eval(document.calcForm.A_JOB.value);
        if (21 <= n_A_JOB && n_A_JOB <= 40) {
            isRebirth = 1;
            if (34 <= n_A_JOB && n_A_JOB <= 40)
                n_A_JOB -= 34;
        } else
            isRebirth = 0;
    }


    function n_A_JobSearch() {

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


    function EquipNumSearch(nENS) {
        wENS = 0;
        for (ENSi = 0; ENSi <= 20; ENSi++) {
            if (nENS == n_A_Equip[ENSi])
                wENS += 1;
        }
        return wENS;
    }


    function CardNumSearch(nCNS) {
        wCNS = 0;
        for (CNSi = 0; CNSi <= 25; CNSi++) {
            if (nCNS == n_A_card[CNSi])
                wCNS += 1;
        }
        return wCNS;
    }


    w_ASSP0bk = new Array();
    for (i = 0; i < 20; i++)
        w_ASSP0bk[i] = 999;

    function ActiveSkillSetPlus() {
        w_ASSP0 = new Array();
        w_ASSP9 = new Array();
        for (i = 0; i < 20; i++) {
            w_ASSP0[i] = 999;
            w_ASSP9[i] = 0;
        }

        j = 0;
        for (i = 0; i <= 20; i++) {
            for (j2 = 0; ItemOBJ[n_A_Equip[i]][11 + j2] != 0; j2 += 2) {
                if (ItemOBJ[n_A_Equip[i]][11 + j2] == 220 || ItemOBJ[n_A_Equip[i]][11 + j2] == 221) {
                    w_ASSP0[j] = Math.floor((ItemOBJ[n_A_Equip[i]][12 + j2] % 100000) / 100);
                    w_ASSP9[j] = ItemOBJ[n_A_Equip[i]][12 + j2];
                    j++;
                }
            }
        }

        for (i = 0; i <= 25; i++) {
            for (j2 = 0; cardOBJ[n_A_card[i]][4 + j2] != 0; j2 += 2) {
                if (cardOBJ[n_A_card[i]][4 + j2] == 220 || cardOBJ[n_A_card[i]][4 + j2] == 221) {
                    w_ASSP0[j] = Math.floor((cardOBJ[n_A_card[i]][5 + j2] % 100000) / 100);
                    w_ASSP9[j] = cardOBJ[n_A_card[i]][5 + j2];
                    j++;
                }
            }
        }
        if (CardNumSearch(164) && (n_A_JOB == 9 || n_A_JOB == 23)) {
            w_ASSP0[j] = 162;
            w_ASSP9[j] = 19816205;
            j++;
        }
        if (CardNumSearch(277) && n_A_JobSearch() == 1) {
            w_ASSP0[j] = 76;
            w_ASSP9[j] = 19807605;
            j++;
        }

        w_ASSPch = 0;
        for (i = 0; i < 20; i++) {
            if (w_ASSP0bk[i] != w_ASSP0[i])
                w_ASSPch = 1
        }
        if (w_ASSPch) {

            for (k = 0; JobSkillActiveOBJ[n_A_JOB][k] != 999; k++) ;
            for (i = k + 20; i >= k; i--)
                document.calcForm.A_ActiveSkill.options[i] = null;
            j = 0;
            for (i = k; w_ASSP0[j] != 999; i++, j++) {
                if (w_ASSP9[j] < 200000)
                    document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2], w_ASSP9[j]);
                else
                    document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2] + "(Temp AS)", w_ASSP9[j]);
            }

        }
        for (i = 0; i < 20; i++)
            w_ASSP0bk[i] = w_ASSP0[i];

        if (eval(document.calcForm.A_ActiveSkill.value) == 0)
            document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
    }


    function KakutyouKansuu() {
        wKK = eval(document.calcForm.A_Kakutyou.value);
        if (wKK == 0) {
            myInnerHtml("A_KakutyouData", "", 0);
            return;
        }
        Heal = new Array();
        if (wKK == 1) {
            for (i = 0; i <= 10; i++)
                Heal[i] = HealCalc(i, 1);
            if (n_A_JOB == 3 || n_A_JOB == 9 || n_A_JOB == 13 || n_A_JOB == 14 || n_A_JOB == 15 || n_A_JOB == 20 || n_A_JOB == 23 || n_A_JOB == 27 || n_A_JOB == 28 || n_A_JOB == 29) {
                w = "";
                for (i = 1; i <= 9; i++)
                    w += "Lv" + i + " " + Heal[i] + "<br>";
                w += "Lv10 " + Heal[10] + "<br>";
            } else {
                w = "<table border=0>";
                w += "<tr><td>Heal Lv1 (Vitata Card) </td><td> " + Heal[1] + "</td></tr>";
                w += "<tr><td>Heal Lv2</td><td>" + Heal[2] + "</td></tr>";
                w += "<tr><td>Heal Lv3</td><td>" + Heal[3] + "</td></tr>";
                w += "<tr><td>Heal Lv4</td><td>" + Heal[4] + "</td></tr>";
                w += "<tr><td>Heal Lv5 (Scroll)</td><td>" + Heal[5] + "</td></tr></table>";
            }
            w += "<Font size=2>Required Int/Lv for next bonus: </Font>+" + (8 - (n_A_BaseLV + n_A_INT) % 8);
            myInnerHtml("A_KakutyouData", w, 0);
        } else if (wKK == 2) {
            if (n_A_JOB == 1 || n_A_JOB == 7 || n_A_JOB == 13 || n_A_JOB == 20 || n_A_JOB == 21 || n_A_JOB == 27) {
                HPRLV = eval(document.calcForm.A_KakutyouSelNum.value);
                w = Math.floor((5 + n_A_MaxHP / 500) * HPRLV);
                myInnerHtml("A_KakutyouData", "<br>Regen: " + w, 0);
            } else
                myInnerHtml("A_KakutyouData", "", 0);
        } else if (wKK == 3) {
            if (n_A_JOB == 5 || n_A_JOB == 9 || n_A_JOB == 11 || n_A_JOB == 18 || n_A_JOB == 20 || n_A_JOB == 23 || n_A_JOB == 25 || n_A_JOB == 32) {
                SPRLV = eval(document.calcForm.A_KakutyouSelNum.value);
                w = Math.floor((3 + n_A_MaxSP / 500) * SPRLV);
                myInnerHtml("A_KakutyouData", "<br>Regen: " + w, 0);
            } else
                myInnerHtml("A_KakutyouData", "", 0);
        } else if (wKK == 4) {
            if (n_A_JOB == 15 || n_A_JOB == 29) {
                SPRLV = eval(document.calcForm.A_KakutyouSelNum.value);
                w1 = Math.floor((4 + n_A_MaxHP / 500) * SPRLV);
                w2 = Math.floor((2 + n_A_MaxSP / 500) * SPRLV);
                myInnerHtml("A_KakutyouData", "<br>HP Regen: " + w1 + "<br>SP Regen: " + w2, 0);
            } else
                myInnerHtml("A_KakutyouData", "", 0);
        } else if (wKK == 5) {
            syozijob = [0, 800, 400, 400, 600, 200, 800, 800, 400, 600, 700, 400, 1000, 800, 400, 600, 700, 700, 400, 1000, 0, 800, 400, 600, 700, 400, 1000, 800, 400, 600, 700, 700, 400, 1000, 0, 0, 0, 0, 0, 0, 0, 800, 800, 400, 600, 800];
            syoziryou = 2000 + syozijob[n_A_JOB];
            if (eval(document.calcForm.isAdopted.checked))
                syoziryou = 2000;
            syoziryou += eval(document.calcForm.A_STR.value) * 30;
            if (SkillSearch(78))
                syoziryou += 1000;
            if (n_A_JOB == 6 || n_A_JOB == 12 || n_A_JOB == 19 || n_A_JOB == 20 || n_A_JOB == 26 || n_A_JOB == 33)
                syoziryou += eval(document.calcForm.A_KakutyouSelNum.value) * 200;
            EquipKG = 0;
            for (i = 0; i <= 10; i++)
                EquipKG += ItemOBJ[n_A_Equip[i]][6];
            myInnerHtml("A_KakutyouData", "Weight Limit: " + syoziryou + "<BR>Total Weight of Equipment: " + EquipKG, 0);
        }
    }

    function KakutyouKansuu2() {
        wKK = eval(document.calcForm.A_Kakutyou.value);
        if (wKK == 2) {
            if (n_A_JOB == 1 || n_A_JOB == 7 || n_A_JOB == 13 || n_A_JOB == 20 || n_A_JOB == 21 || n_A_JOB == 27) {
                myInnerHtml("A_KakutyouSel", "Increased HP Recovery Level: " + '<select name="A_KakutyouSelNum"onChange="StAllCalc()"></select>', 0);
                for (i = 0; i <= 10; i++)
                    document.calcForm.A_KakutyouSelNum.options[i] = new Option(i, i);
                document.calcForm.A_KakutyouSelNum.value = 10;
                return;
            } else {
                myInnerHtml("A_KakutyouSel", "Not Available for this Class", 0);
                return;
            }
        }
        if (wKK == 3) {
            if (n_A_JOB == 5 || n_A_JOB == 9 || n_A_JOB == 11 || n_A_JOB == 18 || n_A_JOB == 20 || n_A_JOB == 23 || n_A_JOB == 25 || n_A_JOB == 32) {
                myInnerHtml("A_KakutyouSel", "Increased SP Recovery Level: " + '<select name="A_KakutyouSelNum"onChange="StAllCalc()"></select>', 0);
                for (i = 0; i <= 10; i++)
                    document.calcForm.A_KakutyouSelNum.options[i] = new Option(i, i);
                document.calcForm.A_KakutyouSelNum.value = 10;
                return;
            } else {
                myInnerHtml("A_KakutyouSel", "Not Available for this Class", 0);
                return;
            }
        }
        if (wKK == 4) {
            if (n_A_JOB == 15 || n_A_JOB == 29) {
                myInnerHtml("A_KakutyouSel", "Spiritual Cadence Lv: " + '<select name="A_KakutyouSelNum"onChange="StAllCalc()"></select>', 0);
                for (i = 0; i <= 5; i++)
                    document.calcForm.A_KakutyouSelNum.options[i] = new Option(i, i);
                document.calcForm.A_KakutyouSelNum.value = 5;
                return;
            } else {
                myInnerHtml("A_KakutyouSel", "Not Available for this Class", 0);
                return;
            }
        }
        if (wKK == 5) {
            if (n_A_JOB == 6 || n_A_JOB == 12 || n_A_JOB == 19 || n_A_JOB == 20 || n_A_JOB == 26 || n_A_JOB == 33) {
                myInnerHtml("A_KakutyouSel", "Enlarge Weight Limit Lv: " + '<select name="A_KakutyouSelNum"onChange="StAllCalc()"></select><BR>', 0);
                for (i = 0; i <= 10; i++)
                    document.calcForm.A_KakutyouSelNum.options[i] = new Option(i, i);
                if (n_A_JOB == 20)
                    document.calcForm.A_KakutyouSelNum.value = 0;
                else
                    document.calcForm.A_KakutyouSelNum.value = 5;
            } else {
                myInnerHtml("A_KakutyouSel", "", 0);
            }
            return;
        }
        myInnerHtml("A_KakutyouSel", "", 0);
    }

    function SetCardShort() {
        w = eval(document.calcForm.A_cardshort.value);
        if (CardShort[w][1] < 10000) {
            document.calcForm.A_weapon1_card1.value = CardShort[w][1];
            document.calcForm.A_weapon1_card2.value = CardShort[w][2];
            document.calcForm.A_weapon1_card3.value = CardShort[w][3];
            document.calcForm.A_weapon1_card4.value = CardShort[w][4];

            if (w == 9 || w == 10) {
                w = MonsterOBJ[eval(document.calcForm.B_Enemy.value)][3];

                if (10 <= w && w <= 14)
                    document.calcForm.A_weapon1_card1.value = 204;
                if ((20 <= w && w <= 24) || (80 <= w && w <= 94))
                    document.calcForm.A_weapon1_card1.value = 203;
                if (30 <= w && w <= 34)
                    document.calcForm.A_weapon1_card1.value = 201;
                if (40 <= w && w <= 44)
                    document.calcForm.A_weapon1_card1.value = 202;
            }
        } else {
            if (CardShort[w][2] != 0)
                document.calcForm.A_weapon1_card1.value = CardShort[w][2];
            if (CardShort[w][3] != 0)
                document.calcForm.A_head1_card.value = CardShort[w][3];
            if (CardShort[w][4] != 0)
                document.calcForm.A_left_card.value = CardShort[w][4];
            if (CardShort[w][5] != 0)
                document.calcForm.A_body_card.value = CardShort[w][5];
            if (CardShort[w][6] != 0)
                document.calcForm.A_shoulder_card.value = CardShort[w][6];
            if (CardShort[w][7] != 0)
                document.calcForm.A_shoes_card.value = CardShort[w][7];
            if (CardShort[w][8] != 0)
                document.calcForm.A_acces1_card.value = CardShort[w][8];
            if (CardShort[w][9] != 0)
                document.calcForm.A_acces2_card.value = CardShort[w][9];
        }
        ActiveSkillSetPlus();
    }

    function SetCardShortLeft() {
        w = eval(document.calcForm.A_cardshortLeft.value);

        document.calcForm.A_weapon2_card1.value = CardShort[w][1];
        document.calcForm.A_weapon2_card2.value = CardShort[w][2];
        document.calcForm.A_weapon2_card3.value = CardShort[w][3];
        document.calcForm.A_weapon2_card4.value = CardShort[w][4];


        if (w == 9 || w == 10) {
            w = MonsterOBJ[eval(document.calcForm.B_Enemy.value)][3];

            if (10 <= w && w <= 14)
                document.calcForm.A_weapon2_card1.value = 204;
            if ((20 <= w && w <= 24) || (80 <= w && w <= 94))
                document.calcForm.A_weapon2_card1.value = 203;
            if (30 <= w && w <= 34)
                document.calcForm.A_weapon2_card1.value = 201;
            if (40 <= w && w <= 44)
                document.calcForm.A_weapon2_card1.value = 202;
        }
    }

    wESx = new Array();
    for (i = 0; i <= EnemyNum; i++)
        wESx[i] = new Array();

    function EnemySort() {
        ESNum = [1, 3, 2, 21, 22, 16, 17, 13, 100];

        wES2 = eval(document.calcForm.ENEMY_SORT.value)

        if (20 <= wES2 && wES2 <= 99 || wES2 == 0) {
            for (i = EnemyNum; i >= 0; i--)
                document.calcForm.B_Enemy.options[i] = null;
            for (i = 0; ESortStr[wES2][i] != "N"; i++)
                document.calcForm.B_Enemy.options[i] = new Option(MonsterOBJ[ESortStr[wES2][i]][1], MonsterOBJ[ESortStr[wES2][i]][0]);
            return;
        }

        wES = ESNum[eval(document.calcForm.ENEMY_SORT.value)];
        wESx[0][0] = "S";
        wESx[0][1] = "E";
        STERTw = 0;
        ENDw = 0;
        for (i = 1; i <= EnemyNum; i++) {
            j = ENDw;
            if (MonsterOBJ[i][wES] >= MonsterOBJ[j][wES]) {
                wESx[j][1] = i;
                wESx[i][0] = j;
                wESx[i][1] = "E";
                ENDw = i;
            } else {
                j = STERTw;
                if (MonsterOBJ[i][wES] <= MonsterOBJ[j][wES]) {
                    wESx[j][0] = i;
                    wESx[i][0] = "S";
                    wESx[i][1] = j;
                    STERTw = i;
                } else {
                    j = STERTw;
                    jbk = STERTw;
                    while (MonsterOBJ[i][wES] > MonsterOBJ[j][wES]) {
                        jbk = j;
                        j = wESx[j][1];
                    }
                    wESx[jbk][1] = i;
                    wESx[i][0] = jbk;
                    wESx[i][1] = j;
                    wESx[j][0] = i;
                }
            }
        }

        ESwork2 = new Array();
        if (wES == 21 || wES == 22) {
            for (i = 0; i <= EnemyNum; i++)
                ESwork2[i] = MonsterOBJ[i][wES] + ") ";
        } else if (wES == 2) {
            for (i = 0; i <= EnemyNum; i++)
                ESwork2[i] = SyuzokuOBJ[MonsterOBJ[i][2]] + ") ";
        } else if (wES == 3) {
            for (i = 0; i <= EnemyNum; i++)
                ESwork2[i] = elementOBJ[Math.floor(MonsterOBJ[i][3] / 10)] + " " + MonsterOBJ[i][3] % 10 + ") ";
        } else {
            for (i = 0; i <= EnemyNum; i++)
                ESwork2[i] = "";
        }

        document.calcForm.B_Enemy.options[0] = new Option(ESwork2[STERTw] + MonsterOBJ[STERTw][1], MonsterOBJ[STERTw][0]);
        i = STERTw;
        for (j = 1; wESx[i][1] != "E"; j++) {
            document.calcForm.B_Enemy.options[j] = new Option(ESwork2[wESx[i][1]] + MonsterOBJ[wESx[i][1]][1], MonsterOBJ[wESx[i][1]][0]);
            i = wESx[i][1];
        }
    }

    function serializeFormToJSON() {
        var form = document.calcForm;
        var formData = new FormData(form);
        var formObject = {};

        formData.forEach(function (value, key) {
            // Check if property already exists for multi-select or checkboxes
            if (formObject.hasOwnProperty(key)) {
                // If it's an array, push the value
                if (Array.isArray(formObject[key])) {
                    formObject[key].push(value);
                } else { // Convert to array if second occurrence of the key
                    formObject[key] = [formObject[key], value];
                }
            } else {
                formObject[key] = value;
            }
        });

        return JSON.stringify(formObject);
    }

    function repopulateFormFromJSON(jsonData) {
        var form = document.calcForm;
        var formData = jsonData;

        Object.keys(formData).forEach(function (key) {
            var element = form.elements[key];
            if (!element) return; // Skip if element not found

            var value = formData[key];

            // Handle different form element types separately
            switch (element.type) {
                case 'checkbox':
                    if (Array.isArray(value)) {
                        // For array values (multiple checkboxes with same name)
                        value.forEach(val => {
                            if (element.value === val) {
                                element.checked = true;
                            }
                        });
                    } else {
                        element.checked = element.value === value;
                    }
                    break;
                case 'radio':
                    element.checked = element.value === value;
                    break;
                case 'select-multiple':
                    Array.from(element.options).forEach(option => {
                        option.selected = value.includes(option.value);
                    });
                    break;
                default:
                    element.value = value;
            }
        });
    }
    function removeNullValues(obj) {
        for (let key in obj) {
            if (obj[key] === null ||Number.isNaN(obj[key]) || obj[key] === 0) {
                delete obj[key];
            } else if (typeof obj[key] === 'object') {
                removeNullValues(obj[key]);
            }
        }
    }
    function GenerateTestCase() {
        calc();
        let savedDataAsJson = SaveCookie(true);
        let crit_damages = document.querySelector("#CRIATK").textContent.split("~");
        let crit_rate = Number.parseFloat(document.querySelector("#CRInum").textContent);
        let min_dmg = Number.parseFloat(document.querySelector("#ATK_00").textContent);
        let max_dmg = Number.parseFloat(document.querySelector("#ATK_02").textContent);
        let avg_dmg = Number.parseFloat(document.querySelector("#ATK_01").textContent);
        let dps = Number.parseFloat(document.querySelector("#AveSecondATK").textContent);
        let aspd = Number.parseFloat(document.querySelector("#nm023").textContent);
        savedDataAsJson.expected = {
            weapon_min_atk: weaponAttack[0],
            weapon_avg_atk: weaponAttack[1],
            weapon_max_atk: weaponAttack[2],
            base_atk: baseATK,
            hit_ratio: w_HIT / 100.0,
            critical_rate: crit_rate,
            critical_damage_min: Number.parseFloat(crit_damages[0]),
            critical_damage_max: crit_damages.length > 1 ? Number.parseFloat(crit_damages[1]) : Number.parseFloat(crit_damages[0]),
            min_dmg: min_dmg,
            avg_dmg: avg_dmg,
            max_dmg: max_dmg,
            dps: dps,
            aspd: aspd,
            stats_atk_left: ATK_LEFT,
            stats_atk_right: ATK_RIGHT,
        };
        removeNullValues(savedDataAsJson);
        console.log(JSON.stringify(savedDataAsJson));
        navigator.clipboard.writeText(JSON.stringify(savedDataAsJson));
    }

    function aegis_item(value) {
        if (ItemIds[value][2].startsWith("(No")) {
            return null;
        }
        return ItemIds[value][2];
    }

    function card(value) {
        if (!value) {
            return null;
        }
        return CardIds[value][2]
    }

    function SaveCookie(skipSave) {
        const testCaseData = {};

        testCaseData.job = JobName[eval(document.calcForm.A_JOB.value)];
        testCaseData.base_level = eval(document.calcForm.A_BaseLV.value);
        testCaseData.job_level = eval(document.calcForm.A_JobLV.value);
        testCaseData.str = eval(document.calcForm.A_STR.value);
        testCaseData.agi = eval(document.calcForm.A_AGI.value);
        testCaseData.vit = eval(document.calcForm.A_VIT.value);
        testCaseData.dex = eval(document.calcForm.A_DEX.value);
        testCaseData.int = eval(document.calcForm.A_INT.value);
        testCaseData.luk = eval(document.calcForm.A_LUK.value);

        if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n_A_WeaponType != 0)) {
            testCaseData.ammo = eval(document.calcForm.A_Arrow.value);
        }

        testCaseData.speed_potion = eval(document.calcForm.A_SpeedPOT.value);
        testCaseData.weapon = aegis_item(eval(document.calcForm.A_weapon1.value));
        testCaseData.weapon_refinement = eval(document.calcForm.A_Weapon_ATKplus.value);
        testCaseData.weapon_card1 = card(eval(document.calcForm.A_weapon1_card1.value));
        testCaseData.weapon_card2 = card(eval(document.calcForm.A_weapon1_card2.value));
        testCaseData.weapon_card3 = card(eval(document.calcForm.A_weapon1_card3.value));
        testCaseData.weapon_card4 = card(eval(document.calcForm.A_weapon1_card4.value));

        if (document.calcForm.A_weapon2) {
            testCaseData.weapon_left = aegis_item(eval(document.calcForm.A_weapon2.value));
            testCaseData.weapon_left_refinement = eval(document.calcForm.A_Weapon2_ATKplus.value);
            testCaseData.weapon_left_card1 = card(eval(document.calcForm.A_weapon2_card1.value));
            testCaseData.weapon_left_card2 = card(eval(document.calcForm.A_weapon2_card2.value));
            testCaseData.weapon_left_card3 = card(eval(document.calcForm.A_weapon2_card3.value));
            testCaseData.weapon_left_card4 = card(eval(document.calcForm.A_weapon2_card4.value));
        }

        if (document.calcForm.A_Arrow && document.calcForm.A_Arrow.style["visibility"] !== "hidden") {
            testCaseData.ammo = ArrowOBJ[document.calcForm.A_Arrow.value][2];
        }


        testCaseData.headgear_upper = aegis_item(eval(document.calcForm.A_head1.value));
        testCaseData.headgear_upper_card = card(eval(document.calcForm.A_head1_card.value));
        testCaseData.headgear_middle = aegis_item(eval(document.calcForm.A_head2.value));
        testCaseData.headgear_middle_card = card(eval(document.calcForm.A_head2_card.value));
        testCaseData.headgear_lower = aegis_item(eval(document.calcForm.A_head3.value))

        if (document.calcForm.A_left.value !== "305") {
            testCaseData.shield = aegis_item(eval(document.calcForm.A_left.value));
        }
        testCaseData.shield_card = card(eval(document.calcForm.A_left_card.value));
        testCaseData.body = aegis_item(eval(document.calcForm.A_body.value));
        testCaseData.body_card = card(eval(document.calcForm.A_body_card.value));
        testCaseData.shoulder = aegis_item(eval(document.calcForm.A_shoulder.value));
        testCaseData.shoulder_card = card(eval(document.calcForm.A_shoulder_card.value));
        testCaseData.shoes = aegis_item(eval(document.calcForm.A_shoes.value));
        testCaseData.shoes_card = card(eval(document.calcForm.A_shoes_card.value));
        testCaseData.accessory_left = aegis_item(eval(document.calcForm.A_acces1.value));
        testCaseData.accessory_left_card = card(eval(document.calcForm.A_acces1_card.value));
        testCaseData.accessory_right = aegis_item(eval(document.calcForm.A_acces2.value));
        testCaseData.accessory_right_card = card(eval(document.calcForm.A_acces2_card.value));

        n_A_JobSet();
        w = n_A_JOB;
        var saveDataIndex = 45;
        var passiveSkills = [];
        for (var i = 0; i < 15; i++) {
            if (JobSkillPassOBJ[w][i] == 999) break;
            let skill_level = eval(document.calcForm["A_skill" + i].value);
            SaveData[saveDataIndex + i] = skill_level;
            if (skill_level > 0) {
                passiveSkills.push({skid: SkillOBJ[JobSkillPassOBJ[w][i]][3], level: skill_level})
            }
        }
        testCaseData.passiveSkills = passiveSkills;
        testCaseData.weapon_element = eval(document.calcForm.A_Weapon_element.value);

        const supportiveSkillsIds = [
            {skid: 34}, {skid: 29}, {skid: 66}, {skid: 75}, {skid: 33}, {skid: 361}, {skid: 111}, {skid: 112},
            {skid: 486}, {skid: 383}, {state: 'Spirit Sphere'}, {skid: 7}, {state: 'Aloevera'}, {skid: 67}, {skid: 256}];
        var supportiveSkills = [];
        for (i = 0; i <= 12; i++) {
            if (n_A_PassSkill2[i] === undefined || n_A_PassSkill2[i] === 0) {
                continue;
            }
            var value = n_A_PassSkill2[i];
            if (value == true)
                value = 1;
            else if (value == false)
                value = 0;
            if (value > 0){
                supportiveSkills.push({...supportiveSkillsIds[i], value})
            }
        }
        testCaseData.supportiveSkills = supportiveSkills;

        testCaseData.headgear_upper_refinement = eval(document.calcForm.A_HEAD_DEF_PLUS.value);
        testCaseData.body_refinement = eval(document.calcForm.A_BODY_DEF_PLUS.value);
        testCaseData.shield_refinement = eval(document.calcForm.A_LEFT_DEF_PLUS.value);
        testCaseData.shoulder_refinement = eval(document.calcForm.A_SHOULDER_DEF_PLUS.value);
        testCaseData.shoes_refinement = eval(document.calcForm.A_SHOES_DEF_PLUS.value);
        testCaseData.skill_to_use = {
            skid: SkillOBJ[eval(document.calcForm.A_ActiveSkill.value)][3],
            level: eval(document.calcForm.A_ActiveSkillLV.value)
        };
        testCaseData.target = MonsterIds[eval(document.calcForm.B_Enemy.value)][2];

        if (!skipSave) {
            cookieNum = document.calcForm.A_SaveSlot.value;

            bkcN = cookieNum;
            LoadSave();
            document.calcForm.A_SaveSlot.value = bkcN;
            localStorage.setItem(bkcN, serializeFormToJSON())
            console.log(serializeFormToJSON())
        } else {
            return testCaseData;
        }
    }


    function LoadCookie() {

        SaveData = new Array();
        cookieNum = document.calcForm.A_SaveSlot.value;
        SaveData = document.cookie.split("; ");
        wStr = "";
        let json = JSON.parse(localStorage.getItem(cookieNum));
        document.calcForm.A_JOB.value = json.A_JOB;
        ClickJob(json.A_JOB);
        if (json.A2_SKILLSW === "on") {
            document.calcForm.A2_SKILLSW.checked = true;
            Click_SkillSW();
        }
        repopulateFormFromJSON(json);
        ClickWeaponType(json.A_WeaponType);
        if (json.A_Weapon2Type) {
            document.calcForm.A_Weapon2Type.value = json.A_Weapon2Type
            ClickWeaponType2(json.A_Weapon2Type);
        }
        document.calcForm.A_weapon1.value = json.A_weapon1;
        ClickB_Item(json.A_weapon1);
        n_A_JobSet();
        ClickActiveSkill(json.A_ActiveSkill);


        StCalc(1);
        StAllCalc();
        ActiveSkillSetPlus();
    }


    function LoadSave() {

        SaveData = new Array();
        for (k = 1; k <= 19; k++) {
            cookieNum = "num0" + (k - 1);
            if (k == 9)
                cookieNum = "num0" + k;
            if (k >= 10)
                cookieNum = "num" + k;
            let json = JSON.parse(localStorage.getItem(cookieNum));

            if (json) {
                document.calcForm.A_SaveSlot.options[k - 1] = new Option("Save" + k + ": " + JobName[json.A_JOB], cookieNum);
            } else
                document.calcForm.A_SaveSlot.options[k - 1] = new Option("Save" + k + ": no Save Data", cookieNum);
        }
    }

    JobName =
        ["Novice", "Swordsman", "Thief", "Acolyte", "Archer", "Magician", "Merchant", "Knight", "Assassin", "Priest", "Hunter", "Wizard", "Blacksmith", "Crusader", "Rogue", "Monk", "Bard", "Dancer", "Sage", "Alchemist",
            "Super Novice", "LordKnight", "AssassinCross", "HighPriest", "Sniper", "HighWizard", "Whitesmith", "Paladin", "Stalker", "Champion", "Clown", "Gypsy", "Professor", "Creator",
            "High Novice", "High Swordsman", "High Thief", "High Acolyte", "High Archer", "High Magician", "High Merchant", "Taekwon Kid", "Taekwon Master", "Soul Linker", "Ninja", "Gunslinger"];


    for (i = 0; i <= 45; i++)
        document.calcForm.A_JOB.options[i] = new Option(JobName[i], i);

    SpeedPotName = ["None", "Concentration Potion", "Awakening Potion", "Berserk Potion"];


    document.calcForm.A_SpeedPOT.options[0] = new Option(SpeedPotName[0], 0);
    document.calcForm.A_SpeedPOT.options[1] = new Option(SpeedPotName[1], 1);
    document.calcForm.A_SpeedPOT.options[2] = new Option(SpeedPotName[2], 2);
    document.calcForm.A_SpeedPOT.options[3] = new Option(SpeedPotName[3], 3);


    for (i = 0; i <= 16; i++)
        document.calcForm.A_Arrow.options[i] = new Option(ArrowOBJ[i][2], i);

    EnName = ["Neutral", "Water (Sage)", "Earth (Sage)", "Fire (Sage)", "Wind (Sage)", "Poison (EP)", "Holy (Asp)", "Dark", "Ghost", "Undead"];
    for (i = 0; i <= 9; i++)
        document.calcForm.A_Weapon_element.options[i] = new Option(EnName[i], i);


    CardShort = [
        ["Card Shortcuts", 0, 0, 0, 0],
        ["Remove All", 0, 0, 0, 0],
        ["+20%", 1, 0, 0, 0],
        ["+40%", 1, 1, 0, 0],
        ["+60%", 1, 1, 1, 0],
        ["+80%", 1, 1, 1, 1],
        ["Size Type 2x", 3, 3, 0, 0],
        ["Size Type 3x", 3, 3, 3, 0],
        ["Size Type 4x", 3, 3, 3, 3],
        ["Elemental + Star Crumb", 0, 106, 0, 0],
        ["Elemental + Star Crumb 2x", 0, 106, 106, 0],
        ["Star Crumb 3x", 106, 106, 106, 0],
        ["Andre 2x", 11, 11, 0, 0],
        ["Andre 3x", 11, 11, 11, 0],
        ["Andre 4x", 11, 11, 11, 11],
        ["Soldier Skel 2x", 41, 41, 0, 0],
        ["Soldier Skel 3x", 41, 41, 41, 0],
        ["Soldier Skel 4x", 41, 41, 41, 41],
        ["Mummy 2x", 40, 40, 0, 0],
        ["Mummy 3x", 40, 40, 40, 0],
        ["Mummy 4x", 40, 40, 40, 40],
        ["+44%", 1, 2, 0, 0],
        ["+68%", 1, 1, 2, 0],
        ["+96%", 1, 1, 2, 2],
        ["Orc Lady 2x", 252, 252, 0, 0],
        ["Orc Lady 3x", 252, 252, 13, 0],
        ["Orc Lady 4x", 252, 252, 252, 13],
        ["Archer Skel 2x", 107, 107, 0, 0],
        ["Archer Skel 3x", 107, 107, 107, 0],
        ["Archer Skel 4x", 107, 107, 107, 107],
        ["Abysmal Knight 2x", 31, 31, 0, 0],
        ["Abysmal Knight 3x", 31, 31, 31, 0],
        ["Abysmal Knight 4x", 31, 31, 31, 31],
        ["Swordsman Set", 10000, 223, 347, 0, 317, 0, 362, 354, 0],
        ["Thief Set", 10000, 233, 0, 0, 0, 295, 391, 395, 260],
        ["Priest Set", 10000, 253, 383, 307, 301, 0, 0, 270, 0],
        ["Archer Set", 10000, 279, 0, 0, 224, 340, 351, 230, 0],
        ["Mage Set", 10000, 0, 337, 358, 220, 346, 379, 350, 0],
        ["Merchant Set", 10000, 326, 376, 0, 281, 0, 388, 216, 0],
        ["test(N/A)", 0, 0, 0, 0],
    ];
    for (i = 0; i <= 38; i++)
        document.calcForm.A_cardshort.options[i] = new Option(CardShort[i][0], i);


    n_A_PassSkill2 = new Array();
    for (i = 0; i <= 15; i++)
        n_A_PassSkill2[i] = 0;


    n_A_PassSkill3 = new Array();
    for (i = 0; i <= 45; i++)
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


    n_A_PassSkill5 = new Array();
    for (i = 0; i <= 4; i++)
        n_A_PassSkill5[i] = 0;


    n_A_PassSkill6 = new Array();
    for (i = 0; i <= 3; i++)
        n_A_PassSkill6[i] = 0;


    n_A_PassSkill7 = new Array();
    for (i = 0; i <= 10; i++)
        n_A_PassSkill7[i] = 0;


    n_B_IJYOU = new Array();
    for (i = 0; i <= 27; i++)
        n_B_IJYOU[i] = 0;


    n_A_Equip = new Array();
    for (i = 0; i <= 20; i++)
        n_A_Equip[i] = 0;

    n_A_card = new Array();
    for (i = 0; i <= 25; i++)
        n_A_card[i] = 0;


    tPlusTaiseiSyokia();


    document.calcForm.A_JOB.value = 0;
    ClickJob(0);
    EnemySort();
    StCalc();
    calc();
    LoadSave();


}
/*
     FILE ARCHIVED ON 14:38:40 Nov 11, 2008 AND RETRIEVED FROM THE
     INTERNET ARCHIVE ON 07:19:10 Nov 20, 2023.
     JAVASCRIPT APPENDED BY WAYBACK MACHINE, COPYRIGHT INTERNET ARCHIVE.

     ALL OTHER CONTENT MAY ALSO BE PROTECTED BY COPYRIGHT (17 U.S.C.
     SECTION 108(a)(3)).
*/
/*
playback timings (ms):
  captures_list: 78.347
  exclusion.robots: 0.067
  exclusion.robots.policy: 0.058
  cdx.remote: 0.056
  esindex: 0.009
  LoadShardBlock: 47.143 (3)
  PetaboxLoader3.datanode: 89.799 (4)
  load_resource: 716.507
  PetaboxLoader3.resolve: 39.886
*/