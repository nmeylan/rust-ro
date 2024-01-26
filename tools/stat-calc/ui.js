import {CalculateAllStats, CalculateBattle, CalculateEnemyStats, GetTestCase} from "./calc.js";

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
    "Element: ",
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

let n_A_JOB, n_A_WeaponType, n_A_Weapon2Type, isRebirth, SuperNoviceFullWeaponCHECK, loadedTestCase, testcases;
let n_ECname = new Array();
let n_A_Equip = new Array();
for (i = 0; i <= 20; i++)
    n_A_Equip[i] = 0;

let n_A_card = new Array();
for (i = 0; i <= 25; i++)
    n_A_card[i] = 0;

function bindSearchable(selectElement) {
    var wrapperDiv = document.createElement("div");
    wrapperDiv.classList.add("searchable-select-wrapper");
    selectElement.parentNode.insertBefore(wrapperDiv, selectElement);
    wrapperDiv.appendChild(selectElement);

    var inputElement = document.createElement("input");
    inputElement.type = "text";
    inputElement.placeholder = "Search...";
    inputElement.hidden = true;
    wrapperDiv.insertBefore(inputElement, selectElement);
    selectElement.addEventListener("focus", function () {
        selectElement.size = 0;
        inputElement.hidden = false;
        inputElement.focus()
    })
    selectElement.addEventListener("click", function () {
        inputElement.hidden = true;
        inputElement.blur();
        selectElement.dispatchEvent(new Event('change'));
    })
    selectElement.dataset.size = selectElement.options.length;

    inputElement.addEventListener("input", function() {
        var searchText = inputElement.value.toLowerCase();
        var options = selectElement.options;

        let hiddenCount = 0;
        for (var i = 0; i < options.length; i++) {
            var optionText = options[i].text.toLowerCase();
            if (optionText.includes(searchText)) {
                options[i].style.display = "";
            } else {
                options[i].style.display = "none";
                hiddenCount +=1;
            }
        }

        selectElement.size = options.length - hiddenCount + 1;
        selectElement.dataset.size = selectElement.size;
    });
    inputElement.addEventListener("keyup", function(event) {
        var searchText = inputElement.value.toLowerCase();
        var options = selectElement.options;
        let hiddenCount = 0;
        let matchIndex = -1;
        let includingItems = [];
        for (var i = 0; i < options.length; i++) {
            var optionText = options[i].text.toLowerCase();
            if (optionText === searchText) {
                matchIndex = i;
            }
            else if (optionText.includes(searchText)) {
                includingItems.push(i);
            }
            options[i].style.backgroundColor = "";
        }
        if (matchIndex >= 0) {
            options[matchIndex].style.backgroundColor = "#999";
            if (event.key === "Enter") {
                selectElement.selectedIndex = matchIndex;
                selectElement.dispatchEvent(new Event('click'));
            }
        } else if (includingItems.length === 1) {
            options[includingItems[0]].style.backgroundColor = "#999";
            if (event.key === "Enter") {
                selectElement.selectedIndex = includingItems[0];
                selectElement.dispatchEvent(new Event('click'));
            }
        }
    });
    inputElement.addEventListener("blur", function() {
        selectElement.size = 0;
        inputElement.hidden = true;
        selectElement.style.top = 0;
    });

    // Show the select options when the input gains focus
    inputElement.addEventListener("focus", function() {
        selectElement.style.top = 25;
        selectElement.size = selectElement.dataset.size;
    });
}

