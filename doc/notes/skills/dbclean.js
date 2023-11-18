const fs = require('fs');
let input = JSON.parse(fs.readFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\doc\\notes\\skills\\wip-db.json', 'utf8'))

const ids = input.map(entry => entry.id);
input = input.filter(({id}, index) => !ids.includes(id, index + 1));
function camelToSnake(str) {
	return str.replace(/([A-Z])/g, letter => `_${letter.toLowerCase()}`);
}
function cleanData(oldValue, k) {
	if (Array.isArray(oldValue)) {
		return oldValue
	}
	let value = oldValue;
	if (value.includes(" chance")) {
		value = value.replace(" chance", "")
	}
	if (value.includes(" cells")) {
		value = value.replace(" cells", "")
	}
	if (value.includes(" cell")) {
		value = value.replace(" cell", "")
	}
	if (k.toLowerCase().includes("chance") || k.toLowerCase().includes("percentage")) {
		value = value.replace("%", "")
	}
	if (k.toLowerCase().includes("delayinsec")) {
		value = value.replace(" s", "")
	}
	if (k.toLowerCase().includes("duration")) {
		value = value.replace(" s", "")
	}
	if (value.startsWith("+")) {
		value = value.substring(1)
	}
	value = value.replace(",", "");
	if (value.match(/^[\\-]?\d+([.]\d+)?$/)) {
		value = Number.parseFloat(value);
	}
	return value;
}

let keysWithIssue = {};
let uniqueKey = {};

input = input.filter(entry => Object.keys(entry).length > 1)
for (let entry of input) {
	for (let k of Object.keys(entry)) {
		if (Array.isArray(entry[k])) {
			if (!entry[k][0].value) continue;
			let newKey = k;
			if (entry[k][0].value.includes("% chance") && !k.includes("ChancePerLevel")) {
				newKey = k.replace("PerLevel", "ChancePerLevel");
				entry[newKey] = entry[k];
				delete entry[k]
			} else if (entry[k][0].value.includes("%") && !k.includes("PercentagePerLevel") && !k.includes("ChancePerLevel")) {
				newKey = k.replace("PerLevel", "PercentagePerLevel");
				entry[newKey] = entry[k];
				delete entry[k]
			}
			k = newKey;
			if (!uniqueKey[k]) {
				uniqueKey[k] = {count: 1, ids: [entry.id]};
			} else {
				uniqueKey[k].count += 1;
				uniqueKey[k].ids.push(entry.id);
			}
			for (let kv of entry[k]) {
				kv.value = cleanData(kv.value, k);
			}
		} else {
			let newKey = k;
			if (entry[k].includes("% chance") && !k.includes("Chance")) {
				newKey = k + "Chance";
				entry[k + "Chance"] = entry[k];
				delete entry[k]

			} else if (entry[k].includes("%") && !k.includes("Percentage") && !k.includes("Chance")) {
				newKey = k + "Percentage";
				entry[k + "Percentage"] = entry[k];
				delete entry[k]
			}
			k = newKey;
			if (!uniqueKey[k]) {
				uniqueKey[k] = {count: 1, ids: [entry.id]};
			} else {
				uniqueKey[k].count += 1;
				uniqueKey[k].ids.push(entry.id);
			}
			entry[k] = cleanData(entry[k], k);
		}
	}
}

let entries = Object.entries(uniqueKey);

entries.sort((a, b) => b[1].count - a[1].count);
entries = entries.filter(e => e[1].count >= 1)

console.log("Unique key count:", entries.length);
for (let [k, v] of entries) {
//	console.log(k, "- count:", v);
	if (k.includes("PerLevel")) {
		console.log("#[serde(rename = \""+k+"\", deserialize_with = \"deserialize_tuples_i32\", default)]");
		console.log(camelToSnake(k) + ":", "Option<Vec<i32>>,");
	} else {
		console.log("#[serde(rename = \""+k+"\", default)]");
		console.log(camelToSnake(k) + ":", "Option<i32>,");
	}
}

let original = JSON.parse(fs.readFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill.json', 'utf8'))


for (let originalEntry of original.skills) {
	let matchingNewEntry = input.find(e => e.id === originalEntry.id);
	if (matchingNewEntry) {
		for (let [k,v ] of Object.entries(matchingNewEntry))  {
			if (k !== "id") {
				originalEntry[k] = v;
			}

		}
	}
}
fs.writeFileSync('C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\skill.json', JSON.stringify(original, null, 2));