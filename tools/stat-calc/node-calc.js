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

// const command = "console";
//
// switch (command) {
//     case "convert":
//         break;
//     case "console":
//         let formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
//         let testCase= GetTestCase(formData);
//         break;
// }

for (let n = 14; n < JobName.length; n++) {
    let formData = {};
    formData.A_JobLV = 1;
    formData.A_JOB = n;
    formData.A_BaseLV = 1;
    formData.A_STR = 1;
    formData.A_AGI = 1;
    formData.A_VIT = 1;
    formData.A_INT = 1;
    formData.A_DEX = 1;
    formData.A_LUK = 1;
    formData.A_Arrow = 0;

    formData.A_acces1 = "326",
    formData.A_acces2 = "326",
    formData.A_left = "305",
    formData.A_body = "279",
    formData.A_head1 = "142",
    formData.A_head2 = "243",
    formData.A_head3 = "268",
    formData.A_shoes = "317",
    formData.A_shoulder = "311",
    formData.A_weapon1 = "0",
    formData.A_WeaponType = "0",

    formData.B_Enemy = "144";
    for (let i = 0; i < JobSkillPassOBJ[n].length; i++) {
        if (JobSkillPassOBJ[n][i] != 999) {
            formData["A_PASSIVE_SKILL" + i] = SkillOBJ[JobSkillPassOBJ[n][i]][1];
        }
    }
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
        formData.A_ActiveSkillLV = SkillOBJ[JobSkillActiveOBJ[n][i]][1];
        console.log("generate test case for job", n, "skill", i, "(",SkillOBJ[JobSkillActiveOBJ[n][i]][2], ")");
        let testCase = GetTestCase(formData);
        // console.log(testCase)
    }

}