function ClickJob(n) {

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

    let {n_A_JOB, isRebirth} = n_A_JobSet();
    n = n_A_JOB;

    document.calcForm.A_JobLV.options.length = 0;
    let w = 0;
    if (n == 0) w = 10;
    else if (n <= 19 || (41 <= n && n <= 43)) w = 50;
    else if (n == 20) w = 71;
    else w = 70;
    for (let i = 1; i <= w; i++)
        document.calcForm.A_JobLV.options[i - 1] = new Option(i, i);
    if (n == 20) {
        document.calcForm.A_JobLV.options[69] = new Option("70-99", 70);
        document.calcForm.A_JobLV.options[70] = new Option("+10", 71);
    }

    SuperNoviceFullWeaponCHECK = n_A_JOB === 20;
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


    for (let i = 0; i <= 14; i++) {
        document.calcForm["A_PASSIVE_SKILL" + i].value = 0;
        if (global.JobSkillPassOBJ[n][i] != 999) {
            document.getElementById("P_Skill" + i + "s").hidden = false;
            myInnerHtml("P_Skill" + i, global.SkillOBJ[global.JobSkillPassOBJ[n][i]][2], 0);
       } else {
            myInnerHtml("P_Skill" + i, "", 0);
            document.getElementById("P_Skill" + i + "s").hidden = true;
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
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][0]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][1] != 999) {
        document.calcForm.A_PASSIVE_SKILL1.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][1]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL1.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][2] != 999) {
        document.calcForm.A_PASSIVE_SKILL2.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][2]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL2.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][3] != 999) {
        document.calcForm.A_PASSIVE_SKILL3.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][3]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL3.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][4] != 999) {
        document.calcForm.A_PASSIVE_SKILL4.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][4]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL4.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][5] != 999) {
        document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][5]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][6] != 999) {
        document.calcForm.A_PASSIVE_SKILL6.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][6]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL6.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][7] != 999) {
        document.calcForm.A_PASSIVE_SKILL7.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][7]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL7.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][8] != 999) {
        document.calcForm.A_PASSIVE_SKILL8.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][8]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL8.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][9] != 999) {
        document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][9]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][10] != 999) {
        document.calcForm.A_PASSIVE_SKILL10.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][10]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL10.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][11] != 999) {
        document.calcForm.A_PASSIVE_SKILL11.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][11]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL11.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][12] != 999) {
        document.calcForm.A_PASSIVE_SKILL12.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][12]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL12.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][13] != 999) {
        document.calcForm.A_PASSIVE_SKILL13.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][13]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL13.options[i] = new Option(i, i);
    }
    if (global.JobSkillPassOBJ[n][14] != 999) {
        document.calcForm.A_PASSIVE_SKILL14.options.length = 0;
        for (let i = 0; i <= global.SkillOBJ[global.JobSkillPassOBJ[n][14]][1]; i++)
            document.calcForm.A_PASSIVE_SKILL14.options[i] = new Option(i, i);
    }


    if (global.JobSkillPassOBJ[n][0] == 58) {
        document.calcForm.A_PASSIVE_SKILL0.options.length = 0;
        let n_ECname = ["0", "6% Reduction", "12% Reduction", "18% Reduction", "24% Reduction", "30% Reduction"];
        for (let i = 0; i <= 5; i++)
            document.calcForm.A_PASSIVE_SKILL0.options[i] = new Option(n_ECname[i], i);
    }

    if (global.JobSkillPassOBJ[n][5] == 78) {
        document.calcForm.A_PASSIVE_SKILL5.options.length = 0;
        let n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
        for (let i = 0; i <= 6; i++)
            document.calcForm.A_PASSIVE_SKILL5.options[i] = new Option(n_ECname[i], i);
    }

    if (global.JobSkillPassOBJ[n][9] == 78) {
        document.calcForm.A_PASSIVE_SKILL9.options.length = 0;
        let n_ECname = ["No Peco", "Mastery 0", "Mastery 1", "Mastery 2", "Mastery 3", "Mastery 4", "Mastery 5"];
        for (let i = 0; i <= 6; i++)
            document.calcForm.A_PASSIVE_SKILL9.options[i] = new Option(n_ECname[i], i);
    }


    document.calcForm.A_ActiveSkill.options.length = 0;
    for (let i = 0; i <= 57 && global.JobSkillActiveOBJ[n][i] != 999; i++)
        document.calcForm.A_ActiveSkill.options[i] = new Option(global.SkillOBJ[global.JobSkillActiveOBJ[n][i]][2], global.JobSkillActiveOBJ[n][i]);


    for (let i = 0; i < 20; i++)
        w_ASSP0bk[i] = 999;
    ActiveSkillSetPlus();

    ClickActiveSkill(0);
    ItemSet();
}

function ClickWeaponType(n) {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    let j, i = 0;
    if (n_A_JobSearch() == 2 || n_A_JobSearch() == 4 || (n_A_JOB == 45 && n != 0)) {
        document.calcForm.A_Arrow.style.visibility = "visible";
        document.calcForm.A_Arrow.options.length = 0;
        if (n == 10 || n == 14 || n == 15) {
            j = 17;
            for (let i = 0; i <= 4; i++)
                global.ArrowOBJ[i] = global.ArrowOBJbackup[i];
        } else if (n == 17 || n == 18 || n == 19 || n == 20) {
            j = 2;
            for (let i = 0; i <= 2; i++)
                global.ArrowOBJ[i] = global.BulletOBJ[i];
        } else if (n == 21) {
            j = 4;
            for (let i = 0; i <= 4; i++)
                global.ArrowOBJ[i] = global.GrenadeOBJ[i]
        } else {
            j = 1;
            global.ArrowOBJ[0] = [0, 0, "No Arrow"];
            global.ArrowOBJ[1] = global.ArrowOBJ[16];
        }
        for (let i = 0; i <= j; i++)
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
        document.querySelectorAll("[data-left-hand=true]").forEach(el => el.hidden = false)
    } else {
        document.querySelectorAll("[data-left-hand=true]").forEach(el => {
            el.hidden = true;
            el.querySelectorAll("select").forEach(select => select.value = 0)
        })
    }
    n_A_Equip[0] = eval(document.calcForm.A_weapon1.value);
    ActiveSkillSetPlus();
}


