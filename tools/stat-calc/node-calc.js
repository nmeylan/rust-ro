import "./global.js"
import "./db.js"
import {
    CalculateAllStats,
    CalculateEnemyStats,
    CalculateBattle,
    buildEquipment,
    buildCard,
    GetTestCase
} from "./calc.js"
import fs from "fs";
import path from "path";

const command = "generate";
let formData, testCase, testCases, file;
switch (command) {
    case "convert":
        const dir = path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/");
        fs.readdir(dir, (err, files) => {
            files.forEach(file => {
                testCases = JSON.parse(fs.readFileSync(path.join(dir, file)));
                let updatedTestCases = [];
                testCases.forEach(tc => {
                    formData = JSON.parse(atob(tc.formData));
                    formData._id = tc._id;
                    testCase = GetTestCase(formData);
                    testCase.desc = tc.desc;
                    updatedTestCases.push(testCase)
                });

                fs.writeFileSync(path.join(dir, file), JSON.stringify(updatedTestCases, null, 2))
            });
        });

        break;
    case "console":
        formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
        testCase = GetTestCase(formData);
        console.log(testCase);
        break;
    case "debug":
        file = "battle-all-skills-weapon-no-passives.json";
        testCases = JSON.parse(fs.readFileSync(path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/" + file)));
        const id = "nmwiju";
        formData = JSON.parse(atob(testCases.find(testCase => testCase._id == id).formData));
        testCase = GetTestCase(formData);
        break;
    case "generate":
        // generate_offensive_skills()
        // generate_job_level_stat_bonus();
        // generate_item_stat_bonus();
        // generate_stats_test();
        generate_card_stat_bonus();
        break;
    case "db":
        let array_index = [];
        let array_name = [];
        for (let item of ItemIds) {
            if (item[1] < 0 && !item[2].startsWith("(No")) {
                // fromsql.filter(v => parseInt(v.rindex) == item[0])
                //     .map(v => {
                //         item[1] = v.id;
                //         item[2] = v.name_aegis;
                //     } )
                console.log(item[0], item[2])
                array_index.push("'" + item[0] + "'");
                array_name.push("'" + item[2].replace("'", " ") + "'");
            }
        }
        // console.log(JSON.stringify(ItemIds));
        console.log("UNNEST(ARRAY[" + array_index.join(",") + "],ARRAY[" + array_name.join(",") + "])")
        break;
    case "bonus":
        let orderedItems = [];
        for (let item of ItemOBJ) {
            orderedItems.push(item[8]);
            for (let itemStatIndex = 0; item[itemStatIndex + 11] != 0; itemStatIndex += 2) {
                let bonus = item[itemStatIndex + 11];
                if (bonus === BYPASS_DEFENSE_ON_RACE) {
                    console.log(item)
                }
            }
        }
        console.log(JSON.stringify(orderedItems));
        break;
}


function generate_offensive_skills() {
    let count = 0;
    let start = Date.now();
    let testcases = [];
    let skills = JSON.parse(fs.readFileSync(path.join(process.cwd(), "../../config/skill.json")));
    let defaultWeaponsPerType = {
        0: 0,
        1: 1,
        2: 16,
        3: 39,
        4: 49,
        5: 54,
        6: 61,
        7: 67,
        8: 75,
        9: 85,
        10: 94,
        11: 110,
        12: 114,
        13: 126,
        14: 131,
        15: 136,
        16: 546,
        17: 549,
        18: 553,
        19: 556,
        20: 558,
        21: 561
    }
    let skillsWeaponFlagsNamePerType = {
        0: "fist",
        1: "dagger",
        2: "1hSword",
        3: "2hSword",
        4: "1hSpear",
        5: "2hSpear",
        6: "1hAxe",
        7: "2hAxe",
        8: "mace",
        9: "staff",
        10: "bow",
        11: "katar",
        12: "book",
        13: "knuckle",
        14: "musical",
        15: "whip",
        16: "huuma",
        17: "revolver",
        18: "rifle",
        19: "shotgun",
        20: "gatling",
        21: "grenade"
    }
    for (let n = 1; n < JobName.length; n++) {
        if (n >= 34 && n <= 40) {
            continue;
        }
        let formData = {};
        formData.A_JobLV = 1;
        formData.A_JOB = n;
        formData.A_BaseLV = 95;
        formData.A_STR = 50;
        formData.A_AGI = 50;
        formData.A_VIT = 50;
        formData.A_INT = 50;
        formData.A_DEX = 50;
        formData.A_LUK = 50;
        formData.A_Arrow = 0;

        formData.A_acces1 = "326";
        formData.A_acces2 = "326";
        formData.A_left = "305";
        formData.A_body = "279";
        formData.A_head1 = "142";
        formData.A_head2 = "243";
        formData.A_head3 = "268";
        formData.A_shoes = "317";
        formData.A_shoulder = "311";
        formData.A_weapon1 = "0";
        formData.A_WeaponType = "0";

        formData.B_Enemy = 272;
        // for (let i = 0; i < JobSkillPassOBJ[n].length; i++) {
        //     if (JobSkillPassOBJ[n][i] != 999) {
        //         formData["A_PASSIVE_SKILL" + i] = SkillOBJ[JobSkillPassOBJ[n][i]][1];
        //     }
        // }
        let weaponTypes = [];
        for (let i = 0; i <= 21; i++) {
            if (JobASPD[n][i] != 0) {
                weaponTypes.push(i);
            }
        }
        let weapons = [];
        for (let weaponType of weaponTypes) {
            for (let i = 0; i < ItemOBJ.length; i++) {
                if (ItemOBJ[i][1] === weaponType) {
                    weapons.push({index: i, type: weaponType});
                }
            }
        }
        console.log("job", n, weaponTypes)

        for (let i = 0; i < JobSkillActiveOBJ[n].length && JobSkillActiveOBJ[n][i] != 999; i++) {
            formData.A_ActiveSkill = JobSkillActiveOBJ[n][i];
            switch (formData.A_ActiveSkill) {
                case 326:
                case 66:
                    formData.SkillSubNum = 8000;
                    break;
                case 112:
                case 113:
                    formData.SkillSubNum = 1;
                    break;
                case 131:
                    formData.SkillSubNum = 5;
                    break;
                case 88:
                    formData.SkillSubNum = 5;
                    break;
                case 197:
                case 405:
                    formData.SkillSubNum = 1000;
                    break;
                case 394:
                case 395:
                    formData.SkillSubNum = 1;
                    break;
            }
            // *****************for each cards********************
            // for (let c = 0; c < 121; c++) {
            //     formData.A_acces1_card = CardSortOBJ[7][c] && CardSortOBJ[7][c] !== "NULL" ? cardOBJ[CardSortOBJ[7][c]][0] : cardOBJ[CardSortOBJ[7][0]][0]
            //     formData.A_acces2_card = CardSortOBJ[7][c] && CardSortOBJ[7][c] !== "NULL" ? cardOBJ[CardSortOBJ[7][c]][0] : cardOBJ[CardSortOBJ[7][0]][0]
            //     formData.A_left_card = CardSortOBJ[3][c] && CardSortOBJ[3][c] !== "NULL" ? cardOBJ[CardSortOBJ[3][c]][0] : cardOBJ[CardSortOBJ[3][0]][0]
            //     formData.A_body_card = CardSortOBJ[4][c] && CardSortOBJ[4][c] !== "NULL" ? cardOBJ[CardSortOBJ[4][c]][0] : cardOBJ[CardSortOBJ[4][0]][0]
            //     formData.A_head1_card = CardSortOBJ[2][c] && CardSortOBJ[2][c] !== "NULL" ? cardOBJ[CardSortOBJ[2][c]][0] : cardOBJ[CardSortOBJ[2][0]][0]
            //     formData.A_head2_card = CardSortOBJ[2][c] && CardSortOBJ[2][c] !== "NULL" ? cardOBJ[CardSortOBJ[2][c]][0] : cardOBJ[CardSortOBJ[2][0]][0]
            //     formData.A_shoes_card = CardSortOBJ[6][c] && CardSortOBJ[6][c] !== "NULL" ? cardOBJ[CardSortOBJ[6][c]][0] : cardOBJ[CardSortOBJ[6][0]][0]
            //     formData.A_shoulder_card = CardSortOBJ[5][c] && CardSortOBJ[5][c] !== "NULL" ? cardOBJ[CardSortOBJ[5][c]][0] : cardOBJ[CardSortOBJ[5][0]][0]
            //     formData.A_weapon1_card1 = CardSortOBJ[1][c] && CardSortOBJ[1][c] !== "NULL" ? cardOBJ[CardSortOBJ[0][c]][0] : cardOBJ[CardSortOBJ[0][0]][0]
            //     formData.A_weapon1_card2 = CardSortOBJ[1][c] && CardSortOBJ[1][c] !== "NULL" ? cardOBJ[CardSortOBJ[1][c]][0] : cardOBJ[CardSortOBJ[1][0]][0]
            //     formData.A_weapon1_card3 = CardSortOBJ[1][c] && CardSortOBJ[1][c] !== "NULL" ? cardOBJ[CardSortOBJ[1][c]][0] : cardOBJ[CardSortOBJ[1][0]][0]
            //     formData.A_weapon1_card4 = CardSortOBJ[1][c] && CardSortOBJ[1][c] !== "NULL" ? cardOBJ[CardSortOBJ[1][c]][0] : cardOBJ[CardSortOBJ[1][0]][0]
            //     formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
            //     console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")");
            //     let testCase = GetTestCase(formData);
            //     count += 1;
            // }

            // *****************for each monster********************
            // for (let m = 0; m < MonsterOBJ.length; m++) {
            //     formData.B_Enemy = m;
            //     formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
            //     console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")", "target", m);
            //     let testCase = GetTestCase(formData);
            //     count += 1;
            // }


            // *****************for each weapon type********************


            formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
            let skillId = SkillOBJ[JobSkillActiveOBJ[n][i]][3];
            let skillFromDb = skills.skills.find(s => s.id === skillId);
            if (SkillOBJ[JobSkillActiveOBJ[n][i]][2] === "Basic Attack") {
                for (let type of weaponTypes) {
                    let weapon = defaultWeaponsPerType[type];
                    if (type === 10 || type === 14 || type === 15) {
                        formData.A_Arrow = 0;
                    }
                    formData.A_weapon1 = weapon + "";
                    formData.A_WeaponType = type + "";
                    console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")", "weapon", ItemIds[weapon][2]);
                    let testCase = GetTestCase(formData);
                    testcases.push(testCase);
                    count += 1;
                }
            } else if (!!skillFromDb) {
                for (let type of weaponTypes) {
                    if (!skillFromDb.requires.weaponFlags || (skillFromDb.requires.weaponFlags && skillFromDb.requires.weaponFlags[skillsWeaponFlagsNamePerType[type]])) {
                        let weapon = defaultWeaponsPerType[type];
                        if (type === 10 || type === 14 || type === 15) {
                            formData.A_Arrow = 0;
                        }
                        formData.A_weapon1 = weapon + "";
                        formData.A_WeaponType = type + "";
                        console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")", "weapon", ItemIds[weapon][2]);
                        let testCase = GetTestCase(formData);
                        testcases.push(testCase);
                        count += 1;
                    }
                }
                // console.log("Can't find skill from db with id", skillId, "name", SkillOBJ[JobSkillActiveOBJ[n][i]][2])
            }
            // console.log("generate test case for job", n, "skill", i, "(", SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")");
            // let testCase = GetTestCase(formData);
            // console.log(testCase)


        }

    }
    console.log("Generated", count, "test cases, in", (Date.now() - start) + "ms")
    fs.writeFileSync(path.join(process.cwd(), "..\\..\\server\\src\\tests\\common\\fixtures\\data\\battle-all-skills-weapon-no-passives.json"), JSON.stringify(testcases))
}

