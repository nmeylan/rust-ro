const fs = require('fs');

const readline = require('readline');
let input = JSON.parse(fs.readFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill.json', 'utf8'))
// let input = JSON.parse(fs.readFileSync('/home/nmeylan/dev/ragnarok/rust-ragnarok-server/config/skill.json', 'utf8'))

let notesStream = fs.createReadStream('C:\\dev\\ragnarok\\rust-ragnarok-server\\doc\\notes\\skill-bonus.txt');
// let notesStream = fs.createReadStream('/home/nmeylan/dev/ragnarok/rust-ragnarok-server/doc/notes/skill-bonus.txt');

const rl = readline.createInterface({
    input: notesStream,
    crlfDelay: Infinity
});

const aliases = {};
const oldFieldBonus = {};
let shouldStartAlias = false;
let jsonName = "";
let rustName = "";
let regexJsonName = /rename = \"(.*)\"/;
let regexRustName = /(.*):/;
let regexBonusName = /.* =>(.*)/;
rl.on('line', (line) => {
    if (line.includes("--- aliases")) {
        shouldStartAlias = true;
    }
    if (!shouldStartAlias) {
        let matches = line.match(regexRustName);
        if (matches && matches.length == 2) {
            rustName = matches[1].trim();
            matches = line.match(regexBonusName);
            if (matches && matches.length == 2) {
                oldFieldBonus[rustName] = matches[1].trim()
            }
        }
    }
    if (shouldStartAlias) {
        if (line.includes("rename")) {
            let matches = line.match(regexJsonName);
            jsonName = matches[1];
        }
        if (line.includes(":")) {
            let matches = line.match(regexRustName);
            rustName = matches[1].trim();
            aliases[jsonName]= rustName;
        }
    }
});
rl.on('close', () => {
    // console.log(aliases)
    for (let skill of input.skills) {
        let bonusToSelf = [];
        let bonusToParty = [];
        let bonusToTarget = [];
        let fieldsToRemove = [];
        for (let field of Object.keys(skill)) {
            let alias = aliases[field];
            if (alias) {
                if (oldFieldBonus[alias]) {
                    // console.log(field, alias, oldFieldBonus[alias])
                    fieldsToRemove.push(field);
                    let bonuses = [];
                    if (Array.isArray(skill[field])) {
                        for(let entry of skill[field]) {
                            bonuses.push({level: entry.level, value: {bonus: oldFieldBonus[alias], value: entry.value}})
                        }
                    } else {
                        bonuses.push({value: {bonus: oldFieldBonus[alias], value: skill[field].value}})
                    }
                    if (skill.targetType === "Self") {
                        bonusToSelf.push(bonuses)
                    } else{
                        bonusToTarget.push(bonuses)
                    }
                }
            }
        }
        for(let field of fieldsToRemove) {
            let bonuses = [];
            if (skill.targetType === "Self") {
                bonuses = bonusToSelf;
            } else{
                bonuses = bonusToTarget
            }
            console.log(JSON.stringify({[field]: skill[field]}), "=>", JSON.stringify(bonuses))
        }
    }
});