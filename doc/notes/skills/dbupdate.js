const fs = require('fs');

// let input = JSON.parse(fs.readFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill.json', 'utf8'))
let input = JSON.parse(fs.readFileSync('/home/nmeylan/dev/ragnarok/rust-ragnarok-server/config/skill.json', 'utf8'))

// let dest = "C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill-new.json";
let dest = "/home/nmeylan/dev/ragnarok/rust-ragnarok-server/config/skill.json";
for (let skill of input.skills) {
    if (skill.targetType === undefined) {
        skill["targetType"] = "Passive";
    }
}

fs.writeFileSync(dest, JSON.stringify(input, null, 2));