function GetWord( wordValue )
{
	"use strict";
	if ( wordValue === 85 )
	{
		return "ATK"; //WordData[40][Language];
	}
	
	if ( WordData[wordValue][Language] === undefined )
	{
		if ( WordData[wordValue][1] === undefined )
		{
			return WordData[wordValue][0];
		}
		else
		{
			return WordData[wordValue][1];
		}
	}
	
	return WordData[wordValue][Language];
}

function myInnerHtml( elementId, insertValue, concatenate )
{ // insert into html
	"use strict";
	var formElement = document.getElementById( elementId );
	
	if ( formElement === null )
	{
		formElement = document.getElementById( "PRT" );
		formElement.insertAdjacentHTML( 'BeforeEnd', elementId + " " + insertValue );
		return;
	}

	if ( concatenate === 0 )
	{ // replace
		while ( formElement.hasChildNodes() )
		{
			formElement.removeChild( formElement.firstChild );
		}
		formElement.innerHTML = insertValue;
	}
	else
	{ // add
		// not ff compatible
		formElement.insertAdjacentHTML( 'BeforeEnd', insertValue );
	}
}

function Kanma(num)
{ // Round - floating point
	if (isNaN(num)) return 0;
	var str = "";
	var x = new Array();
	if(num < 0)
	{
		num = num * -1;
		str += "-";
	}
	for(var i=0;Math.floor(num / 1000) != 0;i++)
	{
		var w = (num % 1000);
		if(w == 0)
		{
			x[i] = ",000";
		}
		else if(w < 10)
		{
			x[i] = ",00" + w;
		}
		else if(w < 100)
		{
			x[i] = ",0" + w;
		}
		else
		{
			x[i] = "," + w;
		}
		num = Math.floor(num / 1000);
	}
	x[i] = num;
	while(i>=0)
	{
		str += x[i];
		i--;
	}
	return str;
}

function calcSpecialTok() {
	if (!isNaN(parseInt(document.calcForm.E_BOOST_ATK.value))) n_tok[bon_ATK] += parseInt(document.calcForm.E_BOOST_ATK.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_ATK_PERC.value))) n_tok[bon_PHY_ATK] += parseInt(document.calcForm.E_BOOST_ATK_PERC.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_MATK.value))) n_tok[bon_MATK] += parseInt(document.calcForm.E_BOOST_MATK.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_MATK_PERC.value))) n_tok[bon_MATK_MUL] += parseInt(document.calcForm.E_BOOST_MATK_PERC.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_HIT.value))) n_tok[bon_HIT] += parseInt(document.calcForm.E_BOOST_HIT.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_FLEE.value))) n_tok[bon_FLEE] += parseInt(document.calcForm.E_BOOST_FLEE.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_DODGE.value))) n_tok[bon_PDODGE] += parseInt(document.calcForm.E_BOOST_DODGE.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_HP.value))) n_tok[bon_HP_ADD] += parseInt(document.calcForm.E_BOOST_HP.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_SP.value))) n_tok[bon_SP_ADD] += parseInt(document.calcForm.E_BOOST_SP.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_HP_PERC.value))) n_tok[bon_HP_MUL] += parseInt(document.calcForm.E_BOOST_HP_PERC.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_SP_PERC.value))) n_tok[bon_SP_MUL] += parseInt(document.calcForm.E_BOOST_SP_PERC.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_RANGED.value))) n_tok[bon_DMG_RANGE] += parseInt(document.calcForm.E_BOOST_RANGED.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_DEF.value))) n_tok[bon_DEF] += parseInt(document.calcForm.E_BOOST_DEF.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_MDEF.value))) n_tok[bon_MDEF] += parseInt(document.calcForm.E_BOOST_MDEF.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_CRIT.value))) n_tok[bon_CRIT] += parseInt(document.calcForm.E_BOOST_CRIT.value);
	if (!isNaN(parseInt(document.calcForm.E_BOOST_RED_PERC.value))) {
		n_tok[bon_RED_BOSS] += parseInt(document.calcForm.E_BOOST_RED_PERC.value);
		n_tok[bon_RED_NON_BOSS] += parseInt(document.calcForm.E_BOOST_RED_PERC.value);
	}
	if (!isNaN(parseInt(document.calcForm.E_BOOST_ASPD.value))) {
		n_tok[bon_ASPD_ADD] += parseInt(document.calcForm.E_BOOST_ASPD.value);
	}
	if (!isNaN(parseInt(document.calcForm.E_BOOST_ASPD_PERC.value))) {
		n_tok[bon_ASPD_MUL] += parseInt(document.calcForm.E_BOOST_ASPD_PERC.value);
	}
	if (!isNaN(parseInt(document.calcForm.E_BOOST_CASTING.value))) n_tok[bon_RED_CAST] -= parseInt(document.calcForm.E_BOOST_CASTING.value);
}
function ClearBonuses()
{
	for ( var i = 1; i <= 200; i++ )
	{
		n_tok[i] = 0;
		n_tok[i] += StPlusCalc2(i);
		n_tok[i] += StPlusCard(i);
	}
	for ( var i = 290; i <= 339; i++ )
	{
		n_tok[i] = 0;
		n_tok[i] += StPlusCalc2(i);
		n_tok[i] += StPlusCard(i);
	}
	calcSpecialTok();	
	if ( SkillSearch(skill_SOR_SPIRIT_CONTROL) === 2 &&
	     SkillSearch(skill_SOR_SUMMON_LEVEL) === 2 )
	{
		w = SkillSearch(skill_SOR_SUMMON_TYPE);
		if (w == 0)
		{ // Fire
			n_tok[bon_RED_ELE_FIRE]+=100;
			n_tok[bon_RED_ELE_WATER]-=100;
		}
		else if (w == 1)
		{ // Wind
			n_tok[bon_RED_ELE_WIND]+=100;
			n_tok[bon_RED_ELE_EARTH]-=100;
		}
		else if (w == 2)
		{ // Water
			n_tok[bon_RED_ELE_WATER]+=100;
			n_tok[bon_RED_ELE_WIND]-=100;
		}
		else if (w == 3)
		{ // Earth
			n_tok[bon_RED_ELE_EARTH]+=100;
			n_tok[bon_RED_ELE_FIRE]-=100;
		}
	}
}

function StCalc(nSC)
{ // statPointCalc "Remain Status Points"
	n_A_STR = parseInt(formElements["A_STR"].value);
	n_A_AGI = parseInt(formElements["A_AGI"].value);
	n_A_VIT = parseInt(formElements["A_VIT"].value);
	n_A_DEX = parseInt(formElements["A_DEX"].value);
	n_A_INT = parseInt(formElements["A_INT"].value);
	n_A_LUK = parseInt(formElements["A_LUK"].value);

	StPoint = 0;
	for ( var i = 2; i <= n_A_STR; i++ )
	{
		StPoint += StCalc2( i );
	}
	for ( var i = 2; i <= n_A_AGI; i++ )
	{
		StPoint += StCalc2( i );
	}
	for ( var i = 2; i <= n_A_VIT; i++ )
	{
		StPoint += StCalc2( i );
	}
	for ( var i = 2; i <= n_A_INT; i++ )
	{
		StPoint += StCalc2( i );
	}
	for ( var i = 2; i <= n_A_DEX; i++ )
	{
		StPoint += StCalc2( i );
	}
	for ( var i = 2; i <= n_A_LUK; i++ )
	{
		StPoint += StCalc2( i );
	}

	n_A_BaseLV = parseInt(formElements["A_BaseLV"].value);

	n_A_JobSet();
	if ( rebirthClass )
	{ // trans
		wStPoint = 100;
	}
	else
	{ // non trans
		wStPoint = 48;
	    
	}
	
	var wMAXLV;
	if( thirdClass )
	{ // third class
		wMAXLV = CONST_MAXLVL_THIRD; // EDIT BY PROGM
	}
	else
	{
                if (n_A_JOB == cls_KAGOB || n_A_JOB == cls_ENOVI) {
                        wMAXLV = CONST_MAXLVL_KAGOB_ENOVI;
                } else {
                        wMAXLV = CONST_MAXLVL;
                }
	}
		
	if ( nSC == 1 || formElements["BLVauto"].checked == 0 )
	{ // manual base level
		for ( var i = 1; i < n_A_BaseLV; i++ )
		{
			if ( i <= 100 )
			{
				wStPoint += Math.floor(i / 5) + 3;
			}
			else if (i <= 159 || i == 161 || i == 170)
                        {
                                wStPoint += Math.floor( ( i - 100 ) / 10 ) + 23;
                        } else {
                                wStPoint += Math.floor( ( i - 100 ) / 10 ) + 24;
                        }
		}
	}
	else
	{ // auto base level
		if ( thirdClass == 0 )
		{
		    if (n_A_JOB == cls_KAGOB || n_A_JOB == cls_ENOVI) {
			wStPoint += 1225;
		    } else {
			for ( var i = 1; StPoint > wStPoint && i < wMAXLV; i++ )
			{
				wStPoint += Math.floor( i / 5 ) + 3;
			}
		    }
		}
		else
		{
			wStPoint += 1225;
			for ( var i = CONST_MAXLVL; StPoint > wStPoint && i < wMAXLV; i++ )
			{
				if ( i <= 100 )
				{
					wStPoint += Math.floor( i / 5 ) + 3;
				}
				else if (i <= 159 || i == 161 || i == 170)
				{
					wStPoint += Math.floor( ( i - 100 ) / 10 ) + 23;
				} else {
                                	wStPoint += Math.floor( ( i - 100 ) / 10 ) + 24;
                                }
			}
		}
	}

	// fill out form
	formElements["A_BaseLV"].value = i;
	myInnerHtml( "A_STPOINT", wStPoint - StPoint, 0 );
}

function StCalc2(nSC2)
{ // statToPoints
	if(nSC2<=100)
		return Math.floor((nSC2 - 2) /10) + 2;
	if(nSC2<=CONST_MAXSTAT_THIRD)
		return Math.floor((nSC2-101)/5) * 4 + 16;
	return 0;
}

function n_A_JobSet()
{ // Check 3rd and Rebirth
	n_A_JOB = parseInt(formElements["A_JOB"].value);
	
	rebirthClass = 0; // Rebirth
	thirdClass = 0; // Third Cls
			
	if ( n_A_JOB >= cls_LOR && n_A_JOB <= cls_HMER )
	{ // Rebirth
		rebirthClass = 1;
	}

	if ( n_A_JOB >= cls_RUN && n_A_JOB <= cls_GENt && n_A_JOB % 2 == 1 )
	{ // 3rd - Rebirth
		rebirthClass = 1;
	}
	
	if ( n_A_JOB >= cls_RUN && n_A_JOB <= cls_GENt )
	{ // 3rd Cls
		thirdClass = 1;
	}
}

function n_A_JobSearch()
{ // 1st Cls Hierachy
	if ( n_A_JOB <= cls_MER )
	{ // 1st Cls
		return n_A_JOB;
	}
		
	if ( n_A_JOB == cls_SNOVI || n_A_JOB == cls_HNOV || n_A_JOB === cls_ENOVI )
	{
		return cls_NOV;
	}
		
	var w = n_A_JobSearch2();
	if(w == cls_KNI || w == cls_CRU || n_A_JOB == cls_HSWO)
		return cls_SWO;
	if(w == cls_ASS || w == cls_ROG || n_A_JOB == cls_HTHI)
		return cls_THI;
	if(w == cls_PRI || w == cls_MON || n_A_JOB == cls_HACO)
		return cls_ACO;
	if(w == cls_HUN || w == cls_BAR || w == cls_DAN || n_A_JOB == cls_HARC)
		return cls_ARC;
	if(w == cls_WIZ || w == cls_SAG || n_A_JOB == cls_HMAG)
		return cls_MAG;
	if(w == cls_BLA || w == cls_ALC || n_A_JOB == cls_HMER)
		return cls_MER;
	if(n_A_JOB == cls_TKK || n_A_JOB == cls_TKM || n_A_JOB == cls_SL)
		return cls_TKK;
		
	return cls_NOV;
}

