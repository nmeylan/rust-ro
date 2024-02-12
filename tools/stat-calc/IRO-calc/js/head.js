function calc()
{ // init BattleCalculation
	// Init variables
	totalCastTime = 0;

	// Init the damage strings.
	for ( var i = 0; i < 3; i++ )
	{
		InnStr[i] = "";
	}

	// Re-Calc all stats.
	StAllCalc();
	// Find Functional HIT.
	w_HIT = n_A_HIT - n_B_FLEE;
	w_HIT_EDP = w_HIT;
	w_HIT_EDP = Max( 5, Min( w_HIT_EDP, 100 ) ); // 5 <= x <= 100
	CalcHitAfterSpecialSkills();
	w_HIT_HYOUJI = w_HIT;

	// Find Critical Blow Chance.
	CalcCriticalBlowChance();

	// Find Trifecta Blow Chance.
	CalcTrifectaBlowChance();

	// Find Double Attack Chance.
	CalcDoubleAttackChance();

	// Duple light.
	CalcDupleLightChance();

	// Now Do something with the data.
	CalcFinalCriticalChance();

	damageType = kDmgTypeMelee;
	// Calculate damage.
	// I ADD 2 copies of this calculation because this function sets the ranged and magic property, used in GetBaseDmg.
	//I need to do this because I don't want to remake the entire program. XD
	/*CalcSkillDamage();
	InnStr[0] = "";
	InnStr[1] = "";
	InnStr[2] = "";*/
	CalcSkillDamageType();
	// Prep for damage calculations.
	baseDamage = GetBaseDmg( n_A_Weapon_element, false, 0 );
	for ( var i = 0; i < 3; i++ )
	{
		// Initialize working damage with base damage.
		n_A_DMG[i] = baseDamage[i];

		// Criticals always take the max base damage.
		n_A_CriATK[i] = baseDamage[2];
	}

	// Apply Base Damage Mods
	CalcBaseDamageMods();

	// Calculate damage.
	CalcSkillDamage();

	// Display additional data.
	DisplayAdditionalBattleInfo();
}

function CalcBaseDamageMods()
{ // Things that will affect base damage before anything else

	//MOVED THIS IN CALC ATTACK TO VIEW IN ATTACK WINDOW
	/*var baseDamageMod = 100;

	if ( n_A_ActiveSkill != skill_MO_OCCULT_IMPACTION &&
		 n_A_ActiveSkill != skill_MO_GUILLOTINE_FIST &&
		 n_A_ActiveSkill != skill_MO_MAX_GUILLOTINE_FIST )
	{
		if (SkillSearch(skill_SW_BERSERK))
			baseDamageMod += 32;
		else if (otherBuffs[ksProvoke])
			baseDamageMod += 2 + 3 * otherBuffs[ksProvoke];
		else if (otherBuffs[ksAloe])
			baseDamageMod += 5;
//		if (SkillSearch(skill_LK_SPEAR_DYNAMO))
//			baseDamageMod += SkillSearch(skill_LK_SPEAR_DYNAMO) * 5;
//		if (SkillSearch(skill_SN_FALCON_EYES))
//			baseDamageMod += SkillSearch(skill_SN_FALCON_EYES) * 2;
		if (battleChantBuffs[pass_V_ATK])
			baseDamageMod += 100;
		if (otherBuffs[ksMurderBonus])
			baseDamageMod += 10;
		if (StPlusCalc2(87))
			baseDamageMod += StPlusCalc2(87);
		if (miscEffects[ksCursed])
			baseDamageMod -= 25;
	}

	for ( var i = 0; i < 3; i++ )
	{ // apply to working damage and crit damage
		n_A_DMG[i] = n_A_DMG[i] * baseDamageMod / 100;
		n_A_CriATK[i] = n_A_CriATK[i] * baseDamageMod / 100;
	}*/
}

function CalcAtkMods02( skillMod, criticalAttack )
{ // skillmod + x - (start skillmod, critAtk:normalatk)
	var localAttackMod = CalcSkillModAdditions( skillMod * 100 );

	if ( criticalAttack === 0 )
	{
		// non-crit
		if ( n_A_Weapon_element !== BK_Weapon_element )
		{
			n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		}

		n_A_DMG[0] = Math.floor( n_A_DMG[0] * localAttackMod / 100 );
		n_A_DMG[1] = Math.floor( n_A_DMG[1] * localAttackMod / 100 );
		n_A_DMG[2] = Math.floor( n_A_DMG[2] * localAttackMod / 100 );
	}
	else
	{
		// crit
		if ( n_A_Weapon_element !== BK_Weapon_element )
		{
			n_A_CriATK = GetBaseDmg( n_A_Weapon_element, false, 0 );
		}

		n_A_CriATK[0] = Math.floor( n_A_CriATK[0] * localAttackMod / 100 );
		n_A_CriATK[1] = Math.floor( n_A_CriATK[1] * localAttackMod / 100 );
		n_A_CriATK[2] = Math.floor( n_A_CriATK[2] * localAttackMod / 100 );
	}
}

function CalcFalconDamage()
{ // Calculates Falcon Damage
	if ( n_A_WeaponType == weapTyp_BOW && SkillSearch( skill_HU_BLITZ_BEAT ) && n_A_ActiveSkill != skill_SN_FOCUSED_ARROW_STRIKE )
	{
		hunterPetHits = Math.floor( ( n_A_JobLV - 1 ) / 10 + 1 );
		if (hunterPetHits > 5)
		{
			hunterPetHits =5;
		}
		wBTw2 = SkillSearch( skill_HU_BLITZ_BEAT );
		if (wBTw2 < hunterPetHits)
		{
			hunterPetHits = wBTw2;
		}
		wBT = 80 + Math.floor( n_A_DEX / 10 )*2 + Math.floor( n_A_INT / 2 )*2 + SkillSearch( skill_HU_STEEL_CROW ) * 6;
		wBT = Math.floor( wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL] / 100 );
		wBT = tPlusDamCut(wBT);
		wBTw3 = Math.round( ( 1 + n_A_LUK * 0.3 ) * 100) / 100 ;
		if ( n_B[en_ID] == 44 )
		{
			wBT = 0;
		}
		str_bSUBname += "Falcon Damage<BR>";
		hunterPetDamage = wBT * hunterPetHits;
		str_bSUB += hunterPetDamage +" ("+ wBT +" x "+ hunterPetHits +"Hit)";
		str_bSUB += "("+ wBTw3 +"% Chance)<BR>";
		wBT = hunterPetDamage * wBTw3 / 100;
		wBT = wBT * (w_HIT + ((100 - w_HIT) * criticalAttackChance /100)) /100;
		hunterPetHits = 0;
		return Math.round( wBT * 100 ) / 100;
	}
	else
	{
		hunterPetDamage = 0;
		return 0;
	}
}