function generate_job_level_stat_bonus() {
    let count = 0;
    let start = Date.now();
    let testcases = [];

    for (let n = 1; n < JobName.length; n++) {
        if (n >= 34 && n <= 40) {
            continue;
        }
        let formData = {};
        formData.A_JOB = n;
        formData.A_BaseLV = 95;
        formData.A_STR = 50;
        formData.A_AGI = 50;
        formData.A_VIT = 50;
        formData.A_INT = 50;
        formData.A_DEX = 50;
        formData.A_LUK = 50;
        formData.A_Arrow = 0;
        formData.B_Enemy = 272;
        formData.A_acces1 = "326";
        formData.A_acces2 = "326";
        formData.A_left = "305";
        formData.A_body = "279";
        formData.A_head1 = "142";
        formData.A_head2 = "243";
        formData.A_head3 = "268";
        formData.A_shoes = "317";
        formData.A_shoulder = "311";
        formData.A_weapon1 = "0";
        formData.A_WeaponType = "0";
        formData.A_ActiveSkill = 0;
        formData.A_ActiveSkillLV = 0;
        let maxJobLvl = 0;
        if (n == 0) maxJobLvl = 10;
        else if (n <= 19 || (41 <= n && n <= 43)) maxJobLvl = 50;
        else if (n == 20) maxJobLvl = 71;
        else maxJobLvl = 70;
        for (let i = 1; i <= maxJobLvl; i++) {
            formData.A_JobLV = i;
            let testCase = GetTestCase(formData);
            testcases.push(testCase);
            count += 1;
        }
    }

    console.log("Generated", count, "test cases, in", (Date.now() - start) + "ms")
    fs.writeFileSync(path.join(process.cwd(), "..\\..\\server\\src\\tests\\common\\fixtures\\data\\stats-for-each-job-level.json"), JSON.stringify(testcases))

}

