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

const command = "console";

switch (command) {
    case "convert":
        break;
    case "console":
        let formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
        let testCase= GetTestCase(formData);
        console.log(testCase)
        break;
}