function ClickWeaponType2(n) {
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    if (n != 0) {
        if (n_A_JOB == 8 || n_A_JOB == 22) {
            WeaponSetLeft();
            n_A_Equip[1] = eval(document.calcForm.A_weapon2.value);
            ActiveSkillSetPlus();
        }
    }
}

function ClickActiveSkill(wAS) {
    let n_A_ActiveSkill = eval(document.calcForm.A_ActiveSkill.value);
    let n_A_ActiveSkillLV = 0;
    let skillToUseName = global.SkillOBJ[n_A_ActiveSkill][2];
    if (n_A_ActiveSkill > 100000) {
        n_A_ActiveSkillLV = Math.floor(n_A_ActiveSkill % 100);
        n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) / 100);
    } else
        n_A_ActiveSkillLV = global.SkillOBJ[n_A_ActiveSkill][1];

    document.calcForm.A_ActiveSkillLV.options.length = 0;
    if (n_A_ActiveSkill >= 0)
        for (let i = 1; i <= n_A_ActiveSkillLV; i++)
            document.calcForm.A_ActiveSkillLV.options[i - 1] = new Option(i, i);

    if (global.SkillOBJ[n_A_ActiveSkill][1] == 1)
        document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
    else {
        document.calcForm.A_ActiveSkillLV.style.visibility = "visible";
        document.calcForm.A_ActiveSkillLV.value = n_A_ActiveSkillLV;
    }

    let label = document.getElementById("AASkillName");
    let select = document.getElementById("SkillSubNum");
    label.style.visibility = "visible";
    select.style.visibility = "visible";
    if (n_A_ActiveSkill == 66 || n_A_ActiveSkill == 326) {
        label.textContent = "Card Weight: ";
        select.min = 0;
        select.max = 8000;
        select.step = 500;
    } else if (n_A_ActiveSkill == 112 || n_A_ActiveSkill == 113)  {
        label.textContent = "Number of Stacked Monsters: ";
        select.min = 1;
        select.max = 20;
        select.step = 1;
    } else if (n_A_ActiveSkill == 131) {
        label.textContent = "Number of Hits: ";
        select.min = 1;
        select.max = 15;
        select.step = 1;
    } else if (n_A_ActiveSkill == 88) {
        label.textContent = "Poison React Level: ";
        select.min = 1;
        select.max = 10;
        select.step = 1;
    } else if (n_A_ActiveSkill == 197) {
        label.textContent = "Remaining SP: ";
        select.min = 1;
        select.max = 99999;
        select.step = 100;
    } else if (n_A_ActiveSkill == 394) {
        label.textContent = "";
        for (i = 0; i <= 4; i++)
            document.calcForm.SkillSubNum.options[i] = new Option(SyurikenOBJ[i][2], i);
        document.calcForm.SkillSubNum.value = 0;
    } else if (n_A_ActiveSkill == 395) {
        label.textContent = "";
        for (i = 0; i <= 4; i++)
            document.calcForm.SkillSubNum.options[i] = new Option(KunaiOBJ[i][2], i);
        document.calcForm.SkillSubNum.value = 0;
    } else if (n_A_ActiveSkill == 405) {
        label.textContent = "Remaining SP: ";
        select.min = 1;
        select.max = 99999;
        select.step = 100;
    } else {
        label.style.visibility = "hidden";
        select.style.visibility = "hidden";
    }
}


function refreshEnemyStats(enemyStats) {
    myInnerHtml("Enemy_Race", global.RaceOBJ[enemyStats.race], 0);
    if (enemyStats.element > 0 && global.elementOBJ[enemyStats.element] === undefined) {
        let w = Math.floor(enemyStats.element / 10);
        myInnerHtml("Enemy_Element", (global.elementOBJ[w] + enemyStats.element % 10), 0);
    }
    myInnerHtml("Enemy_Size", global.SizeOBJ[enemyStats.size], 0);
    myInnerHtml("Enemy_HP", enemyStats.hp, 0);
    myInnerHtml("Enemy_ATK", enemyStats.atk1, 0);
    myInnerHtml("Enemy_ATK2", enemyStats.atk2, 0);
    myInnerHtml("Enemy_PerfectHit", enemyStats.perfectHit, 0);
    myInnerHtml("Enemey_PerfectDodge", enemyStats.perfectDodge, 0);
    myInnerHtml("Enemy_Def", enemyStats.def, 0);
    myInnerHtml("Enemy_Mdef", enemyStats.mdef, 0);
    myInnerHtml("Enemy_VitDef", enemyStats.vit, 0);
    myInnerHtml("Enemy_Mdef2", enemyStats.mdef2, 0);
    myInnerHtml("Enemy_RewardBaseEXP", enemyStats.exp, 0);
    myInnerHtml("Enemy_RewardJobEXP", enemyStats.jobExp, 0);
}

