function isRangedWeapon() {
    return n_A_WeaponType ==  WEAPON_TYPE_BOW || n_A_WeaponType ==  WEAPON_TYPE_HANDGUN || n_A_WeaponType ==  WEAPON_TYPE_RIFLE || n_A_WeaponType ==  WEAPON_TYPE_SHOTGUN || n_A_WeaponType ==  WEAPON_TYPE_GATLING_GUN || n_A_WeaponType ==  WEAPON_TYPE_GRENADE_LAUNCHER;
}

{

    hasLeftHand = 0;
    isRebirth = 0;
    n_SpSkill = 0;
    n_Ses = 0;
    n_Enekyori = 0;
    w_AG = [100, 95, 90, 86, 82, 79, 76, 74, 72, 71, 70];
    n_SkillSW = 0;
    n_Skill3SW = 0;
    n_Skill4SW = 0;
    n_Skill5SW = 0;
    n_Skill6SW = 0;
    n_Skill7SW = 0;
    MonsterStats = 0;
    wBCEDPch = 0;
    wLAch = 0;
    wCriTyuu = 0;
    wBTw1 = 0;
    swDelay = 0;
    TyouEnkakuSousa3dan = 0;
    not_use_card = 0;
    SuperNoviceFullWeaponCHECK = 0;
    cast_kotei = 0;
    b = 0;


    function myInnerHtml(wIH1, wIH2, wIH3) {
        if (InTestCaseGenerationMode == true && typeof wIH2 === 'string' && !(wIH2.startsWith("<select") || wIH2.startsWith("<input"))) {
            return;
        }
        if (wIH3 == 0) {
            wIHOB = document.getElementById(wIH1);
            while (wIHOB.hasChildNodes()) {
                wIHOB.removeChild(wIHOB.firstChild);
            }
            wIHOB.innerHTML = wIH2;
        } else {
            wIHOB = document.getElementById(wIH1);
            wIHOB.insertAdjacentHTML('BeforeEnd', wIH2);

        }
    }
    WEAPON_TYPE_UNARMED = 0;
    WEAPON_TYPE_DAGGER = 1;
    WEAPON_TYPE_SWORD = 2;
    WEAPON_TYPE_TWO_HANDED_SWORD = 3;
    WEAPON_TYPE_SPEAR = 4;
    WEAPON_TYPE_TWO_HANDED_SPEAR = 5;
    WEAPON_TYPE_AXE = 6;
    WEAPON_TYPE_TWO_HANDED_AXE = 7;
    WEAPON_TYPE_MACE = 8;
    WEAPON_TYPE_ROD = 9;
    WEAPON_TYPE_BOW = 10;
    WEAPON_TYPE_KATAR = 11;
    WEAPON_TYPE_BOOK = 12;
    WEAPON_TYPE_KNUCKLE = 13;
    WEAPON_TYPE_INSTRUMENT = 14;
    WEAPON_TYPE_WHIP = 15;
    WEAPON_TYPE_HUUMA_SHURIKEN = 16;
    WEAPON_TYPE_HANDGUN = 17;
    WEAPON_TYPE_RIFLE = 18;
    WEAPON_TYPE_SHOTGUN = 19;
    WEAPON_TYPE_GATLING_GUN = 20;
    WEAPON_TYPE_GRENADE_LAUNCHER = 21;
    WeaponName = [
        "Unarmed", 
        "Dagger", 
        "Sword", 
        "Two-handed Sword", 
        "Spear", 
        "Two-handed Spear",
        "Axe", 
        "Two-handed Axe", 
        "Mace", 
        "Rod", 
        "Bow", //10
        "Katar", 
        "Book", 
        "Knuckle", 
        "Instrument", 
        "Whip", 
        "Huuma Shuriken", 
        "Handgun",//17 
        "Rifle", 
        "Shotgun", 
        "Gatling Gun", 
        "Grenade Launcher"];

    ArrowOBJ = [
        [25, 0, "Arrow"],
        [30, 6, "Silver Arrow"],
        [30, 3, "Fire Arrow"],
        [30, 0, "Iron Arrow"],
        [30, 2, "Stone Arrow"],
        [30, 1, "Crystal Arrow"],
        [30, 4, "Arrow of Wind"],
        [30, 7, "Arrow of Shadow"],
        [30, 8, "Immaterial Arrow"],
        [30, 5, "Rusty Arrow"],
        [40, 0, "Steel Arrow"],
        [50, 0, "Oridecon Arrow"],
        [50, 6, "Arrow of Counter Evil"],
        [1, 1, "Frozen Arrow"],
        [1, 5, "Poison Arrow"],
        [10, 0, "Sharp Arrow"],
        [50, 6, "Holy Arrow"],
        [1, 0, "Other (1 Atk)"]
    ];

    ArrowOBJbackup = [
        [25, 0, "Arrow"],
        [30, 6, "Silver Arrow"],
        [30, 3, "Fire Arrow"],
        [30, 0, "Iron Arrow"],
        [30, 2, "Stone Arrow"],
    ];

    BulletOBJ = [
        [10, 0, "Bullet"],
        [15, 6, "Silver Bullet"],
        [30, 0, "Blood Bullet"],
    ];

    GrenadeOBJ = [
        [50, 3, "Flare Sphere"],
        [50, 1, "Freezing Sphere"],
        [50, 4, "Lightning Sphere"],
        [50, 7, "Blind Sphere"],
        [50, 5, "Poison Sphere"],
    ];

    SyurikenOBJ = [
        [10, 0, "Shuriken"],
        [30, 0, "Numbus Shuriken"],
        [45, 0, "Flash Shuriken"],
        [70, 0, "Sharp Leaf Shuriken"],
        [100, 0, "Thorn Needle Shuriken"],
    ];

    KunaiOBJ = [
        [30, 3, "Heat Wave Kunai"],
        [30, 1, "Icicle Kunai"],
        [30, 4, "High Wind Kunai"],
        [30, 2, "Black Earth Kunai"],
        [30, 5, "Fell Poison Kunai"],
    ];


    JobEquipItemOBJ = [
        [0, 50, 100, 999],
        [0, 1, 51, 101, 70, 71, 72, 74, 75, 78, 83, 84, 85, 86, 87, 999],
        [0, 1, 52, 102, 72, 74, 75, 78, 80, 83, 84, 85, 999],
        [0, 1, 53, 103, 71, 73, 74, 77, 78, 85, 89, 999],
        [0, 1, 54, 104, 75, 76, 83, 89, 999],
        [0, 1, 55, 105, 71, 77, 89, 999],
        [0, 1, 56, 106, 70, 71, 72, 73, 74, 75, 78, 83, 84, 85, 86, 999],
        [0, 1, 51, 61, 107, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 999],
        [0, 1, 52, 62, 108, 72, 74, 75, 78, 79, 81, 83, 84, 85, 999],
        [0, 1, 53, 63, 109, 71, 73, 74, 77, 78, 79, 81, 85, 89, 999],
        [0, 1, 54, 64, 110, 75, 76, 79, 80, 83, 88, 89, 999],
        [0, 1, 55, 65, 111, 71, 77, 79, 89, 999],
        [0, 1, 56, 66, 112, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 999],
        [0, 1, 51, 61, 113, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 999],
        [0, 1, 52, 62, 114, 72, 74, 75, 76, 78, 79, 80, 83, 84, 85, 88, 999],
        [0, 1, 53, 63, 115, 71, 73, 74, 77, 78, 79, 85, 89, 999],
        [0, 1, 54, 64, 116, 74, 75, 76, 79, 83, 89, 999],
        [0, 1, 54, 64, 117, 74, 75, 76, 79, 83, 89, 999],
        [0, 1, 55, 65, 118, 71, 77, 79, 89, 999],
        [0, 1, 56, 66, 119, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 999],
        [0, 50, 120, 999],
        [0, 1, 51, 61, 107, 121, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 999],
        [0, 1, 52, 62, 108, 122, 72, 74, 75, 78, 79, 81, 82, 83, 84, 85, 999],
        [0, 1, 53, 63, 109, 123, 71, 73, 74, 77, 78, 79, 81, 82, 85, 89, 999],
        [0, 1, 54, 64, 110, 124, 75, 76, 79, 82, 83, 88, 89, 999],
        [0, 1, 55, 65, 111, 125, 71, 77, 79, 82, 89, 999],
        [0, 1, 56, 66, 112, 126, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 999],
        [0, 1, 51, 61, 113, 127, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 999],
        [0, 1, 52, 62, 114, 128, 72, 74, 75, 76, 78, 79, 80, 82, 83, 84, 85, 88, 999],
        [0, 1, 53, 63, 115, 129, 71, 73, 74, 77, 78, 79, 82, 85, 89, 999],
        [0, 1, 54, 64, 116, 130, 74, 75, 76, 79, 82, 83, 89, 999],
        [0, 1, 54, 64, 117, 131, 74, 75, 76, 79, 82, 83, 89, 999],
        [0, 1, 55, 65, 118, 132, 71, 77, 79, 82, 89, 999],
        [0, 1, 56, 66, 119, 133, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 999],
        [0],
        [0],
        [0],
        [0],
        [0],
        [0],
        [0],
        [0, 1, 83, 84, 85, 86, 999],
        [0, 1, 79, 83, 84, 85, 86, 87, 999],
        [0, 1, 55, 65, 111, 71, 77, 79, 999],
        [0, 1, 58, 52, 999],
        [0, 1, 59, 83, 999],
    ];

    RaceOBJ = ["Formless", "Undead", "Brute", "Plant", "Insect", "Fish", "Demon", "Demi-Human", "Angel", "Dragon"];
    elementOBJ = ["Neutral", "Water", "Earth", "Fire", "Wind", "Poison", "Holy", "Dark", "Ghost", "Undead"];
    SizeOBJ = ["Small", "Medium", "Large"];
    StatusOBJ = ["Poison", "Stun", "Freeze", "Curse", "Blind", "Sleep", "Silence", "Chaos", "Bleeding", "Stone", "Weapon Break", "Armor Break"];

    SubName = ["%", "sec", "Damage", "Critical Damage", "Critical Rate", "Over 1000 Hits", "Too High to Calculate", "Immesurable", "X", "Cast Time", "Off", "On"];



    function ClickJob(n) {

        myInnerHtml("A_KakutyouSel", "", 0);
        myInnerHtml("A_KakutyouData", "", 0);
        document.calcForm.A_Kakutyou.value = 0;

        for (i = 0; i <= 12; i++)
            n_A_PassSkill2[i] = 0;
        if (n_SkillSW) {
            document.calcForm.A_SUPPORTIVE_SKILL0.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL1.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL2.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL3.checked = 0;
            document.calcForm.A_SUPPORTIVE_SKILL4.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL5.checked = 0;
            document.calcForm.A_SUPPORTIVE_SKILL6.checked = 0;
            document.calcForm.A_SUPPORTIVE_SKILL7.checked = 0;
            document.calcForm.A_SUPPORTIVE_SKILL8.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL9.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL10.value = 0;
            document.calcForm.A_SUPPORTIVE_SKILL11.checked = 0;
        }

        n_A_JobSet();
        n = n_A_JOB;

        document.calcForm.A_JobLV.options.length = 0;
        w = 0;
        if (n == 0) w = 10;
        else if (n <= 19 || (41 <= n && n <= 43)) w = 50;
        else if (n == 20) w = 71;
        else w = 70;
        for (i = 1; i <= w; i++)
            document.calcForm.A_JobLV.options[i - 1] = new Option(i, i);
        if (n == 20) {
            document.calcForm.A_JobLV.options[69] = new Option("70-99", 70);
            document.calcForm.A_JobLV.options[70] = new Option("+10", 71);
        }

        if (n_A_JOB != 20)
            SuperNoviceFullWeaponCHECK = 0;
        if (SuperNoviceFullWeaponCHECK)
            JobASPD[20][7] = 120;
        else
            JobASPD[20][7] = 0;

        document.calcForm.A_WeaponType.options.length = 0;
        j = 0;
        for (i = 0; i <= 21; i++) {
            if (JobASPD[n][i] != 0) {
                document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i], i);
                j++;
            }
        }


        ClickWeaponType(0);


        for (i = 0; i <= 14; i++) {
            if (JobSkillPassOBJ[n][i] != 999) {
                myInnerHtml("P_Skill" + i, SkillOBJ[JobSkillPassOBJ[n][i]][2], 0);
                myInnerHtml("P_Skill" + i + "s", '<select name="A_PASSIVE_SKILL' + i + '"onChange="StAllCalc()"></select>', 0);
            } else {
                myInnerHtml("P_Skill" + i, "", 0);
                myInnerHtml("P_Skill" + i + "s", "", 0);
            }
        }


        /*	for(j=0;j<=14;j++){
		if(JobSkillPassOBJ[n][j] != 999){
			wSeOB = document.getElementById("A_PASSIVE_SKILL"+j);
			for(i=10;i>=0;i--)
				wSeOB.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][j]][1];i++)
				wSeOB.options[i] = new Option(i,i);
		}
	}
*/
        if (JobSkillPassOBJ[n][0] != 999) {
            document.calcForm.A_PASSIVE_SKILL0.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][0]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][1] != 999) {
            document.calcForm.A_PASSIVE_SKILL1.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][1]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL1.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][2] != 999) {
            document.calcForm.A_PASSIVE_SKILL2.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][2]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL2.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][3] != 999) {
            document.calcForm.A_PASSIVE_SKILL3.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][3]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL3.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][4] != 999) {
            document.calcForm.A_PASSIVE_SKILL4.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][4]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL4.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][5] != 999) {
            document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][5]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][6] != 999) {
            document.calcForm.A_PASSIVE_SKILL6.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][6]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL6.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][7] != 999) {
            document.calcForm.A_PASSIVE_SKILL7.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][7]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL7.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][8] != 999) {
            document.calcForm.A_PASSIVE_SKILL8.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][8]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL8.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][9] != 999) {
            document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][9]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][10] != 999) {
            document.calcForm.A_PASSIVE_SKILL10.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][10]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL10.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][11] != 999) {
            document.calcForm.A_PASSIVE_SKILL11.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][11]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL11.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][12] != 999) {
            document.calcForm.A_PASSIVE_SKILL12.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][12]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL12.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][13] != 999) {
            document.calcForm.A_PASSIVE_SKILL13.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][13]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL13.options[i] = new Option(i, i);
        }
        if (JobSkillPassOBJ[n][14] != 999) {
            document.calcForm.A_PASSIVE_SKILL14.options.length = 0;
            for (i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][14]][1]; i++)
                document.calcForm.A_PASSIVE_SKILL14.options[i] = new Option(i, i);
        }


        if (JobSkillPassOBJ[n][0] == 58) {
            document.calcForm.A_PASSIVE_SKILL0.options.length = 0;
            n_ECname = ["0", "6% Reduction", "12% Reduction", "18% Reduction", "24% Reduction", "30% Reduction"];
            for (i = 0; i <= 5; i++)
                document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(n_ECname[i], i);
        }

        if (JobSkillPassOBJ[n][5] == 78) {
            document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
            n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
            for (i = 0; i <= 6; i++)
                document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(n_ECname[i], i);
        }

        if (JobSkillPassOBJ[n][9] == 78) {
            document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
            n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
            for (i = 0; i <= 6; i++)
                document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(n_ECname[i], i);
        }


        document.calcForm.A_ActiveSkill.options.length = 0;
        for (i = 0; i <= 57 && JobSkillActiveOBJ[n][i] != 999; i++)
            document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[JobSkillActiveOBJ[n][i]][2], JobSkillActiveOBJ[n][i]);


        for (i = 0; i < 20; i++)
            w_ASSP0bk[i] = 999;
        ActiveSkillSetPlus();

        ClickActiveSkill(0);
        WeaponSet2();
    }

    function ClickWeaponType(n) {
        n_A_JobSet();
        if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n != 0)) {
            document.calcForm.A_Arrow.style.visibility = "visible";
            document.calcForm.A_Arrow.options.length = 0;
            if (n == 10 || n == 14 || n == 15) {
                j = 17;
                for (i = 0; i <= 4; i++)
                    ArrowOBJ[i] = ArrowOBJbackup[i];
            } else if (n == 17 || n == 18 || n == 19 || n == 20) {
                j = 2;
                for (i = 0; i <= 2; i++)
                    ArrowOBJ[i] = BulletOBJ[i];
            } else if (n == 21) {
                j = 4;
                for (i = 0; i <= 4; i++)
                    ArrowOBJ[i] = GrenadeOBJ[i]
            } else {
                j = 1;
                ArrowOBJ[0] = [0, 0, "No Arrow"];
                ArrowOBJ[1] = ArrowOBJ[16];
            }
            for (i = 0; i <= j; i++)
                document.calcForm.A_Arrow.options[i] = new Option(ArrowOBJ[i][2], i);
        } else {
            document.calcForm.A_Arrow.value = 0;
            document.calcForm.A_Arrow.style.visibility = "hidden";
        }
        WeaponSet();


        if (n == 0) {
            myInnerHtml("A_seirenchi_name", "", 0);
            document.calcForm.A_Weapon_ATKplus.style.visibility = "hidden";
            document.calcForm.A_Weapon_ATKplus.value = 0;
        } else {
            myInnerHtml("A_seirenchi_name", "Refine: ", 0);
            document.calcForm.A_Weapon_ATKplus.style.visibility = "visible";
        }


        n_A_JobSet();
        if ((n_A_JOB == 8 || n_A_JOB == 22) && n != 11) {
            if (hasLeftHand == 0)
                myInnerHtml("A_SobWeaponName", " Left Hand: " + '<select name="A_Weapon2Type" onChange = "ClickWeaponType2(this[this.selectedIndex].value) | StAllCalc()"> <option value="0">Unarmed (or Shield)<option value="1">Dagger<option value="2">One-Hand Sword<option value="6">One-Hand Axe</select>', 0);
        } else {
            myInnerHtml("A_SobWeaponName", "", 0);
            myInnerHtml("spanA_weapon2", "", 0);
            myInnerHtml("spanA_weapon2seiren", "", 0);
            myInnerHtml("spanA_weapon2_CardShort", "", 0);
            myInnerHtml("nA_weapon2_c1", "", 0);
            myInnerHtml("nA_weapon2_c2", "", 0);
            myInnerHtml("nA_weapon2_c3", "", 0);
            myInnerHtml("nA_weapon2_c4", "", 0);
            hasLeftHand = 0;
        }
        n_A_Equip[0] = eval(document.calcForm.A_weapon1.value);
        ActiveSkillSetPlus();
    }


    function ClickWeaponType2(n) {

        n_A_JobSet();
        if (n != 0) {
            if (hasLeftHand == 0) {
                myInnerHtml("spanA_weapon2", '<select name="A_weapon2"onChange="StAllCalc()|ClickB_Item(this[this.selectedIndex].value)"></select>', 0);
                myInnerHtml("spanA_weapon2seiren", "Refine(Left):" + '<select name="A_Weapon2_ATKplus"></select>', 0);
                for (i = 0; i <= 10; i++) {
                    document.calcForm.A_Weapon2_ATKplus.options[i] = new Option(i, i);
                }

                myInnerHtml("nA_weapon2_c1", '<select name="A_weapon2_card1"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>', 0);
                myInnerHtml("nA_weapon2_c2", '<select name="A_weapon2_card2"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>', 0);
                myInnerHtml("nA_weapon2_c3", '<select name="A_weapon2_card3"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>', 0);
                myInnerHtml("nA_weapon2_c4", '<select name="A_weapon2_card4"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>', 0);

                for (i = 0; CardSortOBJ[0][i] != "NULL"; i++)
                    document.calcForm.A_weapon2_card1.options[i] = new Option(cardOBJ[CardSortOBJ[0][i]][2], cardOBJ[CardSortOBJ[0][i]][0]);
                for (i = 0; CardSortOBJ[1][i] != "NULL"; i++) {
                    document.calcForm.A_weapon2_card2.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
                    document.calcForm.A_weapon2_card3.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
                    document.calcForm.A_weapon2_card4.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
                }
                document.calcForm.A_weapon2_card4.options[1] = new Option("Top 10 Rank", 106);
            }
            myInnerHtml("spanA_weapon2_CardShort", '<select name="A_cardshortLeft" onChange="SetCardShortLeft()|StAllCalc()|ActiveSkillSetPlus()"></select>', 0);
            document.calcForm.A_cardshortLeft.options[0] = new Option("Card Shortcuts (Left)", 0);
            for (i = 1; i <= 32; i++)
                document.calcForm.A_cardshortLeft.options[i] = new Option(CardShort[i][0], i);
            hasLeftHand = 1;
            WeaponSetLeft();
        } else {
            myInnerHtml("spanA_weapon2", "", 0);
            myInnerHtml("spanA_weapon2seiren", "", 0);
            myInnerHtml("spanA_weapon2_CardShort", "", 0);
            myInnerHtml("nA_weapon2_c1", "", 0);
            myInnerHtml("nA_weapon2_c2", "", 0);
            myInnerHtml("nA_weapon2_c3", "", 0);
            myInnerHtml("nA_weapon2_c4", "", 0);
            hasLeftHand = 0;
        }
        if (hasLeftHand) {
            n_A_Equip[1] = eval(document.calcForm.A_weapon2.value);
            ActiveSkillSetPlus();
        }
    }

    function ClickActiveSkill(wAS) {
        n_A_ActiveSkill = eval(document.calcForm.A_ActiveSkill.value);
        skillToUseName = SkillOBJ[n_A_ActiveSkill][2];
        if (n_A_ActiveSkill > 100000) {
            n_A_ActiveSkillLV = Math.floor(n_A_ActiveSkill % 100);
            n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) / 100);
        } else
            n_A_ActiveSkillLV = SkillOBJ[n_A_ActiveSkill][1];

        document.calcForm.A_ActiveSkillLV.options.length = 0;
        if (n_A_ActiveSkill >= 0)
            for (i = 1; i <= n_A_ActiveSkillLV; i++)
                document.calcForm.A_ActiveSkillLV.options[i - 1] = new Option(i, i);

        if (SkillOBJ[n_A_ActiveSkill][1] == 1)
            document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
        else {
            document.calcForm.A_ActiveSkillLV.style.visibility = "visible";
            document.calcForm.A_ActiveSkillLV.value = n_A_ActiveSkillLV;
        }
        ClickActiveSkill2();
    }


    function ClickActiveSkill2() {
        if (n_A_ActiveSkill == 66 || n_A_ActiveSkill == 326) {
            myInnerHtml("AASkillName", "Card Weight: ", 0);
            myInnerHtml("AASkill", '<input type="text" name="SkillSubNum" value="8000" size=8>', 0);
        } else if (n_A_ActiveSkill == 112 || n_A_ActiveSkill == 113) {
            myInnerHtml("AASkillName", "Number of Stacked Monsters: ", 0);
            myInnerHtml("AASkill", '<select name="SkillSubNum"onChange="calc()"></select>', 0);
            for (i = 1; i <= 20; i++)
                document.calcForm.SkillSubNum.options[i - 1] = new Option(i, i);
            document.calcForm.SkillSubNum.value = 1;
        } else if (n_A_ActiveSkill == 131) {
            myInnerHtml("AASkillName", "Number of Hits: ", 0);
            myInnerHtml("AASkill", '<select name="SkillSubNum"onChange="calc()"></select>', 0);
            for (i = 1; i <= 15; i++)
                document.calcForm.SkillSubNum.options[i - 1] = new Option(i, i);
            document.calcForm.SkillSubNum.value = 3;
        } else if (n_A_ActiveSkill == 88) {
            myInnerHtml("AASkillName", "Poison React Level: ", 0);
            myInnerHtml("AASkill", '<select name="SkillSubNum"onChange="calc()"></select>', 0);
            for (i = 0; i <= 10; i++)
                document.calcForm.SkillSubNum.options[i] = new Option(i, i);
            document.calcForm.SkillSubNum.value = 5;
        } else if (n_A_ActiveSkill == 197) {
            myInnerHtml("AASkillName", "Remaining SP: ", 0);
            myInnerHtml("AASkill", '<input type="text" name="SkillSubNum" size=6>', 0);
            document.calcForm.SkillSubNum.value = n_A_MaxSP - 1;
        } else if (n_A_ActiveSkill == 394) {
            myInnerHtml("AASkillName", "", 0);
            myInnerHtml("AASkill", '<select name="SkillSubNum"onChange="calc()"></select>', 0);
            for (i = 0; i <= 4; i++)
                document.calcForm.SkillSubNum.options[i] = new Option(SyurikenOBJ[i][2], i);
            document.calcForm.SkillSubNum.value = 0;
        } else if (n_A_ActiveSkill == 395) {
            myInnerHtml("AASkillName", "", 0);
            myInnerHtml("AASkill", '<select name="SkillSubNum"onChange="calc()"></select>', 0);
            for (i = 0; i <= 4; i++)
                document.calcForm.SkillSubNum.options[i] = new Option(KunaiOBJ[i][2], i);
            document.calcForm.SkillSubNum.value = 0;
        } else if (n_A_ActiveSkill == 405) {
            myInnerHtml("AASkillName", "Remaining SP: ", 0);
            myInnerHtml("AASkill", '<input type="text" name="SkillSubNum" size=6>', 0);
            document.calcForm.SkillSubNum.value = n_A_MaxHP - 1;
        } else {
            myInnerHtml("AASkillName", "", 0);
            myInnerHtml("AASkill", "", 0);
        }
    }

    function Click_SkillSW() {
        n_SkillSW = document.calcForm.A_SUPPORTIVE_SKILLSW.checked;

        if (n_SkillSW) {
            name_CSSW_SKILL = ["Blessing", "Increase Agi", "Impositio Manus", "Gloria", "Angelus", "Assumptio", "Andrenaline Rush", "Weapon Perfection", "Power Thrust", "Wind Walker", "Spirit Spheres (GG Card)", "Magnum Break Bonus", "Aloevera", "<Font size=2>Suffragium</Font>", "<Font size=2>Resistant Souls</Font>", "<Font size=2>Additional Buffs Found Below</Font>"];
            html_CSSW_SKILL = new Array();
            for (i = 0; i <= 15; i++)
                myInnerHtml("AS" + i + "_1", name_CSSW_SKILL[i], 0);

            html_CSSW_SKILL[0] = '<select name="A_SUPPORTIVE_SKILL0"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[1] = '<select name="A_SUPPORTIVE_SKILL1"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[2] = '<select name="A_SUPPORTIVE_SKILL2"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[3] = '<input type="checkbox" name="A_SUPPORTIVE_SKILL3"onClick="calc()">';
            html_CSSW_SKILL[4] = '<select name="A_SUPPORTIVE_SKILL4"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[5] = '<input type="checkbox" name="A_SUPPORTIVE_SKILL5"onClick="calc()">';
            html_CSSW_SKILL[6] = '<select name="A_SUPPORTIVE_SKILL6"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[7] = '<input type="checkbox" name="A_SUPPORTIVE_SKILL7"onClick="calc()">';
            html_CSSW_SKILL[8] = '<select name="A_SUPPORTIVE_SKILL8"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[9] = '<select name="A_SUPPORTIVE_SKILL9"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[10] = '<select name="A_SUPPORTIVE_SKILL10"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[11] = '<input type="checkbox" name="A_SUPPORTIVE_SKILL11"onClick="calc()">';
            html_CSSW_SKILL[12] = '<input type="checkbox" name="A_SUPPORTIVE_SKILL12"onClick="calc()">';
            html_CSSW_SKILL[13] = '<select name="A_SUPPORTIVE_SKILL13"onChange="StAllCalc()"></select>';
            html_CSSW_SKILL[14] = '<select name="A_SUPPORTIVE_SKILL14"onChange="StAllCalc()"></select>';
            for (i = 0; i <= 14; i++)
                myInnerHtml("AS" + i + "_2", html_CSSW_SKILL[i], 0);


            for (i = 0; i <= 10; i++) {
                document.calcForm.A_SUPPORTIVE_SKILL0.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL1.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL4.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL9.options[i] = new Option(i, i);
            }
            for (i = 0; i <= 5; i++) {
                document.calcForm.A_SUPPORTIVE_SKILL2.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL8.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL10.options[i] = new Option(i, i);
                document.calcForm.A_SUPPORTIVE_SKILL14.options[i] = new Option(i, i);
            }
            if (n_A_JOB == 15 || n_A_JOB == 29)
                myInnerHtml("AS10_1", "-", 0);
            for (i = 0; i <= 3; i++)
                document.calcForm.A_SUPPORTIVE_SKILL13.options[i] = new Option(i, i);
            document.calcForm.A_SUPPORTIVE_SKILL6.options[0] = new Option("None", 0);
            document.calcForm.A_SUPPORTIVE_SKILL6.options[1] = new Option("Regular AR", 1);
            document.calcForm.A_SUPPORTIVE_SKILL6.options[2] = new Option("Full AR", 2);
            document.calcForm.A_SUPPORTIVE_SKILL6.options[3] = new Option("Scroll", 3);

            document.calcForm.A_SUPPORTIVE_SKILL0.value = n_A_PassSkill2[0];
            document.calcForm.A_SUPPORTIVE_SKILL1.value = n_A_PassSkill2[1];
            document.calcForm.A_SUPPORTIVE_SKILL2.value = n_A_PassSkill2[2];
            document.calcForm.A_SUPPORTIVE_SKILL3.checked = n_A_PassSkill2[3];
            document.calcForm.A_SUPPORTIVE_SKILL4.value = n_A_PassSkill2[4];
            document.calcForm.A_SUPPORTIVE_SKILL5.checked = n_A_PassSkill2[5];
            document.calcForm.A_SUPPORTIVE_SKILL6.value = n_A_PassSkill2[6];
            document.calcForm.A_SUPPORTIVE_SKILL7.checked = n_A_PassSkill2[7];
            document.calcForm.A_SUPPORTIVE_SKILL8.value = n_A_PassSkill2[8];
            document.calcForm.A_SUPPORTIVE_SKILL9.value = n_A_PassSkill2[9];
            document.calcForm.A_SUPPORTIVE_SKILL10.value = n_A_PassSkill2[10];
            document.calcForm.A_SUPPORTIVE_SKILL11.checked = n_A_PassSkill2[11];
            document.calcForm.A_SUPPORTIVE_SKILL12.checked = n_A_PassSkill2[12];
            document.calcForm.A_SUPPORTIVE_SKILL13.value = n_A_PassSkill2[13];
            document.calcForm.A_SUPPORTIVE_SKILL14.value = n_A_PassSkill2[14];
        } else {
            for (i = 0; i <= 14; i++) {
                myInnerHtml("AS" + i + "_1", "", 0);
                myInnerHtml("AS" + i + "_2", "", 0);
            }
            myInnerHtml("AS15_1", "", 0);
        }
    }

    SWs3sw = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    function Click_Skill3SW() {
        n_Skill3SW = document.calcForm.A_PERFORMANCE_SKILLSW.checked;

        if (n_Skill3SW) {
            name_CS3SW_SKILL = ["Perfect Tabulature", "Impressive Rift", "Magic Strings", "Song of Lutie", "Focus Ballet", "Lady Luck", "Gypsie's Kiss", "Acoustic Rhythm", "Mental Sensing", "Battle Theme", "Harmonic Lick"];
            html_CS3SW_SKILL = new Array();
            for (i = 0; i <= 10; i++)
                myInnerHtml("EN" + i + "_1", name_CS3SW_SKILL[i], 0);

            html_CS3SW_SKILL[0] = '<select name="A_PERFORMANCE_SKILL0_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[1] = '<select name="A_PERFORMANCE_SKILL1_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[2] = '<select name="A_PERFORMANCE_SKILL2_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[3] = '<select name="A_PERFORMANCE_SKILL3_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[4] = '<select name="A_PERFORMANCE_SKILL4_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[5] = '<select name="A_PERFORMANCE_SKILL5_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[6] = '<select name="A_PERFORMANCE_SKILL6_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
            html_CS3SW_SKILL[7] = '<select name="A_PERFORMANCE_SKILL7"onChange="StAllCalc()"></select>';
            html_CS3SW_SKILL[8] = '<select name="A_PERFORMANCE_SKILL8"onChange="ClickB_Enemy()"></select>';
            html_CS3SW_SKILL[9] = '<select name="A_PERFORMANCE_SKILL9"onChange="StAllCalc()"></select>';
            html_CS3SW_SKILL[10] = '<select name="A_PERFORMANCE_SKILL10"onChange="StAllCalc()"></select>';
            for (i = 0; i <= 10; i++)
                myInnerHtml("EN" + i + "_2", html_CS3SW_SKILL[i], 0);

            myInnerHtml("EN11_1", "Marionette Control" + '<input type="checkbox" name="A_PERFORMANCE_SKILL11"onClick="Skill3SW_2()|calc()">', 0);

            for (i = 0; i <= 10; i++) {
                document.calcForm.A_PERFORMANCE_SKILL0_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL1_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL2_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL3_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL4_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL5_1.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL6_1.options[i] = new Option(i, i);
            }
            for (i = 0; i <= 5; i++) {
                document.calcForm.A_PERFORMANCE_SKILL7.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL8.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL9.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL10.options[i] = new Option(i, i);
            }

            document.calcForm.A_PERFORMANCE_SKILL0_1.value = n_A_PassSkill3[0];
            document.calcForm.A_PERFORMANCE_SKILL1_1.value = n_A_PassSkill3[1];
            document.calcForm.A_PERFORMANCE_SKILL2_1.value = n_A_PassSkill3[2];
            document.calcForm.A_PERFORMANCE_SKILL3_1.value = n_A_PassSkill3[3];
            document.calcForm.A_PERFORMANCE_SKILL4_1.value = n_A_PassSkill3[4];
            document.calcForm.A_PERFORMANCE_SKILL5_1.value = n_A_PassSkill3[5];
            document.calcForm.A_PERFORMANCE_SKILL6_1.value = n_A_PassSkill3[6];
            document.calcForm.A_PERFORMANCE_SKILL7.value = n_A_PassSkill3[7];
            document.calcForm.A_PERFORMANCE_SKILL8.value = n_A_PassSkill3[8];
            document.calcForm.A_PERFORMANCE_SKILL9.value = n_A_PassSkill3[9];
            document.calcForm.A_PERFORMANCE_SKILL10.value = n_A_PassSkill3[10];
            document.calcForm.A_PERFORMANCE_SKILL11.checked = n_A_PassSkill3[11];

            Skill3SW_2();
        } else {
            for (i = 1; i <= 6; i++) {
                myInnerHtml("EN0_" + i, "", 0);
                myInnerHtml("EN1_" + i, "", 0);
                myInnerHtml("EN3_" + i, "", 0);
                myInnerHtml("EN4_" + i, "", 0);
                myInnerHtml("EN5_" + i, "", 0);
                myInnerHtml("EN6_" + i, "", 0);
            }
            for (i = 1; i <= 8; i++)
                myInnerHtml("EN2_" + i, "", 0);
            for (i = 1; i <= 2; i++) {
                myInnerHtml("EN7_" + i, "", 0);
                myInnerHtml("EN8_" + i, "", 0);
                myInnerHtml("EN9_" + i, "", 0);
                myInnerHtml("EN10_" + i, "", 0);
                myInnerHtml("EN11_" + i, "", 0);
            }

            for (i = 0; i <= 11; i++)
                SWs3sw[i] = 0;
        }
    }

    function Skill3SW_2() {
        n_A_PassSkill3[0] = eval(document.calcForm.A_PERFORMANCE_SKILL0_1.value);
        n_A_PassSkill3[1] = eval(document.calcForm.A_PERFORMANCE_SKILL1_1.value);
        n_A_PassSkill3[2] = eval(document.calcForm.A_PERFORMANCE_SKILL2_1.value);
        n_A_PassSkill3[3] = eval(document.calcForm.A_PERFORMANCE_SKILL3_1.value);
        n_A_PassSkill3[4] = eval(document.calcForm.A_PERFORMANCE_SKILL4_1.value);
        n_A_PassSkill3[5] = eval(document.calcForm.A_PERFORMANCE_SKILL5_1.value);
        n_A_PassSkill3[6] = eval(document.calcForm.A_PERFORMANCE_SKILL6_1.value);
        n_A_PassSkill3[11] = eval(document.calcForm.A_PERFORMANCE_SKILL11.checked);

        if (n_A_PassSkill3[0] != 0) {
            if (SWs3sw[0] == 0) {
                myInnerHtml("EN0_3", "Bard's Agi", 0);
                myInnerHtml("EN0_4", '<select name="A_PERFORMANCE_SKILL0_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN0_5", "Music Lessons", 0);
                myInnerHtml("EN0_6", '<select name="A_PERFORMANCE_SKILL0_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 150; i++)
                    document.calcForm.A_PERFORMANCE_SKILL0_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL0_3.options[i - 1] = new Option(i, i);
                SWs3sw[0] = 1;
                document.calcForm.A_PERFORMANCE_SKILL0_2.value = n_A_PassSkill3[20];
                document.calcForm.A_PERFORMANCE_SKILL0_3.value = n_A_PassSkill3[30];
            }
        } else {
            SWs3sw[0] = 0;
            myInnerHtml("EN0_3", "-", 0);
            myInnerHtml("EN0_4", "-", 0);
            myInnerHtml("EN0_5", "", 0);
            myInnerHtml("EN0_6", "", 0);
        }

        if (n_A_PassSkill3[1] != 0) {
            if (SWs3sw[1] == 0) {
                myInnerHtml("EN1_3", "Bard's Agi", 0);
                myInnerHtml("EN1_4", '<select name="A_PERFORMANCE_SKILL1_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN1_5", "Music Lessons", 0);
                myInnerHtml("EN1_6", '<select name="A_PERFORMANCE_SKILL1_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 150; i++)
                    document.calcForm.A_PERFORMANCE_SKILL1_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL1_3.options[i - 1] = new Option(i, i);
                SWs3sw[1] = 1;
                document.calcForm.A_PERFORMANCE_SKILL1_2.value = n_A_PassSkill3[21];
                document.calcForm.A_PERFORMANCE_SKILL1_3.value = n_A_PassSkill3[31];
            }
        } else {
            SWs3sw[1] = 0;
            myInnerHtml("EN1_3", "-", 0);
            myInnerHtml("EN1_4", "-", 0);
            myInnerHtml("EN1_5", "", 0);
            myInnerHtml("EN1_6", "", 0);
        }

        if (n_A_PassSkill3[2] != 0) {
            if (SWs3sw[2] == 0) {
                myInnerHtml("EN2_3", "Bard's Dex", 0);
                myInnerHtml("EN2_4", '<select name="A_PERFORMANCE_SKILL2_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN2_5", "Bard's Int", 0);
                myInnerHtml("EN2_6", '<select name="A_PERFORMANCE_SKILL2_3"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN2_7", "Music Lessons", 0);
                myInnerHtml("EN2_8", '<select name="A_PERFORMANCE_SKILL2_4"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 200; i++)
                    document.calcForm.A_PERFORMANCE_SKILL2_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 150; i++)
                    document.calcForm.A_PERFORMANCE_SKILL2_3.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL2_4.options[i - 1] = new Option(i, i);
                SWs3sw[2] = 1;
                document.calcForm.A_PERFORMANCE_SKILL2_2.value = n_A_PassSkill3[22];
                document.calcForm.A_PERFORMANCE_SKILL2_3.value = n_A_PassSkill3[29];
                document.calcForm.A_PERFORMANCE_SKILL2_4.value = n_A_PassSkill3[32];
            }
        } else {
            SWs3sw[2] = 0;
            myInnerHtml("EN2_3", "-", 0);
            myInnerHtml("EN2_4", "-", 0);
            myInnerHtml("EN2_5", "", 0);
            myInnerHtml("EN2_6", "", 0);
            myInnerHtml("EN2_7", "", 0);
            myInnerHtml("EN2_8", "", 0);
        }

        if (n_A_PassSkill3[3] != 0) {
            if (SWs3sw[3] == 0) {
                myInnerHtml("EN3_3", "Bard's Vit", 0);
                myInnerHtml("EN3_4", '<select name="A_PERFORMANCE_SKILL3_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN3_5", "Music Lessons", 0);
                myInnerHtml("EN3_6", '<select name="A_PERFORMANCE_SKILL3_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 150; i++)
                    document.calcForm.A_PERFORMANCE_SKILL3_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL3_3.options[i - 1] = new Option(i, i);
                SWs3sw[3] = 1;
                document.calcForm.A_PERFORMANCE_SKILL3_2.value = n_A_PassSkill3[23];
                document.calcForm.A_PERFORMANCE_SKILL3_3.value = n_A_PassSkill3[33];
            }
        } else {
            SWs3sw[3] = 0;
            myInnerHtml("EN3_3", "-", 0);
            myInnerHtml("EN3_4", "-", 0);
            myInnerHtml("EN3_5", "", 0);
            myInnerHtml("EN3_6", "", 0);
        }

        if (n_A_PassSkill3[4] != 0) {
            if (SWs3sw[4] == 0) {
                myInnerHtml("EN4_3", "Dancer's Dex", 0);
                myInnerHtml("EN4_4", '<select name="A_PERFORMANCE_SKILL4_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN4_5", "Dancing Lessons", 0);
                myInnerHtml("EN4_6", '<select name="A_PERFORMANCE_SKILL4_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 180; i++)
                    document.calcForm.A_PERFORMANCE_SKILL4_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL4_3.options[i - 1] = new Option(i, i);
                SWs3sw[4] = 1;
                document.calcForm.A_PERFORMANCE_SKILL4_2.value = n_A_PassSkill3[24];
                document.calcForm.A_PERFORMANCE_SKILL4_3.value = n_A_PassSkill3[34];
            }
        } else {
            SWs3sw[4] = 0;
            myInnerHtml("EN4_3", "-", 0);
            myInnerHtml("EN4_4", "-", 0);
            myInnerHtml("EN4_5", "", 0);
            myInnerHtml("EN4_6", "", 0);
        }

        if (n_A_PassSkill3[5] != 0) {
            if (SWs3sw[5] == 0) {
                myInnerHtml("EN5_3", "Dancer's Luck", 0);
                myInnerHtml("EN5_4", '<select name="A_PERFORMANCE_SKILL5_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN5_5", "Dancing Lessons", 0);
                myInnerHtml("EN5_6", '<select name="A_PERFORMANCE_SKILL5_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 180; i++)
                    document.calcForm.A_PERFORMANCE_SKILL5_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL5_3.options[i - 1] = new Option(i, i);
                SWs3sw[5] = 1;
                document.calcForm.A_PERFORMANCE_SKILL5_2.value = n_A_PassSkill3[25];
                document.calcForm.A_PERFORMANCE_SKILL5_3.value = n_A_PassSkill3[35];
            }
        } else {
            SWs3sw[5] = 0;
            myInnerHtml("EN5_3", "-", 0);
            myInnerHtml("EN5_4", "-", 0);
            myInnerHtml("EN5_5", "", 0);
            myInnerHtml("EN5_6", "", 0);
        }

        if (n_A_PassSkill3[6] != 0) {
            if (SWs3sw[6] == 0) {
                myInnerHtml("EN6_3", "Dancer's Int", 0);
                myInnerHtml("EN6_4", '<select name="A_PERFORMANCE_SKILL6_2"onChange="StAllCalc()"></select>', 0);
                myInnerHtml("EN6_5", "Dancing Lessons", 0);
                myInnerHtml("EN6_6", '<select name="A_PERFORMANCE_SKILL6_3"onChange="StAllCalc()"></select>', 0);

                for (i = 1; i <= 180; i++)
                    document.calcForm.A_PERFORMANCE_SKILL6_2.options[i - 1] = new Option(i, i);
                for (i = 1; i <= 10; i++)
                    document.calcForm.A_PERFORMANCE_SKILL6_3.options[i - 1] = new Option(i, i);
                SWs3sw[4] = 1;
                document.calcForm.A_PERFORMANCE_SKILL6_2.value = n_A_PassSkill3[26];
                document.calcForm.A_PERFORMANCE_SKILL6_3.value = n_A_PassSkill3[36];
            }
        } else {
            SWs3sw[6] = 0;
            myInnerHtml("EN6_3", "-", 0);
            myInnerHtml("EN6_4", "-", 0);
            myInnerHtml("EN6_5", "", 0);
            myInnerHtml("EN6_6", "", 0);
        }

        if (n_A_PassSkill3[11] != 0) {
            if (SWs3sw[11] == 0) {
                myInnerHtml("EN11_2",
                    "<br>Controller's Stats: " +
                    '<select name="A_PERFORMANCE_SKILL11_STR"onChange="StAllCalc()"></select>' +
                    '<select name="A_PERFORMANCE_SKILL11_AGI"onChange="StAllCalc()"></select>' +
                    '<select name="A_PERFORMANCE_SKILL11_VIT"onChange="StAllCalc()"></select>' +
                    '<select name="A_PERFORMANCE_SKILL11_INT"onChange="StAllCalc()"></select>' +
                    '<select name="A_PERFORMANCE_SKILL11_DEX"onChange="StAllCalc()"></select>' +
                    '<select name="A_PERFORMANCE_SKILL11_LUK"onChange="StAllCalc()"></select>', 0);
                ;

                document.calcForm.A_PERFORMANCE_SKILL11_STR.options[0] = new Option("STR", 0);
                document.calcForm.A_PERFORMANCE_SKILL11_AGI.options[0] = new Option("AGI", 0);
                document.calcForm.A_PERFORMANCE_SKILL11_VIT.options[0] = new Option("VIT", 0);
                document.calcForm.A_PERFORMANCE_SKILL11_INT.options[0] = new Option("INT", 0);
                document.calcForm.A_PERFORMANCE_SKILL11_DEX.options[0] = new Option("DEX", 0);
                document.calcForm.A_PERFORMANCE_SKILL11_LUK.options[0] = new Option("LUK", 0);

                for (i = 1; i <= 99; i++) {
                    document.calcForm.A_PERFORMANCE_SKILL11_STR.options[i] = new Option(i, i);
                    document.calcForm.A_PERFORMANCE_SKILL11_AGI.options[i] = new Option(i, i);
                    document.calcForm.A_PERFORMANCE_SKILL11_VIT.options[i] = new Option(i, i);
                    document.calcForm.A_PERFORMANCE_SKILL11_INT.options[i] = new Option(i, i);
                    document.calcForm.A_PERFORMANCE_SKILL11_DEX.options[i] = new Option(i, i);
                    document.calcForm.A_PERFORMANCE_SKILL11_LUK.options[i] = new Option(i, i);
                }
                SWs3sw[11] = 1;
                document.calcForm.A_PERFORMANCE_SKILL11_STR.value = n_A_PassSkill3[12];
                document.calcForm.A_PERFORMANCE_SKILL11_AGI.value = n_A_PassSkill3[13];
                document.calcForm.A_PERFORMANCE_SKILL11_VIT.value = n_A_PassSkill3[14];
                document.calcForm.A_PERFORMANCE_SKILL11_INT.value = n_A_PassSkill3[15];
                document.calcForm.A_PERFORMANCE_SKILL11_DEX.value = n_A_PassSkill3[16];
                document.calcForm.A_PERFORMANCE_SKILL11_LUK.value = n_A_PassSkill3[17];
            }
        } else {
            SWs3sw[11] = 0;
            myInnerHtml("EN11_2", "", 0);
        }

    }

    function Click_Skill4SW() {
        n_Skill4SW = document.calcForm.A4_SKILLSW.checked;

        if (n_Skill4SW) {
            name_CS4SW_SKILL = ["Battle Orders", "Great Leadership", "Wounds of Glory", "Soul of Cold", "Sharp Hawk Eyes"];
            html_CS4SW_SKILL = new Array();
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN4" + i + "_1", name_CS4SW_SKILL[i], 0);
            html_CS4SW_SKILL[0] = '<input type="checkbox" name="A_PERFORMANCE_SKILL40"onClick="calc()">';
            html_CS4SW_SKILL[1] = '<select name="A_PERFORMANCE_SKILL41"onChange="StAllCalc()"></select>';
            html_CS4SW_SKILL[2] = '<select name="A_PERFORMANCE_SKILL42"onChange="StAllCalc()"></select>';
            html_CS4SW_SKILL[3] = '<select name="A_PERFORMANCE_SKILL43"onChange="StAllCalc()"></select>';
            html_CS4SW_SKILL[4] = '<select name="A_PERFORMANCE_SKILL44"onChange="StAllCalc()"></select>';
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN4" + i + "_2", html_CS4SW_SKILL[i], 0);

            for (i = 0; i <= 5; i++) {
                document.calcForm.A_PERFORMANCE_SKILL41.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL42.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL43.options[i] = new Option(i, i);
                document.calcForm.A_PERFORMANCE_SKILL44.options[i] = new Option(i, i);
            }
            document.calcForm.A_PERFORMANCE_SKILL40.checked = n_A_PassSkill3[40];
            document.calcForm.A_PERFORMANCE_SKILL41.value = n_A_PassSkill3[41];
            document.calcForm.A_PERFORMANCE_SKILL42.value = n_A_PassSkill3[42];
            document.calcForm.A_PERFORMANCE_SKILL43.value = n_A_PassSkill3[43];
            document.calcForm.A_PERFORMANCE_SKILL44.value = n_A_PassSkill3[44];
        } else {
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN4" + i + "_1", "", 0);
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN4" + i + "_2", "", 0);
        }
    }

    function Click_Skill5SW() {
        n_Skill5SW = document.calcForm.A_BATTLECHANT_SKILLSW.checked;

        if (n_Skill5SW) {
            name_CS5SW_SKILL = ["All Stats+20", "HP+100%", "SP+100%", "ATK+100%", "HIT+50 & FLEE+50"];
            html_CS5SW_SKILL = new Array();
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN5" + i + "_1", name_CS5SW_SKILL[i], 0);
            html_CS5SW_SKILL[0] = '<input type="checkbox" name="A_BATTLECHANT_SKILL0"onClick="calc()">';
            html_CS5SW_SKILL[1] = '<input type="checkbox" name="A_BATTLECHANT_SKILL1"onClick="calc()">';
            html_CS5SW_SKILL[2] = '<input type="checkbox" name="A_BATTLECHANT_SKILL2"onClick="calc()">';
            html_CS5SW_SKILL[3] = '<input type="checkbox" name="A_BATTLECHANT_SKILL3"onClick="calc()">';
            html_CS5SW_SKILL[4] = '<input type="checkbox" name="A_BATTLECHANT_SKILL4"onClick="calc()">';
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN5" + i + "_2", html_CS5SW_SKILL[i], 0);

            document.calcForm.A_BATTLECHANT_SKILL0.checked = n_A_PassSkill5[0];
            document.calcForm.A_BATTLECHANT_SKILL1.checked = n_A_PassSkill5[1];
            document.calcForm.A_BATTLECHANT_SKILL2.checked = n_A_PassSkill5[2];
            document.calcForm.A_BATTLECHANT_SKILL3.checked = n_A_PassSkill5[3];
            document.calcForm.A_BATTLECHANT_SKILL4.checked = n_A_PassSkill5[4];
        } else {
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN5" + i + "_1", "", 0);
            for (i = 0; i <= 4; i++)
                myInnerHtml("EN5" + i + "_2", "", 0);
        }
    }

    function Click_Skill6SW() {
        n_Skill6SW = document.calcForm.A_GROUND_SUPPORTIVE_SKILLSW.checked;

        if (n_Skill6SW) {
            myInnerHtml("EN60_1", '<select name="A_GROUND_SUPPORTIVE_SKILL0"onChange="StAllCalc()"></select>', 0);
            myInnerHtml("EN60_2", '<select name="A_GROUND_SUPPORTIVE_SKILL1"onChange="StAllCalc()"></select>', 0);

            document.calcForm.A_GROUND_SUPPORTIVE_SKILL0.options[0] = new Option("Volcano", 0);
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL0.options[1] = new Option("Deluge", 1);
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL0.options[2] = new Option("Whirlwind", 2);
            for (i = 0; i <= 5; i++)
                document.calcForm.A_GROUND_SUPPORTIVE_SKILL1.options[i] = new Option(i, i);

            myInnerHtml("EN61_1", "Murderer Bonus", 0);
            myInnerHtml("EN61_2", '<select name="A_GROUND_SUPPORTIVE_SKILL2"onChange="StAllCalc()"></select>', 0);
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL2.options[0] = new Option("None", 0);
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL2.options[1] = new Option("ALL+3", 1);
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL2.options[2] = new Option("ALL+5", 2);

            myInnerHtml("EN62_1", "Anolian C(IC1)", 0);
            myInnerHtml("EN62_2", '<input type="checkbox" name="A_GROUND_SUPPORTIVE_SKILL3"onClick="calc()">', 0);

            document.calcForm.A_GROUND_SUPPORTIVE_SKILL0.value = n_A_PassSkill6[0];
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL1.value = n_A_PassSkill6[1];
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL2.value = n_A_PassSkill6[2];
            document.calcForm.A_GROUND_SUPPORTIVE_SKILL3.checked = n_A_PassSkill6[3];
        } else {
            myInnerHtml("EN60_1", "", 0);
            myInnerHtml("EN60_2", "", 0);
            myInnerHtml("EN61_1", "", 0);
            myInnerHtml("EN61_2", "", 0);
            myInnerHtml("EN62_1", "", 0);
            myInnerHtml("EN62_2", "", 0);
        }
    }

    function Click_Skill7SW() {
        n_Skill7SW = document.calcForm.A_FOOD_BOX_BONUSSW.checked;

        if (n_Skill7SW) {
            myInnerHtml("EN70_1", "Sesame Pastry (HIT+30)", 0);
            myInnerHtml("EN70_2", '<input type="checkbox" name="A_FOOD_BOX_BONUS0"onClick="calc()">', 0);

            myInnerHtml("EN71_1", "Honey Pastry (FLEE+30)", 0);
            myInnerHtml("EN71_2", '<input type="checkbox" name="A_FOOD_BOX_BONUS1"onClick="calc()">', 0);

            myInnerHtml("EN72_1", "Rainbow Cake (ATK/MATK+10)", 0);
            myInnerHtml("EN72_2", '<input type="checkbox" name="A_FOOD_BOX_BONUS2"onClick="calc()">', 0);

            myInnerHtml("EN79_1", "Box of Resentment (ATK+20)", 0);
            myInnerHtml("EN79_2", '<input type="checkbox" name="A_FOOD_BOX_BONUS9"onClick="calc()">', 0);

            myInnerHtml("EN710_1", "Box of Drowsiness (MATK+20)", 0);
            myInnerHtml("EN710_2", '<input type="checkbox" name="A_FOOD_BOX_BONUS10"onClick="calc()">', 0);

            myInnerHtml("EN73", '<select name="A_FOOD_BOX_BONUS3"onChange="StAllCalc()"></select> ', 0);
            myInnerHtml("EN74", '<select name="A_FOOD_BOX_BONUS4"onChange="StAllCalc()"></select> ', 0);
            myInnerHtml("EN75", '<select name="A_FOOD_BOX_BONUS5"onChange="StAllCalc()"></select> ', 0);
            myInnerHtml("EN76", '<select name="A_FOOD_BOX_BONUS6"onChange="StAllCalc()"></select> ', 0);
            myInnerHtml("EN77", '<select name="A_FOOD_BOX_BONUS7"onChange="StAllCalc()"></select> ', 0);
            myInnerHtml("EN78", '<select name="A_FOOD_BOX_BONUS8"onChange="StAllCalc()"></select> ', 0);

            document.calcForm.A_FOOD_BOX_BONUS3.options[0] = new Option("STR+Food", 0);
            document.calcForm.A_FOOD_BOX_BONUS4.options[0] = new Option("AGI+Food", 0);
            document.calcForm.A_FOOD_BOX_BONUS5.options[0] = new Option("VIT+Food", 0);
            document.calcForm.A_FOOD_BOX_BONUS6.options[0] = new Option("INT+Food", 0);
            document.calcForm.A_FOOD_BOX_BONUS7.options[0] = new Option("DEX+Food", 0);
            document.calcForm.A_FOOD_BOX_BONUS8.options[0] = new Option("LUK+Food", 0);

            for (i = 1; i <= 10; i++) {
                document.calcForm.A_FOOD_BOX_BONUS3.options[i] = new Option("+" + i, i);
                document.calcForm.A_FOOD_BOX_BONUS4.options[i] = new Option("+" + i, i);
                document.calcForm.A_FOOD_BOX_BONUS5.options[i] = new Option("+" + i, i);
                document.calcForm.A_FOOD_BOX_BONUS6.options[i] = new Option("+" + i, i);
                document.calcForm.A_FOOD_BOX_BONUS7.options[i] = new Option("+" + i, i);
                document.calcForm.A_FOOD_BOX_BONUS8.options[i] = new Option("+" + i, i);
            }

            document.calcForm.A_FOOD_BOX_BONUS0.checked = n_A_PassSkill7[0];
            document.calcForm.A_FOOD_BOX_BONUS1.checked = n_A_PassSkill7[1];
            document.calcForm.A_FOOD_BOX_BONUS2.checked = n_A_PassSkill7[2];
            document.calcForm.A_FOOD_BOX_BONUS3.value = n_A_PassSkill7[3];
            document.calcForm.A_FOOD_BOX_BONUS4.value = n_A_PassSkill7[4];
            document.calcForm.A_FOOD_BOX_BONUS5.value = n_A_PassSkill7[5];
            document.calcForm.A_FOOD_BOX_BONUS6.value = n_A_PassSkill7[6];
            document.calcForm.A_FOOD_BOX_BONUS7.value = n_A_PassSkill7[7];
            document.calcForm.A_FOOD_BOX_BONUS8.value = n_A_PassSkill7[8];
            document.calcForm.A_FOOD_BOX_BONUS9.checked = n_A_PassSkill7[2];
            document.calcForm.A_FOOD_BOX_BONUS10.checked = n_A_PassSkill7[2];
        } else {
            myInnerHtml("EN70_1", "", 0);
            myInnerHtml("EN70_2", "", 0);
            myInnerHtml("EN71_1", "", 0);
            myInnerHtml("EN71_2", "", 0);
            myInnerHtml("EN72_1", "", 0);
            myInnerHtml("EN72_2", "", 0);
            for (i = 73; i <= 78; i++)
                myInnerHtml("EN" + i, "", 0);
            myInnerHtml("EN79_1", "", 0);
            myInnerHtml("EN79_2", "", 0);
            myInnerHtml("EN710_1", "", 0);
            myInnerHtml("EN710_2", "", 0);
        }
    }


    function Click_MonsterStats() {
        MonsterStats = document.calcForm.MonsterStats.checked;

        if (MonsterStats) {
            name_ISW_SKILL = ["Provoke (Non Undead)", "Quagmire", "Poison", "Blind", "Frozen (Non Undead)", "Blessing (Demon/Undead)", "Lex Aeterna", "Stun", "Sleep", "Stone", "Curse", "Agility Down", "Signum Crucis", "Divest Weapon", "Divest Shield", "Divest Armor", "Divest Helm", "Fiber Lock", "Mind Breaker", "Slow Grace", "Down Tempo", "Power Up", "Agility Up", "Eska", "Eske", "Change Element (Monster Skill)", "Elemental Change (Sage Skill)", "Flying"];
            html_ISW_SKILL = new Array();
            for (i = 0; i <= 20; i++)
                myInnerHtml("BI" + i + "_1", name_ISW_SKILL[i], 0);
            if (InWarOfEmperium == 0) {
                for (i = 21; i <= 27; i++)
                    myInnerHtml("BI" + i + "_1", name_ISW_SKILL[i], 0);
            } else {
                myInnerHtml("BI27_1", name_ISW_SKILL[27], 0);
            }
            html_ISW_SKILL[0] = '<select name="TargetStatusFlag0"onChange="calc()"></select>';
            html_ISW_SKILL[1] = '<select name="TargetStatusFlag1"onChange="calc()"></select>';
            html_ISW_SKILL[2] = '<input type="checkbox" name="TargetStatusFlag2"onClick="calc()">';
            html_ISW_SKILL[3] = '<input type="checkbox" name="TargetStatusFlag3"onClick="calc()">';
            html_ISW_SKILL[4] = '<input type="checkbox" name="TargetStatusFlag4"onClick="calc()">';
            html_ISW_SKILL[5] = '<input type="checkbox" name="TargetStatusFlag5"onClick="calc()">';
            html_ISW_SKILL[6] = '<input type="checkbox" name="TargetStatusFlag6"onClick="calc()">';
            html_ISW_SKILL[7] = '<input type="checkbox" name="TargetStatusFlag7"onClick="calc()">';
            html_ISW_SKILL[8] = '<input type="checkbox" name="TargetStatusFlag8"onClick="calc()">';
            html_ISW_SKILL[9] = '<input type="checkbox" name="TargetStatusFlag9"onClick="calc()">';
            html_ISW_SKILL[10] = '<input type="checkbox" name="TargetStatusFlag10"onClick="calc()">';
            html_ISW_SKILL[11] = '<select name="TargetStatusFlag11"onChange="calc()"></select>';
            html_ISW_SKILL[12] = '<select name="TargetStatusFlag12"onChange="calc()"></select>';
            html_ISW_SKILL[13] = '<input type="checkbox" name="TargetStatusFlag13"onClick="calc()">';
            html_ISW_SKILL[14] = '<input type="checkbox" name="TargetStatusFlag14"onClick="calc()">';
            html_ISW_SKILL[15] = '<input type="checkbox" name="TargetStatusFlag15"onClick="calc()">';
            html_ISW_SKILL[16] = '<input type="checkbox" name="TargetStatusFlag16"onClick="calc()">';
            html_ISW_SKILL[17] = '<input type="checkbox" name="TargetStatusFlag17"onClick="calc()">';
            html_ISW_SKILL[18] = '<select name="TargetStatusFlag18"onChange="calc()"></select>';
            html_ISW_SKILL[19] = '<input type="checkbox" name="TargetStatusFlag19"onClick="calc()">';
            html_ISW_SKILL[20] = '<input type="checkbox" name="TargetStatusFlag20"onClick="calc()">';
            html_ISW_SKILL[27] = '<select name="TargetStatusFlag27"onChange="calc()"></select>';

            for (i = 0; i <= 20; i++)
                myInnerHtml("BI" + i + "_2", html_ISW_SKILL[i], 0);

            myInnerHtml("BI27_2", html_ISW_SKILL[27], 0);

            for (i = 0; i <= 10; i++) {
                document.calcForm.TargetStatusFlag0.options[i] = new Option(i, i);
                document.calcForm.TargetStatusFlag11.options[i] = new Option(i, i);
                document.calcForm.TargetStatusFlag12.options[i] = new Option(i, i);
            }
            for (i = 0; i <= 5; i++) {
                document.calcForm.TargetStatusFlag1.options[i] = new Option(i, i);
                document.calcForm.TargetStatusFlag18.options[i] = new Option(i, i);
                document.calcForm.TargetStatusFlag27.options[i] = new Option(i, i);
            }
            if (InWarOfEmperium == 0) {
                myInnerHtml("BI21_2", '<input type="checkbox" name="TargetStatusFlag21"onClick="calc()">', 0);
                myInnerHtml("BI22_2", '<input type="checkbox" name="TargetStatusFlag22"onClick="calc()">', 0);
                myInnerHtml("BI23_2", '<input type="checkbox" name="TargetStatusFlag23"onClick="calc()">', 0);
                myInnerHtml("BI24_2", '<input type="checkbox" name="TargetStatusFlag24"onClick="calc()">', 0);
                ZoHe = [["None", "Neutral 1", "Neutral 2", "Neutral 3", "Neutral 4", "Water 1", "Water 2", "Water 3", "Water 4", "Earth 1", "Eart 2", "Earth 3", "Earth 4", "Fire 1", "Fire 2", "Fire 3", "Fire 4", "Wind 1", "Wind 2", "Wind 3", "Wind 4", "Poison 1", "Poison 2", "Poison 3", "Poison 4", "Holy 1", "Holy 2", "Holy 3", "Holy 4", "Shadow 1", "Shadow 2", "Shadow 3", "Shadow 4", "Ghost 1", "Ghost 2", "Ghost 3", "Ghost 4", "Undead 1", "Undead 2", "Undead 3", "Undead 4"],
                    [0, 1, 2, 3, 4, 11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54, 61, 62, 63, 64, 71, 72, 73, 74, 81, 82, 83, 84, 91, 92, 93, 94]];
                myInnerHtml("BI25_2", '<select name="TargetStatusFlag25"onChange="calc()"></select>', 0);

                for (i = 0; i <= 40; i++)
                    document.calcForm.TargetStatusFlag25.options[i] = new Option(ZoHe[0][i], ZoHe[1][i]);
                ZoHe2 = ["None", "Water", "Earth", "Fire", "Wind"];
                myInnerHtml("BI26_2", '<select name="TargetStatusFlag26"onChange="calc()"></select>', 0);

                for (i = 0; i <= 4; i++)
                    document.calcForm.TargetStatusFlag26.options[i] = new Option(ZoHe2[i], i);
            }


            document.calcForm.TargetStatusFlag0.value = TargetStatusFlags[0];
            document.calcForm.TargetStatusFlag1.value = TargetStatusFlags[1];
            document.calcForm.TargetStatusFlag2.checked = TargetStatusFlags[2];
            document.calcForm.TargetStatusFlag3.checked = TargetStatusFlags[3];
            document.calcForm.TargetStatusFlag4.checked = TargetStatusFlags[4];
            document.calcForm.TargetStatusFlag5.checked = TargetStatusFlags[5];
            document.calcForm.TargetStatusFlag6.checked = TargetStatusFlags[6];
            document.calcForm.TargetStatusFlag7.checked = TargetStatusFlags[7];
            document.calcForm.TargetStatusFlag8.checked = TargetStatusFlags[8];
            document.calcForm.TargetStatusFlag9.checked = TargetStatusFlags[9];
            document.calcForm.TargetStatusFlag10.checked = TargetStatusFlags[10];
            document.calcForm.TargetStatusFlag11.value = TargetStatusFlags[11];
            document.calcForm.TargetStatusFlag12.value = TargetStatusFlags[12];
            document.calcForm.TargetStatusFlag13.checked = TargetStatusFlags[13];
            document.calcForm.TargetStatusFlag14.checked = TargetStatusFlags[14];
            document.calcForm.TargetStatusFlag15.checked = TargetStatusFlags[15];
            document.calcForm.TargetStatusFlag16.checked = TargetStatusFlags[16];
            document.calcForm.TargetStatusFlag17.checked = TargetStatusFlags[17];
            document.calcForm.TargetStatusFlag18.value = TargetStatusFlags[18];
            document.calcForm.TargetStatusFlag19.checked = TargetStatusFlags[19];
            document.calcForm.TargetStatusFlag20.checked = TargetStatusFlags[20];
            if (InWarOfEmperium == 0) {
                document.calcForm.TargetStatusFlag21.checked = TargetStatusFlags[21];
                document.calcForm.TargetStatusFlag22.checked = TargetStatusFlags[22];
                document.calcForm.TargetStatusFlag23.checked = TargetStatusFlags[23];
                document.calcForm.TargetStatusFlag24.checked = TargetStatusFlags[24];
                document.calcForm.TargetStatusFlag25.value = TargetStatusFlags[25];
                document.calcForm.TargetStatusFlag26.value = TargetStatusFlags[26];
            }
            document.calcForm.TargetStatusFlag27.value = TargetStatusFlags[27];
        } else {
            for (i = 0; i <= 20; i++) {
                myInnerHtml("BI" + i + "_1", "", 0);
                myInnerHtml("BI" + i + "_2", "", 0);
            }
            if (InWarOfEmperium == 0) {
                for (i = 21; i <= 26; i++) {
                    myInnerHtml("BI" + i + "_1", "", 0);
                    myInnerHtml("BI" + i + "_2", "", 0);
                }
            }
            myInnerHtml("BI27_1", "", 0);
            myInnerHtml("BI27_2", "", 0);
        }
    }

    function ClickB_Enemy() {
        targetStats = {};
        targetStatsArray = new Array();
        TARGET_STAT_MOB_INDEX = 0;
        TARGET_STAT_RACE = 2;
        TARGET_STAT_ELEMENT = 3;
        TARGET_STAT_SIZE = 4;
        TARGET_STAT_LEVEL = 5;
        TARGET_STAT_HP = 6;
        TARGET_STAT_VIT = 7;
        TARGET_STAT_AGI = 8;
        TARGET_STAT_INT = 9;
        TARGET_STAT_DEX = 10;
        TARGET_STAT_LUK = 11;
        TARGET_STAT_ATK = 12;
        TARGET_STAT_ATK2 = 13;
        TARGET_STAT_DEF = 14;
        TARGET_STAT_MDEF = 15;
        n_B2 = new Array();
        for (i = 0; i <= 22; i++) {
            targetStatsArray[i] = MonsterOBJ[document.calcForm.B_Enemy.value][i];
            n_B2[i] = targetStatsArray[i];
        }


        if (InWarOfEmperium) {
            targetStatsArray[TARGET_STAT_ELEMENT] = eval(document.calcForm.B_element.value);
            targetStatsArray[TARGET_STAT_LEVEL] = eval(document.calcForm.B_LV.value);
            targetStatsArray[TARGET_STAT_VIT] = eval(document.calcForm.B_VIT.value);
            targetStatsArray[TARGET_STAT_AGI] = eval(document.calcForm.B_AGI.value);
            targetStatsArray[TARGET_STAT_INT] = eval(document.calcForm.B_INT.value);
            targetStatsArray[TARGET_STAT_LUK] = eval(document.calcForm.B_LUK.value);
            targetStatsArray[TARGET_STAT_DEF] = eval(document.calcForm.B_DEF.value);
            targetStatsArray[TARGET_STAT_MDEF] = eval(document.calcForm.B_MDEF.value);

            JobHP_A = new Array(0, 0.7, 0.5, 0.4, 0.5, 0.3, 0.4, 1.5, 1.1, 0.75, 0.85, 0.55, 0.9, 1.1, 0.85, 0.9, 0.75, 0.75, 0.75, 0.9, 0, 1.5, 1.1, 0.75, 0.85, 0.55, 0.9, 1.1, 0.85, 0.9, 0.75, 0.75, 0.75, 0.9);
            JobHP_B = new Array(5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5, 5, 5, 5, 5, 5, 5, 5, 7, 5, 6.5, 3, 3, 5, 5);


            w = 0;
            for (i = 2; i <= targetStatsArray[TARGET_STAT_LEVEL]; i++)
                w += Math.round(JobHP_A[targetStatsArray[1]] * i);
            w = (JobHP_B[targetStatsArray[1]] * targetStatsArray[TARGET_STAT_LEVEL]) + 35 + w;

            if (targetStatsArray[1] > 20)
                w = Math.floor(w * 125 / 100);
            targetStatsArray[TARGET_STAT_HP] = Math.floor(w * (100 + targetStatsArray[TARGET_STAT_VIT]) / 100);
            targetStatsArray[TARGET_STAT_HP] += eval(document.calcForm.B_TAISEI11.value);
            targetStatsArray[TARGET_STAT_HP] = Math.floor(targetStatsArray[TARGET_STAT_HP] * (100 + eval(document.calcForm.B_TAISEI12.value)) / 100);
            myInnerHtml("B_HP", targetStatsArray[TARGET_STAT_HP], 0);


            n_B_DEF2[2] = Math.floor(targetStatsArray[TARGET_STAT_VIT] * 0.5) + Math.floor(targetStatsArray[TARGET_STAT_VIT] * 0.3);
            n_B_DEF2[0] = Math.floor(targetStatsArray[TARGET_STAT_VIT] * 0.5) + Math.floor(targetStatsArray[TARGET_STAT_VIT] * targetStatsArray[TARGET_STAT_VIT] / 150) - 1;
            if (n_B_DEF2[2] > n_B_DEF2[0])
                n_B_DEF2[0] = n_B_DEF2[2];
            w = eval(document.calcForm.B_TAISEI4.value);
            if (w) {
                n_B_DEF2[2] *= (1 + 0.05 * w);
                n_B_DEF2[0] *= (1 + 0.05 * w);
            }
            n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2);
        } else {

            n_B2[23] = targetStatsArray[TARGET_STAT_VIT];
            n_B2[24] = targetStatsArray[TARGET_STAT_VIT] + (Math.floor(targetStatsArray[TARGET_STAT_VIT] / 20) * Math.floor(targetStatsArray[TARGET_STAT_VIT] / 20) - 1);
            if (n_B2[23] > n_B2[24])
                n_B2[24] = n_B2[23];
        }
        n_B2[25] = Math.floor(targetStatsArray[TARGET_STAT_VIT] / 2) + targetStatsArray[TARGET_STAT_INT];
        n_B2[26] = targetStatsArray[TARGET_STAT_LEVEL] + targetStatsArray[TARGET_STAT_DEX];
        n_B2[27] = targetStatsArray[TARGET_STAT_LEVEL] + targetStatsArray[TARGET_STAT_AGI];

        if (MonsterStats) {
            TargetStatusFlags[0] = eval(document.calcForm.TargetStatusFlag0.value);
            TargetStatusFlags[1] = eval(document.calcForm.TargetStatusFlag1.value);
            TargetStatusFlags[2] = document.calcForm.TargetStatusFlag2.checked;
            TargetStatusFlags[3] = document.calcForm.TargetStatusFlag3.checked;
            TargetStatusFlags[4] = document.calcForm.TargetStatusFlag4.checked;
            TargetStatusFlags[5] = document.calcForm.TargetStatusFlag5.checked;
            TargetStatusFlags[6] = document.calcForm.TargetStatusFlag6.checked;
            TargetStatusFlags[7] = document.calcForm.TargetStatusFlag7.checked;
            TargetStatusFlags[8] = document.calcForm.TargetStatusFlag8.checked;
            TargetStatusFlags[9] = document.calcForm.TargetStatusFlag9.checked;
            TargetStatusFlags[10] = document.calcForm.TargetStatusFlag10.checked;
            TargetStatusFlags[11] = eval(document.calcForm.TargetStatusFlag11.value);
            TargetStatusFlags[12] = eval(document.calcForm.TargetStatusFlag12.value);
            TargetStatusFlags[13] = document.calcForm.TargetStatusFlag13.checked;
            TargetStatusFlags[14] = document.calcForm.TargetStatusFlag14.checked;
            TargetStatusFlags[15] = document.calcForm.TargetStatusFlag15.checked;
            TargetStatusFlags[16] = document.calcForm.TargetStatusFlag16.checked;
            TargetStatusFlags[17] = document.calcForm.TargetStatusFlag17.checked;
            TargetStatusFlags[18] = eval(document.calcForm.TargetStatusFlag18.value);
            TargetStatusFlags[19] = document.calcForm.TargetStatusFlag19.checked;
            TargetStatusFlags[20] = document.calcForm.TargetStatusFlag20.checked;
            if (InWarOfEmperium == 0) {
                TargetStatusFlags[21] = document.calcForm.TargetStatusFlag21.checked;
                TargetStatusFlags[22] = document.calcForm.TargetStatusFlag22.checked;
                TargetStatusFlags[23] = document.calcForm.TargetStatusFlag23.checked;
                TargetStatusFlags[24] = document.calcForm.TargetStatusFlag24.checked;
                TargetStatusFlags[25] = eval(document.calcForm.TargetStatusFlag25.value);
                TargetStatusFlags[26] = eval(document.calcForm.TargetStatusFlag26.value);
            }
            TargetStatusFlags[27] = eval(document.calcForm.TargetStatusFlag27.value);
        }

        if (TargetStatusFlags[25])
            targetStatsArray[TARGET_STAT_ELEMENT] = TargetStatusFlags[25];
        if (TargetStatusFlags[26])
            targetStatsArray[TARGET_STAT_ELEMENT] = TargetStatusFlags[26] * 10 + (targetStatsArray[TARGET_STAT_ELEMENT] % 10);

        if (TargetStatusFlags[21]) {
            targetStatsArray[TARGET_STAT_ATK] = targetStatsArray[TARGET_STAT_ATK] * 3;
            targetStatsArray[TARGET_STAT_ATK2] = targetStatsArray[TARGET_STAT_ATK2] * 3;
            targetStatsArray[TARGET_STAT_DEX] = targetStatsArray[TARGET_STAT_DEX] * 3;
        }
        if (TargetStatusFlags[22])
            targetStatsArray[TARGET_STAT_AGI] = targetStatsArray[TARGET_STAT_AGI] * 3;

        if (TargetStatusFlags[1]) {
            if (InWarOfEmperium) {
                w2 = TargetStatusFlags[1] * 5;
                w = Math.floor(targetStatsArray[TARGET_STAT_AGI] / 4);
            } else {
                w2 = TargetStatusFlags[1] * 10
                w = Math.floor(targetStatsArray[TARGET_STAT_AGI] / 2);
            }
            if (w > w2)
                targetStatsArray[TARGET_STAT_AGI] -= w2;
            else
                targetStatsArray[TARGET_STAT_AGI] -= w;
            if (InWarOfEmperium)
                w = Math.floor(targetStatsArray[TARGET_STAT_DEX] / 4);
            else
                w = Math.floor(targetStatsArray[TARGET_STAT_DEX] / 2);
            if (w > w2)
                targetStatsArray[TARGET_STAT_DEX] -= w2;
            else
                targetStatsArray[TARGET_STAT_DEX] -= w;
        }

        if (targetStatsArray[19] == 0) {
            if (TargetStatusFlags[5] && (targetStatsArray[TARGET_STAT_RACE] == 6 || targetStatsArray[TARGET_STAT_ELEMENT] >= 90)) {
                targetStatsArray[TARGET_STAT_DEX] = targetStatsArray[TARGET_STAT_DEX] - Math.floor(targetStatsArray[TARGET_STAT_DEX] / 2);
                targetStatsArray[TARGET_STAT_INT] = targetStatsArray[TARGET_STAT_INT] - Math.floor(targetStatsArray[TARGET_STAT_INT] / 2);
            }
            if (TargetStatusFlags[10]) {
                targetStatsArray[TARGET_STAT_LUK] = 0;
                targetStatsArray[TARGET_STAT_ATK] -= Math.floor(targetStatsArray[TARGET_STAT_ATK] * 25 / 100);
                targetStatsArray[TARGET_STAT_ATK2] -= Math.floor(targetStatsArray[TARGET_STAT_ATK2] * 25 / 100);
            }
            if (TargetStatusFlags[11]) {
                targetStatsArray[TARGET_STAT_AGI] -= (TargetStatusFlags[11] + 2);
                if (targetStatsArray[TARGET_STAT_AGI] < 0)
                    targetStatsArray[TARGET_STAT_AGI] = 0;
            }
        }

        if (TargetStatusFlags[15] && InWarOfEmperium == 0)
            targetStatsArray[TARGET_STAT_VIT] -= Math.floor(targetStatsArray[TARGET_STAT_VIT] * 40 / 100);
        if (TargetStatusFlags[16] && InWarOfEmperium == 0)
            targetStatsArray[TARGET_STAT_INT] -= Math.floor(targetStatsArray[TARGET_STAT_INT] * 40 / 100);

        if (InWarOfEmperium == 0) {

            targetStatsArray[23] = targetStatsArray[TARGET_STAT_VIT];
            targetStatsArray[24] = targetStatsArray[TARGET_STAT_VIT] + (Math.floor(targetStatsArray[TARGET_STAT_VIT] / 20) * Math.floor(targetStatsArray[TARGET_STAT_VIT] / 20) - 1);
            if (targetStatsArray[23] > targetStatsArray[24])
                targetStatsArray[24] = targetStatsArray[23];
        }
        targetStatsArray[25] = Math.floor(targetStatsArray[TARGET_STAT_VIT] / 2) + targetStatsArray[TARGET_STAT_INT];
        targetStatsArray[26] = targetStatsArray[TARGET_STAT_LEVEL] + targetStatsArray[TARGET_STAT_DEX];
        targetStatsArray[27] = targetStatsArray[TARGET_STAT_LEVEL] + targetStatsArray[TARGET_STAT_AGI];

        xiATK = 0;
        xiDEF = 0;
        if (targetStatsArray[19] == 0) {
            if (TargetStatusFlags[0] != 0 && targetStatsArray[TARGET_STAT_ELEMENT] < 90) {
                xiDEF += 5 + TargetStatusFlags[0] * 5;
                xiATK += 2 + TargetStatusFlags[0] * 3;
            }
            if (TargetStatusFlags[2]) {
                targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * 25 / 100);
                targetStatsArray[23] -= Math.floor(targetStatsArray[23] * 25 / 100);
                targetStatsArray[24] -= Math.floor(targetStatsArray[24] * 25 / 100);
            }
            if (TargetStatusFlags[3]) {
                targetStatsArray[26] -= 25;
                if (targetStatsArray[26] < 1)
                    targetStatsArray[26] = 1;
                targetStatsArray[27] -= Math.floor(targetStatsArray[27] * 25 / 100);
            }
        }
        if (InWarOfEmperium == 0) {
            if (TargetStatusFlags[24]) {
                xiDEF += 50;
                xiATK += 300;
            }
            if (TargetStatusFlags[27])
                xiDEF += 5 * TargetStatusFlags[27];
        }
        if (xiDEF > 100)
            xiDEF = 100;
        if (InWarOfEmperium == 0)
            targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * xiDEF / 100);
        targetStatsArray[23] -= Math.floor(targetStatsArray[23] * xiDEF / 100);
        targetStatsArray[24] -= Math.floor(targetStatsArray[24] * xiDEF / 100);
        targetStatsArray[TARGET_STAT_ATK] += Math.floor(targetStatsArray[TARGET_STAT_ATK] * xiATK / 100);
        targetStatsArray[TARGET_STAT_ATK2] += Math.floor(targetStatsArray[TARGET_STAT_ATK2] * xiATK / 100);

        if (TargetStatusFlags[13] && InWarOfEmperium == 0) {
            targetStatsArray[TARGET_STAT_ATK] -= Math.floor(targetStatsArray[TARGET_STAT_ATK] * 25 / 100);
            targetStatsArray[TARGET_STAT_ATK2] -= Math.floor(targetStatsArray[TARGET_STAT_ATK2] * 25 / 100);
        }
        if (TargetStatusFlags[14] && InWarOfEmperium == 0)
            targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * 15 / 100);
        if (targetStatsArray[19] == 0) {
            if (TargetStatusFlags[17]) {
                targetStatsArray[27] -= 50;
                if (targetStatsArray[27] < 1)
                    targetStatsArray[27] = 1;
            }

            if (TargetStatusFlags[18] && targetStatsArray[TARGET_STAT_ELEMENT] < 90)
                targetStatsArray[25] -= Math.floor(targetStatsArray[25] * (TargetStatusFlags[18] * 12) / 100);
            if (TargetStatusFlags[4] && targetStatsArray[TARGET_STAT_RACE] != 1) {
                targetStatsArray[TARGET_STAT_ELEMENT] = 11;
                targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * 50 / 100);
                targetStatsArray[23] -= Math.floor(targetStatsArray[23] * 50 / 100);
                targetStatsArray[24] -= Math.floor(targetStatsArray[24] * 50 / 100);
                targetStatsArray[TARGET_STAT_MDEF] += Math.floor(targetStatsArray[TARGET_STAT_MDEF] * 25 / 100);
                targetStatsArray[27] = -19;
            }
            if (TargetStatusFlags[7] || TargetStatusFlags[8])
                targetStatsArray[27] = -19;
            if (TargetStatusFlags[9] && targetStatsArray[TARGET_STAT_RACE] != 1) {
                targetStatsArray[TARGET_STAT_ELEMENT] = 21;
                targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * 50 / 100);
                targetStatsArray[23] -= Math.floor(targetStatsArray[23] * 50 / 100);
                targetStatsArray[24] -= Math.floor(targetStatsArray[24] * 50 / 100);
                targetStatsArray[TARGET_STAT_MDEF] += Math.floor(targetStatsArray[TARGET_STAT_MDEF] * 25 / 100);
                targetStatsArray[27] = -19;
            }
        }

        if (InWarOfEmperium == 0) {
            if (TargetStatusFlags[23]) {
                targetStatsArray[24] += 90;
                targetStatsArray[25] = 90;
            }
        }
        if (TargetStatusFlags[20]) {
            targetStatsArray[TARGET_STAT_DEF] = 0;
            targetStatsArray[23] = 0;
            targetStatsArray[24] = 0;
        }
        if (TargetStatusFlags[12] && (targetStatsArray[TARGET_STAT_RACE] == 6 || targetStatsArray[TARGET_STAT_ELEMENT] >= 90))
            targetStatsArray[TARGET_STAT_DEF] -= Math.floor(targetStatsArray[TARGET_STAT_DEF] * (10 + TargetStatusFlags[12] * 4) / 100);


        if (InWarOfEmperium == 0) {
            w1_Exp = StPlusCard(120 + targetStatsArray[TARGET_STAT_RACE]);
            w1_Exp += StPlusItem(120 + targetStatsArray[TARGET_STAT_RACE]);
            if (n_A_JobSearch() == 3 && CardNumSearch(452) && (targetStatsArray[TARGET_STAT_RACE] == 1 || targetStatsArray[TARGET_STAT_RACE] == 6))
                w1_Exp += 5;
            if (targetStatsArray[TARGET_STAT_RACE] == 2 && n_A_JobSearch() == 4 && CardNumSearch(453))
                w1_Exp += 5;
            if (w1_Exp != 0) {
                targetStatsArray[16] = Math.floor(targetStatsArray[16] * (100 + w1_Exp) / 100);
                targetStatsArray[17] = Math.floor(targetStatsArray[17] * (100 + w1_Exp) / 100);
            }
            if (targetStatsArray[19] == 0) {
                if (n_Skill3SW)
                    n_A_PassSkill3[8] = eval(document.calcForm.A_PERFORMANCE_SKILL8.value);
                if (n_A_PassSkill3[8]) {
                    targetStatsArray[16] = Math.floor(targetStatsArray[16] * (125 + 11 * n_A_PassSkill3[8]) / 100);
                    targetStatsArray[17] = Math.floor(targetStatsArray[17] * (125 + 11 * n_A_PassSkill3[8]) / 100);
                }
            }
        }

        targetStatsArray[21] = targetStatsArray[27] + 20;
        targetStatsArray[22] = targetStatsArray[26] + 75;
        if (InWarOfEmperium == 0) {
            myInnerHtml("B_AA", " + ", 0);
            myInnerHtml("B_AB", " + ", 0);
            wIJ = [6, 12, 13, 21, 22, 14, 15, 23, 25];
            wIJ2 = [16, 17];
            wFront = "<Font color='BLUE'><B>";
            wFront2 = "<Font color='RED'><B>";
            wBack = "</B></Font>";

            for (i = 0; i <= 8; i++) {
                wIJstr = targetStatsArray[wIJ[i]];
                if (targetStatsArray[wIJ[i]] < n_B2[wIJ[i]])
                    wIJstr = wFront + targetStatsArray[wIJ[i]] + wBack;
                if (targetStatsArray[wIJ[i]] > n_B2[wIJ[i]])
                    wIJstr = wFront2 + targetStatsArray[wIJ[i]] + wBack;
                myInnerHtml("B_" + wIJ[i], wIJstr, 0);
            }
            for (i = 0; i <= 1; i++) {
                wIJstr = targetStatsArray[wIJ2[i]];
                if (targetStatsArray[wIJ2[i]] < n_B2[wIJ2[i]])
                    wIJstr = wFront2 + targetStatsArray[wIJ2[i]] + wBack;
                if (targetStatsArray[wIJ2[i]] > n_B2[wIJ2[i]])
                    wIJstr = wFront + targetStatsArray[wIJ2[i]] + wBack;
                myInnerHtml("B_" + wIJ2[i], wIJstr, 0);
            }

            myInnerHtml("B_2", RaceOBJ[targetStatsArray[TARGET_STAT_RACE]], 0);
            w = Math.floor(targetStatsArray[TARGET_STAT_ELEMENT] / 10);
            if (targetStatsArray[TARGET_STAT_ELEMENT] != n_B2[3])
                myInnerHtml("B_3", wFront2 + (elementOBJ[w] + targetStatsArray[TARGET_STAT_ELEMENT] % 10) + wBack, 0);
            else
                myInnerHtml("B_3", (elementOBJ[w] + targetStatsArray[TARGET_STAT_ELEMENT] % 10), 0);
            myInnerHtml("B_4", SizeOBJ[targetStatsArray[TARGET_STAT_SIZE]], 0);
        } else {
            n_B_FLEE += eval(document.calcForm.B_TAISEI7.value);
            n_Ses = document.calcForm.B_Ses.checked;
            if (n_Ses) {
                n_B_FLEE = Math.floor(n_B_FLEE * 0.8);
            }
        }

        n_B_DEF2 = [0, 0, 0];
        n_B_DEF2[2] = targetStatsArray[23];
        n_B_DEF2[0] = targetStatsArray[24];
        n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2);
        n_B_MDEF2 = targetStatsArray[25];
        n_B_HIT = targetStatsArray[26];
        n_B_FLEE = targetStatsArray[27];
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

        if (n_A_WeaponType ==  WEAPON_TYPE_KATAR && n_A_ActiveSkill == 0)
            wBCEDPp = Math.floor(wBCEDPp * (1.01 + SkillSearch(13) * 0.02));


        if (n_A_ActiveSkill == 0) {
            wBCEDPp *= wBCEDPpDA;
            wBCEDPpHOSI *= wBCEDPpDA;
        }
        return (wBCEDPp * www / 100) + (wBCEDPpHOSI * (100 - hitRate) / 100);
    }


    function CastAndDelay() {
        if (wCast != 0) {
            wCAD = n_A_PassSkill3[2];
            if (wCAD != 0) {
                wCAD = wCAD * 3 + n_A_PassSkill3[32] + Math.floor(n_A_PassSkill3[22] / 10);
                wCast *= (100 - wCAD) / 100;
            }
            myInnerHtml("bSUBname", SubName[9], 0);
            myInnerHtml("bSUB", Math.floor(wCast * 100) / 100 + SubName[1], 0);
        }
        if (wDelay != 0) {
            if (swDelay == 1) {
                wCAD = n_A_PassSkill3[2];
                if (wDelay != "(Unknown)") {
                    wDelay = Math.floor(wDelay * (100 - (StPlusCard(ACD_PERCENTAGE) + StPlusItem(ACD_PERCENTAGE)))) / 100;
                    if (wCAD != 0) {
                        if (wCAD == 10)
                            wCAD2 = 5;
                        else
                            wCAD2 = 3;
                        wCAD = wCAD * wCAD2 + n_A_PassSkill3[32] * 2 + Math.floor(n_A_PassSkill3[29] / 5);
                        wDelay *= (100 - wCAD) / 100;
                    }
                    wDelay = Math.floor(wDelay * 100) / 100;
                    if (wCast + wDelay < eval(document.calcForm.Conf01.value) / 100)
                        wDelay = eval(document.calcForm.Conf01.value) / 100 - wCast;
                }
                myInnerHtml("bSUB2name", "Delay (Fixed Type)", 0);
                myInnerHtml("bSUB2", Math.floor(wDelay * 100) / 100 + "s", 0);
                return;
            }
            if (swDelay == 2) {
                myInnerHtml("bSUB2name", "Delay(Motion Type)", 0);
                myInnerHtml("bSUB2", wDelay + "s", 0);
            } else {
                if (n_SpSkill != 1) {
                    if (wDelay != "(�s��)")
                        wDelay = Math.floor(wDelay * 100) / 100;
                    myInnerHtml("bSUB2name", "Delay(Attack Speed Type)", 0);
                    myInnerHtml("bSUB2", wDelay + "s", 0);
                }
            }
        }
    }


}
/*
     FILE ARCHIVED ON 14:35:31 Nov 11, 2008 AND RETRIEVED FROM THE
     INTERNET ARCHIVE ON 07:19:09 Nov 20, 2023.
     JAVASCRIPT APPENDED BY WAYBACK MACHINE, COPYRIGHT INTERNET ARCHIVE.

     ALL OTHER CONTENT MAY ALSO BE PROTECTED BY COPYRIGHT (17 U.S.C.
     SECTION 108(a)(3)).
*/
/*
playback timings (ms):
  captures_list: 74.183
  exclusion.robots: 0.186
  exclusion.robots.policy: 0.17
  cdx.remote: 0.115
  esindex: 0.035
  LoadShardBlock: 46.539 (3)
  PetaboxLoader3.datanode: 60.389 (4)
  load_resource: 340.62
  PetaboxLoader3.resolve: 297.63
*/