function generate_item_stat_bonus(itemsFilter, descriptions) {
    let count = 0;
    let start = Date.now();
    let testcases = [];
    let items = [];
    let isRebirth = 0;
    for (let n = 7; n <= JobName.length; n++) {
        if (n >= 34 && n <= 40) {
            continue;
        }
        if (n >= 21 && n <= 40) {
            isRebirth = 1;
        }
        let maxJobLvl = 0;
        if (n == 0) maxJobLvl = 10;
        else if (n <= 19 || (41 <= n && n <= 43)) maxJobLvl = 50;
        else if (n == 20) maxJobLvl = 71;
        else maxJobLvl = 70;
        let formData = {};
        formData.A_JOB = n;
        formData.A_BaseLV = 95;
        formData.A_JobLV = maxJobLvl;
        formData.A_STR = 50;
        formData.A_AGI = 50;
        formData.A_VIT = 50;
        formData.A_INT = 50;
        formData.A_DEX = 50;
        formData.A_LUK = 50;
        formData.A_Arrow = 0;
        formData.B_Enemy = 272;

        formData.A_weapon1 = "0";
        formData.A_WeaponType = "0";
        formData.A_ActiveSkill = 0;
        formData.A_ActiveSkillLV = 0;

        for (let i = 0; i < ItemOBJ.length; i++) {
            if (items.includes(i) || (itemsFilter && !itemsFilter.includes(i))) {
                continue;
            }
            formData.A_acces1 = "326";
            formData.A_acces2 = "326";
            formData.A_left = "305";
            formData.A_body = "279";
            formData.A_head1 = "142";
            formData.A_head2 = "243";
            formData.A_head3 = "268";
            formData.A_shoes = "317";
            formData.A_shoulder = "311";
            formData.A_weapon1 = "0";
            formData.A_WeaponType = "0";
            let canEquip = false;
            if (ItemOBJ[i][1] == 50 && (checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1)) {
                formData.A_head1 = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 51 && (checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1)) {
                formData.A_head2 = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 52 && (checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1)) {
                formData.A_head3 = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 61 && checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1) {
                formData.A_left = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 60 && checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1) {
                formData.A_body = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 62 && checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1) {
                formData.A_shoulder = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 63 && checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1) {
                formData.A_shoes = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] == 64 && checkIfClassCanWearEquipment(isRebirth, ItemOBJ[i][2], n, i) == 1) {
                formData.A_acces1 = i;
                canEquip = true;
            } else if (ItemOBJ[i][1] < 50 && JobASPD[n][ItemOBJ[i][1]] != 0) {
                formData.A_weapon1 = i;
                formData.A_WeaponType = ItemOBJ[i][1];
                canEquip = true;
            }
            if (canEquip) {
                items.push(i);
                let testCase = GetTestCase(formData);
                if (descriptions) {
                    testCase.desc = descriptions[items.length - 1];
                }
                testcases.push(testCase);
                count += 1;
            }
        }
    }

    let p = path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/stats-for-items.json");
    if (itemsFilter != null) {
        p = path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/stats-for-each-stats.json");
    }
    fs.writeFileSync(p, JSON.stringify(testcases));
    console.log("Generated", count, "test cases, in", (Date.now() - start) + "ms", "at", p);
}