function n_A_JobSearch2()
{ // 2nd Cls Hierachy
	if(n_A_JOB == cls_KNI || n_A_JOB == cls_LOR || n_A_JOB == cls_RUN || n_A_JOB == cls_RUNt)
		return cls_KNI;
	if(n_A_JOB == cls_ASS || n_A_JOB == cls_ASX || n_A_JOB == cls_GLT || n_A_JOB == cls_GLTt)
		return cls_ASS;
	if(n_A_JOB == cls_PRI || n_A_JOB == cls_HPR || n_A_JOB == cls_ABI || n_A_JOB == cls_ABIt)
		return cls_PRI;
	if(n_A_JOB == cls_HUN || n_A_JOB == cls_SNI || n_A_JOB == cls_RAN || n_A_JOB == cls_RANt)
		return cls_HUN;
	if(n_A_JOB == cls_WIZ || n_A_JOB == cls_HWI || n_A_JOB == cls_WAR || n_A_JOB == cls_WARt)
		return cls_WIZ;
	if(n_A_JOB == cls_BLA || n_A_JOB == cls_MAS || n_A_JOB == cls_MEC || n_A_JOB == cls_MECt)
		return cls_BLA;
	if(n_A_JOB == cls_CRU || n_A_JOB == cls_PAL || n_A_JOB == cls_ROY || n_A_JOB == cls_ROYt)
		return cls_CRU;
	if(n_A_JOB == cls_ROG || n_A_JOB == cls_STA || n_A_JOB == cls_SHA || n_A_JOB == cls_SHAt)
		return cls_ROG;
	if(n_A_JOB == cls_MON || n_A_JOB == cls_CHA || n_A_JOB == cls_SUR || n_A_JOB == cls_SURt)
		return cls_MON;
	if(n_A_JOB == cls_BAR || n_A_JOB == cls_CLO || n_A_JOB == cls_MIN || n_A_JOB == cls_MINt)
		return cls_BAR;
	if(n_A_JOB == cls_DAN || n_A_JOB == cls_GYP || n_A_JOB == cls_WAN || n_A_JOB == cls_WANt)
		return cls_DAN; // testing ...
	if(n_A_JOB == cls_SAG || n_A_JOB == cls_SCH || n_A_JOB == cls_SOR || n_A_JOB == cls_SORt)
		return cls_SAG;
	if(n_A_JOB == cls_ALC || n_A_JOB == cls_BIO || n_A_JOB == cls_GEN || n_A_JOB == cls_GENt)
		return cls_ALC;
	if(n_A_JOB == cls_TKM || n_A_JOB == cls_SL) // testing ...
		return n_A_JOB;
	if(n_A_JOB == cls_NIN || n_A_JOB == cls_KAGOB) // testing ...
		return cls_NIN;
		
	return cls_NOV;
}

function defReduction(Def)
{
	return 600.0 / ( Def + 600 );
}

function mdefReduction( MDef )
{
	return( 111.5 / ( MDef + 111.5 ) );
}

var ExpMod = [0.4, 1.15, 1.2, 1.25, 1.3, 1.35, 1.4, 1.35, 1.3, 1.25, 1.2, 1.15, 1.1, 1.05, 1, 0.95, 0.9, 0.85, 0.6, 0.35, 0.1];
function expModByLevelDiff( n_A_BaseLV, monLV )
{
	
	levelDiff = monLV - n_A_BaseLV;
	if ( levelDiff >= 16 )
	{
		return ExpMod[0];
	}
	else if ( levelDiff === 15 )
	{
		return ExpMod[1];
	}
	else if ( levelDiff === 14 )
	{
		return ExpMod[2];
	}
	else if ( levelDiff === 13 )
	{
		return ExpMod[3];
	}
	else if ( levelDiff === 12 )
	{
		return ExpMod[4];
	}
	else if ( levelDiff === 11 )
	{
		return ExpMod[5];
	}
	else if ( levelDiff === 10 )
	{
		return ExpMod[6];
	}
	else if ( levelDiff === 9 )
	{
		return ExpMod[7];
	}
	else if ( levelDiff === 8 )
	{
		return ExpMod[8];
	}
	else if ( levelDiff === 7 )
	{
		return ExpMod[9];
	}
	else if ( levelDiff === 6 )
	{
		return ExpMod[10];
	}
	else if ( levelDiff === 5 )
	{
		return ExpMod[11];
	}
	else if ( levelDiff === 4 )
	{
		return ExpMod[12];
	}
	else if ( levelDiff === 3 )
	{
		return ExpMod[13];
	}
	else if ( levelDiff <= 2 && levelDiff >= (-5) )
	{
		return ExpMod[14];
	}
	else if ( levelDiff <= (-6) && levelDiff >= (-10) )
	{
		return ExpMod[15];
	}
	else if ( levelDiff <= (-11) && levelDiff >= (-15) )
	{
		return ExpMod[16];
	}
	else if ( levelDiff <= (-16) && levelDiff >= (-20) )
	{
		return ExpMod[17];
	}
	else if ( levelDiff <= (-21) && levelDiff >= (-25) )
	{
		return ExpMod[18];
	}
	else if ( levelDiff <= (-26) && levelDiff >= (-30) )
	{
		return ExpMod[19];
	}
	else
	{
		return ExpMod[20];
	}
}

function calcJobStats( n_A_JOB, n_A_JobLV, w2 )
{
	// JobBOBJ --> etc.js ln 1
	for( var i = 0; JobBOBJ[n_A_JOB][i] <= n_A_JobLV && JobBOBJ[n_A_JOB][i] != "n"; i += 2 )
	{
		w2[JobBOBJ[n_A_JOB][i+1]] += 1;
	}
	if ( n_A_JOB === cls_NOV && rebirthClass)
	{ // Novi && Rebirth
		for(var i=0;JobBOBJ[cls_HNOV][i] <= n_A_JobLV && JobBOBJ[cls_HNOV][i] != "n";i+=2)
				w2[JobBOBJ[cls_HNOV][i+1]] += 1;
	}
	if ( ( ( n_A_JOB === cls_SNOVI && n_A_JobLV >= 70 ) || n_A_JOB === cls_ENOVI ) && SkillSearch( skill_ALL_NO_DEATH_BONUS ) )
	{ // No Death Bonus
		for ( var i = 0; i < 6; i++ )
		{ // Stats+10
			w2[i] += 10;
		}
	}
	
	return w2;
}

function SkillSearch( skillIndex )
{ // get passive skills - (skillID) -> sLvl
	if ( skillIndex == 258 && TimeItemNumSearch( 35 ) )
	{
		return 1;
	}

	for ( var k = 0; JobSkillPassOBJ[n_A_JOB][k] != 999; k++)
	{
		if ( JobSkillPassOBJ[n_A_JOB][k] == skillIndex )
		{
			return selfBuffs[k];
		}
	}
	
	return 0;
}

function ResetPassiveSkills()
{ // reset passive skills
	for ( var i = 0; i < selfBuffs.length; i++ )
	{
		selfBuffs[i] = 0;
	}
}

function AdjustJobLevelList( job )
{
	var formElements = document.forms["calcForm"].elements;
	var maxJobLvl = 0;
	
	// Calc Max Job Level
	if ( job === cls_NOV || job === cls_HNOV ) // Novi/ HNovi
	{
		maxJobLvl = 10;
	}
	else if ( job <= cls_ALC || job == cls_KAGOB || ( cls_HSWO <= job && job <= cls_SL ) || job == cls_ENOVI ) // 1st~3rd
	{
		maxJobLvl = 50;
	}
	else if ( job === cls_SNOVI ) // SNovi
	{
		maxJobLvl = 99;
	}
        else if ( cls_RUN <= job && job <= cls_GENt )
        {
		maxJobLvl = 60;
        }    
	else
	{
		maxJobLvl = 70; // 2nd Adv
	}
	
	// save old JLvl
	if ( maxJobLvl < formElements["A_JobLV"].value )
	{
		formElements["A_JobLV"].value = maxJobLvl;
	}
	
	// Adjust Max Job Level List
	var len = formElements["A_JobLV"].length;
	
	if ( len > maxJobLvl )
	{
		for ( var i = len; i !== maxJobLvl; i-- )
		{
			// delete options
			formElements["A_JobLV"].options[i - 1] = null;
		}
	}
	else if ( len < maxJobLvl )
	{
		for ( var i = len; i !== maxJobLvl; i++ )
		{
			// add option
			formElements["A_JobLV"].options[i] = new Option( i + 1, i + 1 );
		}
	}
}

function AdjustBaseLevelList( job )
{
	var formElements = document.forms["calcForm"].elements;
	var maxBaseLvl = 0;
	
	if ( thirdClass === 1 ) // 3rdCls
	{
		maxBaseLvl=CONST_MAXLVL_THIRD-CONST_MAXLVL; // amount of BLvl [99, 175]
	}
	else
	{
                if (job == cls_KAGOB || job == cls_ENOVI) {
                    maxBaseLvl=CONST_MAXLVL_KAGOB_ENOVI-CONST_MAXLVL; // amount of BLvl [99, 160]
                } else {
                    maxBaseLvl=CONST_MAXLVL; // amount of BLvl [1, 99]
                }
	}
	
	// Adjust Max Base Level List
	var len = formElements["A_BaseLV"].length;
	
	if ( len > maxBaseLvl )
	{
		// if new maxBLvl < old maxBLvl (-> 3rd)
		for ( var i = len; i !== maxBaseLvl; i-- )
		{
			// delete options
			formElements["A_BaseLV"].options[i - 1] = null;
		}
		for ( var i = 0; i !== ( maxBaseLvl + 1 ); i++ )
		{
			// refresh labels
			formElements["A_BaseLV"].options[i] = new Option( i + 99, i + 99 );
		}
	}
	else if ( len < maxBaseLvl )
	{
		if ( thirdClass === 1 ) {
		    for ( var i = maxBaseLvl; i !== len; i-- )
		    {
			    // delete options
			    formElements["A_BaseLV"].options[i - 1] = null;
		    }
		    for ( var i = 0; i !== ( maxBaseLvl + 1 ); i++ )
		    {
			    // refresh labels
			    formElements["A_BaseLV"].options[i] = new Option( i + 99, i + 99 );
		    }
		} else {
		    // (-> 2nd)
		    for ( var i = 0; i !== maxBaseLvl; i++ )
		    {
			    // add & refresh options
			    formElements["A_BaseLV"].options[i] = new Option( i + 1, i + 1 );
		    }
		}
	}
}

function AdjustStatLists( job )
{
	var strSelect = document.forms["calcForm"].elements["A_STR"];
	var agiSelect = document.forms["calcForm"].elements["A_AGI"];
	var vitSelect = document.forms["calcForm"].elements["A_VIT"];
	var intSelect = document.forms["calcForm"].elements["A_INT"];
	var dexSelect = document.forms["calcForm"].elements["A_DEX"];
	var lukSelect = document.forms["calcForm"].elements["A_LUK"];
	var maxStatLvl = 0;
	
	// Stats [list refresh]
	if ( thirdClass === 1 ) // third cls
	{
		maxStatLvl = CONST_MAXSTAT_THIRD; // maxStats
	}
	else
	{
                if (job == cls_KAGOB || job == cls_ENOVI) {
                    maxStatLvl = CONST_MAXSTAT_KAGOB_ENOVI;
                } else {
                    maxStatLvl = CONST_MAXSTAT; // maxStats
                }
	}
	
	// Adjust Max Stat Level Lists
	var len = strSelect.length;
	
	if ( len > maxStatLvl )
	{ // (-> 2nd)
		// save old Stats
		if ( maxStatLvl < strSelect.value )
		{
			strSelect.value = maxStatLvl;
		}
		if ( maxStatLvl < agiSelect.value )
		{
			agiSelect.value = maxStatLvl;
		}
		if ( maxStatLvl < vitSelect.value )
		{
			vitSelect.value = maxStatLvl;
		}
		if ( maxStatLvl < intSelect.value )
		{
			intSelect.value = maxStatLvl;
		}
		if ( maxStatLvl < dexSelect.value )
		{
			dexSelect.value = maxStatLvl;
		}
		if ( maxStatLvl < lukSelect.value )
		{
			lukSelect.value = maxStatLvl;
		}

		for ( i = len; i !== maxStatLvl; i-- )
		{
			// delete options
			strSelect.options[i - 1] = null;
			agiSelect.options[i - 1] = null;
			vitSelect.options[i - 1] = null;
			intSelect.options[i - 1] = null;
			dexSelect.options[i - 1] = null;
			lukSelect.options[i - 1] = null;
		}
	}
	else if ( len < maxStatLvl )
	{ // (-> 3rd)
		for ( i = len; i !== maxStatLvl; i++ )
		{
			// add options
			strSelect.options[i] = new Option( i + 1, i + 1 );
			agiSelect.options[i] = new Option( i + 1, i + 1 );
			vitSelect.options[i] = new Option( i + 1, i + 1 );
			intSelect.options[i] = new Option( i + 1, i + 1 );
			dexSelect.options[i] = new Option( i + 1, i + 1 );
			lukSelect.options[i] = new Option( i + 1, i + 1 );
		}
	}
}