function refreshPlayerStats(playerStats) {
    myInnerHtml("A_STR", playerStats.baseStr, 0);
    myInnerHtml("A_AGI", playerStats.baseAgi, 0);
    myInnerHtml("A_VIT", playerStats.baseVit, 0);
    myInnerHtml("A_DEX", playerStats.baseDex, 0);
    myInnerHtml("A_INT", playerStats.baseInt, 0);
    myInnerHtml("A_LUK", playerStats.baseLuk, 0);
    myInnerHtml("A_STRp", playerStats.bonusStr, 0);
    myInnerHtml("A_AGIp", playerStats.bonusAgi, 0);
    myInnerHtml("A_VITp", playerStats.bonusVit, 0);
    myInnerHtml("A_DEXp", playerStats.bonusDex, 0);
    myInnerHtml("A_INTp", playerStats.bonusInt, 0);
    myInnerHtml("A_LUKp", playerStats.bonusLuk, 0);
    myInnerHtml("A_MaxHP", playerStats.maxHp, 0);
    myInnerHtml("A_MaxSP", playerStats.maxSp, 0);
    myInnerHtml("A_totalDEF", playerStats.def, 0);
    myInnerHtml("A_MDEF", playerStats.mdef, 0);
    myInnerHtml("A_HIT", playerStats.hit, 0);
    myInnerHtml("A_FLEE", playerStats.flee, 0);
    myInnerHtml("A_LUCKY", playerStats.perfectDodge, 0);
    myInnerHtml("A_CRI", playerStats.crit, 0);
    myInnerHtml("A_MATK", playerStats.matk, 0);
    myInnerHtml("A_ASPD", playerStats.aspdForDisplay, 0);
    myInnerHtml("A_ATK_LEFT", playerStats.atkLeft, 0);
    myInnerHtml("A_ATK_RIGHT", playerStats.atkRight, 0);
    myInnerHtml("A_MATK_1", playerStats.matk[0], 0);
    myInnerHtml("A_MATK_2", playerStats.matk[2], 0);
}

function refreshBattleResults(battleResults) {
    myInnerHtml("BaseAttackCalc", battleResults.baseAttackCalc, 0);
    myInnerHtml("MinWeaponAttackCalc", battleResults.minWeaponAttackCalc, 0);
    myInnerHtml("AvgWeaponAttackCalc", battleResults.avgWeaponAttackCalc, 0);
    myInnerHtml("MaxWeaponAttackCalc", battleResults.maxWeaponAttackCalc, 0);
    myInnerHtml("BattleHIT", battleResults.battleHit, 0);
    myInnerHtml("BattleFLEE", battleResults.battleFlee, 0);
    myInnerHtml("CRIATKname", battleResults.display.critAtkName, 0);
    myInnerHtml("CRIATK", battleResults.critAtk[0], 0);
    myInnerHtml("CRInumname", battleResults.display.critChanceName, 0);
    myInnerHtml("CRInum", battleResults.critChance + "%", 0);
    myInnerHtml("bSUBname", battleResults.display.bonusSubName, 0);
    myInnerHtml("bSUB", battleResults.display.bonusSub, 0);
    myInnerHtml("bSUB2name", battleResults.display.bonusSub2Name, 0);
    myInnerHtml("bSUB2", battleResults.display.bonusSub2, 0);
    myInnerHtml("delay", battleResults.afterCastDelay, 0);
    myInnerHtml("casttime", battleResults.cast, 0);
    myInnerHtml("ATK_00", battleResults.minAtk + battleResults.display.atk00, 0);
    myInnerHtml("ATK_01", battleResults.avgAtk + battleResults.display.atk01, 0);
    myInnerHtml("ATK_02", battleResults.maxAtk + battleResults.display.atk02, 0);
    myInnerHtml("DPS", battleResults.dps, 0);
    myInnerHtml("MinATKnum", battleResults.display.minAtkNum, 0);
    myInnerHtml("AveATKnum", battleResults.display.avgAtkNum, 0);
    myInnerHtml("MaxATKnum", battleResults.display.maxAtkNum, 0);
    myInnerHtml("BattleTime", battleResults.display.battleTime, 0);
    // myInnerHtml("AtkBaseExp", playerStats.str, 0);
    // myInnerHtml("AtkJobExp", playerStats.str, 0);
    myInnerHtml("AverageReceivedDamage", battleResults.avgDamageReceived, 0);
    myInnerHtml("MinimumReceivedDamage", battleResults.minDamageReceived, 0);
    myInnerHtml("MaximumReceivedDamage", battleResults.maxDamageReceived, 0);
    myInnerHtml("AverageReceivedDamageIncludingDodge", battleResults.averageReceivedDamage, 0);
}

