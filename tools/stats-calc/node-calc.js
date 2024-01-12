import "./global-wip.js"
import "./db-wip.js"
import {CalculateAllStats} from "./calc-wip.js"
import fs from "fs";

let formData = JSON.parse(fs.readFileSync('test-data.json', 'utf-8'))
CalculateAllStats(formData)