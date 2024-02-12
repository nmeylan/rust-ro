function BuildEnemyData()
{ // calc enemy (stats + substats) + display
with( document.calcForm )
{
	var formElements = document.forms["calcForm"].elements;
	n_B = new Array(); // New Values
	n_B2 = new Array(); // Originals (to compare)
	for ( var i = 0; i <= 25; i++ )
	{
		var enemy = formElements["B_Enemy"];
		var index = enemy.value;
		n_B[i] = MonsterOBJ[index][i];
		n_B2[i] = n_B[i];
	}


	/*{ // not human
		
		n_B2[en_SOFTDEF] = n_B[en_VIT];
		n_B2[en_MAXSOFTDEF] = n_B[en_VIT] + (Math.floor(n_B[en_VIT]/20) * Math.floor(n_B[en_VIT]/20) -1);
		if ( n_B2[en_SOFTDEF] > n_B2[en_MAXSOFTDEF] )
		{ // SoftDef (Min) / SoftDef (Max)
			n_B2[en_MAXSOFTDEF] = n_B2[en_SOFTDEF]; // SoftDef (Max) / SoftDef (Min)
		}
	}*/
	//n_B[en_SOFTMDEF] = Math.floor(n_B[en_VIT] / 2) + n_B[en_INT];
	n_B[en_HIT] = n_B[en_LEVEL] + n_B[en_DEX] + Math.floor(n_B[en_LUK] / 3) + 175;
	n_B[en_FLEE] = n_B[en_LEVEL] + n_B[en_AGI] + Math.floor(n_B[en_LUK] / 5) + 100;
	
	{ // Monster Debuffs
		monsterDebuffs[status_en_PROVOKE] = eval(B_IJYOU0.value); // Provoke
		monsterDebuffs[status_en_QUAG] = eval(B_IJYOU1.value); // Quagmire
		monsterDebuffs[status_en_POISON] = B_IJYOU2.checked; // Poison
		monsterDebuffs[status_en_BLIND] = B_IJYOU3.checked; // Blind
		monsterDebuffs[status_en_FREEZE] = B_IJYOU4.checked; // Freeze
		monsterDebuffs[status_en_BLESS] = B_IJYOU5.checked; // Bless
		monsterDebuffs[status_en_LEXA] = B_IJYOU6.checked; // LexA
		monsterDebuffs[status_en_STUN] = B_IJYOU7.checked; // Stun
		monsterDebuffs[status_en_SLEEP] = B_IJYOU8.checked; // Sleep
		monsterDebuffs[status_en_STONE] = B_IJYOU9.checked; // Stone
		monsterDebuffs[status_en_CURSE] = B_IJYOU10.checked; // Curse
		monsterDebuffs[status_en_AGIDOWN] = eval(B_IJYOU11.value);
		monsterDebuffs[status_en_SCRUSIS] = eval(B_IJYOU12.value);
		monsterDebuffs[status_en_STRIPW] = B_IJYOU13.checked;
		monsterDebuffs[status_en_STRIPS] = B_IJYOU14.checked;
		monsterDebuffs[status_en_STRIPA] = B_IJYOU15.checked;
		monsterDebuffs[status_en_STRIPH] = B_IJYOU16.checked;
		monsterDebuffs[status_en_FIBER] = B_IJYOU17.checked;
		monsterDebuffs[status_en_MBREAK] = eval(B_IJYOU18.value);
		monsterDebuffs[status_en_SLGRACE] = B_IJYOU19.checked;
		monsterDebuffs[status_en_DOWNTEMPO] = B_IJYOU20.checked;
		monsterDebuffs[status_en_COINFLING] = eval(B_IJYOU24.value);
		monsterDebuffs[status_en_ESKA] = B_IJYOU21.checked;
		monsterDebuffs[status_en_ESKE] = B_IJYOU22.checked;
		monsterDebuffs[status_en_ELECHANGE] = eval(B_IJYOU23.value);
		monsterDebuffs[status_en_DEEPSLEEP] = formElements["DeepSleep"].checked;
		monsterDebuffs[status_en_VENOM_IMPRESS] = parseInt(formElements["VenomImpress"].value);
		monsterDebuffs[status_en_MARSH_OF_ABYSS] = parseInt(formElements["MarshOfAbyss"].value);
		monsterDebuffs[status_en_GLOOMY_DAY] = parseInt(formElements["GloomyDay"].value);
		monsterDebuffs[status_en_DARK_CLAW] = parseInt(formElements["DarkClaw"].value);
	}
	
	{ // Monster Buffs
		monsterBuffs[status_en_buff_IncreaseAGI] = eval(B_KYOUKA0.value);
		monsterBuffs[status_en_buff_Assumptio] = B_KYOUKA1.checked;
		monsterBuffs[status_en_buff_AdrenalineRush] = B_KYOUKA2.checked;
		monsterBuffs[status_en_buff_MaximizePower] = B_KYOUKA3.checked;
		monsterBuffs[status_en_buff_PowerUp] = B_KYOUKA4.checked;
		monsterBuffs[status_en_buff_FleeUp] = B_KYOUKA5.checked;
		monsterBuffs[status_en_buff_ElementalChange] = eval(B_KYOUKA6.value);
		monsterBuffs[status_en_buff_StoneSkin] = eval(B_KYOUKA7.value);
		monsterBuffs[status_en_buff_MagicMirror] = eval(B_KYOUKA8.value);
		monsterBuffs[status_en_buff_Keeping] = B_KYOUKA9.checked;
		monsterBuffs[status_en_buff_Race] = eval(B_KYOUKA10.value);
		monsterBuffs[status_en_buff_Elemental] = eval(B_KYOUKA11.value);
		monsterBuffs[status_en_buff_Ranged] = eval(B_KYOUKA12.value);
		monsterBuffs[status_en_buff_Size] = eval(B_KYOUKA13.value);
		monsterBuffs[status_en_buff_Normal] = eval(B_KYOUKA14.value);
		monsterBuffs[status_en_buff_Other] = eval(B_KYOUKA15.value);
	}
	
	if (monsterBuffs[status_en_buff_ElementalChange] )
	{ // EleChange
		n_B[en_ELEMENT] = monsterBuffs[status_en_buff_ElementalChange]; // Ele
	}
	
	if ( monsterDebuffs[status_en_ELECHANGE] )
	{
		n_B[en_ELEMENT] = monsterDebuffs[status_en_ELECHANGE] * 10 + (n_B[en_ELEMENT] % 10);
	}

	if ( n_B[en_BOSS] === 0 )
	{
		if ( monsterDebuffs[status_en_FREEZE] && n_B[en_RACE] !== 1 )
		{ // NonUndead (?)
			n_B[en_ELEMENT] = 11; // Ele
		}
	}
	
	if ( n_B[en_BOSS] === 0 )
	{
		if ( monsterDebuffs[status_en_STONE] && n_B[en_RACE] !== 1 )
		{ // NonUndead (?)
			n_B[en_ELEMENT] = 21; // Ele
		}
	}

	if ( monsterBuffs[status_en_buff_MaximizePower] )
	{ // PowerMax
		n_B[en_MINATK] = n_B[en_MAXATK]; // Atk (Min) / Atk (Max)
	}

	if ( n_B[en_BOSS] === 0)
	{
		if ( monsterDebuffs[status_en_CURSE] )
		{
			n_B[en_MINATK] -= Math.floor(n_B[en_MINATK] * 25 /100); // Atk (Min)
			n_B[en_MAXATK] -= Math.floor(n_B[en_MAXATK] * 25 /100); // Atk (Max)
		}
	}

	var wATK=0; // Atk Multiplier
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss ?
		if ( monsterDebuffs[status_en_PROVOKE] !== 0 && n_B[en_ELEMENT] < 90 )
		{ // Ele<?
			wATK += 2 + monsterDebuffs[status_en_PROVOKE] * 3;
		}
	}
	
	if(monsterDebuffs[status_en_ESKE])
	{ // Eske
		wATK += 300;
	}
	
	if ( monsterBuffs[status_en_buff_PowerUp] )
	{ // PowerUp
		wATK += 200;
	}

	n_B[en_MINATK] += Math.floor(n_B[en_MINATK] * wATK / 100); // Atk (Min)
	n_B[en_MAXATK] += Math.floor(n_B[en_MAXATK] * wATK / 100); // Atk (Max)

/*	if(monsterBuffs[status_en_buff_PowerUp])
	{
		n_B[en_MINATK] = n_B[en_MINATK] * 3; // Atk (Min)
		n_B[en_MAXATK] = n_B[en_MAXATK] * 3; // Atk (Max)
	}
*/
	if ( monsterDebuffs[status_en_STRIPW] && PlayerVersusPlayer === 0 )
	{ // DivWeapon && notHuman
		n_B[en_MINATK] -= Math.floor(n_B[en_MINATK] * 25 /100); // Atk (Min)
		n_B[en_MAXATK] -= Math.floor(n_B[en_MAXATK] * 25 /100); // Atk (Max)
	}

	if ( monsterBuffs[status_en_buff_IncreaseAGI] )
	{ // IncAgi
		n_B[en_AGI] += 2 + monsterBuffs[status_en_buff_IncreaseAGI]; // Agi
	}

	if ( monsterDebuffs[status_en_QUAG] )
	{ // Quag (Agi)
		var w;
		var w2;
		w2 = monsterDebuffs[status_en_QUAG] * 10 // Quag
		w = Math.floor(n_B[en_AGI] / 2); // QuagLimit / Agi
		
		if ( w > w2 )
		{
			n_B[en_AGI] -= w2; // Agi
		}
		else
		{
			n_B[en_AGI] -= w; // Agi
		}
	}
	if ( monsterDebuffs[status_en_MARSH_OF_ABYSS] )
	{ // marsh of abyss: AGI/DEX reduction on monsters: - (6 * Skill Level) %
		n_B[en_AGI] -= 6 * monsterDebuffs[status_en_MARSH_OF_ABYSS];
	}

	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( monsterDebuffs[status_en_AGIDOWN] )
		{ // DecAgi
			n_B[en_AGI] -= ( monsterDebuffs[status_en_AGIDOWN] + 2 ); // Agi
			if ( n_B[en_AGI] < 0 )
			{ // Agi
				n_B[en_AGI]=0; // Agi
			}
		}
	}