function CalcWargDamage()
{ // Calculates Warg Damage
	if ( n_A_WeaponType == weapTyp_BOW && SkillSearch( skill_RAN_WARG_STRIKE ) && n_A_ActiveSkill != skill_SN_FOCUSED_ARROW_STRIKE )
	{
		wargHits = 1;
		not_use_card = 1;
		noequipatk = true;
		var TMPATK = GetBaseDmg( ele_NEUTRAL, true, SkillSearch( skill_RAN_TOOTH_OF_WARG ) * 30);
		noequipatk = false
		//wBT = 80 + Math.floor( n_A_DEX / 10 ) * 2 + Math.floor( n_A_INT / 2 ) * 2 + SkillSearch( skill_RAN_TOOTH_OF_WARG ) * 30;
		for (i = 0; i < 3; i++) {
			TMPATK[i] = TMPATK[i] * SkillSearch( skill_RAN_WARG_STRIKE ) * 2;

			if (n_tok[bon_DMG_SKILL + skill_RAN_WARG_STRIKE] !== undefined) {
				TMPATK[i] *= 1 + (n_tok[bon_DMG_SKILL + skill_RAN_WARG_STRIKE] / 100.0);
			}
		}
		not_use_card = 0;
		//wBT = Math.floor( wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL] / 100 );
		//wBT = tPlusDamCut(wBT);
		wBT = ApplyEnemyDefense(TMPATK[1], 2, 0);
		for (i = 0; i < 3; i++) {
			TMPATK[i] = tPlusDamCut(TMPATK[i]);
		}
		wBTw3 = Math.round( ( 1 + n_A_LUK * 0.3 ) * 100) / 100 ;
		if ( n_B[en_ID] === 44 )
		{
			wBT = 0;
		}
		str_bSUBname += "Warg Damage<br/>";
		hunterPetDamage = wBT * wargHits;
		str_bSUB += hunterPetDamage + " ("+ wBTw3 +"% Chance)<br/>";
		wBT = hunterPetDamage * wBTw3 / 100;
		wBT = wBT * ( w_HIT + ( ( 100 - w_HIT ) * criticalAttackChance / 100 ) ) / 100;
		wargHits = 0;
		return Math.round( wBT * 100 ) / 100;
	}
	else
	{
		hunterPetDamage = 0;
		return 0;
	}
}

function CalcHitAfterSpecialSkills()
{
	if ( SkillSearch( skill_BS_WEAPONRY_RESEARCH ) )
	{
		w_HIT = Math.floor( w_HIT * ( 100 + 2 * SkillSearch( skill_BS_WEAPONRY_RESEARCH ) ) / 100 );
	}
	if ( n_A_ActiveSkill==skill_KN_PIERCE || n_A_ActiveSkill==skill_SW_BASH )
		w_HIT *= 1 + n_A_ActiveSkillLV * 0.05;
	if ( ( n_A_ActiveSkill == skill_AS_SONIC_BLOW || n_A_ActiveSkill == skill_AS_SONIC_BLOW_SL ) && SkillSearch( skill_AS_SONIC_ACCELERATION ) )
		w_HIT *= 1.5;
	if ( n_A_ActiveSkill==skill_SW_MAGNUM_BREAK )
		w_HIT *= 1 + n_A_ActiveSkillLV * 0.1;
	if ( n_A_ActiveSkill==skill_SN_FOCUSED_ARROW_STRIKE )
		w_HIT *= (1 + n_A_ActiveSkillLV * 0.1);
	if ( n_A_ActiveSkill==skill_TK_COUNTER_KICK )
		w_HIT = 100;
	if ( n_A_ActiveSkill==skill_CR_SHIELD_BOOMERANG_SL )
		w_HIT = 100;
	if ( SkillSearch(skill_TKM_UNION ) )
		w_HIT = 100;
	w_HIT = Max(5,Min(w_HIT,100)); // 5 <= x <= 100

	if(StPlusCalc2(bon_CH_GUIDE_ATK)+StPlusCard(bon_CH_GUIDE_ATK))
		w_HIT = w_HIT + (100 - w_HIT) * (StPlusCalc2(bon_CH_GUIDE_ATK)+StPlusCard(bon_CH_GUIDE_ATK)) / 100;

	w_HIT = Math.floor( w_HIT * 100) / 100;
}

function CalcCriticalBlowChance()
{
	var bonusCrit = 0;
	if ( n_A_ActiveSkill === skill_SN_FOCUSED_ARROW_STRIKE )
	{
		bonusCrit = 20;
	}
	if ( n_A_ActiveSkill === skill_NIN_SHADOW_SLASH )
	{
		bonusCrit = 25 + ( n_A_ActiveSkillLV * 5 );
	}

	var totalCrit = n_A_CRI + bonusCrit;
	criticalAttackChance = totalCrit - n_B[en_LUK] * 0.2 + 0.1; // CritShield

	if ( monsterDebuffs[status_en_SLEEP] )
	{ // Sleep doubles the chance for a crit
		criticalAttackChance *= 2;
	}

	criticalAttackChance = Max( 0, Min( criticalAttackChance, 100 ) ); // 0 <= x <= 100
}

function CalcTrifectaBlowChance()
{
	trifectaBlowDamage = 0;
	trifectaBlowActivationRate = 0;
	if ( SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW ) )
	{
		trifectaBlowActivationRate = 30 - SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW );
	}
	if ( SkillSearch( skill_RUN_GIANT_GROWTH ) )
	{
		trifectaBlowActivationRate = 15;
	}
	if ( n_A_WeaponType == weapTyp_BOW && SkillSearch( skill_RAN_FEAR_BREEZE ) )
	{
		var breezeLevel = SkillSearch( skill_RAN_FEAR_BREEZE );
		var breezeChance = 12;

		if ( breezeLevel === 3 )
		{
			breezeChance += 9;
		}
		else if ( breezeLevel === 4 )
		{
			breezeChance += 15;
		}
		else if ( breezeLevel === 5 )
		{
			breezeChance += 18;
		}
		trifectaBlowActivationRate = breezeChance;
	}
}
function CalcDupleLightChance() {
	if (SkillSearch(skill_ABI_DUPLE_LIGHT)) {
		dupleLightChance = SkillSearch(skill_ABI_DUPLE_LIGHT)*2 + 10;
	}
}
function CalcDoubleAttackChance()
{
	doubleAttackChance = SkillSearch(skill_TH_DOUBLE_ATTACK) * 5;
	if ( n_A_WeaponType != weapTyp_DAGGER )
	{ // dagger only.
		doubleAttackChance = 0;
	}
	if ( CardNumSearch( 43 ) )
	{ // Side Winder Card
		if ( SkillSearch( skill_TH_DOUBLE_ATTACK ) > 1 )
		{
			doubleAttackChance = SkillSearch( skill_TH_DOUBLE_ATTACK ) * 5;
		}
		else
		{
			doubleAttackChance = 5;
		}
	}
	if ( EquipNumSearch( 570 ) && n_A_WeaponType != weapTyp_NONE )
	{ // Chick Hat
		if ( SkillSearch( skill_TH_DOUBLE_ATTACK ) > 1 )
		{
			doubleAttackChance = SkillSearch( skill_TH_DOUBLE_ATTACK ) * 5;
		}
		else
		{
			doubleAttackChance = 10;
		}
	}
	if ( EquipNumSearch( 1296 ) && n_A_WeaponType != weapTyp_NONE )
	{ // Snake Head
		if ( SkillSearch( skill_TH_DOUBLE_ATTACK ) > 1 )
		{
			doubleAttackChance = SkillSearch( skill_TH_DOUBLE_ATTACK ) * 5;
		}
		else
		{
			doubleAttackChance = 25;
		}
	}
	if ( EquipNumSearch( 399 ) )
	{ // Nagan
		if ( SkillSearch( skill_TH_DOUBLE_ATTACK ) > 5 )
		{
			doubleAttackChance = SkillSearch( skill_TH_DOUBLE_ATTACK ) * 5;
		}
		else
		{
			doubleAttackChance = 25;
		}
	}
	if ( n_A_WeaponType === weapTyp_HANDGUN )
	{ //Chain Action
		doubleAttackChance = SkillSearch( skill_GS_CHAIN_ACTION ) * 5;
		if ( CardNumSearch( 43 ) )
		{ // Side Winder Card
			doubleAttackChance = SkillSearch( skill_GS_CHAIN_ACTION ) * 5 + ( ( 100 - SkillSearch( skill_GS_CHAIN_ACTION ) * 5 ) * 5 / 100 );
		}
		if ( EquipNumSearch( 570 ) )
		{ // Chick Hat
			doubleAttackChance = SkillSearch( skill_GS_CHAIN_ACTION ) * 5 + ( ( 100 - SkillSearch( skill_GS_CHAIN_ACTION ) * 5 ) * 10 / 100 );
		}
		if ( EquipNumSearch( 1296 ) )
		{ // Snake Head
			doubleAttackChance = SkillSearch( skill_GS_CHAIN_ACTION ) * 5 + ( ( 100 - SkillSearch( skill_GS_CHAIN_ACTION ) * 5 ) * 25 / 100 );
		}
	}

	doubleAttackHit = w_HIT;
	if ( doubleAttackChance != 0 && n_A_WeaponType != weapTyp_HANDGUN )
	{
		doubleAttackHit = doubleAttackHit * ( 100 + SkillSearch( skill_TH_DOUBLE_ATTACK ) ) / 100;
		doubleAttackHit = Min( doubleAttackHit, 100 );
	}
}

