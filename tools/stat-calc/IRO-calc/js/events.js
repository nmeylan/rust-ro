function ToggleVisibility( id )
{ // Toggle BattleChant menu
	var element = document.getElementById( id );
	var state = element.style.display;

		  $("#"+id).fadeToggle();
	/*if ( state === 'block' )
	{
	console.log("LOL2?");
		// Hide the Character Section
		  $("#"+id).fadeToggle("slow", "linear", function() {
			//element.style.display = 'none';
		  });

	}
	else
	{
	console.log("LOL?");
		// Show the Character Section
	}*/
}

function TogglePassiveSkills()
{
	// Passive Skills toggled
	StAllCalc();
}

function ToggleAcolyteSkills( selectedItem )
{ // Acolyte Skills toggled
	// check for both pp change and revitalize on
	if ( selectedItem === 1 )
	{
		formElements["ppRevitalize"].value = 0;
	}
	else if ( selectedItem === 2 )
	{
		formElements["ppChange"].value = 0;
	}

	// calculate
	StAllCalc();
}

function TogglePerformerSkills()
{
	// Bard and Dancer skills toggled
	StAllCalc();
}

function ToggleGuildSkills()
{
	// Guild Skills Toggled
	StAllCalc();
}

function ToggleBattleChant()
{
	// Battle Chant Effect Toggled
	StAllCalc();
}

function ToggleOtherBuffs()
{
	// Other Buffs toggled
	StAllCalc();
}

function ToggleItems( selectedItem )
{ // toggle status Items & Food

	// check for vip buffs
	if ( selectedItem === 1 )
	{
		if ( formElements["vipBuff"].checked )
		{
			formElements["strFood"].value = 7;
			formElements["agiFood"].value = 7;
			formElements["vitFood"].value = 7;
			formElements["intFood"].value = 7;
			formElements["dexFood"].value = 7;
			formElements["lukFood"].value = 7;
		}
		else
		{
			formElements["strFood"].value = 0;
			formElements["agiFood"].value = 0;
			formElements["vitFood"].value = 0;
			formElements["intFood"].value = 0;
			formElements["dexFood"].value = 0;
			formElements["lukFood"].value = 0;
		}
	}

	StAllCalc();
}

function ToggleMiscEffects()
{
	// Misc. Effects toggled
	StAllCalc();
}

function ToggleBattleEffects()
{
	// toggle misc battle effects
	calc();
}

function ToggleMonsterDebuff()
{
	// toggle status (enemy) Debuff
	calc();
}

function ToggleMonsterBuff()
{
	// toggle status (enemy) Buff
	calc();
}

function ChangeJob( n )
{
	// Set new Job
	n_A_JobSet();
	n = n_A_JOB;

	// reset passive skills
	ResetPassiveSkills();
	for ( var i = 0; i<= 64; i++ )
	{
		selfBuffs[i] = 0;
	}

	// Adjust form elements
	AdjustJobLevelList( n );
	AdjustBaseLevelList( n );
	AdjustStatLists( n );
	AdjustSpeedPotList( n );
	AdjustWeaponTypeList( n );
	AdjustActiveSkillList( n );
	BuildPassiveSkillTable();

	{ // SNovi FullWeapon ASPD
		if(n_A_JOB != 20)
			SuperNoviceFullWeaponCHECK = 0;
		if(SuperNoviceFullWeaponCHECK)
			JobASPD[20][7] = 120;
		else
			JobASPD[20][7] = 0;
	}

	ClickWeaponType(0);

//	if(n_SaveMode == 0) // new
//		SetShortCut();
	document.forms["calcForm"].elements["saveName"].value = JobName[n_A_JOB][Language];

	// updated extended info
	PrepExtenededInfo();

	// disable adopted checkbox
	CheckAdoptedAvailability();
}

function CheckAdoptedAvailability () {
    if ((n_A_JOB >= 0 && n_A_JOB <= 20) || (n_A_JOB >= 48 && n_A_JOB <= 70 && (n_A_JOB % 2) === 0 ) || (n_A_JOB === 73)) {
	$('#adoptedLabel').text('Adopted');
	$('#adoptedCheck').show();
    } else {
	$('#adoptedLabel').text('');
	$('#adoptedCheck').attr('checked', false);
	$('#adoptedCheck').hide();
    }
}

function PrepExtenededInfo()
{ // SetUp top-right menu
	wKK = parseInt(formElements["ExtendedInfo"].value);

	if ( wKK === 4 )
	{
		if ( n_A_JobSearch() == 6 || n_A_JOB == 20 )
		{
			var str = 'Gym Passes: <select id="GymPass" style="width:50px;" onchange="CalcExtendedInfo()"></select><br/>';
			str += SKILL_NAME[skill_ME_ENLARGE_WEIGHT_LIMIT][Language] + ': <select id="EnlargeWeightLimit" style="width:50px;" onchange="CalcExtendedInfo()"></select><br/>';
			myInnerHtml( "A_KakutyouSel", str, 0 );
			for ( var i = 0; i < 11; i++ )
			{
				formElements["EnlargeWeightLimit"].options[i] = new Option( i, i );
				formElements["GymPass"].options[i] = new Option( i, i );
			}
			formElements["EnlargeWeightLimit"].value = 0;
			formElements["GymPass"].value = 0;
		}
		else
		{
			var str = 'Gym Passes: <select name="GymPass" style="width:50px;" onchange="CalcExtendedInfo()"></select><br/>';
			myInnerHtml( "A_KakutyouSel", str,0);
			for ( var i = 0; i < 11; i++ )
			{
				formElements["GymPass"].options[i] = new Option( i, i );
			}
			formElements["GymPass"].value = 0;
		}
	}
	else if ( wKK === 10 )
	{
		var w;
		w = '<Font size="2">'+ GetWord(143) +'&nbsp;<input type="text" name="A_KakutyouSelNum" value="0" size=4 onChange="CalcExtendedInfo()" style="text-align : right;">%<BR>';
		w += GetWord(144) +'&nbsp;<input type="text" name="A_KakutyouSelNum2" value="0" size=4 onChange="CalcExtendedInfo()" style="text-align : right;">%<BR></Font><hr>';
		myInnerHtml("A_KakutyouSel",w,0);
	}

	CalcExtendedInfo();
}

