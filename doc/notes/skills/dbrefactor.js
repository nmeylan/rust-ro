const fs = require('fs');

const readline = require('readline');
// let input = JSON.parse(fs.readFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill.json', 'utf8'))
let input = JSON.parse(fs.readFileSync('/home/nmeylan/dev/ragnarok/rust-ragnarok-server/config/skill.json', 'utf8'))

// let dest = "C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill-new.json";
let dest = "/home/nmeylan/dev/ragnarok/rust-ragnarok-server/config/skill-new.json";

// let notesStream = fs.createReadStream('C:\\dev\\ragnarok\\rust-ragnarok-server\\doc\\notes\\skill-bonus.txt');
let notesStream = fs.createReadStream('/home/nmeylan/dev/ragnarok/rust-ragnarok-server/doc/notes/skill-bonus.txt');

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
                            getBonus(skill, oldFieldBonus[alias], entry.value).forEach(bonus => bonuses.push({level: entry.level, value: bonus}));
                        }
                    } else {
                        getBonus(skill, oldFieldBonus[alias], skill[field]).forEach(bonus => bonuses.push({value: bonus}));
                    }
                    let toParty = false;
                    if (oldFieldBonus[alias].includes("BonusesToParty")) {
                        toParty = true;
                    }
                    if (toParty) {
                        bonusToTarget.push(...bonuses)
                    }
                    else if (skill.targetType === "Support") {
                        bonusToTarget.push(...bonuses)
                    } else{
                        bonusToSelf.push(...bonuses)
                    }
                }
            }
        }
        for(let field of fieldsToRemove) {
            delete skill[field]
        }
        if (bonusToSelf.length > 0) {
            skill.bonusToSelf = bonusToSelf;
        }
        if (bonusToParty.length > 0) {
            skill.bonusToParty = bonusToParty;
        }
        if (bonusToTarget.length > 0) {
            skill.bonusToTarget = bonusToTarget;
        }
    }


    fs.writeFileSync(dest, JSON.stringify(input, null, 2));
});
const bonusWithParamsRegex = /([A-z]*)\((.*)\)/;
function getBonus(skillConfig, noteBonus, valueFromDb) {
    let bonuses = [];
    if (noteBonus.includes("+")){
        bonuses = noteBonus.split("+")
    } else {
        bonuses.push(noteBonus)
    }
    return bonuses.map(bonus => {
        if (bonus.indexOf("#") > 0) {
            bonus = bonus.substring(0, bonus.indexOf("#"));
        }
        bonus = bonus.trim();
        let matches = bonus.match(bonusWithParamsRegex);
        if (matches && matches.length > 0) {
            bonus = matches[1];
            let v = matches[2];
            if (v.includes(",")) {
                let split = v.split(",");
                if (split[0].length > 0) {
                    if (split[0] === "$SkillId$") {
                        split[0] = skillConfig.id
                    }
                    let value = parseInt(split[0]) || split[0].trim();
                    if (value === "WeaponType") {
                        switch (skillConfig.name) {
                            case "SM_SWORD":
                                value = "1hSword"
                                break;
                            case "SM_TWOHAND":
                                value = "2hSword"
                                break;
                            case "KN_SPEARMASTERY":
                                value = "1hSpear"
                                break;
                            case "PR_MACEMASTERY":
                                value = "Mace"
                                break;
                            case "PR_MACEMASTERY":
                                value = "Mace"
                                break;
                            case "AS_KATAR":
                                value = "Katar"
                                break;
                            case "AM_AXEMASTERY":
                                value = "1hAxe"
                                break;
                            case "SA_ADVANCEDBOOK":
                                value = "Book"
                                break;
                            case "BD_RINGNIBELUNGEN":
                                value = "All"
                                break;
                            case "BA_MUSICALLESSON":
                                value = "Musical"
                                break;
                            case "DC_DANCINGLESSON":
                                value = "Whip"
                                break;
                            case "TK_RUN":
                                value = "Fist"
                                break;
                            case "NJ_TOBIDOUGU":
                                value = "Shuriken"
                                break;
                            case "MO_IRONHAND":
                                value = "Knuckle"
                                break;
                        }
                    }
                    return {
                        bonus: bonus,
                        value: value,
                        value2: valueFromDb,
                    }
                } else {
                    return {
                        bonus: bonus,
                        value: valueFromDb,
                        value2: parseInt(split[1]) || split[1].trim(),
                    }
                }
            } else {
                return {
                    bonus: bonus,
                    value: parseInt(v) || v.trim(),
                    value2: valueFromDb,
                }
            }
        } else {
            return {
                bonus: bonus,
                value: valueFromDb
            }
        }
    })
}