function refreshTestCases() {
    if (testcases) {
        let tableBody = document.getElementById("testcases-table-body");
        while (tableBody.hasChildNodes()) {
            tableBody.removeChild(tableBody.lastChild);
        }
        for(let testcase of testcases) {
            let row = tableBody.insertRow(-1);

            var loadButton = document.createElement('input');
            loadButton.type = "button"
            loadButton.value = 'Load';
            loadButton.addEventListener('click', function() {
                const index = testcases.findIndex(testcase => testcase._id === localStorage.getItem("loadedTestCase"));
                if (localStorage.getItem("autosave")) {
                    testcases[index] = GetTestCase(JSON.parse(localStorage.getItem("autosave")));
                    localStorage.setItem("testcases", JSON.stringify(testcases));
                }
                localStorage.setItem("loadedTestCase", testcase._id);
                LoadForm(JSON.parse(atob(testcase.formData)));
                refreshTestCases();
            });
            var saveButton = document.createElement('input');
            saveButton.type = "button"
            saveButton.value = 'Save';
            saveButton.addEventListener('click', function() {
                const index = testcases.findIndex(testcase => testcase._id === localStorage.getItem("loadedTestCase"));
                if (localStorage.getItem("autosave")) {
                    testcases[index] = GetTestCase(JSON.parse(localStorage.getItem("autosave")));
                    localStorage.setItem("testcases", JSON.stringify(testcases));
                }
                refreshTestCases();
            });
            var deleteButton = document.createElement('input');
            deleteButton.type = "button"
            deleteButton.value = 'Del';
            deleteButton.addEventListener('click', function() {
                const index = testcases.findIndex(testcase => testcase._id === row.id);
                testcases.splice(index, 1);
                localStorage.setItem("testcases", JSON.stringify(testcases));
                if (localStorage.getItem("loadedTestCase") === row.id) {
                    document.calcForm.reset();
                    localStorage.removeItem("loadedTestCase");
                }
                refreshTestCases();
            })

            var copyButton = document.createElement('input');
            copyButton.type = "button"
            copyButton.value = 'Copy';
            copyButton.addEventListener('click', function() {
                const index = testcases.findIndex(testcase => testcase._id === row.id);
                let formData = testcases[index].formData;
                formData = JSON.parse(atob(formData));
                delete formData._id;
                let copiedTestCase = GetTestCase(formData);
                testcases.push(copiedTestCase);
                localStorage.setItem("testcases", JSON.stringify(testcases));
                localStorage.setItem("loadedTestCase", copiedTestCase._id);
                LoadForm(formData);
                refreshTestCases();
            });
            row.id = testcase._id;
            let firstCell = row.insertCell(-1);
            firstCell.appendChild(loadButton);
            firstCell.appendChild(copyButton);
            firstCell.appendChild(saveButton);
            firstCell.appendChild(deleteButton);
            row.insertCell(-1).appendChild(document.createTextNode(testcase._id));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.job));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseLevel + "/" + testcase.jobLevel));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseStr));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseAgi));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseVit));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseInt));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseDex));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.baseLuk));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.targetId));
            row.insertCell(-1).appendChild(document.createTextNode(testcase.skillToUse.name));
        }
        for (const tr of tableBody.getElementsByTagName("tr")) {
            tr.style.backgroundColor = "";
        }
        document.getElementById("editing-testcase").textContent = "";
        if (localStorage.getItem("loadedTestCase") && document.getElementById(localStorage.getItem("loadedTestCase"))) {
            document.getElementById(localStorage.getItem("loadedTestCase")).style.backgroundColor = "#999";
            document.getElementById("editing-testcase").textContent = "Editing " + localStorage.getItem("loadedTestCase");
        }
    }
}

function bindOnChangeEnemy() {
    let select = document.getElementById("enemy-select");
    select.addEventListener("change", (e) => {

        let enemyStats = CalculateEnemyStats(getFormData(document), global.InWarOfEmperium);

        refreshEnemyStats(enemyStats);
    })
}