function CalcExtendedInfo()
{ // calc top-right menu
	wKK = parseInt(formElements["ExtendedInfo"].value);
	if ( wKK === 0 )
	{ // Blank
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", "", 0 );
		return;
	}
	else if ( wKK === 1 )
	{
		myInnerHtml( "A_KakutyouSel", "", 0 );
		// Build Heal Value Table
		var healTable = '<table class="bgLtTable"><tr>';
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 1</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 1, 0, 1 ) + "-" + CalcHeal( 1, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 6</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 6, 0, 1 ) + "-" + CalcHeal( 6, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">H. Heal 1</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHighHeal( 1, 0, 1 ) + "-" + CalcHighHeal( 1, 1, 1 ) + "</td>";
		healTable += "</tr><tr>";
		healTable += '<td class="bgLtRow4 padded optCaption">Heal 2</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHeal( 2, 0, 1 ) + "-" + CalcHeal( 2, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow4 padded optCaption">Heal 7</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHeal( 7, 0, 1 ) + "-" + CalcHeal( 7, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow4 padded optCaption">H. Heal 2</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHighHeal( 2, 0, 1 ) + "-" + CalcHighHeal( 2, 1, 1 ) + "</td>";
		healTable += "</tr><tr>";
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 3</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 3, 0, 1 ) + "-" + CalcHeal( 3, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 8</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 8, 0, 1 ) + "-" + CalcHeal( 8, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">H. Heal 3</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHighHeal( 3, 0, 1 ) + "-" + CalcHighHeal( 3, 1, 1 ) + "</td>";
		healTable += "</tr><tr>";
		healTable += '<td class="bgLtRow4 padded optCaption">Heal 4</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHeal( 4, 0, 1 ) + "-" + CalcHeal( 4, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow4 padded optCaption">Heal 9</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHeal( 9, 0, 1 ) + "-" + CalcHeal( 9, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow4 padded optCaption">H. Heal 4</td>';
		healTable += '<td class="bgLtRow2 padded optArea">' + CalcHighHeal( 4, 0, 1 ) + "-" + CalcHighHeal( 4, 1, 1 ) + "</td>";
		healTable += "</tr><tr>";
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 5</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 5, 0, 1 ) + "-" + CalcHeal( 5, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">Heal 10</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHeal( 10, 0, 1 ) + "-" + CalcHeal( 10, 1, 1 ) + "</td>";
		healTable += '<td class="bgLtRow3 padded optCaption">H. Heal 5</td>';
		healTable += '<td class="bgLtRow1 padded optArea">' + CalcHighHeal( 5, 0, 1 ) + "-" + CalcHighHeal( 5, 1, 1 ) + "</td>";
		healTable += "</tr></table>";
		healTable += GetWord(141) + "+" + ( 5 - ( n_A_BaseLV + n_A_INT ) %5 ) + ".<br/>";
		healTable += "Coluseo Heal. assuming Heal Level 10: " + CalcHeal( 10, 0, 1 ) + "-" + CalcHeal( 10, 1, 1 ) + ".<br/>";
		healTable += "Total of " + (healMultiplier-100) + "% of heal power boost from base.";

		// Post Data
		myInnerHtml( "A_KakutyouData", healTable, 0 );
	}
	else if ( wKK === 2 )
	{ // Increase HP recovery
		if ( n_A_JobSearch() === cls_SWO || n_A_JOB === cls_SNOVI || n_A_JOB === cls_ENOVI )
		{
			var hpRecoveryTable = '<table class="bgLtTable"><tr>';
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 1</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 1 ) + "</td>";
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 6</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 6 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<td class="bgLtRow4 padded optCaption">Level 2</td>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 2 ) + "</td>";
			hpRecoveryTable += '<td class="bgLtRow4 padded optCaption">Level 7</td>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 7 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 3</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 3 ) + "</td>";
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 8</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 8 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<td class="bgLtRow4 padded optCaption">Level 4</td>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 4 ) + "</td>";
			hpRecoveryTable += '<td class="bgLtRow4 padded optCaption">Level 9</td>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 9 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 5</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 5 ) + "</td>";
			hpRecoveryTable += '<td class="bgLtRow3 padded optCaption">Level 10</td>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 5 + n_A_MaxHP / 500 ) * 10 ) + "</td>";
			hpRecoveryTable += "</tr></table>";

			// Post Data
			myInnerHtml( "A_KakutyouSel", "", 0 );
			myInnerHtml( "A_KakutyouData", hpRecoveryTable, 0 );
		}
		else if ( n_A_JobSearch2() === cls_MON )
		{
			var hpRecoveryTable = '<table class="bgLtTable"><tr>';
			hpRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 1</th>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxHP / 500 ) + 4 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<th class="bgLtRow4 padded optCaption">Spiritual Cadence Level 2</th>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( n_A_MaxHP / 250 ) + 8 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 3</th>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxHP / 166 ) + 12 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<th class="bgLtRow4 padded optCaption">Spiritual Cadence Level 4</th>';
			hpRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( n_A_MaxHP / 125 ) + 16 ) + "</td>";
			hpRecoveryTable += "</tr><tr>";
			hpRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 5</th>';
			hpRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxHP / 100 ) + 20 ) + "</td>";
			hpRecoveryTable += "</tr></table>";

			// Post Data
			myInnerHtml( "A_KakutyouSel", "", 0 );
			myInnerHtml( "A_KakutyouData", hpRecoveryTable, 0 );
		}
		else
		{
			myInnerHtml( "A_KakutyouSel", GetWord(142), 0 );
			myInnerHtml( "A_KakutyouData", "", 0 );
		}
	}
	else if ( wKK === 3 )
	{ // Increase SP recovery
		if ( n_A_JobSearch() === cls_MAG || n_A_JobSearch2() === cls_PRI ||
			 n_A_JOB === cls_SNOVI || n_A_JOB === cls_ENOVI || n_A_JOB === cls_NIN )
		{
			var spRecoveryTable = '<table class="bgLtTable"><tr>';
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 1</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 1 ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 6</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 6 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Level 2</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 2 ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Level 7</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 7 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 3</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 3 ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 8</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 8 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Level 4</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 4 ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Level 9</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 9 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 5</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 5 ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Level 10</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( 3 + n_A_MaxSP / 500 ) * 10 ) + "</td>";
			spRecoveryTable += "</tr></table>";

			// Post Data
			myInnerHtml( "A_KakutyouSel", "", 0 );
			myInnerHtml( "A_KakutyouData", spRecoveryTable, 0 );
		}
		else if ( n_A_JobSearch2() === cls_MON )
		{
			var spRecoveryTable = '<table class="bgLtTable"><tr>';
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 1</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxSP / 500 ) + 2 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Spiritual Cadence Level 2</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( n_A_MaxSP / 250 ) + 4 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 3</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxSP / 166 ) + 6 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Spiritual Cadence Level 4</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( ( n_A_MaxSP / 125 ) + 8 ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Spiritual Cadence Level 5</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( ( n_A_MaxSP / 100 ) + 10 ) + "</td>";
			spRecoveryTable += "</tr></table>";

			// Post Data
			myInnerHtml( "A_KakutyouSel", "", 0 );
			myInnerHtml( "A_KakutyouData", spRecoveryTable, 0 );
		}
		else if ( n_A_JOB === cls_MIN || n_A_JOB === cls_MINt ||
			 	  n_A_JOB === cls_WAN || n_A_JOB === cls_WANt )
		{
			var spRecoveryTable = '<table class="bgLtTable"><tr>';
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 1</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 1 ) ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 6</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 6 ) ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Voice Lessons 2</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( 3 + ( 3 * 2 ) ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Voice Lessons 7</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( 3 + ( 3 * 7 ) ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 3</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 3 ) ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 8</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 8 ) ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Voice Lessons 4</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( 3 + ( 3 * 4 ) ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow4 padded optCaption">Voice Lessons 9</th>';
			spRecoveryTable += '<td class="bgLtRow2 padded optArea">' + Math.floor( 3 + ( 3 * 9 ) ) + "</td>";
			spRecoveryTable += "</tr><tr>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 5</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 5 ) ) + "</td>";
			spRecoveryTable += '<th class="bgLtRow3 padded optCaption">Voice Lessons 10</th>';
			spRecoveryTable += '<td class="bgLtRow1 padded optArea">' + Math.floor( 3 + ( 3 * 10 ) ) + "</td>";
			spRecoveryTable += "</tr></table>";

			// Post Data
			myInnerHtml( "A_KakutyouSel", "", 0 );
			myInnerHtml( "A_KakutyouData", spRecoveryTable, 0 );
		}
		else
		{
			myInnerHtml( "A_KakutyouSel", GetWord(142), 0 );
			myInnerHtml( "A_KakutyouData", "", 0 );
		}
	}
	else if ( wKK === 4 )
	{ // Weight Limit
		var jobWeightTable =[   0, 800, 400, 400, 600, 200, 800, 800, 400, 600, //  0- 9
							  700, 400,1000, 800, 400, 600, 700, 700, 400,1000, // 10-19
							    0, 800, 400, 600, 700, 400,1000, 800, 400, 600, // 20-29
							  700, 700, 400,1000,   0, 800, 400, 400, 600, 200, // 30-39
							  800, 800, 800, 400, 600, 800,1500,1500,1200,1200, // 40-49
							 1000,1000,1200,1200,1000,1000,1800,1800,1500,1500, // 50-59
							  800, 800,1000,1000,1200,1200,1200,1200,1000,1000, // 60-69
							 1200,1200, 600, 0];
		var weightLimit = 2000 + jobWeightTable[n_A_JOB];
		var EquipKG = 0;
		var cartWeight = 8000;

		// babies don't get the bonus
		if ( formElements["A_youshi"].checked )
		{
			weightLimit = 2000;
		}

		// Increase weight limit by base strength
		weightLimit += parseInt(formElements["A_STR"].value) * 30;

		// Riding a peco or dragon increases weight limit
		if ( SkillSearch( skill_KN_CAVALIER_MASTERY ) )//|| SkillSearch( skill_RUN_DRAGON_TRAINING ) )
		{
			weightLimit += 1000;
		}
		else if ( SkillSearch( skill_RUN_DRAGON_TRAINING ) )
		{
			weightLimit += 500 + ( SkillSearch( skill_RUN_DRAGON_TRAINING ) * 200 );
		}

		// Increase Weight Limit skill
		if( n_A_JobSearch() === 6 || n_A_JOB === 20 )
		{
			weightLimit += parseInt(formElements["EnlargeWeightLimit"].value) * 200;
		}

		// Gym Passes
		weightLimit += parseInt(formElements["GymPass"].value) * 200;

		// Cart Remodeling
		if ( n_A_JOB == 70 || n_A_JOB == 71 )
		{
			cartWeight += SkillSearch( skill_GEN_CART_REMODELING ) * 500;
		}

		// Calculate current weight from equips
		EquipKG = 0;
		for ( var i = 0; i < 11; i++ )
		{
			EquipKG += ItemOBJ[n_A_Equip[i]][6];
		}

		// build table
		var weightLimitTable = '<table class="bgLtTable"><tr>';
		weightLimitTable += '<th class="bgLtRow4 padded optCaption">' + GetWord(146) + "</th>";
		weightLimitTable += '<td class="bgLtRow2 padded optArea">' + weightLimit + "</td>";
		weightLimitTable += "</tr><tr>";
		weightLimitTable += '<th class="bgLtRow3 padded optCaption">' + GetWord(147) + "</th>";
		weightLimitTable += '<td class="bgLtRow1 padded optArea">' + EquipKG + "</td>";
		if ( n_A_JobSearch() === 6 || n_A_JOB === 20 )
		{
			weightLimitTable += "</tr><tr>";
			weightLimitTable += '<th class="bgLtRow4 padded optCaption">' + GetWord(252) + "</th>";
			weightLimitTable += '<td class="bgLtRow2 padded optArea">' + cartWeight + "</td>";
		}
		weightLimitTable += "</tr></table>";

		// Post Data
		myInnerHtml( "A_KakutyouData", weightLimitTable, 0 );
	}
	else if ( wKK === 5 )
	{ // Elemental Resistance
		var innerStr = new Array();

		// Neutral, Water, Earth, Fire, Wind, Poison, Holy, Dark, Ghost, Undead
		for ( var i = 0; i <= 9; i++ )
		{
			if ( n_A_zokusei[i] == 100 )
			{
				wkk6a = "<b>";
				wkk6b = "</b>";
			}
			if ( n_A_zokusei[i] < 100 )
			{
				wkk6a = "<font color=blue><b>";
				wkk6b = "</b></font>";
			}
			if ( n_A_zokusei[i] > 100 )
			{
				wkk6a = "<font color=red><b>";
				wkk6b = "</b></font>";
			}

			innerStr[i] = wkk6a + n_A_zokusei[i] + " %" + wkk6b;
		}

		var elementResistTable = '<table class="bgLtTable"><tr>';
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[0][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[0] + "</td>";
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[5][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[5] + "</td>";
		elementResistTable += "</tr><tr>";
		elementResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + ZokuseiOBJ[1][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[1] + "</td>";
		elementResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + ZokuseiOBJ[6][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[6] + "</td>";
		elementResistTable += "</tr><tr>";
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[2][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[2] + "</td>";
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[7][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[7] + "</td>";
		elementResistTable += "</tr><tr>";
		elementResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + ZokuseiOBJ[3][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[3] + "</td>";
		elementResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + ZokuseiOBJ[8][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[8] + "</td>";
		elementResistTable += "</tr><tr>";
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[4][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[4] + "</td>";
		elementResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + ZokuseiOBJ[9][Language] + "</b></th>";
		elementResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[9] + "</td>";
		elementResistTable += "</tr></table>";

		// Post Data
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", elementResistTable, 0 );
	}
	else if ( wKK === 6 )
	{ // Race Resistance
		var innerStr = new Array();

		// Formless, Undead, Brute, Plant, Insect, Fish, Demon, Demi-Human, Angel, Dragon
		for ( var i = 50; i <= 59; i++ )
		{
			if ( n_tok[i] == 0 )
			{
				wkk6a = "<b>";
				wkk6b = "</b>";
			}
			if ( n_tok[i] > 0 )
			{
				wkk6a = "<font color=blue><b>";
				wkk6b = "</b></font>";
			}
			if ( n_tok[i] < 0 )
			{
				wkk6a = "<font color=red><b>";
				wkk6b = "</b></font>";
			}
			innerStr[i - 50] = wkk6a + n_tok[i] +" %" + wkk6b;
		}

		var raceResistTable = '<table class="bgLtTable"><tr>';
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[0][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[0] + "</td>";
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[5][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[5] + "</td>";
		raceResistTable += "</tr><tr>";
		raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[1][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[1] + "</td>";
		raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[6][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[6] + "</td>";
		raceResistTable += "</tr><tr>";
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[2][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[2] + "</td>";
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[7][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[7] + "</td>";
		raceResistTable += "</tr><tr>";
		raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[3][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[3] + "</td>";
		raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[8][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[8] + "</td>";
		raceResistTable += "</tr><tr>";
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[4][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[4] + "</td>";
		raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[9][Language] + "</b></th>";
		raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[9] + "</td>";
		raceResistTable += "</tr></table>";

		// Post Data
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", raceResistTable, 0 );
	}
	else if ( wKK === 7 )
	{ // Status Resistance
		var statusResistData = new Array();

		// calculate status resistance for:
		// AGI
		statusResistData[5] = Math.floor( n_A_AGI * 100 ) / 100; // Sleep
		statusResistData[8] = Math.floor( n_A_AGI * 100 ) / 100; // Bleeding
		// VIT
		statusResistData[0] = Math.floor( n_A_VIT * 100 ) / 100; // Poison
		statusResistData[1] = Math.floor( n_A_VIT * 100 ) / 100; // Stun
		// INT
		statusResistData[4] = Math.floor( n_A_INT * 100 ) / 150; // Blind
		statusResistData[6] = Math.floor( n_A_INT * 100 ) / 100; // Silence
		statusResistData[7] = Math.floor( n_A_INT * 100 ) / 150; // Chaos
		// LUK
		statusResistData[3] = Math.floor( n_A_LUK * 100 ) / 100; // Curse

		// MDEF
		statusResistData[2] = Math.floor( n_A_MDEF * 100 ) / 100; // Freeze
		statusResistData[9] = Math.floor( n_A_MDEF * 100 ) / 100; // Stone

		// additional modifiers
		if ( n_A_LUK == 0 )
		{
			statusResistData[3] = 100;
		}
		if ( n_A_BodyZokusei == ele_UNDEAD )
		{
			statusResistData[2] = 100;
			statusResistData[8] = 100;
			statusResistData[9] = 100;
		}
		for ( var i = 0; i <= 9; i++ )
		{
			statusResistData[i] = Math.floor( ( statusResistData[i] + n_tok[150+i] ) );
			statusResistData[i] = Math.min(100, Math.floor( statusResistData[i] * 100 ) / 100);
		}

		// Format strings
		for ( var i = 0; i <= 9; i++ )
		{
			statusResistData[i] += "%"; // Value
		}

		// Build table
		var statusResistTable = '<table class="bgLtTable"><tr>';
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[0][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[0] + "</td>";
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[5][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[5] + "</td>";
		statusResistTable += "</tr><tr>";
		statusResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + AilmentsOBJ[1][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow2 padded optArea">' + statusResistData[1] + "</td>";
		statusResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + AilmentsOBJ[6][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow2 padded optArea">' + statusResistData[6] + "</td>";
		statusResistTable += "</tr><tr>";
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[2][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[2] + "</td>";
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[7][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[7] + "</td>";
		statusResistTable += "</tr><tr>";
		statusResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + AilmentsOBJ[3][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow2 padded optArea">' + statusResistData[3] + "</td>";
		statusResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + AilmentsOBJ[8][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow2 padded optArea">' + statusResistData[8] + "</td>";
		statusResistTable += "</tr><tr>";
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[4][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[4] + "</td>";
		statusResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + AilmentsOBJ[9][Language] + "</b></th>";
		statusResistTable += '<td class="bgLtRow1 padded optArea">' + statusResistData[9] + "</td>";
		statusResistTable += "</tr></table>";

		statusResistTable += GetWord(149);

		// Post Data
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", statusResistTable, 0 );
	}
	else if ( wKK === 8 )
	{ // Other Resistance
		var innerStr = new Array();

		for ( var i = 77; i <= 79; i++ )
		{ // Boss, LongRange, Normal
			if ( n_tok[i] == 0 )
			{
				wkk6a = "<b>";
				wkk6b = "</b>";
			}
			if ( n_tok[i] > 0 )
			{
				wkk6a = "<font color=blue><b>";
				wkk6b = "</b></font>";
			}
			if ( n_tok[i] < 0 )
			{
				wkk6a = "<font color=red><b>";
				wkk6b = "</b></font>";
			}
			innerStr[i - 77] = wkk6a + n_tok[i] + "%" + wkk6b;
		}

		for ( var i = 190; i <= 192; i++ )
		{ // Small, Medium, Large
			if ( n_tok[i] == 0 )
			{
				wkk6a = "<b>";
				wkk6b = "</b>";
			}
			if ( n_tok[i] > 0 )
			{
				wkk6a = "<font color=blue><b>";
				wkk6b = "</b></font>";
			}
			if  ( n_tok[i] < 0 )
			{
				wkk6a = "<font color=red><b>";
				wkk6b = "</b></font>";
			}
			innerStr[i + 3 - 190] = wkk6a + n_tok[i] + "%" + wkk6b;
		}

		var otherResistTable = '<table class="bgLtTable"><tr>';
		otherResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + GetWord(150) + "</b></th>";
		otherResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[0] + "</td>";
		otherResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SizeOBJ[0][Language] + "</b></th>";
		otherResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[3] + "</td>";
		otherResistTable += "</tr><tr>";
		otherResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + GetWord(151) + "</b></th>";
		otherResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[1] + "</td>";
		otherResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SizeOBJ[1][Language] + "</b></th>";
		otherResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[4] + "</td>";
		otherResistTable += "</tr><tr>";
		otherResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + GetWord(152) + "</b></th>";
		otherResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[2] + "</td>";
		otherResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SizeOBJ[2][Language] + "</b></th>";
		otherResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[5] + "</td>";
		otherResistTable += "</tr></table>";

		// Post Data
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", otherResistTable, 0 );
	}
	else if ( wKK == 9 )
	{ // Cast time and Delay
		var fixedCast = ( fixedCastTime * 100 ) + "%";
		var flatFixedCast = (-fixedCastTimeSub) + "s";
		var varCast = Math.round(variableCastTime * 10000 ) / 100 + "%";
		var delay = ( 100 - globalCastDelay ) + "%";
		var cooldown = ( reuseDelay * 100 ) + "%";

		// build table
		var castTimeTable = '<table class="bgLtTable"><tr>';
		castTimeTable += '<th class="bgLtRow4 padded optCaption">' + GetWord(254) + "</th>";
		castTimeTable += '<td class="bgLtRow2 padded optArea">' + fixedCast + "</td>";
		castTimeTable += "</tr><tr>";
		castTimeTable += '<th class="bgLtRow3 padded optCaption">' + GetWord(263) + "</th>";
		castTimeTable += '<td class="bgLtRow1 padded optArea">' + flatFixedCast + "</td>";
		castTimeTable += "</tr><tr>";
		castTimeTable += '<th class="bgLtRow3 padded optCaption">' + GetWord(255) + "</th>";
		castTimeTable += '<td class="bgLtRow1 padded optArea">' + varCast + "</td>";
		castTimeTable += "</tr><tr>";
		castTimeTable += '<th class="bgLtRow4 padded optCaption">' + GetWord(155) + "</th>";
		castTimeTable += '<td class="bgLtRow2 padded optArea">' + delay + "</td>";
		castTimeTable += "</tr><tr>";
		castTimeTable += '<th class="bgLtRow3 padded optCaption">' + GetWord(256) + "</th>";
		castTimeTable += '<td class="bgLtRow1 padded optArea">' + cooldown + "</td>";
		castTimeTable += "</tr></table>";

		// Post Data
		myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", castTimeTable, 0 );
	}
	else if ( wKK == 10 )
	{ // Experience
		var NowBaseExp = !isNaN(document.calcForm.A_KakutyouSelNum.value) ? document.calcForm.A_KakutyouSelNum.value : 0;
		var NowJobExp = !isNaN(document.calcForm.A_KakutyouSelNum2.value) ? document.calcForm.A_KakutyouSelNum2.value : 0;
		var JobType=0; // Novi
		var MaxBaseLV = 0;
		if(n_A_JOB == cls_HNOV)
			JobType = 1; // HNovi
		if((1 <= n_A_JOB && n_A_JOB <= 6) || n_A_JOB == 41 || n_A_JOB == 20)
			JobType = 2; // First Jobs (non GS/Nin)
		else if((35 <= n_A_JOB && n_A_JOB <= 40))
			JobType = 3; // First Trans Jobs
		else if((7 <= n_A_JOB && n_A_JOB <= 19) || n_A_JOB == 43)
			JobType = 4; // 2nd jobs
		else if(21 <= n_A_JOB && n_A_JOB<=33)
			JobType = 5; // 2nd trans jobs
		else if(n_A_JOB == 42)
			JobType = 7; // TKM
		else if(n_A_JOB == 44 || n_A_JOB == 45)
			JobType = 6; // Nin & GS
		else if(46 <= n_A_JOB && n_A_JOB <= 73)
			JobType = 8; // 3rd Cls

		if (n_A_JOB >= 0 && n_A_JOB <= 45)
			MaxBaseLV = CONST_MAXLVL;
		if (n_A_JOB >= 46 && n_A_JOB <= 71)
			MaxBaseLV = CONST_MAXLVL_THIRD;
		if (n_A_JOB >= 72)
			MaxBaseLV = CONST_MAXLVL_KAGOB_ENOVI;

		NowBaseExp = Math.floor(PC_BaseExp[rebirthClass][n_A_BaseLV] * NowBaseExp / 100);

		var wkk11;
		wkk11 = "<Font size=2>Base EXP points needed to reach next level: <B>"+ Kanma(PC_BaseExp[rebirthClass][n_A_BaseLV] - NowBaseExp) +"</B><br/>";

		var MonsterNum=0;
		var OneCheck = 0;
		if ( n_B[en_BASEEXP] !== 0 )
		{
			for ( var i = n_A_BaseLV; i < MaxBaseLV; i++ )
			{
				var LvUpExp = PC_BaseExp[rebirthClass][i];
				var expDiff = LvUpExp - NowBaseExp;
				var monsterExpWithLvlMod = n_B[en_BASEEXP]; //EXP is already modded
				var w1;

				if (expDiff / monsterExpWithLvlMod >= 1) { //If monsterEXP is greater than needed EXP then force 1, otherwise calc
				    w1 = Math.floor( expDiff / monsterExpWithLvlMod );

				    if ((expDiff / monsterExpWithLvlMod) - Math.floor(expDiff / monsterExpWithLvlMod) > 0) { //If there's overflowing EXP, count 1 more
					w1++;
				    }
				} else {
				    w1 = 1;
				}

				MonsterNum += w1;
				NowBaseExp += w1 * monsterExpWithLvlMod;

//				while ( NowBaseExp < LvUpExp )
//				{
//					NowBaseExp += n_B[en_BASEEXP];// * expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL]);
//					MonsterNum += 1
//				}

				if ( OneCheck === 0 ) //For the current level
				{
					OneCheck = 1;
					wkk11 += "You need to kill <B>" +Kanma(MonsterNum) + " " + n_B[en_CLASS] + "</B> to level up.<BR>";
				}

				NowBaseExp -= LvUpExp;
				if ( NowBaseExp > LvUpExp -1 )
				{
					NowBaseExp = LvUpExp -1;
				}
			}

			wkk11 += "To reach Base Lv." + MaxBaseLV + " you need to kill <B> "+ Kanma(MonsterNum) +"</B> " + n_B[en_CLASS] +GetWord(159)+".<BR>";
		}

		NowJobExp = Math.floor(PC_JobExp[JobType][n_A_JobLV] * NowJobExp / 100);
		if((JobType == 0 || JobType == 1) && n_A_JobLV == 10)
			NowJobExp = 0;
		else if((JobType == 2 || JobType == 3 || JobType == 4 || JobType == 7 || JobType == 8) && n_A_JOB != cls_SNOVI && n_A_JOB != cls_ENOVI && n_A_JobLV == 50)
			NowJobExp = 0;
		else if((JobType == 5 || JobType == 6) && n_A_JobLV == 70)
			NowJobExp = 0;
		else if ( ( n_A_JOB == cls_SNOVI || n_A_JOB === cls_ENOVI ) && n_A_JobLV == 99)
			NowJobExp = 0;

		wkk11 += "<hr>Job EXP points needed to reach next level: <B>"+ Kanma(PC_JobExp[JobType][n_A_JobLV] - NowJobExp) +"</B><br/>";

		MonsterNum=0;
		OneCheck = 0;
		if ( n_B[en_JOBEXP] != 0 )
		{
			//for(i=1;PC_JobExp[JobType][i]!=0;i++);
			//var MaxJobLV = i;
			var MaxJobLV = 50;//PC_JobExp[JobType].length-1
			if (n_A_JOB === 0 || n_A_JOB === 34) MaxJobLV = 10;
			if((n_A_JOB >= 1 && n_A_JOB <= 6) || n_A_JOB === 41)
				MaxJobLV = 50;
			if((n_A_JOB >= 21 && n_A_JOB <= 33) || (n_A_JOB >= 44 && n_A_JOB <= 45))
				MaxJobLV = 70;
			if (n_A_JOB === cls_SNOVI)
				MaxJobLV = 99;
			if (n_A_JOB >= 46 && n_A_JOB <= 71)
				MaxJobLV = 60;
			for(i=n_A_JobLV;i<MaxJobLV;i++)
			{
				var LvUpExp = PC_JobExp[JobType][i];
				var jExpDiff = LvUpExp - NowJobExp;
				var monsterJExpWithLvlMod = n_B[en_JOBEXP];
				/*var expReal = Math.floor(n_B[en_JOBEXP] * expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL])) + ((n_B[en_JOBEXP] * expModByLevelDiff(n_A_BaseLV,n_B[en_LEVEL])) % expReal != 0 ? 1 : 0);
				var w1 = Math.floor((LvUpExp - NowJobExp) / expReal) + ((LvUpExp - NowJobExp) % expReal != 0 ? 1 : 0);*/
				//var w1 = Math.floor((LvUpExp - NowJobExp) / n_B[en_JOBEXP] * expModByLevelDiff(n_A_BaseLV,n_B[en_JOBEXP]));
				var w1;

				if (jExpDiff / monsterJExpWithLvlMod >= 1) {
				    w1 = Math.floor( jExpDiff / monsterJExpWithLvlMod );

				    if ((jExpDiff / monsterJExpWithLvlMod) - Math.floor(jExpDiff / monsterJExpWithLvlMod) > 0) {
					w1++;
				    }
				} else {
				    w1 = 1;
				}

				MonsterNum += w1;
				NowBaseExp += w1 * monsterJExpWithLvlMod;
				//var b = Math.floor((LvUpExp - NowJobExp) / (a*1.0));
				//NowJobExp += b*a;
				/*while(NowJobExp < LvUpExp)
				{
					NowJobExp += n_B[en_JOBEXP];
					MonsterNum += 1
				}*/
				//MonsterNum += b;
				if(OneCheck === 0)
				{
					OneCheck = 1;
					wkk11 += "You need to kill <B>" + Kanma(MonsterNum) + " " + n_B[en_CLASS] +" </B>to level up.<BR>";
					//wkk11 += "(Equals <B>" + n_B[en_CLASS] +" "+ Kanma(MonsterNum) +"</B> "+ n_B[1] +" kill"+(Kanma(MonsterNum)!=1?"s":"")+GetWord(159)+")<BR>";
				}
				NowJobExp -= LvUpExp;
				if (NowJobExp < 0)  NowJobExp = 0;
				if(NowJobExp > LvUpExp -1)
					NowJobExp = LvUpExp -1;
			}
			wkk11 += "To reach Job Lv."+ MaxJobLV +" you need to kill<B> "+ Kanma(MonsterNum)+"</B> " + n_B[en_CLASS] +GetWord(159)+".<BR>";
			//wkk11 += "Until JobLv"+ MaxJobLV +": <B>"+ Kanma(MonsterNum) +"</B> more "+ n_B[1] +" kill"+(Kanma(MonsterNum)!=1?"s":"")+"<BR><BR>";
		}

		wkk11 += "</Font>";

		//myInnerHtml( "A_KakutyouSel", "", 0 );
		myInnerHtml( "A_KakutyouData", wkk11, 0 );
	}
}

function ClickWeaponType( weaponType )
{
	n_A_JobSet();

	// Show Arrows?
	if ( n_A_JobSearch() === cls_THI ||
		 n_A_JobSearch() === cls_ARC ||
		 ( n_A_JOB === cls_GUN && weaponType !== weapTyp_NONE ) )
	{
		formElements["A_Arrow"].style.visibility = "visible";
		var len = formElements["A_Arrow"].length;
		for ( var i = 0; i < len; i++ )
		{
			// clear arrow options
			formElements["A_Arrow"].options[0] = null;
		}

		if ( weaponType == weapTyp_BOW || weaponType == weapTyp_INSTRUMENT || weapTyp_WHIP == 15 )
		{
			// Arrows
			for ( var i = 0; i < ArrowOBJ.length; i++ )
			{
				formElements["A_Arrow"].options[i] = new Option( ArrowOBJ[i][2 + Language], i );
			}
		}
		else if ( weaponType == weapTyp_HANDGUN || weaponType == weapTyp_RIFLE ||
				  weaponType == weapTyp_SHOTGUN || weaponType == weapTyp_GATLING_GUN )
		{
			// bullets
			for ( var i = 0; i < BulletOBJ.length; i++ )
			{
				formElements["A_Arrow"].options[i] = new Option( BulletOBJ[i][2 + Language], i );
			}
		}
		else if ( weaponType == weapTyp_GRENADE_LAUNCHER )
		{
			// grenades
			for ( var i = 0; i < GrenadeOBJ.length; i++ )
			{
				formElements["A_Arrow"].options[i] = new Option( GrenadeOBJ[i][2 + Language], i );
			}
		}
		else
		{
			// Arrows
			for ( var i = 0; i < ArrowOBJ.length; i++ )
			{
				formElements["A_Arrow"].options[i] = new Option( ArrowOBJ[i][2 + Language], i );
			}
			// ArrowOBJ[0] = NONEARROW; // Soll ?
			//formElements["A_Arrow"].options[0] = new Option( SecondArrowOBJ[0][2 + Language], 0 );
			//formElements["A_Arrow"].options[1] = new Option( SecondArrowOBJ[1][2 + Language], 1 );
		}
	}
	else if ( n_A_JOB === cls_MEC || n_A_JOB === cls_MECt ||
			  n_A_JOB === cls_GEN || n_A_JOB === cls_GENt )
	{
		formElements["A_Arrow"].style.visibility = "visible";
		var len = formElements["A_Arrow"].length;
		for ( var i = 0; i < len; i++ )
		{ // clear arrow options
			formElements["A_Arrow"].options[0] = null;
		}
		var len = CannonBallOBJ.length;
		for ( var i = 0; i < CannonBallOBJ.length; i++ )
		{ // Cannon Balls
			formElements["A_Arrow"].options[i] = new Option( CannonBallOBJ[i][2 + Language], i );
		}
	}
	else
	{
		formElements["A_Arrow"].value = 0;
		formElements["A_Arrow"].style.visibility = "hidden";
	}
	WeaponSet();

	if ( weaponType === 0 )
	{
		formElements["A_Weapon_ATKplus"].style.visibility = "hidden";
		formElements["A_Weapon_ATKplus"].value = 0;
	}
	else
	{
		formElements["A_Weapon_ATKplus"].style.visibility = "visible";
	}

	n_A_JobSet();
	if ( (n_A_JobSearch2() == cls_ASS && weaponType != 11) || (n_A_JOB == cls_KAGOB && weaponType != weapTyp_HUUMA_SHURIKEN) )
	{ // AssaCls & kagero
		//if ( n_Nitou == 0 )
		//{ // dual handed
			//var htmlStr = "Left Hand: ";
			var htmlStr = '<select id="A_Weapon2Type"  style="width:200px;" onchange="ClickWeaponType2(this[this.selectedIndex].value) | StAllCalc()">';
			if (n_A_JOB == cls_KAGOB)
				htmlStr += '<option value="0">Fist or Shield</option><option value="1">Dagger</option></select>';
			else
				htmlStr += '<option value="0">Fist or Shield</option><option value="1">Dagger</option><option value="6">One-handed Axe</option><option value="2">One-handed Sword</option></select>';
			myInnerHtml( "A_SobWeaponName", htmlStr, 0 );

			// show the off hand table
			var element = document.getElementById( "offweapon" );
			var state = element.style.display;
			element.style.display = 'block';
		//}

	}
	else
	{
		myInnerHtml("A_SobWeaponName","",0);
		//myInnerHtml("id_right1","",0); // soll
		//myInnerHtml("id_right2","",0); // soll
		myInnerHtml("spanA_weapon2","",0);
		myInnerHtml("spanA_weapon2seiren","",0); // soll weg
		myInnerHtml("spanA_weapon2_CardShort","",0);
		myInnerHtml("nA_weapon2_c1","",0);
		myInnerHtml("nA_weapon2_c2","",0);
		myInnerHtml("nA_weapon2_c3","",0);
		myInnerHtml("nA_weapon2_c4","",0);
		n_Nitou = 0;
		formElements["A_LEFT_DEF_PLUS"].style.visibility = "visible";
		formElements["A_left"].style.visibility = "visible";
		formElements["A_left_card"].style.visibility = "visible";
		//myInnerHtml("ID_A_SHORTCUT_LOAD_BUTTON_L","",0); // soll

		// hide the off hand table
		var element = document.getElementById( "offweapon" );
		var state = element.style.display;
		element.style.display = 'none';
	}

	n_A_Equip[0] = parseInt(formElements["A_weapon1"].value);
	ActiveSkillSetPlus();
	DisplayItemDescription( n_A_Equip[0] );
}

function ClickWeaponType2( n )
{
with(document.calcForm)
{
	n_A_JobSet();
	if ( n != 0 )
	{
		if ( n_Nitou === 0 )
		{
			myInnerHtml("spanA_weapon2",'<select name="A_weapon2" style="width:200px;" onChange="StAllCalc()|DisplayItemDescription(this[this.selectedIndex].value)"></select>',0);
			myInnerHtml("spanA_weapon2seiren", '<select name="A_Weapon2_ATKplus" style="width:50px;" onChange = "StAllCalc()"></select>',0);
			for ( var i = 0; i <= 20;i++ )
			{
				A_Weapon2_ATKplus.options[i] = new Option("+"+i,i);
			}

			myInnerHtml("nA_weapon2_c1",'<select name="A_weapon2_card1" style="width:200px;" onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
			myInnerHtml("nA_weapon2_c2",'<select name="A_weapon2_card2" style="width:200px;" onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
			myInnerHtml("nA_weapon2_c3",'<select name="A_weapon2_card3" style="width:200px;" onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
			myInnerHtml("nA_weapon2_c4",'<select name="A_weapon2_card4" style="width:200px;" onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);

			for ( var i = 0; CardSortOBJ[0][i] != "NULL"; i++ )
			{
				A_weapon2_card1.options[i] = new Option(cardOBJ[CardSortOBJ[0][i]][2],cardOBJ[CardSortOBJ[0][i]][0]);
			}
			for ( var i = 0; CardSortOBJ[1][i] != "NULL"; i++ )
			{
				A_weapon2_card2.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
				A_weapon2_card3.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
				A_weapon2_card4.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
			}
			A_weapon2_card4.options[4] = new Option("Top10",106);

			A_LEFT_DEF_PLUS.style.visibility = "hidden";
			A_LEFT_DEF_PLUS.value = 0;
			A_left.style.visibility = "hidden";
			A_left.value = 305;
			A_left_card.style.visibility = "hidden";
			A_left_card.value = 0;
		}
		myInnerHtml("spanA_weapon2_CardShort",'<select name="A_cardshortLeft" style="width:200px;" onChange="SetCardShortLeft()|StAllCalc()|ActiveSkillSetPlus()"></select>',0);
		A_cardshortLeft.options[0] = new Option("Card Shortcuts",0);
		for ( var i = 1; i <= 38; i++ )
		{
			A_cardshortLeft.options[i] = new Option(CardShort[i][0],i);
		}
		n_Nitou = 1;
		WeaponSetLeft();
	}
	else
	{
		myInnerHtml("spanA_weapon2","",0);
		myInnerHtml("spanA_weapon2seiren","",0);
		myInnerHtml("spanA_weapon2_CardShort","",0);
		myInnerHtml("nA_weapon2_c1","",0);
		myInnerHtml("nA_weapon2_c2","",0);
		myInnerHtml("nA_weapon2_c3","",0);
		myInnerHtml("nA_weapon2_c4","",0);
		n_Nitou = 0;

		A_LEFT_DEF_PLUS.style.visibility = "visible";
		A_left.style.visibility = "visible"
		A_left_card.style.visibility = "visible"
	}
	if ( n_Nitou )
	{
		n_A_Equip[1] = eval(A_weapon2.value);
		ActiveSkillSetPlus();
		DisplayItemDescription(n_A_Equip[1]);
	}
}
}

function ClickActiveSkill()
{
with(document.calcForm)
{
	n_A_ActiveSkill = eval(A_ActiveSkill.value);

	if ( n_A_ActiveSkill >= 3000 )
	{
		n_A_ActiveSkillLV = InsertSkill[n_A_ActiveSkill -3000][3];
		n_A_ActiveSkill = InsertSkill[n_A_ActiveSkill -3000][2];
	}
	else if ( n_A_ActiveSkill >= 2000 )
	{
		n_A_ActiveSkillLV = AutoSpellSkill[n_A_ActiveSkill -2000][3];
		n_A_ActiveSkill = AutoSpellSkill[n_A_ActiveSkill -2000][2];
	}
	else
	{
		n_A_ActiveSkillLV = SkillOBJ[n_A_ActiveSkill][1];
	}

	var len = A_ActiveSkillLV.length;
	for(i=0;i<len;i++)
	{
		A_ActiveSkillLV.options[0] = null;
	}

	if ( n_A_ActiveSkill >= 0 )
	{
		for(i=1;i<=n_A_ActiveSkillLV;i++)
		{
			A_ActiveSkillLV.options[i-1] = new Option(i,i);
		}
	}

	if ( SkillOBJ[n_A_ActiveSkill][1] == 1 )
	{
		A_ActiveSkillLV.style.visibility = "hidden";
	}
	else
	{
		A_ActiveSkillLV.style.visibility = "visible";
		A_ActiveSkillLV.value = n_A_ActiveSkillLV;
	}
	ClickActiveSkill2();
}
}
function ClickMonsterSkill(level)
{
with(document.calcForm)
{
	if (level) {
		n_A_MobSkillLV = eval(A_ActiveSkillLV_en.value);
		calcIncomingDamage();
		return;
	}

	n_A_MobSkill = A_ActiveSkill_en.value;
	n_A_MobSkillLV = 1;

	var len = A_ActiveSkillLV_en.length;
	for(i=0;i<len;i++)
	{
		A_ActiveSkillLV_en.options[0] = null;
	}

	if ( n_A_MobSkill >= 0 )
	{
		for(i=1;i<=enemySkills[n_A_MobSkill].length-3;i++)
		{
			A_ActiveSkillLV_en.options[i-1] = new Option(i,i);
		}
	}

	if ( enemySkills[n_A_MobSkill].length-3 == 1 )
	{
		A_ActiveSkillLV_en.style.visibility = "hidden";
	}
	else
	{
		A_ActiveSkillLV_en.style.visibility = "visible";
		A_ActiveSkillLV_en.value = n_A_MobSkillLV;
	}
	calcIncomingDamage();
}
}
function ClickActiveSkill2()
{ // display dropdowns from active skill
with(document.calcForm)
{
	myInnerHtml( "AASkillName2", "", 0 );
	myInnerHtml( "AASkill2", "", 0 );
	myInnerHtml( "AASkillName3", "", 0 );
	myInnerHtml( "AASkill3", "", 0 );

	if ( n_A_ActiveSkill == skill_ME_CART_REVOLUTION ||
		 n_A_ActiveSkill == skill_MS_HIGH_SPEED_CART_RAM )
	{
		myInnerHtml("AASkillName","<br/>Cart Weight:",0);
		myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" value="8000" size=8>',0);

	}
	else if ( n_A_ActiveSkill == skill_WI_STORM_GUST )
	{
		myInnerHtml("AASkillName","<br/>Hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=1;i<=15;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=3;

	}
	else if ( n_A_ActiveSkill == skill_AS_VENOM_SPLASHER )
	{
		myInnerHtml("AASkillName","<br/>Poison React Lv.:",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=0;i<=5;i++)
			SkillSubNum.options[i] = new Option(i+5,i+5);
		SkillSubNum.value=5;
		if(n_A_JobSearch2() == 14)
			SkillSubNum.value=0;
	}
	else if ( n_A_ActiveSkill === skill_MO_GUILLOTINE_FIST )
	{
		myInnerHtml("AASkillName","<br/>Remaining SP:",0);
		myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6>',0);
		SkillSubNum.value = n_A_MaxSP -1;
	}
	else if ( n_A_ActiveSkill === skill_NIN_THROW_DAGGER )
	{
		myInnerHtml("AASkillName","<br/>",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for ( var i = 0; i < 5; i++ )
		{
			SkillSubNum.options[i] = new Option(ShurikenOBJ[i][2 + Language],i);
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill === skill_NIN_THROW_KUNAI ||
			  n_A_ActiveSkill === skill_KAG_SPINTHROW_KUNAI )
	{
		myInnerHtml("AASkillName","<br/>",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for ( var i = 0; i < 5; i++ )
		{
			SkillSubNum.options[i] = new Option(KunaiOBJ[i][2 + Language],i);
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill == skill_KAG_SPIRIT_BREAKER )
	{
		myInnerHtml("AASkillName","<br/>Enemy in Soul Link Status: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["no","yes"];
		for ( var i = 0; i <= 1; i++ )
		{
			SkillSubNum.options[i] = new Option( CHATK_NAME[i], i );
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill == skill_KAG_CROSS_STRIKE )
	{
		myInnerHtml("AASkillName","<br/>Enemy already hit by another Kagero/Oboro: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["no","yes"];
		for ( var i = 0; i <= 1; i++ )
		{
			SkillSubNum.options[i] = new Option( CHATK_NAME[i], i );
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill === skill_NIN_KILLING_STRIKE ||
			  n_A_ActiveSkill === skill_NIN_KILLING_STRIKE_MAX )
	{
		// remaining HP
		if (n_A_ActiveSkill === skill_NIN_KILLING_STRIKE) {
		    myInnerHtml("AASkillName","<br/>Remaining HP:",0);
		    myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6>',0);
		    SkillSubNum.value = n_A_MaxHP -1;
		} else {
		    myInnerHtml("AASkillName","",0);
		    myInnerHtml("AASkill","",0);
		}
		// number of mirrors
		myInnerHtml( "AASkillName2", "<br/># Mirrors: ", 0 );
		myInnerHtml( "AASkill2", '<select id="SkillSubNum2" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 0; i <= 5; i++ )
		{
			SkillSubNum2.options[i] = new Option( i, i );
		}
		SkillSubNum2.value = 5;
	}
	else if ( n_A_ActiveSkill == skill_GS_DESPERADO )
	{
		myInnerHtml("AASkillName","<br/>Hits (Considering the Success Chance) :",0);
		var DEATH = ["1","1.2","1.6","2","2.4","3","3.6","4","5","6","7","8","9","10"];
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=0;i<=13;i++)
			SkillSubNum.options[i] = new Option(DEATH[i] + "Hit",i);
		SkillSubNum.value = 6;
	}
	else if ( n_A_ActiveSkill == skill_KN_CHARGE_ATTACK )
	{
		myInnerHtml("AASkillName","<br/>Enemy Distance: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["0~3 Cells","4~6 Cells","7~9 Cells","10~12 Cells","13+ Cells"];
		for(i=0;i<=4;i++)
			SkillSubNum.options[i] = new Option(CHATK_NAME[i],i);
		SkillSubNum.value=4;
	}
	else if ( n_A_ActiveSkill == skill_WAR_SOUL_EXPANSION )
	{
		myInnerHtml("AASkillName","<br/>Enemy in White Imprison: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["no","yes"];
		for ( var i = 1; i <= 2; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( CHATK_NAME[i - 1], i );
		}
		SkillSubNum.value = 1;
	}
	else if ( n_A_ActiveSkill == skill_WAR_JACK_FROST )
	{
		myInnerHtml("AASkillName","<br/>Enemy in Freezing Status: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["no","yes"];
		for ( var i = 0; i <= 1; i++ )
		{
			SkillSubNum.options[i] = new Option( CHATK_NAME[i], i );
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill == skill_WAR_COMET )
	{
		myInnerHtml("AASkillName","<br/>Area of Effect: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		var CHATK_NAME = ["19x19","15x15","11x11","7x7"];
		for ( var i = 0; i <= 3; i++ )
		{
			SkillSubNum.options[i] = new Option( CHATK_NAME[i], i );
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill == skill_WAR_CHAIN_LIGHTNING )
	{
		myInnerHtml("AASkillName","<br/>No of hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for( var i = 1; i <= 9; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 3;
	}
	else if ( n_A_ActiveSkill == skill_WAR_TETRA_VORTEX )
	{
		myInnerHtml("AASkillName","<br/>Element: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=ele_NEUTRAL;i<=ele_WIND;i++)
			SkillSubNum.options[i] = new Option(ZokuseiOBJ[i][Language],i);
		SkillSubNum.value=1;
	}
	else if ( n_A_ActiveSkill == skill_SOR_FIRE_WALK )
	{
		myInnerHtml("AASkillName","<br/>No of hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=1;i<=16;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=3;
	}
	else if ( n_A_ActiveSkill == skill_SOR_ELECTRIC_WALK )
	{
		myInnerHtml("AASkillName","<br/>No of hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=1;i<=16;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=3;
	}
	else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_FBOLT )
	{
		myInnerHtml("AASkillName","<br/>Bolt Level: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" style="width:50px;" onchange="calc()"></select>',0);
		for(i=1;i<=10;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=10;
	}
	else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_CBOLT )
	{
		myInnerHtml("AASkillName","<br/>Bolt Level: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" style="width:50px;" onchange="calc()"></select>',0);
		for(i=1;i<=10;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=10;
	}
	else if ( n_A_ActiveSkill == skill_SOR_SPELL_FIST_LBOLT )
	{
		myInnerHtml("AASkillName","<br/>Bolt Level: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" style="width:50px;" onchange="calc()"></select>',0);
		for(i=1;i<=10;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=10;
	}
	else if ( n_A_ActiveSkill == skill_SOR_PSYCHIC_WAVE )
	{
		myInnerHtml("AASkillName","<br/>No of hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=1;i<=7;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=7;
	}
	else if ( n_A_ActiveSkill == skill_SOR_CLOUD_KILL )
	{
		myInnerHtml("AASkillName","<br/>No of hits: ",0);
		myInnerHtml("AASkill",'<select name="SkillSubNum" onchange="calc()"></select>',0);
		for(i=1;i<=33;i++)
			SkillSubNum.options[i-1] = new Option(i,i);
		SkillSubNum.value=33;
	}
	else if ( n_A_ActiveSkill === skill_MIWA_GREAT_ECHO )
	{
		myInnerHtml( "AASkillName", "<br/># Performers: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 2; i <= 12; i++ )
		{
			SkillSubNum.options[i - 2] = new Option( i, i );
		}
		SkillSubNum.value = 2;
	}
	else if ( n_A_ActiveSkill === skill_MIWA_REVERBERATION )
	{
		// number of reverberations
		myInnerHtml( "AASkillName", "<br/># Reverberations: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 20; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 1;
		// number of enemies
		myInnerHtml( "AASkillName2", "<br/># Enemies: ", 0 );
		myInnerHtml( "AASkill2", '<select id="SkillSubNum2" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 20; i++ )
		{
			SkillSubNum2.options[i - 1] = new Option( i, i );
		}
		SkillSubNum2.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_MEC_AXE_TORNADO )
	{
		myInnerHtml( "AASkillName", "<br/># Hits: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:100px;" onchange="calc()"></select>', 0 );
		SkillSubNum.options[0] = new Option( "5x5 area", 0 );
		SkillSubNum.options[1] = new Option( "7x7 area", 1 );
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill === skill_RUN_IGNITION_BREAK )
	{
		myInnerHtml( "AASkillName", "<br/>Distance: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:100px;" onchange="calc()"></select>', 0 );
		SkillSubNum.options[0] = new Option( "Near", 1 );
		SkillSubNum.options[1] = new Option( "Medium", 2 );
		SkillSubNum.options[2] = new Option( "Far", 3 );
		SkillSubNum.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_RUN_DEATH_BOUND )
	{
		myInnerHtml( "AASkillName", "<br/>Damage: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:100px;" onchange="calc()"></select>', 0 );
		var damage = 5000;
		for ( var i = 0; damage <= 200000; i++ )
		{
			SkillSubNum.options[i] = new Option( damage, damage );
			damage += 5000;
		}
		SkillSubNum.value = 5000;
	}
	else if ( n_A_ActiveSkill === skill_RUN_HUNDRED_SPEAR )
	{
		myInnerHtml( "AASkillName", "<br/>Clashing Spiral: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 0; i <= 5; i++ )
		{
			SkillSubNum.options[i] = new Option( i, i );
		}
		SkillSubNum.value = 5;
	}
	else if ( n_A_ActiveSkill === skill_RUN_DRAGON_BREATH || n_A_ActiveSkill === skill_RUN_DRAGON_BREATH_WATER)
	{
		myInnerHtml("AASkillName","<br/>Remaining HP:",0);
		myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6 onchange="calc()">',0);
		SkillSubNum.value = n_A_MaxHP;
	}
	else if ( n_A_ActiveSkill === skill_SUR_EARTH_SHAKER )
	{
		myInnerHtml( "AASkillName", "<br/>Target is visible:", 0 );
		myInnerHtml( "AASkill", '<input type="checkbox" name="SkillSubNum" onchange="calc()">', 0 );
		SkillSubNum.checked = true;
	}
	else if ( n_A_ActiveSkill === skill_SUR_KNUCKLE_ARROW )
	{
		myInnerHtml( "AASkillName", "<br/>Knockback damage:", 0 );
		myInnerHtml( "AASkill", '<input type="checkbox" name="SkillSubNum" onchange="calc()">', 0 );
		SkillSubNum.checked = false;
	}
	else if ( n_A_ActiveSkill === skill_SUR_RAMPAGE_BLASTER )
	{
		myInnerHtml( "AASkillName", "<br/>Number of Spheres: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 15; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 15;
	}
	else if ( n_A_ActiveSkill === skill_SUR_TIGER_CANNON )
	{
		myInnerHtml( "AASkillName", "<br/>Used after Fallen Empire:", 0 );
		myInnerHtml( "AASkill", '<input type="checkbox" name="SkillSubNum" onchange="calc()">', 0 );
		SkillSubNum.checked = false;
	}
	else if ( n_A_ActiveSkill === skill_RAN_AIMED_BOLT )
	{
		myInnerHtml( "AASkillName", "<br/>Number of Hits: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 5; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_GLT_CROSS_RIPPER_SLASHER )
	{
		myInnerHtml( "AASkillName", "<br/>Rolling Cutters: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 0; i <= 10; i++ )
		{
			SkillSubNum.options[i] = new Option( i, i );
		}
		SkillSubNum.value = 0;
	}
	else if ( n_A_ActiveSkill === skill_ROY_VANISHING_POINT )
	{
		myInnerHtml( "AASkillName", "<br/>Bash Level: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 0; i <= 10; i++ )
		{
			SkillSubNum.options[i] = new Option( i, i );
		}
		SkillSubNum.value = 10;
	}
	else if ( n_A_ActiveSkill === skill_ROY_RAGE_BURST )
	{
		// currentHP
		myInnerHtml("AASkillName","<br/>Remaining HP:",0);
		myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6 onchange="calc()">',0);
		SkillSubNum.value = n_A_MaxHP;
		// number of rage counters
		myInnerHtml( "AASkillName2", "<br/># Rage Counters: ", 0 );
		myInnerHtml( "AASkill2", '<select id="SkillSubNum2" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 15; i++ )
		{
			SkillSubNum2.options[i - 1] = new Option( i, i );
		}
		SkillSubNum2.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_ROY_OVERBRAND )
	{
		myInnerHtml( "AASkillName", "<br/># Hits: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 3; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_ROY_OVERBRAND_OLD )
	{
		myInnerHtml( "AASkillName", "<br/># Hits: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 1; i <= 3; i++ )
		{
			SkillSubNum.options[i - 1] = new Option( i, i );
		}
		SkillSubNum.value = 1;
	}
	else if ( n_A_ActiveSkill === skill_MEC_SELF_DESTRUCTION )
	{
		// currentHP
		myInnerHtml( "AASkillName","<br/>Remaining HP:", 0 );
		myInnerHtml( "AASkill", '<input type="text" name="SkillSubNum" size=6 onchange="calc()">', 0 );
		SkillSubNum.value = n_A_MaxHP;
		// currentSP
		myInnerHtml( "AASkillName2", "<br/>Remaining SP: ", 0 );
		myInnerHtml( "AASkill2", '<input type="text" name="SkillSubNum2" size=6 onchange="calc()">', 0 );
		SkillSubNum2.value = n_A_MaxSP;
	}
	else if ( n_A_ActiveSkill === skill_SUR_SKY_NET_BLOW )
	{
		myInnerHtml( "AASkillName", "<br/>After Dragon Combo: ", 0 );
		myInnerHtml( "AASkill", '<input type="checkbox" name="SkillSubNum" onchange="calc()">', 0 );
		SkillSubNum.checked = false;
	}
	else if ( n_A_ActiveSkill === skill_SUR_GATE_OF_HELL )
	{
		// after combo
		myInnerHtml( "AASkillName", "<br/>After a combo: ", 0 );
		myInnerHtml( "AASkill", '<input type="checkbox" name="SkillSubNum" onchange="calc()">', 0 );
		SkillSubNum.checked = false;
		// currentHP
		myInnerHtml( "AASkillName2","<br/>Remaining HP:", 0 );
		myInnerHtml( "AASkill2", '<input type="text" name="SkillSubNum2" size=6 onchange="calc()">', 0 );
		SkillSubNum2.value = n_A_MaxHP;
		// currentSP
		myInnerHtml( "AASkillName3", "<br/>Remaining SP: ", 0 );
		myInnerHtml( "AASkill3", '<input type="text" name="SkillSubNum3" size=6 onchange="calc()">', 0 );
		SkillSubNum3.value = n_A_MaxSP;
	}
	else if ( n_A_ActiveSkill === skill_SUR_CRESCENT_ELBOW )
	{
		// damage
		myInnerHtml( "AASkillName", "<br/>Damage: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:100px;" onchange="calc()"></select>', 0 );
		var damage = 5000;
		for ( var i = 0; damage <= 200000; i++ )
		{
			SkillSubNum.options[i] = new Option( damage, damage );
			damage += 5000;
		}
		SkillSubNum.value = 5000;
	}
	else if ( n_A_ActiveSkill === skill_SOR_VARETYR_SPEAR )
	{
		// Striking Level
		myInnerHtml( "AASkillName", "<br/>Striking Level: ", 0 );
		myInnerHtml( "AASkill", '<select id="SkillSubNum" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 0; i <= 5; i++ )
		{
			SkillSubNum.options[i] = new Option( i, i );
		}
		SkillSubNum.value = 0;

		// Endow Tornado Level
		myInnerHtml( "AASkillName2", "<br/>Endow Tornado Level: ", 0 );
		myInnerHtml( "AASkill2", '<select id="SkillSubNum2" style="width:50px;" onchange="calc()"></select>', 0 );
		for ( var i = 2; i <= 5; i++ )
		{
			SkillSubNum2.options[i - 2] = new Option( i, i );
		}
		SkillSubNum2.value = 2;
	}
	else
	{
		myInnerHtml("AASkillName","",0);
		myInnerHtml("AASkill","",0);
		myInnerHtml( "AASkillName2", "", 0 );
		myInnerHtml( "AASkill2", "", 0 );
		myInnerHtml( "AASkillName3", "", 0 );
		myInnerHtml( "AASkill3", "", 0 );
	}
}
}

function DisplayItemDescription( ItemIndex )
{
	ItemCardNumberCheck = ItemIndex;
	ActiveSkillSetPlus();

	// Build general information
	myInnerHtml( "nm080", ITEM_NAME[ItemIndex][1 + Language * 2], 0 );
	myInnerHtml( "ITEM1", GetWord( 160 ),0 );
	myInnerHtml( "ITEM3", GetWord( 161 ),0 );
	myInnerHtml( "ITEM4", GetWord( 162 ),0 );
	if ( ItemOBJ[ItemIndex][itm_TYPE] < 50 )
	{
		myInnerHtml( "ITEM0", "ATK", 0 );
		myInnerHtml( "ITEM2", GetWord( 163 ), 0 );
		myInnerHtml( "ITEM_W_LV", ItemOBJ[ItemIndex][itm_WLVL], 0 );
	}
	else
	{
		myInnerHtml( "ITEM0", "DEF", 0 );
		myInnerHtml( "ITEM2", "", 0 );
		myInnerHtml( "ITEM_W_LV", "", 0 );
	}

	myInnerHtml( "ITEM_DATA", ItemOBJ[ItemIndex][itm_DEF], 0 );
	myInnerHtml( "ITEM_SLOT", ItemOBJ[ItemIndex][itm_SLOTS], 0 );
	myInnerHtml( "ITEM_LV", ItemOBJ[ItemIndex][itm_REQ_BLVL], 0 );
	myInnerHtml( "ITEM_WAIT", ItemOBJ[ItemIndex][itm_WEIGHT], 0 );

	// Description String
	descriptionString = "";
	for ( var i = itm_BONUS_START; ItemOBJ[ItemIndex][i] !== bon_NONE; i += 2 )
	{ // Collect Bonusses
		descriptionString += BuildItemDescription(ItemOBJ[ItemIndex][i], ItemOBJ[ItemIndex][i + 1] ); // add them to Strings
	}
	if ( ITEM_NAME[ItemIndex][2 + 2 * Language] !== bon_NONE )
	{
		descriptionString += ITEM_NAME[ItemIndex][2 + 2 * Language] +"<BR>";
	}

	// Build Set Bonus Descriptions if any
	var check = 0;
	for ( var i = itm_BONUS_START; ItemOBJ[ItemIndex][i] !== bon_NONE; i += 2 )
	{ // Check for Sets
		if ( ItemOBJ[ItemIndex][i] === bon_SETID )
		{
			descriptionString += "<br/><b>When equipping "+ SetEquipName(ItemOBJ[ItemIndex][i + 1]);
			var setIndex = w_SE[ItemOBJ[ItemIndex][i + 1]][0];
			while ( ItemOBJ[ItemIndex][i + 2] !== bon_NONE && check === 0 )
			{
				if ( setIndex == w_SE[ItemOBJ[ItemIndex][i+3]][0])
				{
					descriptionString += " or<br/>" + SetEquipName(ItemOBJ[ItemIndex][i + 3]);
					i += 2;
				}
				else
				{
					check = 1;
				}
			}
			descriptionString += " at the same time:</b><br/>";
			check = 0;
			for ( var j = itm_BONUS_START; ItemOBJ[setIndex][j] !== bon_NONE; j += 2 )
			{
				descriptionString += BuildItemDescription( ItemOBJ[setIndex][j], ItemOBJ[setIndex][j + 1] );
			}
			if ( ITEM_NAME[setIndex][2 + 2 * Language] !== bon_NONE )
			{
				descriptionString += ITEM_NAME[setIndex][2 + 2 * Language] + "<br/>";
			}
			descriptionString += "";
		}
	}

	// Post the description
	myInnerHtml( "ItemDescription", descriptionString, 0 );
}

function Click_Card( CardIndex )
{ // display item data
	ItemCardNumberCheck = CardIndex;

	ActiveSkillSetPlus();

	for ( i = 0;i <= 4; i++ )
	{
		myInnerHtml( "ITEM" + i, "", 0 );
	}
	myInnerHtml("ITEM_W_LV","",0);
	myInnerHtml("ITEM_DATA","",0);
	myInnerHtml("ITEM_SLOT","",0);
	myInnerHtml("ITEM_LV","",0);
	myInnerHtml("ITEM_WAIT","",0);

	if(CardIndex == 106)
	{ // StarCrumb
		myInnerHtml("nm080","Very Strong or Top10",0);
		var str = "A damaged piece of stars +5<BR>If the star piece is constant damage (?)";
		str += "+40<BR>TOP10 Rank damage +10<BR>(Rank card manufacturing weapons of TOP10 the 4th column)";
		myInnerHtml( "B_SETUMEI", str, 0 );
		return;
	}
	if(201 <= CardIndex && CardIndex <= 204)
	{ // Ele Stones
		myInnerHtml("nm080",cardOBJ[CardIndex][card_att_NAME],0);
		myInnerHtml("B_SETUMEI", "[" + ZokuseiOBJ[CardIndex-200][Language] + "]" + GetWord(173),0);
		return;
	}
	myInnerHtml("nm080",cardOBJ[CardIndex][card_att_NAME] +" Card",0);

	CBIstr = "";
	var tempDesc = "";
	for(i=card_att_BONUS_START;cardOBJ[CardIndex][i] != bon_NONE;i+=2) // Collect Bonusses
		tempDesc += BuildItemDescription(cardOBJ[CardIndex][i],cardOBJ[CardIndex][i+1]); // add them to String
	CBIstr += tempDesc;
	if(cardOBJ[CardIndex][card_att_DESC] != 0)
		CBIstr += cardOBJ[CardIndex][card_att_DESC] +"<BR>";

	var check = 0;
	for(var i=card_att_BONUS_START;cardOBJ[CardIndex][i] != bon_NONE;i+=2){ // Check for Sets
		if(cardOBJ[CardIndex][i] == 90){
			CBIstr += "<Font size=2><BR><B>When equipping "+ SetCardName(cardOBJ[CardIndex][i+1]);
			var w = w_SC[cardOBJ[CardIndex][i+1]][0];
			while(cardOBJ[CardIndex][i+2] != bon_NONE && check == 0){
				if(w == w_SE[cardOBJ[CardIndex][i+3]][0]){
					CBIstr += " or<BR>"+ SetCardName(CardOBJ[CardIndex][i+3]);
					i += 2;
				}else
					check = 1;
			}
			CBIstr += " at the same time:<BR>";
			check = 0;
			for(var j=card_att_BONUS_START;cardOBJ[w][j] != bon_NONE;j+=2)
				BuildItemDescription(cardOBJ[w][j],cardOBJ[w][j+1]);
			if(cardOBJ[w][card_att_DESC] != bon_NONE)
				CBIstr += cardOBJ[w][card_att_DESC] +"<BR>";
			CBIstr += "</Font></B>";
		}
	}

	myInnerHtml( "ItemDescription", CBIstr, 0 );
}

function ChangeShortCut_R()
{ // EventWeaponCardShortcuts
	if(n_SaveMode == 1)
		LoadShortCut();
}

function LoadShortCut()
{ // Apply WeaponCardShortcuts
	var num = eval(document.calcForm.A_SHORTCUT_R.value);
	if(n_SaveMode == 0)
	{
		var w = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][1];
		var w2 = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][2];
		var w4 = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][4];
		if(JobASPD[n_A_JOB][w] != 0 && JobEquipItemSearch(w2) == 1 || (w4 == 4 && SuperNoviceFullWeaponCHECK))
		{
			document.calcForm.A_WeaponType.value = w;
			ClickWeaponType(w);
			document.calcForm.A_weapon1.value = DataShortCut[n_LastSaveNum][num][0];
			document.calcForm.A_Weapon_ATKplus.value = DataShortCut[n_LastSaveNum][num][1];
			document.calcForm.A_Weapon_element.value = DataShortCut[n_LastSaveNum][num][2];
			document.calcForm.A_weapon1_card1.value = DataShortCut[n_LastSaveNum][num][3];
			document.calcForm.A_weapon1_card2.value = DataShortCut[n_LastSaveNum][num][4];
			document.calcForm.A_weapon1_card3.value = DataShortCut[n_LastSaveNum][num][5];
			document.calcForm.A_weapon1_card4.value = DataShortCut[n_LastSaveNum][num][6];
		}
	}
	else
	{
		document.calcForm.A_weapon1_card1.value = DataShortCut[0][num][3];
		document.calcForm.A_weapon1_card2.value = DataShortCut[0][num][4];
		document.calcForm.A_weapon1_card3.value = DataShortCut[0][num][5];
		document.calcForm.A_weapon1_card4.value = DataShortCut[0][num][6];
	}
	StAllCalc();
	ActiveSkillSetPlus();
}

function LoadShortCut_L()
{
	var num = eval(document.calcForm.A_SHORTCUT_R.value);
	if(n_SaveMode == 0)
	{
		var w = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][1];
		if(w == 11)
			return;
		var w2 = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][2];
		var w4 = ItemOBJ[DataShortCut[n_LastSaveNum][num][0]][4];
		if(JobASPD[n_A_JOB][w] != 0 && JobEquipItemSearch(w2) == 1 || (w4 == 4 && SuperNoviceFullWeaponCHECK))
		{
			document.calcForm.A_Weapon2Type.value = w;
			ClickWeaponType2(w);
			document.calcForm.A_weapon2.value = DataShortCut[n_LastSaveNum][num][0];
			document.calcForm.A_Weapon2_ATKplus.value = DataShortCut[n_LastSaveNum][num][1];
			document.calcForm.A_weapon2_card1.value = DataShortCut[n_LastSaveNum][num][3];
			document.calcForm.A_weapon2_card2.value = DataShortCut[n_LastSaveNum][num][4];
			document.calcForm.A_weapon2_card3.value = DataShortCut[n_LastSaveNum][num][5];
			document.calcForm.A_weapon2_card4.value = DataShortCut[n_LastSaveNum][num][6];
		}
	}
	StAllCalc();
	ActiveSkillSetPlus();
}

function SetCardShort()
{ // EquipCardShortcuts
with(document.calcForm)
{
	w = eval(A_cardshort.value); // dropdown
	if ( w > 0 )
	{
		if ( EquipShortCutData[w][0] !== 9999 )
		{
			if(EquipShortCutData[w][1] != 0)
				A_weapon1_card1.value = EquipShortCutData[w][1];
			if(EquipShortCutData[w][2] != 0)
				A_head1_card.value = EquipShortCutData[w][2];
			if(EquipShortCutData[w][3] != 0)
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
		else
		{
			A_weapon1_card1.value = 0;
			A_weapon1_card2.value = 0;
			A_weapon1_card3.value = 0;
			A_weapon1_card4.value = 0;

			if(typeof A_weapon2_card1 != "undefined")
			{
				A_weapon2_card1.value = 0;
				A_weapon2_card2.value = 0;
				A_weapon2_card3.value = 0;
				A_weapon2_card4.value = 0;
			}

			A_head1_card.value = 0;
			A_head2_card.value = 0;
			A_left_card.value = 0;
			A_body_card.value = 0;
			A_shoulder_card.value = 0;
			A_shoes_card.value = 0;
			A_acces1_card.value = 0;
			A_acces2_card.value = 0;
		}
		ActiveSkillSetPlus();
	}
}
}

function calcReductionTable() {
	var current_race = $("#reduction_race").val();
	var current_size = $("#reduction_size").val();
	var current_type = $("#reduction_type").val();

	var physicalTable = '<tr>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">Element List</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">Elemental Reduction</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Race Reductions</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Size Reductions</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Distance Reductions</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Normal/Boss Reductions</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Defense reduction</th>'
	physicalTable += '<th class="bgLtRow3 padded optCaption">+ Magic Defense reduction</th>'
	physicalTable += '</tr>'
	physicalTable += '<tr>'

	var names = ['Neutral', 'Water', 'Earth', 'Fire', 'Wind', 'Poison', 'Holy', 'Dark', 'Ghost', 'Undead'];

	var table = []

	for (var i = 0; i < 10; i++) {

		var row = [];

		// Element
		row.push(n_A_zokusei[i] / 100);
		// Race
		row.push(row[0] * (100 - n_tok[parseInt(current_race)]) / 100);
		// Size
		row.push(row[1] * (100 - n_tok[parseInt(current_size)]) / 100);
		// Distance
		row.push(row[2] * (parseInt(current_type) > 0 ? (100 - n_tok[78]) : 100) / 100);
		// Normal/Boss
		row.push(row[3] * (100 - n_tok[parseInt(current_type)]) / 100);
		// DEF
		row.push(row[4] * defReduction(n_A_totalDEF));
		// MDEF
		row.push(row[4] * mdefReduction(n_A_MDEF));

		physicalTable += '<tr>'
		physicalTable += '<td class="bgLtRow1 padded optArea">' + names[i] + '</td>'
		for (var j = 0; j < row.length; j++) {
			physicalTable += '<td class="bgLtRow1 padded optArea">' + (row[j] * 100) + ' %</td>'
		}
		physicalTable += '</tr>'
	}
	// var raceResistTable = '<table class="bgLtTable"><tr>';
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[0][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[0] + "</td>";
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[5][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[5] + "</td>";
	// raceResistTable += "</tr><tr>";
	// raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[1][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[1] + "</td>";
	// raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[6][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[6] + "</td>";
	// raceResistTable += "</tr><tr>";
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[2][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[2] + "</td>";
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[7][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[7] + "</td>";
	// raceResistTable += "</tr><tr>";
	// raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[3][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[3] + "</td>";
	// raceResistTable += '<th class="bgLtRow4 padded optCaption"><b>' + SyuzokuOBJ[8][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow2 padded optArea">' + innerStr[8] + "</td>";
	// raceResistTable += "</tr><tr>";
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[4][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[4] + "</td>";
	// raceResistTable += '<th class="bgLtRow3 padded optCaption"><b>' + SyuzokuOBJ[9][Language] + "</b></th>";
	// raceResistTable += '<td class="bgLtRow1 padded optArea">' + innerStr[9] + "</td>";
	// raceResistTable += "</tr></table>";

	// Post Data
	// myInnerHtml( "A_KakutyouSel", "", 0 );
	myInnerHtml( "physical_reductions", physicalTable, 0 );
}
