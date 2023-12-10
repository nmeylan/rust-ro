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
    return n_A_WeaponType !=  WEAPON_TYPE_BOW && n_A_WeaponType !=  WEAPON_TYPE_INSTRUMENT && n_A_WeaponType !=  WEAPON_TYPE_WHIP && n_A_WeaponType !=  WEAPON_TYPE_HANDGUN && n_A_WeaponType !=  WEAPON_TYPE_RIFLE && n_A_WeaponType !=  WEAPON_TYPE_SHOTGUN && n_A_WeaponType !=  WEAPON_TYPE_GATLING_GUN && n_A_WeaponType !=  WEAPON_TYPE_GRENADE_LAUNCHER;
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
            let itemName = ItemOBJ[n_A_Equip[ENSi]][8];
            if (nENS === itemName)
                wENS += 1;
        }
        return wENS;
    }


    function CardNumSearch(nCNS) {
        wCNS = 0;
        for (CNSi = 0; CNSi <= 25; CNSi++) {
            let cardName = cardOBJ[n_A_card[CNSi]][2];
            if (nCNS === cardName)
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
                ESwork2[i] = RaceOBJ[MonsterOBJ[i][2]] + ") ";
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
            if (obj[key] === null || Number.isNaN(obj[key]) || obj[key] === 0) {
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
        let dps = Number.parseFloat(document.querySelector("#DPS").textContent);
        let aspd = Number.parseFloat(document.querySelector("#nm023").textContent);
        savedDataAsJson.expected = {
            weapon_min_atk: weaponAttack[0],
            weapon_avg_atk: weaponAttack[1],
            weapon_max_atk: weaponAttack[2],
            base_atk: baseATK,
            hit_ratio: hitRate / 100.0,
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

        if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n_A_WeaponType !=  WEAPON_TYPE_UNARMED)) {
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
            if (value > 0) {
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

// 0: Provoke (Non Undead)
// 1: Quagmire
// 2: Poison
// 3: Blind
// 4: Frozen (Non Undead)
// 5: Blessing (Demon/Undead)
// 6: Lex Aeterna
// 7: Stun
// 8: Sleep
// 9: Stone
// 10: Curse
// 11: Agility Down
// 12: Signum Crucis
// 13: Divest Weapon
// 14: Divest Shield
// 15: Divest Armor
// 16: Divest Helm
// 17: Fiber Lock
// 18: Mind Breaker
// 19: Slow Grace
// 20: Down Tempo
// 21: Power Up
// 22: Agility Up
// 23: Eska
// 24: Eske
// 25: Change Element (Monster Skill)
// 26: Elemental Change (Sage Skill)
// 27: Flying
    TargetStatusFlags = new Array();
    for (i = 0; i <= 27; i++)
        TargetStatusFlags[i] = 0;


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