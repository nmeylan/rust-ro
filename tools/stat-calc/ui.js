import {CalculateEnemyStats} from "./calc.js";
function myInnerHtml(wIH1, wIH2, wIH3) {
    if (wIH3 == 0) {
        let wIHOB = document.getElementById(wIH1);
        if (!wIHOB) {
            console.log("Element", wIH1, "does not exist")
            return;
        }
        while (wIHOB.hasChildNodes()) {
            wIHOB.removeChild(wIHOB.firstChild);
        }
        wIHOB.innerHTML = wIH2;
    } else {
        let wIHOB = document.getElementById(wIH1);
        wIHOB.insertAdjacentHTML('BeforeEnd', wIH2);

    }
}

console.log("setting globals to window")
for (let g in global) {
    window[g] = global[g];
}

let NameCalc = [
    "Base Stats",
    "Auto Adjust Base Level",
    "LV",
    "JobLV",
    "Class",
    "STR",
    "AGI",
    "VIT",
    "DEX",
    "INT",
    "LUK",
    "Remaining Status Points",
    "Extended Info",
    "Max HP",
    "Max SP",
    "DEF",
    "MDEF",
    "HIT",
    "FLEE",
    "Perfect Dodge",
    "Critical",
    "MATK",
    "ASPD",
    "HP Regen",
    "SP Regen",
    "Speed Potion: ",
    "Weapon Type: ",
    "Weapon & Armor / Cards",
    "Weapon: ",
    "Attribute: ",
    "Weapon: ",
    "Display",
    "Passive / Durration Skills",
    "Supportive / Party Skills",
    "Display ",
    "Attack Skills: ",
    "Monster: ",
    "HP",
    "BaseExp",
    "ATK",
    "&nbsp;- ",
    "JobExp",
    "DEF",
    "Race",
    "MDEF",
    "Element",
    "Perfect Hit",
    "Size",
    "95% Dodge",
    "Monster Status",
    "Display",
    "Combat Results",
    "Hit Ratio",
    "Dodge Ratio",
    "Minimum Damage",
    "Average Damage",
    "Maximum Damage",
    "Damage Per Second",
    "Minimum Number of Hits",
    "Average Number of Hits",
    "Maximum Number of Hits",
    "Average Battle Durration",
    "Base Exp Per Hit",
    "Job Exp Per Hit",
    "Average Damage Recieved",
    "Average Damage Recieved (w/dodge)",
    "%",
    "%",
    "Music and Dance Skills",
    "Display",
    "Guild Skills",
    "Display",
    "Battle Chant Effects",
    "Display",
    "Miscellaneous Effects",
    "Display",
    "Minimum Delay Between Active Skills",
    "Adopted",
    "Status Items (Foods/Box)",
    "Item Data",
    "Display",
];

let JobName =
    ["Novice", "Swordsman", "Thief", "Acolyte", "Archer", "Magician", "Merchant", "Knight", "Assassin", "Priest", "Hunter", "Wizard", "Blacksmith", "Crusader", "Rogue", "Monk", "Bard", "Dancer", "Sage", "Alchemist",
        "Super Novice", "LordKnight", "AssassinCross", "HighPriest", "Sniper", "HighWizard", "Whitesmith", "Paladin", "Stalker", "Champion", "Clown", "Gypsy", "Professor", "Creator",
        "High Novice", "High Swordsman", "High Thief", "High Acolyte", "High Archer", "High Magician", "High Merchant", "Taekwon Kid", "Taekwon Master", "Soul Linker", "Ninja", "Gunslinger"];



let SpeedPotName = ["None", "Concentration Potion", "Awakening Potion", "Berserk Potion"];

let EnName = ["Neutral", "Water (Sage)", "Earth (Sage)", "Fire (Sage)", "Wind (Sage)", "Poison (EP)", "Holy (Asp)", "Dark", "Ghost", "Undead"];


let WeaponName = [
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


let SubName = ["%", "sec", "Damage", "Critical Damage", "Critical Rate", "Over 1000 Hits", "Too High to Calculate", "Immesurable", "X", "Cast Time", "Off", "On"];
let JobEquipItemOBJ = [
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

let n_A_JOB, n_A_WeaponType, isRebirth, hasLeftHand, SuperNoviceFullWeaponCHECK;
let  n_ECname = new Array();
let n_A_Equip = new Array();
for (i = 0; i <= 20; i++)
    n_A_Equip[i] = 0;

let n_A_card = new Array();
for (i = 0; i <= 25; i++)
    n_A_card[i] = 0;

function ClickJob(n) {

    myInnerHtml("A_KakutyouSel", "", 0);
    myInnerHtml("A_KakutyouData", "", 0);
    document.calcForm.A_Kakutyou.value = 0;

    if (global.n_SkillSW) {
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

    let {n_A_JOB, isRebirth} = n_A_JobSet();
    n = n_A_JOB;

    document.calcForm.A_JobLV.options.length = 0;
    let w = 0;
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

    SuperNoviceFullWeaponCHECK= n_A_JOB === 20;
    if (SuperNoviceFullWeaponCHECK)
        global.JobASPD[20][7] = 120;
    else
        global.JobASPD[20][7] = 0;

    document.calcForm.A_WeaponType.options.length = 0;
    let j = 0;
    for (let i = 0; i <= 21; i++) {
        if (global.JobASPD[n][i] != 0) {
            document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i], i);
            j++;
        }
    }


    ClickWeaponType(0);


    for (i = 0; i <= 14; i++) {
        if (global.JobSkillPassOBJ[n][i] != 999) {
            myInnerHtml("P_Skill" + i, global.SkillOBJ[global.JobSkillPassOBJ[n][i]][2], 0);
            myInnerHtml("P_Skill" + i + "s", '<select name="A_PASSIVE_SKILL' + i + '"onChange="StAllCalc()"></select>', 0);
        } else {
            myInnerHtml("P_Skill" + i, "", 0);
            myInnerHtml("P_Skill" + i + "s", "", 0);
        }
    }


    /*	for(j=0;j<=14;j++){
    if(global.JobSkillPassOBJ[n][j] != 999){
        wSeOB = document.getElementById("A_PASSIVE_SKILL"+j);
        for(i=10;i>=0;i--)
            wSeOB.options[i] = null;
        for(i=0;i<=global.SkillOBJ[global.JobSkillPassOBJ[n][j]][1];i++)
            wSeOB.options[i] = new Option(i,i);
    }
}
*/
    if (global.JobSkillPassOBJ[n][0] != 999) {
        document.calcForm.A_PASSIVE_SKILL0.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][0]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][1] != 999) {
        document.calcForm.A_PASSIVE_SKILL1.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][1]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL1.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][2] != 999) {
        document.calcForm.A_PASSIVE_SKILL2.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][2]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL2.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][3] != 999) {
        document.calcForm.A_PASSIVE_SKILL3.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][3]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL3.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][4] != 999) {
        document.calcForm.A_PASSIVE_SKILL4.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][4]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL4.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][5] != 999) {
        document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][5]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][6] != 999) {
        document.calcForm.A_PASSIVE_SKILL6.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][6]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL6.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][7] != 999) {
        document.calcForm.A_PASSIVE_SKILL7.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][7]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL7.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][8] != 999) {
        document.calcForm.A_PASSIVE_SKILL8.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][8]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL8.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][9] != 999) {
        document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][9]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][10] != 999) {
        document.calcForm.A_PASSIVE_SKILL10.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][10]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL10.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][11] != 999) {
        document.calcForm.A_PASSIVE_SKILL11.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][11]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL11.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][12] != 999) {
        document.calcForm.A_PASSIVE_SKILL12.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][12]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL12.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][13] != 999) {
        document.calcForm.A_PASSIVE_SKILL13.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][13]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL13.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][14] != 999) {
        document.calcForm.A_PASSIVE_SKILL14.options.length = 0;
        for (i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][14]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL14.options[i] = new Option(i, i);
    }


    if (global.JobSkillPassOBJ[n][0] == 58) {
        document.calcForm.A_PASSIVE_SKILL0.options.length = 0;
        n_ECname = ["0", "6% Reduction", "12% Reduction", "18% Reduction", "24% Reduction", "30% Reduction"];
        for (i = 0; i <= 5; i++)
            document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(n_ECname[i], i);
    }

    if (global.JobSkillPassOBJ[n][5] == 78) {
        document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
        n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
        for (i = 0; i <= 6; i++)
            document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(n_ECname[i], i);
    }

    if (global.JobSkillPassOBJ[n][9] == 78) {
        document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
        n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
        for (i = 0; i <= 6; i++)
            document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(n_ECname[i], i);
    }


    document.calcForm.A_ActiveSkill.options.length = 0;
    for (i = 0; i <= 57 && global.JobSkillActiveOBJ[n][i] != 999; i++)
        document.calcForm.A_ActiveSkill.options[i] = new Option(global.SkillOBJ[global.JobSkillActiveOBJ[n][i]][2], global.JobSkillActiveOBJ[n][i]);


    for (i = 0; i < 20; i++)
        w_ASSP0bk[i] = 999;
    ActiveSkillSetPlus();

    ClickActiveSkill(0);
    WeaponSet2();
}

