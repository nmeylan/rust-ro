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

const command = "convert";
let formData, testCase, testCases;
switch (command) {
    case "convert":
        let file = "battle-all-skills-weapon-no-passives.json";
        testCases = JSON.parse(fs.readFileSync(path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/" + file)));
        let updatedTestCases = [];
        testCases.forEach(tc => {
            formData = JSON.parse(atob(tc.formData));
            testCase = GetTestCase(formData);
            updatedTestCases.push(testCase)
        });

        fs.writeFileSync(path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/" + file), JSON.stringify(updatedTestCases, null, 2))
        break;
    case "console":
        formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
        testCase = GetTestCase(formData);
        console.log(testCase);
        break;
    case "debug":
        testCases = JSON.parse(fs.readFileSync(path.join(process.cwd(), "../../server/src/tests/common/fixtures/data/data.json")));
        const id = "oatiz7";
        formData = JSON.parse(atob(testCases.find(testCase => testCase._id == id).formData));
        testCase = GetTestCase(formData);
        break;
    case "generate":
        generate()
        break;
}


function generate() {
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
        11: 100,
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
        if (n >=34 && n <=40) {
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
            for (let i = 0; i < global.ItemOBJ.length; i++) {
                if (global.ItemOBJ[i][1] === weaponType) {
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
            //     formData.A_acces1_card = global.CardSortOBJ[7][c] && global.CardSortOBJ[7][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[7][c]][0] : cardOBJ[global.CardSortOBJ[7][0]][0]
            //     formData.A_acces2_card = global.CardSortOBJ[7][c] && global.CardSortOBJ[7][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[7][c]][0] : cardOBJ[global.CardSortOBJ[7][0]][0]
            //     formData.A_left_card = global.CardSortOBJ[3][c] && global.CardSortOBJ[3][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[3][c]][0] : cardOBJ[global.CardSortOBJ[3][0]][0]
            //     formData.A_body_card = global.CardSortOBJ[4][c] && global.CardSortOBJ[4][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[4][c]][0] : cardOBJ[global.CardSortOBJ[4][0]][0]
            //     formData.A_head1_card = global.CardSortOBJ[2][c] && global.CardSortOBJ[2][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[2][c]][0] : cardOBJ[global.CardSortOBJ[2][0]][0]
            //     formData.A_head2_card = global.CardSortOBJ[2][c] && global.CardSortOBJ[2][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[2][c]][0] : cardOBJ[global.CardSortOBJ[2][0]][0]
            //     formData.A_shoes_card = global.CardSortOBJ[6][c] && global.CardSortOBJ[6][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[6][c]][0] : cardOBJ[global.CardSortOBJ[6][0]][0]
            //     formData.A_shoulder_card = global.CardSortOBJ[5][c] && global.CardSortOBJ[5][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[5][c]][0] : cardOBJ[global.CardSortOBJ[5][0]][0]
            //     formData.A_weapon1_card1 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[0][c]][0] : cardOBJ[global.CardSortOBJ[0][0]][0]
            //     formData.A_weapon1_card2 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
            //     formData.A_weapon1_card3 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
            //     formData.A_weapon1_card4 = global.CardSortOBJ[1][c] && global.CardSortOBJ[1][c] !== "NULL" ? cardOBJ[global.CardSortOBJ[1][c]][0] : cardOBJ[global.CardSortOBJ[1][0]][0]
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