function CalcFinalCriticalChance()
{
	meleeChanceAfterTrifecta = 100 - trifectaBlowActivationRate;
	w998B = trifectaBlowActivationRate * w_HIT / 100;
	w998C = trifectaBlowActivationRate - w998B;
	w998D = meleeChanceAfterTrifecta * doubleAttackChance / 100;
	w998E = w998D * doubleAttackHit /100;
	w998F = w998D - w998E;
	w998G = ( 100 - trifectaBlowActivationRate - w998D ) * criticalAttackChance /100;
	w998H = 100 - trifectaBlowActivationRate -w998D -w998G;
	w998I = w998H * w_HIT /100;
	w998J = w998H - w998I;
	w998K = w998B + w998E + w998G + w998I;
	w998L = 100 - w998K;

	if ( n_A_ActiveSkill == skill_ALL_BASIC_ATTACK ||
		 n_A_ActiveSkill == skill_SN_FOCUSED_ARROW_STRIKE ||
		 n_A_ActiveSkill == skill_NIN_SHADOW_SLASH ||
		 ( n_A_ActiveSkill == skill_AS_POISON_REACT &&
		   ( n_B[en_ELEMENT] >= ele_POISON * 10 &&
		   	 n_B[en_ELEMENT] < ele_HOLY * 10 ) ) )
	{
		w_HIT_HYOUJI = Math.floor( w998K * 100 ) / 100;
	}
}

function CalcFinalDamage( damage, type )
{ // Calc Dmg from RAWDmg (rawDmg, (min,avg,max,crit:=10))
	damage = ApplyDamageModifiers( damage );
	damage = ApplySkillModifiers( damage );

	if ( type == 10 )
	{
		damage = ApplyEnemyDefense( damage * 1.4, type, 0 );
	}
	else
	{
		damage = ApplyEnemyDefense( damage, type, 0 );
	}
	damage = Math.floor(tPlusDamCut(damage));
	damage = Max( 0, damage );
	return Math.floor( damage );

/* Old BattleCalc order
	if ( type == 10 )
	{
		damage = ApplyEnemyDefense( damage * 1.4, type, 0 );
	}
	else
	{
		damage = ApplyEnemyDefense( damage, type, 0 );
	}

	damage = Max( 0, damage );
	damage = ApplyDamageModifiers( damage );
	return Math.floor( damage );
*/
}

function CalcRightHandDamage( w998 )
{
	trifectaDamage = w998B * trifectaBlowDamage;
	doubleAttackDamage = w998E * w998 * 2;
	critDamage = w998G * n_A_CriATK[1];
	dupleLightPhysicalDamage = (dupleLightChance * w_HIT / 100) * dupleLightPhysicalDamage;
	dupleLightMagicalDamage = (dupleLightChance * w_HIT / 100) * dupleLightMagicalDamage;
	normalDamage = w998I * w998;
	neverMissDamage = w998L * ApplyDamageModifiers(0);

	totalDamage = ( dupleLightMagicalDamage + dupleLightPhysicalDamage + trifectaDamage + doubleAttackDamage + critDamage + normalDamage + neverMissDamage ) / 100;

	return totalDamage;
}

function CalcLeftHandDamage(w998)
{
	wBC3L2 = 0;
	for ( i = 4; i <= 7; i++ )
	{
		if ( cardOBJ[n_A_card[i]][0] == 106 )
		{
			wBC3L2 += 5;
		}
	}

	wBC3_Normal = w998 * w_HIT / 100;
	wBC3_Miss = wBC3L2 * (100-w_HIT) / 100;

	wBC3_X = wBC3_Normal + wBC3_Miss;

	wBC3_X = tPlusDamCut( wBC3_X );

	return wBC3_X;
}

function ApplyEnemyDefense( damage, index, wBC4_3 )
{ // calcDef & DefIgnore - (rawAtk, (min,avg,max,crit:=10)dmg , left hand--> upgradeatk)
	if ( n_A_ActiveSkill == skill_HW_STAVE_CRASHER )
	{
		return Math.floor( damage * defReduction( n_B[en_HARDDEF] ) ) - n_B_DEF2[0] + wBC4_3;
	}

	if ( n_A_ActiveSkill == skill_GS_WOUNDING_SHOT )
	{
		return damage + wBC4_3;
	}

	if ( n_tok[bon_IGN_DEF_RC_FORMLESS+n_B[en_RACE]] >= 1 )
	{
		return damage + wBC4_3;
	}

	if ( n_tok[bon_IGN_DEF_NONBOSS] >= 1 && n_B[en_BOSS] == 0 )
	{
		return damage + wBC4_3;
	}

	if ( n_tok[bon_IGN_DEF_NONBOSS] >= 10 )
	{
		return damage + wBC4_3;
	}

	if ( SkillSearch( skill_TKM_UNION ) )
	{
		return damage + wBC4_3;
	}

	if ( n_tok[bon_ICE_PICK] === 0 )
	{ // Player has no Ice Pick, apply defence
		damage = Math.floor( damage * defReduction( n_B[en_HARDDEF] ) ) - n_B[en_SOFTDEF] + wBC4_3;
	}
	else
	{ // Player has an Ice Pick, ignore defence
		damage += wBC4_3;
	}

	damage = Max( 1, damage );

	return damage;
}

