
import "./db.js"
import fs from "fs";


let jobs = JSON.parse(fs.readFileSync("C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\job.json"));
for (let i = 0; i < JobStatsBonusPerLevel.length; i++) {
    // console.log(JobIds[i])
    Object.values(jobs.jobs).forEach(k => {
        if (k.id === JobIds[i] && JobStatsBonusPerLevel[i]) {
            k.bonus_stats = []
            for(let j = 0; j < JobStatsBonusPerLevel[i].length; j++) {
                k.bonus_stats.push({
                    str: JobStatsBonusPerLevel[i][j][0],
                    agi: JobStatsBonusPerLevel[i][j][1],
                    vit: JobStatsBonusPerLevel[i][j][2],
                    int: JobStatsBonusPerLevel[i][j][3],
                    dex: JobStatsBonusPerLevel[i][j][4],
                    luk: JobStatsBonusPerLevel[i][j][5],
                });
            }
        }
    })
}

fs.writeFileSync("C:\\dev\\ragnarok\\rust-ragnarok-server\\config\\job.json",  JSON.stringify(jobs, null, 2));