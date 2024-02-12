function CalcHeal( HealLv, maxHeal, healOnSelf )
{
	// Base Heal Formula
	var healAmount = CalcBaseHeal( HealLv, maxHeal );
	
	// Add in recovery multiplier
	if ( healOnSelf === 1 )
	{
		healAmount = Math.floor( healAmount * CalcRecoveryMultiplier() / 100 );
	}

	return healAmount;
}

function CalcHighHeal( HealLv, maxHeal, healOnSelf )
{
	// Base Heal Formula
	var healAmount = CalcBaseHeal( 10, maxHeal );
	
	// Calc High Heal amount
	healAmount *= 1.7 + HealLv * 0.3;

	// Add in recovery multiplier
	if ( healOnSelf === 1 )
	{
		healAmount = Math.floor( healAmount * CalcRecoveryMultiplier() / 100 );
	}

	return healAmount;
}

var healMultiplier = 100;
function CalcBaseHeal( HealLv, maxHeal )
{
	// Base Heal Formula
	var wHeal = Math.floor( ( n_A_BaseLV + n_A_INT ) / 5 ) * 30 * HealLv * 0.1;
	healMultiplier = 100;
	// Multipliers
	
	// Skills
	if ( SkillSearch( skill_HP_MEDIATIO ) )
	{ // Meditatio
		healMultiplier += 2 * SkillSearch( skill_HP_MEDIATIO );
	}
	
	// Equipment
	healMultiplier += n_tok[bon_HEAL_MUL];
	if ( EquipNumSearch( 1266 ) )
	{ // Yggdrasil Crown
		for ( var i = 0; i < 7; i++ )
		{
			if ( i <= n_A_HEAD_DEF_PLUS )
			{
				healMultiplier++;
			}
		}
		if ( n_A_HEAD_DEF_PLUS >= 6 )
		{
			healMultiplier += ( n_A_HEAD_DEF_PLUS - 5 ) * 2;
		}
	}
	if ( EquipNumSearch( 1085 ) )
	{ // Glorious Cure Wand
		// Blessing Bonus
		var upgradeCount = 0;
		for ( var i = 0; i < n_A_Weapon_ATKplus; i++ )
		{
			if ( i <= 14 )
			{
				upgradeCount++;
			}
		}
		if ( n_A_Weapon_ATKplus >= 6 )
		{
			healMultiplier += ( upgradeCount - 5 ) * 2;
		}
		
		// Upgrade Bonuses
		if ( n_A_Weapon_ATKplus >= 6 )
		{
			healMultiplier += 5;
		}
		if ( n_A_Weapon_ATKplus >= 10 )
		{
			healMultiplier += 10;
		}
	}
	if ( EquipNumSearch( 1161 ) )
	{ // Veteran Hammer
		healMultiplier += SkillSearch( skill_AC_DIVINE_PROTECTION );
	}
	if ( EquipNumSearch( 644 ) )
	{ // Staff of Recovery
		healMultiplier += Math.floor( n_A_Weapon_ATKplus * 1.5 );
	}
	if ( EquipNumSearch( 565 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Dress Hat
		healMultiplier += 1;
	}
	if ( EquipNumSearch( 1000 ) )
	{ // Angelic Ring
		healMultiplier += 20;
	}
	if ( EquipNumSearch( 1307 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Benevolent Guardian
		healMultiplier += 3;
	}
	if ( EquipNumSearch( 1308 ) )
	{ // Benevolent Guardian + Healing Staff
		healMultiplier += n_A_Weapon_ATKplus;
	}
	if ( EquipNumSearch( 1309 ) )
	{ // Benevolent Guardian + Holy Stick
		healMultiplier += n_A_Weapon_ATKplus * 3;
	}
	if ( EquipNumSearch( 1338 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Cancer Diadem
		healMultiplier += 3;
	}
	if ( EquipNumSearch( 1339 ) && n_A_HEAD_DEF_PLUS >= 9 )
	{ // Capricorn Diadem
		healMultiplier += 4;
	}
	if ( EquipNumSearch( 1388 ))
	{ // Light Of Recovery
		healMultiplier += Math.floor( n_A_Weapon_ATKplus * 1.5 );
	}
	if ( EquipNumSearch( 1401 ))
	{ // Ancient Gold Ornament
		if(n_A_JobSearch()==cls_ACO || n_A_JobSearch()==cls_MAG)
			healMultiplier += 7;
	}
	if ( EquipNumSearch( 1522 ) && n_A_HEAD_DEF_PLUS >= 7 )
	{ // Chibi Pope
		healMultiplier += (n_A_HEAD_DEF_PLUS-6);
	}
	
	if ( CardNumSearch( 534 ))
	{ // Parus Card
		if(n_A_JobSearch()==cls_ACO)
		healMultiplier += Math.floor(n_A_HEAD_DEF_PLUS/2);
	}
	
	if (SkillSearch(skill_ABI_OFFERTORIUM)) {
	    healMultiplier += 30 * SkillSearch(skill_ABI_OFFERTORIUM);
	}
	
	
	// Apply Multiplier
	wHeal = Math.floor( wHeal * healMultiplier / 100 );
	
	// MATK is an Additive
	// RO doesn't include equipMATK and MATK Multipliers though
	if ( maxHeal )
	{
		wHeal += n_A_MATK[2];
	}
	else
	{
		wHeal += n_A_MATK[0];
	}

	return wHeal;
}

function CalcRecoveryMultiplier()
{
	var recoveryMultiplier = 100;
	
	// Equipment
	recoveryMultiplier += n_tok[bon_HEAL_REC];
	if ( EquipNumSearch( 1266 ) )
	{
		// ygg crown also increases recovery rate
		for ( var i = 0; i < 7; i++ )
		{
			if ( i <= n_A_HEAD_DEF_PLUS )
			{
				recoveryMultiplier++;
			}
		}
		if ( n_A_HEAD_DEF_PLUS >= 6 )
		{
			recoveryMultiplier += ( n_A_HEAD_DEF_PLUS - 5 ) * 2;
		}
	}
	
	return recoveryMultiplier;
}