function AdjustSpeedPotList( job )
{
	var speedPotSelect = document.forms["calcForm"].elements["speedPot"];
	
	// SpeedPot [list refresh]
	for ( var i = 2; i <= 3; i++ )
	{
		speedPotSelect.options[2] = null;
	}

	// Fill options for awake
	if ( n_A_JOB !== 3 && n_A_JobSearch2() !== 9 && n_A_JobSearch2() !== 16 )
	{
		speedPotSelect.options[2] = new Option( SpeedPotName[2][Language] + "(Lv40)", 2 );
	}
	else
	{
		speedPotSelect.options[2] = new Option( "-", 0 );
	}
	
	// Fill options for beserk
	if ( n_A_JobSearch() === 1   || n_A_JobSearch() === 6   || 
	     n_A_JobSearch() === 41  || n_A_JobSearch2() === 14 || 
		 n_A_JobSearch2() === 11 || n_A_JOB === 5           || 
		 n_A_JOB === 45 )
	{
		speedPotSelect.options[3] = new Option( SpeedPotName[3][Language] + "(Lv85)", 3 );
	}
	else if ( n_A_JOB === 22 )
	{
		speedPotSelect.options[3] = new Option( "(" + SkillOBJ[304][2] + "Lv85)/" + GetWord( 86 ), 3 );
	}
	else
	{
		speedPotSelect.options[3] = new Option( "(" + SkillOBJ[304][2] + ")(Lv85)", 3 );
	}
}

function AdjustWeaponTypeList( job )
{
	var weaponTypeSelect = document.forms["calcForm"].elements["A_WeaponType"];
	
	// WeaponType [list refresh]
	for( var i = 21; i >= 0; i-- )
	{
		// clear items
		weaponTypeSelect.options[i] = null;
	}
	
	var weaponTypeArray = new Array();
	
	for ( var i = 0; i <= 21; i++ )
	{
		if ( JobASPD[job][i] != 0 )
		{
		    if (i > 0) {
			weaponTypeArray.push([WeaponName[i][Language], i]);
		    }
		    
		}
	}
	
	weaponTypeArray.sort();	
	weaponTypeArray.unshift([WeaponName[0][Language], 0]);
	
	for (var i = 0; i < weaponTypeArray.length; i++) {
	    weaponTypeSelect.options[i] = new Option(weaponTypeArray[i][0], weaponTypeArray[i][1]);
	}
	
	weaponTypeArray;
}

function AdjustActiveSkillList( job )
{
	//player active skills
	var activeSkillsSelect = document.forms["calcForm"].elements["A_ActiveSkill"];
	
	// active skills [list refresh]	
	var len = activeSkillsSelect.length;
	
	for ( var i = 0; i < len; i++ )
	{
		//activeSkillsSelect.options[0] = null;
		$('select[name="A_ActiveSkill"]').children().remove();
	}
	
	$('select[name="A_ActiveSkill"]').append('<option value="0">Basic Attack</option>'); // Always put Basic Attack
	
	for (var i = 0; i < JobSkillTreeActiveOBJ[job].length; i++) {
	    var jobSkillTree = JobSkillTreeActiveOBJ[job][i][0];
	    var jobSkillTreeSkills = JobSkillTreeActiveOBJ[job][i][1];
	    
	    AppendOptGroup(jobSkillTree);
	    for (var jobID = 0; jobID < jobSkillTreeSkills.length; jobID++) {
		$('select[name="A_ActiveSkill"] optgroup[name="jobSkillTree' + jobSkillTree + '"]').append('<option value=" '+ jobSkillTreeSkills[jobID] + '">' + SKILL_NAME[jobSkillTreeSkills[jobID]][Language] + '</option>');
	    }
	}
	
	for ( var i = 0; JobSkillActiveOBJ[job][i] !== 999; i++ )
	{
		//$A_ActiveSkill.children('optgroup').append('<option value="' + JobSkillActiveOBJ[job][i] + '">' + SKILL_NAME[JobSkillActiveOBJ[job][i]][Language] + '</option>');
		
		
		//activeSkillsSelect.options[i] = new Option( SKILL_NAME[JobSkillActiveOBJ[job][i]][Language], JobSkillActiveOBJ[job][i] );
	}
	
	for ( var i = 0; i < 20; i++ )
	{
		w_ASSP0bk[i] = 999;
	}
	
	ActiveSkillSetPlus();
	ClickActiveSkill();
	WeaponSet2();
}

function AppendOptGroup (optGroupId) {
    $('select[name="A_ActiveSkill"]').append('<optgroup name="jobSkillTree' + optGroupId + '" label="' + JobSkillTreeNameOBJ[optGroupId] + '"></optgroup>');
}

