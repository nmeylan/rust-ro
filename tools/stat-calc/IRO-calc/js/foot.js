function SuperNoviceFullWeapon(nSNFW)
{
	if ( nSNFW == 1 )
	{
		SuperNoviceFullWeaponCHECK = 1;
		JobASPD[20][7] = 120;
	}
	else
	{
		SuperNoviceFullWeaponCHECK = 0;
		JobASPD[20][7] = 0;
	}

	var len = document.calcForm.A_WeaponType.length;
	for ( var i = 0; i < len; i++ )
	{
		document.calcForm.A_WeaponType.options[0] = null;
	}

	var j = 0;
	for ( var i = 0; i <= 21; i++ )
	{
		if ( JobASPD[20][i] != 0 )
		{
			document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i][Language],i);
			j++;
		}
	}

	with ( document.calcForm )
	{
		if ( ItemOBJ[n_A_Equip[0]][2] !== 7 && JobEquipItemSearch( ItemOBJ[n_A_Equip[0]][2] ) )
		{
			var BK_BUKI = n_A_Equip[0];
			A_WeaponType.value = ItemOBJ[BK_BUKI][1];
			ClickWeaponType(ItemOBJ[BK_BUKI][1]);

			WeaponSet2();
			A_weapon1.value = BK_BUKI;
		}
		else
		{
			ClickWeaponType(0);

			WeaponSet2();
		}

		if(JobEquipItemSearch(ItemOBJ[n_A_Equip[2]][2]))
		{
			A_head1.value = n_A_Equip[2];
		}

		if(JobEquipItemSearch(ItemOBJ[n_A_Equip[3]][2]))
		{
			A_head2.value = n_A_Equip[3];
		}

		if(JobEquipItemSearch(ItemOBJ[n_A_Equip[4]][2]))
		{
			A_head3.value = n_A_Equip[4];
		}

		A_left.value = n_A_Equip[eq_SHIELD];
		A_body.value = n_A_Equip[6];
		A_shoulder.value = n_A_Equip[7];
		A_shoes.value = n_A_Equip[8];
		A_acces1.value = n_A_Equip[9];
		A_acces2.value = n_A_Equip[10];
	}

	if ( n_SaveMode === 0 )
	{
		SetShortCut();
	}
}