function ApplyDamageModifiers( damage )
{ // % Dmg Mod - (startMod=100)
	var dmgMultiplier = 0;

	if ( determiningEDPdamage == 0 )
	{
		if(n_A_ActiveSkill==skill_TH_ENVENOM || n_A_ActiveSkill==skill_HU_FANTASTIC_ARROW)
			damage += 15 * n_A_ActiveSkillLV;
		if(n_A_ActiveSkill==skill_AS_POISON_REACT && (n_B[en_ELEMENT] < 50 ||  60 <= n_B[en_ELEMENT]))
			damage += 75;
	}
	if ( n_A_ActiveSkill==skill_GS_MAGICAL_BULLET )
		damage += Math.floor(n_A_MATK[w_MagiclBulet] * defReduction(n_B[en_HARDMDEF]) - n_B_MDEF2); // MDef
	if ( n_A_ActiveSkill==skill_GS_GUNSLINGER_MINE )
		damage += n_A_ActiveSkillLV * 50;

	// Ninja throwing
	if ( n_A_ActiveSkill === skill_NIN_THROW_DAGGER )
	{
		damage += ShurikenOBJ[eval(document.calcForm.SkillSubNum.value)][0];
		damage += 3 * SkillSearch(skill_NIN_DAGGER_THROWING_PRACTICE);
		damage += 4 * n_A_ActiveSkillLV;
	}
	if ( n_A_ActiveSkill === skill_NIN_THROW_KUNAI )
	{
		damage += KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][0] * 3;
	}

	if ( determiningEDPdamage == 0 && not_use_card == 0 )
	{
		// Crit Bonus
		if ( wCriTyuu == 1 &&
		 	 n_A_ActiveSkill != skill_SN_FOCUSED_ARROW_STRIKE &&
		 	 n_A_ActiveSkill != skill_NIN_SHADOW_SLASH )
		{
			damage = Math.floor( damage * ( 100 + criticalMod ) / 100.0 );
		}

		// What is this?
		damage = Math.floor(damage * (100+StPlusCalc2(1000+n_B[en_ID])+StPlusCard(1000+n_B[en_ID])) /100);

		if ( SkillSearch( skill_LK_FRENZY ) )
		{ // Frenzy Doubles melee damage
			damage = damage * 2;
		}
		if ( n_A_ActiveSkill==skill_AS_POISON_REACT && (50 <= n_B[en_ELEMENT] && n_B[en_ELEMENT] < 60) )
		{
			damage = Math.floor( damage * ( 100 + 30 * n_A_ActiveSkillLV) / 100 );
		}

		if ( n_A_WeaponType == weapTyp_KATAR && SkillSearch( skill_AX_ADVANCED_KATAR_MASTERY ) )
		{ // Advanced Katar Mastery Bonus
			damage = Math.floor(damage * (110 + 2 * SkillSearch(skill_AX_ADVANCED_KATAR_MASTERY))/100);
		}

		// TKM Solar, Lunar, and Stellar Bonuses to damage
		dmgMultiplier = 0;
		if ( PlayerVersusPlayer === 0 )
		{ // PvM
			if ( SkillSearch( skill_TKM_STELLAR_WRATH ) && SkillSearch( skill_TKM_SOLAR_PROTECTION ) )
			{
				dmgMultiplier += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch(skill_TKM_STELLAR_WRATH) * 3 );
			}
			else if ( SkillSearch(skill_TKM_STELLAR_WRATH) && n_B[en_SIZE]==2 && n_B[en_HP] >= 17392)
			{
				dmgMultiplier += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch(skill_TKM_STELLAR_WRATH) * 3 );
			}
			else if ( SkillSearch(skill_TKM_SOLAR_WRATH) && n_B[en_SIZE] == 0 )
			{
				dmgMultiplier += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(skill_TKM_SOLAR_WRATH) * 3 );
			}
			else if ( SkillSearch(skill_TKM_LUNAR_WRATH) && n_B[en_SIZE]==1 && n_B[en_HP] >= 5218 )
			{
				dmgMultiplier += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(skill_TKM_LUNAR_WRATH) * 3 );
			}
		}

		damage = Math.floor( damage * ( 100 + dmgMultiplier ) / 100 );
	}

	//PRIMA ERA QUI, MO SPOSTO!!!
	//damage = Math.floor(tPlusDamCut(damage));

	//damage = ApplySkillModifiers( damage );

	return damage;
}

function ApplySkillAdditions( skillMod )
{ // skillmod + x
	// Power Thrust and Maximum Power Thrust
	if ( SkillSearch( skill_MS_MAXIMUM_POWER_THUST ) )
	{
		skillMod += 20 * SkillSearch( skill_MS_MAXIMUM_POWER_THUST );
	}
	else
	{
		if ( SkillSearch( skill_BS_POWER_THRUST ) )
		{
			skillMod += SkillSearch( skill_BS_POWER_THRUST ) * 5;
		}
		else if ( otherBuffs[ksPowerThrust] )
		{
			skillMod += 5;
		}
	}

	// Spear Dynamo
	if ( SkillSearch( skill_LK_SPEAR_DYNAMO ) )
	{
		skillMod += SkillSearch( skill_LK_SPEAR_DYNAMO ) * 5;
	}

	// Falcon Eyes
	if ( SkillSearch( skill_SN_FALCON_EYES ) )
	{
		skillMod += SkillSearch( skill_SN_FALCON_EYES ) * 2;
	}

	// Kihop
	if ( SkillSearch( skill_TK_KIHOP ) )
	{
		skillMod += 2 * SkillSearch( skill_TK_KIHOP ) * SkillSearch( skill_TK_KIHOP_PARTY );
	}

	// Windmill Rush
	if ( performerBuffs[ksMaestroSolo] === ksWindmillRush && performerBuffs[ksMaestroSoloLevel] > 0 )
	{
		var skillBonus = performerBuffs[ksMaestroSoloLevel] * 6;
		var voiceLessonsBonus = performerBuffs[ksMaestroVoiceLessons];
		var jobLvlBonus = performerBuffs[ksMaestroJobLevel] * 0.2;

		skillMod += skillBonus + voiceLessonsBonus + jobLvlBonus;
	}

	return skillMod;
}

