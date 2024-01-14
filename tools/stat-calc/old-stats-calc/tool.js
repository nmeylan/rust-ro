function selectOptionsCount(selectName) {
    return document.querySelectorAll("select[name=" + selectName + "] > option").length
}

function selectOption(selectName, index) {
    let element = document.querySelector("select[name=" + selectName + "]");
    element.click();
    element.selectedIndex = index;
    element.onchange();
}

function generateAttackSkillTestCases() {
    const baseLevelSelect = "A_BaseLV";
    const jobLevelSelect = "A_JobLV";
    const activeSkillSelect = "A_ActiveSkill";
    const strSelect = "A_STR";
    const agiSelect = "A_AGI";
    const intSelect = "A_INT";
    const dexSelect = "A_DEX";
    const vitSelect = "A_VIT";
    const lukSelect = "A_LUK";
    const jobSelect = "A_JOB";
    const weaponTypeSelect = "A_WeaponType";
    const baseLevels = [1, 10, 54, 78, 96, 99];
    const jobsCount = selectOptionsCount(jobSelect);

    for (let jobIndex = 0; jobIndex < jobsCount; jobIndex++) {
        selectOption(jobSelect, jobIndex);
        let weaponTypeCount = selectOptionsCount(weaponTypeSelect);
        for(baseLevel of baseLevels) {
            selectOption(baseLevelSelect, baseLevel - 1);
            let activeSkillCount = selectOptionsCount(activeSkillSelect);
            // for(let weaponTypeIndex = 0;  weaponTypeIndex < weaponTypeCount; weaponTypeIndex++) {
            for(let weaponTypeIndex = 0;  weaponTypeIndex < weaponTypeCount; weaponTypeIndex++) {
                selectOption(weaponTypeSelect, weaponTypeIndex);
                for(let activeSkillIndex = 0; activeSkillIndex < activeSkillCount; activeSkillIndex++) {
                    selectOption(activeSkillSelect, activeSkillIndex);
                    GenerateTestCase();
                }
            }
        }
    }
}