function StAllCalc()
{ // stat- & subStatCalc
with ( document.calcForm )
{
	var formElements = document.forms["calcForm"].elements;
	n_A_JobSet();

{ // get rid of those variables --------------------
	n_A_WeaponLV_seirenATK = 0;
	n_A_WeaponLV_Minplus = 0;
	n_A_WeaponLV_Maxplus = 0;
	n_A_Weapon2LV_seirenATK = 0;
	n_A_Weapon2LV_Minplus = 0;
	n_A_Weapon2LV_Maxplus = 0;
}

{ // Super Novis Bonus -----------------------------------------------
	if ( n_A_JOB === cls_SNOVI || n_A_JOB === cls_ENOVI )
	{
		skillValue = parseInt(formElements["A_Skill9"].value);
		if ( SuperNoviceFullWeaponCHECK == 0 && skillValue === 1 )
		{
			SuperNoviceFullWeapon(1);
		}
		else if ( SuperNoviceFullWeaponCHECK == 1 && skillValue === 0 )
		{
			SuperNoviceFullWeapon(0);
		}
	}
}

{ // LoadStats -----------------------------------------------
	n_A_BaseLV = parseInt(formElements["A_BaseLV"].value);
	n_A_JobLV = parseInt(formElements["A_JobLV"].value);
	n_A_STR = parseInt(formElements["A_STR"].value);
	n_A_AGI = parseInt(formElements["A_AGI"].value);
	n_A_VIT = parseInt(formElements["A_VIT"].value);
	n_A_INT = parseInt(formElements["A_INT"].value);
	n_A_DEX = parseInt(formElements["A_DEX"].value);
	n_A_LUK = parseInt(formElements["A_LUK"].value);

	// BaseStats
	SU_STR = n_A_STR;
	SU_AGI = n_A_AGI;
	SU_VIT = n_A_VIT;
	SU_INT = n_A_INT;
	SU_DEX = n_A_DEX;
	SU_LUK = n_A_LUK;
}

	n_A_WeaponType = parseInt(formElements["A_WeaponType"].value);
	n_A_Arrow = parseInt(formElements["A_Arrow"].value);

{ // LoadUpgrades -----------------------------------------------
	// Armor
	n_A_HEAD_DEF_PLUS = parseInt(formElements["A_HEAD_DEF_PLUS"].value);
	n_A_BODY_DEF_PLUS = parseInt(formElements["A_BODY_DEF_PLUS"].value);
	n_A_LEFT_DEF_PLUS = parseInt(formElements["A_LEFT_DEF_PLUS"].value);
	n_A_SHOES_DEF_PLUS = parseInt(formElements["A_SHOES_DEF_PLUS"].value);
	n_A_SHOULDER_DEF_PLUS = parseInt(formElements["A_SHOULDER_DEF_PLUS"].value);

	// Weapons
	n_A_Weapon_ATKplus = parseInt(formElements["A_Weapon_ATKplus"].value);
	n_A_Weapon2_ATKplus = 0;
	if ( n_Nitou )
	{ // Dual Hand
		n_A_Weapon2_ATKplus = parseInt(formElements["A_Weapon2_ATKplus"].value);
	}
}

{ // LoadActiveSkill -----------------------------------------------
	n_A_ActiveSkill = parseInt(formElements["A_ActiveSkill"].value);
	if ( n_A_ActiveSkill >= 3000 )
	{
		n_A_ActiveSkill = InsertSkill[n_A_ActiveSkill -3000][2];
	}
	else if ( n_A_ActiveSkill >= 2000 )
	{
		n_A_ActiveSkill = AutoSpellSkill[n_A_ActiveSkill -2000][2];
	}

	n_A_ActiveSkillLV = parseInt(formElements["A_ActiveSkillLV"].value);

	n_A_MobSkill = parseInt(formElements["A_ActiveSkill_en"].value);
	n_A_MobSkillLV = parseInt(formElements["A_ActiveSkillLV_en"].value);
}

{ // LoadEquip ----------------------------------------------------
	n_A_Equip[eq_WEAPON] = parseInt(formElements["A_weapon1"].value);
	n_A_Equip[eq_LEFT_WEAPON] = 0;
	if ( n_Nitou )
	{ // left weapon on
		n_A_Equip[eq_LEFT_WEAPON] = parseInt(formElements["A_weapon2"].value);
	}
	n_A_Equip[eq_HEAD_UPPER] = parseInt(formElements["A_head1"].value);
	n_A_Equip[eq_HEAD_MIDDLE] = parseInt(formElements["A_head2"].value);
	n_A_Equip[eq_HEAD_LOWER] = parseInt(formElements["A_head3"].value);
	n_A_Equip[eq_SHIELD] = parseInt(formElements["A_left"].value);
	n_A_Equip[eq_ARMOR] = parseInt(formElements["A_body"].value);
	n_A_Equip[eq_GARMENT] = parseInt(formElements["A_shoulder"].value);
	n_A_Equip[eq_SHOES] = parseInt(formElements["A_shoes"].value);
	n_A_Equip[eq_ACCI] = parseInt(formElements["A_acces1"].value);
	n_A_Equip[eq_ACCII] = parseInt(formElements["A_acces2"].value);

	SetEquip();
}

{ // LoadCards ----------------------------------------------------
	// weapons
	n_A_card[card_loc_WEAPON_I] = parseInt(formElements["A_weapon1_card1"].value);
	n_A_card[card_loc_WEAPON_II] = parseInt(formElements["A_weapon1_card2"].value);
	n_A_card[card_loc_WEAPON_III] = parseInt(formElements["A_weapon1_card3"].value);
	n_A_card[card_loc_WEAPON_IV] = parseInt(formElements["A_weapon1_card4"].value);
	if ( n_Nitou )
	{ // DualHand
		n_A_card[card_loc_WEAPONII_I] = parseInt(formElements["A_weapon2_card1"].value);
		n_A_card[card_loc_WEAPONII_II] = parseInt(formElements["A_weapon2_card2"].value);
		n_A_card[card_loc_WEAPONII_III] = parseInt(formElements["A_weapon2_card3"].value);
		n_A_card[card_loc_WEAPONII_IV] = parseInt(formElements["A_weapon2_card4"].value);
	}
	else
	{
		for ( var i = 4; i <= 7; i++ )
		{
			n_A_card[i] = 0;
		}
	}

	// armor
	n_A_card[card_loc_HEAD_UPPER] = parseInt(formElements["A_head1_card"].value);
	n_A_card[card_loc_HEAD_MIDDLE] = parseInt(formElements["A_head2_card"].value);
	n_A_card[card_loc_SHIELD] = parseInt(formElements["A_left_card"].value);
	n_A_card[card_loc_ARMOR] = parseInt(formElements["A_body_card"].value);
	n_A_card[card_loc_GARMENT] = parseInt(formElements["A_shoulder_card"].value);
	n_A_card[card_loc_SHOES] = parseInt(formElements["A_shoes_card"].value);
	n_A_card[card_loc_ACCI] = parseInt(formElements["A_acces1_card"].value);
	n_A_card[card_loc_ACCII] = parseInt(formElements["A_acces2_card"].value);

	SetCard();
}

{ // Buffs and other effects ----------------------------
	// Passive Jobskills
	for ( var i = 0; JobSkillPassOBJ[n_A_JOB][i] != 999; i++ )
	{
		var formElement = formElements["A_Skill" + i];
		selfBuffs[i] = parseInt( formElement.value );
	}

	{ // Acolyte Skills
		acolyteBuffs[ksBlessing] = parseInt(formElements["blessing"].value);
		acolyteBuffs[ksIncreaseAgi] = parseInt(formElements["increaseAgi"].value);
		acolyteBuffs[ksAngelus] = parseInt(formElements["angelus"].value);
		acolyteBuffs[ksImposito] = parseInt(formElements["imposito"].value);
		acolyteBuffs[ksSuffragium] = parseInt(formElements["suffragium"].value);
		acolyteBuffs[ksGloria] = formElements["gloria"].checked;
		acolyteBuffs[ksAssumptio] = parseInt(formElements["assumptio"].value);
		acolyteBuffs[ksSpheres] = parseInt(formElements["spheres"].value);
		acolyteBuffs[ksClementia] = parseInt(formElements["clementia"].value);
		acolyteBuffs[ksCandidus] = parseInt(formElements["candidus"].value);
		acolyteBuffs[ksExpiatio] = parseInt(formElements["expiatio"].value);
		acolyteBuffs[ksSacrament] = parseInt(formElements["sacrament"].value);
		acolyteBuffs[ksLaudaAgnus] = parseInt(formElements["agnus"].value);
		acolyteBuffs[ksLaudaRamus] = parseInt(formElements["ramus"].value);
		acolyteBuffs[ksPPChange] = parseInt(formElements["ppChange"].value);
		acolyteBuffs[ksPPRevitalize] = parseInt(formElements["ppRevitalize"].value);
		acolyteBuffs[ksSuraStrength] = parseInt(formElements["suraStr"].value);
		acolyteBuffs[ksSuraAgility] = parseInt(formElements["suraAgi"].value);
		acolyteBuffs[ksSuraVitality] = parseInt(formElements["suraVit"].value);
		acolyteBuffs[ksSuraIntelligence] = parseInt(formElements["suraInt"].value);
		acolyteBuffs[ksSuraDexterity] = parseInt(formElements["suraDex"].value);
	}

	{ // Performer Skills
		performerBuffs[ksBardSolo] = parseInt(formElements["bardSkills"].value);
		performerBuffs[ksBardSoloLevel] = parseInt(formElements["bardSkillLevel"].value);
		performerBuffs[ksMusicLessons] = parseInt(formElements["musicLessons"].value);
		performerBuffs[ksBardAgi] = parseInt(formElements["bardAgi"].value);
		performerBuffs[ksBardInt] = parseInt(formElements["bardInt"].value);
		performerBuffs[ksBardDex] = parseInt(formElements["bardDex"].value);
		performerBuffs[ksBardVit] = parseInt(formElements["bardVit"].value);
		performerBuffs[ksMaestroSolo] = parseInt(formElements["maestroSkills"].value);
		performerBuffs[ksMaestroSoloLevel] = parseInt(formElements["maestroSkillLevel"].value);
		performerBuffs[ksMaestroVoiceLessons] = parseInt(formElements["voiceLessonsM"].value);
		performerBuffs[ksMaestroJobLevel] = parseInt(formElements["bardJob"].value);

		performerBuffs[ksDancerSolo] = parseInt(formElements["dancerSkills"].value);
		performerBuffs[ksDancerSoloLevel] = parseInt(formElements["dancerSkillLevel"].value);
		performerBuffs[ksDanceLessons] = parseInt(formElements["danceLessons"].value);
		performerBuffs[ksDancerAgi] = parseInt(formElements["dancerAgi"].value);
		performerBuffs[ksDancerInt] = parseInt(formElements["dancerInt"].value);
		performerBuffs[ksDancerDex] = parseInt(formElements["dancerDex"].value);
		performerBuffs[ksDancerLuk] = parseInt(formElements["dancerLuk"].value);
		performerBuffs[ksWandererSolo] = parseInt(formElements["wandererSkills"].value);
		performerBuffs[ksWandererSoloLevel] = parseInt(formElements["wandererSkillLevel"].value);
		performerBuffs[ksWandererVoiceLessons] = parseInt(formElements["voiceLessonsW"].value);
		performerBuffs[ksWandererJobLevel] = parseInt(formElements["dancerJob"].value);

		performerBuffs[ksEnsemble] = parseInt(formElements["ensembleSkills"].value);
		performerBuffs[ksBardEnsembleLevel] = parseInt(formElements["bardLevel"].value);
		performerBuffs[ksDancerEnsembleLevel] = parseInt(formElements["dancerLevel"].value);
		if ( performerBuffs[ksBardEnsembleLevel] > 0 && performerBuffs[ksDancerEnsembleLevel] > 0 )
		{
			var ensembleLevel = ( performerBuffs[ksBardEnsembleLevel] + performerBuffs[ksDancerEnsembleLevel] ) / 2;
			performerBuffs[ksEnsembleLevel] = Math.floor( ensembleLevel );
		}
		else
		{
			performerBuffs[ksEnsembleLevel] = 0;
		}
		performerBuffs[ksChorus] = parseInt(formElements["chorusSkill"].value);
		performerBuffs[ksChorusLevel] = parseInt(formElements["chorusLevel"].value);
		performerBuffs[ksNumPerformers] = parseInt(formElements["performerCount"].value);

		performerBuffs[ksMarionette] = formElements["marionetteControl"].checked;
		performerBuffs[ksPerformerStr] = parseInt(formElements["marionetteStr"].value);
		performerBuffs[ksPerformerAgi] = parseInt(formElements["marionetteAgi"].value);
		performerBuffs[ksPerformerVit] = parseInt(formElements["marionetteVit"].value);
		performerBuffs[ksPerformerInt] = parseInt(formElements["marionetteInt"].value);
		performerBuffs[ksPerformerDex] = parseInt(formElements["marionetteDex"].value);
		performerBuffs[ksPerformerLuk] = parseInt(formElements["marionetteLuk"].value);
	}

	{ // Guild Skills
		guildBuffs[pass_IV_BAT_ORDER] = formElements["guildSkill0"].checked;
		guildBuffs[pass_IV_GRE_LEADER] = parseInt(formElements["guildSkill1"].value);
		guildBuffs[pass_IV_WOU_GLORY] = parseInt(formElements["guildSkill2"].value);
		guildBuffs[pass_IV_SOU_COLD] = parseInt(formElements["guildSkill3"].value);
		guildBuffs[pass_IV_SHA_EYES] = parseInt(formElements["guildSkill4"].value);
	}

	{ // Battle Chant
		battleChantBuffs[pass_V_STATS] = formElements["battleChant0"].checked;
		battleChantBuffs[pass_V_HP] = formElements["battleChant1"].checked;
		battleChantBuffs[pass_V_SP] = formElements["battleChant2"].checked;
		battleChantBuffs[pass_V_ATK] = formElements["battleChant3"].checked;
		battleChantBuffs[pass_V_HIT_FLEE] = formElements["battleChant4"].checked;
		battleChantBuffs[pass_V_DAMAGE] = formElements["battleChant5"].checked;
	}

	{ // Other Buffs
		otherBuffs[ksElementField] = parseInt(formElements["eleField"].value);
		otherBuffs[ksElementFieldLvl] = parseInt(formElements["eleFieldLvl"].value);
		otherBuffs[ksMurderBonus] = parseInt(formElements["murderBonus"].value);
		otherBuffs[ksImproveConcentration] = parseInt(formElements["improveCon"].value);
		otherBuffs[ksMindBreaker] = parseInt(formElements["mindBreaker"].value);
		otherBuffs[ksProvoke] = parseInt(formElements["provoke"].value);
		otherBuffs[ksBSS] = formElements["bss"].checked;
		otherBuffs[ksAdrenalineRush] = parseInt(formElements["adrenalineRush"].value);
		otherBuffs[ksWeaponPerfection] = formElements["weaponPerfection"].checked;
		otherBuffs[ksPowerThrust] = parseInt(formElements["powerThrust"].value);
		otherBuffs[ksWindWalker] = parseInt(formElements["windWalker"].value);
		otherBuffs[ksMagnumBreak] = formElements["magnumBreak"].checked;
		otherBuffs[ksAloe] = formElements["aloe"].checked;
		otherBuffs[ksResistantSouls] = parseInt(formElements["resistantSouls"].value);
		otherBuffs[ksStriking] = parseInt(formElements["striking"].value);
		otherBuffs[ksStrikingEndowBonus] = parseInt(formElements["strikingEndow"].value);
		otherBuffs[ksOdinsPower] = parseInt(formElements["odinsPower"].value);
		//otherBuffs[ksFriggsSong] = parseInt(formElements["friggsSong"].value);
	}

	{ // Misc Effects
		miscEffects[ksPetEffects] = parseInt(formElements["petBonus"].value);
		miscEffects[ksSupNovMarriage] = formElements["noviceMarried"].checked;
		miscEffects[ksFirstTempEffect] = parseInt(formElements["tempOne"].value);
		miscEffects[ksSecondTempEffect] = parseInt(formElements["tempTwo"].value);
		miscEffects[ksThirdTempEffect] = parseInt(formElements["tempThree"].value);
		miscEffects[ksFourthTempEffect] = parseInt(formElements["tempFour"].value);
		miscEffects[ksNumEnemies] = parseInt(formElements["numEnemies"].value);
		miscEffects[ksTransFirstSpirit] = formElements["firstSpirit"].checked;
		miscEffects[ksNoCrit] = formElements["playerNoCrit"].checked;
		miscEffects[ksQuagmire] = parseInt(formElements["playerQuaged"].value);
		miscEffects[ksAgiDown] = parseInt(formElements["playerAgiDowned"].value);
		miscEffects[ksPoisoned] = formElements["playerPoisoned"].checked;
		miscEffects[ksCursed] = formElements["playerCursed"].checked;
	}
	{ // Usable Items
		usableItems[ksSesamePastry] = formElements["sesamePastry"].checked;
		usableItems[ksHoneyPastry] = formElements["honeyPastry"].checked;
		usableItems[ksRainbowCake] = formElements["rainbowCake"].checked;
		usableItems[ksStrengthFood] = parseInt(formElements["strFood"].value);
		usableItems[ksAgilityFood] = parseInt(formElements["agiFood"].value);
		usableItems[ksVitalityFood] = parseInt(formElements["vitFood"].value);
		usableItems[ksIntelligenceFood] = parseInt(formElements["intFood"].value);
		usableItems[ksDexterityFood] = parseInt(formElements["dexFood"].value);
		usableItems[ksLuckFood] = parseInt(formElements["lukFood"].value);
		usableItems[ksBoxOfResentment] = formElements["resentment"].checked;
		usableItems[ksBoxOfDrowsiness] = formElements["drowsiness"].checked;
		usableItems[ksColdproof] = formElements["coldproof"].checked;
		usableItems[ksEarthproof] = formElements["earthproof"].checked;
		usableItems[ksFireproof] = formElements["fireproof"].checked;
		usableItems[ksThunderproof] = formElements["thunderproof"].checked;
		usableItems[ksCastScrolls] = formElements["castScrolls"].checked;
		usableItems[ksBoucheDeNoel] = formElements["boucheDeNoel"].checked;
		usableItems[ksRuneStrawberryCake] = formElements["runeStrawberry"].checked;
		usableItems[ksSchwartzwaldPineJubilee] = formElements["pineJubilee"].checked;
		usableItems[ksArunafeltzDesertSandwich] = formElements["desertSandwich"].checked;
		usableItems[ksDurian] = formElements["durian"].checked;
		usableItems[ksCelermineJuice] = formElements["celermineJuice"].checked;
		usableItems[ksGuaranaCandy] = formElements["guaranaCandy"].checked;
		usableItems[ksLuckyRiceCake] = formElements["luckyRiceCake"].checked;
		usableItems[ksMilitaryRationB] = formElements["militaryRationB"].checked;
		usableItems[ksMilitaryRationC] = formElements["militaryRationC"].checked;
		usableItems[ksPinkRation] = formElements["pinkRation"].checked;
		usableItems[ksWhiteRation] = formElements["whiteRation"].checked;
		usableItems[ksVitataFiveHundred] = formElements["vitataFiveHundred"].checked;
		usableItems[ksAttackSpeed] = parseInt(formElements["speedPot"].value);
		usableItems[ksDistilledFightingSpirit] = formElements["fightingSpirit"].checked;
		usableItems[ksIncreaseHP] = parseInt(formElements["increaseHpPotion"].value);
		usableItems[ksIncreaseSP] = parseInt(formElements["increaseSpPotion"].value);
		usableItems[ksVipBuffs] = formElements["vipBuff"].checked;
		//usableItems[ksAbrasive] = formElements["abrasive"].checked;
		//usableItems[ksHolyElemental] = formElements["holyEl"].checked;
		//usableItems[ksUndeadElemental] = formElements["undeadEl"].checked;
	}

	{ // Additional Battle Effects
		battleEffects[pass_VIII_BAT_MANUAL] = parseInt(formElements["baseManual"].value);
		battleEffects[pass_VIII_JOB_MANUAL] = formElements["jobManual"].checked;
		battleEffects[pass_VIII_VIPSTATUS] = formElements["vipStatus"].checked;
		battleEffects[pass_VIII_PAR_COUNT] = parseInt(formElements["partySize"].value);
		battleEffects[pass_VIII_EXP_TAB] = parseInt(formElements["expTap"].value);
		battleEffects[pass_VIII_SER_EXP] = parseInt(formElements["serverExp"].value);
		battleEffects[pass_VIII_SPE_ENVIRONMENT] = parseInt(formElements["specialEnv"].value);
		battleEffects[pass_VIII_DEF_INVEST] = parseInt(formElements["castleDefense"].value);
	}
}

{ // Inc Damage with Crit -------------------------------------
	if ( EquipNumSearch( 1080 ) && n_A_Weapon_ATKplus >= 6 )
	{ // Glorious Claymore
		n_tok[bon_DEFIGN_RC_DEMI_HUMAN] += 5;
	}
	if( n_A_Equip[1] == 1076 && n_A_Weapon2_ATKplus >= 6 )
	{ // glorious gladius
		n_tok[bon_DEFIGN_RC_DEMI_HUMAN] += 5;
	}
	if( n_A_Equip[1] == 1077 && n_A_Weapon2_ATKplus >= 6 )
	{ // glorious flamberge
		n_tok[bon_DEFIGN_RC_DEMI_HUMAN] += 5;
	}
	if ( 1076 <= n_A_Equip[0] && n_A_Equip[0] <= 1103 )
	{
		// Glorious Weapon Def Ignore
		if ( n_A_Weapon_ATKplus >= 6 )
		{
			if( n_A_Equip[0] !== 1078 &&
			    n_A_Equip[0] !== 1079 &&
			    n_A_Equip[0] !== 1080 &&
			    n_A_Equip[0] !== 1083 &&
			    n_A_Equip[0] !== 1084 &&
			    n_A_Equip[0] !== 1085 &&
			    n_A_Equip[0] !== 1091 &&
			    n_A_Equip[0] !== 1095 )
			{
				n_tok[bon_DEFIGN_RC_DEMI_HUMAN] += 5;
			}
		}
	}

	if ( acolyteBuffs[ksExpiatio] )
	{ // expiatio
		n_tok[bon_DEFIGN_RC_FORMLESS] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_UNDEAD] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_BRUTE] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_PLANT] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_INSECT] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_FISH] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_DEMON] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_DEMI_HUMAN] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_ANGEL] += acolyteBuffs[ksExpiatio] * 5;
		n_tok[bon_DEFIGN_RC_DRAGON] += acolyteBuffs[ksExpiatio] * 5;
	}

	if ( EquipNumSearch(645) )
	{ // Piercing Staff
		n_tok[bon_DEFIGN_RC_ALL] += 10 + n_A_Weapon_ATKplus;
	}
	if ( n_A_WeaponType === weapTyp_STAFF )
	{ // necromancer card
		n_tok[bon_DEFIGN_RC_ALL] += 2 * CardNumSearch(card_WEPN_NECROMANCER);
	}
	if ( n_B[en_BOSS]==1 && CardNumSearch(card_HEAD_VESPER) )
	{ // Vesper card
		n_tok[297] += 30 * CardNumSearch(card_HEAD_VESPER);
	}
	if ( EquipNumSearch(936) )
	{ // Thorn Staff of Darkness
		n_tok[bon_DEFIGN_RC_ALL] += n_A_Weapon_ATKplus * 1;
	}
	if ( n_B[en_BOSS] == 1 && EquipNumSearch(1228) )
	{ // Southern Cross headgear?
		if(n_A_HEAD_DEF_PLUS >= 6)
			n_tok[297] += (n_A_HEAD_DEF_PLUS - 5);
	}
	if ( n_B[en_BOSS] == 1 )
	{
		n_tok[bon_DEFIGN_RC_ALL] += n_tok[297];
	}

	if ( EquipNumSearch(1083) && n_A_Weapon_ATKplus >= 6 )
	{ // glorious destruction staff
		n_tok[bon_MDEFIGN_RC_DEMI_HUMAN] += Math.min(20, 2*(n_A_Weapon_ATKplus-5));
	}
	if ( (EquipNumSearch(1084) || EquipNumSearch(1085) || EquipNumSearch(1095)) && n_A_Weapon_ATKplus >= 6 )
	{ // glorious staffs
		n_tok[bon_MDEFIGN_RC_DEMI_HUMAN] += 5;
	}
}

 // LoadEnemy - -------------------------------------------------
	BuildEnemyData();

	// Adopted ------------------------------------------------------
	n_A_Adopted = formElements["A_youshi"].checked;


 	// Weap- & ArmorElement --------------------------------------------------
	getWeaponElement();
	n_A_BodyZokusei = getArmorElement( n_A_BodyZokusei = 0 );

	// fill Substats (n_tok[x]) ------------------------------------
	ClearBonuses();

	// Bonus Stats
	StPlusCalc(); // set bStat + %

 	LoadStatsFromScripts();
 	// CalcHP --------------------------------------------------------
	n_A_MaxHP = calcHP();
	myInnerHtml("A_MaxHP",n_A_MaxHP,0);

 	// CalcSP -------------------------------------------------------
	n_A_MaxSP = calcSP(n_A_MaxSP=0);
	myInnerHtml("A_MaxSP",n_A_MaxSP,0);

 	// CalcDEF -------------------------------------------------------
	n_A_totalDEF = calcHardDef(n_A_totalDEF=0);
 	n_A_VITDEF = calcSoftDef(n_A_VITDEF=0);
	STR = n_A_VITDEF + " + " + n_A_totalDEF + "<BR>(" + Math.floor((1-defReduction(n_A_totalDEF))*1000)/10 + "% Reduction)";
	myInnerHtml("A_totalDEF", STR,0);

 	// CalcMDEF -----------------------------------------------
	n_A_MDEF = calcHardMDef(n_A_MDEF=0);
 	n_A_INTMDEF = calcSoftMDef(n_A_INTMDEF=0);
	STR = n_A_INTMDEF + " + " + n_A_MDEF + "<BR>(" + Math.floor((1-mdefReduction(n_A_MDEF))*1000)/10 + "% Reduction)";
	myInnerHtml("A_MDEF", STR,0);

 	// CalcHit/Flee/ PDodge -------------------------------------------
	n_A_HIT = calcHit(n_A_HIT=0);
	myInnerHtml("A_HIT",n_A_HIT,0);

 	n_A_FLEE = calcFlee(n_A_FLEE=0);
	myInnerHtml("A_FLEE",n_A_FLEE,0);

	n_A_LUCKY = calcPDodge(n_A_LUCKY=0);
	myInnerHtml("A_LUCKY",n_A_LUCKY,0);

 	// CalcCrit ------------------------------------------------------
	n_A_CRI = calcCrit(n_A_CRI=0);
	myInnerHtml("A_CRI",n_A_CRI,0);

 	// CalcAtk -----------------------------------------------------
	var totalAtk = GetDisplayAtk();
	var displayAtk = n_A_Weapon_ATK + weaponUpgradeAttack + equipmentAttack;
	STR = Math.floor(statusAttack) +" + "+ displayAtk;
	STR += "<br/>(" + Math.floor( statusAttack * 2 * element[n_B[en_ELEMENT]][ele_NEUTRAL] / 100 );
	STR += " + " + Math.floor( totalAtk * element[n_B[en_ELEMENT]][n_A_Weapon_element] / 100 ) + " &plusmn; ";
	STR += Math.floor( varianceAttack * element[n_B[en_ELEMENT]][n_A_Weapon_element] / 100 ) + ")";

	myInnerHtml("A_ATK",STR,0);

 	// CalcMATK ----------------------------------------------------
	calcMAtk(0);
	STR = n_A_StatMATK +" + "+ n_A_EquipMATK;
	calcMAtk(1);
	STR += "<BR>(" + n_A_StatMATK +" + "+ n_A_EquipMATK + " &plusmn; " + n_A_MATK_Variance + ")";

	myInnerHtml("A_MATK", STR,0);

 	// CalcASPD (Display) ------------------------------------------
	n_A_ASPD = calcASPD();
	myInnerHtml( "A_ASPD", n_A_ASPD, 0 );