function BuildPassiveSkillTable()
{
	var job = n_A_JOB;
	var skillCount = 0;
	var rowCaptionA = 'class="bgLtRow3 padded optCaption"';
	var rowAreaA = 'class="bgLtRow1 padded optArea"';
	var rowCaptionB = 'class="bgLtRow4 padded optCaption"';
	var rowAreaB = 'class="bgLtRow2 padded optArea"';
	var formElements = document.forms["calcForm"].elements;
	
	// Calc total number of passive skills
	for ( skillCount; JobSkillPassOBJ[job][skillCount] !== 999; skillCount++ );
	if ( skillCount <= 0 )
	{
		$('#id_passiveSkills table').remove();
		return;
	}

	// Build Skill Table
	var str = '<table class="bgLtTable">';
	var rowCount = 0;
	for ( var i = 0; i < skillCount; i += 2 )
	{
		var rowCaptionClass = rowCaptionA;
		var rowAreaClass = rowAreaA;
		if ( rowCount % 2 === 1 )
		{
			rowCaptionClass = rowCaptionB;
			rowAreaClass = rowAreaB;
		}
		str += '<tr>';
		str += '<td ' + rowCaptionClass + ' id="P_Skill' + i         + '"></td>';
		str += '<td ' + rowAreaClass    + ' id="P_Skill' + i         + 's"></td>';
		str += '<td ' + rowCaptionClass + ' id="P_Skill' + ( i + 1 ) + '"></td>';
		str += '<td ' + rowAreaClass    + ' id="P_Skill' + ( i + 1 ) + 's"></td>';
		str += '</tr>';
		rowCount++;
	}
	str += '</table>';		
	myInnerHtml( "id_passiveSkills", str, 0 );

	// Build Skill Labels and Option Boxes
	for ( var i = 0; i < skillCount; i++ )
	{
		if ( JobSkillPassOBJ[job][i] != 999 ) // 999 is end of record
		{
			myInnerHtml( "P_Skill" + i, SKILL_NAME[JobSkillPassOBJ[job][i]][Language], 0 );
			myInnerHtml( "P_Skill" + i + "s", '<select name="A_Skill' + i + '" style="width:50px;" onchange="TogglePassiveSkills()"></select>',0);
		}
	}

	// Fill Option Boxes
	for ( var j = 0; j < skillCount; j++ )
	{ // SkillLevel
		var skillIndex = JobSkillPassOBJ[job][j];
		// on-off skills
		var toggleSkills = [12,68,74,152,153,155,196,253,258,301,309,310,322,345,364,365,383,379,385,386,389,390,392,420,421,422];

		if ( skillIndex !== 999 )
		{
			if ( SkillOBJ[skillIndex][1] === 1 || NumSearch( skillIndex, toggleSkills ) )
			{ // Skill only has 1 level to mastery, so just make it on/off
				var docElement = formElements["A_Skill" + j];
				docElement.options[0] = new Option( "Off", 0 );
				docElement.options[1] = new Option( "On", 1 );
			}
			else
			{
				var docElement = formElements["A_Skill" + j];
				for ( var i = docElement.length = 1; i >= 0; i-- )
				{
					docElement.options[i] = null;
				}
				for ( var i = 0; i <= SkillOBJ[JobSkillPassOBJ[job][j]][1]; i++ )
				{
					docElement.options[i] = new Option( i, i );
				}
			}
		}
	}

	// Handle Special Skills
	var skillASelect = formElements["A_Skill0"];
	var skillBSelect = formElements["A_Skill5"];
	var skillCSelect = formElements["A_Skill8"];
	var skillDSelect = formElements["A_Skill9"];
	var skillESelect = formElements["A_Skill11"];
	var skillFSelect = formElements["A_Skill6"];
	var skillGSelect = formElements["A_Skill7"];
	
	// Energy Coat
	if ( JobSkillPassOBJ[job][0] === skill_MA_ENERGY_COAT )
	{ 
		for ( var i = 10; i >= 0; i-- )
		{
			skillASelect.options[i] = null;
		}
		var w_ECname=["Off","6%","12%","18%","24%","30%"];
		for ( var i = 1; i <= 5; i++ )
		{
			w_ECname[i] += GetWord( 224 );
		}
		for( var i = 0; i <= 5; i++ )
		{
			skillASelect.options[i] = new Option( w_ECname[i], i );
		}
		// adjust the width
		skillASelect.style.width = 110+'px';
	}

	// Cavalier Mastery for Knight
	if ( JobSkillPassOBJ[job][5] === skill_KN_CAVALIER_MASTERY )
	{
		for ( var i = 10; i >= 0; i-- )
		{
			skillBSelect.options[i] = null;
		}
		var w_name = new Array();
		w_name[0] = GetWord(225);
		for ( var i = 1; i <= 6; i++ )
		{
			w_name[i] = GetWord(226) + (i-1);
		}
		for ( var i = 0; i <= 6; i++ )
		{
			skillBSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillBSelect.style.width = 85+'px';
	}
	
	// Dragon Training for Rune Knight
	if ( JobSkillPassOBJ[job][5] === skill_RUN_DRAGON_TRAINING )
	{
		for ( var i = 10; i >= 0; i-- )
		{
			skillBSelect.options[i] = null;
		}
		var w_name = new Array();
		w_name[0] = GetWord(251);
		for ( var i = 1; i < 6; i++ )
		{
			w_name[i] = GetWord(226) + i;
		}
		for ( var i = 0; i < 6; i++ )
		{
			skillBSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillBSelect.style.width = 85+'px';
	}
	
	// Cavalier Mastery for Sader
	if ( JobSkillPassOBJ[job][9] === skill_KN_CAVALIER_MASTERY )
	{
		for ( var i = 10; i >= 0; i-- )
		{
			skillDSelect.options[i] = null;
		}
		var w_name = new Array();
		w_name[0] = GetWord(225);
		for ( var i = 1; i <= 6; i++ )
		{
			w_name[i] = GetWord(226) + (i-1);
		}
		for ( var i = 0; i <= 6; i++ )
		{
			skillDSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillDSelect.style.width = 85+'px';
	}
	
	if ( JobSkillPassOBJ[job][11] === skill_TKM_BLESSINGS )
	{ // Solar/Lunar/Stellar Bless
		for ( var i = 10; i >= 0; i-- )
		{
			skillESelect.options[i] = null;
		}
		var blessingValues=["Off","+10% EXP","+20% EXP","+30% EXP","+40% EXP","+50% EXP"];
		for ( var i = 0; i < 6; i++ )
		{
			skillESelect.options[i] = new Option( blessingValues[i], i );
		}
		// adjust the width
		skillESelect.style.width = 85+'px';
	}
	
	if (JobSkillPassOBJ[job][7] === skill_SOR_SPIRIT_CONTROL) {
		for ( var i = 3; i >= 0; i-- )
		{
			skillGSelect.options[i] = null;
		}
		var w_name=["0 (Idle)","1 (Passive)","2 (Defense)","3 (Offense)"];
		for ( var i = 0; i <= 3; i++ )
		{
			skillGSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillGSelect.style.width = 95+'px';
	}
	
	if (JobSkillPassOBJ[job][8] === skill_SOR_SPIRIT_CONTROL) {
		for ( var i = 3; i >= 0; i-- )
		{
			skillCSelect.options[i] = null;
		}
		var w_name=["0 (Idle)","1 (Passive)","2 (Defense)","3 (Offense)"];
		for ( var i = 0; i <= 3; i++ )
		{
			skillCSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillCSelect.style.width = 95+'px';
	}
	
	if ( JobSkillPassOBJ[job][8] === skill_SOR_SUMMON_TYPE )
	{
		for ( var i = 3; i >= 0; i-- )
		{
			skillCSelect.options[i] = null;
		}
		var w_name=["Agni","Ventus","Aqua","Terra"];
		for ( var i = 0; i <= 3; i++ )
		{
			skillCSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillCSelect.style.width = 85+'px';
	}
	
	if ( JobSkillPassOBJ[job][9] === skill_SOR_SUMMON_TYPE )
	{
		for ( var i = 3; i >= 0; i-- )
		{
			skillDSelect.options[i] = null;
		}
		var w_name=["Agni","Ventus","Aqua","Terra"];
		for ( var i = 0; i <= 3; i++ )
		{
			skillDSelect.options[i] = new Option(w_name[i],i);
		}
		// adjust the width
		skillDSelect.style.width = 85+'px';
	}

	for ( var i = 0; JobSkillPassOBJ[n_A_JOB][i] !== 999; i++ )
	{
		var wOBJ = formElements["A_Skill" + i];
		wOBJ.value = selfBuffs[i];
	}
	if ( JobSkillPassOBJ[job][6] === skill_KAG_GET_ELEMENTAL_SEAL )
	{
		for ( var i = 10; i >= 0; i-- )
		{
			skillFSelect.options[i] = null;
		}
		var w_name = new Array("Water","Earth","Fire","Wind");
		for ( var i = 0; i <= 3; i++ )
		{
			skillFSelect.options[i] = new Option(w_name[i],i+1);
		}
		// adjust the width
		skillFSelect.style.width = 85+'px';
	}
}

function FillPerformerBuffOptions()
{
	var musicLessonsSelect = document.getElementById( 'musicLessons' );
	var voiceLessonsMSelect = document.getElementById( 'voiceLessonsM' );
	var bardAgiSelect = document.getElementById( 'bardAgi' );
	var bardIntSelect = document.getElementById( 'bardInt' );
	var bardDexSelect = document.getElementById( 'bardDex' );
	var bardVitSelect = document.getElementById( 'bardVit' );
	var danceLessonsSelect = document.getElementById( 'danceLessons' );
	var voiceLessonsWSelect = document.getElementById( 'voiceLessonsW' );
	var dancerAgiSelect = document.getElementById( 'dancerAgi' );
	var dancerIntSelect = document.getElementById( 'dancerInt' );
	var dancerDexsSelect = document.getElementById( 'dancerDex' );
	var dancerLuksSelect = document.getElementById( 'dancerLuk' );
	var marStrSelect = document.getElementById( 'marionetteStr' );
	var marAgiSelect = document.getElementById( 'marionetteAgi' );
	var marVitSelect = document.getElementById( 'marionetteVit' );
	var marIntSelect = document.getElementById( 'marionetteInt' );
	var marDexSelect = document.getElementById( 'marionetteDex' );
	var marLukSelect = document.getElementById( 'marionetteLuk' );
	var bSkillLvlSelect = document.getElementById( 'bardSkillLevel' );
	var mSkillLvlSelect = document.getElementById( 'maestroSkillLevel' );
	var dSkillLvlSelect = document.getElementById( 'dancerSkillLevel' );
	var wSkillLvlSelect = document.getElementById( 'wandererSkillLevel' );
	var bardLevelSelect = document.getElementById( 'bardLevel' );
	var dancerLevelSelect = document.getElementById( 'dancerLevel' );
	var chorusLevelSelect = document.getElementById( 'chorusLevel' );
	var performCountSelect = document.getElementById( 'performerCount' );
	var bardJobSelect = document.getElementById( 'bardJob' );
	var dancerJobSelect = document.getElementById( 'dancerJob' );

	// clear select objects
	for ( var i = 1; i < musicLessonsSelect.length; i++ )
	{
		musicLessonsSelect.remove( i );
	}
	for ( var i = 1; i < voiceLessonsMSelect.length; i++ )
	{
		voiceLessonsMSelect.remove( i );
	}
	for ( var i = 1; i < bardAgiSelect.length; i++ )
	{
		bardAgiSelect.remove( i );
	}
	for ( var i = 1; i < bardIntSelect.length; i++ )
	{
		bardIntSelect.remove( i );
	}
	for ( var i = 1; i < bardDexSelect.length; i++ )
	{
		bardDexSelect.remove( i );
	}
	for ( var i = 1; i < bardVitSelect.length; i++ )
	{
		bardVitSelect.remove( i );
	}
	for ( var i = 1; i < danceLessonsSelect.length; i++ )
	{
		danceLessonsSelect.remove( i );
	}
	for ( var i = 1; i < voiceLessonsWSelect.length; i++ )
	{
		voiceLessonsWSelect.remove( i );
	}
	for ( var i = 1; i < dancerAgiSelect.length; i++ )
	{
		dancerAgiSelect.remove( i );
	}
	for ( var i = 1; i < dancerIntSelect.length; i++ )
	{
		dancerIntSelect.remove( i );
	}
	for ( var i = 1; i < dancerDexsSelect.length; i++ )
	{
		dancerDexsSelect.remove( i );
	}
	for ( var i = 1; i < dancerLuksSelect.length; i++ )
	{
		dancerLuksSelect.remove( i );
	}
	for ( var i = 1; i < marStrSelect.length; i++ )
	{
		marStrSelect.remove( i );
	}
	for ( var i = 1; i < marAgiSelect.length; i++ )
	{
		marAgiSelect.remove( i );
	}
	for ( var i = 1; i < marVitSelect.length; i++ )
	{
		marVitSelect.remove( i );
	}
	for ( var i = 1; i < marIntSelect.length; i++ )
	{
		marIntSelect.remove( i );
	}
	for ( var i = 1; i < marDexSelect.length; i++ )
	{
		marDexSelect.remove( i );
	}
	for ( var i = 1; i < marLukSelect.length; i++ )
	{
		marLukSelect.remove( i );
	}
	for ( var i = 1; i < bSkillLvlSelect.length; i++ )
	{
		bSkillLvlSelect.remove( i );
	}
	for ( var i = 1; i < mSkillLvlSelect.length; i++ )
	{
		mSkillLvlSelect.remove( i );
	}
	for ( var i = 1; i < dSkillLvlSelect.length; i++ )
	{
		dSkillLvlSelect.remove( i );
	}
	for ( var i = 1; i < wSkillLvlSelect.length; i++ )
	{
		wSkillLvlSelect.remove( i );
	}
	for ( var i = 1; i < bardLevelSelect.length; i++ )
	{
		bardLevelSelect.remove( i );
	}
	for ( var i = 1; i < dancerLevelSelect.length; i++ )
	{
		dancerLevelSelect.remove( i );
	}
	for ( var i = 1; i < chorusLevelSelect.length; i++ )
	{
		chorusLevelSelect.remove( i );
	}
	for ( var i = 1; i < performCountSelect.length; i++ )
	{
		performCountSelect.remove( i );
	}
	
	for ( var i = 1; i < bardJobSelect.length; i++ )
	{
		bardJobSelect.remove( i );
	}
	for ( var i = 1; i < dancerJobSelect.length; i++ )
	{
		dancerJobSelect.remove( i );
	}
	
	// fill select objects
	musicLessonsSelect.options[0] = new Option( 'Music Lessons', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		musicLessonsSelect.options[i] = new Option( i, i );
	}
	voiceLessonsMSelect.options[0] = new Option( 'Voice Lessons', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		voiceLessonsMSelect.options[i] = new Option( i, i );
	}
	bardAgiSelect.options[0] = new Option( 'AGI', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		bardAgiSelect.options[i] = new Option( i, i );
	}
	bardIntSelect.options[0] = new Option( 'INT', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		bardIntSelect.options[i] = new Option( i, i );
	}
	bardDexSelect.options[0] = new Option( 'DEX', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		bardDexSelect.options[i] = new Option( i, i );
	}
	bardVitSelect.options[0] = new Option( 'VIT', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		bardVitSelect.options[i] = new Option( i, i );
	}
	danceLessonsSelect.options[0] = new Option( 'Dance Lessons', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		danceLessonsSelect.options[i] = new Option( i, i );
	}
	voiceLessonsWSelect.options[0] = new Option( 'Voice Lessons', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		voiceLessonsWSelect.options[i] = new Option( i, i );
	}
	dancerAgiSelect.options[0] = new Option( 'AGI', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		dancerAgiSelect.options[i] = new Option( i, i );
	}
	dancerIntSelect.options[0] = new Option( 'INT', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		dancerIntSelect.options[i] = new Option( i, i );
	}
	dancerDexsSelect.options[0] = new Option( 'DEX', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		dancerDexsSelect.options[i] = new Option( i, i );
	}
	dancerLuksSelect.options[0] = new Option( 'LUK', 1 );
	for ( var i = 1; i <= 200; i++ )
	{
		dancerLuksSelect.options[i] = new Option( i, i );
	}
	marStrSelect.options[0] = new Option( 'STR', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marStrSelect.options[i] = new Option( i, i );
	}
	marAgiSelect.options[0] = new Option( 'AGI', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marAgiSelect.options[i] = new Option( i, i );
	}
	marVitSelect.options[0] = new Option( 'VIT', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marVitSelect.options[i] = new Option( i, i );
	}
	marIntSelect.options[0] = new Option( 'INT', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marIntSelect.options[i] = new Option( i, i );
	}
	marDexSelect.options[0] = new Option( 'DEX', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marDexSelect.options[i] = new Option( i, i );
	}
	marLukSelect.options[0] = new Option( 'LUK', 1 );
	for ( var i = 1; i <= 150; i++ )
	{
		marLukSelect.options[i] = new Option( i, i );
	}
	bSkillLvlSelect.options[0] = new Option( 'Skill Level', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		bSkillLvlSelect.options[i] = new Option( i, i );
	}
	mSkillLvlSelect.options[0] = new Option( 'Skill Level', 0 );
	for ( var i = 1; i <= 5; i++ )
	{
		mSkillLvlSelect.options[i] = new Option( i, i );
	}
	dSkillLvlSelect.options[0] = new Option( 'Skill Level', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		dSkillLvlSelect.options[i] = new Option( i, i );
	}
	wSkillLvlSelect.options[0] = new Option( 'Skill Level', 0 );
	for ( var i = 1; i <= 5; i++ )
	{
		wSkillLvlSelect.options[i] = new Option( i, i );
	}
	bardLevelSelect.options[0] = new Option( 'Bard', 0 );
	for ( var i = 1; i <= 5; i++ )
	{
		bardLevelSelect.options[i] = new Option( i, i );
	}
	dancerLevelSelect.options[0] = new Option( 'Dancer', 0 );
	for ( var i = 1; i <= 5; i++ )
	{
		dancerLevelSelect.options[i] = new Option( i, i );
	}
	chorusLevelSelect.options[0] = new Option( 'Skill Level', 0 );
	for ( var i = 1; i <= 5; i++ )
	{
		chorusLevelSelect.options[i] = new Option( i, i );
	}
	performCountSelect.options[0] = new Option( 'Performers', 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		performCountSelect.options[i] = new Option( i + 1, i + 1 );
	}
	bardJobSelect.options[0] = new Option( 'Job Lvl', 1 );
	for ( var i = 1; i <= 50; i++ )
	{
		bardJobSelect.options[i] = new Option( i, i );
	}
	dancerJobSelect.options[0] = new Option( 'Job Lvl', 1 );
	for ( var i = 1; i <= 50; i++ )
	{
		dancerJobSelect.options[i] = new Option( i, i );
	}
}

function BuildAcolyteBuffsTable()
{
with(document.calcForm)
{
	n_SkillSW = 1;

	var str;
	str =  '<table class="bgLtTable"><tr>';
	str += '<td id="AS0_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS0_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="AS1_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS1_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS2_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS2_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="AS3_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS3_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS4_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS4_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="AS5_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS5_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS6_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS6_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="AS7_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS7_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS8_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS8_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="AS9_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS9_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS10_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS10_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="AS11_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS11_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS12_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS12_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="AS13_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS13_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS14_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS14_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="AS15_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="AS15_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="AS16_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="AS16_2" colspan="3" class="bgLtRow1 padded optArea"></td>';
	str += '</tr></table>';
	myInnerHtml( "SIENSKILL", str , 0 );

	// Labels
	var w_name = [31,27,30,90,91,100,267,0,493,494,505,508,499,500,641,642,0];
	for( var i = 0; i < w_name.length; i++ )
	{
		w_name[i] = SKILL_NAME[w_name[i]][Language];
	}
	w_name[7] = GetWord(102); // Spheres
	w_name[16] = "Sura stats";
	for ( var i = 0 ; i < 17; i++ ) // fill labels
	{
		myInnerHtml("AS"+i+"_1",w_name[i],0);
	}
	
	// Select Boxes
	var html_CSSW_SKILL = new Array();
	html_CSSW_SKILL[0] = '<select id="blessing" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[1] = '<select id="increaseAgi" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[2] = '<select id="angelus" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[3] = '<select id="imposito" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[4] = '<select id="suffragium" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[5] = '<input type="checkbox" id="gloria" onclick="ToggleAcolyteSkills(0)">';
	html_CSSW_SKILL[6] = '<select id="assumptio" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[7] = '<select id="spheres" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[8] = '<select id="clementia" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[9] = '<select id="candidus" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[10] = '<select id="expiatio" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[11] = '<select id="sacrament" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[12] = '<select id="agnus" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[13] = '<select id="ramus" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	html_CSSW_SKILL[14] = '<select id="ppChange" style="width:50px;" onchange="ToggleAcolyteSkills(1)"></select>';
	html_CSSW_SKILL[15] = '<select id="ppRevitalize" style="width:50px;" onchange="ToggleAcolyteSkills(2)"></select>';
	html_CSSW_SKILL[16] = '<select id="suraStr" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>&nbsp';
	html_CSSW_SKILL[17] = '<select id="suraAgi" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>&nbsp';
	html_CSSW_SKILL[18] = '<select id="suraVit" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>&nbsp';
	html_CSSW_SKILL[19] = '<select id="suraInt" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>&nbsp';
	html_CSSW_SKILL[20] = '<select id="suraDex" style="width:50px;" onchange="ToggleAcolyteSkills(0)"></select>';
	for ( var i = 0; i < 16; i++ )
	{ // insert Drop Down boxes
		myInnerHtml( "AS"+i+"_2", html_CSSW_SKILL[i], 0 );
	}
	myInnerHtml( "AS16_2", html_CSSW_SKILL[16] + html_CSSW_SKILL[17] + html_CSSW_SKILL[18] + html_CSSW_SKILL[19] + html_CSSW_SKILL[20], 0 );

	// Fill Selects with Options
	for ( var i = 0; i <= 10; i++ )
	{ // lvl10 skills
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		blessing.options[i] = new Option( off, i ); // Bless
		increaseAgi.options[i] = new Option( off, i ); // Agi
		angelus.options[i] = new Option( off, i ); // Angelus
	}
	
	for ( var i = 0; i <= 5; i++ )
	{ // lvl5 skills
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		imposito.options[i] = new Option( off, i ); // Imposito Manus
		assumptio.options[i] = new Option( off, i ); // Assumptio
		spheres.options[i] = new Option( off, i ); // Spheres
		sacrament.options[i] = new Option( off, i ); // Sacrament
		expiatio.options[i] = new Option( off, i ); // expiatio
		ppChange.options[i] = new Option( off, i ); // ppChange
		ppRevitalize.options[i] = new Option( off, i ); // ppRevitalize
		clementia.options[i] = new Option( off, i ); // clementia
		candidus.options[i] = new Option( off, i ); // candidus
	}
	
	// Spirit Spheres
	if ( n_A_JobSearch2() === 15 )
	{
		myInnerHtml("AS10_1","-",0); // SphereTxt (?)
	}
	
	for ( var i = 0; i < 4; i++ ) // lvl3 skills
	{
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		suffragium.options[i] = new Option( off, i ); // Suff
	}
	for ( var i = 0; i < 5; i++ ) // lvl4 skills
	{
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		agnus.options[i] = new Option( off, i ); // Agnus
		ramus.options[i] = new Option( off, i ); // Ramus
	}
	
	// Sura Stats
	suraStr.options[0] = new Option( "STR", 1 ); // str
	suraAgi.options[0] = new Option( "AGI", 1 ); // Agi
	suraVit.options[0] = new Option( "VIT", 1 ); // vit
	suraInt.options[0] = new Option( "INT", 1 ); // int
	suraDex.options[0] = new Option( "DEX", 1 ); // dex
	for ( var i = 1; i <= 200; i++ )
	{		
		suraStr.options[i] = new Option( i, i ); // str
		suraAgi.options[i] = new Option( i, i ); // Agi
		suraVit.options[i] = new Option( i, i ); // vit
		suraInt.options[i] = new Option( i, i ); // int
		suraDex.options[i] = new Option( i, i ); // dex
	}
}
}

function BuildOtherBuffsTable()
{
with(document.calcForm)
{
	// Build Table
	var str
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="EN60_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN60_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN61_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN61_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN62_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN62_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="EN63_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN63_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="EN64_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN64_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="EN65_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN65_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="EN66_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN66_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN67_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN67_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN68_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN68_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="EN69_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN69_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="EN610_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN610_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="EN611_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="EN611_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="EN612_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN612_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN613_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN613_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN614_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN614_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="EN615_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN615_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN616_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN616_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="EN617_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="EN617_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr></table>';
	myInnerHtml("OtherBuffs",str,0);

	// Volcano, Deluge, Whirlwind
	myInnerHtml("EN60_1",'<select name="eleField" style="width:110px;" onchange="StAllCalc()"></select>',0);
	myInnerHtml("EN60_2",'<select name="eleFieldLvl" style="width:50px;" onchange="ToggleOtherBuffs()"></select>',0);
	eleField.options[0] = new Option(SKILL_NAME[235][Language],0);
	eleField.options[1] = new Option(SKILL_NAME[236][Language],1);
	eleField.options[2] = new Option(SKILL_NAME[237][Language],2);

	// MurdererBonus
	myInnerHtml("EN61_1",GetWord(122),0);
	myInnerHtml("EN61_2",'<select name="murderBonus" style="width:50px;" onchange="ToggleOtherBuffs()"></select>',0);
	murderBonus.options[0] = new Option(SubName[10][Language],0); // "off"
	murderBonus.options[1] = new Option("ALL+3",1);
	murderBonus.options[2] = new Option("ALL+5",2);

	// IC (Anolian)
	myInnerHtml("EN62_1",SKILL_NAME[42][Language] +" Lv",0);
	myInnerHtml("EN62_2",'<select name="improveCon" style="width:50px;" onchange="ToggleOtherBuffs()"></select>',0);
	for(i=0;i<=2;i++)
		improveCon.options[i] = new Option(i,i);

	// self MindBreaker
	myInnerHtml("EN63_1",GetWord(123) + SKILL_NAME[298][Language],0);
	myInnerHtml("EN63_2",'<select name="mindBreaker" style="width:50px;" onchange="ToggleOtherBuffs()"></select>',0);

	// self Provoke
	myInnerHtml("EN64_1",GetWord(123) + SKILL_NAME[8][Language],0);
	myInnerHtml("EN64_2",'<select name="provoke" style="width:50px;" onchange="ToggleOtherBuffs()"></select>',0);

	// BSS
	myInnerHtml("EN65_1",SKILL_NAME[93][Language],0);
	myInnerHtml("EN65_2",'<input type="checkbox" name="bss" onclick="ToggleOtherBuffs()">',0);
	
	// AR
	myInnerHtml( "EN66_1", SKILL_NAME[152][Language], 0 );
	myInnerHtml( "EN66_2", '<select name="adrenalineRush" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	adrenalineRush.options[0] = new Option( "Off", 0 );
	adrenalineRush.options[1] = new Option( "Regular AR", 1 );
	adrenalineRush.options[2] = new Option( "Full AR", 2 );
	adrenalineRush.options[3] = new Option( "AR Scroll", 3 );
	
	// Weapon Perfection
	myInnerHtml( "EN67_1", SKILL_NAME[153][Language], 0 );
	myInnerHtml( "EN67_2", '<input type="checkbox" name="weaponPerfection" onclick="ToggleOtherBuffs()">', 0 );
	
	// Power Thrust
	myInnerHtml( "EN68_1", SKILL_NAME[154][Language], 0 );
	myInnerHtml( "EN68_2", '<select name="powerThrust" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	// Wind Walker
	myInnerHtml( "EN69_1", SKILL_NAME[273][Language], 0 );
	myInnerHtml( "EN69_2", '<select name="windWalker" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	// Magnum Break Bonus
	myInnerHtml( "EN610_1", GetWord(103), 0 );
	myInnerHtml( "EN610_2", '<input type="checkbox" name="magnumBreak" onclick="ToggleOtherBuffs()">', 0 );
	
	// Aloe
	myInnerHtml( "EN611_1", GetWord(104), 0 );
	myInnerHtml( "EN611_2", '<input type="checkbox" name="aloe" onclick="ToggleOtherBuffs()">', 0 );
	
	// Resistant Souls
	myInnerHtml( "EN612_1", SKILL_NAME[164][Language], 0 );
	myInnerHtml( "EN612_2", '<select name="resistantSouls" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	// Striking
	myInnerHtml( "EN613_1", SKILL_NAME[681][Language], 0 );
	myInnerHtml( "EN613_2", '<select name="striking" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	// Striking Endow Bonus
	
	myInnerHtml( "EN614_1", "Striking Endow Bonus", 0 );
	myInnerHtml( "EN614_2", '<select name="strikingEndow" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	for (var i = 0; i < 21; i++) {
	    strikingEndow.options[i] = new Option (i.toString(), i);
	}
	
	// Odin's Power
	myInnerHtml( "EN615_1", SKILL_NAME[721][Language], 0 );
	myInnerHtml( "EN615_2", '<select name="odinsPower" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	// Frigg's Song
	//myInnerHtml( "EN615_1", SKILL_NAME[741][Language], 0 );
	//myInnerHtml( "EN615_2", '<select name="friggsSong" style="width:50px;" onchange="ToggleOtherBuffs()"></select>', 0 );
	
	for ( var i = 0; i <= 10; i++ )
	{ // lvl10 skills
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		provoke.options[i] = new Option(i,i); // Self Provoke
		windWalker.options[i] = new Option( off, i ); // WindWalk
	}
	
	for ( var i = 0; i <= 5; i++ )
	{ // lvl5 skills
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		eleFieldLvl.options[i] = new Option(i,i); // Volcano, Deluge, Whirlwind
		mindBreaker.options[i] = new Option(i,i); // Self Mindbreaker
		powerThrust.options[i] = new Option( off, i ); // PowThrust
		resistantSouls.options[i] = new Option( off, i ); // ResiSouls
		striking.options[i] = new Option( off, i ); // Striking
		//friggsSong.options[i] = new Option( i , i ); //Frigg's Song
	}
	for ( var i = 0; i <= 2; i++ )
	{ // lvl2 skills
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		odinsPower.options[i] = new Option( off, i ); // Odin's Power
	}
}
}

function BuildMiscEffectsTable()
{
with(document.calcForm)
{
	var str;
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="petsLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="petsInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="numEnemiesLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="numEnemiesInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="tempOneLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="tempOneInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="quagLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="quagInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="tempTwoLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="tempTwoInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="agiDownLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="agiDownInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="tempThreeLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="tempThreeInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="noCritLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="noCritInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="tempFourLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="tempFourInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="poisonedLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="poisonedInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="firstSpiritLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="firstSpiritInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="cursedLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="cursedInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="noviceLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="noviceInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td class="bgLtRow3 padded optCaption"></td>';
	str += '<td class="bgLtRow1 padded optArea"></td>';
	str += '</tr></table>';
	myInnerHtml( "miscBuffs", str, 0 );
	
	// Pets
	myInnerHtml( "petsLabel", "Pet Bonuses", 0 );
	myInnerHtml( "petsInput", '<select id="petBonus" onchange="ToggleMiscEffects()"></select>', 0 );
	for ( var i = 0; i <= PET_OBJ.length - 1; i++ )
	{
		petBonus.options[i] = new Option(PET_OBJ[i][1],PET_OBJ[i][0]);
	}

	// Super Novice Marriage Bonus
	myInnerHtml( "noviceLabel", "SN Marriage Bonus", 0 );
	myInnerHtml( "noviceInput", '<input type="checkbox" id="noviceMarried" onclick="ToggleMiscEffects()">&nbsp(Stats + 1)', 0 );

	// Temp Effects
	myInnerHtml( "tempOneLabel", "Temp Effect", 0 );
	myInnerHtml( "tempTwoLabel", "Temp Effect", 0 );
	myInnerHtml( "tempThreeLabel", "Temp Effect", 0 );
	myInnerHtml( "tempFourLabel", "Temp Effect", 0 );
	myInnerHtml( "tempOneInput", '<select id="tempOne" onchange="ToggleMiscEffects()"></select>', 0 );
	myInnerHtml( "tempTwoInput", '<select id="tempTwo" onchange="ToggleMiscEffects()"></select>', 0 );
	myInnerHtml( "tempThreeInput", '<select id="tempThree" onchange="ToggleMiscEffects()"></select>', 0 );
	myInnerHtml( "tempFourInput", '<select id="tempFour" onchange="ToggleMiscEffects()"></select>', 0 );
	for ( var i = 0; i < ITEM_SP_TIME_OBJ_SORT.length; i++ )
	{
		var n = ITEM_SP_TIME_OBJ_SORT[i];
		tempOne.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] +" ["+ ITEM_SP_TIME_OBJ[n][2] +"]",n);
		tempTwo.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] +" ["+ ITEM_SP_TIME_OBJ[n][2] +"]",n);
		tempThree.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] +" ["+ ITEM_SP_TIME_OBJ[n][2] +"]",n);
		tempFour.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] +" ["+ ITEM_SP_TIME_OBJ[n][2] +"]",n);
	}
	
	// Number of enemies hitting you
	myInnerHtml( "numEnemiesLabel", "# of Enemies", 0 );
	myInnerHtml( "numEnemiesInput", '<select id="numEnemies" style="width:50px;" onchange="ToggleMiscEffects()"></select>', 0 );
	for ( var i = 0; i <= 22; i++ )
	{
		numEnemies.options[i] = new Option(i + "",i);
	}
	
	// Advanced First Class Spirit
	myInnerHtml( "firstSpiritLabel", "Advance 1st Spirit", 0 );
	myInnerHtml( "firstSpiritInput", '<input type="checkbox" id="firstSpirit" onclick="ToggleMiscEffects()">&nbsp(Max Stats)', 0 );

	// No Crit
	myInnerHtml( "noCritLabel", "Set CRIT% to 0", 0 );
	myInnerHtml( "noCritInput", '<input type="checkbox" id="playerNoCrit" onclick="ToggleMiscEffects()">', 0 );
	
	// Quagmire
	myInnerHtml( "quagLabel", "Quagmire", 0 );
	myInnerHtml( "quagInput", '<select id="playerQuaged" style="width:50px;" onchange="ToggleMiscEffects()"></select>', 0 );
	playerQuaged.options[0] = new Option("Off",0);
	for ( var i = 1; i <= 5; i++ )
	{
		playerQuaged.options[i] = new Option("Lv"+i,i);
	}
	
	// AgiDown
	myInnerHtml( "agiDownLabel", "AGI Down", 0 );
	myInnerHtml( "agiDownInput", '<select id="playerAgiDowned" style="width:50px;" onchange="ToggleMiscEffects()"></select>', 0 );
	playerAgiDowned.options[0] = new Option( "Off", 0 );
	for ( var i = 1; i <= 10; i++ )
	{
		playerAgiDowned.options[i] = new Option( "Lv" + i, i );
	}
	playerAgiDowned.options[11] = new Option( "Lv46", 46 );

	// Poisoned
	myInnerHtml( "poisonedLabel", "Poisoned", 0 );
	myInnerHtml( "poisonedInput", '<input type="checkbox" id="playerPoisoned" onclick="ToggleMiscEffects()">', 0 );
	
	// Cursed
	myInnerHtml( "cursedLabel", "Cursed", 0 );
	myInnerHtml( "cursedInput", '<input type="checkbox" id="playerCursed" onclick="ToggleMiscEffects()">', 0 );
}
}

function BuildBattleEffectsTable()
{
with(document.calcForm)
{
	var str;
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="vipLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="vipInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="jobManualLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="jobManualInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="baseManualLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="baseManualInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="serverExpLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="serverExpInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="partyCountLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="partyCountInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="tapBonusLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="tapBonusInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="specEnvLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="specEnvInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="investmentLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="investmentInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="minDelayLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="minDelayInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td class="bgLtRow3 padded optCaption"></td>';
	str += '<td class="bgLtRow1 padded optArea"></td>';
	str += '</tr></table>';
	myInnerHtml( "battleEffects", str, 0 );
	
	// Battle Manual
	myInnerHtml( "baseManualLabel", "Battle Manual", 0 );
	myInnerHtml( "baseManualInput", '<select id="baseManual" style="width:100px;" onchange="ToggleBattleEffects()"></select>', 0 );
	baseManual.options[0] = new Option("None",0);
	baseManual.options[1] = new Option("150%",50);
	baseManual.options[2] = new Option("200%",100);
	baseManual.options[3] = new Option("300%",200);
	
	// Job Manual
	myInnerHtml( "jobManualLabel", "Job Manual 50", 0 );
	myInnerHtml( "jobManualInput", '<input type="checkbox" id="jobManual" onclick="ToggleBattleEffects()">', 0 );
	
	// VIP
	myInnerHtml( "vipLabel", "VIP +50% Exp", 0 );
	myInnerHtml( "vipInput", '<input type="checkbox" id="vipStatus" onclick="ToggleBattleEffects()">', 0 );
	
	// Server EXP
	myInnerHtml( "serverExpLabel", "Server Experience Rate", 0 );
	myInnerHtml( "serverExpInput", '<select id="serverExp" style="width:80px;" onchange="ToggleBattleEffects()"></select>', 0 );
	serverExp.options[0] = new Option("Normal",0);
	for ( var i = 1; i <= 8; i++ )
	{
		serverExp.options[i] = new Option("+"+(25*i)+"%",i);
	}

	// Party size
	myInnerHtml( "partyCountLabel", "Partymember Count", 0 );
	myInnerHtml( "partyCountInput", '<select id="partySize" style="width:100px;" onchange="ToggleBattleEffects()"></select>', 0 );
	partySize.options[0] = new Option("Solo",0);
	for ( var i = 1; i <= 11; i++ )
	{
		partySize.options[i] = new Option((i+1)+"",i);
	}
	
	// tap bonus
	myInnerHtml( "tapBonusLabel", "Exp Tap Bonus", 0 );
	myInnerHtml( "tapBonusInput", '<select id="expTap" style="width:80px;" onchange="ToggleBattleEffects()"></select>', 0 );
	expTap.options[0] = new Option("None",0);
	for ( var i = 1; i <= 20; i++ )
	{
		expTap.options[i] = new Option("+"+ (i*5) +"%",i);
	}
	
	// special environment
	myInnerHtml( "specEnvLabel", "Special Environment", 0 );
	myInnerHtml( "specEnvInput", '<select id="specialEnv" style="width:100px;" onchange="ToggleBattleEffects()"></select>', 0 );
	specialEnv.options[0] = new Option("PvM",0);
	specialEnv.options[1] = new Option("WoE Zone",1);
	specialEnv.options[2] = new Option("Guild Dungeon",2);
	specialEnv.options[3] = new Option("URDR Server",3);
	
	// castle investment
	myInnerHtml( "investmentLabel", "Defense Investment", 0 );
	myInnerHtml( "investmentInput", '<select id="castleDefense" style="width:80px;" onchange="ToggleBattleEffects()"></select>', 0 );
	castleDefense.options[0] = new Option("None",0);
	for ( var i = 1; i <= 20; i++ )
	{
		castleDefense.options[i] = new Option(i * 5,i);
	}
	
	// min delay between skills
	myInnerHtml( "minDelayLabel", "Minimum Skill Delay", 0 );
	myInnerHtml( "minDelayInput", '<select id="Conf01" style="width:100px;" onchange="ToggleBattleEffects()"></select>', 0 );
	for ( var i = 0; i < ATKTIME_NAME.length; i++ )
	{ // MinDelayList (Bottom)
		Conf01.options[i] = new Option(ATKTIME_NAME[i][1 + Language],ATKTIME_NAME[i][0]);
	}
}
}

function BuildMonsterDebuffTable()
{
with(document.calcForm)
{
	// Build the table
	var str;
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="BI0_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI0_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI1_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI1_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI2_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI2_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI3_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI3_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI4_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI4_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI5_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI5_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI6_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI6_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI7_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI7_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI8_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI8_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI9_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI9_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI10_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI10_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI11_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI11_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI12_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI12_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI13_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI13_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI14_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI14_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI15_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI15_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI16_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI16_2" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="BI17_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI17_2" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI18_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI18_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI19_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI19_2" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="BI20_1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="BI20_2" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="BI24_1" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="BI24_2" class="bgLtRow2 padded optArea"></td>';
	if ( PlayerVersusPlayer === 0 )
	{
		str += '<td id="BI21_1" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="BI21_2" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="BI22_1" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="BI22_2" class="bgLtRow2 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="BI23_1" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="BI23_2" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="deepSleepLabel" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="deepSleepInput" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="venomImpressLabel" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="venomImpressInput" class="bgLtRow1 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="MarshOfAbyssLabel" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="MarshOfAbyssInput" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="GloomyDayLabel" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="GloomyDayInput" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="DarkClawLabel"class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="DarkClawInput" class="bgLtRow2 padded optArea"></td>';
	}
	else
	{
		str += '<td class="bgLtRow4 padded optCaption"></td>';
		str += '<td class="bgLtRow2 padded optArea"></td>';
		str += '<td class="bgLtRow4 padded optCaption"></td>';
		str += '<td class="bgLtRow2 padded optArea"></td>';
	}
	str += '</tr></table>';
	myInnerHtml( "MonsterDebuffs", str, 0 );
	
	// Build the labels
	var w_name = new Array();
	w_name[0] = SKILL_NAME[8][Language] + GetWord(136); // Provoke
	w_name[1] = SKILL_NAME[134][Language]; // Quag
	w_name[2] = AilmentsOBJ[0][Language]; // Poison
	w_name[3] = AilmentsOBJ[4][Language]; // Blind
	w_name[4] = AilmentsOBJ[2][Language] + GetWord(136) // Freeze
	w_name[5] = SKILL_NAME[31][Language] + GetWord(137) // Blessing
	w_name[6] = SKILL_NAME[103][Language]; // LexAtern
	w_name[7] = AilmentsOBJ[1][Language]; // Stun
	w_name[8] = AilmentsOBJ[5][Language]; // Sleep
	w_name[9] = AilmentsOBJ[9][Language]; // Stone
	w_name[10] = AilmentsOBJ[3][Language]; // Curse
	w_name[11] = SKILL_NAME[28][Language]; // DecAgi
	w_name[12] = SKILL_NAME[29][Language]; // SignumCrusis
	w_name[13] = SKILL_NAME[172][Language]; // StripWeap
	w_name[14] = SKILL_NAME[173][Language]; // StripShie
	w_name[15] = SKILL_NAME[174][Language]; // StripArmor
	w_name[16] = SKILL_NAME[175][Language]; // StripHelm
	w_name[17] = SKILL_NAME[313][Language]; // FiberLock
	w_name[18] = SKILL_NAME[298][Language]; // MindBReaker
	w_name[19] = SKILL_NAME[211][Language] + GetWord(138); // Slow Grace
	w_name[20] = SKILL_NAME[218][Language]; // DownTempo
	w_name[21] = SKILL_NAME[377][Language]; // Eska
	w_name[22] = SKILL_NAME[378][Language]; // Eske
	w_name[23] = SKILL_NAME[314][Language]; // EleChange(Sage)
	w_name[24] = SKILL_NAME[417][Language]; // Coin Fling
	w_name[25] = AilmentsOBJ[12][Language]; // Deep Sleep Lullaby
	w_name[26] = AilmentsOBJ[13][Language]; // Venom Impress
	w_name[27] = AilmentsOBJ[14][Language]; // Marsh of Abyss
	w_name[28] = AilmentsOBJ[17][Language]; // Gloomy Day
	w_name[29] = AilmentsOBJ[18][Language]; // Dark Claw
	for ( var i = 0; i <= 20; i++ )
	{
		myInnerHtml( "BI" + i + "_1", w_name[i], 0 );
	}
	if ( PlayerVersusPlayer === 0 )
	{ // not human
		for ( var i = 21; i <= 23; i++ )
		{
			myInnerHtml( "BI" + i + "_1", w_name[i], 0 );
		}
	}
	myInnerHtml( "BI24_1", w_name[24], 0 );
	myInnerHtml( "deepSleepLabel", w_name[25], 0 );
	myInnerHtml( "venomImpressLabel", w_name[26], 0 );
	myInnerHtml( "MarshOfAbyssLabel", w_name[27], 0 );
	myInnerHtml( "GloomyDayLabel", w_name[28], 0 );
	myInnerHtml( "DarkClawLabel", w_name[29], 0 );
	
	// Build the selection options
	var html_SKILL = new Array();
	html_SKILL[0] = '<select name="B_IJYOU0" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[1] = '<select name="B_IJYOU1" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[2] = '<input type="checkbox" name="B_IJYOU2"onclick="ToggleMonsterDebuff()">';
	html_SKILL[3] = '<input type="checkbox" name="B_IJYOU3"onclick="ToggleMonsterDebuff()">';
	html_SKILL[4] = '<input type="checkbox" name="B_IJYOU4"onclick="ToggleMonsterDebuff()">';
	html_SKILL[5] = '<input type="checkbox" name="B_IJYOU5"onclick="ToggleMonsterDebuff()">';
	html_SKILL[6] = '<input type="checkbox" name="B_IJYOU6"onclick="ToggleMonsterDebuff()">';
	html_SKILL[7] = '<input type="checkbox" name="B_IJYOU7"onclick=ToggleMonsterDebuff()">';
	html_SKILL[8] = '<input type="checkbox" name="B_IJYOU8"onclick="ToggleMonsterDebuff()">';
	html_SKILL[9] = '<input type="checkbox" name="B_IJYOU9"onclick="ToggleMonsterDebuff()">';
	html_SKILL[10] = '<input type="checkbox" name="B_IJYOU10"onclick="ToggleMonsterDebuff()">';
	html_SKILL[11] = '<select name="B_IJYOU11" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[12] = '<select name="B_IJYOU12" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[13] = '<input type="checkbox" name="B_IJYOU13"onclick="ToggleMonsterDebuff()">';
	html_SKILL[14] = '<input type="checkbox" name="B_IJYOU14"onclick="ToggleMonsterDebuff()">';
	html_SKILL[15] = '<input type="checkbox" name="B_IJYOU15"onclick="ToggleMonsterDebuff()">';
	html_SKILL[16] = '<input type="checkbox" name="B_IJYOU16"onclick="ToggleMonsterDebuff()">';
	html_SKILL[17] = '<input type="checkbox" name="B_IJYOU17"onclick="ToggleMonsterDebuff()">';
	html_SKILL[18] = '<select name="B_IJYOU18" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[19] = '<input type="checkbox" name="B_IJYOU19"onclick="ToggleMonsterDebuff()">';
	html_SKILL[20] = '<input type="checkbox" name="B_IJYOU20"onclick="ToggleMonsterDebuff()">';
	html_SKILL[24] = '<select name="B_IJYOU24" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[25] = '<input type="checkbox" id="DeepSleep" onclick="ToggleMonsterDebuff()">';
	html_SKILL[26] = '<select name="VenomImpress" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[27] = '<select name="MarshOfAbyss" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[28] = '<select name="GloomyDay" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>';
	html_SKILL[29] = '<select name="DarkClaw" style="width:50px;" onchange="ToggleMonsterDebuff()"></select>'
	for ( var i = 0; i <= 20; i++ )
	{
		myInnerHtml("BI"+i+"_2",html_SKILL[i],0);
	}
	myInnerHtml("BI24_2",html_SKILL[24],0);
	myInnerHtml("deepSleepInput",html_SKILL[25],0);
	myInnerHtml("venomImpressInput",html_SKILL[26],0);
	myInnerHtml("MarshOfAbyssInput",html_SKILL[27],0);
	myInnerHtml("GloomyDayInput",html_SKILL[28],0);
	myInnerHtml("DarkClawInput", html_SKILL[29],0);

	// Build options for 10 level skills
	for ( var i = 0; i <= 10; i++ )
	{
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		B_IJYOU0.options[i] = new Option( off, i );
		B_IJYOU11.options[i] = new Option( off, i );
		B_IJYOU12.options[i] = new Option( off, i );
	}
	
	// Build options for 5 level skills
	for ( var i = 0; i <= 5; i++ )
	{
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		B_IJYOU1.options[i] = new Option( off, i );
		B_IJYOU18.options[i] = new Option( off, i );
		B_IJYOU24.options[i] = new Option( off, i );
		VenomImpress.options[i] = new Option( off, i );
		MarshOfAbyss.options[i] = new Option( off, i );
		GloomyDay.options[i] = new Option( off, i );
		DarkClaw.options[i] = new Option( off, i );
	}
	
	// PvM Debuffs
	if ( PlayerVersusPlayer === 0 )
	{
		myInnerHtml("BI21_2",'<input type="checkbox" name="B_IJYOU21"onclick="ToggleMonsterDebuff()">',0);
		myInnerHtml("BI22_2",'<input type="checkbox" name="B_IJYOU22"onclick="ToggleMonsterDebuff()">',0);
		myInnerHtml("BI23_2",'<select name="B_IJYOU23" style="width:80px;"onchange="ToggleMonsterDebuff()"></select>',0);

		// InsertElements
		B_IJYOU23.options[0] = new Option(SubName[10][Language],0);
		for ( var i = 1; i <= 4; i++ )
		{
			B_IJYOU23.options[i] = new Option(ZokuseiOBJ[i][Language],i);
		}
	}
}
}

function BuildMonsterBuffTable()
{
with( document.calcForm )
{
	// Build Table
	var str;
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="ID_K0" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="ID_Kb0" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="ID_K1" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="ID_Kb1" class="bgLtRow1 padded optArea"></td>';
	if ( PlayerVersusPlayer === 0 )
	{ // not human
		str += '<td id="ID_K2" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb2" class="bgLtRow1 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="ID_K3" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb3" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="ID_K4" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb4" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="ID_K5" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb5" class="bgLtRow2 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="ID_K6" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb6" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K7" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb7" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K8" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb8" class="bgLtRow1 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="ID_K9" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb9" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="ID_K10" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb10" class="bgLtRow2 padded optArea"></td>';
		str += '<td id="ID_K11" class="bgLtRow4 padded optCaption"></td>';
		str += '<td id="ID_Kb11" class="bgLtRow2 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="ID_K12" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb12" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K13" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb13" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K14" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb14" class="bgLtRow1 padded optArea"></td>';
		str += '</tr><tr>';
		str += '<td id="ID_K15" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb15" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K16" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb16" class="bgLtRow1 padded optArea"></td>';
		str += '<td id="ID_K17" class="bgLtRow3 padded optCaption"></td>';
		str += '<td id="ID_Kb17" class="bgLtRow1 padded optArea"></td>';
	}
	else
	{
		str += '<td class="bgLtRow3 padded optCaption"></td>';
		str += '<td class="bgLtRow1 padded optArea"></td>';
	}
	str += '</tr></table>';
	myInnerHtml( "MonsterBuffs", str, 0 );

	// Build Labels
	var w_name = new Array();
	w_name[0] = SKILL_NAME[27][Language]; // IncAgi
	w_name[1] = SKILL_NAME[267][Language]; // Assu
	w_name[2] = SKILL_NAME[152][Language]; // ARush
	w_name[3] = SKILL_NAME[155][Language]; // MaxPower
	w_name[4] = SKILL_NAME[439][Language] + GetWord(139); // PowerUp
	w_name[5] = SKILL_NAME[440][Language] + GetWord(140); // Flee up
	w_name[6] = SKILL_NAME[441][Language]; // EleChange
	w_name[7] = SKILL_NAME[442][Language]; // StoneSkin
	w_name[8] = SKILL_NAME[443][Language]; // MMirror
	w_name[9] = SKILL_NAME[444][Language]; // Keeping
	w_name[10] = "Race Reductions"; // Race Reductions
	w_name[11] = "Elemental Reductions"; // Elemental Reductions
	w_name[12] = "Ranged Reductions"; // Ranged Reductions
	w_name[13] = "Size Reductions"; // Size Reductions
	w_name[14] = "Normal Monster Reductions"; // Normal Monsters Reductions
	w_name[15] = "Other Reductions"; // Other Reductions
	for ( var i = 0; i <= 1; i++ )
	{
		myInnerHtml( "ID_K" + i, w_name[i], 0 );
	}

	// Build Input
	var html_SKILL = new Array();
	html_SKILL[0] = '<select name="B_KYOUKA0" style="width:50px;" onchange="ToggleMonsterBuff()"></select>';
	html_SKILL[1] = '<input type="checkbox" name="B_KYOUKA1" onclick="ToggleMonsterBuff()">';
	html_SKILL[2] = '<input type="checkbox" name="B_KYOUKA2" onclick="ToggleMonsterBuff()">';
	html_SKILL[3] = '<input type="checkbox" name="B_KYOUKA3" onclick="ToggleMonsterBuff()">';
	html_SKILL[4] = '<input type="checkbox" name="B_KYOUKA4" onclick="ToggleMonsterBuff()">';
	html_SKILL[5] = '<input type="checkbox" name="B_KYOUKA5" onclick="ToggleMonsterBuff()">';
	html_SKILL[6] = '<select name="B_KYOUKA6" style="width:100px;" onchange="ToggleMonsterBuff()"></select>';
	html_SKILL[7] = '<select name="B_KYOUKA7" style="width:50px;" onchange="ToggleMonsterBuff()"></select>';
	html_SKILL[8] = '<select name="B_KYOUKA8" style="width:50px;" onchange="ToggleMonsterBuff()"></select>';
	html_SKILL[9] = '<input type="checkbox" name="B_KYOUKA9" onclick="ToggleMonsterBuff()">';
	html_SKILL[10] = '<input name="B_KYOUKA10" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	html_SKILL[11] = '<input name="B_KYOUKA11" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	html_SKILL[12] = '<input name="B_KYOUKA12" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	html_SKILL[13] = '<input name="B_KYOUKA13" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	html_SKILL[14] = '<input name="B_KYOUKA14" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	html_SKILL[15] = '<input name="B_KYOUKA15" value="0" type="number" max="100" min="-100" style="width:50px;" onchange="ToggleMonsterBuff()" />';
	for ( var i = 0; i <= 1; i++ )
	{
		myInnerHtml( "ID_Kb" + i, html_SKILL[i], 0 );
	}

	// Build Options
	for ( var i = 0; i <= 10; i++ )
	{ // 10 lvl agi up
		var off = '0';
		if ( i === 0 )
		{
			off = 'Off';
		}
		else
		{
			off = i;
		}
		
		B_KYOUKA0.options[i] = new Option( off, i );
	}

	if ( PlayerVersusPlayer === 0 )
	{ // not human
		for ( var i = 2; i <= 9; i++ )
		{
			myInnerHtml( "ID_K" + i, w_name[i], 0 );
		}

		for ( var i = 2; i <= 9; i++ )
		{
			myInnerHtml( "ID_Kb" + i, html_SKILL[i], 0 );
		}
		
		for ( var i = 0; i <= 5; i++ )
		{
			var off = '0';
			if ( i === 0 )
			{
				off = 'Off';
			}
			else
			{
				off = i;
			}
		
			B_KYOUKA7.options[i] = new Option( off, i );
			B_KYOUKA8.options[i] = new Option( off, i );
		}

		// EleChange list
		var ZoHe = new Array();
		ZoHe[0] = "Element";
		for ( var i = 1; i < 41; i++ )
		{
			ZoHe[i] = ZokuseiOBJ[Math.floor((i-1) / 4)][Language] + ((i-1) % 4 +1);
		}
		
		var ZoHe2 =	[0,1,2,3,4,11,12,13,14,21,22,23,24,31,32,33,34,41,42,43,44,51,52,53,54,61,62,63,64,71,72,73,74,81,82,83,84,91,92,93,94];
		for ( var i = 0; i <= 40; i++ )
		{
			B_KYOUKA6.options[i] = new Option(ZoHe[i],ZoHe2[i]);
		}
		for ( var i = 10; i <= 15; i++ )
		{
			myInnerHtml( "ID_K" + i, w_name[i], 0 );
		}

		for ( var i = 10; i <= 15; i++ )
		{
			myInnerHtml( "ID_Kb" + i, html_SKILL[i], 0 );
		}
	}
}
}

function BuildItemsTable()
{
	var formElements = document.forms["calcForm"].elements;
	
	// Build Table
	var str;
	str = '<table class="bgLtTable"><tr>';
	str += '<td id="aspdLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="aspdInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="sesameLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="sesameInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="fireLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="fireInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="runeLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="runeInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="strFoodLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="strFoodInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="honeyLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="honeyInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="coldLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="coldInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="schwartzLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="schwartzInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="agiFoodLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="agiFoodInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="rainbowLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="rainbowInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="thunderLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="thunderInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="arunafeltzLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="arunafeltzInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="vitFoodLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="vitFoodInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="militarybLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="militarybInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="earthLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="earthInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="boucheLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="boucheInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="intFoodLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="intFoodInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="militarycLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="militarycInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="celermineLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="celermineInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="distilledLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="distilledInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="dexFoodLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="dexFoodInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="pinkLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="pinkInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="vitataLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="vitataInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="durianLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="durianInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="lukFoodLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="lukFoodInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="whiteLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="whiteInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="increasehpLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="increasehpInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="guaranaLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="guaranaInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="vipBuffLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="vipBuffInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="resentmentLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="resentmentInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="increasespLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="increasespInput" class="bgLtRow2 padded optArea"></td>';
	str += '<td id="scrollLabel" class="bgLtRow4 padded optCaption"></td>';
	str += '<td id="scrollInput" class="bgLtRow2 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="luckyLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="luckyInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="drowsinessLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="drowsinessInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="abrasiveLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="abrasiveInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td id="holyElLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="holyElInput" class="bgLtRow1 padded optArea"></td>';
	str += '</tr><tr>';
	str += '<td id="undeadElLabel" class="bgLtRow3 padded optCaption"></td>';
	str += '<td id="undeadElInput" class="bgLtRow1 padded optArea"></td>';
	str += '<td class="bgLtRow3 padded optCaption"></td>';
	str += '<td class="bgLtRow1 padded optArea"></td>';
	str += '<td class="bgLtRow3 padded optCaption"></td>';
	str += '<td class="bgLtRow1 padded optArea"></td>';
	str += '<td class="bgLtRow3 padded optCaption"></td>';
	str += '<td class="bgLtRow1 padded optArea"></td>';
	str += '</tr></table>';
	myInnerHtml( "items", str, 0 );

	// Build Input Fields
	// Add Castscrolls etc. to Skill List
	myInnerHtml( "scrollLabel", GetWord(134), 0 );
	myInnerHtml( "scrollInput", '<input type="checkbox" id="castScrolls" onclick="ToggleItems(0)|ActiveSkillSetPlus()">', 0 );
	// Sesame Pastery
	myInnerHtml( "sesameLabel", GetWord(124) + " (HIT +30)", 0 );
	myInnerHtml( "sesameInput", '<input type="checkbox" id="sesamePastry" onclick="ToggleItems(0)">', 0 );
	// Honey Pastry
	myInnerHtml( "honeyLabel", GetWord(125) + " (FLEE +30)", 0 );
	myInnerHtml( "honeyInput", '<input type="checkbox" id="honeyPastry" onclick="ToggleItems(0)">', 0 );
	// Rainbow Cake
	myInnerHtml( "rainbowLabel", GetWord(126) + " (ATK/MATK +10)", 0 );
	myInnerHtml( "rainbowInput", '<input type="checkbox" id="rainbowCake" onclick="ToggleItems(0)">', 0 );
	// Box of Resentment
	myInnerHtml( "resentmentLabel", GetWord(127) + " (ATK +20)", 0 );
	myInnerHtml( "resentmentInput", '<input type="checkbox" id="resentment" onclick="ToggleItems(0)">', 0 );
	// Box of Drowsiness
	myInnerHtml( "drowsinessLabel", GetWord(128) + " (MATK +20)", 0 );
	myInnerHtml( "drowsinessInput", '<input type="checkbox" id="drowsiness" onclick="ToggleItems(0)">', 0 );
	// Abrasive
	myInnerHtml( "abrasiveLabel", GetWord(259) + " (Crit +30)", 0 );
	myInnerHtml( "abrasiveInput", '<input type="checkbox" id="abrasive" onclick="ToggleItems(0)">', 0 );
	// Holy Elemental Scroll
	myInnerHtml( "holyElLabel", GetWord(260), 0 );
	myInnerHtml( "holyElInput", '<input type="checkbox" id="holyEl" onclick="ToggleItems(0)">', 0 );
	// Undead Elemental Scroll
	myInnerHtml( "undeadElLabel", GetWord(261), 0 );
	myInnerHtml( "undeadElInput", '<input type="checkbox" id="undeadEl" onclick="ToggleItems(0)">', 0 );
	// ColdProof
	myInnerHtml( "coldLabel", GetWord(129), 0 );
	myInnerHtml( "coldInput", '<input type="checkbox" id="coldproof" onclick="ToggleItems(0)">', 0 );
	// EarthProof
	myInnerHtml( "earthLabel", GetWord(130), 0 );
	myInnerHtml( "earthInput", '<input type="checkbox" id="earthproof" onclick="ToggleItems(0)">', 0 );
	// FireProof
	myInnerHtml( "fireLabel", GetWord(131), 0 );
	myInnerHtml( "fireInput", '<input type="checkbox" id="fireproof" onclick="ToggleItems(0)">', 0 );
	//ThunderProof
	myInnerHtml( "thunderLabel", GetWord(132), 0 );
	myInnerHtml( "thunderInput", '<input type="checkbox" id="thunderproof" onclick="ToggleItems(0)">', 0 );
	// Bouche de Noel
	myInnerHtml( "boucheLabel", GetWord(234) + " (HIT +3/CRIT +7)", 0 );
	myInnerHtml( "boucheInput", '<input id="boucheDeNoel" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Rune Strawberry Cake
	myInnerHtml( "runeLabel", GetWord(235) + " (ATK/MATK +5)", 0 );
	myInnerHtml( "runeInput", '<input id="runeStrawberry" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Schwartzwald Pine Jubilee
	myInnerHtml( "schwartzLabel", GetWord(236) + " (HIT +10/FLEE +20)", 0 );
	myInnerHtml( "schwartzInput", '<input id="pineJubilee" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Arunafeltz Desert Sandwich
	myInnerHtml( "arunafeltzLabel", GetWord(237) + " (CRIT +7)", 0 );
	myInnerHtml( "arunafeltzInput", '<input id="desertSandwich" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Distilled Fighting Spirit
	myInnerHtml( "distilledLabel", GetWord( 241 ) + " (ATK +30)", 0 );
	myInnerHtml( "distilledInput", '<input id="fightingSpirit" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// VIP Buffs
	myInnerHtml( "vipBuffLabel", GetWord( 253 ) + " (STATS +7)", 0 );
	myInnerHtml( "vipBuffInput", '<input id="vipBuff" type="checkbox" onclick="ToggleItems(1)">', 0 );
	// Durian
	myInnerHtml( "durianLabel", GetWord( 242 ) + " (ATK/MATK +10)", 0 );
	myInnerHtml( "durianInput", '<input id="durian" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Enriched Celermine Juice
	myInnerHtml( "celermineLabel", GetWord( 243 ) + " (ASPD +10%)", 0 );
	myInnerHtml( "celermineInput", '<input id="celermineJuice" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Guarana Candy
	myInnerHtml( "guaranaLabel", GetWord( 244 ) + " (AGI LVL 5/ASPD +10%)", 0 );
	myInnerHtml( "guaranaInput", '<input id="guaranaCandy" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Lucky Rice Cake
	myInnerHtml( "luckyLabel", GetWord( 245 ) + " (LUK +21)", 0 );
	myInnerHtml( "luckyInput", '<input id="luckyRiceCake" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Military Ration B
	myInnerHtml( "militarybLabel", GetWord( 246 ) + " (HIT +33)", 0 );
	myInnerHtml( "militarybInput", '<input id="militaryRationB" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Military Ration C
	myInnerHtml( "militarycLabel", GetWord( 247 ) + " (FLEE +33)", 0 );
	myInnerHtml( "militarycInput", '<input id="militaryRationC" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Tasty Pink Ration
	myInnerHtml( "pinkLabel", GetWord( 248 ) + " (ATK +15)", 0 );
	myInnerHtml( "pinkInput", '<input id="pinkRation" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Tasty White Ration
	myInnerHtml( "whiteLabel", GetWord( 249 ) + " (MATK +15)", 0 );
	myInnerHtml( "whiteInput", '<input id="whiteRation" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// Vitata500
	myInnerHtml( "vitataLabel", GetWord( 250 ) + " (SP +5%)", 0 );
	myInnerHtml( "vitataInput", '<input id="vitataFiveHundred" type="checkbox" onclick="ToggleItems(0)">', 0 );
	// ASPD
	myInnerHtml( "aspdLabel", GetWord( 238 ), 0 );
	myInnerHtml( "aspdInput", '<select id="speedPot" style="width:100px;" onchange="ToggleItems(0)"></select>', 0 );
	// Increase HP
	myInnerHtml( "increasehpLabel", GetWord( 239 ), 0 );
	myInnerHtml( "increasehpInput", '<select id="increaseHpPotion" style="width:80px;" onchange="ToggleItems(0)"></select>', 0 );
	// Increase SP
	myInnerHtml( "increasespLabel", GetWord( 240 ), 0 );
	myInnerHtml( "increasespInput", '<select id="increaseSpPotion" style="width:80px;" onchange="ToggleItems(0)"></select>', 0 );
	// STR Food
	myInnerHtml( "strFoodLabel", "STR " + GetWord( 133 ), 0 );
	myInnerHtml( "strFoodInput", '<select id="strFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	// AGI Food
	myInnerHtml( "agiFoodLabel", "AGI " + GetWord( 133 ), 0 );
	myInnerHtml( "agiFoodInput", '<select id="agiFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	// VIT Food
	myInnerHtml( "vitFoodLabel", "VIT " + GetWord( 133 ), 0 );
	myInnerHtml( "vitFoodInput", '<select id="vitFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	// INT Food
	myInnerHtml( "intFoodLabel", "INT " + GetWord( 133 ), 0 );
	myInnerHtml( "intFoodInput", '<select id="intFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	// DEX Food
	myInnerHtml( "dexFoodLabel", "DEX " + GetWord( 133 ), 0 );
	myInnerHtml( "dexFoodInput", '<select id="dexFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	// LUK Food
	myInnerHtml( "lukFoodLabel", "LUK " + GetWord( 133 ), 0 );
	myInnerHtml( "lukFoodInput", '<select id="lukFood" style="width:60px;" onchange="ToggleItems(0)"></select>', 0 );
	
	// Build Options
	// Food
	formElements["strFood"].options[0] = new Option( "Off", 0 );
	formElements["agiFood"].options[0] = new Option( "Off", 0 );
	formElements["vitFood"].options[0] = new Option( "Off", 0 );
	formElements["intFood"].options[0] = new Option( "Off", 0 );
	formElements["dexFood"].options[0] = new Option( "Off", 0 );
	formElements["lukFood"].options[0] = new Option( "Off", 0 );
	for ( var i = 1; i <= 30; i++ )
	{
		formElements["strFood"].options[i] = new Option( "+" + i, i );
		formElements["agiFood"].options[i] = new Option( "+" + i, i );
		formElements["vitFood"].options[i] = new Option( "+" + i, i );
		formElements["intFood"].options[i] = new Option( "+" + i, i );
		formElements["dexFood"].options[i] = new Option( "+" + i, i );
		formElements["lukFood"].options[i] = new Option( "+" + i, i );
	}
	
	// SpeedPots
	formElements["speedPot"].options[0] = new Option(SpeedPotName[0][Language],0);
	formElements["speedPot"].options[1] = new Option(SpeedPotName[1][Language],1);
	
	// HP increase potion
	formElements["increaseHpPotion"].options[0] = new Option( "Off", 0 );
	formElements["increaseHpPotion"].options[1] = new Option( "Small", 1 );
	formElements["increaseHpPotion"].options[2] = new Option( "Medium", 2 );
	formElements["increaseHpPotion"].options[3] = new Option( "Large", 3 );
	
	// SP increase potion
	formElements["increaseSpPotion"].options[0] = new Option( "Off", 0 );
	formElements["increaseSpPotion"].options[1] = new Option( "Small", 1 );
	formElements["increaseSpPotion"].options[2] = new Option( "Medium", 2 );
	formElements["increaseSpPotion"].options[3] = new Option( "Large", 3 );
}