function ApplySkillModifiers( damage )
{
	// Skill Multipliers
	dmgMultiplier = 0;
	if ( n_A_ActiveSkill == skill_SW_BASH )
	{
		if ( n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch( 362 ) )
		{ // Freezer Card gives 10% to bash
			dmgMultiplier += 10;
		}
	}
	if ( n_A_ActiveSkill == skill_KN_BOWLING_BASH )
	{
		if ( n_A_WeaponType == weapTyp_SWORD || n_A_WeaponType == weapTyp_2HSWORD )
		{ // Sword Guardian card bonus
			dmgMultiplier += 25 * CardNumSearch(464);
		}
	}
	if(n_A_ActiveSkill == skill_AR_ARROW_SHOWER)
	{
		if ( n_A_WeaponType == weapTyp_BOW )
		{ // Bow Guardian card bonus
			dmgMultiplier += 50 * CardNumSearch(465);
		}
	}
	if ( n_A_ActiveSkill == skill_AR_DS )
	{
		if ( n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1089) )
		{
			// Glorious Hunter Bow Bonus
			dmgMultiplier += 20;
		}
	}
	if ( n_A_ActiveSkill == skill_GS_TRIGGER_HAPPY_SHOT )
	{
		if ( n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1099) )
		{
			// glorious pistol increases trigger happy shot
			dmgMultiplier += 2 * n_A_Weapon_ATKplus;
		}
	}
	if ( n_A_ActiveSkill == skill_GS_TRACKING )
	{
		if ( n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1100) )
		{
			// glorious rifle
			dmgMultiplier += 3 * n_A_Weapon_ATKplus;
		}
	}
	if ( n_A_ActiveSkill == skill_GS_SPREAD_SHOT )
	{
		if ( n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1102) )
		{
			// glorious shotgun
			dmgMultiplier += 2 * n_A_Weapon_ATKplus;
		}
	}
	if ( n_A_ActiveSkill == skill_GS_GUNSLINGER_MINE )
	{
		if ( n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1103) )
		{
			// glorious grenade launcher
			dmgMultiplier += 2 * n_A_Weapon_ATKplus;
		}
	}
	if ( n_A_ActiveSkill == skill_GS_TRIPLE_ACTION )
	{
		if ( EquipNumSearch( 1100 ) ||
			 EquipNumSearch( 1101 ) ||
			 EquipNumSearch( 1102 ) ||
			 EquipNumSearch( 1103 ) )
		{
			// glorious guns increase triple action by 30%
			dmgMultiplier += 30;
		}
	}
	if ( n_A_ActiveSkill == skill_SW_BASH || n_A_ActiveSkill == skill_KN_BOWLING_BASH )
	{
		if ( n_A_ActiveSkillLV == 10 && EquipNumSearch( 1159 ) )
		{ // Verteran Sword bonus
			dmgMultiplier += 50;
		}
	}
	if ( n_A_ActiveSkill == skill_ME_MAMMONITE )
	{
		if ( SU_LUK >= 90 && SU_DEX >= 90 && EquipNumSearch( 1164 ) )
		{ // Berchel Axe?
			dmgMultiplier += 15;
		}
	}
	if ( n_A_ActiveSkill == skill_AX_METEOR_ASSAULT )
	{
		if ( EquipNumSearch( 1176 ) && SkillSearch( skill_AS_KATAR_MASTERY ) == 10 )
		{ // Chakram
			dmgMultiplier += 20;
		}
	}
	if ( trifectaBlowDamage == -1 && EquipNumSearch( 639 ) )
	{
		// Combo Battle Glove +15% bonus to Trfecta and Quadruple
		dmgMultiplier += 15;
	}
	if ( ( n_A_ActiveSkill == skill_AS_SONIC_BLOW ||
		   n_A_ActiveSkill == skill_AS_SONIC_BLOW_SL ) &&
		 SkillSearch( skill_AS_SONIC_ACCELERATION ) && determiningEDPdamage == 0 )
	{ // Sonic Acceleration bonus to Sonic Blow
		dmgMultiplier += 10;
	}
	if ( n_A_ActiveSkill === skill_MEC_AXE_TORNADO )
	{
		if ( n_A_Weapon_element === ele_WIND )
		{ // does more with wind element weapon
			dmgMultiplier += 25;
		}
	}
	if ( (n_A_JOB == cls_KAGOB) && SkillSearch( skill_KAG_SUMMON_ELEMENTAL_SEAL ) &&
		(n_A_ActiveSkill !== skill_KAG_THROW_EXPLOSIVE_KUNAI &&
		 n_A_ActiveSkill !== skill_KAG_OVERTHROW &&
		 n_A_ActiveSkill !== skill_NIN_THROW_COINS &&
		 n_A_ActiveSkill !== skill_ALL_BASIC_ATTACK))
	{ // Summon Elemental Seals damage multiplier
		if (n_A_Weapon_element == ele_NEUTRAL + SkillSearch( skill_KAG_GET_ELEMENTAL_SEAL ) && SkillSearch( skill_KAG_GET_ELEMENTAL_SEAL ) !== ele_EARTH)
			dmgMultiplier += 10*SkillSearch( skill_KAG_SUMMON_ELEMENTAL_SEAL );
	}

	damage = damage * (100+StPlusCalc2(bon_DMG_SKILL+n_A_ActiveSkill)+StPlusCard(bon_DMG_SKILL+n_A_ActiveSkill) + dmgMultiplier) /100;
	if ( n_A_ActiveSkill == skill_RG_BACK_STAB &&
		 n_A_WeaponType == weapTyp_BOW )
	{
		// Backstab with a bow does half the damage
		damage = Math.floor( damage / 2 );
	}
	if ( n_Nitou && n_A_ActiveSkill === skill_ALL_BASIC_ATTACK )
	{
		if ( n_A_WeaponType !== weapTyp_NONE )
		{ // dual handed with a weapon in both hands
			if (n_A_JOB == cls_KAGOB)
				damage = Math.floor(damage * (70 + SkillSearch(skill_AS_RIGHTHAND_MASTERY) *10) /100);
			else
				damage = Math.floor(damage * (50 + SkillSearch(skill_AS_RIGHTHAND_MASTERY) *10) /100);
		}
	}
	if ( n_A_ActiveSkill == skill_GS_MAGICAL_BULLET )
	{ // Magical Bullet is forced ghost
		damage = damage * element[n_B[en_ELEMENT]][ele_GHOST] / 100;
	}
	if ( n_A_ActiveSkill == skill_GS_GUNSLINGER_MINE )
	{ // gunslinger mine is forced neutral
		damage = damage * element[n_B[en_ELEMENT]][ele_NEUTRAL] / 100;
	}
	if ( performerBuffs[ksWandererSolo] === ksGloomyShynessW && performerBuffs[ksWandererSoloLevel] > 0 )
	{ // Gloomy Shyness
		var maxPercentage = ( performerBuffs[ksWandererVoiceLessons] * 5 ) + ( performerBuffs[ksWandererSoloLevel] * 10 );
		if ( maxPercentage < 15 )
		{
			maxPercentage = 15;
		}
		var randomNumber = Math.floor( Math.random() * ( maxPercentage - 14 ) ) + 15;
		gloomyMultiplier = randomNumber / 100.0;
		gloomyMultiplier += 1.0;

		if ( n_A_ActiveSkill == skill_KN_BRANDISH_SPEAR   ||
		   	 n_A_ActiveSkill == skill_CR_SHIELD_BOOMERANG ||
		   	 n_A_ActiveSkill == skill_LK_CLASHING_SPIRAL  ||
		   	 n_A_ActiveSkill == skill_PA_RAPID_SMITING    ||
		   	 n_A_ActiveSkill == skill_RUN_HUNDRED_SPEAR   ||
		   	 n_A_ActiveSkill == skill_ROY_SHIELD_PRESS    ||
		   	 n_A_ActiveSkill == skill_CR_SMITE )
		{
			damage *= gloomyMultiplier;
			damage = Math.floor(damage);
		}
	}
	else if ( performerBuffs[ksMaestroSolo] === ksGloomyShynessM && performerBuffs[ksMaestroSoloLevel] > 0 )
	{ // Gloomy Shyness
		var maxPercentage = ( performerBuffs[ksMaestroVoiceLessons] * 5 ) + ( performerBuffs[ksMaestroSoloLevel] * 10 );
		if ( maxPercentage < 15 )
		{
			maxPercentage = 15;
		}
		var randomNumber = Math.floor( Math.random() * ( maxPercentage - 14 ) ) + 15;
		gloomyMultiplier = randomNumber / 100.0;
		gloomyMultiplier += 1.0;

		if ( n_A_ActiveSkill == skill_KN_BRANDISH_SPEAR   ||
		   	 n_A_ActiveSkill == skill_CR_SHIELD_BOOMERANG ||
		   	 n_A_ActiveSkill == skill_LK_CLASHING_SPIRAL  ||
		   	 n_A_ActiveSkill == skill_PA_RAPID_SMITING    ||
		   	 n_A_ActiveSkill == skill_RUN_HUNDRED_SPEAR   ||
		   	 n_A_ActiveSkill == skill_ROY_SHIELD_PRESS    ||
		   	 n_A_ActiveSkill == skill_CR_SMITE )
		{
			damage *= gloomyMultiplier;
			damage = Math.floor(damage);
		}
	}

	return damage;
}