{ // CalcDelays -----------------------------------------------
	// n_Delay[ksDelayA] = AL_BOMB/ HeatWall
	// n_Delay[ksDelayASPD] = ASPD
	// n_Delay[ksDelayGlobal] = Global AfterCast Delay
	// n_Delay[ksDelayAnimation] = Fixed animation
	// n_Delay[ksDelayE] = input limit
	// n_Delay[ksDelayF] = Heat (s/hit ?)
	// n_Delay[ksDelaySkillDuration] = GravField (fixed ?)
	// n_Delay[ksDelayCooldown] = skill re-use delay

	for ( var i = 0; i <= 7; i++ )
	{
		n_Delay[i] = 0;
	}

	n_A_ASPD = ( 200 - n_A_ASPD ) / 50;

	n_Delay[ksDelayASPD] = Math.floor( n_A_ASPD * 1000 ) / 1000;
	if ( n_A_ActiveSkill === skill_TH_ENVENOM )
	{
		n_Delay[ksDelayASPD] = Math.floor( n_A_ASPD * 75) / 100;
	}

	sandanDelay = 0;
	if(SkillSearch(skill_MO_RAGING_TRIFECTA_BLOW) && n_A_ActiveSkill === skill_ALL_BASIC_ATTACK)
	{
		sandanDelay = (1000 - n_A_AGI *4 - n_A_DEX *2) /1000;
		if(SkillSearch(301)) // ?
			sandanDelay += 0.3;
	}
}

 // CalcCast -----------------------------------------------
	variableCastTime = CalcVariableCast();
	fixedCastTime = CalcFixedCast();
	totalCastTime = variableCastTime * 0.8 + fixedCastTime * 0.2;

 // CastDelay -----------------------------------------------
	CalcDelay(); // globalCastDelay calculation

 // CalcRegeneration -----------------------------------------------
	n_A_HPR = calcHPReg(n_A_HPR=0);
	myInnerHtml("A_HPR",n_A_HPR,0);

	n_A_SPR = calcSPReg(n_A_SPR=0);
	myInnerHtml("A_SPR",n_A_SPR,0);

	calcRaceElementalReduction();

{ // Elemental Modifier ----------------------------------------
	n_A_zokusei = new Array();
	for(i=0;i<=9;i++)
	{
		n_A_zokusei[i] = element[n_A_BodyZokusei * 10 +1][i];
		n_A_zokusei[i] = n_A_zokusei[i] - Math.floor(n_A_zokusei[i] * n_tok[bon_RED_ELE_NEUTRAL+i]) / 100;
	}
}
{ // Inc Damage with Crit -------------------------------------
	n_tok[bon_DMG_CRIT] += n_tok[bon_CRIT_DMG_RC_FORMLESS+n_B[en_RACE]];
}

	// updated extended info
	PrepExtenededInfo();
	//CalcExtendedInfo();
}
}

function DamageCut(w_num,w_ch)
{ // floor some [w_num], ignore [w_ch]

	var w = n_tok[50+n_B[en_RACE]];
	if(w != 0)
		w_num -= Math.floor(w_num * w / 100);

	w = n_tok[60];
	if(w != 0)
		w_num -= Math.floor(w_num * w / 100);

	if(EquipNumSearch(957))
		w_num -= Math.floor(w_num * 30 / 100);

	w = n_tok[190+n_B[en_SIZE]];
	if(w != 0)
		w_num -= Math.floor(w_num * w / 100);

	w = n_tok[79];
	if(w != 0 && n_B[en_BOSS] == 0)
		w_num -= Math.floor(w_num * w / 100);

	w = n_tok[77];
	if(w != 0 && n_B[en_BOSS] == 1)
		w_num -= Math.floor(w_num * w / 100);

	w = n_tok[78];
	if(w != 0 && n_B[20])
		w_num -= Math.floor(w_num * w / 100);

	w = n_tok[330 + Math.floor(n_B[en_ELEMENT] / 10)];
	if(w != 0)
		w_num -= Math.floor(w_num * w / 100);

	return w_num;
}