/* [START] */

	if ( monsterDebuffs[status_en_QUAG] )
	{ // Quag (Dex)
		var w;
		var w2;
		if ( PlayerVersusPlayer )
		{ // Human
			w2 = monsterDebuffs[status_en_QUAG] * 5; // Quag
			w = Math.floor(n_B[en_DEX] / 4); // QuagLimit / Dex
		}
		else
		{
			w2 = monsterDebuffs[status_en_QUAG] * 10 // Quag
			w = Math.floor(n_B[en_DEX] / 2); // Quaglimit / Dex
		}
		if ( w > w2 )
		{
			n_B[en_DEX] -= w2; // Dex
		}
		else
		{
			n_B[en_DEX] -= w; // Dex
		}
	}
	if ( monsterDebuffs[status_en_MARSH_OF_ABYSS] )
	{ // marsh of abyss: AGI/DEX reduction on monsters: - (6 * Skill Level) %
		n_B[en_DEX] -= 6 * monsterDebuffs[status_en_MARSH_OF_ABYSS];
	}
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_BLESS] && (n_B[en_RACE]==6||n_B[en_ELEMENT]>=90))
		{ //Bless&Demon|Undead
			n_B[en_DEX] = n_B[en_DEX] - Math.floor(n_B[en_DEX] /2); // Dex
		}
	}
	
	if ( monsterDebuffs[status_en_STRIPA] && PlayerVersusPlayer === 0 )
	{ // StripArmor
		n_B[en_VIT] -= Math.floor(n_B[en_VIT] * 40 /100); // Vit
	}
	
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( monsterDebuffs[status_en_BLESS] && ( n_B[en_RACE] === 6 || n_B[en_ELEMENT] >= 90 ) )
		{ //Bless&Demon|Undead
			n_B[en_INT] = n_B[en_INT] - Math.floor(n_B[en_INT] /2);  // Int
		}
	}

	if ( monsterDebuffs[status_en_STRIPH] && PlayerVersusPlayer === 0 )
	{ // StripHelm
		n_B[en_INT] -= Math.floor(n_B[en_INT] * 40 /100); // Int
	}

	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( monsterDebuffs[status_en_CURSE] )
		{ // Curse
			n_B[en_LUK] = 0; // Luk
		}
	}

	/*if ( PlayerVersusPlayer === 0 )
	{ // NotHuman
		n_B[en_SOFTDEF] = n_B[en_VIT]; // SoftDef (Min) / Vit
		n_B[en_MAXSOFTDEF] = n_B[en_VIT] + (Math.floor(n_B[en_VIT]/20) * Math.floor(n_B[en_VIT]/20) -1); // SoftDef (Min) / Vit
		if ( n_B[en_SOFTDEF] > n_B[en_MAXSOFTDEF] )
		{ // SoftDef (Min) / SoftDef (Max)
			n_B[en_MAXSOFTDEF] = n_B[en_SOFTDEF]; // SoftDef (Max) / SoftDef (Min)
		}
	}*/
	//n_B[en_SOFTMDEF] = Math.floor(n_B[en_VIT] / 2) + n_B[en_INT]; // SoftMDef / Vit / Int
	n_B[en_HIT] = n_B[en_LEVEL] + n_B[en_DEX] + Math.floor(n_B[en_LUK] / 3) + 175; // Hit / Lvl / Dex / Luk
	n_B[en_FLEE] = n_B[en_LEVEL] + n_B[en_AGI] + Math.floor(n_B[en_LUK] / 5) + 100; // Flee / Lvl / Agi / Luk
	
	var wDEF = 0; // DefSubber
	if( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_PROVOKE]!=0 && n_B[en_ELEMENT]<90) // Provoke / EUndead
			wDEF += 5 + monsterDebuffs[status_en_PROVOKE] * 5; // Provoke
	}
	
	if(monsterDebuffs[22])
	{
		wDEF += 50;
	}
	
	if(monsterDebuffs[24])
	{
		wDEF += 5 * monsterDebuffs[24];
	}
	
	if ( wDEF > 100 )
	{
		wDEF=100;
	}
	
	n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * wDEF /100); // Def

	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_POISON]) // Poison
			n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * 25 / 100); // Def
	}

	// DEF--------------------------------
	var w = 0; // Def
	w += n_tok[290];
	w += n_tok[300+n_B[en_RACE]]; // Race
	// RSmiting / ShieldBoom / ShieldBoom(SL) / GCross / OImpact / FinalStrike / FinalStrike(MHP)
	// DefIgnore ?
	if ( n_A_ActiveSkill==324 || n_A_ActiveSkill==159 || n_A_ActiveSkill==384 || n_A_ActiveSkill==162 || n_A_ActiveSkill==193 || n_A_ActiveSkill==405 || n_A_ActiveSkill==438)
	{
		w = 0;
	}
	
	if ( w )
	{
		if ( w < 0 )
		{
			w = 0;
		}
		n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * w /100); // Def
	}
	
	// monster buffs
	if ( monsterBuffs[status_en_buff_MagicMirror] )
	{ // NotHuman
		n_B[en_SOFTDEF] -= Math.floor(n_B[en_SOFTDEF] * 20 * monsterBuffs[status_en_buff_MagicMirror] /100); // SoftDef (Min)
		n_B[en_MAXSOFTDEF] -= Math.floor(n_B[en_MAXSOFTDEF] * 20 * monsterBuffs[status_en_buff_MagicMirror] /100); // SoftDef (Max)
	}
	if ( monsterBuffs[status_en_buff_Keeping] )
	{
		n_B[en_HARDDEF] *= 2; // Def
	}
	if ( monsterBuffs[status_en_buff_Assumptio] )
	{
		n_B[en_HARDDEF] *= 2; // Def
		n_B[en_SOFTDEF] *=2
	}
	
	// monster debuffs
	if ( monsterDebuffs[status_en_STRIPS] )
	{ // && NotHuman
		n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * 15 /100); // Def
	}
	if ( monsterDebuffs[status_en_SCRUSIS] && ( n_B[en_RACE] == race_DEMON || n_B[en_ELEMENT] >= 90 ) )
	{ // Signum Crusis on Demon Or Undead
		n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * (10 + monsterDebuffs[12] * 4) /100); // Def
	}
	if ( monsterDebuffs[status_en_GLOOMY_DAY] )
	{ // GLOOMY DAY
		n_B[en_FLEE] -= (20 + 5 * monsterDebuffs[status_en_GLOOMY_DAY]); // Flee
		//n_B[en_FLEE] -= (20 + 5 * monsterDebuffs[status_en_GLOOMY_DAY]); // Flee
	}
	
	n_B[en_SOFTDEF] -= Math.floor(n_B[en_SOFTDEF] * wDEF / 100); // SoftDef (Min)
	n_B[en_MAXSOFTDEF] -= Math.floor(n_B[en_MAXSOFTDEF] * wDEF / 100); // SoftDef (Max)

	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_POISON])
		{ // Poison
			n_B[en_SOFTDEF] -= Math.floor(n_B[en_SOFTDEF] * 25 / 100); // SoftDef (Min)
			n_B[en_MAXSOFTDEF] -= Math.floor(n_B[en_MAXSOFTDEF] * 25 / 100); // SoftDef (Max)
		}
	}
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if( monsterDebuffs[status_en_FREEZE] && n_B[en_RACE] !== 1 )
		{ // Freeze / RUndead
			n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * 50 /100); // Def
			n_B[en_SOFTDEF] -= Math.floor(n_B[en_SOFTDEF] * 50 /100); // SoftDef (Min)
			n_B[en_MAXSOFTDEF] -= Math.floor(n_B[en_MAXSOFTDEF] * 50 /100); // SoftDef (Max)
		}
	}
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if( monsterDebuffs[status_en_STONE] && n_B[en_RACE] !== 1 )
		{ // Stone / Undead
			n_B[en_HARDDEF] -= Math.floor(n_B[en_HARDDEF] * 50 /100); // Def
			n_B[en_SOFTDEF] -= Math.floor(n_B[en_SOFTDEF] * 50 /100); // SoftDef (Min)
			n_B[en_MAXSOFTDEF] -= Math.floor(n_B[en_MAXSOFTDEF] * 50 /100); // SoftDef (Max)
		}
	}
	if ( monsterDebuffs[status_en_ESKA] )
	{ // Eska ?
		n_B[en_MAXSOFTDEF] += 90; // SoftDef (Max)
	}
	if ( monsterDebuffs[status_en_DOWNTEMPO] )
	{ // DownTempo
		n_B[en_HARDDEF] = 0; // Def
		n_B[en_SOFTDEF] = 0; // SoftDef (Min)
		n_B[en_MAXSOFTDEF] = 0; // SoftDef (Max)
	}
	
	
	// MDEF-------------------------------------------------
	var w = 0; // MDef
	w += n_tok[bon_DEFIGN_RC_ALL];
	w += n_tok[310+n_B[en_RACE]]; // Race
	if ( w )
	{
		if(w < 0)
			w = 0;
		n_B[en_HARDMDEF] -= Math.floor(n_B[en_HARDMDEF] * w /100); // MDef
	}
	
	// monster buffs
	if ( monsterBuffs[status_en_buff_StoneSkin] )
	{
		n_B[en_SOFTMDEF] -= Math.floor(n_B[en_SOFTMDEF] * 20 * monsterBuffs[status_en_buff_StoneSkin] /100); // SoftMDef
	}
	
	// monster debuffs
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_FREEZE] && n_B[en_RACE]!=1) // Freeze / RUndead
			n_B[en_HARDMDEF] += Math.floor(n_B[en_HARDMDEF] * 25 /100); // MDef
	}
	
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_STONE] && n_B[en_RACE]!=1) // Stone / RUndead
			n_B[en_HARDMDEF] += Math.floor(n_B[en_HARDMDEF] * 25 /100); // MDef
	}

	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if(monsterDebuffs[status_en_MBREAK] && n_B[en_ELEMENT]<90) // EUndead
			n_B[en_SOFTMDEF] -= Math.floor(n_B[en_SOFTMDEF] * (monsterDebuffs[18] * 12) / 100); // SoftMDef
	}
	if(monsterDebuffs[status_en_ESKA])
	{
		n_B[en_SOFTMDEF] = 90; // SoftMDef
	}
	if ( monsterBuffs[status_en_buff_Assumptio] )
	{
		n_B[en_HARDMDEF] *= 2; // Def
		n_B[en_SOFTMDEF] *=2
	}
	
	// HIT & FLEE--------------------------------------------------
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if (monsterDebuffs[3])
		{ // Blind
			n_B[en_HIT] -= 25; // Hit
			if(n_B[en_HIT] < 1)
			{ // Hit
				n_B[en_HIT] = 1; // Hit
			}
		}
	}
	
	if ( monsterBuffs[status_en_buff_PowerUp])
	{ // PowerUp
		n_B[en_HIT] = n_B[en_HIT] * 2; // Hit
	}
	if ( monsterBuffs[status_en_buff_FleeUp] )
	{ // AgiUp Flee*2
		n_B[en_FLEE] = n_B[en_FLEE] * 2; // Flee
	}
	
	/* Truncate calculation speed enhancement after dark (FLEE * 2) in planning */
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( monsterDebuffs[status_en_BLIND] )
		{ // Blind
			n_B[en_FLEE] -= Math.floor(n_B[en_FLEE] * 25 / 100); // Flee
		}
	}
	if ( monsterDebuffs[status_en_FIBER] )
	{ // FiberLock
		n_B[en_FLEE] = Max(0, n_B[en_FLEE]-50);
	}
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( ( monsterDebuffs[status_en_FREEZE] || monsterDebuffs[status_en_STONE] ) && n_B[en_RACE] != 1 )
		{ // Freeze Or Stone and NOT Undead
			n_B[en_FLEE] = -99; // Flee
		}
	}
	if ( n_B[en_BOSS] === 0 )
	{ // NonBoss
		if ( monsterDebuffs[status_en_STUN] || monsterDebuffs[status_en_SLEEP] || monsterDebuffs[status_en_DEEPSLEEP] )
		{ // Stun / Sleep / Deep Sleep
			n_B[en_FLEE] = -99; // Flee
		}
	}
	