function ApplyMagnumBreakBonus()
{
/*
	// version 1
	if ( SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) || otherBuffs[ksMagnumBreak] )
	{ // EDP or Magnum Break
		w_DMG[2] += EDP_DMG( 2 ) * HitNum;
		w_DMG[1] += EDP_DMG( 1 ) * HitNum;

		if ( w_HIT_EDP == 100 )
		{
			w_DMG[0] += EDP_DMG(0) * HitNum;
		}

		EDPhyouzi( HitNum );
	}

	// version 2
	if(SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) || otherBuffs[ksMagnumBreak])
	{
		var wE = 0;
		if(w_HIT_HYOUJI == 100)
			wE = 1;
		if(n_PerHIT_DMG)
			wE = 1;
		str_bSUBname += "MB/EDP Part chase(?)<BR>";
		var w0 = n_A_EDP_DMG[0] * HitNum;
		var w2 = n_A_EDP_DMG[2] * HitNum;
		if(wE)
			str_bSUB += w0 +"~"+ w2 +"( add invocation rate(?)"+ w_HIT_EDP +"%)<BR>";
		else
			str_bSUB += w0 +"~"+ w2 +"( add invocation rate(?)"+ (Math.floor(w_HIT * w_HIT_EDP) / 100) +"%)<BR>";
	}

	// version 3
	if (SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) || otherBuffs[ksMagnumBreak])
	{
		if(n_A_ActiveSkill == 17 && 52 <= n_B[en_ELEMENT] && n_B[en_ELEMENT] <= 59)
			return 0;
		if ( ( n_A_ActiveSkill == 66 ||
			   n_A_ActiveSkill == 193 ||
			   n_A_ActiveSkill == 197 ||
			   n_A_ActiveSkill == 321 ) &&
			 83 <= n_B[en_ELEMENT] && n_B[en_ELEMENT] <= 89 )
		{
			return 0;
		}
		if(element[n_B[en_ELEMENT]][n_A_Weapon_element] <= 0 && n_PerHIT_DMG == 0)
		{
			return 0;
		}

		if ( num == 0 )
		{
			if(w_HIT_EDP == 100)
				return n_A_EDP_DMG[0];
			else
				return 0;
		}
		else if ( num == 1 )
		{
			var wE = 0;
			if(w_HIT_HYOUJI == 100)
				wE = 1;
			if(n_PerHIT_DMG)
				wE = 1;
			if(wE)
				return Math.floor(n_A_EDP_DMG[1] * w_HIT_EDP / 100);
			else
				return Math.floor(n_A_EDP_DMG[1] * w_HIT / 100 * w_HIT_EDP / 100);
		}
		else if ( num == 2 )
		{
			return n_A_EDP_DMG[2];
		}
	}
	return 0;

	// version 4
	if ( otherBuffs[ksMagnumBreak] )
	{ // Magnum Break
		y = CalcFinalDamage(wBCEDP,wBCEDP2);
		y = Math.floor( ( y * element[n_B[en_ELEMENT]][ele_FIRE]) / 5 );
	}
*/
}

function tPlusDamCut( damage )
{ // some modifiers
	if ( PlayerVersusPlayer == 0 )
	{
		if ( battleEffects[pass_VIII_SPE_ENVIRONMENT] == 1 )
		{ // WoE zone?
			if ( n_A_WeaponType==weapTyp_BOW ||
				 n_A_WeaponType==weapTyp_HANDGUN ||
				 n_A_WeaponType==weapTyp_RIFLE ||
				 n_A_WeaponType==weapTyp_SHOTGUN ||
				 n_A_WeaponType==weapTyp_GATLING_GUN ||
				 n_A_WeaponType==weapTyp_GRENADE_LAUNCHER )
			{
				damage = Math.floor(damage * 0.6);
			}
			else if ( n_A_ActiveSkill != skill_ALL_BASIC_ATTACK )
			{
				damage = Math.floor(damage * 0.6);
			}
			else
			{
				damage = Math.floor(damage * 0.8);
			}

			if ( battleEffects[pass_VIII_DEF_INVEST] )
			{ // Defense Investment
				damage = Math.floor(damage * (10 / (battleEffects[pass_VIII_DEF_INVEST] * 5)));
			}
		}
	}

	if ( hunterPetHits === 0 )
	{
		// Monster Debuffs
		if ( monsterDebuffs[status_en_LEXA] && wLAch === 0 )
		{ // Lex Aeterna
			damage *= 2;
		}
		if (monsterDebuffs[status_en_DARK_CLAW] > 0 && n_B[en_BOSS] == 0 &&
			(n_A_WeaponType != weapTyp_BOW &&
			n_A_WeaponType != weapTyp_INSTRU &&
			n_A_WeaponType != weapTyp_WHIP &&
			n_A_WeaponType != weapTyp_HANDGUN &&
			n_A_WeaponType != weapTyp_RIFLE &&
			n_A_WeaponType != weapTyp_SHOTGUN &&
			n_A_WeaponType != weapTyp_GATLING_GUN &&
			n_A_WeaponType != weapTyp_GRENADE_LAUNCHER)) {
		    damage *= 1 + (0.3 * monsterDebuffs[status_en_DARK_CLAW]);
		}
		if ( monsterDebuffs[status_en_FIBER] && n_A_Weapon_element === ele_FIRE )
		{ // Fiberlock
			damage *= 2;
		}
		if ( monsterDebuffs[status_en_DEEPSLEEP] )
		{ // Deep Sleep
			damage = Math.floor( damage * 1.5 );
		}
		if ( monsterDebuffs[status_en_VENOM_IMPRESS] && n_A_Weapon_element === ele_POISON )
		{ // Venom Impress
			damage *= 1.0 + 0.1 * monsterDebuffs[status_en_VENOM_IMPRESS];
		}

		// damage increased by land enchants
		multipliers = [110,114,117,119,120];
		if ( otherBuffs[ksElementField] === 0 && otherBuffs[ksElementFieldLvl] >= 1 && n_A_Weapon_element === ele_FIRE )
		{ // Volcano
			damage = Math.floor( damage * multipliers[otherBuffs[ksElementFieldLvl] - 1] / 100 );
		}
		if ( otherBuffs[ksElementField] === 1 && otherBuffs[ksElementFieldLvl] >= 1 && n_A_Weapon_element === ele_WATER )
		{ // Deluge
			damage = Math.floor( damage * multipliers[otherBuffs[ksElementFieldLvl] - 1] / 100 );
		}
		if ( otherBuffs[ksElementField] === 2 && otherBuffs[ksElementFieldLvl] >= 1 && n_A_Weapon_element === ele_WIND )
		{ // Whirlwind
			damage = Math.floor( damage * multipliers[otherBuffs[ksElementFieldLvl] - 1] / 100 );
		}
	}

	// Monster Buffs
	/*if ( monsterBuffs[status_en_buff_Assumptio] && PlayerVersusPlayer === 0 )
	{ // Assumptio
		damage = Math.floor( damage / 2 );
	}
	if ( monsterBuffs[status_en_buff_Assumptio] && PlayerVersusPlayer === 1 )
	{ // Assumptio
		damage = Math.floor( damage * 2 / 3 );
	}*/
	if ( monsterBuffs[status_en_buff_StoneSkin] && damageType !== kDmgTypeMagic )
	{ // Stone Skin
		damage -= Math.floor( damage * 20 * monsterBuffs[status_en_buff_StoneSkin] / 100 );
	}
	if ( monsterBuffs[status_en_buff_MagicMirror] && damageType === kDmgTypeRanged )
	{ // Anti Magic
		damage -= Math.floor( PDC * 20 * monsterBuffs[status_en_buff_MagicMirror] / 100 );
	}
	if ( monsterBuffs[status_en_buff_Race]  && damageType === kDmgTypeMagic ) {
		damage -= Math.floor( damage * monsterBuffs[status_en_buff_Race] / 100 );
	}
	if ( monsterBuffs[status_en_buff_Elemental]  && damageType === kDmgTypeMagic ) {
		damage -= Math.floor( damage * monsterBuffs[status_en_buff_Elemental] / 100 );
	}
	if ( monsterBuffs[status_en_buff_Size]  && damageType === kDmgTypeMagic ) {
		damage -= Math.floor( damage * monsterBuffs[status_en_buff_Size] / 100 );
	}
	if ( monsterBuffs[status_en_buff_Other] ) {
		damage -= Math.floor( damage * monsterBuffs[status_en_buff_Other] / 100 );
	}
	if ( n_B[en_BOSS] === 5 )
	{
		damage = 1;
	}

	return damage;
}