function ClickWeaponType(n) {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    let j, i = 0;
    if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n != 0)) {
        document.calcForm.A_Arrow.style.visibility = "visible";
        document.calcForm.A_Arrow.options.length = 0;
        if (n == 10 || n == 14 || n == 15) {
            j = 17;
            for (i = 0; i <= 4; i++)
                global.ArrowOBJ[i] = global.ArrowOBJbackup[i];
        } else if (n == 17 || n == 18 || n == 19 || n == 20) {
            j = 2;
            for (i = 0; i <= 2; i++)
                global.ArrowOBJ[i] = global.BulletOBJ[i];
        } else if (n == 21) {
            j = 4;
            for (i = 0; i <= 4; i++)
                global.ArrowOBJ[i] = global.GrenadeOBJ[i]
        } else {
            j = 1;
            global.ArrowOBJ[0] = [0, 0, "No Arrow"];
            global.ArrowOBJ[1] = global.ArrowOBJ[16];
        }
        for (i = 0; i <= j; i++)
            document.calcForm.A_Arrow.options[i] = new Option(global.ArrowOBJ[i][2], i);
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

    let {n_A_JOB, isRebirth} = n_A_JobSet();
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
    let n_A_ActiveSkill = eval(document.calcForm.A_ActiveSkill.value);
    let  n_A_ActiveSkillLV = 0;
    let skillToUseName = global.SkillOBJ[n_A_ActiveSkill][2];
    if (n_A_ActiveSkill > 100000) {
        n_A_ActiveSkillLV = Math.floor(n_A_ActiveSkill % 100);
        n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) / 100);
    } else
        n_A_ActiveSkillLV = global.SkillOBJ[n_A_ActiveSkill][1];

    document.calcForm.A_ActiveSkillLV.options.length = 0;
    if (n_A_ActiveSkill >= 0)
        for (i = 1; i <= n_A_ActiveSkillLV; i++)
            document.calcForm.A_ActiveSkillLV.options[i - 1] = new Option(i, i);

    if (global.SkillOBJ[n_A_ActiveSkill][1] == 1)
        document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
    else {
        document.calcForm.A_ActiveSkillLV.style.visibility = "visible";
        document.calcForm.A_ActiveSkillLV.value = n_A_ActiveSkillLV;
    }
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


function bindOnChangeEnemy() {
    let select = document.getElementById("enemy-select");
    select.addEventListener("change", (e) => {

        let enemyStats = CalculateEnemyStats(getFormData(document), global.InWarOfEmperium);
        console.log(enemyStats);

        myInnerHtml("Enemy_Race", global.RaceOBJ[enemyStats.race], 0);
        if (enemyStats.element > 0 && global.elementOBJ[enemyStats.element] === undefined){
            let w = Math.floor(enemyStats.element / 10);
            myInnerHtml("Enemy_Element", (global.elementOBJ[w] + enemyStats.element % 10), 0);
        }
        myInnerHtml("Enemy_Size", global.SizeOBJ[enemyStats.size], 0);
        myInnerHtml("Enemy_HP", enemyStats.hp, 0);
        myInnerHtml("Enemy_ATK", enemyStats.atk, 0);
        myInnerHtml("Enemy_ATK2", enemyStats.atk2, 0);
        myInnerHtml("Enemy_PerfectHit", enemyStats.perfectHit, 0);
        myInnerHtml("Enemey_PerfectDodge", enemyStats.perfectDodge, 0);
        myInnerHtml("Enemy_Def", enemyStats.def, 0);
        myInnerHtml("Enemy_Mdef", enemyStats.mdef, 0);
        myInnerHtml("Enemy_VitDef", enemyStats.vit, 0);
        myInnerHtml("Enemy_Mdef2", enemyStats.mdef2, 0);
        myInnerHtml("Enemy_RewardBaseEXP", enemyStats.exp, 0);
        myInnerHtml("Enemy_RewardJobEXP", enemyStats.jobExp, 0);
    })
}

function bindOnChangeJob() {
    let select = document.getElementById("job-select");
    select.addEventListener("change", (e) => {
        ClickJob(e.target.value)
    });
}

function bindOnChangeWeaponType() {
    let select = document.getElementById("weapon-type-select");
    select.addEventListener("change", (e) => {
        ClickWeaponType(e.target.value)
    });
}