function StPlusCalc()
{ // recalc non-base stats (bStat + %)
	n_A_JobSet();
	n_A_JobLV = document.forms["calcForm"].elements["A_JobLV"].value;

	var w2 = [0,0,0,0,0,0]; // bonus stats

	w2 = calcJobStats(n_A_JOB,n_A_JobLV,w2);

	var wSPC_STR = w2[bon_STR-1];
	var wSPC_AGI = w2[bon_AGI-1];
	var wSPC_VIT = w2[bon_VIT-1];
	var wSPC_INT = w2[bon_INT-1];
	var wSPC_DEX = w2[bon_DEX-1];
	var wSPC_LUK = w2[bon_LUK-1];


	wSPCall = StPlusCalc2(bon_ALL_STATS);
	wSPC_STR += StPlusCalc2(bon_STR) + wSPCall;
	wSPC_AGI += StPlusCalc2(bon_AGI) + wSPCall;
	wSPC_VIT += StPlusCalc2(bon_VIT) + wSPCall;
	wSPC_VIT += StPlusCalc2(bon_SET_VIT); // SetBoni
	wSPC_INT += StPlusCalc2(bon_INT) + wSPCall;
	wSPC_INT += StPlusCalc2(bon_SET_INT); // SetBoni
	wSPC_DEX += StPlusCalc2(bon_DEX) + wSPCall;
	wSPC_LUK += StPlusCalc2(bon_LUK) + wSPCall;

	if (!isNaN(parseInt(document.calcForm.E_BOOST_STR.value))) wSPC_STR += parseInt(document.calcForm.E_BOOST_STR.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_AGI.value))) wSPC_AGI += parseInt(document.calcForm.E_BOOST_AGI.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_INT.value))) wSPC_INT += parseInt(document.calcForm.E_BOOST_INT.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_VIT.value))) wSPC_VIT += parseInt(document.calcForm.E_BOOST_VIT.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_DEX.value))) wSPC_DEX += parseInt(document.calcForm.E_BOOST_DEX.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_LUK.value))) wSPC_LUK += parseInt(document.calcForm.E_BOOST_LUK.value);

	wSPC_DEX += SkillSearch(skill_AR_OWL_EYE);
	if(SkillSearch(skill_ME_CRAZY_UPROAR) || TimeItemNumSearch(17)) // ?
		wSPC_STR += 4;
	wSPC_STR += SkillSearch(skill_BS_HILT_BINDING);
	wSPC_STR += SkillSearch(skill_NIN_NINJA_AURA);
	wSPC_INT += SkillSearch(skill_NIN_NINJA_AURA);
	if(SkillSearch(skill_SA_DRAGONOLOGY))
		wSPC_INT += Math.round(SkillSearch(skill_SA_DRAGONOLOGY) /2);
	if(SkillSearch(skill_ST_STEALTH))
	{
		if(SkillSearch(skill_ST_STEALTH)==5) wSPC_STR +=16;
		if(SkillSearch(skill_ST_STEALTH)==4) wSPC_STR +=8;
		if(SkillSearch(skill_ST_STEALTH)==3) wSPC_STR +=4;
		if(SkillSearch(skill_ST_STEALTH)==2) wSPC_STR +=2;
		if(SkillSearch(skill_ST_STEALTH)==1) wSPC_STR +=1;
	}

	var w = SkillSearch(skill_AR_IMPROVE_CONCENTRATION);
	if(w)
	{
		w += 102;
		wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * w / 100) - n_A_DEX;
		wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * w / 100) - n_A_AGI;
	}
	else if(otherBuffs[ksImproveConcentration])
	{
		wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * (102 + otherBuffs[ksImproveConcentration]) / 100) - n_A_DEX;
		wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * (102 + otherBuffs[ksImproveConcentration]) / 100) - n_A_AGI;
	}
	else if(TimeItemNumSearch(31))
	{
		wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * 104 / 100) - n_A_DEX;
		wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * 104 / 100) - n_A_AGI;
	}
	else if(TimeItemNumSearch(4))
	{
		wSPC_DEX = Math.floor((n_A_DEX + wSPC_DEX) * 103 / 100) - n_A_DEX;
		wSPC_AGI = Math.floor((n_A_AGI + wSPC_AGI) * 103 / 100) - n_A_AGI;
	}

	wSPC_AGI += StPlusCalc2(bon_SET_AGI);
	wSPC_DEX += StPlusCalc2(bon_SET_DEX);
	if(n_A_JobSearch()==cls_TKK && EquipNumSearch(672))
		wSPC_AGI += 1;
	if(n_A_JobSearch()==cls_TKK && EquipNumSearch(673))
		wSPC_INT += 1;
	if(n_A_JobSearch()==cls_TKK && EquipNumSearch(675))
		wSPC_LUK += 2;
	if(n_A_JobSearch()==cls_TKK && EquipNumSearch(676))
		wSPC_DEX += 2;
	if(n_A_JobSearch()==cls_TKK && EquipNumSearch(678))
		wSPC_LUK += 1;
	if(n_A_SHOES_DEF_PLUS >= 9 && EquipNumSearch(717))
		wSPC_AGI += 2;
	if(n_A_HEAD_DEF_PLUS >= 5 && EquipNumSearch(1069))
		wSPC_LUK += (n_A_HEAD_DEF_PLUS - 4);
	if(n_A_Weapon_ATKplus >= 6 && EquipNumSearch(1168))
		wSPC_INT += (n_A_Weapon_ATKplus - 5);
	if(EquipNumSearch(1171) && SkillSearch(skill_SA_DRAGONOLOGY) == 5)
		wSPC_INT += 3;
	if ( EquipNumSearch( 1172 ) && n_A_Weapon_ATKplus > 0 )
	{ // Kronos
		var kronosMod = Math.floor(n_A_Weapon_ATKplus / 2);
		wSPC_INT += kronosMod;
	}

	if(n_A_Equip[eq_WEAPON]==1078 || n_A_Equip[eq_WEAPON]==1079)
		wSPC_INT += (n_A_Weapon_ATKplus -5);
	if(n_A_Weapon_ATKplus >= 9 && n_A_Equip[eq_WEAPON]==1078)
		wSPC_INT += 5;
	if(n_A_Weapon_ATKplus >= 10 && n_A_Equip[eq_WEAPON]==1079)
		wSPC_INT += 5;

	if(n_A_Equip[eq_WEAPONII]==1078 || n_A_Equip[eq_WEAPONII]==1079)
		wSPC_INT += (n_A_Weapon2_ATKplus -5);
	if(n_A_Weapon2_ATKplus >= 9 && n_A_Equip[eq_WEAPONII]==1078)
		wSPC_INT += 5;
	if(n_A_Weapon2_ATKplus >= 10 && n_A_Equip[eq_WEAPONII]==1079)
		wSPC_INT += 5;

	if(EquipNumSearch(649)) // BerserkGuitar ?
		wSPC_DEX -= SU_DEX;

	if ( EquipNumSearch( 1339 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Capricorn Diadem
		wSPC_INT += 2;
	}
	if ( EquipNumSearch( 1343 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Pisces Diadem
		wSPC_INT += 3;
	}
	if ( EquipNumSearch( 1345 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Scorpio Diadem
		wSPC_DEX += 1;
	}
	if ( EquipNumSearch( 1346 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Taurus Diadem
		wSPC_VIT += 2;
	}
	if ( EquipNumSearch( 1347 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Aquarius Crown
		wSPC_DEX += 1;
	}
	if ( EquipNumSearch( 1354 ) && n_A_HEAD_DEF_PLUS >= 8 )
	{ // Sagittarius Crown
		wSPC_AGI += 2;
		if ( EquipNumSearch( 1354 ) && n_A_HEAD_DEF_PLUS >= 10 )
		{ // Sagittarius Crown
			wSPC_AGI += 10;
			wSPC_DEX += 10;
		}
	}
	if ( EquipNumSearch( 1367 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Virgo Crown
		wSPC_DEX += 20;
	}
	if ( EquipNumSearch( 1370 ) )
	{ // Sigrun's Wings
		if ( n_A_JobSearch() == cls_SWO || n_A_JobSearch() == cls_THI || n_A_JobSearch() == cls_MER || n_A_JOB == cls_TKK )
		{ // Swordsman, Merchant, Thief, and Taekwon
			wSPC_STR += 1;
		}
		else if ( n_A_JobSearch() == cls_MAG || n_A_JobSearch() == cls_ACO || n_A_JOB == cls_NIN )
		{ // Mage, Acolyte, and Ninja
			wSPC_INT += 1;
		}
		else if ( n_A_JobSearch() == cls_ARC || n_A_JOB == cls_GUN )
		{ // Archer and Gunslinger
			wSPC_DEX += 1;
		}
	}
	if ( EquipNumSearch( 1401 ) && n_A_BaseLV >= 150 )
	{ // Ancient Gold Ornament
			wSPC_STR += 2;
			wSPC_DEX += 2;
			wSPC_INT += 2;
			wSPC_AGI += 2;
			wSPC_LUK += 2;
			wSPC_VIT += 2;
			if ( n_A_JobSearch() == cls_ARC) {
				wSPC_DEX += 3;
			}
	}
	if ( (EquipNumSearch( 1414 ) || EquipNumSearch( 1421 ) || EquipNumSearch( 1428 ) || EquipNumSearch( 1435 )) && SU_INT >= 120)
	{ // Aqua, Golden, Crimson, Forest Rod Sets
		wSPC_INT += 1;
	}
	if ( EquipNumSearch( 1454 ) )
	{ // Special Ninja Suit
		wSPC_AGI += Math.floor(n_A_BODY_DEF_PLUS / 3);
	}
	if ( EquipNumSearch( 1503 ) )
		wSPC_AGI += Math.min(n_A_SHOES_DEF_PLUS-7, 1);

	if(n_A_WeaponType==weapTyp_ROD)
		wSPC_INT += CardNumSearch(card_WEPN_NECROMANCER);

	wSPCall = StPlusCard(bon_ALL_STATS);
	wSPC_STR += StPlusCard(bon_STR) + wSPCall;
	wSPC_AGI += StPlusCard(bon_AGI) + wSPCall;
	wSPC_VIT += StPlusCard(bon_VIT) + wSPCall;
	wSPC_INT += StPlusCard(bon_INT) + wSPCall;
	wSPC_DEX += StPlusCard(bon_DEX) + wSPCall;
	wSPC_LUK += StPlusCard(bon_LUK) + wSPCall;


	if(n_A_JobSearch()==cls_ACO)
		wSPC_INT += CardNumSearch(card_HEAD_RIDEWORD);
	if(CardNumSearch(card_SHLD_DESPEROFTHAN)) wSPC_INT += n_A_LEFT_DEF_PLUS; //Despero of Thanatos
	if(CardNumSearch(card_GRMT_GREENMAIDEN)) wSPC_LUK += n_A_SHOULDER_DEF_PLUS; //Green Maiden
	if(CardNumSearch(card_FEET_ODIUMOFTHANA)) wSPC_AGI += n_A_SHOES_DEF_PLUS; //Odium of Thanatos
	if(CardNumSearch(card_BODY_DIMIK)) wSPC_VIT += n_A_BODY_DEF_PLUS; //Dimik
	if(n_A_card[card_loc_HEAD_UPPER] == 180) wSPC_STR += n_A_HEAD_DEF_PLUS;

	if(CardNumSearch(card_BODY_OBSIDIAN)) wSPC_VIT += Math.floor(SU_DEX /18); //Obsidian
	if(CardNumSearch(card_BODY_EGNIGEMCENIA)) wSPC_STR += Math.floor(SU_INT /18); //Egnigem
	if(CardNumSearch(card_BODY_VENATU)) wSPC_LUK += Math.floor(SU_AGI /18); //Venatu
	if(CardNumSearch(card_BODY_ANCIENTMIMIC)) wSPC_AGI += Math.floor(SU_LUK /18); //Ancient Mimic
	if(CardNumSearch(card_BODY_MISTRESSOFSH)) wSPC_INT += Math.floor(SU_STR /18); //Mistress of Shelter
	if(CardNumSearch(card_BODY_DAMEOFSENTIN)) wSPC_DEX += Math.floor(SU_VIT /18); //Dame of Sentinel

	if(CardNumSearch(card_GRMT_ALIOT)) //Aliot
	{ // Aliot
		if(n_A_JobSearch()==cls_SWO || n_A_JobSearch()==cls_THI || n_A_JobSearch()==cls_MER)
			wSPC_STR += 2;
		if(n_A_JobSearch()==cls_ACO || n_A_JobSearch()==cls_ARC || n_A_JobSearch()==cls_MAG)
			wSPC_INT += 2;
	}
	if (EquipNumSearch(1549)) {
		wSPC_DEX += Math.floor(n_A_SHOES_DEF_PLUS / 3)
	}

	var numMegs = EquipNumSearch( 348 );
	for ( var i = 0; i < numMegs; i++ )
	{ // Megingjard
		wSPC_STR += Math.floor( n_A_BaseLV / 5 );
	}

	// Acolyte Buffs
	wSPC_STR += acolyteBuffs[ksBlessing];
	wSPC_INT += acolyteBuffs[ksBlessing];
	wSPC_DEX += acolyteBuffs[ksBlessing];
	if ( acolyteBuffs[ksBlessing] > 0 )
	{
		wSPC_STR += acolyteBuffs[ksClementia];
		wSPC_INT += acolyteBuffs[ksClementia];
		wSPC_DEX += acolyteBuffs[ksClementia];
	}
	if ( acolyteBuffs[ksIncreaseAgi] > 0 && miscEffects[ksQuagmire] === 0 && miscEffects[ksAgiDown] === 0 )
	{
		if ( acolyteBuffs[ksIncreaseAgi] >= 5 )
		{
			wSPC_AGI += acolyteBuffs[ksIncreaseAgi] + 2;
			wSPC_AGI += acolyteBuffs[ksCandidus];
		}
		else
		{
			if ( usableItems[ksGuaranaCandy] )
			{
				// gives agi up level 5
				wSPC_AGI += 7;
			}
			else
			{
				wSPC_AGI += acolyteBuffs[ksIncreaseAgi] + 2;
				wSPC_AGI += acolyteBuffs[ksCandidus];
			}
		}
	}
	else if ( usableItems[ksGuaranaCandy]       &&
			  acolyteBuffs[ksIncreaseAgi] === 0 &&
			  miscEffects[ksQuagmire] === 0      &&
			  miscEffects[ksAgiDown] === 0 )
	{
		// gives agi up level 5
		wSPC_AGI += 7;
	}
	if ( acolyteBuffs[ksGloria] )
	{
		wSPC_LUK += 30;
	}
	if ( acolyteBuffs[ksLaudaAgnus] )
	{
		wSPC_VIT += 4 + acolyteBuffs[ksLaudaAgnus];
	}
	if ( acolyteBuffs[ksLaudaRamus] )
	{
		wSPC_LUK += 4 + acolyteBuffs[ksLaudaRamus];
	}

	if ( performerBuffs[ksChorus] === ksSinkingMelody &&
		 performerBuffs[ksChorusLevel] > 0 &&
		 performerBuffs[ksNumPerformers] >= 2 )
	{ // Sinking Melody
		wSPC_INT -= performerBuffs[ksChorusLevel] * performerBuffs[ksNumPerformers];
	}
	if ( performerBuffs[ksChorus] === ksWarcryFromBeyond &&
		 performerBuffs[ksChorusLevel] > 0 &&
		 performerBuffs[ksNumPerformers] >= 2 )
	{ // Warcry from Beyond
		wSPC_STR += performerBuffs[ksChorusLevel] * performerBuffs[ksNumPerformers];
	}

	// Sniper
	if ( SkillSearch( skill_SN_FALCON_EYES ) )
	{
		wSPC_STR += 5;
		wSPC_AGI += 5;
		wSPC_VIT += 5;
		wSPC_DEX += 5;
		wSPC_INT += 5;
		wSPC_LUK += 5;
	}
	if ( SkillSearch( skill_TK_SPRINT_STR_STATE ) && n_A_WeaponType === weapTyp_NONE )
	{
		wSPC_STR += 10;
	}
	if ( SkillSearch( skill_GS_INCREASE_ACCURACY ) )
	{
		wSPC_DEX += 4;
		wSPC_AGI += 4;
	}
	if ( SkillSearch( skill_RUN_GIANT_GROWTH ) )
	{
		wSPC_STR += 30;
	}
	if ( SkillSearch( skill_RAN_RESEARCH_TRAP ) )
	{
		wSPC_INT += SkillSearch( skill_RAN_RESEARCH_TRAP );
	}
	if ( SkillSearch( skill_ROY_INSPIRATION ) )
	{ // Inspiration stats increase by [(Caster s Base Level / 10 ) + (Caster s Job Level / 5 )]
		var statIncrease = Math.floor( ( n_A_BaseLV / 10 ) + ( n_A_JobLV / 5 ) );
		wSPC_STR += statIncrease;
		wSPC_AGI += statIncrease;
		wSPC_VIT += statIncrease;
		wSPC_DEX += statIncrease;
		wSPC_INT += statIncrease;
		wSPC_LUK += statIncrease;
	}

	// Guild Skills
	if ( guildBuffs[pass_IV_BAT_ORDER] )
	{
		wSPC_STR += 5;
		wSPC_DEX += 5;
		wSPC_INT += 5;
	}
	wSPC_STR += guildBuffs[pass_IV_GRE_LEADER];
	wSPC_VIT += guildBuffs[pass_IV_WOU_GLORY];
	wSPC_AGI += guildBuffs[pass_IV_SOU_COLD];
	wSPC_DEX += guildBuffs[pass_IV_SHA_EYES];

	// Battle Chant
	if ( battleChantBuffs[pass_V_STATS] )
	{
		wSPC_STR += 20;
		wSPC_AGI += 20;
		wSPC_VIT += 20;
		wSPC_DEX += 20;
		wSPC_INT += 20;
		wSPC_LUK += 20;
	}

	if ( otherBuffs[ksMurderBonus] == 1 )
	{
		wSPC_STR += 3;
		wSPC_AGI += 3;
		wSPC_VIT += 3;
		wSPC_DEX += 3;
		wSPC_INT += 3;
		wSPC_LUK += 3;
	}
	if ( otherBuffs[ksMurderBonus] == 2 )
	{
		wSPC_STR += 5;
		wSPC_AGI += 5;
		wSPC_VIT += 5;
		wSPC_DEX += 5;
		wSPC_INT += 5;
		wSPC_LUK += 5;
	}
	if ( miscEffects[ksSupNovMarriage] )
	{
		wSPC_STR += 1;
		wSPC_AGI += 1;
		wSPC_VIT += 1;
		wSPC_DEX += 1;
		wSPC_INT += 1;
		wSPC_LUK += 1;
	}
	if ( SkillSearch( skill_ALL_MARRIAGE_STATUS ) )
	{
		wSPC_STR -= 1;
		wSPC_AGI -= 1;
		wSPC_VIT -= 1;
		wSPC_DEX -= 1;
		wSPC_INT -= 1;
		wSPC_LUK -= 1;
	}

	// Stat-Food
	wSPC_STR += usableItems[ksStrengthFood];
	wSPC_AGI += usableItems[ksAgilityFood];
	wSPC_VIT += usableItems[ksVitalityFood];
	wSPC_INT += usableItems[ksIntelligenceFood];
	wSPC_DEX += usableItems[ksDexterityFood];
	wSPC_LUK += usableItems[ksLuckFood];
	if ( usableItems[ksLuckyRiceCake] )
	{
		wSPC_LUK += 21;
	}

	var wHSE = document.forms["calcForm"].elements["A_HSE"].value; // Hidden SocketEnch
	if(wHSE)
	{
		var w = wHSE % 10;
		if(1 <= wHSE && wHSE <= 9)
			wSPC_STR += w;
		if(11 <= wHSE && wHSE <= 19)
			wSPC_AGI += w;
		if(21 <= wHSE && wHSE <= 29)
			wSPC_VIT += w;
		if(31 <= wHSE && wHSE <= 39)
			wSPC_INT += w;
		if(41 <= wHSE && wHSE <= 49)
			wSPC_DEX += w;
		if(51 <= wHSE && wHSE <= 59)
			wSPC_LUK += w;
	}
	var wHSE2 = document.forms["calcForm"].elements["A_HSE_HEAD1"].value; // Hidden SocketEnch
	if(wHSE2)
	{
		var w = wHSE2 % 10;
		if(1 <= wHSE2 && wHSE2 <= 9)
			wSPC_STR += w;
		if(11 <= wHSE2 && wHSE2 <= 19)
			wSPC_AGI += w;
		if(21 <= wHSE2 && wHSE2 <= 29)
			wSPC_VIT += w;
		if(31 <= wHSE2 && wHSE2 <= 39)
			wSPC_INT += w;
		if(41 <= wHSE2 && wHSE2 <= 49)
			wSPC_DEX += w;
		if(51 <= wHSE2 && wHSE2 <= 59)
			wSPC_LUK += w;
	}
	if(Math.floor(wHSE / 10) == Math.floor(wHSE2 / 10))
	{ // no double Enchant
		var w1 = wHSE % 10;
		var w2 = wHSE2 % 10;
		if(w1 > w2)
			w1 = w2;
		if(1 <= wHSE && wHSE <= 9)
			wSPC_STR -= w1;
		if(11 <= wHSE && wHSE <= 19)
			wSPC_AGI -= w1;
		if(21 <= wHSE && wHSE <= 29)
			wSPC_VIT -= w1;
		if(31 <= wHSE && wHSE <= 39)
			wSPC_INT -= w1;
		if(41 <= wHSE && wHSE <= 49)
			wSPC_DEX -= w1;
		if(51 <= wHSE && wHSE <= 59)
			wSPC_LUK -= w1;
	}

	if(battleEffects[13])
	{ // not used ? - old socket enchant
		var w = ((battleEffects[13] - 1) % 3) + 1;
		if(1 <= battleEffects[13] && battleEffects[13] <= 3)
			wSPC_STR += w;
		if(4 <= battleEffects[13] && battleEffects[13] <= 6)
			wSPC_AGI += w;
		if(7 <= battleEffects[13] && battleEffects[13] <= 9)
			wSPC_VIT += w;
		if(10 <= battleEffects[13] && battleEffects[13] <= 12)
			wSPC_INT += w;
		if(13 <= battleEffects[13] && battleEffects[13] <= 15)
			wSPC_DEX += w;
		if(16 <= battleEffects[13] && battleEffects[13] <= 18)
			wSPC_LUK += w;
	}

	if(miscEffects[ksTransFirstSpirit])
	{
		 if(cls_HSWO <= n_A_JOB && n_A_JOB <= cls_HMER && n_A_BaseLV < 70)
		 {
			if(n_A_STR + wSPC_STR <= 50)
					wSPC_STR = 50 - n_A_STR;
			if(n_A_AGI + wSPC_AGI <= 50)
					wSPC_AGI = 50 - n_A_AGI;
			if(n_A_VIT + wSPC_VIT <= 50)
					wSPC_VIT = 50 - n_A_VIT;
			if(n_A_INT + wSPC_INT <= 50)
					wSPC_INT = 50 - n_A_INT;
			if(n_A_DEX + wSPC_DEX <= 50)
					wSPC_DEX = 50 - n_A_DEX;
			if(n_A_LUK + wSPC_LUK <= 50)
					wSPC_LUK = 50 - n_A_LUK;
		}
	}

	// Calculate Marionette Controll
	if ( performerBuffs[ksMarionette] )
	{
		if ( n_A_STR + wSPC_STR < 99 )
		{
			if ( n_A_STR + wSPC_STR + Math.floor( performerBuffs[ksPerformerStr] / 2 ) < 99 )
			{
				wSPC_STR += Math.floor( performerBuffs[ksPerformerStr] / 2 );
			}
			else
			{
				wSPC_STR = ( 99 - n_A_STR );
			}
		}
		if ( n_A_AGI + wSPC_AGI < 99 )
		{
			if ( n_A_AGI + wSPC_AGI + Math.floor( performerBuffs[ksPerformerAgi] / 2 ) < 99 )
			{
				wSPC_AGI += Math.floor( performerBuffs[ksPerformerAgi] / 2 );
			}
			else
			{
				wSPC_AGI = ( 99 - n_A_AGI );
			}
		}
		if ( n_A_VIT + wSPC_VIT < 99 )
		{
			if ( n_A_VIT + wSPC_VIT + Math.floor( performerBuffs[ksPerformerVit] / 2 ) < 99 )
			{
				wSPC_VIT += Math.floor( performerBuffs[ksPerformerVit] / 2 );
			}
			else
			{
				wSPC_VIT = ( 99 - n_A_VIT );
			}
		}
		if ( n_A_INT + wSPC_INT < 99 )
		{
			if ( n_A_INT + wSPC_INT + Math.floor( performerBuffs[ksPerformerInt] / 2 ) < 99 )
			{
				wSPC_INT += Math.floor( performerBuffs[ksPerformerInt] / 2 );
			}
			else
			{
				wSPC_INT = ( 99 - n_A_INT );
			}
		}
		if ( n_A_DEX + wSPC_DEX < 99 )
		{
			if ( n_A_DEX + wSPC_DEX + Math.floor( performerBuffs[ksPerformerDex] / 2 ) < 99 )
			{
				wSPC_DEX += Math.floor( performerBuffs[ksPerformerDex] / 2 );
			}
			else
			{
				wSPC_DEX = ( 99 - n_A_DEX );
			}
		}
		if ( n_A_LUK + wSPC_LUK < 99 )
		{
			if ( n_A_LUK + wSPC_LUK + Math.floor( performerBuffs[ksPerformerLuk] / 2 ) < 99 )
			{
				wSPC_LUK += Math.floor( performerBuffs[ksPerformerLuk] / 2 );
			}
			else
			{
				wSPC_LUK = ( 99 - n_A_LUK );
			}
		}
	}

	//CUSTOM (1st Transcendent Spirit)
	if(SkillSearch(392) && (rebirthClass == 1) && (n_A_BaseLV > 10) && (n_A_BaseLV < 70))
	{
		var linkboni = n_A_BaseLV - 10;
		if(wSPC_STR > 50);
		else if((wSPC_STR + linkboni) > 50)
			wSPC_STR = 50;
		else
			wSPC_STR += linkboni;
		if(wSPC_AGI > 50);
		else if((wSPC_AGI + linkboni) > 50)
			wSPC_AGI = 50;
		else
			wSPC_AGI += linkboni;
		if(wSPC_VIT > 50);
		else if((wSPC_VIT + linkboni) > 50)
			wSPC_VIT = 50;
		else
			wSPC_VIT += linkboni;
		if(wSPC_INT > 50);
		else if((wSPC_INT + linkboni) > 50)
			wSPC_INT = 50;
		else
			wSPC_INT += linkboni;
		if(wSPC_DEX > 50);
		else if((wSPC_DEX + linkboni) > 50)
			wSPC_DEX = 50;
		else
			wSPC_DEX += linkboni;
		if(wSPC_LUK > 50);
		else if((wSPC_LUK + linkboni) > 50)
			wSPC_LUK = 50;
		else
			wSPC_LUK += linkboni;
	}
	//END CUSTOM

	if ( miscEffects[ksQuagmire] )
	{
		var w1 = Math.floor((n_A_AGI + wSPC_AGI) / 2);
		var w2;
		if(PlayerVersusPlayer)
			w2 = 5 * miscEffects[ksQuagmire];
		else
			w2 = 10 * miscEffects[ksQuagmire];
		if(w1 > w2)
			wSPC_AGI -= w2;
		else
			wSPC_AGI -= w1;
		w1 = Math.floor((n_A_DEX + wSPC_DEX) / 2);
		if(w1 > w2)
			wSPC_DEX -= w2;
		else
			wSPC_DEX -= w1;
	}
	if(miscEffects[ksAgiDown])
		wSPC_AGI -= (miscEffects[ksAgiDown] + 2);
	if(miscEffects[ksCursed])
		wSPC_LUK = -1 * n_A_LUK;

	// Harmonize
	if ( performerBuffs[ksMaestroSolo] === ksHarmonize && performerBuffs[ksMaestroSoloLevel] > 0 )
	{
		var reduction = 5 + performerBuffs[ksMaestroSoloLevel] * 5;

		wSPC_STR -= reduction;
		wSPC_AGI -= reduction;
		wSPC_VIT -= reduction;
		wSPC_INT -= reduction;
		wSPC_DEX -= reduction;
		wSPC_LUK -= reduction;
	}

	n_A_STR += wSPC_STR;
	n_A_AGI += wSPC_AGI;
	n_A_VIT += wSPC_VIT;
	n_A_INT += wSPC_INT;
	n_A_DEX += wSPC_DEX;
	n_A_LUK += wSPC_LUK;

	// Full Throttle
	if (SkillSearch(skill_3RD_FULL_THROTTLE)) {
	    wSPC_STR += Math.floor(n_A_STR * 0.2);
	    wSPC_AGI += Math.floor(n_A_AGI * 0.2);
	    wSPC_VIT += Math.floor(n_A_VIT * 0.2);
	    wSPC_INT += Math.floor(n_A_INT * 0.2);
	    wSPC_DEX += Math.floor(n_A_DEX * 0.2);
	    wSPC_LUK += Math.floor(n_A_LUK * 0.2);
	}

	// Display Stats
	if(wSPC_STR >= 0)
		myInnerHtml("A_STRp","+"+wSPC_STR + " (" + StCalc2(SU_STR+1) + ")",0);
	else
		myInnerHtml("A_STRp",wSPC_STR + " (" + StCalc2(SU_STR+1) + ")",0);
	if(wSPC_AGI >= 0)
		myInnerHtml("A_AGIp","+"+wSPC_AGI + " (" + StCalc2(SU_AGI+1) + ")",0);
	else
		myInnerHtml("A_AGIp",wSPC_AGI + " (" + StCalc2(SU_AGI+1) + ")",0);
	if(wSPC_VIT >= 0)
		myInnerHtml("A_VITp","+"+wSPC_VIT + " (" + StCalc2(SU_VIT+1) + ")",0);
	else
		myInnerHtml("A_VITp",wSPC_VIT + " (" + StCalc2(SU_VIT+1) + ")",0);
	if(wSPC_INT >= 0)
		myInnerHtml("A_INTp","+"+wSPC_INT + " (" + StCalc2(SU_INT+1) + ")",0);
	else
		myInnerHtml("A_INTp",wSPC_INT + " (" + StCalc2(SU_INT+1) + ")",0);
	if(wSPC_DEX >= 0)
		myInnerHtml("A_DEXp","+"+wSPC_DEX + " (" + StCalc2(SU_DEX+1) + ")",0);
	else
		myInnerHtml("A_DEXp",wSPC_DEX + " (" + StCalc2(SU_DEX+1) + ")",0);
	if(wSPC_LUK >= 0)
		myInnerHtml("A_LUKp","+"+wSPC_LUK + " (" + StCalc2(SU_LUK+1) + ")",0);
	else
		myInnerHtml("A_LUKp",wSPC_LUK + " (" + StCalc2(SU_LUK+1) + ")",0);
}

function StPlusCalc2( nSTP2 )
{ // Additional [Stats] by equip
	var w = bon_NONE;
	for ( var i = 0; i <= 20; i++ )
	{	 // for each equip/ card
		for ( var j = 0; ItemOBJ[n_A_Equip[i]][j + itm_BONUS_START] != bon_NONE; j += 2 )
		{
			if ( nSTP2 == ItemOBJ[n_A_Equip[i]][j + itm_BONUS_START] )
			{
				w += ItemOBJ[n_A_Equip[i]][j + itm_BONUS_START + 1];
			}
		}
	}

	return w;
}

function LoadStatsFromScripts()
{ // Additional [Stats] by equip
	for ( var i = 0; i <= 20; i++ )
	{	 // for each equip/ card
		if (ItemOBJ[n_A_Equip[i]][itm_BONUS_START] === bon_SCRIPT) {
			var effects = parseItemEffects(ItemOBJ[n_A_Equip[i]][itm_BONUS_START + 1], { type: ItemOBJ[n_A_Equip[i]][itm_TYPE] });
			for (var key in effects) {
				if (key > 450) {
					n_tok[key] = effects[key];
				} else {
					n_tok[key] += effects[key];
				}
			}
		}
	}
}


function StPlusCard( nSTP2 )
{ // Additional [Stats] by cards
	var w=0;
	for(var i=0;i<=25;i++)
	{
		for(var j=0;cardOBJ[n_A_card[i]][j +4] != 0;j += 2)
		{
			if(nSTP2 == cardOBJ[n_A_card[i]][j +4])
				w += cardOBJ[n_A_card[i]][j +5];
		}
	}

	for(var j=0;PET_OBJ[miscEffects[ksPetEffects]][j +3] != 0;j += 2)
	{
		if(nSTP2 == PET_OBJ[miscEffects[ksPetEffects]][j +3])
			w += PET_OBJ[miscEffects[ksPetEffects]][j +4];
	}

	var w_num = [0,0,0,0];
	for(i=0;i<=3;i++)
		w_num[i] = miscEffects[ksFirstTempEffect + i];
	for(i=0;i<=2;i++)
		for(j=i+1;j<=3;j++)
			if(w_num[i] == w_num[j])
				w_num[j] = 0;
	for(i=0;i<=3;i++)
	{
		for(var j=0;ITEM_SP_TIME_OBJ[w_num[i]][5 + j] != 0;j += 2)
		{
			if(nSTP2 == ITEM_SP_TIME_OBJ[w_num[i]][5 + j])
				w += ITEM_SP_TIME_OBJ[w_num[i]][6 + j];
		}
	}

	return w;
}

function StPlusWeapon(nSTP2)
{ // Additional [Stats]/ MAtk by Weapon
	var w=bon_NONE;
	for(var j=0;ItemOBJ[n_A_Equip[eq_WEAPON]][j +itm_BONUS_START] != bon_NONE;j += 2)
	{
		if(nSTP2 == ItemOBJ[n_A_Equip[eq_WEAPON]][j +itm_BONUS_START])
			w += ItemOBJ[n_A_Equip[eq_WEAPON]][j +itm_BONUS_START+1];
	}
	return w;
}

function WeaponSet()
{ // generate WeaponList
	n_A_JobSet();
	n_A_WeaponType = document.forms["calcForm"].elements["A_WeaponType"].value;
	var len = document.calcForm.A_weapon1.length;
	for(var i=0;i<len;i++)
		document.calcForm.A_weapon1.options[0] = null;

	work = new Array();
	j = 0;
	for (i=0;i<=ItemMax; i++)
	{
		if(ItemOBJ[i][1] == n_A_WeaponType && JobEquipItemSearch(ItemOBJ[i][2]) == 1)
		{
			work[j] = i;
			j++;
		}
		else if(ItemOBJ[i][4] == 4 && ItemOBJ[i][1] == n_A_WeaponType && SuperNoviceFullWeaponCHECK)
		{
			work[j] = i;
			j++;
		}
	}
	work[j] = "EOF";

	work = sort(work);
	for (i=0;i<j; i++)
		document.calcForm.A_weapon1.options[i] = new Option(ITEM_NAME[work[i]][1+ Language*2],ItemOBJ[work[i]][0]);
}

function WeaponSetLeft()
{ // generate left weaponlist
	n_A_JobSet();
	n_A_Weapon2Type = document.forms["calcForm"].elements["A_Weapon2Type"].value;
	var len = document.forms["calcForm"].elements["A_weapon2"].length;
	for(var i=0;i<len;i++)
		document.forms["calcForm"].elements["A_weapon2"].options[0] = null;
	work = new Array();
	j = 0;
	for (i=0;i<=ItemMax; i++)
	{
		if(ItemOBJ[i][1] == n_A_Weapon2Type && JobEquipItemSearch(ItemOBJ[i][2]) == 1)
		{
			work[j] = i;
			j++;
		}
	}
	work[j] = "EOF";
	work = sort(work);
	for (i=0;i<j; i++)
		document.forms["calcForm"].elements["A_weapon2"].options[i] = new Option(ITEM_NAME[work[i]][1+ Language *2],ItemOBJ[work[i]][0]);
}

function WeaponSet2()
{ // generate EquipList
with(document.calcForm)
{
	n_A_JobSet();

	// reset equips ------------------
	var len = A_head1.length;
	for(var i=0;i<len;i++)
		A_head1.options[0] = null;
	var len = A_head2.length;
	for(i=0;i<len;i++)
		A_head2.options[0] = null;
	var len = A_head3.length;
	for(i=0;i<len;i++)
		A_head3.options[0] = null;
	var len = A_left.length;
	for(i=0;i<len;i++)
		A_left.options[0] = null;
	var len = A_body.length;
	for(i=0;i<len;i++)
		A_body.options[0] = null;
	var len = A_shoulder.length;
	for(i=0;i<len;i++)
		A_shoulder.options[0] = null;
	var len = A_shoes.length;
	for(i=0;i<len;i++)
		A_shoes.options[0] = null;
	var len = A_acces1.length;
	for(i=0;i<len;i++){
		A_acces1.options[0] = null;
		A_acces2.options[0] = null;
	}
	// insert first time ? -----------
	var nx = 1 + Language*2;
	if(first_check == 0)
	{ // insert "empty" labels
		first_check = 1;
		A_head1.options[0] = new Option(ITEM_NAME[142][nx],ItemOBJ[142][0]);
		A_head2.options[0] = new Option(ITEM_NAME[243][nx],ItemOBJ[243][0]);
		A_head3.options[0] = new Option(ITEM_NAME[268][nx],ItemOBJ[268][0]);
		A_left.options[0] = new Option(ITEM_NAME[305][nx],ItemOBJ[305][0]);
		A_body.options[0] = new Option(ITEM_NAME[279][nx],ItemOBJ[279][0]);
		A_shoulder.options[0] = new Option(ITEM_NAME[311][nx],ItemOBJ[311][0]);
		A_shoes.options[0] = new Option(ITEM_NAME[317][nx],ItemOBJ[317][0]);
		A_acces1.options[0] = new Option(ITEM_NAME[326][nx],ItemOBJ[326][0]);
		A_acces2.options[0] = new Option(ITEM_NAME[326][nx],ItemOBJ[326][0]);
		return;
	}
	first_check = 2;

	var workB = new Array(); // itemlist[equiptyp][accu number]
	for(i=0;i<=7;i++)
		workB[i] = new Array();
	var wsj = new Array(); // amount of items
	for(i=0;i<=7;i++)
		wsj[i]=0;
	for(i=0;i<=ItemMax; i++){ // check all items for type & jobReq

		if(ItemOBJ[i][itm_TYPE] == itm_type_HEAD_UPPER && (JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1 || SuperNoviceFullWeaponCHECK)){
				workB[0][wsj[0]] = i;
				wsj[0]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_HEAD_MIDDLE && (JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1 || SuperNoviceFullWeaponCHECK)){
				workB[1][wsj[1]] = i;
				wsj[1]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_HEAD_LOWER && (JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1 || SuperNoviceFullWeaponCHECK)){
				workB[2][wsj[2]] = i;
				wsj[2]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_SHIELD && JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1){
				workB[3][wsj[3]] = i;
				wsj[3]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_ARMOR && JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1){
				workB[4][wsj[4]] = i;
				wsj[4]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_GARMENT && JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1){
				workB[5][wsj[5]] = i;
				wsj[5]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_SHOES && JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1){
				workB[6][wsj[6]] = i;
				wsj[6]++;

		}else if(ItemOBJ[i][itm_TYPE] == itm_type_ACCESSORY && JobEquipItemSearch(ItemOBJ[i][itm_REQ_JOB]) == 1){
				workB[7][wsj[7]] = i;
				wsj[7]++;
		}

	}

	for(i=0;i<=7;i++)
		workB[i][wsj[i]] = "EOF";

	for(var m=0;m<=7;m++) // sort
		workB[m] = sort(workB[m]);

	var z = 0;
	for(i=0;i<wsj[0];i++){ // Upper Head - fill dropdown
		z = workB[0][i];
		if(z < ITEM_NAME.length)
			A_head1.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_head1.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[1];i++){ // middle head - fill dropdown
		z = workB[1][i];
		if(z < ITEM_NAME.length)
			A_head2.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_head2.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[2];i++){ // lower head - fill dropdown
		z = workB[2][i];
		if(z < ITEM_NAME.length)
			A_head3.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_head3.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[3];i++){ // shield - fill dropdown
		z = workB[3][i];
		if(z < ITEM_NAME.length)
			A_left.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_left.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[4];i++){ // armor - fill dropdown
		z = workB[4][i];
		if(z < ITEM_NAME.length)
			A_body.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_body.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[5];i++){ // garment - fill dropdown
		z = workB[5][i];
		if(z < ITEM_NAME.length)
			A_shoulder.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_shoulder.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[6];i++){ // shoes - fill dropdown
		z = workB[6][i];
		if(z < ITEM_NAME.length)
			A_shoes.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		else
			A_shoes.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
	}
	for(i=0;i<wsj[7];i++){ // accessory - fill dropdown
		z = workB[7][i];
		if(z < ITEM_NAME.length){
			A_acces1.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
			A_acces2.options[i] = new Option(ITEM_NAME[z][nx],ItemOBJ[z][0]);
		}else{
			A_acces1.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
			A_acces2.options[i] = new Option("[missing name]",ItemOBJ[z][0]);
		}
	}
}}

function FirstNovis()
{
	if ( first_check == 1 )
	{
		first_check = 2;
		WeaponSet2();
	}
}

{ // JobEquipmItemOBJ[n_A_JOB][i] != ReqJob --> allowed
//   0 = all
//   1 = all axc novi
//  50 = novis
//  51 = Swo Cls
//  52 = Thi Cls
//  53 = Aco Cls
//  54 = Arc Cls
//  55 = Mag Cls & SL
//  56 = Mer Cls
//  58 = Nin
//  59 = GS
//  61~66 = Swo~Arc 2nd Cls
//  65 = Wiz & Sage Cls & SLi
//  66 = Mer 2nd Cls
//  70 = Swo & Merc Cls
//  71 = Swo & Aco & Mage & Merc Cls & SLi
//  72 = Swo & Thi & Merc Cls
//  73 = Aco & Merc
//  74 = Swo & Thi & Aco & Merc Cls & Ba/Da
//  75 = Swo & Thi & Arc & Merc Cls
//  76 = Arc & Rog Cls
//  77 = Aco & Mage Cls & SLi
//  78 = Swo & Thi & Aco & Merc Cls
//  79 = 2nd Cls
//  80 = Thi & Rog Cls & Hunter Cls
//  81 = Ass & Prie Cls
//  82 = 2nd trans Cls
//  83 = Swo & Thi & Arc & Merc Cls & TKK & TKM & GS
//  84 = Swo & Thi & Merc Cls & TKK & TKM
//  85 = Swo & Thi & Aco & Merc Cls & TKK & TKM
//  86 = Swo & Merc Cls & TKK & TKM
//  87 = Swo Cls & TKM
//  88 = Hun & Rog Cls
//  89 = Aco & Arc & Mag Cls & SLi
//  90 = Nov & Swo & Mer & Ass Cls & Thi
//  91 = Swo & Thi & Merc Cls & TKM & SLi
// 100 = Novi & HNovi
// 101 = Swo & HSwo
// 102 = Thi & HThi
// 103 = Aco & HAco
// 104 = Arc & HArc
// 105 = Mag & HMag
// 106 = Mer & HMer
// 107 = Kni Cls
// 108 = Ass Cls
// 109 = Pri Cls
// 110 = Hun Cls
// 111 = Wiz Cls
// 112 = Bla Cls
// 113 = Cru Cls
// 114 = Rog Cls
// 115 = Mon Cls
// 116 = Bar Cls
// 117 = Dan Cls
// 118 = Sag Cls
// 119 = Alc Cls
// 120 = SNov
// 121 = LK Cls
// 122 = AX Cls
// 123 = HP Cls
// 124 = Sni Cls
// 125 = HW Cls
// 126 = BS Cls
// 127 = PA Cls
// 128 = Sta Cls
// 129 = Cha Cls
// 130 = Clo Cls
// 131 = Gyp Cls
// 132 = Pro Cls
// 133 = Bio Cls
// 141 = TKK
// 142 = TKM
// 143 = SLi
// 144 = Nin
// 145 = GS (also 58)
// 146 = Aco & Mag & SLi & Nov
// 147 = Swo & Thi & Arc & Merc Cls & TKK & TKM & Nin
// >1000 -> Rebi & Sub
// >2000 -> 3rd  & Sub

JobEquipItemOBJ = [
/* Nov */[0,    50, 90,100, 146, 999],
/* Swo */[0, 1, 51,101, 70, 71, 72, 74, 75, 78, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* Thi */[0, 1, 52,102, 72, 74, 75, 78, 80, 83, 84, 85, 90, 91, 147, 999],
/* Aco */[0, 1, 53,103, 71, 73, 74, 77, 78, 85, 89, 146, 999],
/* Arc */[0, 1, 54,104, 75, 76, 83, 89, 147, 999],
/* Mag */[0, 1, 55,105, 71, 77, 89, 146, 999],
/* Mer */[0, 1, 56,106, 70, 71, 72, 73, 74, 75, 78, 83, 84, 85, 86, 90, 91, 147, 999],

/* Kni */[0, 1, 51, 61,107, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* Ass */[0, 1, 52, 62,108, 72, 74, 75, 78, 79, 81, 83, 84, 85, 90, 91, 147, 999],
/* Pri */[0, 1, 53, 63,109, 71, 73, 74, 77, 78, 79, 81, 85, 89, 146, 999],
/* Hun */[0, 1, 54, 64,110, 75, 76, 79, 80, 83, 88, 89, 147, 999],
/* Wiz */[0, 1, 55, 65,111, 71, 77, 79, 89, 146, 999],
/* Bla */[0, 1, 56, 66,112, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 90, 91, 147, 999],

/* Cru */[0, 1, 51, 61,113, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* Rog */[0, 1, 52, 62,114, 72, 74, 75, 76, 78, 79, 80, 83, 84, 85, 88, 91, 147, 999],
/* Mon */[0, 1, 53, 63,115, 71, 73, 74, 77, 78, 79, 85, 89, 146, 999],
/* Bar */[0, 1, 54, 64,116, 74, 75, 76, 79, 83, 89, 147, 999],
/* Dan */[0, 1, 54, 64,117, 74, 75, 76, 79, 83, 89, 147, 999],
/* Sag */[0, 1, 55, 65,118, 71, 77, 79, 89, 146, 999],
/* Alc */[0, 1, 56, 66,119, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 90, 91, 147, 999],

/* Sno */[0,    50, 90,120,999],
/* Lor */[0, 1, 51, 61,107,121, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* ASX */[0, 1, 52, 62,108,122, 72, 74, 75, 78, 79, 81, 82, 83, 84, 85, 90, 91, 147, 999],
/* HPr */[0, 1, 53, 63,109,123, 71, 73, 74, 77, 78, 79, 81, 82, 85, 89, 146, 999],
/* Sni */[0, 1, 54, 64,110,124, 75, 76, 79, 80, 82, 83, 88, 89, 147, 999],
/* HWi */[0, 1, 55, 65,111,125, 71, 77, 79, 82, 89, 146, 999],
/* Mas */[0, 1, 56, 66,112,126, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91, 147, 999],

/* Pal */[0, 1, 51, 61,113,127, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* Sta */[0, 1, 52, 62,114,128, 72, 74, 75, 76, 78, 79, 80, 82, 83, 84, 85, 88, 91, 147, 999],
/* Cha */[0, 1, 53, 63,115,129, 71, 73, 74, 77, 78, 79, 82, 85, 89, 146, 999],
/* Clo */[0, 1, 54, 64,116,130, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Gyp */[0, 1, 54, 64,117,131, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Sch */[0, 1, 55, 65,118,132, 71, 77, 79, 82, 89, 146, 999],
/* Bio */[0, 1, 56, 66,119,133, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91, 147, 999],

/* HNo */[0,    50, 90,100, 146, 999],
/* HSw */[0, 1, 51,101, 70, 71, 72, 74, 75, 78, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* HTh */[0, 1, 52,102, 72, 74, 75, 78, 80, 83, 84, 85, 90, 91, 147, 999],
/* HAc */[0, 1, 53,103, 71, 73, 74, 77, 78, 85, 89, 146, 999],
/* HAr */[0, 1, 54,104, 75, 76, 83, 89, 147, 999],
/* HMa */[0, 1, 55,105, 71, 77, 89, 146, 999],
/* HMe */[0, 1, 56,106, 70, 71, 72, 73, 74, 75, 78, 83, 84, 85, 86, 90, 91, 147, 999],

/* TKK */[0, 1,141, 83, 84, 85, 86, 147, 999],
/* TKM */[0, 1,142, 79, 83, 84, 85, 86, 87, 91, 93, 147, 999],
/* SLi */[0, 1,143, 55, 65,111, 71, 77, 79, 89, 146, 999],
/* Nin */[0, 1,144, 58, 52, 91, 147, 999],
/* GSl */[0, 1,145, 59, 83,999],

/* Run */[0, 1, 51, 61,107,121, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* Run */[0, 1, 51, 61,107,121, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* GlX */[0, 1, 52, 62,108,122, 72, 74, 75, 78, 79, 81, 82, 83, 84, 85, 90, 91, 147, 999],
/* GlX */[0, 1, 52, 62,108,122, 72, 74, 75, 78, 79, 81, 82, 83, 84, 85, 90, 91, 147, 999],
/* ABi */[0, 1, 53, 63,109,123, 71, 73, 74, 77, 78, 79, 81, 82, 85, 89, 92, 94, 146, 999], //ADDED 94
/* ABi */[0, 1, 53, 63,109,123, 71, 73, 74, 77, 78, 79, 81, 82, 85, 89, 92, 94, 146, 999], //ADDED 94
/* Ran */[0, 1, 54, 64,110,124, 75, 76, 79, 80, 82, 83, 88, 89, 147, 999],
/* Ran */[0, 1, 54, 64,110,124, 75, 76, 79, 80, 82, 83, 88, 89, 147, 999],
/* WLo */[0, 1, 55, 65,111,125, 71, 77, 79, 82, 89, 94, 146, 999], //ADDED 94
/* WLo */[0, 1, 55, 65,111,125, 71, 77, 79, 82, 89, 94, 146, 999], //ADDED 94
/* Mec */[0, 1, 56, 66,112,126, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91,92, 147, 999],
/* Mec */[0, 1, 56, 66,112,126, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91,92, 147, 999],

/* RGu */[0, 1, 51, 61,113,127, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* RGu */[0, 1, 51, 61,113,127, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 93, 147, 999],
/* SCh */[0, 1, 52, 62,114,128, 72, 74, 75, 76, 78, 79, 80, 82, 83, 84, 85, 88, 91, 147, 999],
/* SCh */[0, 1, 52, 62,114,128, 72, 74, 75, 76, 78, 79, 80, 82, 83, 84, 85, 88, 91, 147, 999],
/* Sur */[0, 1, 53, 63,115,129, 71, 73, 74, 77, 78, 79, 82, 85, 89, 92, 146, 999],
/* Sur */[0, 1, 53, 63,115,129, 71, 73, 74, 77, 78, 79, 82, 85, 89, 92, 146, 999],
/* Min */[0, 1, 54, 64,116,130, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Min */[0, 1, 54, 64,116,130, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Wan */[0, 1, 54, 64,117,131, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Wan */[0, 1, 54, 64,117,131, 74, 75, 76, 79, 82, 83, 89, 147, 999],
/* Sor */[0, 1, 55, 65,118,132, 71, 77, 79, 82, 89, 146, 999],
/* Sor */[0, 1, 55, 65,118,132, 71, 77, 79, 82, 89, 146, 999],
/* Gen */[0, 1, 56, 66,119,133, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91,92, 147, 999],
/* Gen */[0, 1, 56, 66,119,133, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91,92, 147, 999],
/* Obo */[0, 1,144, 58, 52, 91,999],
/* Eno */[0,    50, 90,120,999],
];
}

function JobEquipItemSearch( nJEIS )
{ // [Equip] matches current job ?

	if ( nJEIS >= 2000 )
	{
		if ( n_A_JOB == cls_KAGOB ) {
			return 0;
		}
		if ( thirdClass ) // 3rd cls
			nJEIS -= 2000;
		else
			return 0;
	}
	if ( nJEIS >= 1000 )
	{
		if ( n_A_JOB == cls_KAGOB )
			return 0;
		if(rebirthClass || thirdClass) // rebirth or 3rd cls
			nJEIS -= 1000;
		else
			return 0;
	}
	for ( var j = 0; JobEquipItemOBJ[n_A_JOB][j] != 999; j++ )
	{
		if ( JobEquipItemOBJ[n_A_JOB][j] == nJEIS )
		{
			return 1;
		}
	}
	return 0;
}

function EquipNumSearch( nENS )
{ // Search equipCount
	var wENS=0;
	for(var ENSi=0;ENSi<=20;ENSi++)
	{
		if(nENS == n_A_Equip[ENSi])
			wENS += 1;
	}
	return wENS;
}

function CardNumSearch( nCNS )
{ // Search cardCount
	var wCNS=0;
	for(var CNSi=0;CNSi<=25;CNSi++)
	{
		if(nCNS == n_A_card[CNSi])
			wCNS += 1;
	}
	return wCNS;
}

function TimeItemNumSearch( index )
{ // TemporaryEffects
	var count = 0;
	for ( var i = 0; i <= 3; i++ )
	{
		if ( index === miscEffects[ksFirstTempEffect + i] )
		{
			count += 1;
		}
	}

	return count;
}

function ActiveSkillSetPlus()
{ // generate skillList ?
	w_ASSP0=new Array();
	w_ASSP9=new Array();
	for ( var i = 0; i <= 20; i++ )
	{
		w_ASSP0[i]=999;
		w_ASSP9[i]=0;
	}

	j=0;
	for ( var i = 0; i <= 20; i++ )
	{
		for(j2=0;ItemOBJ[n_A_Equip[i]][8+j2] != 0;j2 += 2)
		{
			if(ItemOBJ[n_A_Equip[i]][8+j2] == 220)
			{
				if(InsertSkill[ItemOBJ[n_A_Equip[i]][9+j2]][1] == 1)
				{
					w_ASSP0[j] = InsertSkill[ItemOBJ[n_A_Equip[i]][9+j2]][2];
					w_ASSP9[j] = InsertSkill[ItemOBJ[n_A_Equip[i]][9+j2]][0] + 3000;
					j++;
				}
			}
			else if(ItemOBJ[n_A_Equip[i]][8+j2] == 221)
			{
				if(AutoSpellSkill[ItemOBJ[n_A_Equip[i]][9+j2]][1] == 1)
				{
					w_ASSP0[j] = AutoSpellSkill[ItemOBJ[n_A_Equip[i]][9+j2]][2];
					w_ASSP9[j] = AutoSpellSkill[ItemOBJ[n_A_Equip[i]][9+j2]][0] + 2000;
					j++;
				}
			}
		}
	}

	for ( var i = 0; i <= 25; i++ )
	{
		for(j2=0;cardOBJ[n_A_card[i]][4+j2] != 0;j2 += 2)
		{
			if(cardOBJ[n_A_card[i]][4+j2] == 220)
			{
				if(InsertSkill[cardOBJ[n_A_card[i]][5+j2]][1] == 1)
				{
					w_ASSP0[j] = InsertSkill[cardOBJ[n_A_card[i]][5+j2]][2];
					w_ASSP9[j] = cardOBJ[n_A_card[i]][5+j2] + 3000;
					j++;
				}
			}
			else if(cardOBJ[n_A_card[i]][4+j2] == 221)
			{
				if(AutoSpellSkill[cardOBJ[n_A_card[i]][5+j2]][1] == 1)
				{
					w_ASSP0[j] = AutoSpellSkill[cardOBJ[n_A_card[i]][5+j2]][2];
					w_ASSP9[j] = cardOBJ[n_A_card[i]][5+j2] + 2000;
					j++;
				}
			}
		}
	}
	if(CardNumSearch(card_WEPN_LADYSOLAC) && (n_A_JOB == 9 || n_A_JOB == 23)) //Lady Solace
	{
		w_ASSP0[j] = 162;
		w_ASSP9[j] = 2095;
		j++;
	}
	if(CardNumSearch(card_WEPN_GRYPHON) && n_A_JobSearch()==1) //Gryphon
	{
		w_ASSP0[j] = 76;
		w_ASSP9[j] = 2096;
		j++;
	}
	if(EquipNumSearch(1096) && n_A_JobSearch2() != 9)
	{
		w_ASSP0[j] = 193;
		w_ASSP9[j] = 2108;
		j++;
	}

	if ( usableItems[ksCastScrolls] )
	{
		var wSC = [33,34,35,36,13,37,38,39,7];
		for ( var i = 0; i < 9; i++ )
		{
			w_ASSP0[j] = InsertSkill[wSC[i]][2];
			w_ASSP9[j] = wSC[i] + 3000;
			j++;
		}
		w_ASSP0[j] = InsertSkill[40][2];

		w_ASSP9[j] = 3040;
		j++;
	}

	w_ASSPch=0;
	for ( var i = 0; i < 20; i++ )
	{
		if ( w_ASSP0bk[i] != w_ASSP0[i] )
		{
			w_ASSPch = 1;
		}
	}

	if ( w_ASSPch )
	{
		var k;
		for ( k = 0; JobSkillActiveOBJ[n_A_JOB][k] !== 999; k++ );
		for ( var i = k + 20; i >= k; i-- )
		{
			document.calcForm.A_ActiveSkill.options[i] = null;
		}
		var j = 0;
		for ( var i = k; w_ASSP0[j] !== 999; i++, j++ )
		{
			if ( w_ASSP9[j] >= 3000 )
			{
				document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2],w_ASSP9[j]);
			}
			else
			{
				document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[w_ASSP0[j]][2]+"(Temp As)",w_ASSP9[j]);
			}
		}
	}
	for ( var i = 0; i < 20; i++ )
	{
		w_ASSP0bk[i] = w_ASSP0[i];
	}

	if ( document.forms["calcForm"].elements["A_ActiveSkill"].value === 0 )
	{
		document.forms["calcForm"].elements["A_ActiveSkillLV"].style.visibility = "hidden";
	}
}

function SetEquipShortCut()
{ //
with(document.calcForm)
{
	w = eval(A_EQUIP_SET_SHORT.value);
	if(EquipShortCutData[w][0] == 10000)
	{
		if(EquipShortCutData[w][1] != 0)
			A_weapon1_card1.value = EquipShortCutData[w][1];
		if(EquipShortCutData[w][2] != 0)
			A_head1_card.value = EquipShortCutData[w][2];
		if(EquipShortCutData[w][3] != 0 && n_Nitou == 0)
			A_left_card.value = EquipShortCutData[w][3];
		if(EquipShortCutData[w][4] != 0)
			A_body_card.value = EquipShortCutData[w][4];
		if(EquipShortCutData[w][5] != 0)
			A_shoulder_card.value = EquipShortCutData[w][5];
		if(EquipShortCutData[w][6] != 0)
			A_shoes_card.value = EquipShortCutData[w][6];
		if(EquipShortCutData[w][7] != 0)
			A_acces1_card.value = EquipShortCutData[w][7];
		if(EquipShortCutData[w][8] != 0)
			A_acces2_card.value = EquipShortCutData[w][8];
	}
	if(EquipShortCutData[w][0] == 9999)
	{
		A_HEAD_DEF_PLUS.value = 0;
		A_BODY_DEF_PLUS.value = 0;
		A_LEFT_DEF_PLUS.value = 0;
		A_SHOULDER_DEF_PLUS.value = 0;
		A_SHOES_DEF_PLUS.value = 0;

		A_head1.value = 142;
		A_head2.value = 243;
		A_head3.value = 268;
		A_left.value = 305;
		A_body.value = 279;
		A_shoulder.value = 311;
		A_shoes.value = 317;
		A_acces1.value = 326;
		A_acces2.value = 326;


		A_head1_card.value = 0;
		A_head2_card.value = 0;
		A_left_card.value = 0;
		A_body_card.value = 0;
		A_shoulder_card.value = 0;
		A_shoes_card.value = 0;
		A_acces1_card.value = 0;
		A_acces2_card.value = 0;
	}
	StAllCalc();
	ActiveSkillSetPlus();
}
}

function SetCardShortLeft()
{ // ActivateShortlist (left)
with(document.calcForm)
{
	w = eval(A_cardshortLeft.value);

	A_weapon2_card1.value = CardShort[w][1];
	A_weapon2_card2.value = CardShort[w][2];
	A_weapon2_card3.value = CardShort[w][3];
	A_weapon2_card4.value = CardShort[w][4];


	if(w == 9 || w == 10)
	{
		w = MonsterOBJ[eval(B_Enemy.value)][3];

		if(10 <= w && w <= 14)
			A_weapon2_card1.value = 204;
		if((20 <= w && w <= 24) || (80 <= w && w <= 94))
			A_weapon2_card1.value = 203;
		if(30 <= w && w <= 34)
			A_weapon2_card1.value = 201;
		if(40 <= w && w <= 44)
			A_weapon2_card1.value = 202;
	}
}
}

wESx = new Array();
for(i=0;i<=EnemyNum;i++)
	wESx[i]=new Array();

function EnemySort()
{ // Sort Monsters by order
	var len = document.calcForm.B_Enemy.length; // EnemyList
	for ( var i=0;i<len;i++) // clear
		document.calcForm.B_Enemy.options[0] = null;

	ESNum= [1,3,2,5,21,22,16,17,13,100]; // SortOrder

	var wES2 = eval( document.calcForm.ENEMY_SORT.value ); // SortOrder
	if ( wES2 === 0 )
	{ // Alphabetical
		var x = new Array();
		for ( var i = 0 ;i <= EnemyNum; i++)
		{
			x[i] = MonsterABC[i];
		}

		x = SZ( x );
		var j = 0;
		for( var i = 0; i <= EnemyNum; i++ )
		{
			if ( x[i] !== -1 )
			{
				//console.log(x[i]);
				document.calcForm.B_Enemy.options[j] = new Option( MonsterOBJ[x[i]][en_CLASS] + " (Lvl " + MonsterOBJ[x[i]][en_LEVEL] + ")", x[i] );
				j++;
			}
		}
		return;
	}

	wES = ESNum[eval(document.calcForm.ENEMY_SORT.value)];
	wESx[0][0] = "S";
	wESx[0][1] = "E";
	STERTw = 0;
	ENDw = 0;
	for ( var i = 1; i <= EnemyNum; i++ )
	{
		j=ENDw;
		if(MonsterOBJ[i][wES] >= MonsterOBJ[j][wES])
		{
			wESx[j][1] = i;
			wESx[i][0] = j;
			wESx[i][1] = "E";
			ENDw=i;
		}
		else
		{
			j=STERTw;
			if(MonsterOBJ[i][wES] <= MonsterOBJ[j][wES])
			{
				wESx[j][0] = i;
				wESx[i][0] = "S";
				wESx[i][1] = j;
				STERTw=i;
			}
			else
			{
				j=STERTw;
				jbk=STERTw;
				while(MonsterOBJ[i][wES] > MonsterOBJ[j][wES])
				{
					jbk=j;
					j = wESx[j][1];
				}
				wESx[jbk][1] = i;
				wESx[i][0] = jbk;
				wESx[i][1] = j;
				wESx[j][0] = i;
			}
		}
	}

	var x = new Array();
	var i;
	x[0] = i = STERTw;
	for(var j=1;wESx[i][1]!="E";j++)
	{
		x[j] = wESx[i][1];
		i = wESx[i][1];
	}
	x = SZ(x);

	ESwork2 = new Array();
	if(wES==21||wES==22)
	{ // PerfHit/ Dodge
		for(i=0;i<=EnemyNum;i++)
			ESwork2[i] = MonsterOBJ[i][wES] +" << ";
	}
	else if(wES==2)
	{ // Race
		for(i=0;i<=EnemyNum;i++)
			ESwork2[i] = SyuzokuOBJ[MonsterOBJ[i][en_RACE]][Language] +" << ";
	}
	else if(wES==3)
	{ // Element
		for(i=0;i<=EnemyNum;i++)
			ESwork2[i] = ZokuseiOBJ[Math.floor(MonsterOBJ[i][en_ELEMENT] /10)][Language] + MonsterOBJ[i][en_ELEMENT] % 10 +" << ";
	}
	else
	{ // rest ?
		for(i=0;i<=EnemyNum;i++)
			ESwork2[i] = "";
	}

	// Fill Options List
	var j=0;
	for( var i = 0; i <= EnemyNum; i++ )
	{
		if ( x[i] != -1 )
		{
			document.calcForm.B_Enemy.options[j] = new Option(ESwork2[x[i]] + MonsterOBJ[x[i]][en_CLASS] + " (Lvl " + MonsterOBJ[x[i]][en_LEVEL] + ")",x[i]);
			j++;
		}
	}
}

function SZ( wSTR )
{ // MonsterMapFilter Event
	var w = document.forms["calcForm"].elements["ENEMY_SORT2"].value;

	if ( w != 0 )
	{
		for ( var i = 0;i <= EnemyNum; i++ )
		{
			if ( wSTR[i] != -1 )
			{
				for ( var j = 0; MonMap[w][j] != "N"; j++ )
				{
					if ( wSTR[i] == MonMap[w][j] )
						break;
				}

				if ( MonMap[w][j] == "N" )
					wSTR[i] = -1;
			}
		}
	}

	return wSTR;
}

Init();

function Init()
{
	formElements = document.forms["calcForm"].elements;

	//enemies active skills
	var mobSkills = formElements["A_ActiveSkill_en"];
	var mobSkillsLV = formElements["A_ActiveSkillLV_en"];

	// active skills [list refresh]
	var len2 = mobSkills.length;
	var len3 = mobSkillsLV.length;

	for ( var i = 0; i < len2; i++ )
		mobSkills.options[0] = null;

	for ( var i = 0; i < len3; i++ )
		mobSkillsLV.options[0] = null;


	for ( var i = 0; i < enemySkills.length; i++ )
	{
		mobSkills.options[i] = new Option( enemySkills[i][0], i );
	}
	for ( var i = 3; i < enemySkills[0].length; i++ )
	{
		mobSkillsLV.options[i-3] = new Option( i - 2, i - 2 );
	}

	for ( var i = 1; i <= 99; i++ )
	{ // Fill BLvl and stats
		formElements["A_BaseLV"].options[i-1] = new Option(i,i);
		formElements["A_STR"].options[i-1] = new Option(i,i);
		formElements["A_AGI"].options[i-1] = new Option(i,i);
		formElements["A_VIT"].options[i-1] = new Option(i,i);
		formElements["A_INT"].options[i-1] = new Option(i,i);
		formElements["A_DEX"].options[i-1] = new Option(i,i);
		formElements["A_LUK"].options[i-1] = new Option(i,i);
	}

	// initialize buff arrays
	for ( var i = 0; i < 40; i++ )
	{
		selfBuffs[i] = 0;
		acolyteBuffs[i] = 0;
		performerBuffs[i] = 0;
		guildBuffs[i] = 0;
		battleChantBuffs[i] = 0;
		otherBuffs[i] = 0;
		miscEffects[i] = 0;
		usableItems[i] = 0;
		battleEffects[i] = 0;
		monsterDebuffs[i] = 0;
		monsterBuffs[i] = 0;
	}

	for ( var i = 0; i <= 25; i++ )
	{
		n_A_Equip[i] = 0;
		n_A_card[i] = 0;
	}

	for ( var i = 0; i <= 20; i++ )
	{
		w_ASSP0bk[i]=999;
	}

	// initialize stats array
	for ( var i = 0; i <= 450; i++ )
	{
		n_tok[i] = 0;
	}

	/*
	if(location.href.match("file:/"))
	{
  		document.calcForm.A_SaveType.value = 1;
  	}
  	else
  	{
 		var w_iech=0;
 		for(var i=8;i<=15;i++)
 		{
 			var w_ie = "MSIE " +i;
 			if(window.navigator.userAgent.indexOf(w_ie) != -1)
 			{
 				w_iech = 1;
 				break;
 			}
 		}
 		if(w_iech)
 		{
 			document.calcForm.A_SaveType.value = 0;
 			n_SaveMode = 0;
 		}
 		if(window.navigator.userAgent.indexOf("Firefox") != -1)
 		{
 			document.calcForm.A_SaveType.value = 0;
 			n_SaveMode = 0;
 		}
 		if(window.navigator.userAgent.indexOf("Safari") != -1)
 		{
 			if(window.navigator.userAgent.indexOf("Version/4") != -1 || window.navigator.userAgent.indexOf("Version/5") != -1)
 			{
 				document.calcForm.A_SaveType.value = 0;
 				n_SaveMode = 0;
 			}
 		}
 	}

 	if(n_SaveMode == 0)
 	{
 		var w = "";
 		w = '<input type="button" value="S" onClick="SaveShortCut()">'
 		w += '<input type="button" value="L" onClick="LoadShortCut()">';
 		myInnerHtml("ID_A_SHORTCUT_SAVE_BUTTON",w,0);
 	}*/

	for(i=0;i<=99;i++)
	{
		DataShortCut[i] = new Array();
		for(j=0;j<=49;j++)
		{
			DataShortCut[i][j] = new Array();
			DataShortCut[i][j][0] = 0;
			DataShortCut[i][j][1] = 0;
			DataShortCut[i][j][2] = 0;
			DataShortCut[i][j][3] = 0;
			DataShortCut[i][j][4] = 0;
			DataShortCut[i][j][5] = 0;
			DataShortCut[i][j][6] = 0;
		}
	}

	// -----------

	/*for ( var i = 0; i < cls_COUNT; i++ )
	{ // Jobs
		document.calcForm.A_JOB.options[i] = new Option(JobName[i][Language],i);
	}*/

	for ( var i = 0; i < ArrowOBJ.length; i++ )
	{ // Build Arrow List
		formElements["A_Arrow"].options[i] = new Option( ArrowOBJ[i][Language + 2], i );
	}

	// WeapElement
	document.calcForm.A_Weapon_element.options[0] = new Option("-",0);
	for(i=1;i<=9;i++)
		document.calcForm.A_Weapon_element.options[i] = new Option(ZokuseiOBJ[i][Language],i);

	for(i=0;i<=20;i++)
	{ // Upgrades
		document.calcForm.A_Weapon_ATKplus.options[i] = new Option("+"+i,i);
		document.calcForm.A_HEAD_DEF_PLUS.options[i] = new Option("+"+i,i);
		document.calcForm.A_BODY_DEF_PLUS.options[i] = new Option("+"+i,i);
		document.calcForm.A_LEFT_DEF_PLUS.options[i] = new Option("+"+i,i);
		document.calcForm.A_SHOULDER_DEF_PLUS.options[i] = new Option("+"+i,i);
		document.calcForm.A_SHOES_DEF_PLUS.options[i] = new Option("+"+i,i);
	}

	var wLang = Language * 2;

	for ( var i = 0; CardSortOBJ[0][i] != "NULL"; i++ )
	{ // WeaponSlot1
		document.calcForm.A_weapon1_card1.options[i] = new Option( cardOBJ[CardSortOBJ[card_comp_NONE][i]][2], CardSortOBJ[card_comp_NONE][i] );
	}
	for ( var i = 0;CardSortOBJ[1][i]!="NULL";i++)
	{ // WeaponSlot2-4
		document.calcForm.A_weapon1_card2.options[i] = new Option(cardOBJ[CardSortOBJ[card_comp_WEAPON][i]][2],CardSortOBJ[card_comp_WEAPON][i]);
		document.calcForm.A_weapon1_card3.options[i] = new Option(cardOBJ[CardSortOBJ[card_comp_WEAPON][i]][2],CardSortOBJ[card_comp_WEAPON][i]);
		document.calcForm.A_weapon1_card4.options[i] = new Option(cardOBJ[CardSortOBJ[card_comp_WEAPON][i]][2],CardSortOBJ[card_comp_WEAPON][i]);
	}
	document.calcForm.A_weapon1_card4.options[4] = new Option(GetWord(91),106); // "Top10Rank"

	for ( var i=0;CardSortOBJ[2][i]!="NULL";i++ )
	{ // HeadCard
		document.calcForm.A_head1_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_comp_HEAD][i]][2],CardSortOBJ[card_comp_HEAD][i]);
		document.calcForm.A_head2_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_comp_HEAD][i]][2],CardSortOBJ[card_comp_HEAD][i]);
	}
	for(i=0;CardSortOBJ[3][i]!="NULL";i++) // Shield-/ LeftHandCard
		document.calcForm.A_left_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_SHIELD][i]][2],CardSortOBJ[card_com_SHIELD][i]);
	for(i=0;CardSortOBJ[4][i]!="NULL";i++) // ArmorCard
		document.calcForm.A_body_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_ARMOR][i]][2],CardSortOBJ[card_com_ARMOR][i]);
	for(i=0;CardSortOBJ[5][i]!="NULL";i++) // GarmentCard
		document.calcForm.A_shoulder_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_GARMENT][i]][2],CardSortOBJ[card_com_GARMENT][i]);
	for(i=0;CardSortOBJ[6][i]!="NULL";i++) // ShoesCard
		document.calcForm.A_shoes_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_SHOES][i]][2],CardSortOBJ[card_com_SHOES][i]);
	for(i=0;CardSortOBJ[7][i]!="NULL";i++)
	{ // AccessoryCards
		document.calcForm.A_acces1_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_ACC][i]][2],CardSortOBJ[card_com_ACC][i]);
		document.calcForm.A_acces2_card.options[i] = new Option(cardOBJ[CardSortOBJ[card_com_ACC][i]][2],CardSortOBJ[card_com_ACC][i]);
	}

	for(i=0;i<CardShort.length;i++) // CardShortcuts
		document.calcForm.A_SHORTCUT_R.options[i] = new Option(CardShort[i][Language+4],i);

	for(i=0;i<EquipShortCutData.length;i++) // Sets
		document.calcForm.A_cardshort.options[i] = new Option(EquipShortCutData[i][Language+9],i);
		//document.calcForm.A_EQUIP_SET_SHORT.options[i] = new Option(EquipShortCutData[i][9 + Language],i);

	// Hidden Slot enchant
	var HSEname = ["STR","AGI","VIT","INT","DEX","LUK"];
	document.calcForm.A_HSE.options[0] = new Option( "(Armor Enchant)", 0 );
	var iHSE=1;
	for(i=0;i<=5;i++)
	{
		for( var j=1;j<=3;j++ )
		{
			document.calcForm.A_HSE.options[iHSE] = new Option( HSEname[i] + "+" + j, ( i * 10 ) + j );
			iHSE++;
		}
	}
	document.calcForm.A_HSE_HEAD1.options[0] = new Option( "(Headgear Enchant)", 0 );
	var iHSE=1;
	for(i=0;i<=5;i++)
	{
		for( var j=1;j<=3;j++ )
		{
			document.calcForm.A_HSE_HEAD1.options[iHSE] = new Option( HSEname[i] + "+" + j, ( i * 10 ) + j );
			iHSE++;
		}
	}

	// insert temporary descriptions into item (card) descs ---------------
	for (var i = 0; i < ITEM_SP_TIME_OBJ.length; i++ )
	{
		if ( ITEM_SP_TIME_OBJ[i][3] === 1 )
		{
			var str = "<b style=\"color:#828282\">(Special Effect: [" + ITEM_SP_TIME_OBJ[i][2] + "] can be activated under 'Miscellaneous Effects' in the Self &amp; Party Buffs section)</b>";
			if ( ITEM_NAME[ITEM_SP_TIME_OBJ[i][4]][Language*3+1] === 0 )
			{
				ITEM_NAME[ITEM_SP_TIME_OBJ[i][4]][Language*3+1] = str;
			}
			else
			{
				ITEM_NAME[ITEM_SP_TIME_OBJ[i][4]][Language*3+1] += "<BR>"+ str;
			}
		}
	}

	for ( var i = 0; i < ITEM_SP_TIME_OBJ.length; i++ )
	{ // special time based bonuses for cards
		if ( ITEM_SP_TIME_OBJ[i][3] === 2 )
		{
			var str = "<b style=\"color:#7a7a7a\">(Special Effect: [" + ITEM_SP_TIME_OBJ[i][2] + "] can be activated under 'Miscellaneous Effects' in the Self &amp; Party Buffs section)</b>";
			if ( cardOBJ[ITEM_SP_TIME_OBJ[i][4]][3] === 0 )
			{
				cardOBJ[ITEM_SP_TIME_OBJ[i][4]][3] = str;
			}
			else
			{
				cardOBJ[ITEM_SP_TIME_OBJ[i][4]][3] += "<BR>" + str;
			}
		}
	}
	//Special enchants
	document.calcForm.E_BOOST_STR.value = 0;
	document.calcForm.E_BOOST_AGI.value = 0;
	document.calcForm.E_BOOST_INT.value = 0;
	document.calcForm.E_BOOST_VIT.value = 0;
	document.calcForm.E_BOOST_DEX.value = 0;
	document.calcForm.E_BOOST_LUK.value = 0;
	document.calcForm.E_BOOST_ATK.value = 0;
	document.calcForm.E_BOOST_ATK_PERC.value = 0;
	document.calcForm.E_BOOST_MATK.value = 0;
	document.calcForm.E_BOOST_MATK_PERC.value = 0;
	document.calcForm.E_BOOST_HIT.value = 0;
	document.calcForm.E_BOOST_FLEE.value = 0;
	document.calcForm.E_BOOST_DODGE.value = 0;
	document.calcForm.E_BOOST_HP.value = 0;
	document.calcForm.E_BOOST_SP.value = 0;
	document.calcForm.E_BOOST_HP_PERC.value = 0;
	document.calcForm.E_BOOST_SP_PERC.value = 0;
	document.calcForm.E_BOOST_RANGED.value = 0;
	document.calcForm.E_BOOST_DEF.value = 0;
	document.calcForm.E_BOOST_MDEF.value = 0;
	document.calcForm.E_BOOST_CRIT.value = 0;
	document.calcForm.E_BOOST_RED_PERC.value = 0;
	document.calcForm.E_BOOST_ASPD.value = 0;
	document.calcForm.E_BOOST_ASPD_PERC.value = 0;
	document.calcForm.E_BOOST_CASTING.value = 0;


	// -------------------------------------------------

	if ( PlayerVersusPlayer === 0 )
	{ // not human
		for ( var i = 38; i <= 49; i++ )
		{
			myInnerHtml( "nm0" + i, GetWord(i), 0 );
		}
		for ( var i = 53; i <= 66; i++ )
		{
			myInnerHtml( "nm0" + i, GetWord(i), 0 );
		}

		for ( var i = 0; i < EXTENDED_INFO_NAME.length; i++ )
		{ // ExtendedInfoList
			formElements["ExtendedInfo"].options[i] = new Option( EXTENDED_INFO_NAME[i][1 + Language], EXTENDED_INFO_NAME[i][0] );
		}
		for(var i=0;i<SORT_NAME.length;i++) // EnemySortList
			document.calcForm.ENEMY_SORT.options[i] = new Option(SORT_NAME[i][Language],i);
		for(var i=0;i<MAP_NAME.length;i++) // MapList
			document.calcForm.ENEMY_SORT2.options[i] = new Option(MAP_NAME[i][Language],i);

		//document.calcForm.ID_X0.value = GetWord(165); // SaveCookieConf
		document.calcForm.ID_X1.value = GetWord(166); // Save as URL
		document.calcForm.ID_X2.value = GetWord(167); // Calc

		EnemySort();
	}
	//Build Enemy Skills

	// Build Tables
	BuildAcolyteBuffsTable();
	BuildOtherBuffsTable();
	BuildMiscEffectsTable();
	BuildBattleEffectsTable();
	BuildMonsterDebuffTable();
	BuildMonsterBuffTable();
	BuildItemsTable();
	FillPerformerBuffOptions();

	// init player
	formElements["A_JOB"].value = 0;
	ChangeJob( 0 );
	FirstNovis();

	StCalc();
	StAllCalc();
	calc();
	LoadCookie3(); // shall be del
	//LoadCookieConf();
	URLIN();
	formElements["saveName"].value = GetWord(84);

	LoadDataINIT(); // loads cookies
}