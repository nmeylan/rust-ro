import "./global.js"
import "./db.js"
import {CalculateAllStats, CalculateEnemyStats, CalculateBattle} from "./calc.js"
import fs from "fs";

let formData = JSON.parse(fs.readFileSync('./test-data.json', 'utf-8'))
// let targetStats = Calculate(formData);
let targetStats = CalculateEnemyStats(formData, 0);
let sourceStats = CalculateAllStats(formData, targetStats);
let battleResult = CalculateBattle(sourceStats, targetStats, 0);

console.log(targetStats);
console.log(sourceStats);
console.log(battleResult);