function bindOnChangeGear() {
    document.getElementById("weapon-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("headgear-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("headgear2-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("headgear3-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("body-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shield-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shoulder-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shoes-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("accessory1-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("accessory2-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("item-sw-checkbox").addEventListener("click", (e) => {
        ClickB_Item('SW')
    });
}

function bindOnChangeCard() {
    document.getElementById("weapon1-card-1-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon1-card-2-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon1-card-3-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon1-card-4-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("headgear-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("headgear2-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("body-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shield-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shoulder-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("shoes-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("accessory1-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("accessory2-card-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
}

function bindOnChangeActiveSkill() {
    document.getElementById("active-skill-select").addEventListener("change", (e) => {
        ClickActiveSkill(e.target.value)
    });
}

function bindOnChangeStat() {
    document.getElementById("base-level-select").addEventListener("change", (e) => {
        OnChangeStat(1)
    });
    document.getElementById("str-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
    document.getElementById("agi-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
    document.getElementById("vit-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
    document.getElementById("int-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
    document.getElementById("dex-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
    document.getElementById("luk-select").addEventListener("change", (e) => {
        OnChangeStat()
    });
}

function bindOnChangeCardShortcut() {
    document.getElementById("card-shortcut-select").addEventListener("change", (e) => {
        SetCardShortcut()
    });
}
function bindOnChangeExtendedInfo() {
    document.getElementById("extended-info-select").addEventListener("change", (e) => {
        ExtendedInfo()
    });
}
function bindOnClickPerformanceSkills() {
    document.getElementById("performance-skill-checkbox").addEventListener("click", (e) => {
        Click_PerformanceSkills()
    });
}
function bindOnClickPerformance2Skills() {
    document.getElementById("performance2-skill-checkbox").addEventListener("click", (e) => {
        Click_Performance2Skills()
    });
}
function bindOnClickBattleChants() {
    document.getElementById("battle-chant-checkbox").addEventListener("click", (e) => {
        Click_BattleChant()
    });
}
function bindOnGroundSupportiveSkills() {
    document.getElementById("ground-supportive-skills-checkbox").addEventListener("click", (e) => {
        Click_GroundSupportiveSkills()
    });
}
function bindOnFoodBox() {
    document.getElementById("food-box-checkbox").addEventListener("click", (e) => {
        Click_FoodBox()
    });
}


function OnChangeStat(nSC) {
    let FORM_DATA = getFormData(document);
    let n_A_STR = FORM_DATA.A_STR;
    let n_A_AGI = FORM_DATA.A_AGI
    let n_A_VIT = FORM_DATA.A_VIT
    let n_A_DEX = FORM_DATA.A_DEX
    let n_A_INT = FORM_DATA.A_INT
    let n_A_LUK = FORM_DATA.A_LUK
    let i = 2;
        let StPoint = 0;
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

    let n_A_BaseLV = FORM_DATA.A_BaseLV

    let {n_A_JOB, isRebirth} = n_A_JobSet();
    let statusPoint = 40;
    if (isRebirth)
        statusPoint = 100;

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
        global.JobASPD[20][7] = 120;
    } else {
        SuperNoviceFullWeaponCHECK = 0;
        global.JobASPD[20][7] = 0;
    }

    document.calcForm.A_WeaponType.options.length = 0;
    j = 0;
    for (i = 0; i <= 21; i++) {
        if (global.JobASPD[20][i] != 0) {
            document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i], i);
            j++;
        }
    }
    ClickWeaponType(0);
    WeaponSet();
    WeaponSet2();
}

function sort(work) {
    for (let i = 1; work[i] != "EOF"; i++) {
        for (let k = i; k > 0; k--) {
            if (global.ItemOBJ[work[k - 1]][8] > global.ItemOBJ[work[k]][8]) {
                let work_backup = work[k - 1];
                work[k - 1] = work[k];
                work[k] = work_backup;
            }
        }
    }
    return work;
}

function WeaponSet() {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    n_A_WeaponType = eval(document.calcForm.A_WeaponType.value);
    document.calcForm.A_weapon1.options.length = 0;

    let work = new Array();
    let j = 0;
    for (let i = 0; i < global.ItemOBJ.length; i++) {
        if (global.ItemOBJ[i][1] == n_A_WeaponType && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            work[j] = i;
            j++;
        } else if (global.ItemOBJ[i][4] == 4 && global.ItemOBJ[i][1] == n_A_WeaponType && SuperNoviceFullWeaponCHECK) {
            work[j] = i;
            j++;
        }
    }
    work[j] = "EOF";


    work = sort(work);
    for (let i = 0; i < j; i++)
        document.calcForm.A_weapon1.options[i] = new Option(global.ItemOBJ[work[i]][8], global.ItemOBJ[work[i]][0]);

}

function WeaponSetLeft() {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    n_A_Weapon2Type = eval(document.calcForm.A_Weapon2Type.value);
    document.calcForm.A_weapon2.options.length = 0;
    let work = new Array();
    let j = 0;
    for (let i = 0; i < global.ItemOBJ.length; i++) {
        if (global.ItemOBJ[i][1] == n_A_Weapon2Type && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            work[j] = i;
            j++;
        }
    }
    work[j] = "EOF";
    work = sort(work);
    for (let i = 0; i < j; i++)
        document.calcForm.A_weapon2.options[i] = new Option(global.ItemOBJ[work[i]][8], global.ItemOBJ[work[i]][0]);

}


function WeaponSet2() {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    document.calcForm.A_head1.options.length = 0;
    document.calcForm.A_head2.options.length = 0;
    document.calcForm.A_head3.options.length = 0;
    document.calcForm.A_left.options.length = 0;
    document.calcForm.A_body.options.length = 0;
    document.calcForm.A_shoulder.options.length = 0;
    document.calcForm.A_shoes.options.length = 0;
    document.calcForm.A_acces1.options.length = 0;
    document.calcForm.A_acces2.options.length = 0;
    let workB = new Array();
    for (let i = 0; i <= 7; i++)
        workB[i] = new Array();
    let wsj = new Array();
    for (let i = 0; i <= 7; i++)
        wsj[i] = 0;
    for (let i = 0; i < global.ItemOBJ.length; i++) {
        if (global.ItemOBJ[i][1] == 50 && (JobEquipItemSearch(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[0][wsj[0]] = i;
            wsj[0]++;
        } else if (global.ItemOBJ[i][1] == 51 && (JobEquipItemSearch(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[1][wsj[1]] = i;
            wsj[1]++;
        } else if (global.ItemOBJ[i][1] == 52 && (JobEquipItemSearch(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[2][wsj[2]] = i;
            wsj[2]++;
        } else if (global.ItemOBJ[i][1] == 61 && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            workB[3][wsj[3]] = i;
            wsj[3]++;
        } else if (global.ItemOBJ[i][1] == 60 && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            workB[4][wsj[4]] = i;
            wsj[4]++;
        } else if (global.ItemOBJ[i][1] == 62 && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            workB[5][wsj[5]] = i;
            wsj[5]++;
        } else if (global.ItemOBJ[i][1] == 63 && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            workB[6][wsj[6]] = i;
            wsj[6]++;
        } else if (global.ItemOBJ[i][1] == 64 && JobEquipItemSearch(global.ItemOBJ[i][2]) == 1) {
            workB[7][wsj[7]] = i;
            wsj[7]++;
        }
    }
    for (let i = 0; i <= 7; i++)
        workB[i][wsj[i]] = "EOF";

    for (let m = 0; m <= 7; m++)
        workB[m] = sort(workB[m]);

    let z;
        for (let i = 0; i < wsj[0]; i++) {
        z = workB[0][i];
        document.calcForm.A_head1.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[1]; i++) {
        z = workB[1][i];
        document.calcForm.A_head2.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[2]; i++) {
        z = workB[2][i];
        document.calcForm.A_head3.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[3]; i++) {
        z = workB[3][i];
        document.calcForm.A_left.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[4]; i++) {
        z = workB[4][i];
        document.calcForm.A_body.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[5]; i++) {
        z = workB[5][i];
        document.calcForm.A_shoulder.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[6]; i++) {
        z = workB[6][i];
        document.calcForm.A_shoes.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (i = 0; i < wsj[7]; i++) {
        z = workB[7][i];
        document.calcForm.A_acces1.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
        document.calcForm.A_acces2.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
}

function JobEquipItemSearch(nJEIS) {
    if (isRebirth == 0) {
        if (global.ItemOBJ[i][11] == 200)
            return 0;
    }
    for (let nJEISi = 0; JobEquipItemOBJ[n_A_JOB][nJEISi] != 999; nJEISi++) {
        if (JobEquipItemOBJ[n_A_JOB][nJEISi] == nJEIS)
            return 1;
    }
    return 0;
}

function n_A_JobSet() {
    n_A_JOB = eval(document.calcForm.A_JOB.value);
    isRebirth = 0;
    if (21 <= n_A_JOB && n_A_JOB <= 40) {
        isRebirth = 1;
        if (34 <= n_A_JOB && n_A_JOB <= 40)
            n_A_JOB -= 34;
    } 
    return {n_A_JOB, isRebirth}
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
    let wENS = 0;
    for (ENSi = 0; ENSi <= 20; ENSi++) {
        let itemName = global.ItemOBJ[n_A_Equip[ENSi]][8];
        if (nENS === itemName)
            wENS += 1;
    }
    return wENS;
}


function CardNumSearch(nCNS) {
    let wCNS = 0;
    for (let CNSi = 0; CNSi <= 25; CNSi++) {
        let cardName = global.cardOBJ[n_A_card[CNSi]][2];
        if (nCNS === cardName)
            wCNS += 1;
    }
    return wCNS;
}


let w_ASSP0bk = new Array();
for (i = 0; i < 20; i++)
    w_ASSP0bk[i] = 999;

function ActiveSkillSetPlus() {
    let w_ASSP0 = new Array();
    let w_ASSP9 = new Array();
    let i, j2;
    let j = 0;
    for (i = 0; i < 20; i++) {
        w_ASSP0[i] = 999;
        w_ASSP9[i] = 0;
    }
    for (i = 0; i <= 20; i++) {
        for (j2 = 0; global.ItemOBJ[n_A_Equip[i]][11 + j2] != 0; j2 += 2) {
            if (global.ItemOBJ[n_A_Equip[i]][11 + j2] == 220 || global.ItemOBJ[n_A_Equip[i]][11 + j2] == 221) {
                w_ASSP0[j] = Math.floor((global.ItemOBJ[n_A_Equip[i]][12 + j2] % 100000) / 100);
                w_ASSP9[j] = global.ItemOBJ[n_A_Equip[i]][12 + j2];
                j++;
            }
        }
    }

    for (i = 0; i <= 25; i++) {
        for (j2 = 0; global.cardOBJ[n_A_card[i]][4 + j2] != 0; j2 += 2) {
            if (global.cardOBJ[n_A_card[i]][4 + j2] == 220 || global.cardOBJ[n_A_card[i]][4 + j2] == 221) {
                w_ASSP0[j] = Math.floor((global.cardOBJ[n_A_card[i]][5 + j2] % 100000) / 100);
                w_ASSP9[j] = global.cardOBJ[n_A_card[i]][5 + j2];
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

    let w_ASSPch = 0;
    for (i = 0; i < 20; i++) {
        if (w_ASSP0bk[i] != w_ASSP0[i])
            w_ASSPch = 1
    }
    if (w_ASSPch) {

        for (k = 0; global.JobSkillActiveOBJ[n_A_JOB][k] != 999; k++) ;
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
            EquipKG += global.ItemOBJ[n_A_Equip[i]][6];
        myInnerHtml("A_KakutyouData", "Weight Limit: " + syoziryou + "<BR>Total Weight of Equipment: " + EquipKG, 0);
    }
}

function ExtendedInfo() {
    let wKK = eval(document.calcForm.A_Kakutyou.value);
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

function SetCardShortcut() {
    let w = eval(document.calcForm.A_cardshort.value);
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
    let w = eval(document.calcForm.A_cardshortLeft.value);

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


function getFormData(document) {
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
    return formObject;
}

function serializeFormToJSON() {
    var formObject = getFormData(document);

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
    OnChangeStat(true);
    calc(true);
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
    if (!InTestCaseGenerationMode) {
        navigator.clipboard.writeText(JSON.stringify(savedDataAsJson));
    }
}

function aegis_item(value) {
    if (global.ItemIds[value][2].startsWith("(No")) {
        return null;
    }
    return global.ItemIds[value][2];
}

function card(value) {
    if (!value) {
        return null;
    }
    return global.CardIds[value][2]
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

    if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n_A_WeaponType != WEAPON_TYPE_UNARMED)) {
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
        testCaseData.ammo = global.ArrowOBJ[document.calcForm.A_Arrow.value][2];
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

    let {n_A_JOB, isRebirth} = n_A_JobSet();
    w = n_A_JOB;
    var saveDataIndex = 45;
    var passiveSkills = [];
    for (var i = 0; i < 15; i++) {
        if (global.JobSkillPassOBJ[w][i] == 999) break;
        let skill_level = eval(document.calcForm["A_PASSIVE_SKILL" + i].value);
        SaveData[saveDataIndex + i] = skill_level;
        if (skill_level > 0) {
            passiveSkills.push({skid: SkillOBJ[global.JobSkillPassOBJ[w][i]][3], level: skill_level})
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
    if (json.A_SUPPORTIVE_SKILLSW === "on") {
        document.calcForm.A_SUPPORTIVE_SKILLSW.checked = true;
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
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    ClickActiveSkill(json.A_ActiveSkill);


    OnChangeStat(1);
    StAllCalc();
    ActiveSkillSetPlus();
}


function LoadSave() {

    let SaveData = new Array();
    let cookieNum = "";
    for (let k = 1; k <= 19; k++) {
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

let CardShort = [
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


function cardsort(work) {
    for (var i = 1; work[i] != "NULL"; i++) {
        for (var k = i; k > 0; k--) {
            if (global.cardOBJ[work[k - 1]][2] > global.cardOBJ[work[k]][2]) {
                var  work_backup = work[k - 1];
                work[k - 1] = work[k];
                work[k] = work_backup;
            }
        }
    }
    return work;
}

function Click_Card(CBI) {
    ActiveSkillSetPlus();

    if (eval(document.calcForm.ITEM_SW.checked) == 0)
        return;
    for (i = 0; i <= 4; i++)
        myInnerHtml("ITEM" + i, "", 0);
    myInnerHtml("ITEM_W_LV", "", 0);
    myInnerHtml("ITEM_DATA", "", 0);
    myInnerHtml("ITEM_SLOT", "", 0);
    myInnerHtml("ITEM_LV", "", 0);
    myInnerHtml("ITEM_WAIT", "", 0);

    if (CBI == 106) {
        myInnerHtml("nm080", "���̂����� or ���������NTOP10", 0);
        myInnerHtml("B_SETUMEI", "���̂�����1�ŕK���_���[�W+5<BR>���̂�����3�����̏ꍇ�͕K���_���[�W+40<BR>���������NTOP10�̏ꍇ�͕K���_���[�W+10<BR>(���������NTOP10�͈�ԉE�̕���J�[�h��)", 0);
        return;
    }
    if (201 <= CBI && CBI <= 204) {
        myInnerHtml("nm080", global.cardOBJ[CBI][2], 0);
        myInnerHtml("B_SETUMEI", "���킪 " + elementOBJ[CBI - 200] + "���� �ɂȂ�B", 0);
        return;
    }
    myInnerHtml("nm080", global.cardOBJ[CBI][2] + " Card", 0);

    CBIstr = "";
    for (i = 4; global.cardOBJ[CBI][i] != 0; i += 2)
        Item_Description(global.cardOBJ[CBI], i);
    if (global.cardOBJ[CBI][3] != 0)
        CBIstr += global.cardOBJ[CBI][3] + "<BR>";

    for (i = 4; global.cardOBJ[CBI][i] != 0; i += 2) {
        if (global.cardOBJ[CBI][i] == 90) {
            CBIstr += "<Font size=2><BR><B>When " + SetCardName(global.cardOBJ[CBI][i + 1]) + " are equipped at the same time:<BR>";
            for (j = 4; global.cardOBJ[global.cardOBJ[CBI][i + 1]][j] != 0; j += 2)
                Item_Description(global.cardOBJ[global.cardOBJ[CBI][i + 1]], j);
            if (global.cardOBJ[global.cardOBJ[CBI][i + 1]][3] != 0)
                CBIstr += global.cardOBJ[global.cardOBJ[CBI][i + 1]][3] + "<BR>";
            CBIstr += "</Font></B>";
        }
    }

    myInnerHtml("B_SETUMEI", CBIstr, 0);
}

for (let i = 0; i < global.w_SC.length; i++) {
    for (let k = 1; global.w_SC[i][k] != "NULL"; k++) {
        let j;
        for (j = 4; global.cardOBJ[global.w_SC[i][k]][j] != 0; j += 2) ;
        global.cardOBJ[global.w_SC[i][k]][j] = 90;
        global.cardOBJ[global.w_SC[i][k]][j + 1] = global.w_SC[i][0];
        global.cardOBJ[global.w_SC[i][k]][j + 2] = 0;
    }
}

function SetCardName(SENw) {
    SENstr = "";
    for (SENi = 0; SENi <= SC_MAXnum; SENi++) {
        if (w_SC[SENi][0] == SENw) {
            for (SENj = 1; w_SC[SENi][SENj] != "NULL"; SENj++) {
                SENstr += "[" + global.cardOBJ[w_SC[SENi][SENj]][2] + " C]";
                if (w_SC[SENi][SENj + 1] != "NULL")
                    SENstr += " + ";
            }
            return SENstr;
        }
    }
}

function SetCard() {
    for (SEi = 16; SEi <= 25; SEi++)
        n_A_card[SEi] = 0;

    w_SE_num = 16;
    w_SE_ch = 0;
    for (SEk = 0; SEk <= SC_MAXnum; SEk++) {
        for (SEj = 1; w_SC[SEk][SEj] != "NULL" && (w_SE_ch == 1 || (w_SE_ch == 0 && SEj == 1)); SEj++) {
            w_SE_ch = 0;
            for (SEi = 0; SEi <= 15 && w_SE_ch == 0; SEi++) {
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

function ClickB_Item(CBI)
{
    const start = Date.now();
    ActiveSkillSetPlus();

    if(CBI == "SW"){
        if(eval(document.calcForm.ITEM_SW.checked)==0){
            myInnerHtml("nm080","Item Data",0);
            for(i=0;i<=4;i++)
                myInnerHtml("ITEM"+i,"",0);
            myInnerHtml("ITEM_W_LV","",0);
            myInnerHtml("ITEM_DATA","",0);
            myInnerHtml("ITEM_SLOT","",0);
            myInnerHtml("ITEM_LV","",0);
            myInnerHtml("ITEM_WAIT","",0);
            myInnerHtml("B_SETUMEI","",0);
            return;
        }else{
            CBI = eval(document.calcForm.A_head1.value);
        }
    }
    if(eval(document.calcForm.ITEM_SW.checked)==0)
        return;
    myInnerHtml("nm080",ItemOBJ[CBI][8],0);
    myInnerHtml("ITEM1","Slot",0);
    myInnerHtml("ITEM3","Min Lv",0);
    myInnerHtml("ITEM4","Weight",0);
    if(ItemOBJ[CBI][1] < 50){
        myInnerHtml("ITEM0","ATK",0);
        myInnerHtml("ITEM2","Weapon Lv",0);
        myInnerHtml("ITEM_W_LV",ItemOBJ[CBI][4],0);
    }
    else{
        myInnerHtml("ITEM0","DEF",0);
        myInnerHtml("ITEM2","-",0);
        myInnerHtml("ITEM_W_LV","-",0);
    }
    myInnerHtml("ITEM_DATA",ItemOBJ[CBI][3],0);
    myInnerHtml("ITEM_SLOT",ItemOBJ[CBI][5],0);
    myInnerHtml("ITEM_LV",ItemOBJ[CBI][7],0);
    myInnerHtml("ITEM_WAIT",ItemOBJ[CBI][6],0);

    let CBIstr = "";
    for(i=11;ItemOBJ[CBI][i] != 0;i+=2)
        Item_Description(ItemOBJ[CBI],i);
    if(ItemOBJ[CBI][10] != 0)
        CBIstr += ItemOBJ[CBI][10] +"<BR>";

    for(i=11;ItemOBJ[CBI][i] != 0;i+=2){
        if(ItemOBJ[CBI][i] == 90){

            CBIstr += "<Font size=2><BR><B>When "+ SetEquipName(ItemOBJ[CBI][i+1]) + " are equipped at the same time:<BR>";
            for(j=11;ItemOBJ[ItemOBJ[CBI][i+1]][j] != 0;j+=2)
                Item_Description(ItemOBJ[ItemOBJ[CBI][i+1]],j);
            if(ItemOBJ[ItemOBJ[CBI][i+1]][10] != 0)
                CBIstr += ItemOBJ[ItemOBJ[CBI][i+1]][10] +"<BR>";
            CBIstr += "</Font></B>";
        }
    }
    myInnerHtml("B_SETUMEI",CBIstr,0);
}

function Item_Description(num, CBI2)
{
    const start = Date.now();
    wNAME1 = ["0","STR","AGI","VIT","INT","DEX","LUK","ALL_STATS","HIT","FLEE","CRIT","PERFECT_DODGE","ASPD","MHP","MSP","MHP","MSP","ATK","DEF","MDEF"];
    wIS = " + ";
    CBIstr = "";
    var stat = "";
    var stat2 = "";
    if(num[CBI2+1] < 0)
        wIS = " ";

    if(1 <= num[CBI2] && num[CBI2] <=11){
        CBIstr += wNAME1[num[CBI2]] + wIS + num[CBI2+1] +"<BR>";
        stat = wNAME1[num[CBI2]];
    }
    if(12 == num[CBI2]) {
        CBIstr += wNAME1[num[CBI2]] + wIS + num[CBI2 + 1] + "%<BR>";
        stat = wNAME1[num[CBI2]] + "_PERCENTAGE";
    }
    if(13 <= num[CBI2] && num[CBI2] <=14) {
        CBIstr += wNAME1[num[CBI2]] + wIS + num[CBI2 + 1] + "<BR>";
        stat = wNAME1[num[CBI2]];
    }
    if(15 <= num[CBI2] && num[CBI2] <=16) {
        CBIstr += wNAME1[num[CBI2]] + wIS + num[CBI2 + 1] + "%<BR>";
        stat = wNAME1[num[CBI2]] + "_PERCENTAGE";
    }
    if(17 <= num[CBI2] && num[CBI2] <=19) {
        CBIstr += wNAME1[num[CBI2]] + wIS + num[CBI2 + 1] + "<BR>";
        stat = wNAME1[num[CBI2]];
    }
    if(20 == num[CBI2]) {
        CBIstr += elementOBJ[num[CBI2 + 1]] + " Element Weapon.<BR>";
        stat = "ELEMENT_WEAPON";
    }
    if(22 == num[CBI2]){
        stat = "BYPASS_DEFENSE_ON_RACE"
        if(num[CBI2+1] != 99)
            CBIstr += "Bypasses defence on " + RaceOBJ[num[CBI2+1]] + " monsters.<BR>";
        else
            CBIstr += "Completely bypasses defence on the target.<BR>";
    }
    if(23 == num[CBI2]) {
        stat = "WEAPON_ATK_INCREASE_ON_TARGET_DEFENSE";
        CBIstr += "Attack power of the weapon increases against enemies with high VIT and defence.<BR>";
    }
    if(24 == num[CBI2]) {
        stat = "REDUCE_DEFENSE";
        CBIstr += "Reduces your defence by 1/" + num[CBI2 + 1] + ".<BR>";
    }
    if(25 == num[CBI2]) {
        stat = "REDUCE_DEFENSE_PERCENTAGE";
        CBIstr += "Increases ranged damage by " + num[CBI2 + 1] + "%.<BR>";
    }
    if(26 == num[CBI2]) {
        stat = "INCREASE_DAMAGE_AGAINST_BOSS_PERCENTAGE";
        CBIstr += "Increases damage against boss type monsters + " + num[CBI2 + 1] + "% damage.<BR>";
    }
    if(27 <= num[CBI2] && num[CBI2] <=29) {
        stat = "INCREASE_DAMAGE_AGAINST_SIZE_PERCENTAGE";
        CBIstr += "Increases damage against " + SizeOBJ[num[CBI2] - 27] + " size monsters by " + num[CBI2 + 1] + "%.<BR>";
    }
    if(30 <= num[CBI2] && num[CBI2] <=39) {
        stat = "INCREASE_DAMAGE_RACE_PERCENTAGE";
        CBIstr += "Increases damage against " + RaceOBJ[num[CBI2] - 30] + " type monsters by " + num[CBI2 + 1] + "%.<BR>";
    }
    if(40 <= num[CBI2] && num[CBI2] <=49) {
        stat = "INCREASE_DAMAGE_ELEMENT_"+elementOBJ[num[CBI2] - 40].toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 40) {
            stat2 = "INCREASE_DAMAGE_ELEMENT_PERCENTAGE";
        }
        CBIstr += "Increases damage against " + elementOBJ[num[CBI2] - 40] + " element monsters by " + num[CBI2 + 1] + "%.<BR>";
    }
    if(50 <= num[CBI2] && num[CBI2] <=59){
        stat = "DAMAGE_INC_DEC_RACE_"+RaceOBJ[num[CBI2]-50].toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 50) {
            stat2 = "DAMAGE_INC_DEC_RACE_PERCENTAGE";
        }
        if(num[CBI2+1] > 0)
            CBIstr += "Decreases damage from " + RaceOBJ[num[CBI2]-50] +" type monsters by "+ num[CBI2+1] +"%.<BR>";
        else
            CBIstr += "Increases damage from " + RaceOBJ[num[CBI2]-50] +" type monsters by "+ (-1 * num[CBI2+1]) +"%.<BR>";
    }
    if(60 <= num[CBI2] && num[CBI2] <=69){
        stat = "DAMAGE_INC_DEC_ELEMENT_" + elementOBJ[num[CBI2]-60].toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 60) {
            stat2 = "DAMAGE_INC_DEC_ELEMENT_PERCENTAGE";
        }
        if(num[CBI2+1] < 0)
            CBIstr += "Decreases resistance to " + elementOBJ[num[CBI2]-60] +" element attacks by "+ wIS + num[CBI2+1] +"%.<BR>";
        else
            CBIstr += "Increases resistance to " + elementOBJ[num[CBI2]-60] +" element attacks by "+ wIS + num[CBI2+1] +"%.<BR>";
    }
    if(70 == num[CBI2]) {
        stat = "CRITICAL_DAMAGE_PERCENTAGE";
        CBIstr += "Critical Damage + " + num[CBI2 + 1] + "%<BR>";
    }
    if(73 == num[CBI2]) {
        stat = "CAST_TIME_PERCENTAGE";
        CBIstr += "Cast Time" + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(74 == num[CBI2]) {
        stat = "ACD_PERCENTAGE";
        CBIstr += "After cast delay - " + num[CBI2 + 1] + "%<BR>";
    }
    if(75 == num[CBI2]) {
        stat = "HP_REGEN_PERCENTAGE";
        CBIstr += "HP Regen" + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(76 == num[CBI2]) {
        stat = "SP_REGEN_PERCENTAGE";
        CBIstr += "SP Regen" + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(78 == num[CBI2]) {
        stat = "RESISTANCE_RANGE_ATTACK_PERCENTAGE";
        CBIstr += "Adjusts your resistance to ranged attacks by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(79 == num[CBI2]) {
        stat = "NORMAL_ATTACK_PERCENTAGE";
        CBIstr += "Adjusts your resistance to normal monsters by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(81 == num[CBI2]) {
        stat = "INCREASE_DAMAGE_GOBLIN_PERCENTAGE";
        CBIstr += "Increases damage on goblin monsters by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(82 == num[CBI2]) {
        stat = "INCREASE_DAMAGE_KOBOLD_PERCENTAGE";
        CBIstr += "Increases damage on kobold monsters by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(83 == num[CBI2]) {
        stat = "INCREASE_DAMAGE_ORC_PERCENTAGE";
        CBIstr += "Increases damage on orc monsters (with the exception of Orc Lord and Orc Hero) by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(84 == num[CBI2]) {
        stat = "INCREASE_DAMAGE_GOLEM_PERCENTAGE";
        CBIstr += "Increases damage on golem monsters by " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(85 == num[CBI2]) {
        stat = "LOWER_DEFENCE_PERCENTAGE";
        CBIstr += "Lowers your defence rate by " + num[CBI2 + 1] + "%.<BR>";
    }
    if(86 == num[CBI2]) {
        stat = "INCREASE_HIT_PERCENTAGE";
        CBIstr += "Increases your chance to hit all targets by a fixed " + num[CBI2 + 1] + "%.<BR>";
    }
    if(87 == num[CBI2]) {
        stat = "ATK_PERCENTAGE";
        CBIstr += "ATK" + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(88 == num[CBI2]) {
        stat = "MATK_BASED_ON_STAFF_PERCENTAGE";
        CBIstr += "MATK" + wIS + num[CBI2 + 1] + "% (Staff Type)<BR>";
    }
    if(89 == num[CBI2]) {
        stat = "MATK_PERCENTAGE";
        CBIstr += "MATK" + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(110 <= num[CBI2] && num[CBI2] <=119) {
        stat = "CRITICAL_AGAINST_RACE_"+RaceOBJ[num[CBI2] - 110].toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 110) {
            stat2 = "CRITICAL_AGAINST_RACE_PERCENTAGE";
        }
        CBIstr += "Increases critical rate against " + RaceOBJ[num[CBI2] - 110] + "type monsters by " + wIS + num[CBI2 + 1] + "<BR>";
    }
    if(120 <= num[CBI2] && num[CBI2] <=129) {
        CBIstr += "Experience obtained from " + RaceOBJ[num[CBI2] - 120] + " type monsters " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    if(130 <= num[CBI2] && num[CBI2] <=149) {
        if (!StatusOBJ[num[CBI2] - 130] ){
            return
        }
        stat = "CHANCE_TO_INFLICT_STATUS_"+StatusOBJ[num[CBI2] - 130] .toUpperCase()+"_ON_ATTACK";
        if (num[CBI2] === 130) {
            stat2 = "CHANCE_TO_INFLICT_STATUS_ON_ATTACK";
        }
        CBIstr += "When attacking, adds a " + num[CBI2 + 1] + "% chance to inflict [" + StatusOBJ[num[CBI2] - 130] + "] on the enemy.<BR>";
    }
    if(150 <= num[CBI2] && num[CBI2] <=169) {
        if (!StatusOBJ[num[CBI2] - 150] ){
            return
        }
        stat = "RESISTANCE_TO_STATUS_"+StatusOBJ[num[CBI2] - 150] .toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 150) {
            stat2 = "RESISTANCE_TO_STATUS_PERCENTAGE";
        }
        CBIstr += "Status effect [" + StatusOBJ[num[CBI2] - 150] + "] resistance +" + num[CBI2 + 1] + "%<BR>";
    }
    if(190 <= num[CBI2] && num[CBI2] <=192){
        stat = "DAMAGE_INC_DEC_SIZE_"+SizeOBJ[num[CBI2]-190] .toUpperCase()+"_PERCENTAGE";
        if (num[CBI2] === 190) {
            stat2 = "DAMAGE_INC_DEC_SIZE_PERCENTAGE";
        }
        if(num[CBI2+1] > 0)
            CBIstr += "Decreases damage from " + SizeOBJ[num[CBI2]-190] +" size monsters by "+ num[CBI2+1] +"%.<BR>";
        else
            CBIstr += "Increases damage from " + SizeOBJ[num[CBI2]-190] +" size monsters by "+ (-1 * num[CBI2+1]) +"%.<BR>";
    }
    if(193 == num[CBI2]) {
        CBIstr += "<Font color='#FF0000'>Unrefinable.</Font><BR>";
    }
    if(194 == num[CBI2]) {
        CBIstr += "Cannot be broken.<BR>";
    }
    if(195 == num[CBI2]) {
        CBIstr += "Two-Handed Staff.<BR>";
    }
    if(198 == num[CBI2]) {
        CBIstr += "Armor becomes " + elementOBJ[num[CBI2 + 1]] + " element.<BR>";
    }
    if(212 <= num[CBI2] && num[CBI2] <= 215) {
        CBIstr += wNAME1[num[CBI2] - 210] + wIS + num[CBI2 + 1] + "<BR>";
    }
    if(220 == num[CBI2] || 230 == num[CBI2]) {
        CBIstr += "Allows usage fo the skill [" + SkillOBJ[Math.floor((num[CBI2 + 1] - 100000) / 100)][2] + "] Lv " + Math.floor((num[CBI2 + 1] - 100000) % 100) + ".<BR>";
    }
    if(221 == num[CBI2] || 231 == num[CBI2]){
        wNAME99 = [0,"When performing a physical attack, ","When performing a short range physical attack, ","When performing a long range physical attack, ","When performing a magical attack, ","When attacking, ","When recieving physical damage, ","When recieving short range physical damage, ","When recieving long range physical damage, ","When recieving magical damage, ","When recieving physical or magical damage, "];
        wNAME98 = ["low","fixed","high"];
        CBIstr += wNAME99[Math.floor(num[CBI2+1] / 10000000)] +"there is a ";
        if(Math.floor((num[CBI2+1] % 10000000) / 100000) >= 97)
            CBIstr += wNAME98[Math.floor((num[CBI2+1] % 10000000) / 100000)-97];
        else
            CBIstr += Math.floor((num[CBI2+1] % 10000000) / 100000) + "%";
        CBIstr += " chance to cast the skill ["+ SkillOBJ[Math.floor((num[CBI2+1] % 100000)/100)][2] +"] Lv "+ Math.floor((num[CBI2+1] % 100000)%100) +".<BR>";
    }
    if(1000 <= num[CBI2] && num[CBI2] <= 2999) {
        CBIstr += "Increases damage against the monster " + MonsterOBJ[num[CBI2] - 1000][1] + " by " + wIS + num[CBI2 + 1] + "%.<BR>";
    }
    if(3000 <= num[CBI2] && num[CBI2] <=4999){
        if(num[CBI2+1] > 0)
            CBIstr += "Reduces damage from the monster " + MonsterOBJ[num[CBI2]-3000][1] +" by "+ num[CBI2+1] +"%.<BR>";
        else
            CBIstr += "Increases damage recieved from the monster " + MonsterOBJ[num[CBI2]-3000][1] +" by "+ (-1 * num[CBI2+1]) +"%.<BR>";
    }
    if(5000 <= num[CBI2] && num[CBI2] <= 6999) {
        CBIstr += SkillOBJ[num[CBI2] - 5000][2] + "'s damage " + wIS + num[CBI2 + 1] + "%<BR>";
    }
    // console.log(CBIstr)

    // var stat = [];
    // for (var i = 0; i < 1000; i++) {
    // 	stat.push(i);
    // }
    // for (var i = 0; i < 1000; i++) {
    // 	var a = Item_Description(stat, i);
    // 	if (a && a !== "") {
    // 		console.log(i, a.replace("<BR>", ""));
    // 	}
    // }

    return [stat, stat2];
}
function SetEquipName(SENw){
    const start = Date.now();
    SENstr = "";
    for(SENi=0;SENi<=SE_MAXnum;SENi++){
        if(w_SE[SENi][0] == SENw){
            for(SENj=1;w_SE[SENi][SENj] != "NULL";SENj++){
                SENstr += "["+ ItemOBJ[w_SE[SENi][SENj]][8] +"]";
                if(w_SE[SENi][SENj+1] != "NULL")
                    SENstr += " + ";
            }
            return SENstr;
        }
    }
    console.log("SetEquipName end. Took", Date.now() - start, "ms")
}



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

for(i=1;i<=81;i++)
    myInnerHtml("nm0"+i,NameCalc[i-1],0);


for (i = 0; i <= 45; i++)
    document.calcForm.A_JOB.options[i] = new Option(JobName[i], i);

for (i = 0; i <= 16; i++)
    document.calcForm.A_Arrow.options[i] = new Option(global.ArrowOBJ[i][2], i);

for (i = 0; i <= 9; i++)
    document.calcForm.A_Weapon_element.options[i] = new Option(EnName[i], i);

document.calcForm.A_SpeedPOT.options[0] = new Option(SpeedPotName[0], 0);
document.calcForm.A_SpeedPOT.options[1] = new Option(SpeedPotName[1], 1);
document.calcForm.A_SpeedPOT.options[2] = new Option(SpeedPotName[2], 2);
document.calcForm.A_SpeedPOT.options[3] = new Option(SpeedPotName[3], 3);


cardsort(global.CardSortOBJ[0]);
cardsort(global.CardSortOBJ[1]);
cardsort(global.CardSortOBJ[2]);
cardsort(global.CardSortOBJ[3]);
cardsort(global.CardSortOBJ[4]);
cardsort(global.CardSortOBJ[5]);
cardsort(global.CardSortOBJ[6]);
cardsort(global.CardSortOBJ[7]);

for (var i = 0; global.CardSortOBJ[0][i] != "NULL"; i++)
    document.calcForm.A_weapon1_card1.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[0][i]][2], global.cardOBJ[global.CardSortOBJ[0][i]][0]);
for (var i = 0; global.CardSortOBJ[1][i] != "NULL"; i++) {
    document.calcForm.A_weapon1_card2.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[1][i]][2], global.cardOBJ[global.CardSortOBJ[1][i]][0]);
    document.calcForm.A_weapon1_card3.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[1][i]][2], global.cardOBJ[global.CardSortOBJ[1][i]][0]);
    document.calcForm.A_weapon1_card4.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[1][i]][2], global.cardOBJ[global.CardSortOBJ[1][i]][0]);
}
document.calcForm.A_weapon1_card4.options[1] = new Option("Top 10 Ranked", 106);

for (var i = 0; global.CardSortOBJ[2][i] != "NULL"; i++) {
    document.calcForm.A_head1_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[2][i]][2], global.cardOBJ[global.CardSortOBJ[2][i]][0]);
    document.calcForm.A_head2_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[2][i]][2], global.cardOBJ[global.CardSortOBJ[2][i]][0]);
}
for (var i = 0; global.CardSortOBJ[3][i] != "NULL"; i++)
    document.calcForm.A_left_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[3][i]][2], global.cardOBJ[global.CardSortOBJ[3][i]][0]);
for (var i = 0; global.CardSortOBJ[4][i] != "NULL"; i++)
    document.calcForm.A_body_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[4][i]][2], global.cardOBJ[global.CardSortOBJ[4][i]][0]);
for (var i = 0; global.CardSortOBJ[5][i] != "NULL"; i++)
    document.calcForm.A_shoulder_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[5][i]][2], global.cardOBJ[global.CardSortOBJ[5][i]][0]);
for (var i = 0; global.CardSortOBJ[6][i] != "NULL"; i++)
    document.calcForm.A_shoes_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[6][i]][2], global.cardOBJ[global.CardSortOBJ[6][i]][0]);
for (var i = 0; global.CardSortOBJ[7][i] != "NULL"; i++) {
    document.calcForm.A_acces1_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[7][i]][2], global.cardOBJ[global.CardSortOBJ[7][i]][0]);
    document.calcForm.A_acces2_card.options[i] = new Option(global.cardOBJ[global.CardSortOBJ[7][i]][2], global.cardOBJ[global.CardSortOBJ[7][i]][0]);
}


for (i = 0; i <= 38; i++)
    document.calcForm.A_cardshort.options[i] = new Option(CardShort[i][0], i);

let sortedMonster = global.MonsterOBJ.sort((a, b) => {
    if (a[1] < b[1]) {
        return -1;
    }
    if (a[1] > b[1]) {
        return 1;
    }
})
for (i = 0; i < sortedMonster.length; i++)
    document.calcForm.B_Enemy.options[i] = new Option(sortedMonster[i][1], sortedMonster[i][0]);

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

document.calcForm.A_JOB.value = 0;
ClickJob(0);
LoadSave();

bindOnChangeEnemy();
bindOnChangeJob();
bindOnChangeWeaponType();
bindOnChangeGear();
bindOnChangeCard();
bindOnChangeActiveSkill();
bindOnChangeStat();
bindOnChangeCardShortcut();
bindOnChangeExtendedInfo();