function DisplayAdditionalBattleInfo()
{ // Display lower battle results

	// display FLEE information
	w_FLEE = 100 - ( n_B_HIT - n_A_FLEE );
	w_FLEE = Between( 0, w_FLEE, 95 );
	myInnerHtml( "BattleFLEE", Math.floor( ( w_FLEE + ( 100 - w_FLEE ) * n_A_LUCKY / 100 ) * 100 ) / 100 + SubName[0][Language], 0 );

	// cast time and delay info
	DisplayCastAndDelay();

	if ( n_PerHIT_DMG > 0 && w_HIT_HYOUJI < 100 )
	{
		str_bSUBname += "Damage When Missing";
		if(str_PerHIT_DMG == 0)
			str_bSUB += n_PerHIT_DMG;
		else
			str_bSUB += "<br/>" + str_PerHIT_DMG;
	}
	myInnerHtml( "bSUBname", str_bSUBname, 0 );
	myInnerHtml( "bSUB", str_bSUB, 0 );
	myInnerHtml( "BattleHIT", w_HIT_HYOUJI + SubName[0][Language], 0 );

	if ( n_B[en_ID] == 44 && n_A_ActiveSkill != skill_ALL_BASIC_ATTACK && n_A_ActiveSkill != 325 )
	{ // Emp
		for ( var i = 0; i <= 2; i++ )
		{
			w_DMG[i] = 0;
			myInnerHtml( "ATK_0" + i, 0, 0 );
		}
	}

	// Min Number of Hits ---------------------------------
	var minNumHits;
	minNumHits = Math.floor(n_B[en_HP] / w_DMG[2]);
	if(n_B[en_HP] % Math.floor(w_DMG[2]) != 0)
		minNumHits += 1;
	if ( minNumHits <10000)
		myInnerHtml("MinATKnum",minNumHits,0);
	else
		myInnerHtml("MinATKnum",SubName[5][Language],0);


	if(SG_Special_HITnum != 0)
	{
		if ( minNumHits == 1 )
		{
			var wHITnum;
			var x;
			wHITnum = SG_Special_HITnum;
			x = (SG_Special_DMG[2] * w_TotalHits - n_B[en_HP]) / (SG_Special_DMG[2] * w_TotalHits - SG_Special_DMG[0] * w_TotalHits);
			x = Between(0,x,1);
			if(wHITnum == 2)
			{
				if(x < 0.5)
					x = 2 * x * x;
				else
					x = 1 - 2 * (1-x) * (1-x);
			}
			if(wHITnum == 3)
			{
				if(x < (1/3))
					x = 4.5 * Math.pow(x,3);
				else if((1/3) <= x && x < (2/3))
					x = 4.5 * (Math.pow(x,3) - 3 * Math.pow(x-1/3,3));
				else if((2/3) <= x)
					x = 1 - 4.5 * Math.pow(1-x,3);
			}
			if(wHITnum >= 4)
			{
				var y = Math.sqrt(Math.pow(SG_Special_DMG[2]-SG_Special_DMG[0],2) / 12 * wHITnum);
				x = (SG_Special_DMG[1] * w_TotalHits - n_B[en_HP]) / y;
				if(x >= 0)
					x = 0.5+0.5*Math.sqrt(1-Math.exp(-2*Math.pow(x,2)/Math.PI));
				else
					x = 0.5-0.5*Math.sqrt(1-Math.exp(-2*Math.pow(x,2)/Math.PI));
			}
			x = Math.floor(x * 10000) / 100;
			myInnerHtml("MinATKnum","1 ("+ x +"% Chance)",0);
		}
		SG_Special_HITnum = 0;
	}

	// Max Number of Hits ------------------------------------
	if(w_HIT_HYOUJI < 100 && n_PerHIT_DMG == 0)
	{
		myInnerHtml( "MaxATKnum","<Font size=2>Infinite (no 100% Hit)</font>",0);
	}
	else
	{
		var wX = w_DMG[0];
		if(w_HIT_HYOUJI < 100)
			wX = n_PerHIT_DMG;
		minNumHits = Math.floor(n_B[en_HP] / wX);
		if(n_B[en_HP] % Math.floor(wX) != 0)
			minNumHits += 1;
		if(minNumHits<10000)
			myInnerHtml("MaxATKnum", minNumHits, 0 );
		else
			myInnerHtml("MaxATKnum",SubName[5][Language],0);
	}

	minNumHits = Math.floor( n_B[en_HP] / w_DMG[1] );
	if ( n_B[en_HP] % w_DMG[1] != 0 )
	{
		minNumHits += 1;
	}

	// Experience earned
	if ( minNumHits < 10000 )
	{
		myInnerHtml("AtkBaseExp",Math.round(n_B[en_BASEEXP]*expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL]) / minNumHits) +"Exp",0);
		myInnerHtml("AtkJobExp",Math.round(n_B[en_BASEEXP]*expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL]) / minNumHits) +"Exp",0);
	}
	else
	{
		myInnerHtml("AtkBaseExp",SubName[7][Language],0);
		myInnerHtml("AtkJobExp",SubName[7][Language],0);
	}

	// Battle Duration
	if ( minNumHits < 10000 )
	{
		myInnerHtml( "AveATKnum", minNumHits, 0 );

		n_AveATKnum = minNumHits;

		var battleDuration = (totalCastTime + totalDelay) * n_AveATKnum;
		if ( n_AveATKnum === 1 )
		{
			battleDuration -= totalDelay;
		}
		battleDuration = Math.floor( battleDuration * 100 ) / 100;

		if ( n_Delay[0] )
		{
			myInnerHtml( "BattleTime", "Special", 0 );
		}
		else
		{
			myInnerHtml( "BattleTime", battleDuration + "s", 0 );
		}
	}
	else
	{
		myInnerHtml( "AveATKnum", SubName[5][Language], 0 );
		myInnerHtml( "BattleTime", SubName[6][Language], 0 );
	}

	var w = 1 / (totalCastTime + totalDelay) * w_DMG[1];
	w *= 100;
	w = Math.round(w);
	w /= 100;

	if ( n_Delay[0] )
	{
		myInnerHtml( "AveSecondATK", "Special", 0 );
	}
	else
	{
		myInnerHtml( "AveSecondATK", w, 0 );
	}

	// Damage taken
	if ( PlayerVersusPlayer == 0 )
	{
		calcIncomingDamage();
	}

	// Fill in the battle results
	for ( var i = 0; i < InnStr.length; i++ )
	{
		myInnerHtml( "strID_" + i, InnStr[i], 0 );
	}
}