/* [END] */
	{ // EXP Mods -------------------------------------------
		var w1_Exp = 100;
		w1_Exp += StPlusCard(120+n_B[en_RACE]); // Race
		w1_Exp += StPlusCalc2(120+n_B[en_RACE]); // Race
		var w2_Exp = 0;
		if(EquipNumSearch(1030))
			w1_Exp += 5 * EquipNumSearch(1030);
		if(n_A_JobSearch()==3 && CardNumSearch(card_ISET_ACOLYTESET) && (n_B[en_RACE]==1 || n_B[en_RACE]==6)) // RUndead, Acolyte Card Set
			w1_Exp += 5;
		if(n_B[en_RACE] == 2 && n_A_JobSearch()==4 && CardNumSearch(card_ISET_ARCHERSET)) // RUndead, Archer Card Set
			w1_Exp += 5;
		if(battleEffects[pass_VIII_BAT_MANUAL])
			w1_Exp += battleEffects[pass_VIII_BAT_MANUAL];
		if(battleEffects[pass_VIII_JOB_MANUAL])
			w2_Exp += 50;
		if(battleEffects[pass_VIII_VIPSTATUS])
			w1_Exp += 50;
		if(otherBuffs[ksMurderBonus])
			w1_Exp += 100;
		if(battleEffects[pass_VIII_SPE_ENVIRONMENT] == 3 || otherBuffs[ksMurderBonus])
		{
			w1_Exp = w1_Exp * 2;
			w2_Exp = w2_Exp * 2;
		}
		if(w1_Exp != 0 || w2_Exp != 0)
		{
			n_B[en_BASEEXP] = Math.floor(n_B[en_BASEEXP] * w1_Exp / 100); // bExp
			n_B[en_JOBEXP] = Math.floor(n_B[en_JOBEXP] * (w1_Exp + w2_Exp) / 100);// jExp
		}
		if(battleEffects[pass_VIII_PAR_COUNT])
		{
			n_B[en_BASEEXP] = Math.floor(n_B[en_BASEEXP] * (1 + (20 * battleEffects[pass_VIII_PAR_COUNT] / 100)) / (1 + battleEffects[pass_VIII_PAR_COUNT]) + 1); // bExp
			n_B[en_JOBEXP] = Math.floor(n_B[en_JOBEXP] * (1 + (20 * battleEffects[pass_VIII_PAR_COUNT] / 100)) / (1 + battleEffects[pass_VIII_PAR_COUNT]) + 1);
		}
		if(battleEffects[pass_VIII_EXP_TAB])
		{
			n_B[en_BASEEXP] = Math.floor(n_B[en_BASEEXP] * (100 + 5 * battleEffects[pass_VIII_EXP_TAB])/100); // bExp
			n_B[en_JOBEXP] = Math.floor(n_B[en_JOBEXP] * (100 + 5 * battleEffects[pass_VIII_EXP_TAB])/100);
		}
		if(SkillSearch(367))
		{
			n_B[en_BASEEXP] = Math.floor(n_B[en_BASEEXP] * (100 + 10 * SkillSearch(367))/100); // bExp
			n_B[en_JOBEXP] = Math.floor(n_B[en_JOBEXP] * (100 + 10 * SkillSearch(367))/100);
		}
		if(battleEffects[pass_VIII_SER_EXP])
		{
			n_B[en_BASEEXP] = Math.floor(n_B[en_BASEEXP] * (100 + 25 * battleEffects[pass_VIII_SER_EXP])/100); // bExp
			n_B[en_JOBEXP] = Math.floor(n_B[en_JOBEXP] * (100 + 25 * battleEffects[pass_VIII_SER_EXP])/100);
		}
		
		if ( n_B[en_BOSS] === 0 )
		{ // NonBoss
			if ( performerBuffs[ksEnsemble] === ksMentalSensing && performerBuffs[ksEnsembleLevel] > 0 )
			{ // Mental Sensing
				n_B[en_BASEEXP] = Math.floor( n_B[en_BASEEXP] * ( 125 + 11 * performerBuffs[ksEnsembleLevel] ) / 100 ); // bExp
				n_B[en_JOBEXP] = Math.floor( n_B[en_JOBEXP] * ( 125 + 11 * performerBuffs[ksEnsembleLevel] ) / 100 );
			}
			n_B[en_BASEEXP] = Max(1, Math.floor(n_B[en_BASEEXP] * expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL])));
			n_B[en_JOBEXP] = Max(1, Math.floor(n_B[en_JOBEXP] * expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL])));
		}
	// --------------------------------------
	}

	n_B[en_PERFECT_HIT] = n_B[en_FLEE]+100; // PerfHit = Flee+100
	n_B[en_PERFECT_DODGE] = n_B[en_HIT] -5; // PerfDodge = Hit-5

	{
		myInnerHtml("B_AA"," + ",0);
		myInnerHtml("B_AB"," + ",0);
		myInnerHtml("B_AC","~",0);
		var wIJ = [en_HP,en_MINATK,en_MAXATK,en_PERFECT_HIT,en_PERFECT_DODGE,en_HARDDEF,en_HARDMDEF,en_SOFTDEF,en_MAXSOFTDEF,en_SOFTMDEF];
		var wIJ2 = [en_BASEEXP,en_JOBEXP];
		var wFront = "<Font color='BLUE'><B>";
		var wFront2 = "<Font color='RED'><B>";
		var wBack = "</B></Font>";

		for ( var i = 0; i <= 9; i++ )
		{ // DispStats
			var wIJstr = n_B[wIJ[i]];
			if(n_B[wIJ[i]] < n_B2[wIJ[i]]) // If better - blue
				wIJstr =  wFront + n_B[wIJ[i]] + wBack;
			if(n_B[wIJ[i]] > n_B2[wIJ[i]]) // if worse - red
				wIJstr =  wFront2 + n_B[wIJ[i]] + wBack;
			myInnerHtml("B_"+wIJ[i],wIJstr,0);
		}
		if (1)
		{ // SoftDef (Min) / SoftDef (Max)
			myInnerHtml("B_AC","",0);
			myInnerHtml("B_24","",0);
		}
		for ( var i = 0; i < 2; i++ )
		{ // DispExp
			var wIJstr = n_B[wIJ2[i]];
			if(n_B[wIJ2[i]] < n_B2[wIJ2[i]])
				wIJstr =  wFront2 + n_B[wIJ2[i]] + wBack;
			if(n_B[wIJ2[i]] > n_B2[wIJ2[i]])
				wIJstr =  wFront + n_B[wIJ2[i]] + wBack;
			myInnerHtml("B_"+wIJ2[i],wIJstr,0);
		}

		myInnerHtml("B_2",SyuzokuOBJ[n_B[en_RACE]][Language],0); // DispRace
		w = Math.floor(n_B[en_ELEMENT] / 10); // Ele
		if ( n_B[en_ELEMENT] !== n_B2[en_ELEMENT] )
		{ // Ele != OrgEle
			myInnerHtml( "B_3", wFront2 + (ZokuseiOBJ[w][Language] + " " + n_B[en_ELEMENT] % 10 ) + wBack, 0 ); // DispEle
		}
		else
		{
			myInnerHtml( "B_3", (ZokuseiOBJ[w][Language] + " " + n_B[en_ELEMENT] % 10 ), 0 ); // Ele
		}
		myInnerHtml("B_4",SizeOBJ[n_B[en_SIZE]][Language],0); // DispSize
	}

	n_B_DEF2 = [0,0,0];
	n_B_DEF2[2] = n_B[en_SOFTDEF]; // SoftDef (Min)
	n_B_DEF2[0] = n_B_DEF2[2];
	n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) /2);
	n_B_MDEF2 = n_B[en_SOFTMDEF]; // SoftMDef
	n_B_HIT = n_B[en_HIT]; // Hit
	n_B_FLEE = n_B[en_FLEE]; // Flee
}
}
