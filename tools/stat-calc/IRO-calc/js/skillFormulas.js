var RANGED_SKILLS = [
    skill_SN_FOCUSED_ARROW_STRIKE,
    skill_AR_ARROW_SHOWER,
    skill_KN_SPEAR_STAB,
    skill_KN_SPEAR_BOOMERANG,
    skill_BA_MELODY_STRIKE,
    skill_DA_SLINGING_ARROW,
    skill_LK_TRAUMATIC_BLOW,
    skill_LK_VITAL_STRIKE,
    skill_CG_ARROW_VULCAN,
    skill_ALL_TOMAHAWK_THROWING,
    skill_AS_VENOM_KNIFE,
    skill_HU_FANTASTIC_ARROW,
    skill_GS_BULLS_EYE,
    skill_GS_MAGICAL_BULLET,
    skill_GS_TRIGGER_HAPPY_SHOT,
    skill_GS_TRACKING,
    skill_GS_DISARM,
    skill_GS_WOUNDING_SHOT,
    skill_GS_CROWD_CONTROL_SHOT,
    skill_GS_FULL_BLAST,
    skill_GS_WOUNDING_SHOT,
    skill_GS_CROWD_CONTROL_SHOT,
    skill_GS_FULL_BLAST,
    skill_GS_SPREAD_SHOT,
    skill_GS_GUNSLINGER_MINE,
    skill_MIWA_GREAT_ECHO,
    skill_RUN_HUNDRED_SPEAR,
    skill_GEN_CART_CANNON,
    skill_SUR_RAMPAGE_BLASTER,
    skill_SUR_KNUCKLE_ARROW,
    skill_SUR_LIGHTENING_RIDE,
    skill_HW_STAVE_CRASHER,
    skill_AR_DOUBLE_STRAFE,
    skill_MO_THROW_SPIRIT_SPHERES,
    skill_GS_TRIPLE_ACTION,
    skill_HU_BEAST_STRAFING,
    skill_HU_BLITZ_BEAT,
    skill_SN_FALCON_ASSAULT,
    skill_RAN_WARG_DASH,
    skill_RAN_WARG_BITE,
    skill_RAN_WARG_STRIKE,
    skill_CR_SHIELD_BOOMERANG,
    skill_CR_SHIELD_BOOMERANG_SL,
    skill_PA_RAPID_SMITING,
    skill_LK_CLASHING_SPIRAL,
    skill_AX_SOUL_DESTROYER,
    skill_NIN_THROW_DAGGER,
    skill_NIN_THROW_KUNAI,
    skill_NIN_THROW_HUUMA_SHURIKEN,
    skill_NIN_KILLING_STRIKE,
    skill_NIN_KILLING_STRIKE_MAX,
    skill_AL_ACID_TERROR,
    skill_BC_ACID_DEMONSTRATION,
    skill_SHA_TRIANGLE_SHOT,
    skill_KAG_CROSS_STRIKE,
    skill_KAG_HUUMA_SHURIKEN_STRIKE,
    skill_GEN_HALLUCINATION_DRUG,
    skill_RUN_DRAGON_BREATH
];
var MAGICAL_SKILLS = [
    skill_MON_DARK_STRIKE,
    skill_MA_FIRE_BOLT,
    skill_MA_COLD_BOLT,
    skill_MA_LIGHTNING_BOLT,
    skill_MA_FIRE_BALL,
    skill_MA_FIRE_WALL,
    skill_MA_FROST_DIVER,
    skill_MA_THUNDER_STORM,
    skill_MA_NAPALM_BEAT,
    skill_MA_SOUL_STRIKE,
    skill_WI_FIRE_PILLAR,
    skill_WI_SIGHTRASHER,
    skill_WI_METEOR_STORM,
    skill_WI_JUPITEL_THUNDER,
    skill_WI_LORD_OF_VERMILLION,
    skill_WI_WATER_BALL,
    skill_WATER_BALL2,
    skill_WI_FROST_NOVA,
    skill_WI_STORM_GUST,
    skill_WI_EARTH_SPIKE,
    skill_WI_HEAVENS_DRIVE,
    skill_HEAVENS_DRIVE2,
    skill_HW_NAPALM_VULCAN,
    skill_AC_HOLY_LIGHT,
    skill_PR_HOLY_LIGHT_SL,
    skill_PR_MAGNUS_EXORCISMUS,
    skill_SL_ESTIN,
    skill_SL_ESTUN,
    skill_SL_ESMA,
    skill_NIN_FLAMING_PETALS,
    skill_NIN_BLAZE_SHIELD,
    skill_NIN_EXPLODING_DRAGON,
    skill_NIN_FREEZING_SPEAR,
    skill_NIN_SNOW_FLAKE_DRAFT,
    skill_NIN_WIND_BLADE,
    skill_NIN_LIGHTNING_JOLT,
    skill_NIN_FIRST_WIND,
    skill_WAR_DRAIN_LIFE,
    skill_WAR_SOUL_EXPANSION,
    skill_WAR_CRIMSON_ROCK,
    skill_WAR_COMET,
    skill_WAR_FROST_MISTY,
    skill_WAR_JACK_FROST,
    skill_WAR_CHAIN_LIGHTNING,
    skill_WAR_EARTH_STRAIN,
    skill_WAR_TETRA_VORTEX,
    skill_ABI_JUDEX,
    skill_ABI_ADORAMUS,
    skill_SOR_FIRE_WALK,
    skill_SOR_ELECTRIC_WALK,
    skill_SOR_EARTH_GRAVE,
    skill_SOR_DIAMOND_DUST,
    skill_SOR_CLOUD_KILL,
    skill_SOR_POISON_BUSTER,
    skill_SOR_PSYCHIC_WAVE,
    skill_SOR_SPELL_FIST_FBOLT,
    skill_SOR_SPELL_FIST_CBOLT,
    skill_SOR_SPELL_FIST_LBOLT,
    skill_MIWA_METALLIC_SOUND,
    skill_ROY_SHIELD_SPELL_MATK,
    skill_AC_HEAL,
    skill_PR_SANCTUARY,
    skill_PR_TURN_UNDEAD,
    skill_PR_RESSURECTION,
    skill_HW_GRAVITY_FIELD
];
function clashingATKFormula(weight)
{
	CalcAtk();

	// for damage calculation
	var tempAttack = (weight + equipmentAttack + n_A_Weapon_ATK + weaponUpgradeAttack + overrefineAttack + strengthBonusAttack) * element[n_B[en_ELEMENT]][n_A_Weapon_element] / 100 * 0.8;
	// multiply race mod
	tempAttack *= ( racialMod + 100 ) / 100;
	// multiply special race mod
	tempAttack *= ( specialRacialMod + 100 ) / 100;
	// multiply size mod
	tempAttack *= ( sizeMod + 100 ) / 100;
	// multiply boss mod
	tempAttack *= ( bossMod + 100 ) / 100;
	// multiply attack mod
	tempAttack *= attackMod;
	tempAttack += statusAttack * 2 * element[n_B[en_ELEMENT]][ele_NEUTRAL] / 100;
	tempAttack = (Max(tempAttack,0));
	return tempAttack;
}
function CalcSkillDamageType() {
	if (  n_A_WeaponType === weapTyp_BOW 		  ||
		   n_A_WeaponType === weapTyp_HANDGUN     ||
	 	   n_A_WeaponType === weapTyp_RIFLE       ||
	 	   n_A_WeaponType === weapTyp_SHOTGUN     ||
	 	   n_A_WeaponType === weapTyp_GATLING_GUN ||
	 	   n_A_WeaponType === weapTyp_GRENADE_LAUNCHER )
	{ // Ranged Attack
		damageType = kDmgTypeRanged;
	}
	else
	{ // Melee Attack
		damageType = kDmgTypeMelee;
	}
	if (RANGED_SKILLS.indexOf(n_A_ActiveSkill) != -1) {
		damageType = kDmgTypeRanged;
	}
	if (MAGICAL_SKILLS.indexOf(n_A_ActiveSkill) != -1) {
		damageType = kDmgTypeMagic;
	}
	if ( n_A_ActiveSkill === skill_GS_DESPERADO )
		damageType = kDmgTypeMelee;
	if ( n_A_ActiveSkill === skill_SUR_GATE_OF_HELL )
	{ // Gate of Hell
		if ( n_A_ActiveSkillLV > 4 )
			damageType = kDmgTypeRanged;
		else
			damageType = kDmgTypeMelee;
	}


}
function CalcSkillDamage()
{
	// Ready!!!
	w_SkillMod = 1; // SkillMod
	w_VarCast = 0.8;
	w_FixCast = 0.2;
	w_TotalHits = 1; // HitCount
	wLAch = 0;
	w_DMG = [0,0,0];
	not_use_card = 0;
	str_PerHIT_DMG = 0;
	SG_Special_ch = 0;
	str_bSUBname = "";
	str_bSUB = "";
	n_Max_DMG = 0;
	n_Min_DMG = 9999999;
	myInnerHtml("CRIATK","",0);
	myInnerHtml("CRInum","",0);
	myInnerHtml("CRIATKname","",0);
	myInnerHtml("CRInumname","",0);
	n_PerHIT_DMG = ApplyDamageModifiers(0); // NeverMissDmg
	dupleLightMagicalDamage = 0;
	dupleLightPhysicalDamage = 0;
	for ( var i = 0; i < 3; i++ )
	{
		Last_DMG_A[i] = 0;
		Last_DMG_B[i] = 0;
	}
	if (  n_A_WeaponType === weapTyp_BOW 		  ||
		   n_A_WeaponType === weapTyp_HANDGUN     ||
	 	   n_A_WeaponType === weapTyp_RIFLE       ||
	 	   n_A_WeaponType === weapTyp_SHOTGUN     ||
	 	   n_A_WeaponType === weapTyp_GATLING_GUN ||
	 	   n_A_WeaponType === weapTyp_GRENADE_LAUNCHER )
	{ // Ranged Attack
		damageType = kDmgTypeRanged;
	}
	else
	{ // Melee Attack
		damageType = kDmgTypeMelee;
	}

	// Fight!!!
	if ( n_A_ActiveSkill == skill_ALL_BASIC_ATTACK ||
		 ( n_A_ActiveSkill == skill_AS_POISON_REACT &&
		   ( n_B[en_ELEMENT] >= ele_POISON * 10 &&
		   	 n_B[en_ELEMENT] < ele_HOLY * 10 ) ) )
	{
		// Get other ATK Mods and apply to crit.
		CalcAtkMods02( w_SkillMod, 1 );

		// there is no fixed or variable cast time for basic attacks.
		fixedCastTime = 0;
		variableCastTime = 0;

		// Crit is a guarantee, calc it.
		wCriTyuu=1;
		for ( var i = 0; i < 3; i++ )
		{
			n_A_CriATK[i] = CalcFinalDamage( n_A_CriATK[i], 10 );
		}
		wCriTyuu=0;

		// Now display it.
		DisplayCriticalDamage();

		if ( w998G > 0 )
		{ // Normal Attack, Crit Damage is the highest possible damage.
			n_Min_DMG = n_A_CriATK[0];
			n_Max_DMG = n_A_CriATK[2];
		}

		if ( n_A_ActiveSkill === skill_AS_POISON_REACT )
		{ // Poison React gets a 1 second delay.
			n_Delay[ksDelayA] = 1;
		}

		if ( n_Nitou )
		{ // dual handed
			// Local Variables
			var lefthandAttack = new Array();
			trifectaBlowDamage = 0;
			// Get other ATK Mods and apply to working damage.
			CalcAtkMods02( w_SkillMod, 0 );
			lefthandAttack = GetOffhandDmg( n_A_Weapon_element );

			for ( var i = 0; i < 3; i++ )
			{
				lefthandAttack[i] = ApplyEnemyDefense( lefthandAttack[i] * w_SkillMod, i, 0 );
				lefthandAttack[i] = tPlusDamCut( lefthandAttack[i] );
			}

			// Adjust Minimum and Maximum Damage
			n_Min_DMG += lefthandAttack[0];
			n_Max_DMG += lefthandAttack[2];

			// Calc final damage
			w_DMG[0] = CalcFinalDamage( n_A_DMG[0], 0 );

			// Post data to form
			var wX = w_DMG[0];
			Last_DMG_A[0] = Last_DMG_B[0] = wX + lefthandAttack[0];
			InnStr[0] += wX + " + " + lefthandAttack[0];
			if ( w998D )
			{
				str_bSUBname += "Double Attack chance<br/>";
				str_bSUB += ( wX * 2 + lefthandAttack[0] ) + "~";
			}
			if ( wX + lefthandAttack[0] < n_Min_DMG && w998G < 100 )
			{
				n_Min_DMG = wX + lefthandAttack[0];
			}
			w_DMG[0] = n_Min_DMG;

			w_DMG[2] = CalcFinalDamage( n_A_DMG[2], 2 );
			var wX = w_DMG[2] + lefthandAttack[2];
			Last_DMG_A[2] = Last_DMG_B[2] = wX + lefthandAttack[2];
			InnStr[2] += w_DMG[2] + " + " + lefthandAttack[2];
			if ( w998D )
			{
				wX = w_DMG[2] * 2 + lefthandAttack[2];
				str_bSUB += wX + " ( " + w998D + "% )<br/>";
			}
			if ( wX > n_Max_DMG && w998G < 100 )
			{
				n_Max_DMG = wX;
			}
			w_DMG[2] = n_Max_DMG;

			w_DMG[1] = CalcFinalDamage( n_A_DMG[1], 1 );
			var wX = w_DMG[1];
			Last_DMG_A[1] = Last_DMG_B[1] = wX + lefthandAttack[1];
			InnStr[1] += wX +" + "+ lefthandAttack[1];

			w_DMG[1] = CalcRightHandDamage( w_DMG[1] );
			w_DMG[1] += CalcLeftHandDamage( lefthandAttack[1] );

			var wX = ApplyDamageModifiers(0);
			n_PerHIT_DMG = wX;
			str_PerHIT_DMG = n_PerHIT_DMG;
		}
		else
		{ // single handed
			hunterPetDamage = 0;
			var hunterPet = 0;
			if ( SkillSearch( skill_RAN_WARG_STRIKE ) )
			{
				hunterPet = CalcWargDamage();
			}
			else if ( SkillSearch( skill_HU_BLITZ_BEAT ) )
			{
				hunterPet = CalcFalconDamage();
			}

			trifectaBlowDamage = 0;
			var trifectaDamage = [0,0,0]; // min, ave, max

			if ( SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW ) )
			{
				var trifectaBlowLevel = SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW );
				var trifectaBlowSkillMod = 1.0 + trifectaBlowLevel * 0.2;
				trifectaBlowDamage = -1;

				// Calculate Min, Ave, and Max damage
				for ( var i = 0; i <= 2; i++ )
				{
					trifectaDamage[i] = CalcFinalDamage( n_A_DMG[i] * trifectaBlowSkillMod, i );
					trifectaDamage[i] = Math.floor( trifectaDamage[i] / 3 ) * 3;

					// On plant monster
					if ( n_B[en_BOSS] === 5 )
					{
						trifectaDamage[i] = 3;
					}
				}

				// Build Label to display damage
				str_bSUBname += "Raging Trifecta Blow Damage<BR>";
				str_bSUB += trifectaDamage[0] + "~" + trifectaDamage[2] + " (" + ( 30 - trifectaBlowLevel ) + "% Chance)<BR>";
				trifectaBlowDamage = 0;

				// Cap Min and Max Damage
				if ( n_Min_DMG > trifectaDamage[0] )
				{
					n_Min_DMG = trifectaDamage[0];
				}
				if ( n_Max_DMG < trifectaDamage[2] )
				{
					n_Max_DMG = trifectaDamage[2];
				}
			}
			if ( SkillSearch( skill_ABI_DUPLE_LIGHT ) ) {
				var DLMagicalSkillMod = 1.0 + SkillSearch( skill_ABI_DUPLE_LIGHT ) * 0.3;
				var DLPhysicalSkillMod = 1.0 + SkillSearch( skill_ABI_DUPLE_LIGHT ) * 0.1;
				// Calculate Min, Ave, and Max damage
				var DLDamage = [0,0,0]; // min, ave, max
				var DLMDamage = [0,0,0]; // min, ave, max
				for ( var i = 0; i <= 2; i++ )
				{
					DLDamage[i] = CalcFinalDamage( n_A_DMG[i] * DLPhysicalSkillMod, i );
					DLDamage[i] = Math.floor( DLDamage[i]);
					DLMDamage[i] = n_A_MATK[i] * DLMagicalSkillMod;
					DLMDamage[i] = Math.floor( DLMDamage[i]);
					// On plant monster
					if ( n_B[en_BOSS] === 5 )
					{
						DLDamage[i] = 1;
						DLMDamage[i] = 1;
					}
				}
				str_bSUBname += "Duple Light Physical Damage<BR>";
				str_bSUB += DLDamage[0] + "~" + DLDamage[2] + " (" + ( 10 + SkillSearch( skill_ABI_DUPLE_LIGHT )*2 ) + "% Chance)<BR>";
				str_bSUBname += "Duple Light Magical Damage<BR>";
				str_bSUB += DLMDamage[0] + "~" + DLMDamage[2] + " (" + ( 10 + SkillSearch( skill_ABI_DUPLE_LIGHT )*2 ) + "% Chance)<BR>";
				dupleLightMagicalDamage = DLMDamage[1];
				dupleLightPhysicalDamage = DLDamage[1];
			}
			if ( SkillSearch( skill_RUN_GIANT_GROWTH ) )
			{
				var giantGrowthSkillMod = 3.0;
				trifectaBlowDamage = -1;

				// Calculate Min, Ave, and Max damage
				for ( var i = 0; i <= 2; i++ )
				{
					trifectaDamage[i] = CalcFinalDamage( n_A_DMG[i] * giantGrowthSkillMod, i );
				}

				// Build Label to display damage
				str_bSUBname += "Giant Growth Damage<br/>";
				str_bSUB += trifectaDamage[0] + "~" + trifectaDamage[2] + " (15% Chance)<br/>";
				trifectaBlowDamage = 0;

				// Cap Min and Max Damage
				if ( n_Min_DMG > trifectaDamage[0] )
				{
					n_Min_DMG = trifectaDamage[0];
				}
				if ( n_Max_DMG < trifectaDamage[2] )
				{
					n_Max_DMG = trifectaDamage[2];
				}
			}

			if ( n_A_WeaponType == weapTyp_BOW && SkillSearch( skill_RAN_FEAR_BREEZE ) )
			{
				var breezeLevel = SkillSearch( skill_RAN_FEAR_BREEZE );
				var minBreezeAttacks = 2.0;
				var maxBreezeAttacks = breezeLevel;
				if ( breezeLevel === 1 )
				{
					maxBreezeAttacks += 1;
				}
				var aveBreezeAttacks = ( minBreezeAttacks + maxBreezeAttacks ) / 2.0;
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

				// Calculate Min, Ave, and Max damage
				trifectaDamage[0] = CalcFinalDamage( n_A_DMG[0] * minBreezeAttacks, 0 );
				trifectaDamage[1] = CalcFinalDamage( n_A_DMG[1] * aveBreezeAttacks, 1 );
				trifectaDamage[2] = CalcFinalDamage( n_A_DMG[2] * maxBreezeAttacks, 2 );

				// Build Label to display damage
				str_bSUBname += "Fear Breeze Damage<br/>";
				str_bSUB += trifectaDamage[0] + "~" + trifectaDamage[2] + " (" + breezeChance + "% Chance)<br/>";
				trifectaBlowDamage = 0;

				// Cap Min and Max Damage
				if ( n_Min_DMG > trifectaDamage[0] )
				{
					n_Min_DMG = trifectaDamage[0];
				}
				if ( n_Max_DMG < trifectaDamage[2] )
				{
					n_Max_DMG = trifectaDamage[2];
				}
			}

			// Enchant Blade goes here?
			if ( SkillSearch( skill_RUN_ENCHANT_BLADE ) )
			{
				for ( var i = 0; i < 3; i++ )
				{
					n_A_DMG[i] += ( SkillSearch( skill_RUN_ENCHANT_BLADE ) * 20 + 100 ) * ( n_A_BaseLV / 150.0 ) + n_A_INT;
				}
			}

			// Get other ATK Mods and apply to working damage.
			CalcAtkMods02( w_SkillMod , 0 );

			for ( var i = 0; i < 3; i++ )
			{
				w_DMG[i] = CalcFinalDamage( n_A_DMG[i], i );
			}

			var w_KATARU = [0,0,0];
			var katarOffhandDamage = 0;
			if ( n_A_WeaponType === weapTyp_KATAR )
			{
				for ( var i = 0; i < 3; i++ )
				{
					w_KATARU[i] = Math.floor( w_DMG[i] * ( 0.01 + SkillSearch( skill_TH_DOUBLE_ATTACK ) * 0.02 ) );
				}
				katarOffhandDamage = Math.floor( w_DMG[1] * ( 0.01 + SkillSearch( skill_TH_DOUBLE_ATTACK ) * 0.02 ) );
			}

			// Determine true minDamage
			Last_DMG_B[0] = w_DMG[0];
			Last_DMG_A[0] = Last_DMG_B[0] + w_KATARU[0];
			InnStr[0] += Last_DMG_A[0];
			if ( n_A_WeaponType == weapTyp_KATAR )
			{
				InnStr[0] = Last_DMG_A[0] +" ("+ Last_DMG_B[0] +"+"+ w_KATARU[0] +")";
			}
			if(Last_DMG_A[0] < n_Min_DMG && w998G < 100)
			{
				n_Min_DMG = Last_DMG_A[0];
			}
			if ( w998D )
			{ // Double Attack
				if ( n_A_WeaponType == weapTyp_HANDGUN &&
					 SkillSearch( skill_GS_CHAIN_ACTION ) )
				{
					if ( CardNumSearch( 43 ) || EquipNumSearch( 570 ) || EquipNumSearch( 1296 ) )
					{ // Sidewinder Card, Chick Hat, or Snake Head
						str_bSUBname += "Double attack chance<BR>";
					}
					else
					{
						str_bSUBname += "Chain action chance<BR>";
					}
				}
				else
				{
					// dagger
					str_bSUBname += "Double attack chance<BR>";
				}
				str_bSUB += Last_DMG_A[0] * 2 +"~";
			}
			w_DMG[0] = n_Min_DMG;

			// Determine true maxDamage
			Last_DMG_B[2] = w_DMG[2];
			Last_DMG_A[2] = Last_DMG_B[2] + w_KATARU[2];
			InnStr[2] += Last_DMG_A[2];
			if ( n_A_WeaponType === weapTyp_KATAR )
			{
				InnStr[2] = Last_DMG_A[2] + " (" + Last_DMG_B[2] + "+" + w_KATARU[2] + ")";
			}
			n_Max_DMG += hunterPetDamage;
			var wX = Last_DMG_A[2];
			wX += hunterPetDamage;
			if ( n_Max_DMG < wX && w998G < 100 )
			{
				n_Max_DMG = wX;
			}

			if ( w998D )
			{ // Double Attack
				var wX = ( w_DMG[2] + w_KATARU[2] ) * 2;
				str_bSUB += wX + " (" + w998D + "%)<BR>";
				wX += hunterPetDamage;
				if ( n_Max_DMG < wX )
				{
					n_Max_DMG = wX;
				}
			}

			w_DMG[2] = n_Max_DMG;

			// Determine true ave damage
			Last_DMG_B[1] = w_DMG[1];
			Last_DMG_A[1] = Last_DMG_B[1] + w_KATARU[1];
			InnStr[1] += Last_DMG_A[1];
			if ( n_A_WeaponType == weapTyp_KATAR )
			{
				InnStr[1] = Last_DMG_A[1] + " (" + Last_DMG_B[1] + "+" + w_KATARU[1] + ")";
			}

			if ( SkillSearch( skill_MO_RAGING_TRIFECTA_BLOW ) ||
				 SkillSearch( skill_RUN_GIANT_GROWTH ) ||
				 SkillSearch( skill_RAN_FEAR_BREEZE ) )
			{
				trifectaBlowDamage = trifectaDamage[1];
			}

			w_DMG[1] += katarOffhandDamage;
			w_DMG[1] = CalcRightHandDamage( w_DMG[1] );
			w_DMG[1] += hunterPet;
		}
		setupPVPPlayerMods();
		return;
	}
	else if ( n_A_ActiveSkill == skill_SN_FOCUSED_ARROW_STRIKE || n_A_ActiveSkill == skill_NIN_SHADOW_SLASH )
	{
		myInnerHtml("CRIATKname",SubName[3][Language],0);
		myInnerHtml("CRInumname",SubName[4][Language],0);

		if ( n_A_ActiveSkill === skill_SN_FOCUSED_ARROW_STRIKE )
		{ // Focused Arrow Strike
			damageType = kDmgTypeRanged;
			w_SkillMod += ( 1 + 0.5 * n_A_ActiveSkillLV );
			myInnerHtml( "CRInum", ( Math.round( w998G * 100 ) / 100 ) + SubName[0][Language], 0 );

			fixedCastTime *= 0.0;
			if ( EquipNumSearch( 1306 ) && n_A_Weapon_ATKplus >= 10 )
			{ // Little Feather Hat + Falken Blitz
				variableCastTime *= 1.0;
			}
			else
			{
				variableCastTime *= 2.0;
			}
			n_Delay[ksDelayGlobal] = 1.5;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else
		{ // Shadow Slash
			damageType = kDmgTypeMelee;
			w_SkillMod += ( n_A_ActiveSkillLV - 1 );
			myInnerHtml( "CRInum", ( Math.round( w998G * 100 ) / 100 ) + SubName[0][Language], 0 );

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.0;
			n_Delay[ksDelayAnimation] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		 // Crit Part -----------------------------
		for ( var i = 0; i < 3; i++ )
		{
			n_A_CriATK[i] = n_A_DMG[i];
		}
		CalcAtkMods02( w_SkillMod, 1 );

		wCriTyuu=1;
		for ( var i = 0; i < 3; i++ )
		{
			n_A_CriATK[i] = CalcFinalDamage(n_A_CriATK[i],10);
		}
		wCriTyuu=0;

		if(w998G >= 100)
			n_Min_DMG = n_A_CriATK[0];
		if(w998G > 0)
			n_Max_DMG = n_A_CriATK[2];
		myInnerHtml("CRIATK",n_A_CriATK[0] +"~"+ n_A_CriATK[2],0);

		// NonCrit Part -------------------------
		CalcAtkMods02(w_SkillMod,0);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		if(w998G >= 100)
		{
			w_DMG[0] = n_Min_DMG;
		}
		if(w998G > 0)
		{
			w_DMG[2] = n_Max_DMG;
		}

		w_DMG[1] = CalcRightHandDamage(w_DMG[1]);
		setupPVPPlayerMods();
		return;
	}

	// Physical normal single-hit-calc skills (?)
	w_ActS=[skill_SW_BASH,
			skill_SW_MAGNUM_BREAK,
			skill_TH_SAND_ATTACK,
			skill_AR_ARROW_SHOWER,
			skill_AR_ARROW_REPEL,
			skill_ME_MAMMONITE,
			skill_KN_SPEAR_STAB,
			skill_KN_SPEAR_BOOMERANG,
			skill_KN_BRANDISH_SPEAR,
			skill_AS_SONIC_BLOW,
			skill_AS_GRIMTOOTH,
			skill_HU_FREEZING_TRAP,
			skill_CR_SMITE,
			skill_CR_HOLY_CROSS,
			skill_RG_BACK_STAB,
			skill_RG_SIGHTLESS_MIND,
			skill_RG_GANK,
			skill_MO_RAGING_QUADRUPLE_BLOW,
			skill_MO_RAGING_THRUST,
			skill_BA_MELODY_STRIKE,
			skill_DA_SLINGING_ARROW,
			skill_AL_BOMB,
			skill_LK_TRAUMATIC_BLOW,
			skill_LK_VITAL_STRIKE,
			skill_AX_METEOR_ASSAULT,
			skill_CH_RAGING_PALM_STRIKE,
			skill_CH_GLACIER_FIST,
			skill_CH_CHAIN_CRUSH_COMBO,
			skill_CG_ARROW_VULCAN,
			skill_ALL_TOMAHAWK_THROWING,
			skill_MON_PULSE_STRIKE,
			skill_TKK_FLYING_KICK_SPRINT,
			skill_AS_VENOM_KNIFE,
			skill_HU_FANTASTIC_ARROW,
			skill_KN_CHARGE_ATTACK,
			skill_MS_HIGH_SPEED_CART_RAM,
			skill_HEAT,
			skill_HEAT_WALL,
			skill_TK_TORNADO_KICK,
			skill_TK_HEEL_DROP,
			skill_TK_ROUNDOUSE,
			skill_TK_COUNTER_KICK,
			skill_TK_FLYING_KICK,
			skill_MO_EXCRUCIATING_PALM,
			skill_AS_SONIC_BLOW_SL,
			skill_NIN_FLIP_TATAMI,
			skill_NIN_HAZE_SLASHER,
			skill_GS_BULLS_EYE,
			skill_GS_MAGICAL_BULLET,
			skill_GS_TRIGGER_HAPPY_SHOT,
			skill_GS_TRACKING,
			skill_GS_DISARM,
			skill_GS_WOUNDING_SHOT,
			skill_GS_CROWD_CONTROL_SHOT,
			skill_GS_FULL_BLAST,
			skill_GS_SPREAD_SHOT,
			skill_GS_GUNSLINGER_MINE,
			skill_MIWA_GREAT_ECHO,
			skill_RUN_SONIC_WAVE,
			skill_RUN_WIND_CUTTER,
			skill_RUN_IGNITION_BREAK,
			skill_RUN_PHANTOM_THRUST,
			skill_RUN_DEATH_BOUND,
			skill_RUN_CRUSH_STRIKE,
			skill_RUN_STORM_BLAST,
			skill_RUN_HUNDRED_SPEAR,
			skill_GEN_CART_CANNON,
			skill_SUR_SKY_NET_BLOW,
			skill_SUR_EARTH_SHAKER,
			skill_SUR_RAMPAGE_BLASTER,
			skill_SUR_KNUCKLE_ARROW,
			skill_SUR_WINDMILL,
			skill_SUR_LION_HOWLING,
			skill_SUR_LIGHTENING_RIDE,
			skill_SUR_GENTLE_TOUCH_SILENCE,
			skill_RAN_ARROW_STORM,
			skill_RAN_CLUSTER_BOMB,
			skill_GLT_PHANTOM_MENACE,
			skill_GLT_DARK_ILLUSION,
			skill_GLT_WEAPON_CRUSH,
			skill_GLT_COUNTER_SLASH,
			skill_GLT_ROLLING_CUTTER,
			skill_GLT_CROSS_RIPPER_SLASHER,
			skill_ROY_SPEAR_CANNON,
			skill_ROY_VANISHING_POINT,
			skill_ROY_PINPOINT_ATTACK,
			skill_ROY_RAGE_BURST,
			skill_ROY_SHIELD_SPELL_ATK,
			skill_ROY_EXCEED_BREAK,
			skill_ROY_OVERBRAND,
			skill_ROY_OVERBRAND_OLD,
			skill_ROY_MOON_SLASHER,
			skill_ROY_EARTH_DRIVE,
			skill_MEC_AXE_BOOMERANG,
			skill_MEC_POWER_SWING,
			skill_MEC_BOOST_KNUCKLE,
			skill_MEC_PILE_BUNKER,
			skill_MEC_VULCAN_ARM,
			skill_MEC_ARM_CANNON,
			skill_MEC_FLAME_LAUNCHER,
			skill_MEC_COLD_SLOWER,
			skill_KAG_CROSS_STRIKE,
			skill_KAG_SPIRIT_BREAKER,
			skill_KAG_HUUMA_SHURIKEN_STRIKE,
			skill_KAG_THROW_EXPLOSIVE_KUNAI,
			skill_GLT_DARK_CLAW,
			skill_GEN_HALLUCINATION_DRUG,
			"NULL"];
	for ( var iw=0; w_ActS[iw] != n_A_ActiveSkill && w_ActS[iw] != "NULL"; iw++ );
	if ( n_A_ActiveSkill == w_ActS[iw] )
	{
		wActiveHitNum = 1;
		if ( n_A_ActiveSkill === skill_SW_BASH )
		{
			w_SkillMod += n_A_ActiveSkillLV * 0.3;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_SW_MAGNUM_BREAK )
		{
			w_SkillMod += n_A_ActiveSkillLV * 0.2;
			n_A_Weapon_element = ele_FIRE;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_TH_SAND_ATTACK )
		{
			not_use_card = 1;
			w_SkillMod += 0.3;
			n_A_Weapon_element = ele_EARTH;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_AR_ARROW_SHOWER )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV *0.05 - 0.25; // update to RE !

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_AR_ARROW_REPEL )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += 0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.5;
		}
		else if ( n_A_ActiveSkill === skill_ME_MAMMONITE )
		{
			w_SkillMod += n_A_ActiveSkillLV *0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_KN_SPEAR_STAB)
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV *0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_AS_GRIMTOOTH)
		{
			if ( n_A_ActiveSkillLV >= 3 )
			{
				damageType = kDmgTypeRanged;
			}
			w_SkillMod += 0.2 * n_A_ActiveSkillLV;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_CR_SMITE)
		{
			w_SkillMod += n_A_ActiveSkillLV * 0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_CR_HOLY_CROSS)
		{
			w_SkillMod += n_A_ActiveSkillLV *0.35;
			n_A_Weapon_element = ele_HOLY;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_RG_SIGHTLESS_MIND)
		{
			w_SkillMod += n_A_ActiveSkillLV *0.4;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_KN_SPEAR_BOOMERANG)
		{
			w_SkillMod += n_A_ActiveSkillLV *0.5;
			damageType = kDmgTypeRanged;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_KN_BRANDISH_SPEAR)
		{
			w = (1+n_A_ActiveSkillLV*0.2);
			if(n_A_ActiveSkillLV == 10)w_SkillMod += 4.625;
			else if(n_A_ActiveSkillLV >= 7)w_SkillMod += w+w/2+w/4-1;
			else if(n_A_ActiveSkillLV >= 4)w_SkillMod += w+w/2-1;
			else w_SkillMod += w-1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.7;
		}
		else if ( n_A_ActiveSkill == skill_AS_SONIC_BLOW || n_A_ActiveSkill == skill_AS_SONIC_BLOW_SL )
		{

			wActiveHitNum = 8;
			w_SkillMod += n_A_ActiveSkillLV * 0.5 + 2;
			if ( SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) )
			{ // half with edp on
				w_SkillMod /= 2.0;
			}

			if ( n_A_ActiveSkill == skill_AS_SONIC_BLOW_SL && PlayerVersusPlayer == 0 )
			{
				w_SkillMod *= 2;
			}
			else if ( n_A_ActiveSkill == skill_AS_SONIC_BLOW_SL && PlayerVersusPlayer == 1 )
			{
				if(n_Ses) // WoE
					w_SkillMod *= 1.25;
				else
					w_SkillMod *= 2;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayAnimation] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_HU_FREEZING_TRAP )
		{
			not_use_card = 1;
			n_A_Weapon_element = ele_WATER;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_RG_BACK_STAB)
		{

			w_SkillMod += n_A_ActiveSkillLV *0.4 + 2;
			w_HIT = 100;
			w_HIT_HYOUJI = 100;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if(n_A_ActiveSkill==skill_RG_GANK)
		{
			w_SkillMod += n_A_ActiveSkillLV * 0.3;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_MO_RAGING_QUADRUPLE_BLOW)
		{
			wActiveHitNum = 4;
			w_SkillMod += 0.5+n_A_ActiveSkillLV *0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayASPD] = 0.1;
			n_Delay[ksDelayAnimation] = 1.0 - ( 0.004 * n_A_AGI ) - ( 0.002 * n_A_DEX );
		}
		else if(n_A_ActiveSkill==skill_MO_RAGING_THRUST)
		{
			w_SkillMod += 1.4+n_A_ActiveSkillLV *0.6;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayASPD] = 0.1;
			n_Delay[ksDelayAnimation] = 0.7 - ( 0.004 * n_A_AGI ) - ( 0.002 * n_A_DEX );
		}
		else if ( n_A_ActiveSkill == skill_BA_MELODY_STRIKE || n_A_ActiveSkill == skill_DA_SLINGING_ARROW )
		{
			w_SkillMod += (n_A_ActiveSkillLV * 0.4 - 0.4);
			n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
			if(eval(document.calcForm.A_Weapon_element.value) != 0)
				n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
			damageType = kDmgTypeRanged;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.5;
		}
		else if(n_A_ActiveSkill==skill_AL_BOMB)
		{
			not_use_card = 1;
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod += n_A_ActiveSkillLV *0.2;
			w_HIT = 100;
			w_HIT_HYOUJI = 100;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_LK_TRAUMATIC_BLOW)
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV *0.4;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if(n_A_ActiveSkill==skill_LK_VITAL_STRIKE)
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += (n_A_ActiveSkillLV *0.1 -0.5);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			if ( n_A_ActiveSkillLV > 5 )
			{
				n_Delay[ksDelayGlobal] = 1;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 0.8;
			}
		}
		else if ( n_A_ActiveSkill == skill_AX_METEOR_ASSAULT )
		{
			not_use_card = 1;
			w_SkillMod += (n_A_ActiveSkillLV *0.4 -0.6);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if(n_A_ActiveSkill==skill_CH_RAGING_PALM_STRIKE)
		{
			w_SkillMod += (1 + n_A_ActiveSkillLV);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.3;
		}
		else if(n_A_ActiveSkill==skill_CH_GLACIER_FIST)
		{
			w_SkillMod += n_A_ActiveSkillLV -0.6;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayASPD] = 0.1;
			n_Delay[ksDelayAnimation] = 0.7 - ( 0.004 * n_A_AGI ) - ( 0.002 * n_A_DEX );
		}
		else if (n_A_ActiveSkill==skill_CH_CHAIN_CRUSH_COMBO)
		{
			w_SkillMod += (3 + n_A_ActiveSkillLV);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
			if ( n_A_ActiveSkillLV > 6 )
			{
				n_Delay[ksDelayGlobal] = 1.0;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 0.8;
			}
		}
		else if ( n_A_ActiveSkill === skill_CG_ARROW_VULCAN )
		{
			wActiveHitNum = 9;
			w_SkillMod += 1 + n_A_ActiveSkillLV;
			n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
			if ( parseInt( formElements["A_Weapon_element"].value ) !== 0 )
			{
				n_A_Weapon_element = parseInt( formElements["A_Weapon_element"].value );
			}
			damageType = kDmgTypeRanged;
			var cast = 1.8 + 0.2 * n_A_ActiveSkillLV;
			fixedCastTime *= 0.2 * cast;
			variableCastTime *= 0.8 * cast;
			if ( n_A_ActiveSkillLV > 6 )
			{
				n_Delay[ksDelayGlobal] = 1.0;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 0.8;
			}
			n_Delay[ksDelayAnimation] = 3.0;
		}
		else if(n_A_ActiveSkill==skill_ALL_TOMAHAWK_THROWING)
		{
			damageType = kDmgTypeRanged;
			not_use_card = 1;
			n_A_Weapon_element = ele_WIND;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_MON_PULSE_STRIKE)
		{
			w_SkillMod += (n_A_ActiveSkillLV -1) * 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_AS_VENOM_KNIFE)
		{
			damageType = kDmgTypeRanged;
			not_use_card = 1;
			n_A_DMG[1] += Math.floor(14.5 * weaponSizeMod);
			n_A_DMG[2] += Math.floor(29 * weaponSizeMod);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_HU_FANTASTIC_ARROW)
		{
			damageType = kDmgTypeRanged;
			not_use_card = 1;
			w_SkillMod += 0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill == skill_KN_CHARGE_ATTACK )
		{
			var distance = eval(document.calcForm.SkillSubNum.value);
			var baseCast = 0.5 * ( distance + 1.0 );
			if ( baseCast > 1.5 )
			{
				baseCast = 1.5;
			}
			w_SkillMod += distance;

			fixedCastTime *= 0.0;
			variableCastTime *= baseCast;
		}
		else if ( n_A_ActiveSkill == skill_HEAT || n_A_ActiveSkill == skill_HEAT_WALL )
		{
			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			if ( n_A_ActiveSkill == skill_HEAT )
			{
				n_Delay[ksDelayA] = 1.0;
			}
			n_Delay[ksDelayF] = 0.05;
			if ( n_B[en_BOSS] == 1 )
			{
				n_Delay[ksDelayF] = 0.1;
			}
			if ( PlayerVersusPlayer == 1 )
			{
				if ( n_A_ActiveSkill == skill_HEAT_WALL )
				{
					n_Delay[ksDelayA] = 1.0;
				}
				str_bSUBname += "SP damage<br/>";
				str_bSUB += "15<br/>";
			}
		}
		else if(n_A_ActiveSkill==skill_MS_HIGH_SPEED_CART_RAM)
		{
			not_use_card = 1;
			w_SkillMod += Math.floor((eval(document.calcForm.SkillSubNum.value) / (16 - n_A_ActiveSkillLV) / 100 -1) * 100) /100;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_MO_EXCRUCIATING_PALM)
		{
			not_use_card = 1;
			w_SkillMod += 2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if(n_A_ActiveSkill==skill_TK_TORNADO_KICK || n_A_ActiveSkill==skill_TK_HEEL_DROP)
		{
			w_SkillMod += (0.6 + n_A_ActiveSkillLV * 0.2);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_TK_ROUNDOUSE || n_A_ActiveSkill==skill_TK_COUNTER_KICK)
		{
			w_SkillMod += (0.9 + n_A_ActiveSkillLV * 0.3);
			if(n_A_ActiveSkill==skill_TK_COUNTER_KICK)
				wActiveHitNum = 3;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_TK_FLYING_KICK)
		{
			w_SkillMod += (-0.7 + n_A_ActiveSkillLV * 0.1);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_TKK_FLYING_KICK_SPRINT)
		{
			if(SkillSearch(skill_TK_SPRINT_STR_STATE) && n_A_WeaponType==0)
				w_SkillMod += (n_A_BaseLV * 0.08 - 1);
			else
				w_SkillMod += (n_A_BaseLV * 0.04 - 1);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if(n_A_ActiveSkill==skill_NIN_FLIP_TATAMI)
		{
			w_SkillMod += (n_A_ActiveSkillLV * 0.1);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else if(n_A_ActiveSkill==skill_NIN_HAZE_SLASHER)
		{
			w_SkillMod += (n_A_ActiveSkillLV * 0.1);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_BULLS_EYE )
		{ // Bulls Eye
			not_use_card = 1;
			damageType = kDmgTypeRanged;
			wActiveHitNum = 5;
			if(n_B[en_RACE] == 2 || n_B[en_RACE] == 7)
			{
				w_SkillMod += 4;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_MAGICAL_BULLET )
		{ // Magical Bullet
			damageType = kDmgTypeRanged;
			n_A_Weapon_element = ele_GHOST;
			not_use_card = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_TRIGGER_HAPPY_SHOT )
		{ // Trigger Happy Shot
			damageType = kDmgTypeRanged;
			wActiveHitNum = 5;
			w_SkillMod += n_A_ActiveSkillLV * 0.5 + 4;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_TRACKING )
		{ // Tracking
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV * 1 + 1;
			w_HIT = w_HIT * 5 +5;
			if ( w_HIT > 100 )
			{
				w_HIT = 100;
			}
			w_HIT_HYOUJI = w_HIT;

			fixedCastTime *= 1.0 + 0.2 * n_A_ActiveSkillLV;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_DISARM )
		{ // Disarm
			damageType = kDmgTypeRanged;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_WOUNDING_SHOT )
		{ // Wounding Shot
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV *0.2;
			w_HIT = 100;
			w_HIT_HYOUJI = 100;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.5;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_CROWD_CONTROL_SHOT )
		{ // Crowd Control Shot
			damageType = kDmgTypeMelee;
			w_SkillMod += n_A_ActiveSkillLV *0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.0;
			n_Delay[ksDelayAnimation] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_FULL_BLAST )
		{ // Full Blast
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV * 1 + 2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_SPREAD_SHOT )
		{ // Spread Shot
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV * 0.2 - 0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_GUNSLINGER_MINE )
		{
			damageType = kDmgTypeRanged;
			not_use_card = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 0.0;
		}
		else if ( n_A_ActiveSkill === skill_MIWA_GREAT_ECHO )
		{ // Great Echo
			var numPerformers = parseInt(formElements["SkillSubNum"].value);
			var performerBonus = 0;
			if ( numPerformers === 3 )
			{
				performerBonus = 1;
			}
			else if ( numPerformers === 4 )
			{
				performerBonus = 2;
			}
			else if ( numPerformers === 5 )
			{
				performerBonus = 4;
			}
			else if ( numPerformers === 6 )
			{
				performerBonus = 8;
			}
			else if ( numPerformers >= 7 )
			{
				performerBonus = 16;
			}
			damageType = kDmgTypeRanged;

			w_SkillMod = n_A_ActiveSkillLV * 2 + 4; // base mod
			w_SkillMod *= n_A_BaseLV / 100.0; // level bonus
			w_SkillMod += performerBonus; // performer count bonus

			fixedCastTime *= 0.5;
			variableCastTime *= 1.8 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 10.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_SONIC_WAVE )
		{
			// 600%-1000% damage
			w_SkillMod = n_A_ActiveSkillLV + 5.0;

			if ( n_A_BaseLV > 99 )
			{
				w_SkillMod *= 1 + ( n_A_BaseLV - 100 ) / 200.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_WIND_CUTTER )
		{
			// 150%-350% damage
			w_SkillMod = ( n_A_ActiveSkillLV + 2 ) * 0.5 * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_IGNITION_BREAK )
		{ // Ignition Break

			// Local variables
			var distance = parseInt(formElements["SkillSubNum"].value);
			var distanceMod = 0;
			if ( distance === 1 )
			{ // close 300%-1500% damage
				//
				distanceMod = 3.0;
			}
			else if ( distance === 2 )
			{ // medium 250%-1250% damage
				distanceMod = 2.5;
			}
			else
			{ // far away 200%-1000% damage
				distanceMod = 2.0;
			}

			// ATK [{(Skill Level x DistanceMod) x (1 + [(Caster s Base Level - 100) / 100])}] %
			w_SkillMod = ( n_A_ActiveSkillLV * distanceMod ) * n_A_BaseLV / 100.0;

			if ( n_A_Weapon_element === ele_FIRE )
			{ // does more with fire element weapon
				w_SkillMod += ( n_A_ActiveSkillLV * 1.0 );
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 0.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_PHANTOM_THRUST )
		{
			// 50%-250% damage
			w_SkillMod = ( n_A_ActiveSkillLV / 2.0 + ( SkillSearch( skill_KN_SPEAR_MASTERY ) / 10.0 ) ) * n_A_BaseLV / 150.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_DEATH_BOUND )
		{
			// Amplification: 600%-1500% damage
			var damage = parseInt(formElements["SkillSubNum"].value);
			damage *= n_A_ActiveSkillLV * 5.0;
			w_SkillMod = 1;

			// calc reflected damage
			myInnerHtml( "CRIATKname", '<font color="#0000FF"><b>Reflected Damage</b></font>', 0 );
			var damageOut = 0.7 * damage;
			myInnerHtml( "CRIATK", '<font color="#0000FF"><b>' + damageOut + '</b></font>', 0 );

			// Calculate incoming damage
			myInnerHtml( "CRInumname", '<font color="#FF0000"><b>Incoming Damage</b></font>', 0 );
			var damageIn = 0.3 * damage;
			myInnerHtml( "CRInum", '<font color="#FF0000"><b>' + damageIn + '</b></font>', 0 );

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_CRUSH_STRIKE )
		{
			// (( WeaponLevel * (WeaponUpgradeLevel + 6) * 100) + WeaponAtk + WeaponWeight)%
			var weaponAtk = ItemOBJ[n_A_Equip[eq_WEAPON]][itm_ATK];
			var weaponWeight = ItemOBJ[n_A_Equip[eq_WEAPON]][itm_WEIGHT];
			w_SkillMod = n_A_WeaponLV * ( n_A_Weapon_ATKplus + 6 ) + weaponAtk / 100.0 + weaponWeight / 100.0;

			fixedCastTime *= 1.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayCooldown] = 30.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_STORM_BLAST )
		{
			// ((RuneMastery + (INT / 8)) * 100)%
			w_SkillMod = SkillSearch( skill_KN_SPEAR_MASTERY ) + Math.floor( n_A_INT / 8 );

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
		}
		else if ( n_A_ActiveSkill === skill_RUN_HUNDRED_SPEAR )
		{
			// 680%-1400% damage
			damageType = kDmgTypeRanged;
			var clashingLevel = parseInt(formElements["SkillSubNum"].value);
			var weaponWeight = ItemOBJ[n_A_Equip[eq_WEAPON]][itm_WEIGHT];
			if ( weaponWeight > 1000 )
			{
				weaponWeight = 1000;
			}
			w_SkillMod = ( n_A_ActiveSkillLV * 0.8 ) + 6 + ( ( 1000 - weaponWeight ) / 100.0 );

			if ( n_A_BaseLV > 99 )
			{
				w_SkillMod *= 1 + ( n_A_BaseLV - 100 ) / 200.0;
			}

			// add in CS bonus damage?
			w_SkillMod += clashingLevel / 5.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.1 - 0.1 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_KAG_CROSS_STRIKE )
		{
			damageType = kDmgTypeRanged;
			var cross = parseInt(formElements["SkillSubNum"].value);
			if (cross) {
				w_SkillMod = (( n_A_ActiveSkillLV * 1.5 ) * n_A_BaseLV / 100.0)*1.5;
			} else {
				w_SkillMod = ( n_A_ActiveSkillLV * 1.5 ) * n_A_BaseLV / 100.0;
			}
			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_KAG_HUUMA_SHURIKEN_STRIKE )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod = (( n_A_ActiveSkillLV * 1.5 ) + ( n_A_DEX + n_A_AGI ) / 200.0 + 2.5) * n_A_BaseLV / 100.0;
			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_KAG_THROW_EXPLOSIVE_KUNAI )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod = SkillSearch( skill_NIN_DAGGER_THROWING_PRACTICE ) * 0.5 * n_A_ActiveSkillLV;
			fixedCastTime *= 0.0;
			variableCastTime *= 0.5*(1+n_A_ActiveSkillLV);
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_KAG_SPIRIT_BREAKER )
		{
			damageType = kDmgTypeMelee;
			var linked = parseInt(formElements["SkillSubNum"].value);
			if (linked) {
				w_SkillMod = n_A_ActiveSkillLV * 2;
			} else {
				w_SkillMod = n_A_ActiveSkillLV;
			}
			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GEN_CART_CANNON )
		{
			damageType = kDmgTypeRanged;

			// Old formula:  350% + (Skill Level)*50% + (Cart Remodeling Level)*Int/2%
			// Old formula: w_SkillMod = ( 1.4 + n_A_ActiveSkillLV * 0.6 ) + ( SkillSearch( skill_GEN_CART_REMODELING ) * ( n_A_INT ) ) / 100.0;
			// Current Wiki Formula: [(( Cart Remodeling Skill Level * 50 ) * ( INT / 40 )) + ( Cart Cannon Skill Level * 60 )] % ATK

			w_SkillMod = (((SkillSearch( skill_GEN_CART_REMODELING ) * 50) * ( n_A_INT / 40)) + (n_A_ActiveSkillLV * 60))/100

			//w_SkillMod *= n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill === skill_SUR_SKY_NET_BLOW )
		{
			var afterDragonCombo = formElements["SkillSubNum"].checked;

			// ATK [{(Skill Level x 80) + (Caster s AGI)} x Caster s Base Level / 100] %
			// If used after Dragon Combo,
			// ATK [{(Skill Level x 100) + (Caster s AGI) + 150} x Caster s Base Level / 100] %
			if ( afterDragonCombo )
			{
				w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + ( n_A_AGI / 100.0 ) + 1.5 ) * n_A_BaseLV / 100.0;
			}
			else
			{
				w_SkillMod = ( n_A_ActiveSkillLV * 0.8 ) + ( n_A_AGI / 100.0 ) * n_A_BaseLV / 100.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill === skill_SUR_EARTH_SHAKER )
		{
			var visible = formElements["SkillSubNum"].checked;

			// [(Skill Level x 50) x (Caster s Base Level / 100) + (Caster s INT x 2)] %
			// On hidden targets,
			// [(Skill Level x 150) x (Caster s Base Level / 100) + (Caster s INT x 3)] %
			if ( visible )
			{
				w_SkillMod = ( n_A_ActiveSkillLV * 0.5 ) * ( n_A_BaseLV / 100.0 ) + ( ( n_A_INT * 2 ) / 100.0 );
			}
			else
			{
				w_SkillMod = ( n_A_ActiveSkillLV * 1.5 ) * ( n_A_BaseLV / 100.0 ) + ( ( n_A_INT * 3 ) / 100.0 );
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_RAMPAGE_BLASTER )
		{
			var numSpheres = parseInt(formElements["SkillSubNum"].value);
			damageType = kDmgTypeRanged;

			// ATK [{(Skill Level x 20) x Number of Spirit Spheres} x Base Level / 150] %
			// If in Fury state,
			// ATK [{(Fury Skill Level x 20) + (Rampage Blaster Skill Level x 20)} x Number of Spirit Spheres x Caster s Base Level / 120] %
			if ( SkillSearch( skill_MO_FURY ) )
			{
				w_SkillMod = ( ( SkillSearch( skill_MO_FURY ) * 0.2 ) + ( n_A_ActiveSkillLV * 0.2 ) ) * numSpheres * n_A_BaseLV / 120.0;
			}
			else
			{
				w_SkillMod = ( n_A_ActiveSkillLV * 0.2 ) * numSpheres * n_A_BaseLV / 100.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 10.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_KNUCKLE_ARROW )
		{
			var knockback = formElements["SkillSubNum"].checked;
			damageType = kDmgTypeRanged;

			// ATK [(Skill Level x 100 + 500) x Caster s Base Level / 100] %
			// Knockback bonus:
			// ATK [(Skill Level x 150) + (1000 x Target s current weight / Maximum weight) + (Target s Base Level x 5) x (Caster s Base Level / 150)] %
			// On monsters, weight portion is changed to (Monster Level x 50) <-- using this from fallen empire formula
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + 5.0 ) * n_A_BaseLV / 100.0;
			if ( knockback )
			{
				w_SkillMod += ( n_A_ActiveSkillLV * 1.5 ) + ( 10 * 1 ) + ( n_B[en_LEVEL] * 5 / 100.0 ) * n_A_BaseLV / 150.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_WINDMILL )
		{ // Windmill
			// ATK [(Caster s Base Level + Caster s DEX) x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_BaseLV + n_A_DEX ) / 100.0 ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_LION_HOWLING )
		{ // Howling Lion
			// ATK [(Skill Level x 300) x Caster s Base Level / 150] %
			w_SkillMod = n_A_ActiveSkillLV * 3.0 * n_A_BaseLV / 150.0;

			fixedCastTime *= 0.5;
			variableCastTime *= 1.0;
			n_Delay[ksDelayCooldown] = 10.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_LIGHTENING_RIDE )
		{ // Ride in Lightning
			var windBonus = 0;
			damageType = kDmgTypeRanged;
			if ( n_A_Weapon_element === ele_WIND )
			{
				windBonus = 50 * n_A_ActiveSkillLV / 100.0;
			}

			// ATK [{(Skill Level x 200) + Additional Damage} x Caster s Base Level / 100] %
			// Additional damage (if using Wind Element weapon), (50 * Skill Level), else 0.
			w_SkillMod = ( ( n_A_ActiveSkillLV * 2.0 ) + windBonus ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= n_A_ActiveSkillLV;
			n_Delay[ksDelayCooldown] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_SUR_GENTLE_TOUCH_SILENCE )
		{ // Gentle Touch: Silence
			// ATK [(Skill Level x 100 + Caster s DEX) x (Caster s Base Level / 100)] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + ( n_A_DEX / 100.0 ) ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayCooldown] = 0.5 + 0.5 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_RAN_ARROW_STORM )
		{ // Arrow Storm
			// 1080%-1800% damage
			w_SkillMod = ( n_A_ActiveSkillLV * 0.8 + 10 ) * n_A_BaseLV / 100.0;
			if ( EquipNumSearch( 1399 ) )
			{ // Giant Crossbow
				w_SkillMod *= 1 + 0.05*n_A_Weapon_ATKplus;
			}
			fixedCastTime *= 0.0;
			variableCastTime *= 1.8 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 0.0;
			n_Delay[ksDelayCooldown] = 5.2 - 0.2 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_RAN_CLUSTER_BOMB )
		{
			// ( ( ( SkillLvl * DEX ) + ( INT * 5 ) ) * ( 1.5 + ( BaseLvl / 100 ) ) ) * ( ( TrapResearch * 20 ) / 50 )
			w_SkillMod = ( ( n_A_ActiveSkillLV * n_A_DEX ) + ( n_A_INT * 5 ) ) / 100.0;
			w_SkillMod *= 1.5 + n_A_BaseLV / 100.0;
			w_SkillMod *= SkillSearch( skill_RAN_RESEARCH_TRAP ) * 20 / 50.0;

			// doesn't miss
			w_HIT_HYOUJI = 100;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GLT_PHANTOM_MENACE )
		{
			// 300% damage
			w_SkillMod = 3.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_GLT_DARK_ILLUSION )
		{
			// 100% damage
			w_SkillMod = 1.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.5;
		}
		else if ( n_A_ActiveSkill === skill_GLT_WEAPON_CRUSH )
		{
			// 100% damage
			w_SkillMod = 1.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GLT_COUNTER_SLASH )
		{
			// ATK [{(Skill Level x 100) + 300} x Caster's Base Level / 120]% + ATK [(AGI x 2) + (Caster's Job Level x 4)]%
			w_SkillMod = ( n_A_ActiveSkillLV + 3.0 ) * n_A_BaseLV / 120.0;
			w_SkillMod += ( ( n_A_AGI * 2 ) + ( n_A_JobLV * 4 ) ) / 100.0;

			if ( SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) )
			{ // half with edp on
				w_SkillMod /= 2.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_GLT_ROLLING_CUTTER )
		{
			// ATK [{(Skill Level x 50) + 50} x Caster's Base Level/100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.5 ) + 0.5 ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.2;
		}
		else if ( n_A_ActiveSkill === skill_GLT_CROSS_RIPPER_SLASHER )
		{
			var counters = parseInt(formElements["SkillSubNum"].value);

			// Total Damage = A + B
			// A = ATK [{(Skill Level x 80) + 400} x Caster's Base Level / 100] %
			// B = ATK [(Rolling Cutter Counter x Caster's AGI)] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.8 ) + 4 ) * n_A_BaseLV / 100.0;
			w_SkillMod += ( counters * n_A_AGI ) / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_SPEAR_CANNON )
		{ // Spear Cannon
			// ATK [{(Skill Level x 50) + (Caster s STR x Skill Level)} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.5 ) + ( n_A_STR * n_A_ActiveSkillLV / 100.0 ) ) * n_A_BaseLV / 100.0;
			if ( EquipNumSearch( 1269 ) )
			{ // Imperial Spear gives 20% more damage
				var spearBonusDamage = 1.2;
				spearBonusDamage += 0.03 * Math.floor( n_A_Weapon_ATKplus / 2 );
				w_SkillMod *= spearBonusDamage;
			}
			if ( EquipNumSearch( 1385 ) )
			{ // Cannon Spear
				var spearBonusDamage = 1.1;
				spearBonusDamage += 0.03 * Math.floor( n_A_Weapon_ATKplus );
				w_SkillMod *= spearBonusDamage;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_VANISHING_POINT )
		{ // Banishing Point
			var bashLevel = parseInt(formElements["SkillSubNum"].value);

			// ATK [{(Skill Level x 50) + (Caster s learned Bash Level x 30)} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.5 ) + ( bashLevel * 0.3 ) ) * n_A_BaseLV / 100.0;
			if ( EquipNumSearch( 1269 ) )
			{ // Imperial Spear gives 20% more damage
				var spearBonusDamage = 1.2;
				spearBonusDamage += 0.03 * Math.floor( n_A_Weapon_ATKplus / 2 );
				w_SkillMod *= spearBonusDamage;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_PINPOINT_ATTACK )
		{
			// ATK [{(Skill Level x 100) + (Caster s AGI x 5)} x Caster s Base Level / 120] %
			w_SkillMod = ( n_A_ActiveSkillLV + ( n_A_AGI * 5 / 100.0 ) ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_RAGE_BURST )
		{
			var rageCounter = parseInt(formElements["SkillSubNum2"].value);
			var currentHP = parseInt(formElements["SkillSubNum"].value);
			if ( currentHP < 1 )
			{
				currentHP = 1;
			}

			// ATK [{(Number of Spirit Spheres x 200) + <(Caster s Max HP - Current HP) / 100>} x Caster s Base Level / 100] %
			w_SkillMod = ( ( rageCounter * 2 ) + ( n_A_MaxHP - currentHP ) / 100.0 ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_SHIELD_SPELL_ATK )
		{
			// ATK [{(Caster s Base Level x 4) + (Shield DEF x 10)} + (Caster s VIT x 2)] %
			w_SkillMod = ( n_A_BaseLV * 4 + ItemOBJ[n_A_Equip[eq_SHIELD]][itm_DEF] * 10 + n_A_VIT * 2 ) / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_EXCEED_BREAK )
		{
			var weaponWeight = ItemOBJ[n_A_Equip[0]][itm_WEIGHT];//ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WEIGHT];
			var weaponLevel = ItemOBJ[n_A_Equip[0]][itm_WLVL];//ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WLVL];
			console.log(weaponLevel*weaponWeight*n_A_BaseLV/100.0);
			// ATK [{(Skill Level x 100) + (Caster s Job Level x 10)} + (Weapon Weight x Weapon Level) x (Caster s Base Level / 100)] %
			w_SkillMod = ( ( n_A_ActiveSkillLV + ( n_A_JobLV * 10 / 100.0 ) ) + ( ( weaponWeight * weaponLevel ) ) * (n_A_BaseLV / 100.0) / 100.0 );

			// Crit is a guarantee, calc it.
			wCriTyuu=1;
			for ( var i = 0; i < 3; i++ )
			{
				n_A_CriATK[i] = Math.floor(CalcFinalDamage( n_A_CriATK[i], 10 )*w_SkillMod);
			}
			wCriTyuu=0;

			DisplayCriticalDamage();
			fixedCastTime *= 0.0;
			variableCastTime *= 4.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_OVERBRAND )
		{
			var numHits = parseInt(formElements["SkillSubNum"].value);

			// Deals Pierce Damage to targets up to 7 cells infront of the Caster.
			// ATK [{(Skill Level x 200) + (Spear Quicken Skill Level x 50)} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 2 ) + ( SkillSearch( skill_CR_SPEAR_QUICKEN ) * .5 ) ) * n_A_BaseLV / 100.0;

			if ( numHits >= 2 )
			{ // Deals Swing Damage to targets in 5x2 cell infront of the Caster.
			  // ATK [{(Skill Level x 100) + (Caster s STR + DEX)} x Caster s Base Level / 100] %
				w_SkillMod += ( n_A_ActiveSkillLV + ( ( n_A_STR + n_A_DEX ) / 100.0 ) ) * n_A_BaseLV / 100.0;
			}
			if ( numHits >= 3 )
			{ // Deals additional knockback damage.
			  // ATK [(Skill Level x 100) + Random number between (10 ~ 100)] %
			  	var randomNumber = Math.floor( Math.random() * 91 ) + 10;
				w_SkillMod += ( ( n_A_ActiveSkillLV ) + ( randomNumber / 100.0 ) );
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			n_Delay[ksDelayGlobal] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_OVERBRAND_OLD )
		{
			var numHits = parseInt(formElements["SkillSubNum"].value);

			// From RG compilation thread: http://forums.irowiki.org/showpost.php?p=644548&postcount=3366
			//Suggested formula:
			//First hit:
			//[Atk]*[(8*(skill level) + (level of spear quicken))*base level/3]%
			//Or
			//[Atk]*[266.66%*(skill level) + (level of spear quicken)*33.33%]*base level/100 ***THIS IS THE ONE I'M USING FOR CALC***
			//Second hit:
			//[Atk]*[200%*(skill level) + 2%*(Str+Dex)/3]*(base level)/100 ***Times 2%? or times 2?***
			//Third hit:
			//[Atk]*[160%*(skill level)]*(base level)/100

			// Deals Pierce Damage to targets up to 7 cells infront of the Caster.
			w_SkillMod = ( ( n_A_ActiveSkillLV * 8.0 / 3.0 ) + ( SkillSearch( skill_CR_SPEAR_QUICKEN ) / 3.0 ) ) * n_A_BaseLV / 100.0;

			if ( numHits >= 2 )
			{ // Deals Swing Damage to targets in 5x2 cell infront of the Caster.
				w_SkillMod += ( n_A_ActiveSkillLV * 2.0 + ( 0.02 * ( n_A_STR + n_A_DEX ) / 3.0 ) ) * n_A_BaseLV / 100.0;
			}
			if ( numHits >= 3 )
			{ // Deals additional knockback damage.
			  	var randomNumber = Math.floor( Math.random() * 91 ) + 10;
				w_SkillMod += ( n_A_ActiveSkillLV * 1.6 ) * n_A_BaseLV / 100.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			n_Delay[ksDelayGlobal] = 2.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_MOON_SLASHER )
		{
			// ATK [{(Skill Level x 120) + (Overbrand Skill Level x 80)} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.2 ) + ( SkillSearch( skill_ROY_OVERBRAND ) * .8 ) ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 7.0 - 1.0 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_ROY_EARTH_DRIVE )
		{
			var shieldWeight = ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WEIGHT];

			// ATK [{(Skill Level + 1) x Shield Weight} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV + 1 ) * shieldWeight / 100.0 ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 8.0 - 1.0 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_MEC_AXE_BOOMERANG )
		{ // Axe Boomerang
			var axeWeight = ItemOBJ[n_A_Equip[eq_WEAPON]][itm_WEIGHT];

			// ATK [ { ( Skill Level x 50 ) + 250 + Axe Weight } x Caster s Base Level / 100 ] %
			if ( n_A_WeaponType == weapTyp_AXE || n_A_WeaponType == weapTyp_AXEII )
			{
				w_SkillMod = ( ( n_A_ActiveSkillLV * 0.5 ) + 2.5 + ( axeWeight / 100.0 ) ) * n_A_BaseLV / 100.0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayCooldown] = 5.5 - 0.5 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_MEC_POWER_SWING )
		{ // power swing
			// Damage: [(300 + 100 * Skill Level) + Bonus ATK] %
			// Bonus ATK = { ( Caster s STR + DEX ) x Caster s Base Level / 100 }
			var bonusAttack = ( ( n_A_STR + n_A_DEX ) / 100.0 ) * n_A_BaseLV / 100.0;
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + 3.0 ) + bonusAttack;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_MEC_BOOST_KNUCKLE )
		{
			// ATK [ { ( Skill Level x 100 ) + 200 + (Caster s DEX) } x Caster s Base Level / 120 ] %
			w_SkillMod = ( n_A_ActiveSkillLV + 2.0 + n_A_DEX / 100.0 ) * n_A_BaseLV / 120.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.2 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_MEC_PILE_BUNKER )
		{
			// ATK [ { ( Skill Level x 100 ) + 300 + Caster s STR } x Caster s Base Level / 100 ] %
			w_SkillMod = ( n_A_ActiveSkillLV + 3.0 + n_A_STR / 100.0 ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill === skill_MEC_VULCAN_ARM )
		{
			// ATK [ { ( Skill Level x 70 ) + Caster s DEX } x Caster s Base Level / 120 ] %
			w_SkillMod = ( n_A_ActiveSkillLV * 0.7 + n_A_DEX / 100.0 ) * n_A_BaseLV / 120.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.4 - 0.1 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_MEC_ARM_CANNON )
		{
			// Small Monster:  ATK [ { ( Skill Level x 400 ) + 300 } x Caster s Base Level / 120 ] %
			// Medium Monster: ATK { { ( Skill Level x 350 ) + 300 } x Caster s Base Level / 120 ] %
			// Large Monster:  ATK [ { ( Skill Level x 300 ) + 300 } x Caster s Base Level / 120 ] %
			var sizeModifier = 4.0 - n_B[en_SIZE] * 0.5;
			w_SkillMod = ( ( n_A_ActiveSkillLV * sizeModifier ) + 3.0 ) * n_A_BaseLV / 120.0;

			fixedCastTime *= 0.8 - 0.2 * n_A_ActiveSkillLV;
			variableCastTime *= 1.2 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 0.5 * n_A_ActiveSkillLV;
			if ( n_A_ActiveSkillLV === 3 )
			{
				n_Delay[ksDelayGlobal] = 2.0;
			}
		}
		else if ( n_A_ActiveSkill === skill_MEC_FLAME_LAUNCHER )
		{
			// ATK [ { ( Skill Level x 300 ) + 300 } x Caster s Base Level / 150 ] %
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = ( n_A_ActiveSkillLV * 3.0 + 3.0 ) * n_A_BaseLV / 150.0;

			fixedCastTime *= 0.5;
			variableCastTime *= 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 2.0 - 0.5 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_MEC_COLD_SLOWER )
		{
			// ATK [ { ( Skill Level x 300 ) + 300 } x Caster s Base Level / 150 ] %
			n_A_Weapon_element = ele_WATER;
			w_SkillMod = ( n_A_ActiveSkillLV * 3.0 + 3.0 ) * n_A_BaseLV / 150.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_GLT_DARK_CLAW )
		{
			w_SkillMod = 1 * n_A_ActiveSkillLV;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.4 - 0.1 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_GEN_HALLUCINATION_DRUG )
		{
			w_SkillMod = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.4 - 0.1 * n_A_ActiveSkillLV;
		}

		CalcAtkMods02( w_SkillMod, 0 );

		for ( var i = 0; i < 3; i++ )
		{
			w_MagiclBulet = i;
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			if(wActiveHitNum > 1)
			{
				w_DMG[i] = Math.floor(w_DMG[i] / wActiveHitNum) * wActiveHitNum;
			}
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
			if(wActiveHitNum > 1)
			{
				InnStr[i] += " (" + ( w_DMG[i] / wActiveHitNum ) + " x " + wActiveHitNum + "Hit)";
			}
		}
		w_MagiclBulet = 1;
		w_DMG[1] = (w_DMG[1] * w_HIT + ApplyDamageModifiers(0) *(100-w_HIT))/100;
	}
	else if ( n_A_ActiveSkill === skill_HW_STAVE_CRASHER )
	{
		damageType = kDmgTypeRanged;
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(BK_n_A_MATK[i],i);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		n_PerHIT_DMG = ApplyDamageModifiers(0)+n_A_WeaponLV_seirenATK;
		w_DMG[1] = (w_DMG[1] * w_HIT + n_PerHIT_DMG *(100-w_HIT))/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.3;
		n_Delay[ksDelayGlobal] = 0.3;
	}
	// Physical normal attacks with subhits ----------------------------
	else if ( n_A_ActiveSkill === skill_AR_DOUBLE_STRAFE ||
			  n_A_ActiveSkill === skill_KN_PIERCE ||
			  n_A_ActiveSkill === skill_HU_FREEZING_TRAP ||
			  n_A_ActiveSkill === skill_MO_THROW_SPIRIT_SPHERES ||
			  n_A_ActiveSkill === skill_KN_BOWLING_BASH ||
			  n_A_ActiveSkill === skill_GS_TRIPLE_ACTION ||
			  n_A_ActiveSkill === skill_HU_BEAST_STRAFING ||
			  n_A_ActiveSkill === skill_GS_DESPERADO ||
			  n_A_ActiveSkill === skill_MEC_AXE_TORNADO ||
			  n_A_ActiveSkill === skill_SUR_DRAGON_COMBO ||
			  n_A_ActiveSkill === skill_RAN_AIMED_BOLT ||
			  n_A_ActiveSkill === skill_ROY_HESPERUS_LIT ||
			  n_A_ActiveSkill === skill_SHA_TRIANGLE_SHOT ||
			  n_A_ActiveSkill === skill_GLT_CROSS_IMPACT )
	{
		if ( n_A_ActiveSkill === skill_AR_DOUBLE_STRAFE )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_ActiveSkillLV *0.1 -0.1;
			w_TotalHits = 2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_KN_PIERCE )
		{
			w_SkillMod += n_A_ActiveSkillLV *0.1;
			w_TotalHits = n_B[en_SIZE]+1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_KN_BOWLING_BASH )
		{
			w_SkillMod += n_A_ActiveSkillLV *0.4;
			w_TotalHits = 2;
			if(n_A_ActiveSkillLV == 1)
				w_TotalHits = 1;
			wLAch=1;
			if(monsterDebuffs[6] == 1) // LexA
				w_TotalHits += 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.7;
		}
		else if ( n_A_ActiveSkill === skill_MO_THROW_SPIRIT_SPHERES )
		{
			var numSpheres = 1;
			damageType = kDmgTypeRanged;

			w_SkillMod += n_A_ActiveSkillLV *0.5;
			if(n_A_JobSearch2()==15)
				numSpheres = SkillSearch( skill_MO_SUMMON_SPIRIT_SPHERE );
			else
				numSpheres = acolyteBuffs[ksSpheres];

			numSpheres = Min( n_A_ActiveSkillLV, numSpheres );

			w_TotalHits = numSpheres;


			fixedCastTime *= 0.0;
			variableCastTime *= 1.0 + numSpheres;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill === skill_GS_TRIPLE_ACTION )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += 0.5;
			w_TotalHits = 3;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_HU_BEAST_STRAFING )
		{
			damageType = kDmgTypeRanged;
			w_SkillMod += n_A_STR *0.08 - 0.5;
			w_TotalHits = 2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayA] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_GS_DESPERADO )
		{
			damageType = kDmgTypeMelee;
			w_SkillMod += n_A_ActiveSkillLV * 0.5 - 0.5;
			var DEATH = [1,1.2,1.6,2,2.4,3,3.6,4,5,6,7,8,9,10];
			w_TotalHits = DEATH[eval(document.calcForm.SkillSubNum.value)];

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_MEC_AXE_TORNADO && ( n_A_WeaponType == weapTyp_AXE || n_A_WeaponType == weapTyp_AXEII ) )
		{
			// show HP drain
			myInnerHtml( "CRInumname", '<font color="#FF0000"><b>Health Drain</b></font>', 0 );
			myInnerHtml( "CRInum", '<font color="#FF0000"><b>' + 20 * n_A_ActiveSkillLV + '</b></font>', 0 );

			// are we in the small area or outer area
			var areaSize = parseInt( formElements["SkillSubNum"].value );

			damageType = kDmgTypeMelee;

			// ATK [ { ( Skill Level x 100 ) + 200 + Caster s VIT } x Caster s Base Level / 100 ] %
			w_SkillMod = ( n_A_ActiveSkillLV + 2 + n_A_VIT / 100.0 ) * n_A_BaseLV / 100.0;

			if ( areaSize != 0 )
			{ // oustide 5x5 area
				if ( n_A_ActiveSkillLV >= 3 )
				{ // over level 3
					w_SkillMod *= 0.75;
				}
				else
				{ // levels 1 and 2 do 0 damage
					w_SkillMod = 0;
				}
			}
			w_TotalHits = 6;
			w_SkillMod = w_SkillMod / w_TotalHits;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 4.5 - 0.5 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill === skill_SUR_DRAGON_COMBO )
		{
			// ATK [{(Skill Level x 40) + 100} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.4 ) + 1.0 ) * n_A_BaseLV / 100.0;

			w_TotalHits = 2;
			w_SkillMod = w_SkillMod / w_TotalHits;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill === skill_RAN_AIMED_BOLT )
		{
			var numHits = formElements["SkillSubNum"].value;

			// 550%-1000% damage
			w_SkillMod = ( n_A_ActiveSkillLV * 0.5 + 5 ) * n_A_BaseLV / 100.0;
			w_TotalHits = numHits;
			w_SkillMod = w_SkillMod;// / w_TotalHits;

			fixedCastTime *= 0.5;
			variableCastTime *= 4.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill === skill_ROY_HESPERUS_LIT )
		{
			var numRoyalGuards = SkillSearch( skill_ROY_NUM_GUARDS );
			if ( numRoyalGuards < 3 )
			{
				numRoyalGuards = 3;
			}
			w_TotalHits = numRoyalGuards;

			// Royal Guard in inspiration status will be calculated as having 3 Royal Guards in banding status during damage calculation.
			// ATK [{(Skill Level x 120) + (Number of Royal Guards in Banding x 200)} x Caster s Base Level / 100] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.2 ) + ( numRoyalGuards * 2.0 ) ) * n_A_BaseLV / 100.0;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 3.0;
			n_Delay[ksDelayCooldown] = 20.0;
		}
		else if ( n_A_ActiveSkill === skill_GLT_CROSS_IMPACT )
		{ // Cross Impact
			// ATK [{Skill Level x 100) + 1000} x (Caster's Base Level / 120)]
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + 10.0 ) * n_A_BaseLV / 120.0;
			w_TotalHits = 7;
			w_SkillMod = w_SkillMod / w_TotalHits;

			if ( SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) )
			{ // half with edp on
				w_SkillMod /= 2.0;
			}
			if ( EquipNumSearch( 1382 ) )
			{ // Guillotine Katar
				w_SkillMod *= 1.3;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
			n_Delay[ksDelayGlobal] = 3.0 - 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayAnimation] = 2.0;
		}
		else if (n_A_ActiveSkill === skill_SHA_TRIANGLE_SHOT) {
			damageType = kDmgTypeRanged;
			w_SkillMod = ((3.0 + (n_A_ActiveSkillLV-1)*(n_A_AGI/200))*n_A_BaseLV/120)/3;
			w_TotalHits = 3;
			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}

		CalcAtkMods02(w_SkillMod,0);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);

			if ( n_A_ActiveSkill == skill_HU_BEAST_STRAFING &&
				 n_B[en_RACE]!= 2 && n_B[en_RACE] != 4 )
			{
				w_DMG[i] = 0;
			}
			Last_DMG_B[i] = w_DMG[i];
			if ( n_A_ActiveSkill == skill_KN_BOWLING_BASH )
			{
				Last_DMG_B[i] = w_DMG[i] * 2;
			}
			Last_DMG_A[i] = w_DMG[i] * w_TotalHits;
			if ( monsterDebuffs[status_en_LEXA] == 0 || wLAch == 0 )
			{
				InnStr[i] += Math.floor( w_DMG[i] * w_TotalHits ) + " ( " + w_DMG[i] + SubName[8][Language] + w_TotalHits + " hit )";
			}
			else
			{
				InnStr[i] += w_DMG[i] * 3 + "( " + w_DMG[i] * 2 + " + " + w_DMG[i] +" )";
				Last_DMG_B[i] = w_DMG[i] * 3;
			}
			w_DMG[i] *= w_TotalHits;
		}
		var wX = ApplyDamageModifiers(0);
		w_DMG[1] = (w_DMG[1] * w_HIT + wX * w_TotalHits *(100-w_HIT))/100;

		if ( w_TotalHits == 0 && n_A_ActiveSkill==skill_MO_THROW_SPIRIT_SPHERES)
		{
			InnStr[0] = "<font color=Red><b>No spheres available</b></font><br/>" + InnStr[0];
			InnStr[2] += "<br/><font color=Red><b>No spheres available</b></font>";
		}

		n_PerHIT_DMG = 0; //wX * w_TotalHits;
		str_PerHIT_DMG = wX * w_TotalHits + " (" + w_TotalHits + SubName[8][Language] + wX + " Damage)";
	}
	// Physical but special formula skills --------------------------
	else if ( n_A_ActiveSkill === skill_HU_BLITZ_BEAT || n_A_ActiveSkill === skill_SN_FALCON_ASSAULT )
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_NEUTRAL;
		damageType = kDmgTypeRanged;
		wBT = 80 + Math.floor(n_A_DEX /10)*2 + Math.floor(n_A_INT/2)*2 + SkillSearch(skill_HU_STEEL_CROW) *6;
		if ( n_A_ActiveSkill == skill_SN_FALCON_ASSAULT )
		{
			wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) /100);
			wBT = Math.floor(wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			wBT = tPlusDamCut(wBT);
			wBT *= 5;
			if(n_B[en_BOSS] == 5)
				wBT = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else
		{
			wBT = Math.floor(wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			wBT = tPlusDamCut(wBT);
			wBT *= n_A_ActiveSkillLV;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.5;
			n_Delay[ksDelayGlobal] = 1.0;
		}

		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = wBT;
			InnStr[i] += Last_DMG_A[i];
			if ( n_A_ActiveSkill === skill_HU_BLITZ_BEAT )
			{
				Last_DMG_B[i] = wBT / n_A_ActiveSkillLV;
				InnStr[i] += " ("+ Last_DMG_B[i] +" x "+ n_A_ActiveSkillLV +"Hit)";
			}
			w_DMG[i] = wBT;
		}
		w_HIT_HYOUJI = 100;
	}
	else if ( n_A_ActiveSkill === skill_RAN_WARG_DASH ||
			  n_A_ActiveSkill === skill_RAN_WARG_BITE ||
			  n_A_ActiveSkill === skill_RAN_WARG_STRIKE )
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_NEUTRAL;
		damageType = kDmgTypeRanged;
		wBT = 80 + Math.floor( n_A_DEX / 10 ) * 2 + Math.floor( n_A_INT / 2) * 2 + SkillSearch( skill_RAN_TOOTH_OF_WARG ) * 30;

		if ( n_A_ActiveSkill === skill_RAN_WARG_DASH )
		{ // Warg Dash
			var Weight = parseInt(formElements["SkillSubNum"].value);
			wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) /100);
			wBT = Math.floor(wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			wBT = tPlusDamCut(wBT);
			wBT *= 5;
			if(n_B[en_BOSS] == 5)
				wBT = 1;
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = wBT;
				InnStr[i] += Last_DMG_A[i];
				w_DMG[i] = wBT;
			}
		}
		else if ( n_A_ActiveSkill === skill_RAN_WARG_BITE )
		{ // Warg Bite
			/*wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) /100);
			wBT = Math.floor(wBT * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			wBT = tPlusDamCut(wBT);
			wBT *= 5;
			if(n_B[en_BOSS] == 5)
				wBT = 1;*/
			not_use_card = 1;
			var TMPATK = GetBaseDmg( ele_NEUTRAL, true,  SkillSearch( skill_RAN_TOOTH_OF_WARG ) * 30);
			for (i = 0; i < 3; i++) {
				TMPATK[i] = TMPATK[i] * (6 + ( n_A_ActiveSkillLV ) * 2 + 1 * (n_A_ActiveSkillLV == 5)) ;
				TMPATK[i] = ApplyEnemyDefense(TMPATK[i], 2, 0);
				if ( n_B[en_ID] === 44 )
				{
					TMPATK[i] = 0;
				}
			}
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 2.0 + 2.0 * n_A_ActiveSkillLV;
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = TMPATK[i];
				InnStr[i] += Last_DMG_A[i];
				w_DMG[i] = TMPATK[i];
			}
		}
		else
		{ // Warg Strike
			not_use_card = 1;
			var TMPATK = GetBaseDmg( ele_NEUTRAL, true,  SkillSearch( skill_RAN_TOOTH_OF_WARG ) * 30);
			for (i = 0; i < 3; i++) {
				TMPATK[i] = TMPATK[i] * SkillSearch( skill_RAN_WARG_STRIKE ) * 2;
				TMPATK[i] = ApplyEnemyDefense(TMPATK[i], 2, 0);
				if ( n_B[en_ID] === 44 )
				{
					TMPATK[i] = 0;
				}
			}
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = TMPATK[i];
				InnStr[i] += Last_DMG_A[i];
				w_DMG[i] = TMPATK[i];
			}
		}
		/*
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = wBT;
			InnStr[i] += Last_DMG_A[i];
			w_DMG[i] = wBT;
		}*/

		// doesn't miss
		w_HIT_HYOUJI = 100;
	}
	else if ( n_A_ActiveSkill == skill_TH_ENVENOM ||
			  ( n_A_ActiveSkill == skill_AS_POISON_REACT &&
			    ( n_B[en_ELEMENT] < 50 ||  60 <= n_B[en_ELEMENT] ) ) )
	{
		n_A_Weapon_element = ele_POISON;

		CalcAtkMods02(w_SkillMod,0);

		wINV = Math.floor(ApplyDamageModifiers(0) * element[n_B[en_ELEMENT]][ele_POISON]/100);
		n_PerHIT_DMG = wINV;

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_POISON]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_DMG[1] = (w_DMG[1] * w_HIT + wINV *(100-w_HIT))/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if(n_A_ActiveSkill==skill_CR_SHIELD_BOOMERANG || n_A_ActiveSkill==skill_CR_SHIELD_BOOMERANG_SL)
	{
		n_PerHIT_DMG = 0;
		damageType = kDmgTypeRanged;
		n_A_Weapon_element = ele_NEUTRAL;
		wSBr = n_A_LEFT_DEF_PLUS *4;

		wbairitu2 = (1 + n_A_ActiveSkillLV *0.3);
		if(n_A_ActiveSkill==skill_CR_SHIELD_BOOMERANG_SL)
			wbairitu2 *= 2;

		n_A_ATK_w = Math.round(Math.floor(n_A_STR/10) * Math.floor(n_A_STR/10));
		var boomAttack = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = boomAttack * w_SkillMod + ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WEIGHT] + wSBr;
			w_DMG[i] = Math.floor(Math.floor(w_DMG[i] * defReduction(n_B[en_HARDDEF]) - n_B_DEF2[i]) * wbairitu2);
			w_DMG[i] = ApplyDamageModifiers(w_DMG[i]);
			if(w_DMG[i] < 1)w_DMG[i] = 1;
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_DMG[1] = (w_DMG[1] * w_HIT)/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 0.7;
		if ( n_A_ActiveSkill == skill_CR_SHIELD_BOOMERANG_SL )
		{
			n_Delay[ksDelayGlobal] = 0.35;
		}
	}
	else if(n_A_ActiveSkill==skill_PA_RAPID_SMITING)
	{
		n_PerHIT_DMG = 0;
		damageType = kDmgTypeRanged;
		n_A_Weapon_element = ele_NEUTRAL;
		wSBr = n_A_LEFT_DEF_PLUS;
		wSC  = ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WEIGHT];

		wbairitu2 = (1 + n_A_ActiveSkillLV *0.3);

		n_A_ATK_w = Math.round(Math.floor(n_A_STR/10) * Math.floor(n_A_STR/10));
		var rapidSmiteAttack = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
		rapidSmiteAttack = rapidSmiteAttack * w_SkillMod + wSC + wSBr * 4;

		wSC -= 100;
		wSC = Max(wSC,0);
		wSC2 = [0,0,0];
		wSC2[2] = 100 + wSC + (wSBr * 2) * wSBr;
		wSC2[1] = 100 + (wSC + (wSBr * 2) * wSBr)/2;
		wSC2[0] = 100

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = ( rapidSmiteAttack * defReduction(n_B[en_HARDDEF]) - n_B_DEF2[i]) * wbairitu2;
			w_DMG[i] += wSC2[i];
			w_DMG[i] = ApplyDamageModifiers(w_DMG[i]);
			if(w_DMG[i] < 1)w_DMG[i] = 1;
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = w_DMG[i] * 5;
			Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i] +" ("+ Last_DMG_B[i] + SubName[8][Language] +"5hit)";
			w_DMG[i] = Last_DMG_A[i];
		}
		w_DMG[1] = (w_DMG[1] * w_HIT)/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill === skill_LK_CLASHING_SPIRAL )
	{
		// this skill has a range of damage to be displayed
		damageType = kDmgTypeRanged;

		// from irowiki
		//Floor[Floor(Weapon Weight/2)*skill level + ATK ]*(100%+50%*s.lvl) * 5
		/*var weightMod = Math.floor( Math.floor( ItemOBJ[n_A_Equip[0]][itm_WEIGHT] / 2 ) * n_A_ActiveSkillLV );
		var skillMod = attack * ( ( 100 + 50 * n_A_ActiveSkillLV ) / 100 );*/

		// from mrk012:
		var attack = ApplyEnemyDefense(clashingATKFormula(Math.floor( ItemOBJ[n_A_Equip[0]][itm_WEIGHT] / 2 )), 2, n_B[en_SOFTDEF]);
		//attack = ApplyDamageModifiers( attack );
		//attack = tPlusDamCut( attack );
		if ( window.console )
		{
			window.console.log( "Attack: " + attack );
		}
		var skillMod = ( 100 + 50 * n_A_ActiveSkillLV ) / 100;
		if (SkillSearch( skill_LK_SPEAR_DYNAMO) )
		{
			skillMod += SkillSearch( skill_LK_SPEAR_DYNAMO) * 5 / 100;
		}
		if ( window.console )
		{
			window.console.log( "skillMod: " + skillMod );
		}
		n_PerHIT_DMG = Math.floor(Math.floor(attack) * skillMod + masteryAttack);
		if ( window.console )
		{
			window.console.log( "final damage: " + n_PerHIT_DMG );
		}
		w_DMG[2] = n_PerHIT_DMG;
		w_DMG[2] = ApplyDamageModifiers(w_DMG[2]);
		w_DMG[2] *= ( rangedMod + 100 ) / 100;
		w_DMG[2] = Math.floor(w_DMG[2]);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_B[i] = w_DMG[i];
			Last_DMG_A[i] = Last_DMG_B[i] * 5;
			InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8][Language] + "5hit)";
			w_DMG[i] = Last_DMG_A[i];
		}
		w_DMG[1] = w_DMG[1] * w_HIT / 100 + n_PerHIT_DMG * ( 100 - w_HIT ) / 100;
		/*wSPP2 = n_A_WeaponLV_seirenATK * element[n_B[en_ELEMENT]][n_A_Weapon_element] / 100;
		wSPP2 = ApplyDamageModifiers( wSPP2 );
		wSPP2 = tPlusDamCut( wSPP2 );
		n_PerHIT_DMG = wSPP2 * 5;

		wSPP = Math.floor( n_A_STR / 10 );
		w_DMG[2] = wSPP * wSPP + ItemOBJ[n_A_Equip[0]][itm_WEIGHT] * 0.8 * ( 1 + 0.5 * n_A_ActiveSkillLV );
		wSPP = 1.25 -(n_B[en_SIZE] * 0.25);
		w_DMG[2] = Math.floor(w_DMG[2] * wSPP + n_A_WeaponLV_seirenATK);
		w_DMG[2] = w_DMG[2] * element[n_B[en_ELEMENT]][n_A_Weapon_element] / 100;
		w_DMG[2] = ApplyDamageModifiers(w_DMG[2]);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_B[i] = w_DMG[i];
			Last_DMG_A[i] = Last_DMG_B[i] * 5;
			InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8][Language] + "5hit)";
			w_DMG[i] = Last_DMG_A[i];
		}*/
		//w_DMG[1] = w_DMG[1] * w_HIT / 100 + n_PerHIT_DMG * ( 100 - w_HIT ) / 100;

		// cast time
		fixedCastTime *= 0.0;
		if ( n_A_ActiveSkillLV === 5 )
		{
			variableCastTime *= 1.0;
		}
		else
		{
			variableCastTime *= 0.1 + 0.2 * n_A_ActiveSkillLV;
		}
		if ( EquipNumSearch( 1386 ) )
		{ // Gigantic Lance
			n_Delay[ksDelayCooldown] = 20.0;
		}
		n_Delay[ksDelayGlobal] = 1.0 + 0.2 * n_A_ActiveSkillLV;
	}
	else if(n_A_ActiveSkill==skill_AS_VENOM_SPLASHER)
	{
		n_PerHIT_DMG = 0;
		not_use_card = 1;

		if(n_B[en_BOSS] == 0)
		{

			w_SkillMod += (500 + 50 * n_A_ActiveSkillLV + 20 * eval(document.calcForm.SkillSubNum.value)) /100;
			CalcAtkMods02(w_SkillMod,0);


			for(var i=0;i<=2;i++)
			{
				w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
				w_DMG[i] = Math.floor(w_DMG[i]);
			}
		}
		else if(n_B[en_BOSS] == 5)
		{
			w_DMG[0] = w_DMG[1] = w_DMG[2] = 1;
		}
		else
		{
			w_DMG[0] = w_DMG[1] = w_DMG[2] = 0;
		}
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0;
		n_Delay[ksDelayA] = 1.0;
	}
	else if ( n_A_ActiveSkill === skill_AX_SOUL_DESTROYER )
	{
		// Local variables
		var physicalDamage = new Array();
		var magicDamage = new Array();
		var baseRandomMagicDamage = 250;
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		not_use_card = 1;
		damageType = kDmgTypeRanged;

		// Calculate Physical portion
		// (((ATK*SkillLv)*((100-Enemy DEF)/100) - Enemy VIT) (Elemental Modifier)
		// This formula is whack for renewal...Just gonna do skillLvl * atk for now
		myInnerHtml( "CRIATKname", "Physical damage portion", 0 );
		w_SkillMod = 1;
		//CalcAtkMods02( w_SkillMod, 0 );
		if ( SkillSearch( skill_AX_ENCHANT_DEADLY_POISON ) )
		{ // half with edp on
			w_SkillMod = 0.5;
		}
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * (((300 + (50 * n_A_ActiveSkillLV)) * w_SkillMod)/100), i, 0 );
		}
		myInnerHtml( "CRIATK", physicalDamage[0] + "-" + physicalDamage[2], 0 );

		// Calculate Magical portion
		// INT*5*SkillLv + Random(500~1000)
		myInnerHtml( "CRInumname", "Magical damage portion", 0 );
		w_SkillMod = n_A_INT * 5 * n_A_ActiveSkillLV;
		// Calc Raw Damage
		for ( var i = 0; i < 3; i++ )
		{
			magicDamage[i] = n_A_MATK[i] * ((300 + (50 * n_A_ActiveSkillLV))/100);
			magicDamage[i] += baseRandomMagicDamage + i * baseRandomMagicDamage;
		}
		myInnerHtml( "CRInum", magicDamage[0] + "-" + magicDamage[2], 0 );

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i] + magicDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i] + " ("
			InnStr[i] += physicalDamage[i] + "-";
			InnStr[i] += magicDamage[i] + ")";
		}

		// still hits on a miss
		var minDamage = ApplyDamageModifiers(0) * n_A_ActiveSkillLV;
		n_PerHIT_DMG = minDamage + magicDamage[1];
		str_PerHIT_DMG = ( minDamage + magicDamage[0] ) +"~"+ ( minDamage + magicDamage[2] );
		if ( n_B[en_BOSS] === 5 )
		{ // plant monsters
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] = 1;
				InnStr[i] += Last_DMG_A[i];
			}
		}
		w_DMG[1] = ( w_DMG[1] * w_HIT + n_PerHIT_DMG * ( 100 - w_HIT ) ) / 100;

		// old stuff
		/*w_SBr = new Array();
		w = n_A_INT * 5 * n_A_ActiveSkillLV;
		w_SBr[2] = w + 1000 - Math.floor((n_B[en_HARDDEF] + n_B[en_HARDMDEF] + n_B_MDEF2 + n_B_DEF2[2])/2);
		w_SBr[1] = w + 750 - Math.floor((n_B[en_HARDDEF] + n_B[en_HARDMDEF] + n_B_MDEF2 + n_B_DEF2[1])/2);
		w_SBr[0] = w + 500 - Math.floor((n_B[en_HARDDEF] + n_B[en_HARDMDEF] + n_B_MDEF2 + n_B_DEF2[0])/2);
		for ( var i = 0; i < 3; i++ )
		{
			w_SBr[i] = tPlusDamCut(w_SBr[i]);
		}

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] *= n_A_ActiveSkillLV;
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + w_SBr[i];
			InnStr[i] += Last_DMG_A[i] +" ("+ w_DMG[i] +" + "+ w_SBr[i] +")";
			w_DMG[i] = Last_DMG_A[i];
		}

		var wX = ApplyDamageModifiers(0) * n_A_ActiveSkillLV;
		n_PerHIT_DMG = wX + w_SBr[1];
		str_PerHIT_DMG = (wX + w_SBr[0]) +"~"+ (wX + w_SBr[2]);
		if ( n_B[en_BOSS] === 5 )
		{
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] = 1;
				InnStr[i] += Last_DMG_A[i];
			}
		}
		w_DMG[1] = (w_DMG[1] * w_HIT + n_PerHIT_DMG *(100-w_HIT))/100;*/

		fixedCastTime *= 0.0;
		variableCastTime *= 0.5;
		n_Delay[ksDelayGlobal] = 0.8 + 0.2 * n_A_ActiveSkillLV;
	}
	else if(n_A_ActiveSkill==skill_CR_GRAND_CROSS)
	{ // Grand Cross
		n_PerHIT_DMG = 0;
		var impearialSetBonus = 1.0;
		if ( EquipNumSearch( 1278 ) )
		{ // Imperial Set gives 10% more damage
			impearialSetBonus = 1.1;
		}

		myInnerHtml( "CRIATKname", '<Font color="#FF0000">Health Drain</Font>', 0 );
		var str = '<font color="#FF0000">';
		str += Math.floor( n_A_MaxHP / 5 );
		str += "</font>";
		myInnerHtml( "CRIATK", str, 0 );

		myInnerHtml("CRInumname",'<Font color="#FF0000">Damage Backlash</Font>',0);

		work_A_VITDEF = [0,0,0];
		work_A_VITDEF[0] = n_A_VITDEF;
		work_A_VITDEF[1] = n_A_VITDEF;
		work_A_VITDEF[2] = n_A_VITDEF;
		n_A_INTMDEF = n_A_INT + Math.floor(n_A_VIT /2);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = baseDamage[i] * (100 - n_A_DEF) /100 - work_A_VITDEF[i] + n_A_WeaponLV_seirenATK;
			w_DMG[i] = Math.floor(w_DMG[i] * (w_SkillMod + n_A_ActiveSkillLV * 0.4));

			w = BK_n_A_MATK[i] *(100 - n_A_MDEF)/100 - n_A_INTMDEF;
			w = Math.floor(w * (n_A_ActiveSkillLV * 0.4 +1));

			w_DMG[i] += w;
			w_DMG[i] = Math.floor(w_DMG[i] * (100-n_tok[57]) /100);
			w_DMG[i] = Math.floor(w_DMG[i] * (100-n_tok[66]) /100);
			w_DMG[i] = Math.floor(w_DMG[i] * (100-n_tok[78]) /100);
			if(eval(document.calcForm.A_youshi.checked))
				w_DMG[i] = Math.floor(w_DMG[i] * (100-n_tok[190]) /100);
			else
				w_DMG[i] = Math.floor(w_DMG[i] * (100-n_tok[191]) /100);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_A_BodyZokusei * 10 +1][6]/100);

			w_DMG[i] = Math.floor(w_DMG[i] /2);
		}
		myInnerHtml("CRInum",'<Font color="#FF0000">'+3+ SubName[8][Language] + w_DMG[0] +"~"+ w_DMG[2] +" Damage</Font>",0);

		// Magic Portion
		damageType = kDmgTypeMagic;
		n_A_Weapon_element = ele_HOLY;
		wLAch=1;

		for(var i=0;i<=2;i++)
		{
			w_DMG[i] = baseDamage[i] * defReduction(n_B[en_HARDDEF]) - n_B_DEF2[i] + n_A_WeaponLV_seirenATK;
			w_DMG[i] *= w_SkillMod + n_A_ActiveSkillLV * 0.4;
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_DARK]/100);
			w = BK_n_A_MATK[i] *(100 - n_B[en_HARDMDEF])/100 -n_B_MDEF2;
			w *= (n_A_ActiveSkillLV * 0.4 +1);
			w = Math.floor(w * element[n_B[en_ELEMENT]][ele_DARK]/100);
			w_DMG[i] = tPlusDamCut(Math.floor((w+w_DMG[i])*element[n_B[en_ELEMENT]][ele_DARK]/100));
			w_DMG[i] = Math.floor( w_DMG[i] * impearialSetBonus );
			if(w_DMG[i] < 1)w_DMG[i]=1;
			if(60<=n_B[en_ELEMENT]&&n_B[en_ELEMENT]<=69)w_DMG[i]=0;
		}

		if ( monsterDebuffs[6] === 0 )
		{
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * 3;
				InnStr[i] += Last_DMG_A[i] + " ("+w_DMG[i]+ SubName[8][Language] +"3hit)";
				w_DMG[i] = Last_DMG_A[i];
			}
		}
		else
		{
			for ( var i = 0; i < 3; i++ )
			{
				Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * 4;
				InnStr[i] += Last_DMG_A[i] + " ("+ (w_DMG[i] * 2) +" + " +w_DMG[i]+ SubName[8][Language] +"2hit)";
				w_DMG[i] = Last_DMG_A[i];
			}
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 3.0;
		n_Delay[ksDelayGlobal] = 1.5;
	}
	else if(n_A_ActiveSkill==skill_ME_CART_REVOLUTION)
	{
		wCR = 100;
		n_PerHIT_DMG = Math.floor(ApplyDamageModifiers(0) * 2 * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
		if ( CardNumSearch( 529 ) ) n_PerHIT_DMG *= 1.5; //Heavy Metaling
		n_PerHIT_DMG = Math.floor(n_PerHIT_DMG);
		if(SkillSearch(327)){
			wCR += 20 * SkillSearch(327);
		}
		else
		{
			if(SkillSearch(154))
				wCR += SkillSearch(154) * 5;
			if(SkillSearch(154)==0 && otherBuffs[ksPowerThrust])
				wCR += otherBuffs[ksPowerThrust] * 5 / 10;
		}
		CR_n_A_DMG = [0,0,0];

		CRbai = eval(document.calcForm.SkillSubNum.value) / 8000;
		for( var i=0; i<=2; i++)
			CR_n_A_DMG[i] = Math.floor(n_A_DMG[i] * wCR / 100);

		w_SkillMod += 0.5;
		CalcAtkMods02(w_SkillMod,0);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] += Math.floor(CalcFinalDamage(CR_n_A_DMG[i],i) * CRbai);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_DMG[1] = (w_DMG[1] * w_HIT + ApplyDamageModifiers(0) * 2 *(100-w_HIT))/100;
		w_DMG[1] = Math.floor(w_DMG[1] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if(n_A_ActiveSkill==skill_PA_PRESSURE)
	{
		n_PerHIT_DMG = 0;
		w_DMG[2] = 500 + 300 * n_A_ActiveSkillLV;
		if(n_B[en_BOSS] == 5)
		{
			w_DMG[2] = 1;
		}
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 1.5 + 0.5 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.5 + 0.5 * n_A_ActiveSkillLV;
	}
	else if(n_A_ActiveSkill==skill_PA_MARTYR_RECONING)
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_NEUTRAL;
		w_DMG[2] = Math.floor(n_A_MaxHP * 0.09 * (0.9 + 0.1 * n_A_ActiveSkillLV));
		w_DMG[2] = ApplyDamageModifiers(w_DMG[2]);
		w_DMG[2] = Math.floor(w_DMG[2] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if(n_A_ActiveSkill==skill_MO_OCCULT_IMPACTION)
	{
		n_PerHIT_DMG = 0;
		w_HIT_HYOUJI = 100;
		n_A_Weapon_element = ele_NEUTRAL;
		CalcAtkMods02(w_SkillMod,0);
		w_SkillMod += n_A_ActiveSkillLV *0.75;

		work_B_DEF2 = [0,0,0];
		work_B_DEF2[0] = n_B_DEF2[2];
		work_B_DEF2[1] = n_B_DEF2[1];
		work_B_DEF2[2] = n_B_DEF2[0];

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = Math.floor(Math.floor(baseDamage[i] * w_SkillMod) * (work_B_DEF2[i]+n_B[en_HARDDEF]) /50);
			w_DMG[i] = ApplyDamageModifiers(w_DMG[i]);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0;
		n_Delay[ksDelayGlobal] = 0.5;
	}
	else if ( n_A_ActiveSkill === skill_MO_GUILLOTINE_FIST ||
			  n_A_ActiveSkill === skill_MO_MAX_GUILLOTINE_FIST )
	{
		n_PerHIT_DMG = 0;
		w_HIT_HYOUJI = 100;
		n_A_Weapon_element = ele_NEUTRAL;
		n_tok[bon_ICE_PICK] = 0;
		CalcAtkMods02(w_SkillMod,0);
		n_A_DMG = GetBaseDmg(n_A_Weapon_element, true, 0);
		if(n_A_ActiveSkill==skill_MO_GUILLOTINE_FIST)
			w_SkillMod = 8 + eval(document.calcForm.SkillSubNum.value) /10;
		else
			w_SkillMod = 8 + (n_A_MaxSP-1) /10;
		wASYU = 250 + n_A_ActiveSkillLV * 150;
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = Math.floor(n_A_DMG[i] * w_SkillMod) + wASYU;
			w_DMG[i] = ApplyDamageModifiers(w_DMG[i]);
			w_DMG[i] = tPlusDamCut(w_DMG[i]);
			//w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		fixedCastTime *= (4.5 - 0.5 * n_A_ActiveSkillLV )*0.2;
		variableCastTime *= (4.5 - 0.5 * n_A_ActiveSkillLV)*0.8;
		n_Delay[ksDelayGlobal] = 3.5 - 0.5 * n_A_ActiveSkillLV;
	}
	else if(n_A_ActiveSkill==skill_NIN_THROW_DAGGER)
	{
		damageType = kDmgTypeRanged;
		not_use_card = 1;
		CalcAtkMods02(w_SkillMod,0);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_DMG[1] = (w_DMG[1] * w_HIT + ApplyDamageModifiers(0) * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100 *(100-w_HIT))/100;
		n_PerHIT_DMG = ApplyDamageModifiers(0) * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_NIN_THROW_KUNAI )
	{
		damageType = kDmgTypeRanged;
		not_use_card = 1;
		CalcAtkMods02(w_SkillMod,0);

		n_A_Weapon_element = KunaiOBJ[parseInt(formElements["SkillSubNum"].value)][1];

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			Last_DMG_B[i] = w_DMG[i];
			Last_DMG_A[i] = w_DMG[i] * 3;
			InnStr[i] += Last_DMG_A[i] + " ("+ Last_DMG_B[i] + SubName[8][Language] +"3hit)";
			w_DMG[i] = Last_DMG_A[i];
		}
		var wX = Math.floor(ApplyDamageModifiers(0) * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
		w_DMG[1] = (w_DMG[1] * w_HIT + wX * 3 *(100-w_HIT))/100;
		n_PerHIT_DMG = wX * 3;
		str_PerHIT_DMG = wX * 3 +" (3"+ SubName[8][Language] + wX +" Damage)"

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 2.5;
	}
	else if ( n_A_ActiveSkill === skill_NIN_THROW_HUUMA_SHURIKEN )
	{
		w_SkillMod += (n_A_ActiveSkillLV * 1.5 +0.5);
		damageType = kDmgTypeRanged;
		CalcAtkMods02(w_SkillMod,0);
		wActiveHitNum = 2 + Math.round(n_A_ActiveSkillLV / 2);

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = CalcFinalDamage(n_A_DMG[i],i);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			if(wActiveHitNum > 1)
			{
				w_DMG[i] = Math.floor(w_DMG[i] / wActiveHitNum) * wActiveHitNum;
			}
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
			InnStr[i] += " ("+ (Last_DMG_A[i] / wActiveHitNum) +" x "+ wActiveHitNum +"Hit)";
		}
		w_DMG[1] = (w_DMG[1] * w_HIT + ApplyDamageModifiers(0)* element[n_B[en_ELEMENT]][ele_NEUTRAL]/100 *(100-w_HIT))/100;
		n_PerHIT_DMG = ApplyDamageModifiers(0) * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100;

		fixedCastTime *= 0.0;
		variableCastTime *= 3.0;
		n_Delay[ksDelayGlobal] = 3.0;
	}
	else if ( n_A_ActiveSkill === skill_NIN_KILLING_STRIKE ||
			  n_A_ActiveSkill === skill_NIN_KILLING_STRIKE_MAX )
	{
		// user input
		var remainingHP = 0;
		var numMirrors = parseInt(formElements["SkillSubNum2"].value);

		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_NEUTRAL;
		damageType = kDmgTypeRanged;
		CalcAtkMods02(w_SkillMod,0);
		//CalcAtkMods02( w_SkillMod, 0 );
		if ( n_A_ActiveSkill === skill_NIN_KILLING_STRIKE_MAX )
		{
			remainingHP = n_A_MaxHP - 1;
		} else {
		    remainingHP = parseInt(formElements["SkillSubNum"].value);
		}
		w = ( n_A_Weapon_ATK + weaponUpgradeAttack + equipmentAttack + overrefineAttack + strengthBonusAttack ) * (100 - monsterBuffs[status_en_buff_Elemental]) / 100;

		//w_DMG[0] = (statusAttack + w + n_A_ActiveSkillLV) * 40 + remainingHP * (n_A_BaseLV / 100) * n_A_ActiveSkillLV / 10;
		//w_DMG[0] = ((statusAttack + w) * 17 + remainingHP) * n_A_ActiveSkillLV * 0.1 / 7;
		w_DMG[0] = ((statusAttack + w) * 40) + (remainingHP * ((8 * n_A_ActiveSkillLV) / 100));
		// Mirror Image Modifier
		w_DMG[0] = ApplyEnemyDefense(w_DMG[0],0,0);
		w_DMG[0] *= 1 + (numMirrors * 25 / 100);
		w_DMG[0] = Math.floor(w_DMG[0] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
		w_DMG[0] *= ( 100 - monsterBuffs[status_en_buff_Race] ) / 100;
		w_DMG[0] *= ( rangedMod + 100 ) / 100;
		w_DMG[0] *= weaponSizeMod;

		w_DMG[2] = w_DMG[1] = w_DMG[0];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Math.floor(w_DMG[i]);
			Last_DMG_B[i] = Math.floor(w_DMG[i] / 7);
			InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8][Language] + "7hit)";
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_KAG_OVERTHROW )
	{
		n_PerHIT_DMG = 0;
		not_use_card = 1;
		//CalcAtkMods02(w_SkillMod,0);
		w_DMG[2] = 10000 * n_A_ActiveSkillLV;
		if(n_B[en_BOSS] != 0)
		{
			w_DMG[2] *= 0.5;
		}
		if ( SkillSearch( skill_NIN_DAGGER_THROWING_PRACTICE ) < 10 ) {
			w_DMG[2] *= 0.5;
		}
		w_DMG[2] *= ( -monsterBuffs[status_en_buff_Race] + 100 ) / 100;
		// multiply attack mod
		w_DMG[2] *= weaponElementalMod / 100.0;
		w_DMG[2] *= ( monsterBuffs[status_en_buff_Ranged] + 100 ) / 100.0;
		w_DMG[0] = 0.5 * w_DMG[2];
		w_DMG[1] = 0.75 * w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		w_HIT_HYOUJI = 100;
		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if( n_A_ActiveSkill === skill_KAG_SPINTHROW_KUNAI )
	{
		n_A_Weapon_element = KunaiOBJ[parseInt(formElements["SkillSubNum"].value)][1];
		damageType = kDmgTypeMelee;
		w_SkillMod = ( 0.6 * (5 + n_A_ActiveSkillLV) );
		CalcAtkMods02(w_SkillMod,0);
		if (n_tok[bon_ICE_PICK])
			equipmentAttack -= Math.floor( n_B[en_HARDDEF] / 2 );
		var tempAttack = (equipmentAttack + n_A_Weapon_ATK + weaponUpgradeAttack + overrefineAttack + strengthBonusAttack);
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = w_SkillMod * (tempAttack + statusAttack*2);
			w_DMG[i] = Math.floor(w_DMG[i]);
			w_DMG[i] = tPlusDamCut(Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][n_A_Weapon_element]/100));
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		w_HIT_HYOUJI = 100;
		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill === skill_AL_ACID_TERROR )
	{
		n_PerHIT_DMG = 0;
		not_use_card = 1;
		damageType = kDmgTypeRanged;
		n_A_Weapon_element = ele_NEUTRAL;
		w_SkillMod = (50 + n_A_ActiveSkillLV * 50) /100;

		var baseDamage = GetBaseDmg( n_A_Weapon_element, false, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = Math.floor((baseDamage[i] - n_B_DEF2[i]) * w_SkillMod);
			w_DMG[i] = Math.floor(w_DMG[i] * element[n_B[en_ELEMENT]][ele_NEUTRAL]/100);
			w_DMG[i] = Math.floor(ApplyDamageModifiers(w_DMG[i]));
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if(n_A_ActiveSkill==skill_BC_ACID_DEMONSTRATION)
	{
		n_PerHIT_DMG = 0;
		damageType = kDmgTypeRanged;
		n_A_Weapon_element = ele_NEUTRAL;
		w_TotalHits = n_A_ActiveSkillLV;
		wAD = new Array();
		CalcAtkMods02(w_SkillMod,0);
		var baseWeaponAttack = n_A_Weapon_ATK + strengthBonusAttack + weaponUpgradeAttack;
		var tempAttack = [];//(equipmentAttack + n_A_Weapon_ATK + weaponUpgradeAttack + overrefineAttack + strengthBonusAttack);
		tempAttack[0] = ( baseWeaponAttack - varianceAttack + minOverrefineAttack );// * weaponSizeMod;
		tempAttack[2] = ( baseWeaponAttack + varianceAttack + overrefineAttack );// * weaponSizeMod;
		tempAttack[1] = Math.floor( ( tempAttack[0] + tempAttack[2] ) / 2 );
		//CalcRangedMod();
		//var tempAttack = GetBaseDmg(n_A_Weapon_element, true, 0);
		for ( var i = 0; i < 3; i++ )
		{
			tempAttack[i] += equipmentAttack;
			// multiply race mod
			tempAttack[i] *= ( racialMod + 100 ) / 100;
			// multiply special race mod
			tempAttack[i] *= ( specialRacialMod + 100 ) / 100;
			// multiply size mod
			tempAttack[i] *= ( sizeMod + 100 ) / 100;
			// multiply boss mod
			tempAttack[i] *= ( bossMod + 100 ) / 100;
			if (otherBuffs[ksMagnumBreak])
			tempAttack[i] += tempAttack[i] * 0.2;
			// multiply attack mod
			tempAttack[i] *= attackMod;
			//tempAttack[i] += masteryAttack;

			wAD[i] = (n_A_MATK[i] * 0.07 + 0.07 * (tempAttack[i]) + 0.14 * statusAttack) * n_B[en_VIT];
			w_DMG[i] = Math.floor(wAD[i]);
			w_DMG[i] *= ( rangedMod + 100 ) / 100;
			w_DMG[i] = Math.floor(w_DMG[i] * weaponElementalMod / 100);
			w_DMG[i] = tPlusDamCut(w_DMG[i]);
			if(PlayerVersusPlayer==1)
			{
				w_DMG[i] = Math.floor(w_DMG[i] /2);
			}
		}
		//wAD = 0.7 * n_A_INT * n_A_INT * n_B[en_VIT] / (n_A_INT + n_B[en_VIT]); OLD FORMULA
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_B[i] = w_DMG[i];
			Last_DMG_A[i] = w_DMG[i] * w_TotalHits;
			InnStr[i] += Last_DMG_A[i] + " ("+ Last_DMG_B[i] + SubName[8][Language] +w_TotalHits+"hit)";
			w_DMG[i] = Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill == skill_HU_LAND_MINE ||
			  n_A_ActiveSkill == skill_HU_BLAST_MINE ||
			  n_A_ActiveSkill==skill_HU_CLAYMORE_TRAP )
	{
		n_PerHIT_DMG = 0;
		if(n_A_ActiveSkill==skill_HU_LAND_MINE)
		{
			n_A_Weapon_element = ele_EARTH;
			w_DMG[2] = Math.floor((75 + n_A_DEX) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[en_ELEMENT]][ele_EARTH]/100);
		}
		else if(n_A_ActiveSkill==skill_HU_BLAST_MINE)
		{
			n_A_Weapon_element = ele_WIND;
			w_DMG[2] = Math.floor((50 + n_A_DEX/2) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[en_ELEMENT]][ele_WIND]/100);
		}
		else if(n_A_ActiveSkill==skill_HU_CLAYMORE_TRAP)
		{
			n_A_Weapon_element = ele_FIRE;
			//w_DMG[2] = Math.floor((75 + n_A_DEX/2) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[en_ELEMENT]][ele_FIRE]/100);
			w_DMG[2] = Math.floor((n_A_DEX*(3 + n_A_BaseLV/100) * (1+ n_A_INT /35) + SkillSearch(skill_RAN_RESEARCH_TRAP)) * n_A_ActiveSkillLV * element[n_B[en_ELEMENT]][ele_FIRE]/100);
		}

		w_DMG[2] = tPlusDamCut(w_DMG[2]);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayA] = 1.0;
	}
	else if ( n_A_ActiveSkill == skill_AC_HEAL )
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_HOLY;
		damageType = kDmgTypeMagic;
		w_DMG[2] = CalcHeal( n_A_ActiveSkillLV, 0, 0 );
		w_DMG[2] = Math.floor( Math.floor( w_DMG[2] / 2) * element[n_B[en_ELEMENT]][ele_HOLY] / 100 );
		if ( n_B[en_ELEMENT] < 90 )
		{
			w_DMG[2] = 0;
		}

		var wX = n_tok[170+n_B[en_RACE]];
		w_DMG[2] = Math.floor(w_DMG[2] * (100 + wX) /100);

		wHealBAI = 100 + n_tok[93];
		w_DMG[2] = Math.floor(w_DMG[2] * wHealBAI /100);

		w_DMG[2] = tPlusDamCut(w_DMG[2]);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill == skill_PR_SANCTUARY )
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_HOLY;
		damageType = kDmgTypeMagic;
		if(n_A_ActiveSkillLV <= 6)
			w_DMG[2] = 100 * n_A_ActiveSkillLV;
		else
			w_DMG[2] = 777;

		w_HEAL_BAI = 100 + n_tok[bon_SANC_MUL];
		w_DMG[2] = Math.floor(w_DMG[2] * w_HEAL_BAI / 100);

		w_DMG[2] = Math.floor(Math.floor(w_DMG[2] / 2) * element[n_B[en_ELEMENT]][ele_DARK]/100);
		if(n_B[en_ELEMENT] < 90 && n_B[en_RACE] != 6)
			w_DMG[2]=0;

		var wX = n_tok[170+n_B[en_RACE]];
		w_DMG[2] = Math.floor(w_DMG[2] * (100 + wX) /100);

		w_HEAL_BAI = 100 + n_tok[96];
		w_DMG[2] = Math.floor(w_DMG[2] * w_HEAL_BAI / 100);

		w_DMG[2] = tPlusDamCut(w_DMG[2]);
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		variableCastTime *= 5.0;
		n_Delay[ksDelayA] = 1.0;
	}
	else if ( n_A_ActiveSkill==skill_PR_TURN_UNDEAD || n_A_ActiveSkill==skill_PR_RESSURECTION )
	{
		n_PerHIT_DMG = 0;
		if ( n_A_ActiveSkill == skill_PR_TURN_UNDEAD )
		{
			n_A_Weapon_element = ele_HOLY;
		}
		else
		{
			n_A_Weapon_element = ele_NEUTRAL;
		}
		damageType = kDmgTypeMagic;
		if ( n_B[en_ELEMENT] < 90 )
		{
			w = 0;
			w_DMG[2] = 0;
			w_DMG[0] = 0;
			w_DMG[1] = 0;
		}
		else
		{
			if(n_B[en_BOSS] != 1)
			{
				w = (20 * n_A_ActiveSkillLV + n_A_BaseLV + n_A_INT +n_A_LUK)/1000;
				w_DMG[2] = n_B[en_HP];
			}
			else
			{
				w = 0;
				w_DMG[2] = 0;
			}
			w_DMG[0] = n_A_BaseLV + n_A_INT + n_A_ActiveSkillLV *10;
			w_DMG[0] = Math.floor(w_DMG[0] * element[n_B[en_ELEMENT]][n_A_Weapon_element]/100);
			w_DMG[1] = Math.round((n_B[en_HP] * w + w_DMG[0] * (100-w)/100));
		}
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
		}
		InnStr[0] += w_DMG[0] +" (Damage on Failure)";
		InnStr[1] += w_DMG[1] +" (Considering the Success Chance)";
		InnStr[2] += Math.floor(w_DMG[2] * element[n_B[en_ELEMENT]][n_A_Weapon_element]/100);
		InnStr[2] += " (" +Math.floor(w *10000)/100 +"% Success Chance)";

		w_HIT_HYOUJI = 100;

		fixedCastTime *= 0.0;
		if ( n_A_ActiveSkill == skill_PR_TURN_UNDEAD )
		{
			variableCastTime *= 1.0;
		}
		else
		{
			variableCastTime *= 8.0 - 2.0 * n_A_ActiveSkillLV;
		}
		n_Delay[ksDelayGlobal] = 3.0;
	}
	else if(n_A_ActiveSkill==skill_HW_GRAVITY_FIELD)
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_NEUTRAL;
		damageType = kDmgTypeMagic;
		w_TotalHits = 2 * (n_A_ActiveSkillLV + 4);
		w_DMG[2] = 500 + 100 * n_A_ActiveSkillLV;

		w_DMG[2] = Math.floor(w_DMG[2]);

		if(n_B[en_BOSS] == 5)
			w_DMG[2] = 1;
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * w_TotalHits;
			w_DMG[i] = Last_DMG_A[i]
		}
		var wStrG = Last_DMG_A[0] +" ("+ (w_DMG[0] / w_TotalHits) +" x "+ w_TotalHits +"hit)"
		for ( var i = 0; i < 3; i++ )
		{
			InnStr[i] += wStrG;
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 5.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 2.0;
		n_Delay[ksDelaySkillDuration] = 4 + n_A_ActiveSkillLV;
	}
	else if ( n_A_ActiveSkill === skill_SOR_VARETYR_SPEAR )
	{
		// Local variables
		var strikingLevel = parseInt(formElements["SkillSubNum"].value);
		var endowLevel = parseInt(formElements["SkillSubNum2"].value);
		var physicalDamage = new Array();
		var magicDamage = new Array();
		n_A_Weapon_element = ele_WIND;

		// ATK [{( Striking Level x 50 ) + ( Varetyr Spear Skill Level x 50 )} x Caster s Base Level / 100 ] %
		myInnerHtml( "CRIATKname", "Physical damage", 0 );
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		w_SkillMod = ( ( strikingLevel * 0.5 ) + ( n_A_ActiveSkillLV * 0.5 ) ) * n_A_BaseLV / 100.0;
		for ( var i = 0; i < 3; i++ )
		{ // Apply Enemy Defense
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
		}
		myInnerHtml( "CRIATK", physicalDamage[0] + "-" + physicalDamage[2], 0 );

		// + MATK [{( Endow Tornado skill level x 50 ) + ( Caster s INT x Varetyr Spear Skill level )} x Caster s Base Level / 100 ] %
		myInnerHtml( "CRInumname", "Magical damage", 0 );
		w_SkillMod = ( ( endowLevel * 0.5 ) + ( n_A_ActiveSkillLV * n_A_INT / 100.0 ) ) * n_A_BaseLV / 100.0;
		for ( var i = 0; i < 3; i++ )
		{ // Apply Enemy Magic Defence
			magicDamage[i] = CalcMagicDamage( n_A_MATK[i] * w_SkillMod );
		}
		myInnerHtml( "CRInum", magicDamage[0] + "-" + magicDamage[2], 0 );

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i] + magicDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] ) + " ("
			InnStr[i] += Math.floor( physicalDamage[i] ) + "-";
			InnStr[i] += Math.floor( magicDamage[i] ) + ")";
		}

		// Calculate Cast Time
		fixedCastTime *= 2.0 - 0.2 * n_A_ActiveSkillLV;
		variableCastTime *= 2.0 + 0.2 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 2.0;
	}
	else if ( n_A_ActiveSkill === skill_MIWA_REVERBERATION )
	{
		// Local variables
		var numReverbs = parseInt(formElements["SkillSubNum"].value);
		var numEnemies = parseInt(formElements["SkillSubNum2"].value);
		var physicalDamage = new Array();
		var magicDamage = new Array();

		// Calculate Physical portion
		myInnerHtml( "CRIATKname", "Physical damage per Reverb", 0 );
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		w_SkillMod = ( n_A_ActiveSkillLV + 3 ) * n_A_BaseLV / 100.0;
		for ( var i = 0; i < 3; i++ )
		{ // Apply Enemy Defense
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
		}
		myInnerHtml( "CRIATK", physicalDamage[0] + "-" + physicalDamage[2], 0 );

		// Calculate Magical portion
		myInnerHtml( "CRInumname", "Magical damage per Reverb", 0 );
		w_SkillMod = ( n_A_ActiveSkillLV + 1.0 ) * n_A_BaseLV / 100.0;
		for ( var i = 0; i < 3; i++ )
		{ // Apply Enemy Magic Defence
			magicDamage[i] = CalcMagicDamage( n_A_MATK[i] * w_SkillMod );
		}
		myInnerHtml( "CRInum", magicDamage[0] + "-" + magicDamage[2], 0 );

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = ( physicalDamage[i] + magicDamage[i] ) * numReverbs / numEnemies;
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] ) + " ("
			InnStr[i] += Math.floor( physicalDamage[i] * numReverbs / numEnemies ) + "-";
			InnStr[i] += Math.floor( magicDamage[i] * numReverbs / numEnemies ) + ")";
		}

		// Cast Time
		fixedCastTime *= 0.0;
		variableCastTime *= 1.0 + 0.1 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 0.0;
	}
	else if ( n_A_ActiveSkill === skill_MIWA_SEVERE_RAINSTORM )
	{
		// Local variables
		var numHits = 12;
		var physicalDamage = new Array();

		w_SkillMod = ( ( n_A_DEX + n_A_AGI ) * n_A_ActiveSkillLV / 5 ) / 100.0;
		w_SkillMod *= n_A_BaseLV / 100.0;
		CalcAtkMods02( w_SkillMod, 0 );

		// Calculate and report damage
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = CalcFinalDamage( n_A_DMG[i], i );
			w_DMG[i] = physicalDamage[i] * numHits;
			InnStr[i] = Math.floor( w_DMG[i] ) + " (" + physicalDamage[i] + " x " + numHits + ")";
		}

		// Calculate Cast Time
		fixedCastTime *= 0.5;
		variableCastTime *= 1.0 + 0.5 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 4.5 + 0.5 * n_A_ActiveSkillLV;
	}
	else if ( n_A_ActiveSkill === skill_RUN_DRAGON_BREATH  || n_A_ActiveSkill === skill_RUN_DRAGON_BREATH_WATER)
	{ // Dragon Breath

		// Local variables
		var dragonTrainingMod = 0;
		var elementalMod = 0;
		var baseDamage = 0;
		var totalDamage = 0;
		var currentHP = parseInt(formElements["SkillSubNum"].value);
    damageType = kDmgTypeRanged;
		if ( currentHP > n_A_MaxHP )
		{
			currentHP = n_A_MaxHP;
			formElements["SkillSubNum"].value = n_A_MaxHP;
		}
		else if ( currentHP < 1 )
		{
			currentHP = 1;
			formElements["SkillSubNum"].value = 1;
		}
		w_HIT_HYOUJI = 100;
		if (n_A_ActiveSkill === skill_RUN_DRAGON_BREATH) {
		    n_A_Weapon_element = ele_FIRE;
		} else if (n_A_ActiveSkill === skill_RUN_DRAGON_BREATH_WATER) {
		    n_A_Weapon_element = ele_WATER;
		}

		CalcElementalMod( n_A_Weapon_element );

		// Elemental Mod for fire element
		elementalMod = weaponElementalMod / 100.0;
		// Dragon Breath damage bonus: (95 + 5 * Skill Level) %
		dragonTrainingMod = ( 95 + ( 5 * SkillSearch( skill_RUN_DRAGON_TRAINING ) ) ) / 100.0;
		// Damage = (currentHP / 50 + MSP / 4) * (SkillLevel * BaseLvl / 150) * DragonTrainingBonus
		baseDamage = ( ( currentHP / 50.0 ) + ( n_A_MaxSP / 4.0 ) ) * ( n_A_ActiveSkillLV * n_A_BaseLV / 150.0 );
		// Total Damage
		if ( SkillSearch( skill_RUN_DRAGON_TRAINING ) > 0 )
		{
			totalDamage = Math.floor( baseDamage * dragonTrainingMod * elementalMod );
		}
    totalDamage *= ( rangedMod + 100 ) / 100;
    totalDamage =  Math.floor( totalDamage );
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = totalDamage;
			w_DMG[i] = ApplyDamageModifiers( w_DMG[i] );
			//w_DMG[i] = ApplyEnemyDefense( w_DMG[i], i, 0 ); ***ignores defense?
			InnStr[i] = w_DMG[i];
		}

		// Cast and Delay time
		fixedCastTime *= 0.5;
		if ( n_A_ActiveSkillLV >= 4 && n_A_ActiveSkillLV <= 6 )
		{
			variableCastTime *= 1.0;
		}
		else if ( n_A_ActiveSkillLV >= 7 && n_A_ActiveSkillLV <= 8 )
		{
			variableCastTime *= 1.5;
		}
		else if ( n_A_ActiveSkillLV >= 9 && n_A_ActiveSkillLV <= 10 )
		{
			variableCastTime *= 2.0;
		}
		n_Delay[ksDelayGlobal] = 2.0;
	}
	else if ( n_A_ActiveSkill === skill_RAN_FIRING_TRAP )
	{ // Fire Trap
		n_A_Weapon_element = ele_FIRE; // Always fire
		w_HIT_HYOUJI = 100; // doesn't miss
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// ATK 100% + [{(Fire Trap Skill Level x Caster s DEX) + (INT x 5)} x (1.5 + Caster s Base Level / 100)] x {(Trap Research Skill Level x 20) / 100 }
		w_SkillMod = ( ( n_A_ActiveSkillLV * n_A_DEX ) + ( n_A_INT * 5 ) ) / 100.0;
		w_SkillMod *= 1.5 + n_A_BaseLV / 100.0;
		w_SkillMod *= SkillSearch( skill_RAN_RESEARCH_TRAP ) * 20 / 100.0;

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = n_A_DMG[i] * w_SkillMod;
			w_DMG[i] = ApplyDamageModifiers( n_A_DMG[i] * w_SkillMod );
			w_DMG[i] = ApplyEnemyDefense( w_DMG[i], i, 0 );
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
			InnStr[i] += Last_DMG_A[i];
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_RAN_ICEBOUND_TRAP )
	{ // Ice Trap
		n_A_Weapon_element = ele_WATER; // Always water
		w_HIT_HYOUJI = 100; // doesn't miss
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// ATK 100% + [{(Ice Trap Skill Level x Caster s DEX) + (INT x 5)} x (1.5 + Caster s Base Level / 100)] x {(Trap Research Skill Level x 20) / 100}
		w_SkillMod = ( ( n_A_ActiveSkillLV * n_A_DEX ) + ( n_A_INT * 5 ) ) / 100.0;
		w_SkillMod *= 1.5 + n_A_BaseLV / 100.0;
		w_SkillMod *= SkillSearch( skill_RAN_RESEARCH_TRAP ) * 20 / 100.0;

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = ApplyDamageModifiers( n_A_DMG[i] * w_SkillMod );
			w_DMG[i] = ApplyEnemyDefense( w_DMG[i], i, 0 );
			InnStr[i] = w_DMG[i];
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_MIWA_SOUND_OF_DESTRUCTION )
	{ // Destruction Song
		// Always neutral
		n_A_Weapon_element = ele_NEUTRAL;

		// doesn't miss
		w_HIT_HYOUJI = 100;

		var totalDamage = ( n_A_ActiveSkillLV * 1000 ) + ( n_A_INT * SkillSearch( skill_MIWA_VOICE_LESSONS ) );

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = totalDamage;
			InnStr[i] = w_DMG[i];
		}

		fixedCastTime *= 0.5;
		variableCastTime *= -0.5 + 0.5 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 5 + n_A_ActiveSkillLV;
	}
	else if ( n_A_ActiveSkill === skill_GLT_POISON_LEECH_END )
	{
		// Always neutral
		n_A_Weapon_element = ele_NEUTRAL;

		// doesn't miss
		w_HIT_HYOUJI = 100;

		// Damage per second = {Target VIT x (New Poison Research Skill Level - 3)} + (Target HP/100)
		var totalDamage = ( n_B[en_VIT] * ( SkillSearch( skill_GLT_RESEARCH_NEW_POISON ) - 3 ) ) + ( n_B[en_HP] / 100 );
		totalDamage = Math.floor( totalDamage );

		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = totalDamage;
			InnStr[i] = w_DMG[i];
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill === skill_ROY_SHIELD_PRESS )
	{
		var shieldWeight = ItemOBJ[n_A_Equip[eq_SHIELD]][itm_WEIGHT];
		var physicalDamage = new Array();
		var imperialShieldBonus = 1.0;
		if ( EquipNumSearch( 1277 ) )
		{ // Imperial Shield gives 20% more damage
			imperialShieldBonus = 1.2;
			if ( n_A_LEFT_DEF_PLUS >= 7 )
			{
				imperialShieldBonus += 0.02 * ( n_A_LEFT_DEF_PLUS - 6 );
			}
		}
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// ATK [{(Skill Level x 150) + Caster s STR + Shield Weight} x Caster s Base Level / 100] % + (Caster s VIT x Shield upgrade level)
		w_SkillMod = ( ( n_A_ActiveSkillLV * 1.5 ) + ( ( n_A_STR + shieldWeight ) / 100.0 ) ) * n_A_BaseLV / 100.0;
		w_SkillMod *= imperialShieldBonus;
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
			physicalDamage[i] += n_A_VIT * n_A_LEFT_DEF_PLUS;
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayCooldown] = 2.0;
	}
	else if ( n_A_ActiveSkill === skill_ROY_RAY_OF_GENESIS )
	{
		// Local variables
		var physicalDamage = new Array();
		var magicDamage = new Array();
		var numRoyalGuards = SkillSearch( skill_ROY_NUM_GUARDS );
		var impearialSetBonus = 1.0;
		n_A_Weapon_element = ele_HOLY;
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		if ( EquipNumSearch( 1278 ) )
		{ // Imperial Set gives 10% more damage
			impearialSetBonus = 1.1;
		}

		// Calculate Physical portion
		// ATK [{(Skill Level x 300) + 300} x Caster s Base Level / 100] %
		myInnerHtml( "CRIATKname", "Physical damage of RoG", 0 );
		w_SkillMod = ( ( n_A_ActiveSkillLV + 3 ) + 3 ) * n_A_BaseLV / 100.0;
		w_SkillMod *= impearialSetBonus;
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = Math.floor( ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 ) );
		}
		myInnerHtml( "CRIATK", physicalDamage[0] + "-" + physicalDamage[2], 0 );

		// Calculate Magical portion
		// MATK [{(Skill Level x 300) + (# of Royal Guard in Banding status x 200)} x Caster s Job Level / 25] %
		myInnerHtml( "CRInumname", "Magical damage of RoG", 0 );
		w_SkillMod = ( n_A_ActiveSkillLV + 3 ) + ( numRoyalGuards * 2.0 );
		w_SkillMod *= impearialSetBonus;
		// Calc Raw Damage
		for ( var i = 0; i < 3; i++ )
		{
			magicDamage[i] = n_A_MATK[i] * w_SkillMod;
			magicDamage[i] *= n_A_JobLV / 25.0;
			magicDamage[i] = CalcMagicDamage( magicDamage[i] );
		}
		myInnerHtml( "CRInum", magicDamage[0] + "-" + magicDamage[2], 0 );

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = ( physicalDamage[i] + magicDamage[i] );
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] ) + " ("
			InnStr[i] += Math.floor( physicalDamage[i] ) + "+";
			InnStr[i] += Math.floor( magicDamage[i] ) + ")";
		}

		fixedCastTime *= 0.5;
		variableCastTime *= 1.5 + 0.5 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 2.0;
		n_Delay[ksDelayCooldown] = 5.0;
	}
	else if ( n_A_ActiveSkill === skill_MEC_FAW_SILVER_SNIPER )
	{
		n_A_Weapon_element = ele_NEUTRAL;
		var physicalDamage = new Array();

		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = 200 * n_A_ActiveSkillLV;
			physicalDamage[i] = ApplyEnemyDefense( physicalDamage[i], i, 0 );
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 2.0 - 0.2 * n_A_ActiveSkillLV;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_MEC_FAW_MAGIC_DECOY )
	{
		var magicDamage = new Array();

		// Calculate Magical portion
		for ( var i = 0; i < 3; i++ )
		{
			magicDamage[i] = n_A_ActiveSkillLV * 50 + 250;
			magicDamage[i] = CalcMagicDamage( magicDamage[i] );
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = magicDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 2.0 - 0.2 * n_A_ActiveSkillLV;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_MEC_SELF_DESTRUCTION )
	{ // Suicidal Destruction
		var currentHP = parseInt(formElements["SkillSubNum"].value);
		if ( currentHP > n_A_MaxHP )
		{
			currentHP = n_A_MaxHP;
			formElements["SkillSubNum"].value = n_A_MaxHP;
		}
		else if ( currentHP < 1 )
		{
			currentHP = 1;
			formElements["SkillSubNum"].value = 1;
		}
		var currentSP = parseInt(formElements["SkillSubNum2"].value);
		if ( currentSP > n_A_MaxSP )
		{
			currentSP = n_A_MaxSP;
			formElements["SkillSubNum2"].value = n_A_MaxSP;
		}
		else if ( currentSP < 1 )
		{
			currentSP = 1;
			formElements["SkillSubNum2"].value = 1;
		}
		var physicalDamage = new Array();
		var skillMod = n_A_ActiveSkillLV + 1;
		var remodelMod = SkillSearch( skill_MEC_MAINFRAME_RESTRUCTURE ) + 8.0;
		var spMod = currentSP + n_A_VIT;
		var baseLevelMod = n_A_BaseLV / 100.0;

		// [{(Skill Level + 1) x (Remodel Mainframe Skill Level + 8 ) x
		// (SP Used + Caster s VIT)} x Caster s Base Level / 100] + (Caster s Current HP)
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ( skillMod * remodelMod * spMod ) * baseLevelMod + currentHP;
			physicalDamage[i] = ApplyEnemyDefense( physicalDamage[i], i, 0 );
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.5;
		variableCastTime *= 2.5 - 0.5 * n_A_ActiveSkillLV;
		n_Delay[ksDelayCooldown] = 300.0;
	}
	else if ( n_A_ActiveSkill === skill_WAR_HELL_INFERNO )
	{
		var fireDamage = new Array();
		var shadowDamage = new Array();

		// (Fire Element) = MATK [{( Skill Level x 300 ) x ( Caster s Base Level / 100 ) /5 }] %
		n_A_Weapon_element = ele_FIRE;
		w_SkillMod = ( n_A_ActiveSkillLV * 3.0 ) * n_A_BaseLV / 100.0 / 5.0;
		for ( var i = 0; i < 3; i++ )
		{
			fireDamage[i] = n_A_MATK[i] * w_SkillMod;
			fireDamage[i] = CalcMagicDamage( fireDamage[i] );
		}
		myInnerHtml( "CRIATKname", "Fire portion of damage", 0 );
		myInnerHtml( "CRIATK", fireDamage[0] + "-" + fireDamage[2], 0 );

		// (Shadow Element) = MATK [{( Skill Level x 300 ) x ( Caster s Base Level / 100 ) x 4/5 }] %
		n_A_Weapon_element = ele_DARK;
		w_SkillMod = ( n_A_ActiveSkillLV * 3.0 ) * n_A_BaseLV / 100.0 * 4.0 / 5.0;
		for ( var i = 0; i < 3; i++ )
		{
			shadowDamage[i] = n_A_MATK[i] * w_SkillMod;
			shadowDamage[i] = CalcMagicDamage( shadowDamage[i] );
		}
		myInnerHtml( "CRInumname", "Shadow portion of damage", 0 );
		myInnerHtml( "CRInum", shadowDamage[0] + "-" + shadowDamage[2], 0 );

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = fireDamage[i] + shadowDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 1.0;
		variableCastTime *= 3.0;
		n_Delay[ksDelayGlobal] = 1.0;
	}
	else if ( n_A_ActiveSkill === skill_SUR_FALLEN_EMPIRE )
	{
		var physicalDamage = new Array();
		var monsterSizeValue = 2;
		if ( n_B[en_SIZE] == siz_MEDIUM )
		{
			monsterSizeValue = 4;
		}
		else if ( n_B[en_SIZE] == siz_LARGE )
		{
			monsterSizeValue = 6;
		}
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// ATK [(Skill Level x 150 + 100) x Caster s Base Level / 150] % + [(Target s Size value + Skill Level - 1) x Caster s STR] + [(Target s current weight x Caster s DEX / 120)]
		// Where, Small = 2 / Medium = 4 / Large = 6
		// On monsters, weight portion is changed to (Monster Level x 50)
		w_SkillMod = ( ( n_A_ActiveSkillLV * 1.5 ) + 1.0 ) * n_A_BaseLV / 150.0;
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
			physicalDamage[i] += ( ( monsterSizeValue + n_A_ActiveSkillLV - 1 ) * n_A_STR ) + ( n_B[en_LEVEL] * 50 * n_A_DEX / 120.0 );
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
	}
	else if ( n_A_ActiveSkill === skill_SUR_TIGER_CANNON )
	{ // Tiger Cannon
		var physicalDamage = new Array();
		var afterFallen = formElements["SkillSubNum"].checked;
		var hpCost = n_A_MaxHP * ( ( 10 + ( 2 * n_A_ActiveSkillLV ) ) / 100.0 );
		var spCost = n_A_MaxSP * ( (  5 + ( 1 * n_A_ActiveSkillLV ) ) / 100.0 );
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// HP Cost: (10 + 2 * Skill Level) %
		// SP Cost: (5 + 1 * Skill Level) %
		// ATK [((Caster s consumed HP + SP) / 4) x Caster s Base Level / 100] %
		// + (Tiger Cannon skill level x 240) + (Target s Base Level x 40)
		// If used as a combo skill after Fallen Empire,
		// ATK [((Caster s consumed HP + SP) / 2) x Caster s Base Level / 100] %
		// + (Tiger Cannon skill level x 500) + (Target s Base Level x 40)
		if ( afterFallen )
		{
			w_SkillMod = ( ( ( hpCost + spCost ) / 2 ) / 100.0 ) * n_A_BaseLV / 100.0;
		}
		else
		{
			w_SkillMod = ( ( ( hpCost + spCost ) / 4 ) / 100.0 ) * n_A_BaseLV / 100.0;
		}
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
			if ( afterFallen )
			{
				physicalDamage[i] += ( n_A_ActiveSkillLV * 500 ) + ( n_B[en_LEVEL] * 40 );
			}
			else
			{
				physicalDamage[i] += ( n_A_ActiveSkillLV * 240 ) + ( n_B[en_LEVEL] * 40 );
			}
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 1.0 + 0.1 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 5.0;
	}
	else if ( n_A_ActiveSkill === skill_SUR_GATE_OF_HELL )
	{ // Gate of Hell
		var afterCombo = formElements["SkillSubNum"].checked;
		var currentHP = parseInt(formElements["SkillSubNum2"].value);
		var currentSP = parseInt(formElements["SkillSubNum3"].value);
		var physicalDamage = new Array();
		var spMod = 1 + 0.2 * n_A_ActiveSkillLV;
		w_HIT_HYOUJI = 100; // never misses?
		n_A_Weapon_element = ele_NEUTRAL;
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );
		if ( n_A_ActiveSkillLV > 4 )
		{
			damageType = kDmgTypeRanged;
		}
		if ( currentHP > n_A_MaxHP )
		{
			currentHP = n_A_MaxHP;
			formElements["SkillSubNum2"].value = n_A_MaxHP;
		}
		else if ( currentHP < 1 )
		{
			currentHP = 1;
			formElements["SkillSubNum2"].value = 1;
		}
		if ( currentSP > n_A_MaxSP )
		{
			currentSP = n_A_MaxSP;
			formElements["SkillSubNum3"].value = n_A_MaxSP;
		}
		else if ( currentSP < 1 )
		{
			currentSP = 1;
			formElements["SkillSubNum3"].value = 1;
		}

		// SP Damage modifier: (1 + 0.2 * Skill Level)
		// ATK [(Skill Level x 500) x Caster s Base Level / 100] % + bonusDamage
		// If used as a combo skill,
		// ATK [(Skill Level x 800) x Caster s Base Level / 100] % + bonusDamage
		if ( afterCombo )
		{
			w_SkillMod = n_A_ActiveSkillLV * 8.0 * n_A_BaseLV / 100.0;
		}
		else
		{
			w_SkillMod = n_A_ActiveSkillLV * 5.0 * n_A_BaseLV / 100.0;
		}
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
			if ( afterCombo )
			{ // (Caster s Max HP - Caster s Current HP) + (Caster s Max SP x Damage Modifier) + (Caster s Base Level x40
				physicalDamage[i] += ( n_A_MaxHP - currentHP ) + ( n_A_MaxSP * spMod ) + ( n_A_BaseLV * 40 );
			}
			else
			{ // (Caster s Max HP - Caster s Current HP) + (Caster s SP x Damage modifier) + (Caster s Base Level x 10
				physicalDamage[i] += ( n_A_MaxHP - currentHP ) + ( currentSP * spMod ) + ( n_A_BaseLV * 10 );
			}
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.8 + 0.2 * n_A_ActiveSkillLV;
		n_Delay[ksDelayGlobal] = 0.1 * n_A_ActiveSkillLV;
	}
	else if ( n_A_ActiveSkill === skill_SUR_CRESCENT_ELBOW )
	{ // Crescent Elbow
		var damageTaken = parseInt(formElements["SkillSubNum"].value);
		var physicalDamage = new Array();
		n_A_DMG = GetBaseDmg( n_A_Weapon_element, false, 0 );

		// ATK [{(Target s HP / 100) x Skill Level} x Caster s Base Level / 125] %
		// + [Received damage x {1 + (Skill Level x 0.2)}]
		// (Maximum of 5000% ATK)
		// Caster receives 10% of the damage
		w_SkillMod = ( ( ( n_B[en_HP] / 100.0 ) / 100.0 ) * n_A_ActiveSkillLV ) * n_A_BaseLV / 125.0;
		if ( w_SkillMod > 50.0 )
		{
			w_SkillMod = 50.0;
		}
		//CalcAtkMods02( w_SkillMod, 0 );
		for ( var i = 0; i < 3; i++ )
		{
			physicalDamage[i] = ApplyEnemyDefense( n_A_DMG[i] * w_SkillMod, i, 0 );
			physicalDamage[i] += damageTaken * ( 1 + ( n_A_ActiveSkillLV * 0.2 ) );
		}

		// post damage to form
		for ( var i = 0; i < 3; i++ )
		{
			w_DMG[i] = physicalDamage[i];
			Last_DMG_A[i] = w_DMG[i];
			InnStr[i] += Math.floor( Last_DMG_A[i] );
		}

		fixedCastTime *= 0.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 1.0;
		n_Delay[ksDelayCooldown] = 5.0;
	}
	else if(n_A_ActiveSkill==skill_MEC_LAVA_FLOW)
	{
		n_PerHIT_DMG = 0;
		n_A_Weapon_element = ele_FIRE;
		damageType = kDmgTypeMelee;
		w_TotalHits = 5;
		w_DMG[2] = 1200 + 400 * n_A_ActiveSkillLV;

		w_DMG[2] = Math.floor(w_DMG[2]);

		if(n_B[en_BOSS] == 5)
			w_DMG[2] = 1;
		w_DMG[0] = w_DMG[1] = w_DMG[2];
		for ( var i = 0; i < 3; i++ )
		{
			Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * w_TotalHits;
			w_DMG[i] = Last_DMG_A[i]
		}
		var wStrG = Last_DMG_A[0] +" ("+ (w_DMG[0] / w_TotalHits) +" x "+ w_TotalHits +"hit)"
		for ( var i = 0; i < 3; i++ )
		{
			InnStr[i] += wStrG;
		}
		w_HIT_HYOUJI = 100;

		fixedCastTime *= 5.0;
		variableCastTime *= 0.0;
		n_Delay[ksDelayGlobal] = 2.0;
		n_Delay[ksDelaySkillDuration] = 4 + n_A_ActiveSkillLV;
	}
	// Magic Skills ----------------------------------
	else
	{
		// local variables for magic calculation
		n_PerHIT_DMG = 0;
		damageType = kDmgTypeMagic;
		w_SkillMod = 1;
		n_subHits = 0;

		// Monster Skills
		if ( n_A_ActiveSkill == skill_MON_DARK_STRIKE )
		{
			n_A_Weapon_element = ele_DARK;
			w_TotalHits = Math.round(n_A_ActiveSkillLV / 2);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			if ( n_A_ActiveSkillLV % 2 == 0 )
			{
				n_Delay[ksDelayGlobal] = 0.8 + n_A_ActiveSkillLV / 2 * 0.2;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 1 + ( n_A_ActiveSkillLV + 1 ) / 2 * 0.2;
			}
		}
		// Mage Skills
		else if ( n_A_ActiveSkill == skill_MA_FIRE_BOLT )
		{
			n_A_Weapon_element = ele_FIRE;
			w_TotalHits = n_A_ActiveSkillLV;

			var cast = 0.4 + 0.4 * n_A_ActiveSkillLV;
			fixedCastTime *= 0.2 * cast;
			variableCastTime *= 0.8 * cast;
			n_Delay[ksDelayGlobal] = 0.8 + 0.2 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill == skill_MA_COLD_BOLT )
		{
			n_A_Weapon_element = ele_WATER;
			w_TotalHits = n_A_ActiveSkillLV;
			if ( EquipNumSearch( 1387 ) ) {
				w_SkillMod = 1 + 0.03*n_A_Weapon_ATKplus;
			}
			var cast = 0.4 + 0.4 * n_A_ActiveSkillLV;
			fixedCastTime *= 0.2 * cast;
			variableCastTime *= 0.8 * cast;
			n_Delay[ksDelayGlobal] = 0.8 + 0.2 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill == skill_MA_LIGHTNING_BOLT )
		{
			n_A_Weapon_element = ele_WIND;
			w_TotalHits = n_A_ActiveSkillLV;

			var cast = 0.4 + 0.4 * n_A_ActiveSkillLV;
			fixedCastTime *= 0.2 * cast;
			variableCastTime *= 0.8 * cast;
			n_Delay[ksDelayGlobal] = 0.8 + 0.2 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill == skill_MA_FIRE_BALL )
		{
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = 1.4 + n_A_ActiveSkillLV * 0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.6;
			if(n_A_ActiveSkillLV <=5)
			{
				n_Delay[ksDelayGlobal] = 1.5;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 1;
			}
		}
		else if ( n_A_ActiveSkill == skill_MA_FIRE_WALL )
		{
			n_A_Weapon_element = ele_FIRE;
			w_TotalHits = 4 + n_A_ActiveSkillLV;
			w_SkillMod = 0.5;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.2 - 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 0.1;
		}
		else if ( n_A_ActiveSkill == skill_MA_FROST_DIVER )
		{
			n_A_Weapon_element = ele_WATER;
			w_SkillMod = 1 + n_A_ActiveSkillLV * 0.1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.8;
			n_Delay[ksDelayGlobal] = 1.5;
		}
		else if ( n_A_ActiveSkill == skill_MA_THUNDER_STORM )
		{
			n_A_Weapon_element = ele_WIND;
			w_TotalHits = n_A_ActiveSkillLV;
			w_SkillMod = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.8 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 2.0;
		}
		else if ( n_A_ActiveSkill == skill_MA_NAPALM_BEAT )
		{
			n_A_Weapon_element = ele_GHOST;
			w_SkillMod = 0.7 + n_A_ActiveSkillLV * 0.1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			if( n_A_ActiveSkillLV == 10 )
			{
				n_Delay[ksDelayGlobal] = 0.5;
			}
			else if ( n_A_ActiveSkillLV == 9 )
			{
				n_Delay[ksDelayGlobal] = 0.6;
			}
			else if ( n_A_ActiveSkillLV == 8 )
			{
				n_Delay[ksDelayGlobal] = 0.7;
			}
			else if ( n_A_ActiveSkillLV >= 6 )
			{
				n_Delay[ksDelayGlobal] = 0.8;
			}
			else if ( n_A_ActiveSkillLV >= 4 )
			{
				n_Delay[ksDelayGlobal] = 0.9;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 1;
			}
		}
		else if ( n_A_ActiveSkill == skill_MA_SOUL_STRIKE )
		{
			n_A_Weapon_element = ele_GHOST;
			w_TotalHits = Math.round(n_A_ActiveSkillLV / 2);

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5;
			if(n_A_ActiveSkillLV % 2 == 0)
			{
				n_Delay[ksDelayGlobal] = 0.8 + n_A_ActiveSkillLV / 2 *0.2;
			}
			else
			{
				n_Delay[ksDelayGlobal] = 1 + (n_A_ActiveSkillLV+1) / 2 *0.2;
			}
		}
		// Wizard Skills
		else if ( n_A_ActiveSkill == skill_WI_FIRE_PILLAR )
		{
			n_A_Weapon_element = ele_FIRE;
			w_TotalHits = (n_A_ActiveSkillLV +2);
			w_SkillMod = 0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.64 - (0.24 * n_A_ActiveSkillLV);
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill == skill_WI_SIGHTRASHER )
		{
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = 1 + n_A_ActiveSkillLV * 0.2;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.4;
			n_Delay[ksDelayGlobal] = 2.0;
		}
		else if ( n_A_ActiveSkill == skill_WI_METEOR_STORM )
		{
			n_A_Weapon_element = ele_FIRE;
			w_TotalHits = Math.floor((n_A_ActiveSkillLV+1) / 2);
			w_SkillMod = 1.25;

			fixedCastTime *= 0.0;
			variableCastTime *= 12.0;
			n_Delay[ksDelayGlobal] = Math.floor(n_A_ActiveSkillLV / 2) * 1 +2;
		}
		else if ( n_A_ActiveSkill==skill_WI_JUPITEL_THUNDER )
		{
			n_A_Weapon_element = ele_WIND;
			w_TotalHits = n_A_ActiveSkillLV + 2;

			fixedCastTime *= 0.0;
			variableCastTime *= 2 + n_A_ActiveSkillLV * 0.4;
		}
		else if ( n_A_ActiveSkill==skill_WI_LORD_OF_VERMILLION )
		{
			n_A_Weapon_element = ele_WIND;
			w_TotalHits = 4;
			if (n_A_ActiveSkillLV <= 3)
				w_SkillMod = 1.0;
			else if (n_A_ActiveSkillLV <= 6)
				w_SkillMod = 0.4 + n_A_ActiveSkillLV * 0.2;
			else if (n_A_ActiveSkillLV <= 9)
				w_SkillMod = n_A_ActiveSkillLV * 0.4 - 0.8;
			else // lvl10
				w_SkillMod = 3.3;

			fixedCastTime *= 0.0;
			variableCastTime *= 12.4 - n_A_ActiveSkillLV * 0.4;
			n_Delay[ksDelayGlobal] = 5.0;
			n_Delay[ksDelaySkillDuration] = 4.0;
		}
		else if ( n_A_ActiveSkill==skill_WI_WATER_BALL || n_A_ActiveSkill==skill_WATER_BALL2 )
		{
			n_A_Weapon_element = ele_WATER;
			if(n_A_ActiveSkillLV >= 4)
				w_TotalHits = 25
			else if(n_A_ActiveSkillLV >= 2)
				w_TotalHits = 9;
			SG_Special_HITnum = w_TotalHits;
			w_SkillMod = 1 + n_A_ActiveSkillLV * 0.3;

			fixedCastTime *= 0.0;
			variableCastTime *= n_A_ActiveSkillLV;
			n_Delay[ksDelayAnimation] = 0.1 * w_TotalHits;
		}
		else if ( n_A_ActiveSkill==skill_WI_FROST_NOVA )
		{
			n_A_Weapon_element = ele_WATER;
			w_SkillMod = 1.0 + n_A_ActiveSkillLV * 0.10;

			fixedCastTime *= 0.0;
			if (n_A_ActiveSkillLV <= 4)
			{
				variableCastTime *= 0.8 - Math.floor((n_A_ActiveSkillLV-1) /2) * 0.08;
			}
			else
			{
				variableCastTime *= 0.8 - Math.floor((n_A_ActiveSkillLV-2) /2) * 0.08;
			}
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill==skill_WI_STORM_GUST )
		{
			n_A_Weapon_element = ele_WATER;
			w_TotalHits = eval(document.calcForm.SkillSubNum.value);
			SG_Special_HITnum = w_TotalHits;
			w_SkillMod = 0.7 + n_A_ActiveSkillLV * 0.5;

			var cast = 4 + n_A_ActiveSkillLV * 0.8;
			fixedCastTime *= 0.2 * cast;
			variableCastTime *= 0.8 * cast;
			n_Delay[ksDelayGlobal] = 5.0;
			n_Delay[ksDelaySkillDuration] = 4.5;
		}
		else if ( n_A_ActiveSkill == skill_WI_EARTH_SPIKE ||
				  n_A_ActiveSkill == skill_WI_HEAVENS_DRIVE ||
				  n_A_ActiveSkill == skill_HEAVENS_DRIVE2 )
		{
			n_A_Weapon_element = ele_EARTH;
			w_TotalHits = n_A_ActiveSkillLV;
			if ( n_A_ActiveSkill == skill_WI_EARTH_SPIKE )
			{
				w_SkillMod = 1.0;
				var cast = n_A_ActiveSkillLV *0.56;
				fixedCastTime *= 0.2 * cast;
				variableCastTime *= 0.8 * cast;
				n_Delay[ksDelayGlobal] = 0.8 + n_A_ActiveSkillLV * 0.2;
			}
			else
			{
				w_SkillMod = 1.25;

				var cast = n_A_ActiveSkillLV *0.8;
				fixedCastTime *= cast * 0.2;
				variableCastTime *= cast * 0.8;
				n_Delay[ksDelayGlobal] = 1.0;
			}
		}
		// High Wizard Skills
		else if ( n_A_ActiveSkill==skill_HW_NAPALM_VULCAN )
		{
			w_TotalHits = n_A_ActiveSkillLV;
			n_A_Weapon_element = ele_GHOST;
			w_SkillMod = 0.7 + n_A_ActiveSkillLV * 0.1;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		// Acolyte Skills
		else if ( n_A_ActiveSkill==skill_AC_HOLY_LIGHT || n_A_ActiveSkill==skill_PR_HOLY_LIGHT_SL )
		{
			n_A_Weapon_element = ele_HOLY;
			w_SkillMod = 1.25;
			if(n_A_ActiveSkill==skill_PR_HOLY_LIGHT_SL)
				w_SkillMod *= 5;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
		}
		// Priest Skills
		else if ( n_A_ActiveSkill==skill_PR_MAGNUS_EXORCISMUS )
		{
			n_A_Weapon_element = ele_HOLY;
			w_TotalHits = n_A_ActiveSkillLV;
			if(n_B[en_RACE] != 6 && n_B[en_ELEMENT] < 90)
			{
				n_A_MATK[2]=0;n_A_MATK[0]=0;n_A_MATK[1]=0;
			}

			fixedCastTime *= 0.0;
			variableCastTime *= 15.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayGlobal] = 4.0;
		}
		// Soul Linker Skills
		else if ( n_A_ActiveSkill == skill_SL_ESTIN )
		{
			n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
			if(n_B[en_SIZE] == 0)
				w_SkillMod = n_A_ActiveSkillLV * 0.1;
			else
				w_SkillMod = 0.01;
			if(PlayerVersusPlayer==1)
				w_SkillMod = 0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.1;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill == skill_SL_ESTUN )
		{
			n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
			w_SkillMod = n_A_ActiveSkillLV * 0.05;
			if(PlayerVersusPlayer==1)
				w_SkillMod = 0;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.1;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill == skill_SL_ESMA )
		{
			n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
			w_TotalHits = n_A_ActiveSkillLV;
			w_SkillMod = 0.4 + n_A_BaseLV / 100;
			if(PlayerVersusPlayer==1)
				w_SkillMod = 0;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
			n_Delay[ksDelayA] = 1.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		// Ninja Skills
		else if ( n_A_ActiveSkill == skill_NIN_FLAMING_PETALS )
		{
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = 0.9;
			w_TotalHits = n_A_ActiveSkillLV;

			fixedCastTime *= 0.7 * n_A_ActiveSkillLV * 0.2;
			variableCastTime *= 0.7 * n_A_ActiveSkillLV * 0.8;
		}
		else if ( n_A_ActiveSkill == skill_NIN_BLAZE_SHIELD )
		{
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = 0.5;
			w_TotalHits = Math.round(n_A_ActiveSkillLV / 2) +4 ;

			fixedCastTime *= (6.5 - 0.5 * n_A_ActiveSkillLV) * 0.2;
			variableCastTime *= (6.5 - 0.5 * n_A_ActiveSkillLV) * 0.8;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill == skill_NIN_EXPLODING_DRAGON )
		{
			n_subHits = 1;
			n_A_Weapon_element = ele_FIRE;
			w_SkillMod = 1.5 + n_A_ActiveSkillLV * 1.5;
			w_TotalHits = 3;

			fixedCastTime *= 3.0 * 0.2;
			variableCastTime *= 3.0 * 0.8;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else if ( n_A_ActiveSkill == skill_NIN_FREEZING_SPEAR )
		{
			n_A_Weapon_element = ele_WATER;
			w_TotalHits = n_A_ActiveSkillLV + 2;

			fixedCastTime *= n_A_ActiveSkillLV * 0.7 * 0.2;
			variableCastTime *= n_A_ActiveSkillLV * 0.7 * 0.8;
		}
		else if ( n_A_ActiveSkill == skill_NIN_SNOW_FLAKE_DRAFT )
		{
			n_A_Weapon_element = ele_WATER;
			w_SkillMod = 1.0 + n_A_ActiveSkillLV * 0.5;
			w_TotalHits = 1;

			fixedCastTime *= 3.0 * 0.2;
			variableCastTime *= 3.0 * 0.8;
			n_Delay[ksDelayGlobal] = 3.0;
		}
		else if ( n_A_ActiveSkill == skill_NIN_WIND_BLADE )
		{
			n_A_Weapon_element = ele_WIND;
			w_SkillMod = 1.0;
			w_TotalHits = Math.floor(n_A_ActiveSkillLV / 2) +1;

			fixedCastTime *= (Math.floor(n_A_ActiveSkillLV / 2) + 1)*0.2;
			variableCastTime *= (Math.floor(n_A_ActiveSkillLV / 2) + 1)*0.8;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill == skill_NIN_LIGHTNING_JOLT )
		{
			n_A_Weapon_element = ele_WIND;
			w_SkillMod = 1.6 + 0.4 * n_A_ActiveSkillLV;
			w_TotalHits = 1;

			fixedCastTime *= 4.0*0.2;
			variableCastTime *= 4.0*0.8;
		}
		else if ( n_A_ActiveSkill==skill_NIN_FIRST_WIND )
		{
			n_A_Weapon_element = ele_WIND;
			w_SkillMod = 1.0 + n_A_ActiveSkillLV * 1.0;
			w_TotalHits = 1;

			fixedCastTime *= 4.0 * 0.2;
			variableCastTime *= 4.0 * 0.8;
		}
		// Warlock Skills
		else if ( n_A_ActiveSkill == skill_WAR_DRAIN_LIFE )
		{
			n_A_Weapon_element = ele_NEUTRAL;

			// MATK [{( Skill Level x 200 ) + ( Caster s INT ) } x ( Caster s Base Level / 100 )] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 2.0 ) + n_A_INT / 100.0 ) * n_A_BaseLV / 100.0;
			w_TotalHits = 1;

			fixedCastTime *= 1.0;
			variableCastTime *= 4.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_SOUL_EXPANSION )
		{
			var imprisoned = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_GHOST;

			// MATK [{( Skill Level + 4 ) x 100 ) + ( Caster s INT )} x ( Caster s Base Level / 100 )] %
			w_SkillMod = ( ( n_A_ActiveSkillLV + 4 ) + n_A_INT / 100.0 ) * n_A_BaseLV / 100.0;
			w_SkillMod *= imprisoned;
			w_TotalHits = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill == skill_WAR_CRIMSON_ROCK )
		{
			n_A_Weapon_element = ele_FIRE;

			// MATK [{( Skill Level x 300 ) x ( Caster s Base Level / 100 ) + 1300 }] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 3.0 ) * n_A_BaseLV / 100.0 ) + 13.0;
			w_TotalHits = 1;

			fixedCastTime *= 2.0;
			variableCastTime *= 5.0;
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_COMET )
		{
			var distance = parseInt(formElements["SkillSubNum"].value);
			var distanceMod = 10.0 + 5.0 * distance;
			n_A_Weapon_element = ele_NEUTRAL;

			// MATK [{( Skill Level x 400 ) x ( Caster s Base Level / 120 )} + 2500 ] %
			w_SkillMod = ( ( n_A_ActiveSkillLV * 4.0 ) * n_A_BaseLV / 120.0 ) + distanceMod;
			w_TotalHits = 1;

			fixedCastTime *= 0.5 + 0.5 * n_A_ActiveSkillLV;
			variableCastTime *= 9.0 + 1.0 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 60.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_FROST_MISTY )
		{
			n_A_Weapon_element = ele_WATER;

			// MATK [{( Skill Level x 100 ) + 200 } x ( Caster s Base Level / 100 )] %
			w_SkillMod = ( n_A_ActiveSkillLV + 2.0 ) * n_A_BaseLV / 100.0;
			w_TotalHits = 1;

			fixedCastTime *= 0.5;
			variableCastTime *= 1.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 4.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_JACK_FROST )
		{
			var freezing = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_WATER;

			// (Targets not in Freezing status)
			// = MATK [{( Skill Level x 100 ) + 500 } x ( Caster s Base Level / 150 )] %
			// (Targets in Freezing status)
			// = MATK [{( Skill Level x 300 ) + 1000 } x ( Caster s Base Level / 100 )] %
			if ( freezing )
			{
				w_SkillMod = ( n_A_ActiveSkillLV * 3.0 + 10.0 ) * n_A_BaseLV / 100.0;
			}
			else
			{
				w_SkillMod = ( n_A_ActiveSkillLV + 5.0 ) * n_A_BaseLV / 150.0;
			}
			w_TotalHits = 1;

			fixedCastTime *= 1.0;
			variableCastTime *= 1.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_CHAIN_LIGHTNING )
		{
			w_TotalHits = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_WIND;
			w_SkillMod = 0.0;
			// MATK [{( Skill Level x 100 ) + 500 + ( 9 - Number of Bounce ) x 100 } x ( Caster s Base Level / 100 )] %
			for (i = 0; i < w_TotalHits; i++) {
				w_SkillMod += (( n_A_ActiveSkillLV + 5.0 + ( 9.0 - i ) ) * n_A_BaseLV / 100.0);
			}
			w_SkillMod = Math.floor( w_SkillMod / w_TotalHits );
			SG_Special_HITnum = w_TotalHits;

			fixedCastTime *= 1.0;
			variableCastTime *= 3.0 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayCooldown] = 3.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_EARTH_STRAIN )
		{
			n_A_Weapon_element = ele_EARTH;

			// MATK [{( Skill Level x 100 ) + 2000 } x ( Caster s Base Level / 100 )] %
			w_SkillMod = ( n_A_ActiveSkillLV + 20.0 ) * n_A_BaseLV / 100.0;
			w_TotalHits = 1;

			fixedCastTime *= 2.0;
			variableCastTime *= 1.0 + 1.0 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 10.0;
		}
		else if ( n_A_ActiveSkill == skill_WAR_TETRA_VORTEX )
		{
			n_A_Weapon_element = parseInt(formElements["SkillSubNum"].value);

			// MATK (500 + 500 * Skill Level) % * 4 hits
			w_SkillMod = ( n_A_ActiveSkillLV * 5.0 + 5.0 );
			w_TotalHits = 4;
			SG_Special_HITnum = w_TotalHits;

			fixedCastTime *= 2.0;
			variableCastTime *= /*8.0 */+ 4.0 + 1.0 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 2.0;
			n_Delay[ksDelayCooldown] = 15.0;
		}
		// Arch Bishop Skills
		else if ( n_A_ActiveSkill == skill_ABI_JUDEX )
		{
			// MATK = [ { (Skill Level x 20 ) + 300 } x Caster's BaseLV / 100 ] %
			n_A_Weapon_element = ele_HOLY;
			w_SkillMod = ( ( n_A_ActiveSkillLV * 0.2 ) + 3.0 ) * n_A_BaseLV / 100.0;
			w_TotalHits = 1;

			fixedCastTime *= 0.5;
			variableCastTime *= 2.0;
			n_Delay[ksDelayGlobal] = 0.5;
		}
		else if ( n_A_ActiveSkill == skill_ABI_ADORAMUS )
		{
			// MATK = [{(Skill Level x 100 ) + 500 } x Caster's BaseLV / 100 ] %
			n_A_Weapon_element = ele_HOLY;
			w_SkillMod = ( ( n_A_ActiveSkillLV * 1.0 ) + 5.0 ) * n_A_BaseLV / 100.0;
			w_TotalHits = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 2.0;
			n_Delay[ksDelayGlobal] = 0.5;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		// Sorcerer Skills
		else if ( n_A_ActiveSkill == skill_SOR_FIRE_WALK || n_A_ActiveSkill == skill_SOR_ELECTRIC_WALK )
		{
			if ( n_A_ActiveSkill === skill_SOR_FIRE_WALK )
			{
				n_A_Weapon_element = ele_FIRE;
			}
			else
			{
				n_A_Weapon_element = ele_WIND;
			}
			// MATK [(60 x Skill Level) x Caster s Base Level / 100] %
			w_SkillMod = ( n_A_ActiveSkillLV * 0.6 ) * n_A_BaseLV / 100.0;
			w_TotalHits = parseInt(formElements["SkillSubNum"].value);
			SG_Special_HITnum = w_TotalHits;

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_EARTH_GRAVE )
		{
			// MATK [{ Skill Level x INT ) + ( Endow Quake Level x 200 )} x Caster s Base Level / 100] %
			n_A_Weapon_element = ele_EARTH;
			w_SkillMod = ( n_A_ActiveSkillLV * ( n_A_INT / 100.0 ) + ( SkillSearch( skill_SA_ENDOW_EARTH ) * 2.0 ) ) * n_A_BaseLV / 100.0 / 3.0;
			w_TotalHits = 3;

			fixedCastTime *= 2.0 - 0.2 * n_A_ActiveSkillLV;
			variableCastTime *= 2.0 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_DIAMOND_DUST )
		{
			// MATK [{( Skill Level x INT ) + ( Endow Tsunami Skill Level x 200 )} x Caster s Base Level / 100 ] %
			n_A_Weapon_element = ele_WATER;
			w_SkillMod = ( n_A_ActiveSkillLV * ( n_A_INT / 100.0 ) + ( SkillSearch( skill_SA_ENDOW_WATER ) * 2.0 ) ) * n_A_BaseLV / 100.0 / 5.0;
			w_TotalHits = 5;
			if ( EquipNumSearch( 1387 ) ) {
				w_SkillMod = 1 + 0.03*n_A_Weapon_ATKplus;
			}
			fixedCastTime *= 1 - 0.2 * n_A_ActiveSkillLV;
			variableCastTime *= 4.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_CLOUD_KILL )
		{
			// MATK [( Skill Level x 40 ) x Caster s Base Level / 100 ] %
			n_A_Weapon_element = ele_POISON;
			w_SkillMod = ( n_A_ActiveSkillLV * 0.4 ) * n_A_BaseLV / 100.0;
			w_TotalHits = parseInt(formElements["SkillSubNum"].value);
			SG_Special_HITnum = w_TotalHits;

			fixedCastTime *= 0.9 - 0.2 * n_A_ActiveSkillLV;
			variableCastTime *= 2.1 + 0.2 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_POISON_BUSTER )
		{
			// MATK [{( Skill Level x 300 ) + 1000 } x Caster s Base Level / 120 ]%
			n_A_Weapon_element = ele_POISON;
			w_SkillMod = ( ( n_A_ActiveSkillLV * 3.0 ) + 10.0 ) * n_A_BaseLV / 120.0;
			w_TotalHits = 1;

			fixedCastTime *= 1.0 - 0.2 * n_A_ActiveSkillLV;
			if ( n_A_ActiveSkillLV < 5 )
			{
				variableCastTime = 1.0 + 1.2 * n_A_ActiveSkillLV;
			}
			else
			{
				variableCastTime = 6.0;
			}
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_PSYCHIC_WAVE )
		{
			// MATK = [{( Skill Level x 70 ) + ( Caster s INT x 3 )} x ( Caster s Base Level / 100 )] %
			n_A_Weapon_element = ele_NEUTRAL;
			w_SkillMod = ( n_A_ActiveSkillLV * 0.7 + n_A_INT * 0.03 ) * n_A_BaseLV / 100.0;
			if ( SkillSearch( skill_SOR_SPIRIT_CONTROL ) == 1 && SkillSearch( skill_SOR_SUMMON_LEVEL ) == 2 )
			{
				n_A_Weapon_element = ( ( SkillSearch( skill_SOR_SUMMON_TYPE ) + 2 ) %4 ) + 1;
				w_SkillMod *= 1.2;
			}
			w_TotalHits = parseInt(formElements["SkillSubNum"].value);
			SG_Special_HITnum = w_TotalHits;

			fixedCastTime *= 1.1 - 0.1 * n_A_ActiveSkillLV;
			if ( EquipNumSearch( 1292 ) ) { // Mental Stick
				if (n_A_Weapon_ATKplus > 5) w_SkillMod *= 1.0+((n_A_Weapon_ATKplus-5)*2/100.0);
			}
			variableCastTime *=	7.0 + 1.0 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			//if ( EquipNumSearch( 1292 ) ) variableCastTime = Math.max(variableCastTime-3, 0);
			n_Delay[ksDelayCooldown] = 5.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_FBOLT )
		{
			var boltLevel = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_FIRE;

			// MATK [(100 * Bolt Skill Level) + (50 * Skill Level)] %
			var bonus = (100 + StPlusCalc2(bon_DMG_SKILL+skill_MA_FIRE_BOLT)+StPlusCard(bon_DMG_SKILL+skill_MA_FIRE_BOLT))/100.0;
			w_SkillMod = (( n_A_ActiveSkillLV * 0.5 ) + boltLevel)*bonus;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_CBOLT )
		{
			var boltLevel = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_WATER;

			// MATK [(100 * Bolt Skill Level) + (50 * Skill Level)] %
			var bonus = (100 + StPlusCalc2(bon_DMG_SKILL+skill_MA_COLD_BOLT)+StPlusCard(bon_DMG_SKILL+skill_MA_COLD_BOLT))/100.0;
			w_SkillMod = (( n_A_ActiveSkillLV * 0.5 ) + boltLevel)*bonus;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_LBOLT )
		{
			var boltLevel = parseInt(formElements["SkillSubNum"].value);
			n_A_Weapon_element = ele_WIND;

			// MATK [(100 * Bolt Skill Level) + (50 * Skill Level)] %
			var bonus = (100 + StPlusCalc2(bon_DMG_SKILL+skill_MA_LIGHTNING_BOLT)+StPlusCard(bon_DMG_SKILL+skill_MA_LIGHTNING_BOLT))/100.0;
			w_SkillMod = (( n_A_ActiveSkillLV * 0.5 ) + boltLevel)*bonus;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.0;
		}
		// Performer Skills
		else if ( n_A_ActiveSkill == skill_MIWA_METALLIC_SOUND )
		{
			n_A_Weapon_element = ele_NEUTRAL;
			w_SkillMod = (( n_A_ActiveSkillLV * 1.2 ) +  SkillSearch( skill_MIWA_VOICE_LESSONS ) * 0.6 ) * n_A_BaseLV / 100.0; // skill mod
			if ( n_A_ActiveSkillLV == 1 || n_A_ActiveSkillLV == 2 )
			{
				w_TotalHits = 2;
			}
			else if ( n_A_ActiveSkillLV == 3 || n_A_ActiveSkillLV == 4 )
			{
				w_TotalHits = 3;
			}
			else
			{
				w_TotalHits = 4;
			}
			n_subHits = 1;

			fixedCastTime *= 0.0;
			variableCastTime *= 0.5 + 0.5 * n_A_ActiveSkillLV;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 1.5 + 0.5 * n_A_ActiveSkillLV;
		}
		else if ( n_A_ActiveSkill == skill_ROY_SHIELD_SPELL_MATK )
		{
			var shieldMdef = 0;
			for ( var i = itm_BONUS_START; ItemOBJ[n_A_Equip[eq_SHIELD]][i] !== bon_NONE; i += 2 )
			{ // find shield MDEF
				if ( ItemOBJ[n_A_Equip[eq_SHIELD]][i] === bon_MDEF )
				{
					shieldMdef = ItemOBJ[n_A_Equip[eq_SHIELD]][i + 1];
				}
			}

			// MATK [{(Caster s Base Level x 4) + (Shield MDEF x 100)} + (Caster s INT x 2)] %
			n_A_Weapon_element = ele_HOLY;
			w_SkillMod = ( ( n_A_BaseLV * 4 ) + shieldMdef ) + ( n_A_INT * 2 / 100.0 );

			fixedCastTime *= 0.0;
			variableCastTime *= 1.0;
			n_Delay[ksDelayGlobal] = 1.0;
			n_Delay[ksDelayCooldown] = 2.0;
		}
		if (damageType == kDmgTypeMagic) {
			var mElementBoost = 100 + StPlusCalc2(bon_INC_MAGIC_NEUTRAL+n_A_Weapon_element) + StPlusCard(bon_INC_MAGIC_NEUTRAL+n_A_Weapon_element);
			w_SkillMod *= mElementBoost / 100.0;
		}

    if (n_tok[bon_DMG_SKILL + n_A_ActiveSkill] !== undefined) {
      w_SkillMod *= parseInt(n_tok[bon_DMG_SKILL + n_A_ActiveSkill] / 100.0);
    }

		// calculate damage
		if ( n_subHits === 0 )
		{
			// No sub-hits for the magic formula
			for ( var i = 0; i < 3; i++ )
			{
				w_DMG[i] = CalcMagicDamage( n_A_MATK[i] * w_SkillMod );
				if( SG_Special_HITnum != 0 )
				{
					SG_Special_DMG[i] = w_DMG[i];
				}
				Last_DMG_B[i] = w_DMG[i];
				Last_DMG_A[i] = w_DMG[i] * w_TotalHits;
				InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8][Language] + w_TotalHits + "hit)";
				w_DMG[i] = Last_DMG_A[i];
			}
		}
		else
		{
			// There are sub-hits for the magic formula
			for ( var i = 0; i < 3; i++ )
			{
				w_DMG[i] = Math.floor( CalcMagicDamage( n_A_MATK[i] * w_SkillMod ) / w_TotalHits );
				Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * w_TotalHits;
				InnStr[i] += Last_DMG_A[i] + " (" + w_DMG[i] + SubName[8][Language] + w_TotalHits + "hit)";
				w_DMG[i] *= w_TotalHits;
			}

			if ( n_A_ActiveSkill == skill_MIWA_METALLIC_SOUND )
			{
				// show sp taken from enemy
				var voiceLessons = SkillSearch( skill_MIWA_VOICE_LESSONS );
				myInnerHtml( "CRInumname", '<font color="#0000FF">SP drained from enemy</font>', 0 );
				//SP Drain = 1 SP Drain per [110 - (10 * Lesson Level)] HP Damage
				var modifier = 110 - ( 10 * voiceLessons );
				var spDrainLow = Math.floor( w_DMG[0] / modifier );
				var spDrainHigh = Math.floor( w_DMG[2] / modifier );
				myInnerHtml( "CRInum", '<font color="#0000FF">' + spDrainLow + "-" + spDrainHigh + "</font>", 0 );
			}
		}

		w_HIT_HYOUJI = 100;
	}
	var delayFixRed = StPlusCalc2(bon_DELAY_SKILL_FLAT+n_A_ActiveSkill) + StPlusCard(bon_DELAY_SKILL_FLAT+n_A_ActiveSkill);
	n_Delay[ksDelayCooldown] = Math.max(n_Delay[ksDelayCooldown]-delayFixRed, 0);
	var castTimeFixRed = StPlusCalc2(bon_CAST_SKILL_FLAT+n_A_ActiveSkill) + StPlusCard(bon_CAST_SKILL_FLAT+n_A_ActiveSkill);
	variableCastTime = Math.max(variableCastTime-castTimeFixRed, 0);
	if ( SkillSearch(skill_SOR_SUMMON_TYPE) == 1 && SkillSearch(skill_SOR_SUMMON_LEVEL) > 0 && SkillSearch(skill_SOR_SPIRIT_CONTROL) == 1 ) {
		fixedCastTime = Math.max(fixedCastTime - 1, 0);
	}
  fixedCastTime = Math.max(fixedCastTime - fixedCastTimeSub, 0)
	setupPVPPlayerMods();
}
function setupPVPPlayerMods() {
	var monster = MonsterOBJ[PVP_ENEMY];
	if (n_B[en_ID]===PVP_ENEMY && monster.length >= 27) {
		monsterBuffs[status_en_buff_Race] = document.getElementsByName("B_KYOUKA10")[0].value = monster[26];
		monsterBuffs[status_en_buff_Elemental] = document.getElementsByName("B_KYOUKA11")[0].value = monster[32+n_A_Weapon_element];
		monsterBuffs[status_en_buff_Ranged] = document.getElementsByName("B_KYOUKA12")[0].value = monster[27];
		monsterBuffs[status_en_buff_Size] = document.getElementsByName("B_KYOUKA13")[0].value = monster[28];
		monsterBuffs[status_en_buff_Normal] = document.getElementsByName("B_KYOUKA14")[0].value = monster[29];
		monsterBuffs[status_en_buff_Other] = document.getElementsByName("B_KYOUKA15")[0].value = monster[30];
	}
}