function bindOnClickCalculate() {
    let select = document.getElementById("btn-calculate");
    select.addEventListener("click", (e) => {
        Calculate();
    });
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

function bindOnChangeWeapon2Type() {
    let select = document.getElementById("weapon2-type-select");
    select.addEventListener("change", (e) => {
        ClickWeaponType2(e.target.value)
    });
}

function bindOnChangeGear() {
    document.getElementById("weapon-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon2-select").addEventListener("change", (e) => {
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
    document.getElementById("weapon2-card-1-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon2-card-2-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon2-card-3-select").addEventListener("change", (e) => {
        ClickB_Item(e.target.value)
    });
    document.getElementById("weapon2-card-4-select").addEventListener("change", (e) => {
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
    document.getElementById("left-hand-card-shortcut-select").addEventListener("change", (e) => {
        SetCardShortLeft()
    });
}

function bindAutoCalculate() {
    function autosave() {
        let formData = getFormData(document);
        localStorage.setItem("autosave", serializeFormToJSON(formData));
        if (localStorage.getItem("loadedTestCase") != null) {
            if (document.getElementById("input-sync-testcase").checked) {
                const index = testcases.findIndex(testcase => testcase._id === localStorage.getItem("loadedTestCase"));
                testcases[index] = GetTestCase(formData);
                localStorage.setItem("testcases", JSON.stringify(testcases));
                refreshTestCases();
            } else {
                document.getElementById(localStorage.getItem("loadedTestCase")).style.fontStyle = "italic";
            }
        }
    }

    document.querySelectorAll("input").forEach((input) => {
        input.addEventListener("change", (event) => {
            if (document.getElementById("checkbox-auto-calculate").checked) {
                Calculate();
            }
            if (document.getElementById("checkbox-auto-save").checked) {
                autosave();
            }
        });
    });
    document.querySelectorAll("select").forEach((input) => {
        input.addEventListener("change", (event) => {
            if (document.getElementById("checkbox-auto-calculate").checked) {
                Calculate();
            }
            if (document.getElementById("checkbox-auto-save").checked) {
                autosave();
            }
        });
    });
}
function bindAddTestCase() {
    function addTestCase() {
        let newTestCase = GenerateTestCase();
        if (!testcases) {
           testcases = [];
        }
        testcases.push(newTestCase);
        localStorage.setItem("testcases", JSON.stringify(testcases));
        localStorage.setItem("loadedTestCase", newTestCase._id);
        LoadSave(newTestCase._id);
        refreshTestCases();
    }

    document.getElementById("btn-add-testcase").addEventListener("click", function () {
        document.calcForm.reset();
        addTestCase();
    });
    document.getElementById("btn-add-testcase-keep-current").addEventListener("click", function () {
        addTestCase();
    });
}

function bindUnloadTestCase() {
    document.getElementById("btn-unload-testcase").addEventListener("click", function() {
        localStorage.removeItem("loadedTestCase");
        refreshTestCases();
    });
}

function bindSaveTestCases() {
    document.getElementById('btn-save-testcases').addEventListener('click', function() {
        var jsonData = JSON.stringify(testcases, null, 2);
        var blob = new Blob([jsonData], {type: 'application/json'});

        var url = URL.createObjectURL(blob);
        var a = document.createElement('a');
        a.href = url;
        a.download = 'data.json'; // You can specify any file name

        document.body.appendChild(a); // Append the anchor to body
        a.click(); // Programmatically click the anchor to trigger the download

        document.body.removeChild(a); // Remove the anchor from the body
        URL.revokeObjectURL(url); // Free up memory by revoking the Blob URL
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

function bindCopyToClipboard() {
    document.getElementById("btn-copy-form").addEventListener("click", (e) => {
        let formData= getFormData(document);
        navigator.clipboard.writeText(JSON.stringify(formData));
    });
}

function bindGenerateTestCase() {
    document.getElementById("btn-generate-testcase").addEventListener("click", (e) => {
        GenerateTestCase()
    });
}
function bindUploadTestCases() {
    let elem = document.getElementById("input-edit-testcases");
    elem.addEventListener("change", (e) => {
        if (elem.files.length == 1) {
            let file = elem.files[0];
            const reader = new FileReader();
            reader.addEventListener('load', (event) => {
                testcases = JSON.parse(reader.result);
                localStorage.setItem("testcases", testcases);
                refreshTestCases();
            });
            reader.readAsText(file);
        }
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
    for (let i = 2; i <= n_A_STR; i++)
        StPoint += StCalc2(i);
    for (let i = 2; i <= n_A_AGI; i++)
        StPoint += StCalc2(i);
    for (let i = 2; i <= n_A_VIT; i++)
        StPoint += StCalc2(i);
    for (let i = 2; i <= n_A_INT; i++)
        StPoint += StCalc2(i);
    for (let i = 2; i <= n_A_DEX; i++)
        StPoint += StCalc2(i);
    for (let i = 2; i <= n_A_LUK; i++)
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
    for (let i = 0; i <= 21; i++) {
        if (global.JobASPD[20][i] != 0) {
            document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i], i);
            j++;
        }
    }
    ClickWeaponType(0);
    WeaponSet();
    ItemSet();
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
        if (global.ItemOBJ[i][1] == n_A_WeaponType && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
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
        if (global.ItemOBJ[i][1] == n_A_Weapon2Type && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
            work[j] = i;
            j++;
        }
    }
    work[j] = "EOF";
    work = sort(work);
    for (let i = 0; i < j; i++)
        document.calcForm.A_weapon2.options[i] = new Option(global.ItemOBJ[work[i]][8], global.ItemOBJ[work[i]][0]);

}


function ItemSet() {
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
        if (global.ItemOBJ[i][1] == 50 && (checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[0][wsj[0]] = i;
            wsj[0]++;
        } else if (global.ItemOBJ[i][1] == 51 && (checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[1][wsj[1]] = i;
            wsj[1]++;
        } else if (global.ItemOBJ[i][1] == 52 && (checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1 || SuperNoviceFullWeaponCHECK)) {
            workB[2][wsj[2]] = i;
            wsj[2]++;
        } else if (global.ItemOBJ[i][1] == 61 && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
            workB[3][wsj[3]] = i;
            wsj[3]++;
        } else if (global.ItemOBJ[i][1] == 60 && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
            workB[4][wsj[4]] = i;
            wsj[4]++;
        } else if (global.ItemOBJ[i][1] == 62 && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
            workB[5][wsj[5]] = i;
            wsj[5]++;
        } else if (global.ItemOBJ[i][1] == 63 && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
            workB[6][wsj[6]] = i;
            wsj[6]++;
        } else if (global.ItemOBJ[i][1] == 64 && checkIfClassCanWearEquipment(global.ItemOBJ[i][2]) == 1) {
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
    for (let i = 0; i < wsj[1]; i++) {
        z = workB[1][i];
        document.calcForm.A_head2.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[2]; i++) {
        z = workB[2][i];
        document.calcForm.A_head3.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[3]; i++) {
        z = workB[3][i];
        document.calcForm.A_left.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[4]; i++) {
        z = workB[4][i];
        document.calcForm.A_body.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[5]; i++) {
        z = workB[5][i];
        document.calcForm.A_shoulder.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[6]; i++) {
        z = workB[6][i];
        document.calcForm.A_shoes.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
    for (let i = 0; i < wsj[7]; i++) {
        z = workB[7][i];
        document.calcForm.A_acces1.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
        document.calcForm.A_acces2.options[i] = new Option(global.ItemOBJ[z][8], global.ItemOBJ[z][0]);
    }
}

function checkIfClassCanWearEquipment(nJEIS) {
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
    for (let i = 0; i < 20; i++) {
        w_ASSP0[i] = 999;
        w_ASSP9[i] = 0;
    }
    for (let i = 0; i <= 20; i++) {
        for (j2 = 0; global.ItemOBJ[n_A_Equip[i]][11 + j2] != 0; j2 += 2) {
            if (global.ItemOBJ[n_A_Equip[i]][11 + j2] == 220 || global.ItemOBJ[n_A_Equip[i]][11 + j2] == 221) {
                w_ASSP0[j] = Math.floor((global.ItemOBJ[n_A_Equip[i]][12 + j2] % 100000) / 100);
                w_ASSP9[j] = global.ItemOBJ[n_A_Equip[i]][12 + j2];
                j++;
            }
        }
    }

    for (let i = 0; i <= 25; i++) {
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
    for (let i = 0; i < 20; i++) {
        if (w_ASSP0bk[i] != w_ASSP0[i])
            w_ASSPch = 1
    }
    if (w_ASSPch) {
        let k;
        for (k = 0; global.JobSkillActiveOBJ[n_A_JOB][k] != 999; k++) ;
        for (let i = k + 20; i >= k; i--)
            document.calcForm.A_ActiveSkill.options[i] = null;
        j = 0;
        for (let i = k; w_ASSP0[j] != 999; i++, j++) {
            if (w_ASSP9[j] < 200000)
                document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2], w_ASSP9[j]);
            else
                document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2] + "(Temp AS)", w_ASSP9[j]);
        }

    }
    for (let i = 0; i < 20; i++)
        w_ASSP0bk[i] = w_ASSP0[i];

    if (eval(document.calcForm.A_ActiveSkill.value) == 0)
        document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
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

function serializeFormToJSON(formObject) {

    formObject._id = localStorage.getItem("loadedTestCase");
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

function Calculate() {
    let formData = getFormData(document);
    let targetStats = CalculateEnemyStats(formData, 0);
    let sourceStats = CalculateAllStats(formData, targetStats);
    let battleResult = CalculateBattle(sourceStats, targetStats, 0);

    refreshEnemyStats(targetStats);
    refreshPlayerStats(sourceStats);
    refreshBattleResults(battleResult);
}

function GenerateTestCase() {
    let testCase = GetTestCase(getFormData(document));
    navigator.clipboard.writeText(JSON.stringify(testCase));
    return testCase;
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

function LoadForm(json) {
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
    if (json.A_weapon2) {
        document.calcForm.A_weapon2.value = json.A_weapon2;
        ClickB_Item(json.A_weapon2);

    }
    let {n_A_JOB, isRebirth} = n_A_JobSet();
    ClickActiveSkill(json.A_ActiveSkill);

    ActiveSkillSetPlus();
    Calculate();
    localStorage.setItem("autosave", serializeFormToJSON(getFormData(document)));
}

function LoadSave(saveSlot) {
    if (saveSlot !== "autosave" && testcases && localStorage.getItem("loadedTestCase")) {
        let json = testcases
            .find(testcase => testcase._id === localStorage.getItem("loadedTestCase"));
        LoadForm(JSON.parse(atob(json.formData)));
    } else if (localStorage.getItem("autosave")) {
        localStorage.removeItem("loadedTestCase");
        LoadForm(JSON.parse(localStorage.getItem("autosave")));
    } else {
        Calculate();
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
                var work_backup = work[k - 1];
                work[k - 1] = work[k];
                work[k] = work_backup;
            }
        }
    }
    return work;
}

function Click_Card(CBI) {
    ActiveSkillSetPlus();
}

for (let i = 0; i < global.cardSetCombo.length; i++) {
    for (let k = 1; global.cardSetCombo[i][k] != "NULL"; k++) {
        let j;
        for (j = 4; global.cardOBJ[global.cardSetCombo[i][k]][j] != 0; j += 2) ;
        global.cardOBJ[global.cardSetCombo[i][k]][j] = 90;
        global.cardOBJ[global.cardSetCombo[i][k]][j + 1] = global.cardSetCombo[i][0];
        global.cardOBJ[global.cardSetCombo[i][k]][j + 2] = 0;
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

function ClickB_Item(CBI) {
    const start = Date.now();
    ActiveSkillSetPlus();

}

function SetEquipName(SENw) {
    const start = Date.now();
    let SENstr = "";
    for (let SENi = 0; SENi <= SE_MAXnum; SENi++) {
        if (w_SE[SENi][0] == SENw) {
            for (let SENj = 1; w_SE[SENi][SENj] != "NULL"; SENj++) {
                SENstr += "[" + ItemOBJ[w_SE[SENi][SENj]][8] + "]";
                if (w_SE[SENi][SENj + 1] != "NULL")
                    SENstr += " + ";
            }
            return SENstr;
        }
    }
    console.log("SetEquipName end. Took", Date.now() - start, "ms")
}


for (i = 1; i <= 81; i++)
    myInnerHtml("nm0" + i, NameCalc[i - 1], 0);


for (i = 0; i <= 45; i++)
    document.calcForm.A_JOB.options[i] = new Option(global.JobName[i], i);

for (i = 0; i <= 16; i++)
    document.calcForm.A_Arrow.options[i] = new Option(global.ArrowOBJ[i][2], i);

for (i = 0; i <= 9; i++)
    document.calcForm.A_Weapon_element.options[i] = new Option(EnName[i], i);

document.calcForm.A_Weapon_element.options[10] = new Option("Water (Enchanted Stone)", 1);
document.calcForm.A_Weapon_element.options[11] = new Option("Earth (Enchanted Stone)", 2);
document.calcForm.A_Weapon_element.options[12] = new Option("Fire (Enchanted Stone)", 3);
document.calcForm.A_Weapon_element.options[13] = new Option("Wind (Enchanted Stone)", 4);

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


for (i = 0; i < CardShort.length; i++)
    document.calcForm.A_cardshort.options[i] = new Option(CardShort[i][0], i);

for (i = 0; i <= 10; i++) {
    document.calcForm.A_Weapon2_ATKplus.options[i] = new Option(i, i);
}
for (i = 0; CardSortOBJ[0][i] != "NULL"; i++)
    document.calcForm.A_weapon2_card1.options[i] = new Option(cardOBJ[CardSortOBJ[0][i]][2], cardOBJ[CardSortOBJ[0][i]][0]);
for (i = 0; CardSortOBJ[1][i] != "NULL"; i++) {
    document.calcForm.A_weapon2_card2.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
    document.calcForm.A_weapon2_card3.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
    document.calcForm.A_weapon2_card4.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
}
document.calcForm.A_cardshortLeft.options[0] = new Option("Card Shortcuts (Left)", 0);
for (i = 1; i <= 32; i++)
    document.calcForm.A_cardshortLeft.options[i] = new Option(CardShort[i][0], i);

let sortedMonster = global.MonsterOBJ.sort((a, b) => {
    if (a[1] < b[1]) {
        return -1;
    }
    if (a[1] > b[1]) {
        return 1;
    }
})
let enemySelect = document.getElementById("enemy-select");
for (i = 0; i < sortedMonster.length; i++) {
    enemySelect.options[i] = new Option(sortedMonster[i][1], sortedMonster[i][0]);
}

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

bindSearchable(document.getElementById("enemy-select"))

bindUploadTestCases();
bindCopyToClipboard();
bindGenerateTestCase();
bindAddTestCase();
bindUnloadTestCase();
bindSaveTestCases();
bindOnChangeEnemy();
bindOnChangeJob();
bindOnClickCalculate();
bindOnChangeWeaponType();
bindOnChangeWeapon2Type();
bindOnChangeGear();
bindOnChangeCard();
bindOnChangeActiveSkill();
bindOnChangeStat();
bindOnChangeCardShortcut();
bindAutoCalculate();
refreshTestCases();

if (document.getElementById("checkbox-auto-save").checked) {
    LoadSave("autosave");
} else {
    Calculate();
}

if (localStorage.getItem("testcases")) {
    testcases = JSON.parse(localStorage.getItem("testcases"));
    refreshTestCases();
}



document.getElementById("btn-reset").addEventListener("click", function () {
    localStorage.clear();
    document.location.reload()
});