function generate_card_stat_bonus(itemsFilter, descriptions) {
    let count = 0;
    let start = Date.now();
    let testcases = [];
    let items = [];
    let isRebirth = 0;
    let n = 7;
    let maxJobLvl = 50;
    let formData = {};
    formData.A_JOB = n;
    formData.A_BaseLV = 95;
    formData.A_JobLV = maxJobLvl;
    formData.A_STR = 50;
    formData.A_AGI = 50;
    formData.A_VIT = 50;
    formData.A_INT = 50;
    formData.A_DEX = 50;
    formData.A_LUK = 50;
    formData.A_Arrow = 0;
    formData.B_Enemy = 272;

    formData.A_weapon1 = "16";
    formData.A_WeaponType = "2";
    formData.A_ActiveSkill = 0;
    formData.A_ActiveSkillLV = 0;
    formData.A_acces1 = "339";
    formData.A_acces2 = "326";
    formData.A_left = "308";
    formData.A_body = "298";
    formData.A_head1 = "241";
    formData.A_head2 = "243";
    formData.A_head3 = "268";
    formData.A_shoes = "321";
    formData.A_shoulder = "312";

    for (let i = 4; i < CardSortOBJ[1].length - 1; i++) {
        if (CardSortOBJ[1][i] === 106 || CardSortOBJ[1][i] === 156 || CardSortOBJ[1][i] === 154 || CardSortOBJ[1][i] === 155) continue;
        if(CardSortOBJ[1][i] >= 201 && CardSortOBJ[1][i] <= 211) continue;
        formData.A_weapon1_card1 = CardSortOBJ[1][i];
        console.log(CardSortOBJ[1][i])
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_weapon1_card1 = "0";
    for (let i = 0; i < CardSortOBJ[2].length - 1; i++) {
        if(CardSortOBJ[2][i] >= 201 && CardSortOBJ[2][i] <= 211) continue;
        formData.A_head1_card = CardSortOBJ[2][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_head1_card = "0";
    for (let i = 0; i < CardSortOBJ[3].length - 1; i++) {
        if(CardSortOBJ[3][i] >= 201 && CardSortOBJ[3][i] <= 211) continue;
        formData.A_left_card = CardSortOBJ[3][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_left_card = "0";
    for (let i = 0; i < CardSortOBJ[4].length - 1; i++) {
        if(CardSortOBJ[4][i] >= 201 && CardSortOBJ[4][i] <= 211) continue;
        formData.A_body_card = CardSortOBJ[4][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_body_card = "0";
    for (let i = 0; i < CardSortOBJ[5].length - 1; i++) {
        if(CardSortOBJ[5][i] >= 201 && CardSortOBJ[5][i] <= 211) continue;
        formData.A_shoulder_card = CardSortOBJ[5][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_shoulder_card = "0";
    for (let i = 0; i < CardSortOBJ[6].length - 1; i++) {
        if(CardSortOBJ[6][i] >= 201 && CardSortOBJ[6][i] <= 211) continue;
        formData.A_shoes_card = CardSortOBJ[6][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_shoes_card = "0";
    for (let i = 0; i < CardSortOBJ[7].length - 1; i++) {
        if(CardSortOBJ[7][i] >= 201 && CardSortOBJ[7][i] <= 211) continue;
        formData.A_acces1_card = CardSortOBJ[7][i];
        let testCase = GetTestCase(formData);
        testcases.push(testCase);
        count += 1;
    }
    formData.A_acces1_card = "0";


    let p = path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/stats-for-cards.json");

    fs.writeFileSync(p, JSON.stringify(testcases));
    console.log("Generated", count, "test cases, in", (Date.now() - start) + "ms", "at", p);
}

function generate_stats_test() {
    let stats = {};
    let items = [];
    let descriptions = [];
    a : for (let item of ItemOBJ) {
        for (let itemStatIndex = 0; item[itemStatIndex + 11] != 0; itemStatIndex += 2) {
            if (items.includes(item[0])) {
                continue a;
            }
            let bonus = item[itemStatIndex + 11];
            if (item[itemStatIndex + 12] != 0 && item[8] !== 0) {
                if (stats[bonusLabel[bonus]] === undefined) {
                    items.push(item[0]);
                    descriptions.push(bonusLabel[bonus]);
                    stats[bonusLabel[bonus]] = [{
                        name: item[8],
                        index: item[0],
                        id: ItemIds[item[0]][1],
                        value: item[itemStatIndex + 12]
                    }]
                }

            }
        }
    }
    for (let bonus of Object.entries(bonusLabel)) {
        if (!Object.keys(stats).includes(bonus[1])) {
            console.log("missing stat", bonus[1])
        }
    }
    // console.log(items)
    generate_item_stat_bonus(items, descriptions);
}

function checkIfClassCanWearEquipment(isRebirth, nJEIS, n, i) {
    if (isRebirth == 0) {
        if (ItemOBJ[i][11] == 200)
            return 0;
    }
    for (let nJEISi = 0; JobEquipItemOBJ[n][nJEISi] != 999; nJEISi++) {
        if (JobEquipItemOBJ[n][nJEISi] == nJEIS)
            return 1;
    }
    return 0;
}