function DisplayCastAndDelay()
{ // Cast times and delays
	// print cast time

	totalCastTime = fixedCastTime + variableCastTime;
	if ( totalCastTime != 0 )
	{
		str_bSUBname += SubName[9][Language] +"</br>";
		str_bSUB += fixedCastTime.toFixed(1) + " " + SubName[1][Language] + " (fixed) + ";
		str_bSUB += variableCastTime.toFixed(1) + " " + SubName[1][Language] + " (var)";
	}
	var strSUB2name = "";
	var strSUB2 = "";

	// calculate longest delay
	totalDelay = 0;

	if ( SkillSearch( skill_WAR_READING_SPELLBOOK ) )
	{
		// instant list
		var w2 = [51,54,56,57,125,126,127,128,131,132,133,534,540,542,545,547,553];
		if ( NumSearch( n_A_ActiveSkill, w2 ) )
		{
			n_Delay[ksDelayCooldown] = 0;
		}
	}
	var longestDelay = 0;
	if ( n_Delay[ksDelayASPD] > totalDelay )
	{
		totalDelay = n_Delay[ksDelayASPD];
		longestDelay = ksDelayASPD;
	}
	n_Delay[ksDelayGlobal] = Math.floor( n_Delay[ksDelayGlobal] * ( 100 - globalCastDelay ) ) / 100;
	if ( n_Delay[ksDelayGlobal] > totalDelay )
	{
		totalDelay = n_Delay[ksDelayGlobal];
		longestDelay = ksDelayGlobal;
	}
	if ( n_Delay[ksDelayAnimation] > totalDelay )
	{
		totalDelay = n_Delay[ksDelayAnimation];
		longestDelay = ksDelayAnimation;
	}
	if ( n_A_ActiveSkill != skill_ALL_BASIC_ATTACK && n_A_ActiveSkill != skill_PA_MARTYR_RECONING )
	{
		n_Delay[ksDelayE] = parseInt(formElements["Conf01"].value) / 100;
	}
	if ( n_Delay[ksDelayE] > ( totalDelay + totalCastTime ) ) //Check here later
	{
		totalDelay = n_Delay[ksDelayE] - totalCastTime;
		longestDelay = ksDelayE;
	}
	if ( n_Delay[ksDelayF] != 0 )
	{
		totalDelay = n_Delay[ksDelayF];
		longestDelay = ksDelayF;
	}
	if ( n_Delay[ksDelaySkillDuration] > totalDelay )
	{
		totalDelay = n_Delay[ksDelaySkillDuration] - totalCastTime;
		longestDelay = ksDelaySkillDuration;
	}
	if ( n_Delay[ksDelayCooldown] > totalDelay )
	{
		totalDelay = n_Delay[ksDelayCooldown];
		longestDelay = ksDelayCooldown;
	}

	// print delay info
	if ( longestDelay == ksDelayA )
	{
	}
	else if ( longestDelay == ksDelayASPD )
	{ // ASPD Delay
		if ( n_A_ActiveSkill === skill_ALL_BASIC_ATTACK )
		{
			if ( SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW ) )
			{ // Raging Trifecta Blow
				strSUB2name += "Attack interval (normal)</br>Attack Interval (Raging Trifecta Blow)</br>";
				strSUB2 += n_Delay[ksDelayASPD] + "s<BR>" + sandanDelay + "s<BR>";
				totalDelay = n_Delay[ksDelayASPD] * meleeChanceAfterTrifecta / 100 + sandanDelay * trifectaBlowActivationRate / 100;
			}
			else
			{ // Normal Hits
				strSUB2name += "Time/Hit<BR>";
				strSUB2 += n_Delay[ksDelayASPD].toFixed(1) + " sec</br>";
			}
		}
		else
		{ // skill with ASPD delay
			strSUB2name += "<font size=2>Delay (ASPD Based)</font></br>";
			strSUB2 += n_Delay[ksDelayASPD].toFixed(1) + " sec</br>";
		}
	}
	else if ( longestDelay == ksDelayGlobal )
	{ // global after-cast delay
		strSUB2name += "<Font size=2>Delay (Global)</font></br>";
		strSUB2 += n_Delay[ksDelayGlobal].toFixed(1) + " sec</br>";
	}
	else if ( longestDelay == ksDelayAnimation )
	{
		if ( n_A_ActiveSkill == skill_MO_RAGING_QUADRUPLE_BLOW ||
			 n_A_ActiveSkill == skill_MO_RAGING_THRUST ||
			 n_A_ActiveSkill == skill_CH_GLACIER_FIST )
		{
			strSUB2name += "<font size=2>Delay (+delay reception combo)</font></br>";
			strSUB2 += n_Delay[ksDelayAnimation].toFixed(1) +"~"+ (n_Delay[ksDelayAnimation].toFixed(1) + 0.3) + " sec</br>";
		}
		else
		{
			strSUB2name += "<font size=2>Delay (Forced Motion)</font></br>";
			strSUB2 += n_Delay[ksDelayAnimation].toFixed(1) + " sec</br>";
		}
	}
	else if ( longestDelay == ksDelayE )
	{
		strSUB2name += "<Font size=2>Delay (Input Limit)</font></br>";
		strSUB2 += n_Delay[ksDelayE].toFixed(1) + " sec</br>";
	}
	else if ( longestDelay == ksDelayF )
	{
		strSUB2name += "<Font size=2>Damage Interval</font></br>";
		strSUB2 += n_Delay[ksDelayF].toFixed(1) + " sec</br>";
	}
	else if ( longestDelay == ksDelaySkillDuration )
	{
		strSUB2name += "<font size=2>Delay (Skill-Duration)</font></br>";
		strSUB2 += n_Delay[ksDelaySkillDuration].toFixed(1) + " sec</br>";
	}
	else if ( longestDelay == ksDelayCooldown )
	{ // skill cooldown
		strSUB2name += "<font size=2>Delay (Skill Cooldown)</font></br>";
		strSUB2 += n_Delay[ksDelayCooldown].toFixed(1) + " sec</br>";
	}

	myInnerHtml("bSUB2name",strSUB2name,0);
	myInnerHtml("bSUB2",strSUB2,0);
}

function DisplayCriticalDamage()
{
	myInnerHtml("CRIATKname",SubName[3][Language],0);
	myInnerHtml("CRInumname",SubName[4][Language],0);

	var wk = [0,0,0];
	if ( n_A_WeaponType === weapTyp_KATAR )
	{
		for ( var i = 0; i <= 2; i++ )
		{
			wk[i] = Math.floor(n_A_CriATK[i] * (0.01 + SkillSearch(skill_TH_DOUBLE_ATTACK) * 0.02));
			n_A_CriATK[i] += wk[i];
		}
		if ( n_A_CriATK[0] === n_A_CriATK[2] )
		{
			myInnerHtml("CRIATK",n_A_CriATK[0] +" ("+ (n_A_CriATK[0] - wk[0]) +" + "+ wk[0] +")",0);
		}
		else
		{
			var str = n_A_CriATK[0] + "~" + n_A_CriATK[2];
			str += " (" + ( n_A_CriATK[0] - wk[0] ) + "~";
			str += ( n_A_CriATK[2] - wk[2] ) + " + ";
			str += wk[0] + "~" + wk[2] + ")";
			myInnerHtml( "CRIATK", str,0);
		}
	}
	else
	{
		if ( n_A_CriATK[0] === n_A_CriATK[2] )
		{
			myInnerHtml( "CRIATK", n_A_CriATK[1], 0 );
		}
		else
		{
			myInnerHtml( "CRIATK", n_A_CriATK[0] + "~" + n_A_CriATK[2], 0 );
		}
	}

	myInnerHtml( "CRInum", ( Math.round( w998G * 100 ) / 100 ) + SubName[0][Language], 0 );
}
