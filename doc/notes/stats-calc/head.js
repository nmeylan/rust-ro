var _____WB$wombat$assign$function_____ = function(name) {return (self._wb_wombat && self._wb_wombat.local_init && self._wb_wombat.local_init(name)) || self[name]; };
if (!self.__WB_pmw) { self.__WB_pmw = function(obj) { this.__WB_source = obj; return this; } }

function isRangedWeapon() {
	return n_A_WeaponType == 10 || n_A_WeaponType == 17 || n_A_WeaponType == 18 || n_A_WeaponType == 19 || n_A_WeaponType == 20 || n_A_WeaponType == 21;
}

{
	let window = _____WB$wombat$assign$function_____("window");
	let self = _____WB$wombat$assign$function_____("self");
	let document = _____WB$wombat$assign$function_____("document");
	let location = _____WB$wombat$assign$function_____("location");
	let top = _____WB$wombat$assign$function_____("top");
	let parent = _____WB$wombat$assign$function_____("parent");
	let frames = _____WB$wombat$assign$function_____("frames");
	let opener = _____WB$wombat$assign$function_____("opener");

	hasLeftHand=0;
	isRebirth=0;
	n_SpSkill=0;
	n_Ses=0;
	n_Enekyori=0;
	w_AG=[100,95,90,86,82,79,76,74,72,71,70];
	n_SkillSW=0;
	n_Skill3SW=0;
	n_Skill4SW=0;
	n_Skill5SW=0;
	n_Skill6SW=0;
	n_Skill7SW=0;
	n_IjyouSW=0;
	wBCEDPch=0;
	wLAch=0;
	wCriTyuu=0;
	wBTw1=0;
	swDelay=0;
	TyouEnkakuSousa3dan = 0;
	not_use_card = 0;
	SuperNoviceFullWeaponCHECK = 0;
	cast_kotei = 0;
	b = 0;


	function myInnerHtml(wIH1,wIH2,wIH3)
	{
		if(wIH3 == 0){
			wIHOB = document.getElementById(wIH1);
			while(wIHOB.hasChildNodes()){
				wIHOB.removeChild(wIHOB.firstChild);
			}
			wIHOB.innerHTML = wIH2;
		}else{
			wIHOB = document.getElementById(wIH1);
			wIHOB.insertAdjacentHTML('BeforeEnd',wIH2);

		}
	}

	WeaponName = ["Unarmed","Dagger","Sword","Two-handed Sword","Spear","Two-handed Spear","Axe","Two-handed Axe","Mace","Rod","Bow","Katar","Book","Knuckle","Instrument","Whip","Huuma Shuriken","Handgun","Rifle","Shotgun","Gatling Gun","Grenade Launcher"];

	ArrowOBJ = [
		[25,0,"Arrow"],
		[30,6,"Silver Arrow"],
		[30,3,"Fire Arrow"],
		[30,0,"Iron Arrow"],
		[30,2,"Stone Arrow"],
		[30,1,"Crystal Arrow"],
		[30,4,"Arrow of Wind"],
		[30,7,"Arrow of Shadow"],
		[30,8,"Immaterial Arrow"],
		[30,5,"Rusty Arrow"],
		[40,0,"Steel Arrow"],
		[50,0,"Oridecon Arrow"],
		[50,6,"Arrow of Counter Evil"],
		[ 1,1,"Frozen Arrow"],
		[ 1,5,"Poison Arrow"],
		[10,0,"Sharp Arrow"],
		[50,6,"Holy Arrow"],
		[ 1,0,"Other (1 Atk)"]
	];

	ArrowOBJbackup = [
		[25,0,"Arrow"],
		[30,6,"Silver Arrow"],
		[30,3,"Fire Arrow"],
		[30,0,"Iron Arrow"],
		[30,2,"Stone Arrow"],
	];

	BulletOBJ = [
		[10,0,"Bullet"],
		[15,6,"Silver Bullet"],
		[30,0,"Blood Bullet"],
	];

	GrenadeOBJ = [
		[50,3,"Flare Sphere"],
		[50,1,"Freezing Sphere"],
		[50,4,"Lightning Sphere"],
		[50,7,"Blind Sphere"],
		[50,5,"Poison Sphere"],
	];

	SyurikenOBJ = [
		[10,0,"Shuriken"],
		[30,0,"Numbus Shuriken"],
		[45,0,"Flash Shuriken"],
		[70,0,"Sharp Leaf Shuriken"],
		[100,0,"Thorn Needle Shuriken"],
	];

	KunaiOBJ = [
		[30,3,"Heat Wave Kunai"],
		[30,1,"Icicle Kunai"],
		[30,4,"High Wind Kunai"],
		[30,2,"Black Earth Kunai"],
		[30,5,"Fell Poison Kunai"],
	];




	JobEquipItemOBJ = [
		[0,50,100,999],
		[0, 1, 51,101, 70, 71, 72, 74, 75,78,83,84,85,86,87,999],
		[0, 1, 52,102, 72, 74, 75,78, 80,83,84,85,999],
		[0, 1, 53,103, 71, 73, 74, 77,78,85,89,999],
		[0, 1, 54,104, 75, 76,83,89,999],
		[0, 1, 55,105, 71, 77,89,999],
		[0, 1, 56,106, 70, 71, 72, 73, 74, 75,78,83,84,85,86,999],
		[0, 1, 51, 61,107, 70, 71, 72, 74, 75,78,79,83,84,85,86,87,999],
		[0, 1, 52, 62,108, 72, 74, 75,78,79,81,83,84,85,999],
		[0, 1, 53, 63,109, 71, 73, 74, 77,78,79,81,85,89,999],
		[0, 1, 54, 64,110, 75, 76,79,80,83,88,89,999],
		[0, 1, 55, 65,111, 71, 77,79,89,999],
		[0, 1, 56, 66,112, 70, 71, 72, 73, 74, 75,78,79,83,84,85,86,999],
		[0, 1, 51, 61,113, 70, 71, 72, 74, 75,78,79,83,84,85,86,87,999],
		[0, 1, 52, 62,114, 72, 74, 75, 76,78,79,80,83,84,85,88,999],
		[0, 1, 53, 63,115, 71, 73, 74, 77,78,79,85,89,999],
		[0, 1, 54, 64,116, 74, 75, 76,79,83,89,999],
		[0, 1, 54, 64,117, 74, 75, 76,79,83,89,999],
		[0, 1, 55, 65,118, 71, 77,79,89,999],
		[0, 1, 56, 66,119, 70, 71, 72, 73, 74, 75,78,79,83,84,85,86,999],
		[0,50,120,999],
		[0, 1, 51, 61,107,121, 70, 71, 72, 74, 75,78,79,82,83,84,85,86,87,999],
		[0, 1, 52, 62,108,122, 72, 74, 75,78,79,81,82,83,84,85,999],
		[0, 1, 53, 63,109,123, 71, 73, 74, 77,78,79,81,82,85,89,999],
		[0, 1, 54, 64,110,124, 75, 76,79,82,83,88,89,999],
		[0, 1, 55, 65,111,125, 71, 77,79,82,89,999],
		[0, 1, 56, 66,112,126, 70, 71, 72, 73, 74, 75,78,79,82,83,84,85,86,999],
		[0, 1, 51, 61,113,127, 70, 71, 72, 74, 75,78,79,82,83,84,85,86,87,999],
		[0, 1, 52, 62,114,128, 72, 74, 75, 76,78,79,80,82,83,84,85,88,999],
		[0, 1, 53, 63,115,129, 71, 73, 74,77,78,79,82,85,89,999],
		[0, 1, 54, 64,116,130, 74, 75, 76,79,82,83,89,999],
		[0, 1, 54, 64,117,131, 74, 75, 76,79,82,83,89,999],
		[0, 1, 55, 65,118,132, 71,77,79,82,89,999],
		[0, 1, 56, 66,119,133, 70, 71, 72, 73, 74, 75,78,79,82,83,84,85,86,999],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0, 1,83,84,85,86,999],
		[0, 1,79,83,84,85,86,87,999],
		[0, 1, 55, 65,111, 71, 77,79,999],
		[0, 1, 58, 52,999],
		[0, 1, 59, 83,999],
	];

	SyuzokuOBJ = ["Formless","Undead","Brute","Plant","Insect","Fish","Demon","Demi-Human","Angel","Dragon"];
	elementOBJ = ["Neutral","Water","Earth","Fire","Wind","Poison","Holy","Dark","Ghost","Undead"];
	SizeOBJ = ["Small","Medium","Large"];
	IjyouOBJ = ["Poison","Stun","Freeze","Curse","Blind","Sleep","Silence","Chaos","Bleeding","Stone","Weapon Break","Armor Break"];

	SubName = ["%","sec","Damage","Critical Damage","Critical Rate","Over 1000 Hits","Too High to Calculate","Immesurable","X","Cast Time","Off","On"];

	function BattleCalc999()
	{
		skillModifier = 1;
		wCast = 0;

		hitCount = 1;
		n_Enekyori=0;
		wLAch=0;

		finalDamages = [0,0,0];
		not_use_card = 0;
		cast_kotei = 0;


		myInnerHtml("bSUBname","",0);
		myInnerHtml("bSUB","",0);
		myInnerHtml("bSUB2name","",0);
		myInnerHtml("bSUB2","",0);


		if(n_A_ActiveSkill != 0 && n_A_ActiveSkill !=272 && (n_A_ActiveSkill != 86 || (n_B[3] < 50 && 60 <= n_B[3]))){
			myInnerHtml("CRIATK","",0);
			myInnerHtml("CRInum","",0);
			myInnerHtml("CRIATKname","",0);
			myInnerHtml("CRInumname","",0);
		}


		if(n_A_WeaponType==10 && n_A_ActiveSkill==0)
			n_Enekyori=1;

		if((n_A_WeaponType==17||n_A_WeaponType==18||n_A_WeaponType==19||n_A_WeaponType==20||n_A_WeaponType==21)&& n_A_ActiveSkill==0)
			n_Enekyori=1;


		if(n_A_ActiveSkill==0 || (n_A_ActiveSkill==86 && (50 <= n_B[3] && n_B[3] < 60))){
			myInnerHtml("CRIATKname",SubName[3],0);
			myInnerHtml("CRInumname",SubName[4],0);

			if(n_A_ActiveSkill==86){
				n_SpSkill=1;
				if(n_A_WeaponType != 11)
					myInnerHtml("bSUB",'<Font size="2"><B>Damage Shown with 2x right hand damage.</B></Font>',0);
			}

			if(hasLeftHand){

				if(n_B[19] != 5){
					TyouEnkakuSousa3dan = 0;

					n_A_Weapon2 = eval(document.calcForm.A_weapon2.value);
					n_A_Weapon2LV = ItemOBJ[n_A_Weapon2][4];
					n_A_Weapon2_ATK = ItemOBJ[n_A_Weapon2][3];
					n_A_Weapon2_RefinementLevel = eval(document.calcForm.A_Weapon2_ATKplus.value);


					n_A_Weapon2LV_upgradeBonusATK = 0;
					n_A_Weapon2LV_Minplus = 0;
					n_A_Weapon2LV_overUpgradeBonusATK = 0;
					if(n_A_Weapon2LV == 1){
						n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 2;
						if(n_A_Weapon2_RefinementLevel >= 8){
							n_A_Weapon2LV_Minplus = 1;
							n_A_Weapon2LV_overUpgradeBonusATK = 3 * (n_A_Weapon2_RefinementLevel - 7);
						}
					}else if(n_A_Weapon2LV == 2){
						n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 3;
						if(n_A_Weapon2_RefinementLevel >= 7){
							n_A_Weapon2LV_Minplus = 1;
							n_A_Weapon2LV_overUpgradeBonusATK = 5 * (n_A_Weapon2_RefinementLevel - 6);
						}
					}else if(n_A_Weapon2LV == 3){
						n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 5;
						if(n_A_Weapon2_RefinementLevel >= 6){
							n_A_Weapon2LV_Minplus = 1;
							n_A_Weapon2LV_overUpgradeBonusATK = 8 * (n_A_Weapon2_RefinementLevel - 5);
						}
					}else if(n_A_Weapon2LV == 4){
						n_A_Weapon2LV_upgradeBonusATK = n_A_Weapon2_RefinementLevel * 7;
						if(n_A_Weapon2_RefinementLevel >= 5){
							n_A_Weapon2LV_Minplus = 1;
							n_A_Weapon2LV_overUpgradeBonusATK = 14 * (n_A_Weapon2_RefinementLevel - 4);
						}
					}

					n_A_workDEX = Math.floor(n_A_DEX * (1 + (n_A_Weapon2LV - 1) * 0.2));

					if(n_A_workDEX>=n_A_Weapon2_ATK)
						w_left_Maxatk = baseATK + n_A_Weapon2LV_overUpgradeBonusATK + Math.floor((n_A_Weapon2_ATK + impositioMagnus)* sizeModifier);
					else
						w_left_Maxatk = baseATK + n_A_Weapon2LV_overUpgradeBonusATK + Math.floor((n_A_Weapon2_ATK-1 + impositioMagnus)* sizeModifier);

					w_left_Maxatk = BattleCalc4(w_left_Maxatk * skillModifier,2,1);

					if(w_left_Maxatk<1)w_left_Maxatk=1;
					w_left_Maxatk = Math.floor(w_left_Maxatk * element[n_B[3]][n_A_Weapon2_element]);


					w_left_star = 0;
					if(n_A_card[4]==106 && n_A_card[5]==106 && n_A_card[6]==106){
						w_left_star += 40;
					}else{
						for(i=4;i<=6;i++){
							if(cardOBJ[n_A_card[i]][0]==106)
								w_left_star += 5;
						}
					}
					if(n_A_card[7]==106)
						w_left_star += 10;
					w_left_Maxatk += w_left_star;
					w_left_Maxatk = w_left_Maxatk * (3 + SkillSearch(80)) /10;
					w_left_Maxatk = Math.floor(w_left_Maxatk);


					if(n_A_workDEX > n_A_Weapon2_ATK)
						n_A_workDEX = n_A_Weapon2_ATK;
					w_left_Minatk = baseATK + n_A_Weapon2LV_Minplus + Math.floor((n_A_workDEX + impositioMagnus) * sizeModifier);
					w_left_Minatk = BattleCalc4(w_left_Minatk * skillModifier,0,1);

					if(w_left_Minatk<1)w_left_Minatk=1;
					w_left_Minatk = Math.floor(w_left_Minatk * element[n_B[3]][n_A_Weapon2_element]);
					w_left_Minatk  += w_left_star;
					w_left_Minatk *= (0.3 + SkillSearch(80) /10);
					w_left_Minatk = Math.floor(w_left_Minatk);

					w_left_Aveatk = (w_left_Maxatk + w_left_Minatk) /2;
				}
				else{
					w_left_Maxatk = 1;
					w_left_Minatk = 1;
					w_left_Aveatk = 1;
				}

				applySkillModifier(skillModifier,0);


				finalDamages[2] = BattleCalc(n_A_DMG[2],2);
				myInnerHtml("ATK_02",finalDamages[2] + n_A_EDP_DMG[2] +"("+ w_left_Maxatk +")",0);




				finalDamages[2] = BattleCalc3(finalDamages[2]);
				finalDamages[2] += BattleCalc3left(w_left_Maxatk);
				finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

				finalDamages[0] = BattleCalc(n_A_DMG[0],0);
				myInnerHtml("ATK_00",finalDamages[0] + n_A_EDP_DMG[0] +"("+ w_left_Minatk +")",0);

				finalDamages[0] = BattleCalc3(finalDamages[0]);
				finalDamages[0] += BattleCalc3left(w_left_Minatk);
				finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

				finalDamages[1] = BattleCalc(n_A_DMG[1],1);
				myInnerHtml("ATK_01",finalDamages[1] + n_A_EDP_DMG[1] +"("+ w_left_Aveatk +")",0);

				finalDamages[1] = BattleCalc3(finalDamages[1]);
				finalDamages[1] += BattleCalc3left(w_left_Aveatk);
				finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

				BattleCalc998(0,0);
			}

			else if(n_A_WeaponType == 11){
				applySkillModifier(skillModifier,0);
				finalDamages[2] = BattleCalc(n_A_DMG[2],2);
				wk = Math.floor(finalDamages[2] * (0.01 + SkillSearch(13) * 0.02));
				wk2 = Math.floor((finalDamages[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch(13) * 0.02));
				myInnerHtml("ATK_02",(finalDamages[2]  + wk2 + n_A_EDP_DMG[2]) +"("+ (finalDamages[2] + n_A_EDP_DMG[2]) +"+"+ wk2 +")",0);
				finalDamages[2] += wk;


				finalDamages[2] = BattleCalc3(finalDamages[2]);
				finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

				finalDamages[0] = BattleCalc(n_A_DMG[0],0);
				wk = Math.floor(finalDamages[0] * (0.01 + SkillSearch(13) * 0.02));
				wk2 = Math.floor((finalDamages[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch(13) * 0.02));
				myInnerHtml("ATK_00",(finalDamages[0] + wk2 + n_A_EDP_DMG[0]) +"("+ (finalDamages[0] + n_A_EDP_DMG[0]) +"+"+ wk2 +")",0);
				finalDamages[0] += wk;

				finalDamages[0] = BattleCalc3(finalDamages[0]);
				finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

				finalDamages[1] = BattleCalc(n_A_DMG[1],1);
				wk = Math.floor(finalDamages[1] * (0.01 + SkillSearch(13) * 0.02));
				wk2 = Math.floor((finalDamages[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch(13) * 0.02));
				myInnerHtml("ATK_01",(finalDamages[1] + wk2 + n_A_EDP_DMG[1]) +"("+ (finalDamages[1] + n_A_EDP_DMG[1]) +"+"+ wk2 +")",0);
				finalDamages[1] += wk;

				finalDamages[1] = BattleCalc3(finalDamages[1]);
				finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

				BattleCalc998(0,0);
			}
			else{
				wTAKA = BattleTAKA();
				TyouEnkakuSousa3dan = 0;

				if(SkillSearch(187)){
					TyouEnkakuSousa3dan = -1;
					wBC3_3danAtkBairitu = SkillSearch(187) * 0.2;

					myInnerHtml("bSUBname","Trifecta Damage",0);
					san1 = Math.floor(BattleCalc(n_A_DMG[0] * (skillModifier + wBC3_3danAtkBairitu),0) /3) *3;
					san2 = Math.floor(BattleCalc(n_A_DMG[1] * (skillModifier + wBC3_3danAtkBairitu),1) /3) *3;
					san3 = Math.floor(BattleCalc(n_A_DMG[2] * (skillModifier + wBC3_3danAtkBairitu),2) /3) *3;
					myInnerHtml("bSUB",san1+" ~ "+san3,0);
					myInnerHtml("bSUB2name","Trifecta Rate",0);
					myInnerHtml("bSUB2",30 - SkillSearch(187)+"%",0);
					TyouEnkakuSousa3dan = 0;
				}

				applySkillModifier(skillModifier,0);

				finalDamages[2] = BattleCalc(n_A_DMG[2],2);
				if(SkillSearch(187))
					TyouEnkakuSousa3dan = san3;
				myInnerHtml("ATK_02",(finalDamages[2] + n_A_EDP_DMG[2]),0);


				finalDamages[2] = BattleCalc3(finalDamages[2]);
				finalDamages[2] += wTAKA;
				finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

				finalDamages[0] = BattleCalc(n_A_DMG[0],0);
				myInnerHtml("ATK_00",finalDamages[0] + n_A_EDP_DMG[0],0);
				if(SkillSearch(187))
					TyouEnkakuSousa3dan = san1;

				finalDamages[0] = BattleCalc3(finalDamages[0]);
				finalDamages[0] += wTAKA;
				finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

				finalDamages[1] = BattleCalc(n_A_DMG[1],1);
				myInnerHtml("ATK_01",finalDamages[1] + n_A_EDP_DMG[1],0);
				if(SkillSearch(187))
					TyouEnkakuSousa3dan = san2;

				finalDamages[1] = BattleCalc3(finalDamages[1]);
				finalDamages[1] += wTAKA;
				finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

				CastAndDelay();
				BattleCalc998(wCast,wDelay);
			}
			return;
		}

		else if(n_A_ActiveSkill==272){
			n_Enekyori=1;
			myInnerHtml("CRIATKname","Defence Bypassing Damage",0);
			myInnerHtml("CRInumname","Chance to Bypass Defence",0);

			skillModifier += (1 + 0.5 * n_A_ActiveSkillLV);
			wCast = 2 * n_A_CAST;
			wDelay = 1.5;
			swDelay = 1;


			n_A_CriATK[1] = n_A_DMG[1];
			n_A_CriATK[0] = n_A_DMG[0];
			n_A_CriATK[2] = n_A_DMG[2];

			applySkillModifier(skillModifier,1);

			wCriTyuu=1;
			n_A_CriATK[1] = BattleCalc(n_A_CriATK[1],10);
			n_A_CriATK[0] = BattleCalc(n_A_CriATK[0],10);
			n_A_CriATK[2] = BattleCalc(n_A_CriATK[2],10);
			wCriTyuu=0;


			n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2],0);
			n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0],2);
			n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1],3);

			myInnerHtml("CRIATK",(n_A_CriATK[0] + n_A_EDP_DMG[0]) +" ~ "+ (n_A_CriATK[2] + n_A_EDP_DMG[2]),0);


			n_A_CriATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
			n_A_CriATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
			n_A_CriATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

			applySkillModifier(skillModifier,0);

			finalDamages[2] = BattleCalc(n_A_DMG[2],2);
			myInnerHtml("ATK_02",(finalDamages[2] + n_A_EDP_DMG[2]),0);


			finalDamages[2] = BattleCalc3(finalDamages[2]);
			finalDamages[2] += HitEDPplus(n_A_EDP_DMG[2]);

			finalDamages[0] = BattleCalc(n_A_DMG[0],0);
			myInnerHtml("ATK_00",finalDamages[0] + n_A_EDP_DMG[0],0);

			finalDamages[0] = BattleCalc3(finalDamages[0]);
			finalDamages[0] += HitEDPplus(n_A_EDP_DMG[0]);

			finalDamages[1] = BattleCalc(n_A_DMG[1],1);
			myInnerHtml("ATK_01",finalDamages[1] + n_A_EDP_DMG[1],0);

			finalDamages[1] = BattleCalc3(finalDamages[1]);
			finalDamages[1] += HitEDPplus(n_A_EDP_DMG[1]);

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			return;
		}

		w_ActS=[6,7,19,41,44,65,71,72,73,83,84,158,161,169,171,188,189,199,207,248,260,261,264,288,289,290,292,302,303,326,331,333,335,337,339,382,388,348,349,350,419,423,428,429,430,431,432,434,435,436,437,"NULL"];
		for(iw=0;w_ActS[iw] != n_A_ActiveSkill && w_ActS[iw] != "NULL";iw++);
		if(n_A_ActiveSkill==w_ActS[iw]){
			wActiveHitNum = 1;
			if(n_A_ActiveSkill==6)
				skillModifier += n_A_ActiveSkillLV *0.3;
			else if(n_A_ActiveSkill==348 || n_A_ActiveSkill==349 || n_A_ActiveSkill==350)
			{
				//Heat

				wDelay= 0.1;

			}
			else if(n_A_ActiveSkill==7){
				skillModifier += n_A_ActiveSkillLV *0.2;
				n_A_Weapon_element = 3;
				wDelay = 2;
				swDelay = 1;
			}else if(n_A_ActiveSkill==19){
				not_use_card = 1;
				skillModifier += 0.3;
				n_A_Weapon_element = 2;
			}else if(n_A_ActiveSkill==41){
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV *0.05 - 0.25;
				wDelay = 1;
				swDelay = 1;
			}
			else if(n_A_ActiveSkill==44){
				n_Enekyori=1;
				wCast = 1.5;
				skillModifier += 0.5;
			}else if(n_A_ActiveSkill==65)
				skillModifier += n_A_ActiveSkillLV *0.5;
			else if(n_A_ActiveSkill==71){
				skillModifier += n_A_ActiveSkillLV *0.2;
				n_Enekyori=1;
			}else if(n_A_ActiveSkill==84){
				if(n_A_ActiveSkillLV >= 3)
					n_Enekyori=1;
				skillModifier += 0.2 * n_A_ActiveSkillLV;




			}else if(n_A_ActiveSkill==158){
				skillModifier += n_A_ActiveSkillLV *0.2;

			}
			else if(n_A_ActiveSkill==161){
				skillModifier += n_A_ActiveSkillLV *0.35;
				n_A_Weapon_element = 6;
			}
			else if(n_A_ActiveSkill==171)
				skillModifier += n_A_ActiveSkillLV *0.4;
			else if(n_A_ActiveSkill==72){
				skillModifier += n_A_ActiveSkillLV *0.5;
				wDelay = 1;
				swDelay = 1;
				n_Enekyori=1;
			}else if(n_A_ActiveSkill==73){
				w = (1+n_A_ActiveSkillLV*0.2);
				if(n_A_ActiveSkillLV == 10)skillModifier += 4.625;
				else if(n_A_ActiveSkillLV >= 7)skillModifier += w+w/2+w/4-1;
				else if(n_A_ActiveSkillLV >= 4)skillModifier += w+w/2-1;
				else skillModifier += w-1;
				wCast = 0.7;
			}else if(n_A_ActiveSkill==83 || n_A_ActiveSkill==388){
				wActiveHitNum = 8;
				skillModifier += n_A_ActiveSkillLV *0.5 + 2;
				if(n_A_ActiveSkill==388 && Taijin==0)
					skillModifier *= 2;
				if(n_A_ActiveSkill==388 && Taijin==1)
					skillModifier *= 1.25;
				wDelay = 2;
				swDelay = 2;
			}else if(n_A_ActiveSkill==169){
				skillModifier += n_A_ActiveSkillLV *0.4 + 2;
				wDelay = 0.5;
				swDelay = 1;
				w_HIT = 100;
				myInnerHtml("BattleHIT",100,0);
			}else if(n_A_ActiveSkill==188){
				wActiveHitNum = 4;
				skillModifier += 0.5+n_A_ActiveSkillLV *0.5;
				n_SpSkill=1;
			}else if(n_A_ActiveSkill==189){
				skillModifier += 1.4+n_A_ActiveSkillLV *0.6;
				n_SpSkill=1;
			}else if(n_A_ActiveSkill==199||n_A_ActiveSkill==207){
				wCast = 1.5;
				skillModifier += (n_A_ActiveSkillLV * 0.4 - 0.4);
				n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
				if(eval(document.calcForm.A_Weapon_element.value) != 0)
					n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
				n_Enekyori=1;
			}else if(n_A_ActiveSkill==248){
				not_use_card = 1;
				n_A_Weapon_element = 3;
				n_SpSkill=1;
				wCast = 1;
				skillModifier += n_A_ActiveSkillLV *0.2;
				w_HIT = 100;
				myInnerHtml("BattleHIT",100,0);
			}else if(n_A_ActiveSkill==260){
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV *0.4;
				wDelay = 0.5;
				swDelay = 1;
			}else if(n_A_ActiveSkill==261){
				n_Enekyori=1;
				skillModifier += (n_A_ActiveSkillLV *0.1 -0.5);
				if(n_A_ActiveSkillLV > 5)
					wDelay = 1;
				else
					wDelay = 0.8;
				swDelay = 1;
			}else if(n_A_ActiveSkill==264){
				not_use_card = 1;
				skillModifier += (n_A_ActiveSkillLV *0.4 -0.6);
				wCast = 0.5;
				wDelay = 0.5;
				swDelay = 1;
			}else if(n_A_ActiveSkill==288){
				skillModifier += (1 + n_A_ActiveSkillLV);
				wDelay = 0.3;
				swDelay = 1;
			}else if(n_A_ActiveSkill==289){
				n_SpSkill=1;
				skillModifier += n_A_ActiveSkillLV -0.6;


			}else if(n_A_ActiveSkill==290){
				n_SpSkill=1;
				skillModifier += (3 + n_A_ActiveSkillLV);
				if(n_A_ActiveSkillLV>6) wDelay=1;
				else wDelay=0.8;
				swDelay = 1;
			}else if(n_A_ActiveSkill==292){
				wActiveHitNum = 9;
				skillModifier += 1 + n_A_ActiveSkillLV;
				n_A_Weapon_element = ArrowOBJ[n_A_Arrow][1];
				if(eval(document.calcForm.A_Weapon_element.value) != 0)
					n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
				n_Enekyori=1;
				wCast = 1.8 + n_A_ActiveSkillLV *0.2;
				if(n_A_ActiveSkillLV>=6) wDelay=1;
				else wDelay=0.8;
				wDelay=3;
				swDelay = 2;
			}else if(n_A_ActiveSkill==302){
				n_Enekyori=1;
				not_use_card = 1;
				n_A_Weapon_element = 4;
			}else if(n_A_ActiveSkill==303){
				skillModifier += (n_A_ActiveSkillLV -1) * 1;
			}else if(n_A_ActiveSkill==326){
				not_use_card = 1;
				skillModifier += Math.floor((eval(document.calcForm.SkillSubNum.value) / (16 - n_A_ActiveSkillLV) / 100 -1) * 100) /100;
			}else if(n_A_ActiveSkill==382){
				not_use_card = 1;
				skillModifier += 2;


			}else if(n_A_ActiveSkill==331 || n_A_ActiveSkill==333){
				n_SpSkill=1;
				skillModifier += (0.6 + n_A_ActiveSkillLV * 0.2);
			}else if(n_A_ActiveSkill==335 || n_A_ActiveSkill==337){
				n_SpSkill=1;
				skillModifier += (0.9 + n_A_ActiveSkillLV * 0.3);
				if(n_A_ActiveSkill==337)
					wActiveHitNum = 3;
			}else if(n_A_ActiveSkill==339){
				n_SpSkill=1;
				skillModifier += (-0.7 + n_A_ActiveSkillLV * 0.1);
			}else if(n_A_ActiveSkill==419){
				not_use_card = 1;
				wCast = 0.5;
				n_Enekyori=1;
				wActiveHitNum = 5;
				if(n_B[2] == 2 || n_B[2] == 7)
					skillModifier += 4;
			}else if(n_A_ActiveSkill==423){
				n_Enekyori=1;
				n_A_Weapon_element = 8;
				not_use_card = 1;
			}else if(n_A_ActiveSkill==428){
				n_Enekyori=1;
				wActiveHitNum = 5;
				skillModifier += n_A_ActiveSkillLV *0.5 + 4;
				wDelay = 1.7;
				swDelay = 1;
			}else if(n_A_ActiveSkill==429){
				n_Enekyori=0;
				skillModifier += n_A_ActiveSkillLV *0.5 - 0.5;
				wDelay = 1;
				swDelay = 1;
			}else if(n_A_ActiveSkill==430){
				wCast = 1 + 0.2 * n_A_ActiveSkillLV;
				cast_kotei = 1;
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV *1 +1;
				wDelay = 1;
				swDelay = 1;
				w_HIT = w_HIT * 5 +5;
				if(w_HIT > 100)
					w_HIT = 100;
			}else if(n_A_ActiveSkill==431){
				wCast = 2;
				n_Enekyori=1;
			}else if(n_A_ActiveSkill==432){
				wCast = 1.5;
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV *0.2;
				wDelay = 0;
				swDelay = 1;
				w_HIT = 100;
			}else if(n_A_ActiveSkill==434){
				cast_kotei = 1;
				wCast = 1;
				n_Enekyori=0;
				skillModifier += n_A_ActiveSkillLV *0.5;
				wDelay = 1;
				swDelay = 2
				w_HIT = 100;
			}else if(n_A_ActiveSkill==435){
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV * 1 + 2;
				wDelay = 1 + n_A_ActiveSkillLV *0.2;
				swDelay = 1;
			}else if(n_A_ActiveSkill==436){
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV * 0.2 - 0.2;
				wDelay = "(Unknown)";
				swDelay = 1;
			}else if(n_A_ActiveSkill==437){
				n_Enekyori=1;
				not_use_card = 1;
				wCast = 1;
				wDelay = 1;
				swDelay = 1;
				w_HIT = 100;
			}


			applySkillModifier(skillModifier,0);


			n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0],0);
			n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1],1);
			n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2],2);

			if(cast_kotei == 0)
				wCast = wCast * n_A_CAST;

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				if(wActiveHitNum > 1)
					finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
				myInnerHtml("ATK_0"+b,finalDamages[b] + n_A_EDP_DMG[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT + BattleCalc2(0) *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			if(cast_kotei == 0)
				CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}else if(n_A_ActiveSkill==275){
			n_Enekyori=1;
			wCast = 0.3;
			wDelay= 0.3;
			swDelay = 1;
			n_A_DMG[2] = n_A_MATK[2];
			n_A_DMG[0] = n_A_MATK[0];
			n_A_DMG[1] = (n_A_MATK[0] + n_A_MATK[2])/2;


			for(b=0;b<=2;b++)
				n_A_EDP_DMG[b] = BattleCalcEDP(n_A_DMG[b],b);

			wCast = wCast * n_A_CAST;

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				myInnerHtml("ATK_0"+b,finalDamages[b] + n_A_EDP_DMG[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT + (BattleCalc2(0)+n_A_WeaponLV_upgradeBonusATK) *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==40||n_A_ActiveSkill==70||n_A_ActiveSkill==192||n_A_ActiveSkill==76||n_A_ActiveSkill==418||n_A_ActiveSkill==391){
			if(n_A_ActiveSkill==40){ // double strafe
				n_Enekyori=1;
				skillModifier += n_A_ActiveSkillLV *0.1 -0.1;
				hitCount = 2;
			}else if(n_A_ActiveSkill==70){
				skillModifier += n_A_ActiveSkillLV *0.1;
				hitCount = n_B[4]+1;
			}else if(n_A_ActiveSkill==76){
				skillModifier += n_A_ActiveSkillLV *0.4;
				wCast = 0.7 * n_A_CAST;
				hitCount = 2;
				if(n_A_ActiveSkillLV == 1)
					hitCount = 1;
				wLAch=1;
				if(n_B_IJYOU[6] == 1){
					hitCount = 3;
					if(n_A_ActiveSkillLV == 1)
						hitCount = 2;
				}
			}else if(n_A_ActiveSkill==192){
				skillModifier += n_A_ActiveSkillLV *0.5;
				if(n_A_JOB==15||n_A_JOB==29)
					w = SkillSearch(185);
				else
					w = n_A_PassSkill2[10];
				if(w > n_A_ActiveSkillLV){
					w = n_A_ActiveSkillLV;
				}
				hitCount = w;
				wCast = (1 + w) * n_A_CAST;
				wDelay = 0.5;
				swDelay = 1;
				n_Enekyori=1;
			}else if(n_A_ActiveSkill==418){
				n_Enekyori=1;
				skillModifier += 0.5;
				hitCount = 3;
			}else if(n_A_ActiveSkill==391){
				n_SpSkill=1;
				n_Enekyori=1;
				skillModifier += n_A_STR *0.08 - 0.5;
				hitCount = 2;
			}


			applySkillModifier(skillModifier,0);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] += n_A_EDP_DMG[b];
				if(n_A_ActiveSkill==391 && n_B[2]!=2 && n_B[2]!=4)
					finalDamages[b] = 0;
				if(n_B_IJYOU[6] == 0 || wLAch==0)
					myInnerHtml("ATK_0"+b,finalDamages[b] * hitCount + "("+ finalDamages[b] + SubName[8] +hitCount+"hit)",0);
				else
					myInnerHtml("ATK_0"+b,finalDamages[b] * 3 + "("+finalDamages[b] *2 +" + "+ finalDamages[b] +")",0);
				finalDamages[b] -= n_A_EDP_DMG[b];
				finalDamages[b] *= hitCount;
				finalDamages[b] = (finalDamages[b] * w_HIT + BattleCalc2(0) * hitCount *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]) * hitCount;
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==118 || n_A_ActiveSkill==271){
			n_A_Weapon_element = 0;
			n_Enekyori=1;
			wBT = 80 + Math.floor(n_A_DEX /10)*2 + Math.floor(n_A_INT/2)*2 + SkillSearch(119) *6;
			if(n_A_ActiveSkill==271){
				wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) /100);
				wBT = Math.floor(wBT * element[n_B[3]][0]);
				wBT = tPlusDamCut(wBT);
				wBT *= 5;
				wCast = 1 * n_A_CAST;
				wDelay = 3;
			}else{
				wBT = Math.floor(wBT * element[n_B[3]][0]);
				wBT = tPlusDamCut(wBT);
				wBT *= n_A_ActiveSkillLV;
				wCast = 1.5 * n_A_CAST;
				wDelay = 1;
			}
			swDelay = 1;
			myInnerHtml("ATK_02",wBT,0);
			myInnerHtml("ATK_00",wBT,0);
			myInnerHtml("ATK_01",wBT,0);
			finalDamages[0]=finalDamages[2]=finalDamages[1]=wBT;
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==17 || (n_A_ActiveSkill==86 && (n_B[3] < 50 ||  60 <= n_B[3]))){
			applySkillModifier(skillModifier,0);
			n_A_Weapon_element = 5;


			n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2],2);
			n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0],0);
			n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1],1);

			wINV = Math.floor(BattleCalc2(0) * element[n_B[3]][5]);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][5]);
				myInnerHtml("ATK_0"+b,finalDamages[b] + n_A_EDP_DMG[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT + wINV *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			myInnerHtml("bSUBname",'<Font color="#0000FF">Poison Damage</Font>',0);
			myInnerHtml("bSUB",'<Font color="#0000FF">'+ wINV +"</Font>",0);

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==159 || n_A_ActiveSkill==384)
		{
			n_Enekyori=1;
			n_A_Weapon_element = 0;
			wDelay = 0.7;
			if(n_A_ActiveSkill==384)
				wDelay = 0.35;
			swDelay = 1;
			wSBr = n_A_LEFT_DEF_PLUS *4;

			skillModifier2 = (1 + n_A_ActiveSkillLV *0.3);
			if(n_A_ActiveSkill==384)
				skillModifier2 *= 2;

			baseATK_w = Math.round(Math.floor(n_A_STR/10) * Math.floor(n_A_STR/10));
			baseATK   = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);

			for(b=0;b<=2;b++){
				finalDamages[b] = baseATK * skillModifier + ItemOBJ[n_A_Equip[5]][6] + wSBr;
				finalDamages[b] = Math.floor(Math.floor(finalDamages[b] * (100 - n_B[14]) /100 - n_B_DEF2[b]) * skillModifier2);
				finalDamages[b] = BaiCI(finalDamages[b]);
				if(finalDamages[b] < 1)finalDamages[b] = 1;
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT)/100;
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==324)
		{
			n_Enekyori=1;
			n_A_Weapon_element = 0;
			wCast = 1 * n_A_CAST;
			wDelay = 1;
			swDelay = 1;
			wSBr = n_A_LEFT_DEF_PLUS;
			wSC  = ItemOBJ[n_A_Equip[5]][6];

			skillModifier2 = (1 + n_A_ActiveSkillLV *0.3);

			baseATK_w = Math.round(Math.floor(n_A_STR/10) * Math.floor(n_A_STR/10));
			baseATK   = n_A_STR + baseATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
			baseATK   = baseATK * skillModifier + wSC + wSBr * 4;

			wSC -= 100;
			if(wSC < 0)
				wSC = 0;
			wSC2 = [0,0,0];
			wSC2[2] = 100 + wSC + (wSBr * 2) * wSBr;
			wSC2[1] = 100 + (wSC + (wSBr * 2) * wSBr)/2;
			wSC2[0] = 100

			for(b=0;b<=2;b++){
				finalDamages[b] = (baseATK * (100 - n_B[14]) /100 - n_B_DEF2[b]) * skillModifier2;
				finalDamages[b] += wSC2[b];
				finalDamages[b] = BaiCI(finalDamages[b]);
				if(finalDamages[b] < 1)finalDamages[b] = 1;
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b] * 5 +"("+finalDamages[b]+ SubName[8] +"5hit)",0);
				finalDamages[b] *= 5;
				finalDamages[b] = (finalDamages[b] * w_HIT)/100;
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==259)
		{
			n_Enekyori=1;
			if(n_A_ActiveSkillLV == 5)
				wCast = 1 * n_A_CAST;
			else
				wCast = (0.1 + 0.2 * n_A_ActiveSkillLV) * n_A_CAST;
			wDelay = 1+ 0.2 * n_A_ActiveSkillLV;
			swDelay = 1;

			wSPP = Math.floor(n_A_STR / 10);
			finalDamages[2] = wSPP * wSPP + ItemOBJ[n_A_Equip[0]][6] * 0.8 * (1 + 0.5 * n_A_ActiveSkillLV);
			wSPP = 1.25 -(n_B[4] * 0.25);
			finalDamages[2] = Math.floor(finalDamages[2] * wSPP + n_A_WeaponLV_upgradeBonusATK);
			finalDamages[2] = finalDamages[2] * element[n_B[3]][n_A_Weapon_element];
			finalDamages[2] = BaiCI(finalDamages[2]);
			myInnerHtml("ATK_00",finalDamages[2] * 5 + "("+finalDamages[2]+ SubName[8] +5+"hit)",0);
			myInnerHtml("ATK_01",finalDamages[2] * 5 + "("+finalDamages[2]+ SubName[8] +5+"hit)",0);
			myInnerHtml("ATK_02",finalDamages[2] * 5 + "("+finalDamages[2]+ SubName[8] +5+"hit)",0);
			finalDamages[2] *= 5;
			wSPP2 = n_A_WeaponLV_upgradeBonusATK * element[n_B[3]][n_A_Weapon_element];
			wSPP2 = BaiCI(wSPP2);
			wSPP2 = tPlusDamCut(wSPP2);
			finalDamages[2] = finalDamages[2] * w_HIT /100 + wSPP2 * 5 * (100- w_HIT)/100;


			finalDamages[0] = finalDamages[1] = finalDamages[2];

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==88)
		{
			not_use_card = 1;
			n_SpSkill=1;
			wCast = 1 * n_A_CAST;

			if(n_B[19] == 0){

				skillModifier += (400 + 50 * n_A_ActiveSkillLV + 20 * eval(document.calcForm.SkillSubNum.value)) /100;
				applySkillModifier(skillModifier,0);


				for(b=0;b<=2;b++){
					finalDamages[b] = BattleCalc(n_A_DMG[b],b);
					finalDamages[b] = Math.floor(finalDamages[b]);
					myInnerHtml("ATK_0"+b,finalDamages[b],0);
				}
			}
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==263)
		{
			not_use_card = 1;
			n_Enekyori=1;
			wCast = 0.5 * n_A_CAST;
			wDelay= 0.8 + 0.2 * n_A_ActiveSkillLV;
			swDelay = 1;

			w_SBr = new Array();
			w = n_A_INT * 5 * n_A_ActiveSkillLV;
			w_SBr[2] = w + 1000 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[2])/2);
			w_SBr[1] = w + 750 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[1])/2);
			w_SBr[0] = w + 500 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[0])/2);
			for(i=0;i<=2;i++)
				w_SBr[i] = tPlusDamCut(w_SBr[i]);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] *= n_A_ActiveSkillLV;
				myInnerHtml("ATK_0"+b,finalDamages[b] + w_SBr[b] +"("+ finalDamages[b] +"+"+ w_SBr[b] +")",0);
				finalDamages[b] = ((finalDamages[b] + w_SBr[b]) * w_HIT + (BattleCalc2(0) + w_SBr[b]) *(100-w_HIT))/100;
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}


		else if(n_A_ActiveSkill==162)
		{

			myInnerHtml("CRIATKname",'<Font color="#FF0000">HP Casting Cost</Font>',0);
			myInnerHtml("CRIATK",'<Font color="#FF0000">'+ Math.floor(n_A_MaxHP /5) +"</Font>",0);

			myInnerHtml("CRInumname",'<Font color="#FF0000">Reflect Damage</Font>',0);


			wGXhito = 100 - StPlusCard(57);
			wGXhito -= StPlusCalc2(57);

			wGXsei =  100 - SkillSearch(156) * 5;
			wGXsei -= StPlusCard(66);
			wGXsei -= StPlusCalc2(66);

			wGXen = StPlusCard(78);
			wGXen += StPlusCalc2(78);


			work_A_VITDEF = [0,0,0];
			work_A_VITDEF[0] = n_A_VITDEF[2];
			work_A_VITDEF[1] = n_A_VITDEF[1];
			work_A_VITDEF[2] = n_A_VITDEF[0];
			n_A_INTMDEF = n_A_INT + Math.floor(n_A_VIT /2);

			for(b=0;b<=2;b++){
				finalDamages[b] = BK_n_A_DMG[b] * (100 - n_A_DEF) /100 - work_A_VITDEF[b] + n_A_WeaponLV_upgradeBonusATK;
				finalDamages[b] = Math.floor(finalDamages[b] * (skillModifier + n_A_ActiveSkillLV * 0.4));

				w = n_A_MATK[b] *(100 - n_A_MDEF)/100 - n_A_INTMDEF;
				w = Math.floor(w * (n_A_ActiveSkillLV * 0.4 +1));

				finalDamages[b] += w;
				finalDamages[b] = Math.floor(finalDamages[b] * wGXhito /100);
				finalDamages[b] = Math.floor(finalDamages[b] * wGXsei /100);
				finalDamages[b] = Math.floor(finalDamages[b] * (100-wGXen) /100);

				if(CardNumSearch(135))
					finalDamages[b] = Math.floor(finalDamages[b] * 125 /100);

				if(CardNumSearch(75))
					finalDamages[b] = Math.floor(finalDamages[b] * 150 /100);

				if(CardNumSearch(134) || CardNumSearch(456))
					finalDamages[b] = Math.floor(finalDamages[b] * 0 /100);
				finalDamages[b] = Math.floor(finalDamages[b] /2);
			}
			myInnerHtml("CRInum",'<Font color="#FF0000">'+ finalDamages[0] +"�~3hit ~ "+ finalDamages[1] +"�~3hit</Font>",0);


			n_Enekyori=2;
			n_A_Weapon_element = 6;
			wCast = 3 * n_A_CAST;
			wDelay=1.5;
			swDelay = 1;
			wLAch=1;

			for(b=0;b<=2;b++){
				finalDamages[b] = BK_n_A_DMG[b] * (100 - n_B[14]) /100 - n_B_DEF2[b] + n_A_WeaponLV_upgradeBonusATK;
				finalDamages[b] *= skillModifier + n_A_ActiveSkillLV * 0.4;
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][6]);
				w = n_A_MATK[b] *(100 - n_B[15])/100 -n_B_MDEF2;
				w *= (n_A_ActiveSkillLV * 0.4 +1);
				w = Math.floor(w * element[n_B[3]][6]);
				finalDamages[b] = tPlusDamCut(Math.floor((w+finalDamages[b])*element[n_B[3]][6]));
				if(finalDamages[b] < 1)finalDamages[b]=1;
				if(60<=n_B[3]&&n_B[3]<=69)finalDamages[b]=0;
			}



			if(n_B_IJYOU[6] == 0){
				for(b=0;b<=2;b++){
					myInnerHtml("ATK_0"+b,finalDamages[b] * 3 + "("+finalDamages[b]+ SubName[8] +"3hit)",0);
					finalDamages[b] *= 3;
				}
			}else{
				for(b=0;b<=2;b++){
					myInnerHtml("ATK_0"+b,finalDamages[b] * 4 + "("+ finalDamages[b] *2 +" + " +finalDamages[b]+ SubName[8] +"2hit)",0);
					finalDamages[b] *= 4;
				}
			}
			CastAndDelay();
			BattleCalc998(wCast,wDelay);

			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==66)
		{
			wCR = 100;

			if(SkillSearch(327)){
				wCR += 20 * SkillSearch(327);
			}
			else{
				if(SkillSearch(154))
					wCR += SkillSearch(154) * 5;
				if(SkillSearch(154)==0 && n_A_PassSkill2[8])
					wCR += n_A_PassSkill2[8] * 5 / 10;
			}
			CR_n_A_DMG = [0,0,0];

			CRbai = eval(document.calcForm.SkillSubNum.value) / 8000;
			for(b=0;b<=2;b++)
				CR_n_A_DMG[b] = Math.floor(n_A_DMG[b] * wCR / 100);

			skillModifier += 0.5;
			applySkillModifier(skillModifier,0);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] += Math.floor(BattleCalc(CR_n_A_DMG[b],b) * CRbai);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b],0);

				finalDamages[b] = (finalDamages[b] * w_HIT + BattleCalc2(0) * 2 *(100-w_HIT))/100;
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
			}

			CastAndDelay();
			BattleCalc998(0,0);
		}

		else if(n_A_ActiveSkill==283)
		{
			finalDamages[2] = 500 + 300 * n_A_ActiveSkillLV;
			if(n_Ses)
				finalDamages[2] = Math.floor(finalDamages[2] * 0.6);
			finalDamages[0] = finalDamages[2];
			finalDamages[1] = finalDamages[2];
			myInnerHtml("ATK_02",finalDamages[2],0);
			myInnerHtml("ATK_00",finalDamages[0],0);
			myInnerHtml("ATK_01",finalDamages[1],0);

			wCast = (1.5+ 0.5 * n_A_ActiveSkillLV) * n_A_CAST;
			wDelay=1.5 + n_A_ActiveSkillLV *0.5;
			swDelay = 1;
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==284)
		{
			n_A_Weapon_element = 0;
			finalDamages[2] = Math.floor(n_A_MaxHP * 0.09 * (0.9 + 0.1 * n_A_ActiveSkillLV));
			finalDamages[2] = BaiCI(finalDamages[2]);
			finalDamages[2] = Math.floor(finalDamages[2] * element[n_B[3]][0]);
			myInnerHtml("ATK_02",finalDamages[2],0);
			myInnerHtml("ATK_00",finalDamages[2],0);
			myInnerHtml("ATK_01",finalDamages[2],0);
			finalDamages[0] = finalDamages[2];
			finalDamages[1] = finalDamages[2];

			CastAndDelay();
			BattleCalc998(wCast,wDelay);

			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==193)
		{
			n_A_Weapon_element = 0;
			applySkillModifier(skillModifier,0);
			skillModifier += n_A_ActiveSkillLV *0.75;


			work_B_DEF2 = [0,0,0];
			work_B_DEF2[0] = n_B_DEF2[2];
			work_B_DEF2[1] = n_B_DEF2[1];
			work_B_DEF2[2] = n_B_DEF2[0];

			for(b=0;b<=2;b++){
				finalDamages[b] = Math.floor(Math.floor(BK_n_A_DMG[b] * skillModifier) * (work_B_DEF2[b]+n_B[14]) /50);
				finalDamages[b] = BaiCI(finalDamages[b]);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b],0);
			}

			wCast = 1 * n_A_CAST;
			wDelay=0.5;
			swDelay = 1;
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==197 || n_A_ActiveSkill==321)
		{
			n_A_Weapon_element = 0;
			applySkillModifier(skillModifier,0);
			if(n_A_ActiveSkill==197)
				skillModifier += 7 + eval(document.calcForm.SkillSubNum.value) /10;
			else
				skillModifier += 7 + (n_A_MaxSP-1) /10;
			wASYU = 250 + n_A_ActiveSkillLV * 150;

			for(b=0;b<=2;b++){
				finalDamages[b] = Math.floor(BK_n_A_DMG[b] * skillModifier) + wASYU;
				finalDamages[b] = BaiCI(finalDamages[b]);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b],0);
			}

			wCast = (4.5 - 0.5 * n_A_ActiveSkillLV) * n_A_CAST;
			wDelay= 3.5 - 0.5 * n_A_ActiveSkillLV;
			swDelay = 1;
			CastAndDelay();

			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==394){
			n_Enekyori=1;
			not_use_card = 1;
			applySkillModifier(skillModifier,0);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b] + n_A_EDP_DMG[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT + BattleCalc2(0) * element[n_B[3]][0] *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==395){
			n_Enekyori=1;
			not_use_card = 1;
			applySkillModifier(skillModifier,0);


			if(eval(document.calcForm.A_Weapon_element.value) == 0 && n_A_WeaponType != 0 && StPlusCard(20) == 0)
				n_A_Weapon_element = KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][1];

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				myInnerHtml("ATK_0"+b,finalDamages[b] * 3 + "("+ finalDamages[b] + SubName[8] +"3hit)",0);
				finalDamages[b] = (finalDamages[b] * 3 * w_HIT + BattleCalc2(0) * element[n_B[3]][0] *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==396){
			skillModifier += (n_A_ActiveSkillLV * 1.5 +0.5);
			n_Enekyori=1;
			applySkillModifier(skillModifier,0);
			wCast = 3 * n_A_CAST;
			wDelay=3;
			swDelay = 1;
			wActiveHitNum = 2 + Math.round(n_A_ActiveSkillLV / 2);

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleCalc(n_A_DMG[b],b);
				finalDamages[b] = Math.floor(finalDamages[b] * element[n_B[3]][0]);
				if(wActiveHitNum > 1)
					finalDamages[b] = Math.floor(finalDamages[b] / wActiveHitNum) * wActiveHitNum;
				myInnerHtml("ATK_0"+b,finalDamages[b] + n_A_EDP_DMG[b],0);
				finalDamages[b] = (finalDamages[b] * w_HIT + BattleCalc2(0)* element[n_B[3]][0] *(100-w_HIT))/100;
				finalDamages[b] += HitEDPplus(n_A_EDP_DMG[b]);
			}

			CastAndDelay();
			BattleCalc998(wCast,wDelay);
		}

		else if(n_A_ActiveSkill==405 || n_A_ActiveSkill==438)
		{
			n_A_Weapon_element = 0;
			n_Enekyori=1;
			applySkillModifier(skillModifier,0);
			if(n_A_ActiveSkill==405)
				w_1senHP = eval(document.calcForm.SkillSubNum.value);
			else
				w_1senHP = n_A_MaxHP -1;

			finalDamages[0] = (n_A_STR + n_A_ActiveSkillLV) * 40 + w_1senHP * (n_A_BaseLV / 100) * n_A_ActiveSkillLV / 10;
			finalDamages[0] = finalDamages[0] * (100 - n_B[14]) / 100;
			finalDamages[0] = BaiCI(finalDamages[0]);
			finalDamages[0] = Math.floor(finalDamages[0] * element[n_B[3]][0]);

			finalDamages[2] = finalDamages[1] = finalDamages[0];
			for(b=0;b<=2;b++)
				myInnerHtml("ATK_0"+b,finalDamages[b],0);

			CastAndDelay();

			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==244)
		{
			n_Enekyori=1;
			n_A_Weapon_element = 0;
			skillModifier = (50 + n_A_ActiveSkillLV * 50) /100;

			for(b=0;b<=2;b++){
				finalDamages[b] = Math.floor((BK_n_A_DMG[b] - n_B_DEF2[b]) * skillModifier);
				finalDamages[b] = tPlusDamCut(Math.floor(finalDamages[b] * element[n_B[3]][0]));
				myInnerHtml("ATK_0"+b,finalDamages[b],0);
			}

			wCast = 1 * n_A_CAST;
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==328)
		{
			n_Enekyori=1;
			n_A_Weapon_element = 0;
			hitCount = n_A_ActiveSkillLV;

			wAD = 0.7 * n_A_INT * n_A_INT * n_B[7] / (n_A_INT + n_B[7]);
			finalDamages[2] = Math.floor(wAD);
			finalDamages[2] = tPlusDamCut(Math.floor(finalDamages[2] * element[n_B[3]][0]));
			if(Taijin==1)
				finalDamages[2] = Math.floor(finalDamages[2] /2);
			myInnerHtml("ATK_02",finalDamages[2] * hitCount + "("+ finalDamages[2] + SubName[8] +hitCount+"hit)",0);
			myInnerHtml("ATK_00",finalDamages[2] * hitCount + "("+ finalDamages[2] + SubName[8] +hitCount+"hit)",0);
			myInnerHtml("ATK_01",finalDamages[2] * hitCount + "("+ finalDamages[2] + SubName[8] +hitCount+"hit)",0);
			finalDamages[2] *= hitCount;
			finalDamages[0] = finalDamages[2];
			finalDamages[1] = finalDamages[2];

			wCast = 1 * n_A_CAST;
			wDelay= 1;
			swDelay = 1;
			CastAndDelay();
			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==106 || n_A_ActiveSkill==112 || n_A_ActiveSkill==113){
			n_SpSkill=1;
			if(n_A_ActiveSkill==106){
				n_A_Weapon_element = 2;
				finalDamages[2] = Math.floor((75 + n_A_DEX) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[3]][2]);
			}
			else if(n_A_ActiveSkill==112){
				n_A_Weapon_element = 4;
				finalDamages[2] = Math.floor((50 + n_A_DEX/2) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[3]][4]) * eval(document.calcForm.SkillSubNum.value);
			}
			else if(n_A_ActiveSkill==113){
				n_A_Weapon_element = 3;
				finalDamages[2] = Math.floor((75 + n_A_DEX/2) * (1+ n_A_INT /100) * n_A_ActiveSkillLV * element[n_B[3]][3]) * eval(document.calcForm.SkillSubNum.value);
			}

			finalDamages[2] = tPlusDamCut(finalDamages[2]);
			finalDamages[0]=finalDamages[2];
			finalDamages[1]=finalDamages[2];
			myInnerHtml("ATK_02",finalDamages[2],0);
			myInnerHtml("ATK_00",finalDamages[0],0);
			myInnerHtml("ATK_01",finalDamages[1],0);

			CastAndDelay();

			BattleCalc998(0,0);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==25){
			n_A_Weapon_element = 6;
			wDelay=1;
			swDelay = 1;
			n_Enekyori=2;
			finalDamages[2] = HealCalc(n_A_ActiveSkillLV,0);
			finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[n_B[3]][6]);
			if(n_B[3] < 90){
				finalDamages[2]=0;
			}

			finalDamages[2] = tPlusDamCut(finalDamages[2]);
			finalDamages[0]=finalDamages[2];
			finalDamages[1]=finalDamages[2];
			myInnerHtml("ATK_02",finalDamages[2],0);
			myInnerHtml("ATK_00",finalDamages[0],0);
			myInnerHtml("ATK_01",finalDamages[1],0);

			CastAndDelay();

			BattleCalc998(0,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==94){
			n_A_Weapon_element = 6;
			n_SpSkill=1;
			n_Enekyori=2;
			if(n_A_ActiveSkillLV <= 6)
				finalDamages[2] = 100 * n_A_ActiveSkillLV;
			else
				finalDamages[2] = 777;
			finalDamages[2] = Math.floor(Math.floor(finalDamages[2] / 2) * element[n_B[3]][6]);
			if(n_B[3] < 90 && n_B[2] != 6)
				finalDamages[2]=0;
			if(n_B[2] != 6 && n_B[2] != 1)
				finalDamages[2]=0;

			w_HEAL_BAI = 100;
			if(EquipNumSearch(644))
				w_HEAL_BAI += Math.floor(weaponRefinementLevel *1.5)
			if(CardNumSearch(332))
				w_HEAL_BAI += 30 * CardNumSearch(332);
			finalDamages[2] = Math.floor(finalDamages[2] * w_HEAL_BAI / 100);

			finalDamages[2] = tPlusDamCut(finalDamages[2]);
			finalDamages[0]=finalDamages[2];
			finalDamages[1]=finalDamages[2];
			myInnerHtml("ATK_02",finalDamages[2],0);
			myInnerHtml("ATK_00",finalDamages[0],0);
			myInnerHtml("ATK_01",finalDamages[1],0);

			CastAndDelay();

			BattleCalc998(0,0);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==102){
			n_A_Weapon_element = 6;
			n_Enekyori=2;
			if(n_B[3] < 90){
				w = 0;
				finalDamages[2] = 0;
				finalDamages[0] = 0;
				finalDamages[1] = 0;
			}else{
				if(n_B[19] != 1){
					w = (20 * n_A_ActiveSkillLV + n_A_BaseLV + n_A_INT +n_A_LUK)/1000;
					finalDamages[2] = n_B[6];
				}
				else{
					w = 0;
					finalDamages[2] = 0;
				}
				finalDamages[0] = n_A_BaseLV + n_A_INT + n_A_ActiveSkillLV *10;
				finalDamages[0] = Math.floor(finalDamages[0] * element[n_B[3]][6]);
				finalDamages[1] = Math.round((n_B[6] * w + finalDamages[0] * (100-w)/100));
			}
			myInnerHtml("ATK_02",Math.floor(finalDamages[2] * element[n_B[3]][6]) +"(Success Rate " +Math.round(w *10000)/100 +"%)",0);
			myInnerHtml("ATK_00",finalDamages[0] +"(Failure Damage)",0);
			myInnerHtml("ATK_01",finalDamages[1] +"(Certain One Hit Kill HP)",0);

			wCast = 1 * n_A_CAST;
			wDelay= 3;
			swDelay = 1;
			CastAndDelay();

			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else if(n_A_ActiveSkill==325){
			n_A_Weapon_element = 0;
			n_SpSkill=1;
			n_Enekyori=2;
			hitCount = 4 + n_A_ActiveSkillLV;
			finalDamages[2] = 200 + 200 * n_A_ActiveSkillLV;

			finalDamages[2] = Math.floor(finalDamages[2]);

			wStrG = finalDamages[2] * hitCount +"("+ finalDamages[2] +" x "+ hitCount +"hit)"
			myInnerHtml("ATK_02",wStrG,0);
			myInnerHtml("ATK_00",wStrG,0);
			myInnerHtml("ATK_01",wStrG,0);

			finalDamages[2]=finalDamages[2] * hitCount;
			finalDamages[0]=finalDamages[2];
			finalDamages[1]=finalDamages[2];

			wCast = 5 * n_A_CAST;
			swDelay = 1;
			wDelay= 2;
			CastAndDelay();

			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}

		else
		{
			n_Enekyori=2;
			swDelay = 1;
			skillModifier = 1;
			if(n_A_ActiveSkill==51){
				n_A_Weapon_element = 3;
				hitCount = n_A_ActiveSkillLV;
				wCast = 0.7 * n_A_ActiveSkillLV;
				wDelay= 0.8 + n_A_ActiveSkillLV * 0.2;
			}
			else if(n_A_ActiveSkill==54){
				n_A_Weapon_element = 1;
				hitCount = n_A_ActiveSkillLV;
				wCast = 0.7 * n_A_ActiveSkillLV;
				wDelay= 0.8 + n_A_ActiveSkillLV * 0.2;
			}
			else if(n_A_ActiveSkill==56){
				n_A_Weapon_element = 4;
				hitCount = n_A_ActiveSkillLV;
				wCast = 0.7 * n_A_ActiveSkillLV;
				wDelay= 0.8 + n_A_ActiveSkillLV * 0.2;
			}
			else if(n_A_ActiveSkill==52){
				n_A_Weapon_element = 3;
				if(n_A_ActiveSkillLV <=5){
					wCast = 1.5;
					wDelay= 1.5;
				}else{
					wCast = 1;
					wDelay= 1;
				}
				skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
			}
			else if(n_A_ActiveSkill==53){
				n_A_Weapon_element = 3;
				hitCount = 4 + n_A_ActiveSkillLV;
				wCast = 2.15 - (n_A_ActiveSkillLV * 0.15);
				wDelay= 0.1;
				skillModifier = 0.5;
			}
			else if(n_A_ActiveSkill==55){
				n_A_Weapon_element = 1;
				wCast = 0.8;
				wDelay= 1.5;
				skillModifier = 1 + n_A_ActiveSkillLV * 0.1;
			}
			else if(n_A_ActiveSkill==57){
				n_A_Weapon_element = 4;
				hitCount = n_A_ActiveSkillLV;
				wCast = 1 * n_A_ActiveSkillLV;
				wDelay= 2;
				skillModifier = 0.8;
			}
			else if(n_A_ActiveSkill==46){
				n_A_Weapon_element = 8;
				wCast = 0.5;
				if(n_A_ActiveSkillLV==10)
					wDelay= 0.5;
				else if(n_A_ActiveSkillLV==9)
					wDelay= 0.6;
				else if(n_A_ActiveSkillLV==8)
					wDelay= 0.7;
				else if(n_A_ActiveSkillLV>=6)
					wDelay= 0.8;
				else if(n_A_ActiveSkillLV>=4)
					wDelay= 0.9;
				else
					wDelay= 1;
				skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
			}
			else if(n_A_ActiveSkill==47){
				n_A_Weapon_element = 8;
				hitCount = Math.round(n_A_ActiveSkillLV / 2);
				wCast = 0.5;
				if(n_A_ActiveSkillLV % 2 == 0)
					wDelay= 0.8 + n_A_ActiveSkillLV / 2 *0.2;
				else
					wDelay= 1 + (n_A_ActiveSkillLV+1) / 2 *0.2;
			}
			else if(n_A_ActiveSkill==122){
				n_A_Weapon_element = 3;
				hitCount = n_A_ActiveSkillLV +2;
				wCast = 3.3 - (0.3 * n_A_ActiveSkillLV);
				wDelay= 1;
				skillModifier = 0.2;
			}
			else if(n_A_ActiveSkill==124){
				n_A_Weapon_element = 3;
				wCast = 0.7;
				wDelay= 2;
				skillModifier = 1 + n_A_ActiveSkillLV * 0.2;
			}
			else if(n_A_ActiveSkill==125){
				n_A_Weapon_element = 3;
				hitCount = Math.round(n_A_ActiveSkillLV / 2) * (Math.floor(n_A_ActiveSkillLV / 2) + 2);
				wCast = 15;
				wDelay= Math.floor(n_A_ActiveSkillLV / 2) * 1 +2;
			}
			else if(n_A_ActiveSkill==126){
				n_A_Weapon_element = 4;
				hitCount = n_A_ActiveSkillLV + 2;
				wCast = 2 + n_A_ActiveSkillLV * 0.5;
				wDelay= 0.01;
			}
			else if(n_A_ActiveSkill==127){
				n_A_Weapon_element = 4;
				hitCount = 4;
				wCast = 15.5 - n_A_ActiveSkillLV * 0.5;
				wDelay= 5;
				skillModifier = 0.8 + n_A_ActiveSkillLV * 0.2;
			}
			else if(n_A_ActiveSkill==128 || n_A_ActiveSkill==320){
				swDelay = 2;
				n_A_Weapon_element = 1;
				if(n_A_ActiveSkillLV >= 4)
					hitCount = 25
				else if(n_A_ActiveSkillLV >= 2)
					hitCount = 9;
				wCast = n_A_ActiveSkillLV;
				skillModifier = 1 + n_A_ActiveSkillLV * 0.3;
				wDelay= 0.1 * hitCount;
			}
			else if(n_A_ActiveSkill==130){
				skillModifier = 0.66 + n_A_ActiveSkillLV * 0.066;
				n_A_Weapon_element = 1;
				wCast = 6 - Math.floor((n_A_ActiveSkillLV-1) /2) * 0.5;
				wDelay= 1;
			}
			else if(n_A_ActiveSkill==131){
				n_A_Weapon_element = 1;
				hitCount = eval(document.calcForm.SkillSubNum.value);
				wCast = 5 + n_A_ActiveSkillLV;
				wDelay= 5;
				skillModifier = 1 + n_A_ActiveSkillLV * 0.4;
			}
			else if(n_A_ActiveSkill==132 || n_A_ActiveSkill==133){
				n_A_Weapon_element = 2;
				hitCount = n_A_ActiveSkillLV;
				if(n_A_ActiveSkill==132){
					wCast = n_A_ActiveSkillLV *0.7;
					wDelay= 0.8 + n_A_ActiveSkillLV * 0.2;
				}
				else{
					wCast = n_A_ActiveSkillLV;
					wDelay= 1;
				}
			}
			else if(n_A_ActiveSkill==277){
				hitCount = n_A_ActiveSkillLV;
				n_A_Weapon_element = 8;
				wCast = 1;
				wDelay= 1;
				skillModifier = 0.7 + n_A_ActiveSkillLV * 0.1;
			}
			else if(n_A_ActiveSkill==37 || n_A_ActiveSkill==387){
				n_A_Weapon_element = 6;
				wCast = 2;
				skillModifier = 1.25;
				if(n_A_ActiveSkill==387)
					skillModifier *= 5;
				wDelay= 0.01;
			}
			else if(n_A_ActiveSkill==104){
				n_SpSkill=1;
				n_A_Weapon_element = 6;
				hitCount = n_A_ActiveSkillLV;
				wCast = 15;
				wDelay= 4;
				if(n_B[2] != 6 && n_B[3] < 90){
					n_A_MATK[2]=0;n_A_MATK[0]=0;n_A_MATK[1]=0;
				}
			}else if(n_A_ActiveSkill==373){
				n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
				wCast = 0.1;
				wDelay= 0.5;
				if(n_B[4] == 0)
					skillModifier = n_A_ActiveSkillLV * 0.1;
				else
					skillModifier = 0.01;
				if(Taijin==1)
					skillModifier = 0;
			}
			else if(n_A_ActiveSkill==374){
				n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
				wCast = 0.1;
				wDelay= 0.5;

				skillModifier = n_A_ActiveSkillLV * 0.05;


				if(Taijin==1)
					skillModifier = 0;
			}
			else if(n_A_ActiveSkill==375){
				n_A_Weapon_element = eval(document.calcForm.A_Weapon_element.value);
				n_SpSkill=1;
				hitCount = n_A_ActiveSkillLV;
				wCast = 2;
				wDelay= 0.5;
				skillModifier = 0.4 + n_A_BaseLV / 100;
				if(Taijin==1)
					skillModifier = 0;
			}
			else if(n_A_ActiveSkill==407){
				n_A_Weapon_element = 3;
				skillModifier = 0.9;
				hitCount = n_A_ActiveSkillLV;
				wCast = 0.7 * n_A_ActiveSkillLV;
				wDelay= 0.01;
			}
			else if(n_A_ActiveSkill==408){
				n_A_Weapon_element = 3;
				skillModifier = 0.5;
				hitCount = Math.round(n_A_ActiveSkillLV / 2) +4 ;
				wCast = 6.5 - 0.5 * n_A_ActiveSkillLV;
				wDelay= 1;
				n_SpSkill=1;
			}
			else if(n_A_ActiveSkill==409){
				n_A_Weapon_element = 3;
				skillModifier = 1.5 + n_A_ActiveSkillLV * 1.5;
				hitCount = 1;
				wCast = 3;
				wDelay= 3;
			}
			else if(n_A_ActiveSkill==410){
				n_A_Weapon_element = 1;
				skillModifier = 1;
				hitCount = n_A_ActiveSkillLV + 2;
				wCast = n_A_ActiveSkillLV * 0.7;
				wDelay= 0.01;
			}
			else if(n_A_ActiveSkill==412){
				n_A_Weapon_element = 1;
				skillModifier = 1.0 + n_A_ActiveSkillLV * 0.5;
				hitCount = 1;
				wCast = 3;
				wDelay= 3;
			}
			else if(n_A_ActiveSkill==413){
				n_A_Weapon_element = 4;
				skillModifier = 1.0;
				hitCount = Math.floor(n_A_ActiveSkillLV / 2) +1;
				wCast = Math.floor(n_A_ActiveSkillLV / 2) + 1;
				wDelay= 1;
			}
			else if(n_A_ActiveSkill==414){
				n_A_Weapon_element = 4;
				skillModifier = 1.6 + 0.4 * n_A_ActiveSkillLV;
				hitCount = 1;
				wCast = 4;
				wDelay= 0.01;

			}
			else if(n_A_ActiveSkill==415){
				n_A_Weapon_element = 4;
				skillModifier = 1.0 + n_A_ActiveSkillLV * 1.0;
				hitCount = 1;
				wCast = 4;
				wDelay= 0.01;
			}

			wCast *= n_A_CAST;

			for(b=0;b<=2;b++){
				finalDamages[b] = BattleMagicCalc(n_A_MATK[b] * skillModifier);
				myInnerHtml("ATK_0"+b,finalDamages[b] * hitCount + "("+finalDamages[b]+ SubName[8] +hitCount+"hit)",0);
				finalDamages[b] *= hitCount;
			}

			CastAndDelay();

			BattleCalc998(wCast,wDelay);
			myInnerHtml("BattleHIT",100,0);
		}
	}


	function ATKbai01()
	{
		wA01 = 100;
		if(n_A_ActiveSkill != 193 &&n_A_ActiveSkill != 197 && n_A_ActiveSkill != 321){
			if(SkillSearch(12))
				wA01 += 32;
			else if(n_A_PassSkill2[12])
				wA01 += 5;


			if(SkillSearch(256))
				wA01 += SkillSearch(256) * 5;
			if(SkillSearch(270))
				wA01 += SkillSearch(270) * 2;
			if(n_A_PassSkill5[3])
				wA01 += 100;
			if(n_A_PassSkill6[2])
				wA01 += 10;
			if(StPlusCalc2(87))
				wA01 += StPlusCalc2(87);
		}
		n_A_CriATK[2] = n_A_CriATK[2] * wA01 /100;
		n_A_CriATK[0] = n_A_CriATK[0] * wA01 /100;
		n_A_CriATK[1] = n_A_CriATK[1] * wA01 /100;
		n_A_DMG[2] = n_A_DMG[2] * wA01 /100;
		n_A_DMG[0] = n_A_DMG[0] * wA01 /100;
		n_A_DMG[1] = n_A_DMG[1] * wA01 /100;
	}


	function applySkillModifier(wATKbai,ch_A02)
	{
		wA02 = wATKbai * 100;
		if(SkillSearch(327)){
			wA02 += 20 * SkillSearch(327);
		}
		else{
			if(SkillSearch(154))
				wA02 += SkillSearch(154) * 5;
			if(SkillSearch(154)==0 && n_A_PassSkill2[8])
				wA02 += n_A_PassSkill2[8] * 5 / 5;
		}
		if(SkillSearch(342)){
			wA02 += 2 * SkillSearch(342) * SkillSearch(380);
		}

		if(ch_A02 == 0){
			n_A_DMG[2] = Math.floor(n_A_DMG[2] * wA02 /100);
			n_A_DMG[0] = Math.floor(n_A_DMG[0] * wA02 /100);
			n_A_DMG[1] = Math.floor(n_A_DMG[1] * wA02 /100);
		}else{
			n_A_CriATK[1] = Math.floor(n_A_CriATK[1] * wA02 /100);
			n_A_CriATK[0] = Math.floor(n_A_CriATK[0] * wA02 /100);
			n_A_CriATK[2] = Math.floor(n_A_CriATK[2] * wA02 /100);
		}
	}


	function BattleTAKA()
	{
		if(n_A_WeaponType==10 && SkillSearch(118) && n_A_ActiveSkill !=272){
			myInnerHtml("bSUBname","Bird Damage (Atk Rate))",0);
			wBTw1 = Math.floor((n_A_JobLV -1) / 10 +1);
			if(wBTw1 > 5)wBTw1=5;
			wBTw2 = SkillSearch(118);
			if(wBTw2 < wBTw1)
				wBTw1 = wBTw2;
			wBT = 80 + Math.floor(n_A_DEX /10)*2 + Math.floor(n_A_INT/2)*2 + SkillSearch(119) *6;
			wBT = Math.floor(wBT * element[n_B[3]][0]);
			wBT = tPlusDamCut(wBT);
			wBTw3 = Math.round((1 + n_A_LUK * 0.3)*100)/100;
			if(n_B[0] == 44)
				wBT = 0;
			myInnerHtml("bSUB",wBT * wBTw1 +"("+ wBTw3 +"%)",0);
			wBT = wBT * wBTw1 * wBTw3 /100;
			wBT = wBT * (w_HIT + ((100 - w_HIT) * w_Cri /100)) /100;
			wBTw1=0;
			return Math.round(wBT *100)/100;
		}else
			return 0;
	}


	function HealCalc(HealLv,HealType)
	{
		wHeal = Math.floor((n_A_BaseLV + n_A_INT) /8) * (HealLv *8 +4);
		wHealBAI = 100;
		wHealBAI += SkillSearch(269) *2;
		if(EquipNumSearch(644))
			wHealBAI += Math.floor(weaponRefinementLevel *1.5)
		if(CardNumSearch(332))
			wHealBAI += 30 * CardNumSearch(332);
		if(HealType==0)
			if(EquipNumSearch(751) || EquipNumSearch(771))
				wHealBAI += 50;
		wHeal = Math.floor(wHeal * wHealBAI /100);
		return wHeal;
	}

	function BattleCalc998(BC998Cast,BC998Delay)
	{
		if(n_B[0]==44 && n_A_ActiveSkill !=0){
			for(i=0;i<=2;i++){
				finalDamages[i] = 0;
				myInnerHtml("ATK_0"+i,0,0);
			}
		}

		tPlusAG();
		w = n_B[6];
		for(i=0;0<w && i<1000;i++){
			w -= finalDamages[2];
		}
		if(i<1000)
			myInnerHtml("MinATKnum",i,0);
		else
			myInnerHtml("MinATKnum",SubName[5],0);
		w = n_B[6];
		for(i=0;0<w && i<1000;i++){
			w -= finalDamages[0];
		}
		if(i<1000)
			myInnerHtml("MaxATKnum",i,0);
		else
			myInnerHtml("MaxATKnum",SubName[5],0);
		w = n_B[6];
		for(i=0;0<w && i<1000;i++){
			w -= finalDamages[1];
		}

		if(Taijin==0){
			if(i<1000){
				myInnerHtml("AtkBaseExp",Math.round(n_B[16] / i) +"Exp",0);
				myInnerHtml("AtkJobExp",Math.round(n_B[17] / i) +"Exp",0);
			}else{
				myInnerHtml("AtkBaseExp",SubName[7],0);
				myInnerHtml("AtkJobExp",SubName[7],0);
			}
		}
		if(i<1000)
		{
			myInnerHtml("AveATKnum",i,0);

			n_AveATKnum = i;


			if(BC998Delay == 0)

				w = (BC998Cast + n_A_ASPD) * n_AveATKnum;
			else
				w = (BC998Cast + BC998Delay) * n_AveATKnum;
			w = Math.floor(w * 100) / 100;

			if(n_SpSkill)
				myInnerHtml("BattleTime","Special",0);
			else
				myInnerHtml("BattleTime",w + "s",0);
		}
		else
		{
			myInnerHtml("AveATKnum",SubName[5],0);
			myInnerHtml("BattleTime",SubName[6],0);
		}

		if(BC998Delay == 0)

			w = 1 / (BC998Cast + n_A_ASPD) * finalDamages[1];
		else
			w = 1 / (BC998Cast + BC998Delay) * finalDamages[1];
		w *= 100;
		w = Math.round(w);
		w /= 100;

		if(n_SpSkill)
			myInnerHtml("AveSecondATK","Special",0);
		else
			myInnerHtml("AveSecondATK",w,0);

		n_SpSkill=0

		if(Taijin==0){
			w = BattleHiDam();

			w = Math.round(w *(100-n_A_LUCKY))/100;
			w = Math.round(w *(100-w_FLEE))/100;
			if(SkillSearch(157)){
				w = Math.round(w * w_AG[SkillSearch(157)])/100;
			}
			if(n_A_WeaponType==3 && SkillSearch(255)){
				w = Math.round(w * (80- SkillSearch(255) *3))/100;
			}
			if(SkillSearch(287)){
				w = Math.round(w * (100- SkillSearch(287) *7.5))/100;
			}
			myInnerHtml("B_Ave2Atk",w+"Damage",0);

		}
	}

	function BattleHiDam(){

		w_HiDam = new Array();
		wBHD = n_B[13];
		w_HiDam[0] = n_B[12];
		w_HiDam[1] = (n_B[12] *5 + wBHD) /6;
		w_HiDam[2] = (n_B[12] *4 + wBHD *2) /6;
		w_HiDam[3] = (n_B[12] + wBHD) /2;
		w_HiDam[4] = (n_B[12] *2 + wBHD *4) /6;
		w_HiDam[5] = (n_B[12] + wBHD *5) /6;
		w_HiDam[6] = wBHD;
		if(n_B[12] == n_B[13])
			w_HiDam[6] = wBHD - 1;

		w_HiDam[0] = w_HiDam[0] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[2];
		w_HiDam[1] = w_HiDam[1] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[2];
		w_HiDam[2] = w_HiDam[2] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[2];
		w_HiDam[3] = w_HiDam[3] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[1];
		w_HiDam[4] = w_HiDam[4] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[0];
		w_HiDam[5] = w_HiDam[5] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[0];
		w_HiDam[6] = w_HiDam[6] * (100-n_A_totalDEF) / 100 - n_A_VITDEF[0];


		if(SkillSearch(23) && (n_B[3]>=90 || n_B[2]==6)){
			wBHD = Math.floor((3 + 4/100 * n_A_BaseLV) * SkillSearch(23));
			for(i=0;i<=6;i++)
				w_HiDam[i] -= wBHD;
		}


		if(SkillSearch(355)){
			wBHD = Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 2);
			for(i=0;i<=6;i++)
				w_HiDam[i] -= wBHD;
		}


		wBHD = StPlusCard(60);
		wBHD += StPlusCalc2(60);
		if(EquipNumSearch(737) || EquipNumSearch(769))
			wBHD += n_A_SHOULDER_DEF_PLUS * 3;
		if(SkillSearch(150))
			wBHD += SkillSearch(150);
		if(n_A_SHOULDER_DEF_PLUS >= 9 && CardNumSearch(403))
			wBHD += 5;
		if(wBHD != 0){
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
		}

		if(SkillSearch(58)){
			wBHD = 6 * SkillSearch(58);
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
		}


		wBHD = 0;
		if(CardNumSearch(452) && n_A_JobSearch()==3 && (n_B[2]==6 || n_B[2] == 1))
			wBHD += 30;
		if(n_A_PassSkill2[14] && n_B[2] == 6 && n_A_JOB != 13 && n_A_JOB != 27)
			wBHD += n_A_PassSkill2[14] * 5;
		if(n_B[2]==9  && SkillSearch(234))
			wBHD += SkillSearch(234) *4;
		wBHD += StPlusCard(50+n_B[2]);
		wBHD += StPlusCalc2(50+n_B[2]);
		if(wBHD != 0){
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
		}


		wBHD = 0;
		wBHD += StPlusCard(190+n_B[4]);
		wBHD += StPlusCalc2(190+n_B[4]);
		if(n_B[4] == 1){
			if(EquipNumSearch(624))
				wBHD += weaponRefinementLevel;
		}

		if(wBHD != 0){
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
		}


		if(n_B[19] == 0){
			wBHD = StPlusCard(79);
			wBHD += StPlusCalc2(79);
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
		}


		if(n_B[20]){
			wBHD = StPlusCard(78);
			wBHD += StPlusCalc2(78);
			if(SkillSearch(421))
				wBHD += 20;
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);

			if(SkillSearch(165)){
				wBHD = 5 + 15 * SkillSearch(165);
				for(i=0;i<=6;i++)
					w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);
			}
		}


		if(n_B[19]==1 && CardNumSearch(231)){
			for(i=0;i<=6;i++)
				w_HiDam[i] -= Math.floor(w_HiDam[i] * 40 /100);

		}


		if(n_B[0]==275 && CardNumSearch(370)){
			wBHD = 100 * CardNumSearch(370);
			for(i=0;i<=6;i++)
				w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD /100);

		}


		wBHD = StPlusCard(3000+n_B[0]);
		wBHD += StPlusCalc2(3000+n_B[0]);
		for(i=0;i<=6;i++)
			w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD /100);



		if(EquipNumSearch(696)){
			wBHD = 20;
			for(i=0;i<=6;i++)
				w_HiDam[i] += Math.floor(w_HiDam[i] * wBHD /100);

		}

		for(i=0;i<=6;i++){
			if(w_HiDam[i] < 1)
				w_HiDam[i]=1;
			w_HiDam[i] = Math.floor(w_HiDam[i] * 100) / 100;
		}

		if(n_A_PassSkill2[5])
			for(i=0;i<=6;i++)
				w_HiDam[i] = Math.floor(w_HiDam[i] / 2);

		w_HiDam[0] = Math.floor(w_HiDam[0]);
		w_HiDam[6] = Math.floor(w_HiDam[6]);


		wBHD=0;
		for(i=0;i<=6;i++)
			wBHD += w_HiDam[i];
		wBHD = Math.round(wBHD / 7);
		myInnerHtml("B_AveAtk",wBHD +" ("+ w_HiDam[0] +" ~ "+ w_HiDam[6]+")",0);

		return wBHD;
	}

	function BattleMagicCalc(wBMC)
	{
		wBMC_MDEF = n_B[15];
		wMDEF_w = 0;
		if(EquipNumSearch(645))
			wMDEF_w += 10 + weaponRefinementLevel;
		if(n_B[19]==0 && CardNumSearch(424))
			wMDEF_w += 100;
		if(n_B[19]==1 && CardNumSearch(425))
			wMDEF_w += 30 * CardNumSearch(425);
		if(wMDEF_w > 100)
			wMDEF_w = 100;
		if(wMDEF_w != 0){
			wBMC_MDEF = wBMC_MDEF - Math.floor(wBMC_MDEF * wMDEF_w / 100);
			n_B_MDEF2 = n_B_MDEF2 - Math.floor(n_B_MDEF2 * wMDEF_w / 100);
		}
		if(n_A_ActiveSkill==122)
			wBMC2 = Math.floor(wBMC + 50);
		else
			wBMC2 = Math.floor(wBMC * (100 - wBMC_MDEF) /100 - n_B_MDEF2);
		if(wBMC2 < 1)wBMC2=1;
		if(n_A_ActiveSkill==104){
			if(n_B[2] != 6 && n_B[3] < 90){
				wBMC2=0;
			}
		}

		wBMC2 = Math.floor(wBMC2 * element[n_B[3]][n_A_Weapon_element]);
		if(90 <= n_B[3] && (n_A_ActiveSkill==47 || n_A_ActiveSkill==313))
			wBMC2 = Math.floor(wBMC2 * (1 + 0.05 * n_A_ActiveSkillLV));


		if(n_B[2]==9  && SkillSearch(234))
			wBMC2 = wBMC2 * (100 +SkillSearch(234) *2) /100;

		if(n_B[2]==8)
			wBMC2 = wBMC2 * (100 +CardNumSearch(427) *10) /100;

		if(n_B[2]==6)
			wBMC2 = wBMC2 * (100 +CardNumSearch(428) *2) /100;

		wBMC2 = tPlusDamCut(wBMC2);


		wBMC2 = wBMC2 * (100+StPlusCalc2(5000+n_A_ActiveSkill)+StPlusCard(5000+n_A_ActiveSkill)) /100;

		wBMC2 = Math.floor(wBMC2);

		return wBMC2;
	}

	function ClickJob(n)
	{

		myInnerHtml("A_KakutyouSel","",0);
		myInnerHtml("A_KakutyouData","",0);
		document.calcForm.A_Kakutyou.value = 0;

		for(i=0;i<=12;i++)
			n_A_PassSkill2[i] = 0;
		if(n_SkillSW){
			document.calcForm.A2_Skill0.value = 0;
			document.calcForm.A2_Skill1.value = 0;
			document.calcForm.A2_Skill2.value = 0;
			document.calcForm.A2_Skill3.checked = 0;
			document.calcForm.A2_Skill4.value = 0;
			document.calcForm.A2_Skill5.checked = 0;
			document.calcForm.A2_Skill6.checked = 0;
			document.calcForm.A2_Skill7.checked = 0;
			document.calcForm.A2_Skill8.value = 0;
			document.calcForm.A2_Skill9.value = 0;
			document.calcForm.A2_Skill10.value = 0;
			document.calcForm.A2_Skill11.checked = 0;
		}

		n_A_JobSet();
		n = n_A_JOB;

		for(i=71;i>=0;i--)
			document.calcForm.A_JobLV.options[i] = null;
		w=0;
		if(n == 0)w=10;
		else if(n <= 19 || (41 <= n && n <= 43))w=50;
		else if(n == 20)w=71;
		else w=70;
		for(i=1;i<=w;i++)
			document.calcForm.A_JobLV.options[i-1] = new Option(i,i);
		if(n==20){
			document.calcForm.A_JobLV.options[69] = new Option("70-99",70);
			document.calcForm.A_JobLV.options[70] = new Option("+10",71);
		}

		if(n_A_JOB != 20)
			SuperNoviceFullWeaponCHECK = 0;
		if(SuperNoviceFullWeaponCHECK)
			JobASPD[20][7] = 120;
		else
			JobASPD[20][7] = 0;

		for(i=21;i>=0;i--)
		{
			document.calcForm.A_WeaponType.options[i] = null;
		}
		j = 0;
		for (i=0; i<=21; i++)
		{
			if(JobASPD[n][i] != 0)
			{
				document.calcForm.A_WeaponType.options[j] = new Option(WeaponName[i],i);
				j++;
			}
		}


		ClickWeaponType(0);








		for(i=0;i<=14;i++){
			if(JobSkillPassOBJ[n][i] != 999){
				myInnerHtml("P_Skill"+i,SkillOBJ[JobSkillPassOBJ[n][i]][2],0);
				myInnerHtml("P_Skill"+i+"s",'<select name="A_skill'+i+'"onChange="StAllCalc()"></select>',0);
			}
			else{
				myInnerHtml("P_Skill"+i,"",0);
				myInnerHtml("P_Skill"+i+"s","",0);
			}
		}


		/*	for(j=0;j<=14;j++){
		if(JobSkillPassOBJ[n][j] != 999){
			wSeOB = document.getElementById("A_skill"+j);
			for(i=10;i>=0;i--)
				wSeOB.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][j]][1];i++)
				wSeOB.options[i] = new Option(i,i);
		}
	}
*/
		if(JobSkillPassOBJ[n][0] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill0.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][0]][1];i++)
				document.calcForm.A_skill0.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][1] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill1.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][1]][1];i++)
				document.calcForm.A_skill1.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][2] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill2.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][2]][1];i++)
				document.calcForm.A_skill2.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][3] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill3.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][3]][1];i++)
				document.calcForm.A_skill3.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][4] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill4.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][4]][1];i++)
				document.calcForm.A_skill4.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][5] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill5.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][5]][1];i++)
				document.calcForm.A_skill5.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][6] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill6.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][6]][1];i++)
				document.calcForm.A_skill6.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][7] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill7.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][7]][1];i++)
				document.calcForm.A_skill7.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][8] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill8.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][8]][1];i++)
				document.calcForm.A_skill8.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][9] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill9.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][9]][1];i++)
				document.calcForm.A_skill9.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][10] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill10.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][10]][1];i++)
				document.calcForm.A_skill10.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][11] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill11.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][11]][1];i++)
				document.calcForm.A_skill11.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][12] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill12.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][12]][1];i++)
				document.calcForm.A_skill12.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][13] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill13.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][13]][1];i++)
				document.calcForm.A_skill13.options[i] = new Option(i,i);
		}
		if(JobSkillPassOBJ[n][14] != 999){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill14.options[i] = null;
			for(i=0;i<=SkillOBJ[JobSkillPassOBJ[n][14]][1];i++)
				document.calcForm.A_skill14.options[i] = new Option(i,i);
		}


		if(JobSkillPassOBJ[n][0]==58){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill0.options[i] = null;
			n_ECname=["0","6% Reduction","12% Reduction","18% Reduction","24% Reduction","30% Reduction"];
			for(i=0;i<=5;i++)
				document.calcForm.A_skill0.options[i] = new Option(n_ECname[i],i);
		}

		if(JobSkillPassOBJ[n][5]==78){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill5.options[i] = null;
			n_ECname=["No Peco","Mastery 0","Mastery 1","Mastery 2","Mastery 3","Mastery 4","Mastery 5"];
			for(i=0;i<=6;i++)
				document.calcForm.A_skill5.options[i] = new Option(n_ECname[i],i);
		}

		if(JobSkillPassOBJ[n][9]==78){
			for(i=10;i>=0;i--)
				document.calcForm.A_skill9.options[i] = null;
			n_ECname=["No Peco","Mastery 0","Mastery 1","Mastery 2","Mastery 3","Mastery 4","Mastery 5"];
			for(i=0;i<=6;i++)
				document.calcForm.A_skill9.options[i] = new Option(n_ECname[i],i);
		}




		for(i=57;i>=0;i--)
			document.calcForm.A_ActiveSkill.options[i] = null;
		for(i=0;i<=57 && JobSkillActiveOBJ[n][i] != 999;i++)
			document.calcForm.A_ActiveSkill.options[i] = new Option(SkillOBJ[JobSkillActiveOBJ[n][i]][2],JobSkillActiveOBJ[n][i]);


		for(i=0;i<20;i++)
			w_ASSP0bk[i] = 999;
		ActiveSkillSetPlus();

		ClickActiveSkill(0);
		WeaponSet2();
	}

	function ClickWeaponType(n)
	{
		n_A_JobSet();
		if(n_A_JobSearch()==2 || n_A_JobSearch()==4 || (n_A_JOB==45 && n!=0)){
			document.calcForm.A_Arrow.style.visibility = "visible";
			for(i=17;i>=0;i--)
			{
				document.calcForm.A_Arrow.options[i] = null;
			}
			if(n==10||n==14||n==15){
				j=17;
				for (i=0; i<=4; i++)
					ArrowOBJ[i] = ArrowOBJbackup[i];
			}else if(n==17||n==18||n==19||n==20){
				j=2;
				for (i=0; i<=2; i++)
					ArrowOBJ[i] = BulletOBJ[i];
			}else if(n==21){
				j=4;
				for (i=0; i<=4; i++)
					ArrowOBJ[i] = GrenadeOBJ[i]
			}else{
				j=1;
				ArrowOBJ[0] = [0,0,"No Arrow"];
				ArrowOBJ[1] = ArrowOBJ[16];
			}
			for(i=0; i<=j; i++)
				document.calcForm.A_Arrow.options[i] = new Option(ArrowOBJ[i][2],i);
		}else{
			document.calcForm.A_Arrow.value = 0;
			document.calcForm.A_Arrow.style.visibility = "hidden";
		}
		WeaponSet();


		if(n == 0){
			myInnerHtml("A_seirenchi_name","",0);
			document.calcForm.A_Weapon_ATKplus.style.visibility = "hidden";
			document.calcForm.A_Weapon_ATKplus.value = 0;
		}
		else{
			myInnerHtml("A_seirenchi_name","Refine: ",0);
			document.calcForm.A_Weapon_ATKplus.style.visibility = "visible";
		}


		n_A_JobSet();
		if((n_A_JOB == 8 || n_A_JOB == 22) && n != 11){
			if(hasLeftHand == 0)
				myInnerHtml("A_SobWeaponName"," Left Hand: "+'<select name="A_Weapon2Type" onChange = "ClickWeaponType2(this[this.selectedIndex].value) | StAllCalc()"> <option value="0">Unarmed (or Shield)<option value="1">Dagger<option value="2">One-Hand Sword<option value="6">One-Hand Axe</select>',0);
		}
		else{
			myInnerHtml("A_SobWeaponName","",0);
			myInnerHtml("spanA_weapon2","",0);
			myInnerHtml("spanA_weapon2seiren","",0);
			myInnerHtml("spanA_weapon2_CardShort","",0);
			myInnerHtml("nA_weapon2_c1","",0);
			myInnerHtml("nA_weapon2_c2","",0);
			myInnerHtml("nA_weapon2_c3","",0);
			myInnerHtml("nA_weapon2_c4","",0);
			hasLeftHand = 0;
		}
		n_A_Equip[0] = eval(document.calcForm.A_weapon1.value);
		ActiveSkillSetPlus();
	}


	function ClickWeaponType2(n)
	{

		n_A_JobSet();
		if(n != 0){
			if(hasLeftHand == 0){
				myInnerHtml("spanA_weapon2",'<select name="A_weapon2"onChange="StAllCalc()|ClickB_Item(this[this.selectedIndex].value)"></select>',0);
				myInnerHtml("spanA_weapon2seiren","Refine(Left):"+'<select name="A_Weapon2_ATKplus"></select>',0);
				for(i=0;i<=10;i++){
					document.calcForm.A_Weapon2_ATKplus.options[i] = new Option(i,i);
				}

				myInnerHtml("nA_weapon2_c1",'<select name="A_weapon2_card1"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
				myInnerHtml("nA_weapon2_c2",'<select name="A_weapon2_card2"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
				myInnerHtml("nA_weapon2_c3",'<select name="A_weapon2_card3"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);
				myInnerHtml("nA_weapon2_c4",'<select name="A_weapon2_card4"onChange="StAllCalc()|Click_Card(this[this.selectedIndex].value)"></select>',0);

				for(i=0;CardSortOBJ[0][i]!="NULL";i++)
					document.calcForm.A_weapon2_card1.options[i] = new Option(cardOBJ[CardSortOBJ[0][i]][2],cardOBJ[CardSortOBJ[0][i]][0]);
				for(i=0;CardSortOBJ[1][i]!="NULL";i++){
					document.calcForm.A_weapon2_card2.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
					document.calcForm.A_weapon2_card3.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
					document.calcForm.A_weapon2_card4.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2],cardOBJ[CardSortOBJ[1][i]][0]);
				}
				document.calcForm.A_weapon2_card4.options[1] = new Option("Top 10 Rank",106);
			}
			myInnerHtml("spanA_weapon2_CardShort",'<select name="A_cardshortLeft" onChange="SetCardShortLeft()|StAllCalc()|ActiveSkillSetPlus()"></select>',0);
			document.calcForm.A_cardshortLeft.options[0] = new Option("Card Shortcuts (Left)",0);
			for(i=1;i<=32;i++)
				document.calcForm.A_cardshortLeft.options[i] = new Option(CardShort[i][0],i);
			hasLeftHand = 1;
			WeaponSetLeft();
		}
		else{
			myInnerHtml("spanA_weapon2","",0);
			myInnerHtml("spanA_weapon2seiren","",0);
			myInnerHtml("spanA_weapon2_CardShort","",0);
			myInnerHtml("nA_weapon2_c1","",0);
			myInnerHtml("nA_weapon2_c2","",0);
			myInnerHtml("nA_weapon2_c3","",0);
			myInnerHtml("nA_weapon2_c4","",0);
			hasLeftHand = 0;
		}
		if(hasLeftHand){
			n_A_Equip[1] = eval(document.calcForm.A_weapon2.value);
			ActiveSkillSetPlus();
		}
	}

	function ClickActiveSkill(wAS)
	{
		n_A_ActiveSkill = eval(document.calcForm.A_ActiveSkill.value);
		if(n_A_ActiveSkill > 100000){
			n_A_ActiveSkillLV = Math.floor(n_A_ActiveSkill % 100);
			n_A_ActiveSkill = Math.floor((n_A_ActiveSkill % 100000) /100);
		}else
			n_A_ActiveSkillLV = SkillOBJ[n_A_ActiveSkill][1];

		for(i=10;i>=0;i--)
			document.calcForm.A_ActiveSkillLV.options[i] = null;
		if(n_A_ActiveSkill >= 0)
			for(i=1;i<=n_A_ActiveSkillLV;i++)
				document.calcForm.A_ActiveSkillLV.options[i-1] = new Option(i,i);

		if(SkillOBJ[n_A_ActiveSkill][1] == 1)
			document.calcForm.A_ActiveSkillLV.style.visibility = "hidden";
		else{
			document.calcForm.A_ActiveSkillLV.style.visibility = "visible";
			document.calcForm.A_ActiveSkillLV.value = n_A_ActiveSkillLV;
		}
		ClickActiveSkill2();
	}


	function ClickActiveSkill2(){
		if(n_A_ActiveSkill == 66 || n_A_ActiveSkill == 326){
			myInnerHtml("AASkillName","Card Weight: ",0);
			myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" value="8000" size=8>',0);
		}
		else if(n_A_ActiveSkill == 112 || n_A_ActiveSkill == 113){
			myInnerHtml("AASkillName","Number of Stacked Monsters: ",0);
			myInnerHtml("AASkill",'<select name="SkillSubNum"onChange="calc()"></select>',0);
			for(i=1;i<=20;i++)
				document.calcForm.SkillSubNum.options[i-1] = new Option(i,i);
			document.calcForm.SkillSubNum.value=1;
		}
		else if(n_A_ActiveSkill == 131){
			myInnerHtml("AASkillName","Number of Hits: ",0);
			myInnerHtml("AASkill",'<select name="SkillSubNum"onChange="calc()"></select>',0);
			for(i=1;i<=15;i++)
				document.calcForm.SkillSubNum.options[i-1] = new Option(i,i);
			document.calcForm.SkillSubNum.value=3;
		}
		else if(n_A_ActiveSkill==88){
			myInnerHtml("AASkillName","Poison React Level: ",0);
			myInnerHtml("AASkill",'<select name="SkillSubNum"onChange="calc()"></select>',0);
			for(i=0;i<=10;i++)
				document.calcForm.SkillSubNum.options[i] = new Option(i,i);
			document.calcForm.SkillSubNum.value=5;
		}
		else if(n_A_ActiveSkill==197){
			myInnerHtml("AASkillName","Remaining SP: ",0);
			myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6>',0);
			document.calcForm.SkillSubNum.value = n_A_MaxSP -1;
		}
		else if(n_A_ActiveSkill==394){
			myInnerHtml("AASkillName","",0);
			myInnerHtml("AASkill",'<select name="SkillSubNum"onChange="calc()"></select>',0);
			for(i=0;i<=4;i++)
				document.calcForm.SkillSubNum.options[i] = new Option(SyurikenOBJ[i][2],i);
			document.calcForm.SkillSubNum.value = 0;
		}
		else if(n_A_ActiveSkill==395){
			myInnerHtml("AASkillName","",0);
			myInnerHtml("AASkill",'<select name="SkillSubNum"onChange="calc()"></select>',0);
			for(i=0;i<=4;i++)
				document.calcForm.SkillSubNum.options[i] = new Option(KunaiOBJ[i][2],i);
			document.calcForm.SkillSubNum.value = 0;
		}
		else if(n_A_ActiveSkill==405){
			myInnerHtml("AASkillName","Remaining SP: ",0);
			myInnerHtml("AASkill",'<input type="text" name="SkillSubNum" size=6>',0);
			document.calcForm.SkillSubNum.value = n_A_MaxHP -1;
		}
		else{
			myInnerHtml("AASkillName","",0);
			myInnerHtml("AASkill","",0);
		}
	}

	function Click_SkillSW()
	{
		n_SkillSW = document.calcForm.A2_SKILLSW.checked;

		if(n_SkillSW){
			name_CSSW_SKILL = ["Blessing","Increase Agi","Impositio Manus","Gloria","Angelus","Assumptio","Andrenaline Rush","Weapon Perfection","Power Thrust","Wind Walker","Spirit Spheres (GG Card)","Magnum Break Bonus","Aloevera","<Font size=2>Suffragium</Font>","<Font size=2>Resistant Souls</Font>","<Font size=2>Additional Buffs Found Below</Font>"];
			html_CSSW_SKILL = new Array();
			for(i=0;i<=15;i++)
				myInnerHtml("AS"+i+"_1",name_CSSW_SKILL[i],0);

			html_CSSW_SKILL[0] = '<select name="A2_Skill0"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[1] = '<select name="A2_Skill1"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[2] = '<select name="A2_Skill2"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[3] = '<input type="checkbox" name="A2_Skill3"onClick="calc()">';
			html_CSSW_SKILL[4] = '<select name="A2_Skill4"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[5] = '<input type="checkbox" name="A2_Skill5"onClick="calc()">';
			html_CSSW_SKILL[6] = '<select name="A2_Skill6"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[7] = '<input type="checkbox" name="A2_Skill7"onClick="calc()">';
			html_CSSW_SKILL[8] = '<select name="A2_Skill8"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[9] = '<select name="A2_Skill9"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[10] = '<select name="A2_Skill10"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[11] = '<input type="checkbox" name="A2_Skill11"onClick="calc()">';
			html_CSSW_SKILL[12] = '<input type="checkbox" name="A2_Skill12"onClick="calc()">';
			html_CSSW_SKILL[13] = '<select name="A2_Skill13"onChange="StAllCalc()"></select>';
			html_CSSW_SKILL[14] = '<select name="A2_Skill14"onChange="StAllCalc()"></select>';
			for(i=0;i<=14;i++)
				myInnerHtml("AS"+i+"_2",html_CSSW_SKILL[i],0);


			for(i=0;i<=10;i++){
				document.calcForm.A2_Skill0.options[i] = new Option(i,i);
				document.calcForm.A2_Skill1.options[i] = new Option(i,i);
				document.calcForm.A2_Skill4.options[i] = new Option(i,i);
				document.calcForm.A2_Skill9.options[i] = new Option(i,i);
			}
			for(i=0;i<=5;i++){
				document.calcForm.A2_Skill2.options[i] = new Option(i,i);
				document.calcForm.A2_Skill8.options[i] = new Option(i,i);
				document.calcForm.A2_Skill10.options[i] = new Option(i,i);
				document.calcForm.A2_Skill14.options[i] = new Option(i,i);
			}
			if(n_A_JOB==15||n_A_JOB==29)
				myInnerHtml("AS10_1","-",0);
			for(i=0;i<=3;i++)
				document.calcForm.A2_Skill13.options[i] = new Option(i,i);
			document.calcForm.A2_Skill6.options[0] = new Option("None",0);
			document.calcForm.A2_Skill6.options[1] = new Option("Regular AR",1);
			document.calcForm.A2_Skill6.options[2] = new Option("Full AR",2);
			document.calcForm.A2_Skill6.options[3] = new Option("Scroll",3);

			document.calcForm.A2_Skill0.value = n_A_PassSkill2[0];
			document.calcForm.A2_Skill1.value = n_A_PassSkill2[1];
			document.calcForm.A2_Skill2.value = n_A_PassSkill2[2];
			document.calcForm.A2_Skill3.checked = n_A_PassSkill2[3];
			document.calcForm.A2_Skill4.value = n_A_PassSkill2[4];
			document.calcForm.A2_Skill5.checked = n_A_PassSkill2[5];
			document.calcForm.A2_Skill6.value = n_A_PassSkill2[6];
			document.calcForm.A2_Skill7.checked = n_A_PassSkill2[7];
			document.calcForm.A2_Skill8.value = n_A_PassSkill2[8];
			document.calcForm.A2_Skill9.value = n_A_PassSkill2[9];
			document.calcForm.A2_Skill10.value = n_A_PassSkill2[10];
			document.calcForm.A2_Skill11.checked = n_A_PassSkill2[11];
			document.calcForm.A2_Skill12.checked = n_A_PassSkill2[12];
			document.calcForm.A2_Skill13.value = n_A_PassSkill2[13];
			document.calcForm.A2_Skill14.value = n_A_PassSkill2[14];
		}
		else{
			for(i=0;i<=14;i++){
				myInnerHtml("AS"+i+"_1","",0);
				myInnerHtml("AS"+i+"_2","",0);
			}
			myInnerHtml("AS15_1","",0);
		}
	}

	SWs3sw = [0,0,0,0,0,0,0,0,0,0,0,0];

	function Click_Skill3SW()
	{
		n_Skill3SW = document.calcForm.A3_SKILLSW.checked;

		if(n_Skill3SW){
			name_CS3SW_SKILL = ["Perfect Tabulature","Impressive Rift","Magic Strings","Song of Lutie","Focus Ballet","Lady Luck","Gypsie's Kiss","Acoustic Rhythm","Mental Sensing","Battle Theme","Harmonic Lick"];
			html_CS3SW_SKILL = new Array();
			for(i=0;i<=10;i++)
				myInnerHtml("EN"+i+"_1",name_CS3SW_SKILL[i],0);

			html_CS3SW_SKILL[0] = '<select name="A3_Skill0_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[1] = '<select name="A3_Skill1_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[2] = '<select name="A3_Skill2_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[3] = '<select name="A3_Skill3_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[4] = '<select name="A3_Skill4_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[5] = '<select name="A3_Skill5_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[6] = '<select name="A3_Skill6_1"onChange="Skill3SW_2()|StAllCalc()"></select>';
			html_CS3SW_SKILL[7] = '<select name="A3_Skill7"onChange="StAllCalc()"></select>';
			html_CS3SW_SKILL[8] = '<select name="A3_Skill8"onChange="ClickB_Enemy()"></select>';
			html_CS3SW_SKILL[9] = '<select name="A3_Skill9"onChange="StAllCalc()"></select>';
			html_CS3SW_SKILL[10] = '<select name="A3_Skill10"onChange="StAllCalc()"></select>';
			for(i=0;i<=10;i++)
				myInnerHtml("EN"+i+"_2",html_CS3SW_SKILL[i],0);

			myInnerHtml("EN11_1","Marionette Control"+'<input type="checkbox" name="A3_Skill11"onClick="Skill3SW_2()|calc()">',0);

			for(i=0;i<=10;i++){
				document.calcForm.A3_Skill0_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill1_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill2_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill3_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill4_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill5_1.options[i] = new Option(i,i);
				document.calcForm.A3_Skill6_1.options[i] = new Option(i,i);
			}
			for(i=0;i<=5;i++){
				document.calcForm.A3_Skill7.options[i] = new Option(i,i);
				document.calcForm.A3_Skill8.options[i] = new Option(i,i);
				document.calcForm.A3_Skill9.options[i] = new Option(i,i);
				document.calcForm.A3_Skill10.options[i] = new Option(i,i);
			}

			document.calcForm.A3_Skill0_1.value = n_A_PassSkill3[0];
			document.calcForm.A3_Skill1_1.value = n_A_PassSkill3[1];
			document.calcForm.A3_Skill2_1.value = n_A_PassSkill3[2];
			document.calcForm.A3_Skill3_1.value = n_A_PassSkill3[3];
			document.calcForm.A3_Skill4_1.value = n_A_PassSkill3[4];
			document.calcForm.A3_Skill5_1.value = n_A_PassSkill3[5];
			document.calcForm.A3_Skill6_1.value = n_A_PassSkill3[6];
			document.calcForm.A3_Skill7.value = n_A_PassSkill3[7];
			document.calcForm.A3_Skill8.value = n_A_PassSkill3[8];
			document.calcForm.A3_Skill9.value = n_A_PassSkill3[9];
			document.calcForm.A3_Skill10.value = n_A_PassSkill3[10];
			document.calcForm.A3_Skill11.checked = n_A_PassSkill3[11];

			Skill3SW_2();
		}
		else{
			for(i=1;i<=6;i++){
				myInnerHtml("EN0_"+i,"",0);
				myInnerHtml("EN1_"+i,"",0);
				myInnerHtml("EN3_"+i,"",0);
				myInnerHtml("EN4_"+i,"",0);
				myInnerHtml("EN5_"+i,"",0);
				myInnerHtml("EN6_"+i,"",0);
			}
			for(i=1;i<=8;i++)
				myInnerHtml("EN2_"+i,"",0);
			for(i=1;i<=2;i++){
				myInnerHtml("EN7_"+i,"",0);
				myInnerHtml("EN8_"+i,"",0);
				myInnerHtml("EN9_"+i,"",0);
				myInnerHtml("EN10_"+i,"",0);
				myInnerHtml("EN11_"+i,"",0);
			}

			for(i=0;i<=11;i++)
				SWs3sw[i]=0;
		}
	}

	function Skill3SW_2()
	{
		n_A_PassSkill3[0] = eval(document.calcForm.A3_Skill0_1.value);
		n_A_PassSkill3[1] = eval(document.calcForm.A3_Skill1_1.value);
		n_A_PassSkill3[2] = eval(document.calcForm.A3_Skill2_1.value);
		n_A_PassSkill3[3] = eval(document.calcForm.A3_Skill3_1.value);
		n_A_PassSkill3[4] = eval(document.calcForm.A3_Skill4_1.value);
		n_A_PassSkill3[5] = eval(document.calcForm.A3_Skill5_1.value);
		n_A_PassSkill3[6] = eval(document.calcForm.A3_Skill6_1.value);
		n_A_PassSkill3[11] = eval(document.calcForm.A3_Skill11.checked);

		if(n_A_PassSkill3[0] != 0){
			if(SWs3sw[0] == 0){
				myInnerHtml("EN0_3","Bard's Agi",0);
				myInnerHtml("EN0_4",'<select name="A3_Skill0_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN0_5","Music Lessons",0);
				myInnerHtml("EN0_6",'<select name="A3_Skill0_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=150;i++)
					document.calcForm.A3_Skill0_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill0_3.options[i-1] = new Option(i,i);
				SWs3sw[0] = 1;
				document.calcForm.A3_Skill0_2.value = n_A_PassSkill3[20];
				document.calcForm.A3_Skill0_3.value = n_A_PassSkill3[30];
			}
		}else{
			SWs3sw[0] = 0;
			myInnerHtml("EN0_3","-",0);
			myInnerHtml("EN0_4","-",0);
			myInnerHtml("EN0_5","",0);
			myInnerHtml("EN0_6","",0);
		}

		if(n_A_PassSkill3[1] != 0){
			if(SWs3sw[1] == 0){
				myInnerHtml("EN1_3","Bard's Agi",0);
				myInnerHtml("EN1_4",'<select name="A3_Skill1_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN1_5","Music Lessons",0);
				myInnerHtml("EN1_6",'<select name="A3_Skill1_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=150;i++)
					document.calcForm.A3_Skill1_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill1_3.options[i-1] = new Option(i,i);
				SWs3sw[1] = 1;
				document.calcForm.A3_Skill1_2.value = n_A_PassSkill3[21];
				document.calcForm.A3_Skill1_3.value = n_A_PassSkill3[31];
			}
		}else{
			SWs3sw[1] = 0;
			myInnerHtml("EN1_3","-",0);
			myInnerHtml("EN1_4","-",0);
			myInnerHtml("EN1_5","",0);
			myInnerHtml("EN1_6","",0);
		}

		if(n_A_PassSkill3[2] != 0){
			if(SWs3sw[2] == 0){
				myInnerHtml("EN2_3","Bard's Dex",0);
				myInnerHtml("EN2_4",'<select name="A3_Skill2_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN2_5","Bard's Int",0);
				myInnerHtml("EN2_6",'<select name="A3_Skill2_3"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN2_7","Music Lessons",0);
				myInnerHtml("EN2_8",'<select name="A3_Skill2_4"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=200;i++)
					document.calcForm.A3_Skill2_2.options[i-1] = new Option(i,i);
				for(i=1;i<=150;i++)
					document.calcForm.A3_Skill2_3.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill2_4.options[i-1] = new Option(i,i);
				SWs3sw[2] = 1;
				document.calcForm.A3_Skill2_2.value = n_A_PassSkill3[22];
				document.calcForm.A3_Skill2_3.value = n_A_PassSkill3[29];
				document.calcForm.A3_Skill2_4.value = n_A_PassSkill3[32];
			}
		}else{
			SWs3sw[2] = 0;
			myInnerHtml("EN2_3","-",0);
			myInnerHtml("EN2_4","-",0);
			myInnerHtml("EN2_5","",0);
			myInnerHtml("EN2_6","",0);
			myInnerHtml("EN2_7","",0);
			myInnerHtml("EN2_8","",0);
		}

		if(n_A_PassSkill3[3] != 0){
			if(SWs3sw[3] == 0){
				myInnerHtml("EN3_3","Bard's Vit",0);
				myInnerHtml("EN3_4",'<select name="A3_Skill3_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN3_5","Music Lessons",0);
				myInnerHtml("EN3_6",'<select name="A3_Skill3_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=150;i++)
					document.calcForm.A3_Skill3_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill3_3.options[i-1] = new Option(i,i);
				SWs3sw[3] = 1;
				document.calcForm.A3_Skill3_2.value = n_A_PassSkill3[23];
				document.calcForm.A3_Skill3_3.value = n_A_PassSkill3[33];
			}
		}else{
			SWs3sw[3] = 0;
			myInnerHtml("EN3_3","-",0);
			myInnerHtml("EN3_4","-",0);
			myInnerHtml("EN3_5","",0);
			myInnerHtml("EN3_6","",0);
		}

		if(n_A_PassSkill3[4] != 0){
			if(SWs3sw[4] == 0){
				myInnerHtml("EN4_3","Dancer's Dex",0);
				myInnerHtml("EN4_4",'<select name="A3_Skill4_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN4_5","Dancing Lessons",0);
				myInnerHtml("EN4_6",'<select name="A3_Skill4_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=180;i++)
					document.calcForm.A3_Skill4_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill4_3.options[i-1] = new Option(i,i);
				SWs3sw[4] = 1;
				document.calcForm.A3_Skill4_2.value = n_A_PassSkill3[24];
				document.calcForm.A3_Skill4_3.value = n_A_PassSkill3[34];
			}
		}else{
			SWs3sw[4] = 0;
			myInnerHtml("EN4_3","-",0);
			myInnerHtml("EN4_4","-",0);
			myInnerHtml("EN4_5","",0);
			myInnerHtml("EN4_6","",0);
		}

		if(n_A_PassSkill3[5] != 0){
			if(SWs3sw[5] == 0){
				myInnerHtml("EN5_3","Dancer's Luck",0);
				myInnerHtml("EN5_4",'<select name="A3_Skill5_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN5_5","Dancing Lessons",0);
				myInnerHtml("EN5_6",'<select name="A3_Skill5_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=180;i++)
					document.calcForm.A3_Skill5_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill5_3.options[i-1] = new Option(i,i);
				SWs3sw[5] = 1;
				document.calcForm.A3_Skill5_2.value = n_A_PassSkill3[25];
				document.calcForm.A3_Skill5_3.value = n_A_PassSkill3[35];
			}
		}else{
			SWs3sw[5] = 0;
			myInnerHtml("EN5_3","-",0);
			myInnerHtml("EN5_4","-",0);
			myInnerHtml("EN5_5","",0);
			myInnerHtml("EN5_6","",0);
		}

		if(n_A_PassSkill3[6] != 0){
			if(SWs3sw[6] == 0){
				myInnerHtml("EN6_3","Dancer's Int",0);
				myInnerHtml("EN6_4",'<select name="A3_Skill6_2"onChange="StAllCalc()"></select>',0);
				myInnerHtml("EN6_5","Dancing Lessons",0);
				myInnerHtml("EN6_6",'<select name="A3_Skill6_3"onChange="StAllCalc()"></select>',0);

				for(i=1;i<=180;i++)
					document.calcForm.A3_Skill6_2.options[i-1] = new Option(i,i);
				for(i=1;i<=10;i++)
					document.calcForm.A3_Skill6_3.options[i-1] = new Option(i,i);
				SWs3sw[4] = 1;
				document.calcForm.A3_Skill6_2.value = n_A_PassSkill3[26];
				document.calcForm.A3_Skill6_3.value = n_A_PassSkill3[36];
			}
		}else{
			SWs3sw[6] = 0;
			myInnerHtml("EN6_3","-",0);
			myInnerHtml("EN6_4","-",0);
			myInnerHtml("EN6_5","",0);
			myInnerHtml("EN6_6","",0);
		}

		if(n_A_PassSkill3[11] != 0){
			if(SWs3sw[11] == 0){
				myInnerHtml("EN11_2",
					"<br>Controller's Stats: "+
					'<select name="A3_Skill11_STR"onChange="StAllCalc()"></select>'+
					'<select name="A3_Skill11_AGI"onChange="StAllCalc()"></select>'+
					'<select name="A3_Skill11_VIT"onChange="StAllCalc()"></select>'+
					'<select name="A3_Skill11_INT"onChange="StAllCalc()"></select>'+
					'<select name="A3_Skill11_DEX"onChange="StAllCalc()"></select>'+
					'<select name="A3_Skill11_LUK"onChange="StAllCalc()"></select>',0);;

				document.calcForm.A3_Skill11_STR.options[0] = new Option("STR",0);
				document.calcForm.A3_Skill11_AGI.options[0] = new Option("AGI",0);
				document.calcForm.A3_Skill11_VIT.options[0] = new Option("VIT",0);
				document.calcForm.A3_Skill11_INT.options[0] = new Option("INT",0);
				document.calcForm.A3_Skill11_DEX.options[0] = new Option("DEX",0);
				document.calcForm.A3_Skill11_LUK.options[0] = new Option("LUK",0);

				for(i=1;i<=99;i++){
					document.calcForm.A3_Skill11_STR.options[i] = new Option(i,i);
					document.calcForm.A3_Skill11_AGI.options[i] = new Option(i,i);
					document.calcForm.A3_Skill11_VIT.options[i] = new Option(i,i);
					document.calcForm.A3_Skill11_INT.options[i] = new Option(i,i);
					document.calcForm.A3_Skill11_DEX.options[i] = new Option(i,i);
					document.calcForm.A3_Skill11_LUK.options[i] = new Option(i,i);
				}
				SWs3sw[11] = 1;
				document.calcForm.A3_Skill11_STR.value = n_A_PassSkill3[12];
				document.calcForm.A3_Skill11_AGI.value = n_A_PassSkill3[13];
				document.calcForm.A3_Skill11_VIT.value = n_A_PassSkill3[14];
				document.calcForm.A3_Skill11_INT.value = n_A_PassSkill3[15];
				document.calcForm.A3_Skill11_DEX.value = n_A_PassSkill3[16];
				document.calcForm.A3_Skill11_LUK.value = n_A_PassSkill3[17];
			}
		}else{
			SWs3sw[11] = 0;
			myInnerHtml("EN11_2","",0);
		}

	}

	function Click_Skill4SW()
	{
		n_Skill4SW = document.calcForm.A4_SKILLSW.checked;

		if(n_Skill4SW){
			name_CS4SW_SKILL = ["Battle Orders","Great Leadership","Wounds of Glory","Soul of Cold","Sharp Hawk Eyes"];
			html_CS4SW_SKILL = new Array();
			for(i=0;i<=4;i++)
				myInnerHtml("EN4"+i+"_1",name_CS4SW_SKILL[i],0);
			html_CS4SW_SKILL[0] = '<input type="checkbox" name="A3_Skill40"onClick="calc()">';
			html_CS4SW_SKILL[1] = '<select name="A3_Skill41"onChange="StAllCalc()"></select>';
			html_CS4SW_SKILL[2] = '<select name="A3_Skill42"onChange="StAllCalc()"></select>';
			html_CS4SW_SKILL[3] = '<select name="A3_Skill43"onChange="StAllCalc()"></select>';
			html_CS4SW_SKILL[4] = '<select name="A3_Skill44"onChange="StAllCalc()"></select>';
			for(i=0;i<=4;i++)
				myInnerHtml("EN4"+i+"_2",html_CS4SW_SKILL[i],0);

			for(i=0;i<=5;i++){
				document.calcForm.A3_Skill41.options[i] = new Option(i,i);
				document.calcForm.A3_Skill42.options[i] = new Option(i,i);
				document.calcForm.A3_Skill43.options[i] = new Option(i,i);
				document.calcForm.A3_Skill44.options[i] = new Option(i,i);
			}
			document.calcForm.A3_Skill40.checked = n_A_PassSkill3[40];
			document.calcForm.A3_Skill41.value = n_A_PassSkill3[41];
			document.calcForm.A3_Skill42.value = n_A_PassSkill3[42];
			document.calcForm.A3_Skill43.value = n_A_PassSkill3[43];
			document.calcForm.A3_Skill44.value = n_A_PassSkill3[44];
		}
		else{
			for(i=0;i<=4;i++)
				myInnerHtml("EN4"+i+"_1","",0);
			for(i=0;i<=4;i++)
				myInnerHtml("EN4"+i+"_2","",0);
		}
	}

	function Click_Skill5SW()
	{
		n_Skill5SW = document.calcForm.A5_SKILLSW.checked;

		if(n_Skill5SW){
			name_CS5SW_SKILL = ["All Stats+20","HP+100%","SP+100%","ATK+100%","HIT+50 & FLEE+50"];
			html_CS5SW_SKILL = new Array();
			for(i=0;i<=4;i++)
				myInnerHtml("EN5"+i+"_1",name_CS5SW_SKILL[i],0);
			html_CS5SW_SKILL[0] = '<input type="checkbox" name="A5_Skill0"onClick="calc()">';
			html_CS5SW_SKILL[1] = '<input type="checkbox" name="A5_Skill1"onClick="calc()">';
			html_CS5SW_SKILL[2] = '<input type="checkbox" name="A5_Skill2"onClick="calc()">';
			html_CS5SW_SKILL[3] = '<input type="checkbox" name="A5_Skill3"onClick="calc()">';
			html_CS5SW_SKILL[4] = '<input type="checkbox" name="A5_Skill4"onClick="calc()">';
			for(i=0;i<=4;i++)
				myInnerHtml("EN5"+i+"_2",html_CS5SW_SKILL[i],0);

			document.calcForm.A5_Skill0.checked = n_A_PassSkill5[0];
			document.calcForm.A5_Skill1.checked = n_A_PassSkill5[1];
			document.calcForm.A5_Skill2.checked = n_A_PassSkill5[2];
			document.calcForm.A5_Skill3.checked = n_A_PassSkill5[3];
			document.calcForm.A5_Skill4.checked = n_A_PassSkill5[4];
		}
		else{
			for(i=0;i<=4;i++)
				myInnerHtml("EN5"+i+"_1","",0);
			for(i=0;i<=4;i++)
				myInnerHtml("EN5"+i+"_2","",0);
		}
	}

	function Click_Skill6SW()
	{
		n_Skill6SW = document.calcForm.A6_SKILLSW.checked;

		if(n_Skill6SW){
			myInnerHtml("EN60_1",'<select name="A6_Skill0"onChange="StAllCalc()"></select>',0);
			myInnerHtml("EN60_2",'<select name="A6_Skill1"onChange="StAllCalc()"></select>',0);

			document.calcForm.A6_Skill0.options[0] = new Option("Volcano",0);
			document.calcForm.A6_Skill0.options[1] = new Option("Deluge",1);
			document.calcForm.A6_Skill0.options[2] = new Option("Whirlwind",2);
			for(i=0;i<=5;i++)
				document.calcForm.A6_Skill1.options[i] = new Option(i,i);

			myInnerHtml("EN61_1","Murderer Bonus",0);
			myInnerHtml("EN61_2",'<select name="A6_Skill2"onChange="StAllCalc()"></select>',0);
			document.calcForm.A6_Skill2.options[0] = new Option("None",0);
			document.calcForm.A6_Skill2.options[1] = new Option("ALL+3",1);
			document.calcForm.A6_Skill2.options[2] = new Option("ALL+5",2);

			myInnerHtml("EN62_1","Anolian C(IC1)",0);
			myInnerHtml("EN62_2",'<input type="checkbox" name="A6_Skill3"onClick="calc()">',0);

			document.calcForm.A6_Skill0.value = n_A_PassSkill6[0];
			document.calcForm.A6_Skill1.value = n_A_PassSkill6[1];
			document.calcForm.A6_Skill2.value = n_A_PassSkill6[2];
			document.calcForm.A6_Skill3.checked = n_A_PassSkill6[3];
		}
		else{
			myInnerHtml("EN60_1","",0);
			myInnerHtml("EN60_2","",0);
			myInnerHtml("EN61_1","",0);
			myInnerHtml("EN61_2","",0);
			myInnerHtml("EN62_1","",0);
			myInnerHtml("EN62_2","",0);
		}
	}

	function Click_Skill7SW()
	{
		n_Skill7SW = document.calcForm.A7_SKILLSW.checked;

		if(n_Skill7SW){
			myInnerHtml("EN70_1","Sesame Pastry (HIT+30)",0);
			myInnerHtml("EN70_2",'<input type="checkbox" name="A7_Skill0"onClick="calc()">',0);

			myInnerHtml("EN71_1","Honey Pastry (FLEE+30)",0);
			myInnerHtml("EN71_2",'<input type="checkbox" name="A7_Skill1"onClick="calc()">',0);

			myInnerHtml("EN72_1","Rainbow Cake (ATK/MATK+10)",0);
			myInnerHtml("EN72_2",'<input type="checkbox" name="A7_Skill2"onClick="calc()">',0);

			myInnerHtml("EN79_1","Box of Resentment (ATK+20)",0);
			myInnerHtml("EN79_2",'<input type="checkbox" name="A7_Skill9"onClick="calc()">',0);

			myInnerHtml("EN710_1","Box of Drowsiness (MATK+20)",0);
			myInnerHtml("EN710_2",'<input type="checkbox" name="A7_Skill10"onClick="calc()">',0);

			myInnerHtml("EN73",'<select name="A7_Skill3"onChange="StAllCalc()"></select> ',0);
			myInnerHtml("EN74",'<select name="A7_Skill4"onChange="StAllCalc()"></select> ',0);
			myInnerHtml("EN75",'<select name="A7_Skill5"onChange="StAllCalc()"></select> ',0);
			myInnerHtml("EN76",'<select name="A7_Skill6"onChange="StAllCalc()"></select> ',0);
			myInnerHtml("EN77",'<select name="A7_Skill7"onChange="StAllCalc()"></select> ',0);
			myInnerHtml("EN78",'<select name="A7_Skill8"onChange="StAllCalc()"></select> ',0);

			document.calcForm.A7_Skill3.options[0] = new Option("STR+Food",0);
			document.calcForm.A7_Skill4.options[0] = new Option("AGI+Food",0);
			document.calcForm.A7_Skill5.options[0] = new Option("VIT+Food",0);
			document.calcForm.A7_Skill6.options[0] = new Option("INT+Food",0);
			document.calcForm.A7_Skill7.options[0] = new Option("DEX+Food",0);
			document.calcForm.A7_Skill8.options[0] = new Option("LUK+Food",0);

			for(i=1;i<=10;i++){
				document.calcForm.A7_Skill3.options[i] = new Option("+"+i,i);
				document.calcForm.A7_Skill4.options[i] = new Option("+"+i,i);
				document.calcForm.A7_Skill5.options[i] = new Option("+"+i,i);
				document.calcForm.A7_Skill6.options[i] = new Option("+"+i,i);
				document.calcForm.A7_Skill7.options[i] = new Option("+"+i,i);
				document.calcForm.A7_Skill8.options[i] = new Option("+"+i,i);
			}

			document.calcForm.A7_Skill0.checked = n_A_PassSkill7[0];
			document.calcForm.A7_Skill1.checked = n_A_PassSkill7[1];
			document.calcForm.A7_Skill2.checked = n_A_PassSkill7[2];
			document.calcForm.A7_Skill3.value = n_A_PassSkill7[3];
			document.calcForm.A7_Skill4.value = n_A_PassSkill7[4];
			document.calcForm.A7_Skill5.value = n_A_PassSkill7[5];
			document.calcForm.A7_Skill6.value = n_A_PassSkill7[6];
			document.calcForm.A7_Skill7.value = n_A_PassSkill7[7];
			document.calcForm.A7_Skill8.value = n_A_PassSkill7[8];
			document.calcForm.A7_Skill9.checked = n_A_PassSkill7[2];
			document.calcForm.A7_Skill10.checked = n_A_PassSkill7[2];
		}
		else{
			myInnerHtml("EN70_1","",0);
			myInnerHtml("EN70_2","",0);
			myInnerHtml("EN71_1","",0);
			myInnerHtml("EN71_2","",0);
			myInnerHtml("EN72_1","",0);
			myInnerHtml("EN72_2","",0);
			for(i=73;i<=78;i++)
				myInnerHtml("EN"+i,"",0);
			myInnerHtml("EN79_1","",0);
			myInnerHtml("EN79_2","",0);
			myInnerHtml("EN710_1","",0);
			myInnerHtml("EN710_2","",0);
		}
	}


	function Click_IjyouSW()
	{
		n_IjyouSW = document.calcForm.B_IJYOUSW.checked;

		if(n_IjyouSW){
			name_ISW_SKILL = ["Provoke (Non Undead)","Quagmire","Poison","Blind","Frozen (Non Undead)","Blessing (Demon/Undead)","Lex Aeterna","Stun","Sleep","Stone","Curse","Agility Down","Signum Crucis","Divest Weapon","Divest Shield","Divest Armor","Divest Helm","Fiber Lock","Mind Breaker","Slow Grace","Down Tempo","Power Up","Agility Up","Eska","Eske","Change Element (Monster Skill)","Elemental Change (Sage Skill)","Flying"];
			html_ISW_SKILL = new Array();
			for(i=0;i<=20;i++)
				myInnerHtml("BI"+i+"_1",name_ISW_SKILL[i],0);
			if(Taijin==0){
				for(i=21;i<=27;i++)
					myInnerHtml("BI"+i+"_1",name_ISW_SKILL[i],0);
			}else{
				myInnerHtml("BI27_1",name_ISW_SKILL[27],0);
			}
			html_ISW_SKILL[0] = '<select name="B_IJYOU0"onChange="calc()"></select>';
			html_ISW_SKILL[1] = '<select name="B_IJYOU1"onChange="calc()"></select>';
			html_ISW_SKILL[2] = '<input type="checkbox" name="B_IJYOU2"onClick="calc()">';
			html_ISW_SKILL[3] = '<input type="checkbox" name="B_IJYOU3"onClick="calc()">';
			html_ISW_SKILL[4] = '<input type="checkbox" name="B_IJYOU4"onClick="calc()">';
			html_ISW_SKILL[5] = '<input type="checkbox" name="B_IJYOU5"onClick="calc()">';
			html_ISW_SKILL[6] = '<input type="checkbox" name="B_IJYOU6"onClick="calc()">';
			html_ISW_SKILL[7] = '<input type="checkbox" name="B_IJYOU7"onClick="calc()">';
			html_ISW_SKILL[8] = '<input type="checkbox" name="B_IJYOU8"onClick="calc()">';
			html_ISW_SKILL[9] = '<input type="checkbox" name="B_IJYOU9"onClick="calc()">';
			html_ISW_SKILL[10] = '<input type="checkbox" name="B_IJYOU10"onClick="calc()">';
			html_ISW_SKILL[11] = '<select name="B_IJYOU11"onChange="calc()"></select>';
			html_ISW_SKILL[12] = '<select name="B_IJYOU12"onChange="calc()"></select>';
			html_ISW_SKILL[13] = '<input type="checkbox" name="B_IJYOU13"onClick="calc()">';
			html_ISW_SKILL[14] = '<input type="checkbox" name="B_IJYOU14"onClick="calc()">';
			html_ISW_SKILL[15] = '<input type="checkbox" name="B_IJYOU15"onClick="calc()">';
			html_ISW_SKILL[16] = '<input type="checkbox" name="B_IJYOU16"onClick="calc()">';
			html_ISW_SKILL[17] = '<input type="checkbox" name="B_IJYOU17"onClick="calc()">';
			html_ISW_SKILL[18] = '<select name="B_IJYOU18"onChange="calc()"></select>';
			html_ISW_SKILL[19] = '<input type="checkbox" name="B_IJYOU19"onClick="calc()">';
			html_ISW_SKILL[20] = '<input type="checkbox" name="B_IJYOU20"onClick="calc()">';
			html_ISW_SKILL[27] = '<select name="B_IJYOU27"onChange="calc()"></select>';

			for(i=0;i<=20;i++)
				myInnerHtml("BI"+i+"_2",html_ISW_SKILL[i],0);

			myInnerHtml("BI27_2",html_ISW_SKILL[27],0);

			for(i=0;i<=10;i++){
				document.calcForm.B_IJYOU0.options[i] = new Option(i,i);
				document.calcForm.B_IJYOU11.options[i] = new Option(i,i);
				document.calcForm.B_IJYOU12.options[i] = new Option(i,i);
			}
			for(i=0;i<=5;i++){
				document.calcForm.B_IJYOU1.options[i] = new Option(i,i);
				document.calcForm.B_IJYOU18.options[i] = new Option(i,i);
				document.calcForm.B_IJYOU27.options[i] = new Option(i,i);
			}
			if(Taijin==0){
				myInnerHtml("BI21_2",'<input type="checkbox" name="B_IJYOU21"onClick="calc()">',0);
				myInnerHtml("BI22_2",'<input type="checkbox" name="B_IJYOU22"onClick="calc()">',0);
				myInnerHtml("BI23_2",'<input type="checkbox" name="B_IJYOU23"onClick="calc()">',0);
				myInnerHtml("BI24_2",'<input type="checkbox" name="B_IJYOU24"onClick="calc()">',0);
				ZoHe =[["None","Neutral 1","Neutral 2","Neutral 3","Neutral 4","Water 1","Water 2","Water 3","Water 4","Earth 1","Eart 2","Earth 3","Earth 4","Fire 1","Fire 2","Fire 3","Fire 4","Wind 1","Wind 2","Wind 3","Wind 4","Poison 1","Poison 2","Poison 3","Poison 4","Holy 1","Holy 2","Holy 3","Holy 4","Shadow 1","Shadow 2","Shadow 3","Shadow 4","Ghost 1","Ghost 2","Ghost 3","Ghost 4","Undead 1","Undead 2","Undead 3","Undead 4"],
					[0,1,2,3,4,11,12,13,14,21,22,23,24,31,32,33,34,41,42,43,44,51,52,53,54,61,62,63,64,71,72,73,74,81,82,83,84,91,92,93,94]];
				myInnerHtml("BI25_2",'<select name="B_IJYOU25"onChange="calc()"></select>',0);

				for(i=0;i<=40;i++)
					document.calcForm.B_IJYOU25.options[i] = new Option(ZoHe[0][i],ZoHe[1][i]);
				ZoHe2 =["None","Water","Earth","Fire","Wind"];
				myInnerHtml("BI26_2",'<select name="B_IJYOU26"onChange="calc()"></select>',0);

				for(i=0;i<=4;i++)
					document.calcForm.B_IJYOU26.options[i] = new Option(ZoHe2[i],i);
			}


			document.calcForm.B_IJYOU0.value = n_B_IJYOU[0];
			document.calcForm.B_IJYOU1.value = n_B_IJYOU[1];
			document.calcForm.B_IJYOU2.checked = n_B_IJYOU[2];
			document.calcForm.B_IJYOU3.checked = n_B_IJYOU[3];
			document.calcForm.B_IJYOU4.checked = n_B_IJYOU[4];
			document.calcForm.B_IJYOU5.checked = n_B_IJYOU[5];
			document.calcForm.B_IJYOU6.checked = n_B_IJYOU[6];
			document.calcForm.B_IJYOU7.checked = n_B_IJYOU[7];
			document.calcForm.B_IJYOU8.checked = n_B_IJYOU[8];
			document.calcForm.B_IJYOU9.checked = n_B_IJYOU[9];
			document.calcForm.B_IJYOU10.checked = n_B_IJYOU[10];
			document.calcForm.B_IJYOU11.value = n_B_IJYOU[11];
			document.calcForm.B_IJYOU12.value = n_B_IJYOU[12];
			document.calcForm.B_IJYOU13.checked = n_B_IJYOU[13];
			document.calcForm.B_IJYOU14.checked = n_B_IJYOU[14];
			document.calcForm.B_IJYOU15.checked = n_B_IJYOU[15];
			document.calcForm.B_IJYOU16.checked = n_B_IJYOU[16];
			document.calcForm.B_IJYOU17.checked = n_B_IJYOU[17];
			document.calcForm.B_IJYOU18.value = n_B_IJYOU[18];
			document.calcForm.B_IJYOU19.checked = n_B_IJYOU[19];
			document.calcForm.B_IJYOU20.checked = n_B_IJYOU[20];
			if(Taijin==0){
				document.calcForm.B_IJYOU21.checked = n_B_IJYOU[21];
				document.calcForm.B_IJYOU22.checked = n_B_IJYOU[22];
				document.calcForm.B_IJYOU23.checked = n_B_IJYOU[23];
				document.calcForm.B_IJYOU24.checked = n_B_IJYOU[24];
				document.calcForm.B_IJYOU25.value = n_B_IJYOU[25];
				document.calcForm.B_IJYOU26.value = n_B_IJYOU[26];
			}
			document.calcForm.B_IJYOU27.value = n_B_IJYOU[27];
		}
		else{
			for(i=0;i<=20;i++){
				myInnerHtml("BI"+i+"_1","",0);
				myInnerHtml("BI"+i+"_2","",0);
			}
			if(Taijin==0){
				for(i=21;i<=26;i++){
					myInnerHtml("BI"+i+"_1","",0);
					myInnerHtml("BI"+i+"_2","",0);
				}
			}
			myInnerHtml("BI27_1","",0);
			myInnerHtml("BI27_2","",0);
		}
	}

	function ClickB_Enemy()
	{
		n_B = new Array();
		n_B2 = new Array();
		for(i=0;i<=22;i++){
			n_B[i] = MonsterOBJ[document.calcForm.B_Enemy.value][i];
			n_B2[i] = n_B[i];
		}

		if(Taijin){
			n_B[3] = eval(document.calcForm.B_element.value);
			n_B[5] = eval(document.calcForm.B_LV.value);
			n_B[7] = eval(document.calcForm.B_VIT.value);
			n_B[8] = eval(document.calcForm.B_AGI.value);
			n_B[9] = eval(document.calcForm.B_INT.value);
			n_B[11] = eval(document.calcForm.B_LUK.value);
			n_B[14] = eval(document.calcForm.B_DEF.value);
			n_B[15] = eval(document.calcForm.B_MDEF.value);

			JobHP_A = new Array(0,0.7,0.5,0.4,0.5,0.3,0.4,1.5,1.1,0.75,0.85,0.55,0.9,1.1,0.85,0.9,0.75,0.75,0.75,0.9,0,1.5,1.1,0.75,0.85,0.55,0.9,1.1,0.85,0.9,0.75,0.75,0.75,0.9);
			JobHP_B = new Array(5,  5,  5,  5,  5,  5,  5,  5,  5,   5,   5,   5,  5,  7,   5,6.5,   3,   3,   5,  5,5,  5,  5,   5,   5,   5,  5,  7,   5,6.5,   3,   3,   5,  5);


			w = 0;
			for(i=2;i<=n_B[5];i++)
				w += Math.round(JobHP_A[n_B[1]] * i);
			w = (JobHP_B[n_B[1]] * n_B[5]) + 35 + w;

			if(n_B[1] > 20)
				w = Math.floor(w *125 /100);
			n_B[6] = Math.floor(w * (100 + n_B[7]) / 100);
			n_B[6] += eval(document.calcForm.B_TAISEI11.value);
			n_B[6] = Math.floor(n_B[6] * (100 + eval(document.calcForm.B_TAISEI12.value)) /100);
			myInnerHtml("B_HP",n_B[6],0);


			n_B_DEF2[2] = Math.floor(n_B[7] * 0.5) + Math.floor(n_B[7] * 0.3);
			n_B_DEF2[0] = Math.floor(n_B[7] * 0.5) + Math.floor(n_B[7] * n_B[7] / 150) -1;
			if(n_B_DEF2[2] > n_B_DEF2[0])
				n_B_DEF2[0] = n_B_DEF2[2];
			w = eval(document.calcForm.B_TAISEI4.value);
			if(w){
				n_B_DEF2[2] *= (1 + 0.05 * w);
				n_B_DEF2[0] *= (1 + 0.05 * w);
			}
			n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) /2);
		}else{

			n_B2[23] = n_B[7];
			n_B2[24] = n_B[7] + (Math.floor(n_B[7]/20) * Math.floor(n_B[7]/20) -1);
			if(n_B2[23] > n_B2[24])
				n_B2[24] = n_B2[23];
		}
		n_B2[25] = Math.floor(n_B[7] / 2) + n_B[9];
		n_B2[26] = n_B[5] + n_B[10];
		n_B2[27] = n_B[5] + n_B[8];

		if(n_IjyouSW){
			n_B_IJYOU[0] = eval(document.calcForm.B_IJYOU0.value);
			n_B_IJYOU[1] = eval(document.calcForm.B_IJYOU1.value);
			n_B_IJYOU[2] = document.calcForm.B_IJYOU2.checked;
			n_B_IJYOU[3] = document.calcForm.B_IJYOU3.checked;
			n_B_IJYOU[4] = document.calcForm.B_IJYOU4.checked;
			n_B_IJYOU[5] = document.calcForm.B_IJYOU5.checked;
			n_B_IJYOU[6] = document.calcForm.B_IJYOU6.checked;
			n_B_IJYOU[7] = document.calcForm.B_IJYOU7.checked;
			n_B_IJYOU[8] = document.calcForm.B_IJYOU8.checked;
			n_B_IJYOU[9] = document.calcForm.B_IJYOU9.checked;
			n_B_IJYOU[10] = document.calcForm.B_IJYOU10.checked;
			n_B_IJYOU[11] = eval(document.calcForm.B_IJYOU11.value);
			n_B_IJYOU[12] = eval(document.calcForm.B_IJYOU12.value);
			n_B_IJYOU[13] = document.calcForm.B_IJYOU13.checked;
			n_B_IJYOU[14] = document.calcForm.B_IJYOU14.checked;
			n_B_IJYOU[15] = document.calcForm.B_IJYOU15.checked;
			n_B_IJYOU[16] = document.calcForm.B_IJYOU16.checked;
			n_B_IJYOU[17] = document.calcForm.B_IJYOU17.checked;
			n_B_IJYOU[18] = eval(document.calcForm.B_IJYOU18.value);
			n_B_IJYOU[19] = document.calcForm.B_IJYOU19.checked;
			n_B_IJYOU[20] = document.calcForm.B_IJYOU20.checked;
			if(Taijin==0){
				n_B_IJYOU[21] = document.calcForm.B_IJYOU21.checked;
				n_B_IJYOU[22] = document.calcForm.B_IJYOU22.checked;
				n_B_IJYOU[23] = document.calcForm.B_IJYOU23.checked;
				n_B_IJYOU[24] = document.calcForm.B_IJYOU24.checked;
				n_B_IJYOU[25] = eval(document.calcForm.B_IJYOU25.value);
				n_B_IJYOU[26] = eval(document.calcForm.B_IJYOU26.value);
			}
			n_B_IJYOU[27] = eval(document.calcForm.B_IJYOU27.value);
		}

		if(n_B_IJYOU[25])
			n_B[3] = n_B_IJYOU[25];
		if(n_B_IJYOU[26])
			n_B[3] = n_B_IJYOU[26] * 10 + (n_B[3] % 10);

		if(n_B_IJYOU[21]){
			n_B[12] = n_B[12] * 3;
			n_B[13] = n_B[13] * 3;
			n_B[10] = n_B[10] * 3;
		}
		if(n_B_IJYOU[22])
			n_B[8] = n_B[8] * 3;

		if(n_B_IJYOU[1]){
			if(Taijin){
				w2 = n_B_IJYOU[1] * 5;
				w = Math.floor(n_B[8] / 4);
			}else{
				w2 = n_B_IJYOU[1] * 10
				w = Math.floor(n_B[8] / 2);
			}
			if(w > w2)
				n_B[8] -= w2;
			else
				n_B[8] -= w;
			if(Taijin)
				w = Math.floor(n_B[10] / 4);
			else
				w = Math.floor(n_B[10] / 2);
			if(w > w2)
				n_B[10] -= w2;
			else
				n_B[10] -= w;
		}

		if(n_B[19] == 0){
			if(n_B_IJYOU[5] && (n_B[2]==6||n_B[3]>=90)){
				n_B[10] = n_B[10] - Math.floor(n_B[10] /2);
				n_B[9] = n_B[9] - Math.floor(n_B[9] /2);
			}
			if(n_B_IJYOU[10]){
				n_B[11] = 0;
				n_B[12] -= Math.floor(n_B[12] * 25 /100);
				n_B[13] -= Math.floor(n_B[13] * 25 /100);
			}
			if(n_B_IJYOU[11]){
				n_B[8] -= (n_B_IJYOU[11] + 2);
				if(n_B[8] < 0)
					n_B[8]=0;
			}
		}

		if(n_B_IJYOU[15] && Taijin==0)
			n_B[7] -= Math.floor(n_B[7] * 40 /100);
		if(n_B_IJYOU[16] && Taijin==0)
			n_B[9] -= Math.floor(n_B[9] * 40 /100);

		if(Taijin==0){

			n_B[23] = n_B[7];
			n_B[24] = n_B[7] + (Math.floor(n_B[7]/20) * Math.floor(n_B[7]/20) -1);
			if(n_B[23] > n_B[24])
				n_B[24] = n_B[23];
		}
		n_B[25] = Math.floor(n_B[7] / 2) + n_B[9];
		n_B[26] = n_B[5] + n_B[10];
		n_B[27] = n_B[5] + n_B[8];

		xiATK=0;
		xiDEF=0;
		if(n_B[19] == 0){
			if(n_B_IJYOU[0]!=0 && n_B[3]<90){
				xiDEF += 5 + n_B_IJYOU[0] * 5;
				xiATK += 2 + n_B_IJYOU[0] * 3;
			}
			if(n_B_IJYOU[2]){
				n_B[14] -= Math.floor(n_B[14] * 25 / 100);
				n_B[23] -= Math.floor(n_B[23] * 25 / 100);
				n_B[24] -= Math.floor(n_B[24] * 25 / 100);
			}
			if(n_B_IJYOU[3]){
				n_B[26] -= 25;
				if(n_B[26] < 1)
					n_B[26] = 1;
				n_B[27] -= Math.floor(n_B[27] * 25 / 100);
			}
		}
		if(Taijin==0){
			if(n_B_IJYOU[24]){
				xiDEF += 50;
				xiATK += 300;
			}
			if(n_B_IJYOU[27])
				xiDEF += 5 * n_B_IJYOU[27];
		}
		if(xiDEF > 100)
			xiDEF=100;
		if(Taijin==0)
			n_B[14] -= Math.floor(n_B[14] * xiDEF /100);
		n_B[23] -= Math.floor(n_B[23] * xiDEF /100);
		n_B[24] -= Math.floor(n_B[24] * xiDEF /100);
		n_B[12] += Math.floor(n_B[12] * xiATK /100);
		n_B[13] += Math.floor(n_B[13] * xiATK /100);

		if(n_B_IJYOU[13] && Taijin==0){
			n_B[12] -= Math.floor(n_B[12] * 25 /100);
			n_B[13] -= Math.floor(n_B[13] * 25 /100);
		}
		if(n_B_IJYOU[14] && Taijin==0)
			n_B[14] -= Math.floor(n_B[14] * 15 /100);
		if(n_B[19] == 0){
			if(n_B_IJYOU[17]){
				n_B[27] -= 50;
				if(n_B[27] < 1)
					n_B[27] = 1;
			}

			if(n_B_IJYOU[18] && n_B[3]<90)
				n_B[25] -= Math.floor(n_B[25] * (n_B_IJYOU[18] * 12) / 100);
			if(n_B_IJYOU[4] && n_B[2]!=1){
				n_B[3] = 11;
				n_B[14] -= Math.floor(n_B[14] * 50 /100);
				n_B[23] -= Math.floor(n_B[23] * 50 /100);
				n_B[24] -= Math.floor(n_B[24] * 50 /100);
				n_B[15] += Math.floor(n_B[15] * 25 /100);
				n_B[27] = -19;
			}
			if(n_B_IJYOU[7] || n_B_IJYOU[8])
				n_B[27] = -19;
			if(n_B_IJYOU[9] && n_B[2]!=1){
				n_B[3] = 21;
				n_B[14] -= Math.floor(n_B[14] * 50 /100);
				n_B[23] -= Math.floor(n_B[23] * 50 /100);
				n_B[24] -= Math.floor(n_B[24] * 50 /100);
				n_B[15] += Math.floor(n_B[15] * 25 /100);
				n_B[27] = -19;
			}
		}

		if(Taijin==0){
			if(n_B_IJYOU[23]){
				n_B[24] += 90;
				n_B[25] = 90;
			}
		}
		if(n_B_IJYOU[20]){
			n_B[14] = 0;
			n_B[23] = 0;
			n_B[24] = 0;
		}
		if(n_B_IJYOU[12] && (n_B[2]==6||n_B[3]>=90))
			n_B[14] -= Math.floor(n_B[14] * (10 + n_B_IJYOU[12] * 4) /100);


		if(Taijin==0){
			w1_Exp = StPlusCard(120+n_B[2]);
			w1_Exp += StPlusCalc2(120+n_B[2]);
			if(n_A_JobSearch()==3 && CardNumSearch(452) && (n_B[2]==1 || n_B[2]==6))
				w1_Exp += 5;
			if(n_B[2] == 2 && n_A_JobSearch()==4 && CardNumSearch(453))
				w1_Exp += 5;
			if(w1_Exp != 0){
				n_B[16] = Math.floor(n_B[16] * (100+w1_Exp)/100);
				n_B[17] = Math.floor(n_B[17] * (100+w1_Exp)/100);
			}
			if(n_B[19]==0){
				if(n_Skill3SW)
					n_A_PassSkill3[8] = eval(document.calcForm.A3_Skill8.value);
				if(n_A_PassSkill3[8]){
					n_B[16] = Math.floor(n_B[16] * (125 + 11 * n_A_PassSkill3[8]) /100);
					n_B[17] = Math.floor(n_B[17] * (125 + 11 * n_A_PassSkill3[8]) /100);
				}
			}
		}

		n_B[21] = n_B[27] + 20;
		n_B[22] = n_B[26] + 75;
		if(Taijin == 0)
		{
			myInnerHtml("B_AA"," + ",0);
			myInnerHtml("B_AB"," + ",0);
			wIJ = [6,12,13,21,22,14,15,23,25];
			wIJ2 = [16,17];
			wFront = "<Font color='BLUE'><B>";
			wFront2 = "<Font color='RED'><B>";
			wBack = "</B></Font>";

			for(i=0;i<=8;i++){
				wIJstr = n_B[wIJ[i]];
				if(n_B[wIJ[i]] < n_B2[wIJ[i]])
					wIJstr =  wFront + n_B[wIJ[i]] + wBack;
				if(n_B[wIJ[i]] > n_B2[wIJ[i]])
					wIJstr =  wFront2 + n_B[wIJ[i]] + wBack;
				myInnerHtml("B_"+wIJ[i],wIJstr,0);
			}
			for(i=0;i<=1;i++){
				wIJstr = n_B[wIJ2[i]];
				if(n_B[wIJ2[i]] < n_B2[wIJ2[i]])
					wIJstr =  wFront2 + n_B[wIJ2[i]] + wBack;
				if(n_B[wIJ2[i]] > n_B2[wIJ2[i]])
					wIJstr =  wFront + n_B[wIJ2[i]] + wBack;
				myInnerHtml("B_"+wIJ2[i],wIJstr,0);
			}

			myInnerHtml("B_2",SyuzokuOBJ[n_B[2]],0);
			w = Math.floor(n_B[3] / 10);
			if(n_B[3] != n_B2[3])
				myInnerHtml("B_3",wFront2 +(elementOBJ[w] + n_B[3] % 10)+ wBack,0);
			else
				myInnerHtml("B_3",(elementOBJ[w] + n_B[3] % 10),0);
			myInnerHtml("B_4",SizeOBJ[n_B[4]],0);
		}
		else{
			n_B_FLEE += eval(document.calcForm.B_TAISEI7.value);
			n_Ses = document.calcForm.B_Ses.checked;
			if(n_Ses){
				n_B_FLEE = Math.floor(n_B_FLEE *0.8);
			}
		}

		n_B_DEF2 = [0,0,0];
		n_B_DEF2[2] = n_B[23];
		n_B_DEF2[0] = n_B[24];
		n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) /2);
		n_B_MDEF2 = n_B[25];
		n_B_HIT = n_B[26];
		n_B_FLEE = n_B[27];
	}

	function calc()
	{
		StAllCalc();




		sizeModifier = weaponsize[n_A_WeaponType][n_B[4]];
		if(SkillSearch(78)){
			if((n_A_WeaponType==4 || n_A_WeaponType==5) && n_B[4]==1)
				sizeModifier = 1;
		}
		if(SkillSearch(153) || n_A_PassSkill2[7])
			sizeModifier = 1;

		if(cardOBJ[n_A_card[0]][0]==32||cardOBJ[n_A_card[1]][0]==32||cardOBJ[n_A_card[2]][0]==32||cardOBJ[n_A_card[3]][0]==32||cardOBJ[n_A_card[4]][0]==32||cardOBJ[n_A_card[5]][0]==32||cardOBJ[n_A_card[6]][0]==32||cardOBJ[n_A_card[7]][0]==32)
			sizeModifier = 1;


		impositioMagnus = n_A_PassSkill2[2] *5;


		w_HIT = n_A_HIT + 80 - (n_B_FLEE);
		if(SkillSearch(148))
			w_HIT = Math.floor(w_HIT * (100 + 2 * SkillSearch(148))/100);
		if(n_A_ActiveSkill==70 || n_A_ActiveSkill==6){
			w_HIT *= 1+n_A_ActiveSkillLV *0.05;
		}
		if((n_A_ActiveSkill==83 || n_A_ActiveSkill==388)&& SkillSearch(381)){
			w_HIT *= 1.5;
		}
		if(n_A_ActiveSkill==7){
			w_HIT *= 1+n_A_ActiveSkillLV *0.1;
		}
		if(n_A_ActiveSkill==272){
			w_HIT *= (1 + n_A_ActiveSkillLV * 0.1);
		}
		if(n_A_ActiveSkill==337){
			w_HIT = 100;
		}
		if(n_A_ActiveSkill==384){
			w_HIT = 100;
		}
		if(SkillSearch(364)){
			w_HIT = 100;
		}
		if(w_HIT > 100){
			w_HIT = 100;
		}else if(w_HIT < 5){
			w_HIT = 5;
		}
		if(StPlusCalc2(86)+StPlusCard(86))
			w_HIT = w_HIT + (100 - w_HIT) * (StPlusCalc2(86)+StPlusCard(86)) / 100;

		w_HIT = Math.floor(w_HIT *100)/100;
		myInnerHtml("BattleHIT",w_HIT,0);

		if(n_A_ActiveSkill==272){
			n_A_CRI += 20;
		}
		w_Cri = n_A_CRI - n_B[11] * 0.2 - 0.1;
		if(n_B_IJYOU[8])
			w_Cri *= 2;
		if(w_Cri < 0){
			w_Cri = 0;
		}
		else if(w_Cri > 100){
			w_Cri = 100;
		}
		myInnerHtml("CRInum",Math.round(w_Cri *10) /10 +SubName[0],0);


		wBC3_3danHatudouRitu = 0;
		if(SkillSearch(187))
			wBC3_3danHatudouRitu = 30 - SkillSearch(187);


		wDA = SkillSearch(13) * 5;
		if(n_A_WeaponType != 1)
			wDA = 0;
		if(cardOBJ[n_A_card[0]][0]==43||cardOBJ[n_A_card[1]][0]==43||cardOBJ[n_A_card[2]][0]==43||cardOBJ[n_A_card[3]][0]==43||cardOBJ[n_A_card[4]][0]==43||cardOBJ[n_A_card[5]][0]==43||cardOBJ[n_A_card[6]][0]==43||cardOBJ[n_A_card[7]][0]==43){
			if(SkillSearch(13) > 1)
				wDA = SkillSearch(13) * 5;
			else
				wDA = 5;
		}
		if(ItemOBJ[n_A_Equip[2]][0]==570){
			if(SkillSearch(13) > 1)
				wDA = SkillSearch(13) * 5;
			else
				wDA = 10;
		}
		if(ItemOBJ[n_A_Equip[0]][0]==399||ItemOBJ[n_A_Equip[1]][0]==399)
			wDA = 25;
		if(n_A_WeaponType == 17)
			wDA = SkillSearch(425) * 5;

		w_HIT_DA = w_HIT;
		if(wDA != 0 && n_A_WeaponType != 17){
			w_HIT_DA = w_HIT_DA * (100 + SkillSearch(13)) /100;
			if(w_HIT_DA >= 100)
				w_HIT_DA=100;
		}

		w998A = 100 - wBC3_3danHatudouRitu;
		w998B = wBC3_3danHatudouRitu * w_HIT /100;
		w998C = wBC3_3danHatudouRitu - w998B;
		w998D = w998A * wDA /100;
		w998E = w998D * w_HIT_DA /100;
		w998F = w998D - w998E;
		w998G = (100-wBC3_3danHatudouRitu-w998D) * w_Cri /100;
		w998H = 100 - wBC3_3danHatudouRitu -w998D -w998G;
		w998I = w998H * w_HIT /100;
		w998J = w998H - w998I;
		w998K = w998B +w998E +w998G +w998I;
		w998L = 100 -w998K;





		w_FLEE = n_A_FLEE + 20 - (n_B_HIT);
		if(w_FLEE > 95){
			w_FLEE = 95;
		}else if(w_FLEE < 5){
			w_FLEE = 5;
		}
		if(Taijin==0)
			myInnerHtml("BattleFLEE",w_FLEE,0);

		n_A_workDEX = Math.floor(n_A_DEX * (1 + (n_A_WeaponLV - 1) * 0.2));

		n_A_DMG = [0,0,0];
		weaponAttack = [0,0,0];


		if(n_A_workDEX>=n_A_Weapon_ATK || SkillSearch(155)) // 155 = power maximize
			weaponAttack[2] = n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK + impositioMagnus)* sizeModifier);
		else
			weaponAttack[2] = n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK-1 + impositioMagnus)* sizeModifier);

		if(isRangedWeapon())
			weaponAttack[2] += Math.floor((ArrowOBJ[n_A_Arrow][0]-1) * sizeModifier);


		if(isRangedWeapon())
		{
			w1 = n_A_WeaponLV_overUpgradeBonusATK + Math.floor(n_A_Weapon_ATK * n_A_Weapon_ATK / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);
			w2 = n_A_WeaponLV_overUpgradeBonusATK + Math.floor(n_A_Weapon_ATK * n_A_workDEX / 100 * sizeModifier) + Math.floor(impositioMagnus * sizeModifier);

			w = Math.floor((ArrowOBJ[n_A_Arrow][0]-1) * sizeModifier);
			w1 += w;
			w2 += w;
			if(w1 > w2)w1 = w2;
			if(weaponAttack[2] < w1)weaponAttack[2] = w1;
		}




		if(isRangedWeapon())
		{
			weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_Weapon_ATK / 100 +impositioMagnus) * sizeModifier);
			w = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_workDEX / 100 + impositioMagnus) * sizeModifier);
			if(weaponAttack[0] > w)weaponAttack[0] = w;
		}
		else{
			if(n_A_workDEX >= n_A_Weapon_ATK)
				weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + impositioMagnus) * sizeModifier);
			else{

				if(SkillSearch(155))
					n_A_workDEX = n_A_Weapon_ATK;
				weaponAttack[0] = n_A_WeaponLV_Minplus + Math.floor((n_A_workDEX + impositioMagnus) * sizeModifier);
			}
		}


		weaponAttack[1] = (weaponAttack[0] + weaponAttack[2]) / 2;
		n_A_DMG[0]= baseATK + weaponAttack[0];
		n_A_DMG[1]= baseATK + weaponAttack[1];
		n_A_DMG[2]= baseATK + weaponAttack[2];
		myInnerHtml("BaseAttackCalc", baseATK, 0);
		myInnerHtml("MinWeaponAttackCalc", weaponAttack[0], 0);
		myInnerHtml("AvgWeaponAttackCalc", weaponAttack[1], 0);
		myInnerHtml("MaxWeaponAttackCalc", weaponAttack[2], 0);

		n_Enekyori=0;
		n_A_CriATK = [0,0,0];
		n_A_CriATK[1] = baseATK + (n_A_WeaponLV_Minplus + n_A_WeaponLV_overUpgradeBonusATK) /2 + Math.floor((n_A_Weapon_ATK + impositioMagnus)* sizeModifier);
		n_A_CriATK[0] = baseATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + impositioMagnus)* sizeModifier);
		n_A_CriATK[2] = baseATK + n_A_WeaponLV_overUpgradeBonusATK + Math.floor((n_A_Weapon_ATK + impositioMagnus)* sizeModifier);


		if(n_A_WeaponType==10)
		{
			n_Enekyori=1;
			n_A_CriATK[1] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
			n_A_CriATK[0] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
			n_A_CriATK[2] += Math.floor((ArrowOBJ[n_A_Arrow][0]) * sizeModifier);
		}


		BK_n_A_DMG = [0,0,0];
		BK_n_A_DMG[2] = n_A_DMG[2];
		BK_n_A_DMG[0] = n_A_DMG[0];
		BK_n_A_DMG[1] = n_A_DMG[1];

		ATKbai01();
		applySkillModifier(1,1);

		wCriTyuu=1;
		n_A_CriATK[1] = BattleCalc(n_A_CriATK[1],10);
		n_A_CriATK[0] = BattleCalc(n_A_CriATK[0],10);
		n_A_CriATK[2] = BattleCalc(n_A_CriATK[2],10);
		wCriTyuu=0;


		n_A_EDP_DMG = [0,0,0];
		n_A_EDP_DMG[2] = BattleCalcEDP(n_A_DMG[2],2);
		n_A_EDP_DMG[0] = BattleCalcEDP(n_A_DMG[0],0);
		n_A_EDP_DMG[1] = BattleCalcEDP(n_A_DMG[1],1);

		if(n_A_WeaponType == 11){
			wk = Math.floor(n_A_CriATK[1] * (0.01 + SkillSearch(13) * 0.02));
			wk2 = Math.floor((n_A_CriATK[1] + n_A_EDP_DMG[1]) * (0.01 + SkillSearch(13) * 0.02));
			if(n_A_WeaponLV_Minplus == n_A_WeaponLV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2]){
				myInnerHtml("CRIATK",(n_A_CriATK[1] + wk2 + n_A_EDP_DMG[1]) +"("+ (n_A_CriATK[1] + n_A_EDP_DMG[1]) +"+"+ wk2 +")",0);
			}else{
				w1 = Math.floor((n_A_CriATK[0] + n_A_EDP_DMG[0]) * (0.01 + SkillSearch(13) * 0.02));
				w2 = Math.floor((n_A_CriATK[2] + n_A_EDP_DMG[2]) * (0.01 + SkillSearch(13) * 0.02));
				myInnerHtml("CRIATK",(n_A_CriATK[0] + w1 + n_A_EDP_DMG[0])+" ~ "+(n_A_CriATK[2] + w2 + n_A_EDP_DMG[2]) +"("+ (n_A_CriATK[0] + n_A_EDP_DMG[0]) +" ~ "+ (n_A_CriATK[2] + n_A_EDP_DMG[2]) +"+"+ w1 +" ~ "+ w2 +")",0);
			}
			n_A_CriATK[1] += wk;
		}else{
			if(n_A_WeaponLV_Minplus == n_A_WeaponLV_overUpgradeBonusATK && n_A_EDP_DMG[0] == n_A_EDP_DMG[2])
				myInnerHtml("CRIATK",n_A_CriATK[1] + n_A_EDP_DMG[1],0);
			else
				myInnerHtml("CRIATK",(n_A_CriATK[0] + n_A_EDP_DMG[0]) +" ~ "+ (n_A_CriATK[2] + n_A_EDP_DMG[2]),0);
		}


		n_A_CriATK[2] += HitEDPplus(n_A_EDP_DMG[2]);
		n_A_CriATK[0] += HitEDPplus(n_A_EDP_DMG[0]);
		n_A_CriATK[1] += HitEDPplus(n_A_EDP_DMG[1]);

		BattleCalc999();
	}


	function BattleCalc(w_atk,w_2)
	{

		if(w_2==10)
			w_atk += n_A_WeaponLV_upgradeBonusATK;
		else
			w_atk=BattleCalc4(w_atk,w_2,0);

		if(w_atk < 1)w_atk = 1;


		if(n_A_WeaponType == 1 || n_A_WeaponType == 2)w_atk += 4 * SkillSearch(3);
		else if(n_A_WeaponType == 3)w_atk += 4 * SkillSearch(4);
		else if(n_A_WeaponType == 4 || n_A_WeaponType == 5)
		{
			if(SkillSearch(78) == 0)
				w_atk += 4 * SkillSearch(69);
			else
				w_atk += 5 * SkillSearch(69);

		}
		else if(n_A_WeaponType == 11)w_atk += 3 * SkillSearch(81);
		else if(n_A_WeaponType == 8)w_atk += 3 * SkillSearch(89);
		else if(n_A_WeaponType == 13 || n_A_WeaponType == 0)w_atk += 3 * SkillSearch(183);
		else if(n_A_WeaponType == 14)w_atk += 3 * SkillSearch(198);
		else if(n_A_WeaponType == 15)w_atk += 3 * SkillSearch(206);
		else if(n_A_WeaponType == 12)w_atk += 3 * SkillSearch(224);
		else if(n_A_WeaponType == 6 || n_A_WeaponType == 7)w_atk += 3 *SkillSearch(241);

		if(n_A_WeaponType == 0 && SkillSearch(329))
			w_atk += 10 * SkillSearch(329);

		if(n_A_PassSkill3[10] && n_A_WeaponLV == 4)
			w_atk += 50 + 25 * n_A_PassSkill3[10];


		if(n_B[2] == 6 || (90 <= n_B[3] && n_B[3] <= 99)){
			if(SkillSearch(24))
				w_atk += Math.floor((3 + 5/100 * n_A_BaseLV) * SkillSearch(24));
		}
		if(n_B[2] == 2 || n_B[2] == 4){
			w_atk += 4 * SkillSearch(116);
			if(SkillSearch(390))
				w_atk += n_A_STR;
		}

		w_atk = BattleCalc2(w_atk);

		return Math.floor(w_atk);
	}


	function BattleCalc2(w999)
	{

		w999_AB = 0;
		if(w999 > 0)
			w999_AB = 1;


		w999 += 2 * SkillSearch(148);


		if(wBCEDPch==0)
			w999 = w999 * element[n_B[3]][n_A_Weapon_element];


		if(n_A_WeaponType == 0 && SkillSearch(329))
			if(n_A_ActiveSkill==331 || n_A_ActiveSkill==333 || n_A_ActiveSkill==335 || n_A_ActiveSkill==337)
				w999 += 10 * SkillSearch(329);


		if(n_A_JOB==15||n_A_JOB==29)
			w999 += 3 * SkillSearch(185);
		else
			w999 += 3 * n_A_PassSkill2[10];

		w999 += 3 * SkillSearch(416);


		if(n_A_WeaponType != 0 && w999_AB == 1)
			w999 += 20 * SkillSearch(254);


		if(wBCEDPch==0){
			if(n_A_ActiveSkill==17 || n_A_ActiveSkill==307)
				w999 += 15 * n_A_ActiveSkillLV;
			if(n_A_ActiveSkill==86 && (n_B[3] < 50 ||  60 <= n_B[3]))
				w999 += 75;
		}
		if(n_A_ActiveSkill==423)
			w999 += Math.floor(n_A_MATK[b] * (100 - n_B[15]) /100 - n_B_MDEF2);
		if(n_A_ActiveSkill==437)
			w999 += n_A_ActiveSkillLV * 50;



		if(cardOBJ[n_A_card[0]][0]==106 && cardOBJ[n_A_card[1]][0]==106 && cardOBJ[n_A_card[2]][0]==106){
			w999 += 40;
		}else{
			for(i=0;i<=2;i++){
				if(cardOBJ[n_A_card[i]][0]==106)
					w999 += 5;
			}
		}
		if(n_A_card[3]==106)
			w999 += 10;


		if(n_A_ActiveSkill == 394){
			w999 += SyurikenOBJ[eval(document.calcForm.SkillSubNum.value)][0];
			w999 += 3 * SkillSearch(393);
			w999 += 4 * n_A_ActiveSkillLV;
		}

		if(n_A_ActiveSkill == 395)
			w999 += KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][0] * 3;

		w999 = BaiCI(w999);


		if(n_A_ActiveSkill==169 && n_A_WeaponType==10)
			w999 = Math.floor(w999 / 2);


		if(hasLeftHand && n_A_ActiveSkill==0){

			if(n_A_WeaponType != 0)
				w999 = Math.floor(w999 * (50 + SkillSearch(79) *10) /100);
		}


		if(n_B[19] == 5)
			return 1;

		if(n_A_ActiveSkill==423)
			w999 = w999 * element[n_B[3]][8];
		if(n_A_ActiveSkill==437)
			w999 = w999 * element[n_B[3]][0];

		return w999;
	}


	function BaiCI(wBaiCI)
	{

		if(wBCEDPch==0 && not_use_card == 0){

			w1=0;
			w1 += StPlusCard(30+n_B[2]);
			w1 += StPlusCalc2(30+n_B[2]);
			if(n_B[2]==6){
				if(ArrowOBJ[n_A_Arrow][2]=="Holy Arrow")
					w1 += 5;
			}
			if(n_B[2]==9  && SkillSearch(234))
				w1 += SkillSearch(234) *4;

			wBaiCI = Math.floor(wBaiCI * (100+w1) /100);


			w1=0;
			w1 += StPlusCard(40+Math.floor(n_B[3] / 10));
			w1 += StPlusCalc2(40+Math.floor(n_B[3] / 10));
			wBaiCI = Math.floor(wBaiCI * (100+w1) /100);


			w1=0;
			w1 += StPlusCard(27+n_B[4]);
			w1 += StPlusCalc2(27+n_B[4]);
			wBaiCI = Math.floor(wBaiCI * (100+w1) /100);


			if(n_Enekyori==1){
				if(TyouEnkakuSousa3dan != -1){
					w1=0;
					w1 += StPlusCard(25);
					w1 += StPlusCalc2(25);
					wBaiCI = Math.floor(wBaiCI * (100+w1) /100);
				}
			}


			w1=0;
			if(n_B[19] == 1){
				w1 += StPlusCard(26);
				w1 += StPlusCalc2(26);
			}
			if(EquipNumSearch(454))
				w1 += EquipNumSearch(454) * 5;
			if(CardNumSearch(323))
				w1 += CardNumSearch(323) * 20;
			if(CardNumSearch(363))
				w1 += CardNumSearch(323) * 10;
			wBaiCI = Math.floor(wBaiCI * (100+w1) /100);


			if(wCriTyuu==1 && n_A_ActiveSkill != 272)
				wBaiCI = Math.floor(wBaiCI * (100+StPlusCard(70)) /100);


			if(108<=n_B[0] && n_B[0]<=115 || n_B[0]==319)
				wBaiCI = Math.floor(wBaiCI * (100+StPlusCard(81)) /100);

			if(116<=n_B[0] && n_B[0]<=120)
				wBaiCI = Math.floor(wBaiCI * (100+StPlusCard(82)) /100);

			if(49<=n_B[0] && n_B[0]<=52 || 55==n_B[0] || 221==n_B[0])
				wBaiCI = Math.floor(wBaiCI * (100+StPlusCard(83)) /100);

			if(106==n_B[0] || 152==n_B[0] || 308==n_B[0] || 32==n_B[0])
				wBaiCI = Math.floor(wBaiCI * (100+StPlusCard(84)) /100);


			wBaiCI = Math.floor(wBaiCI * (100+StPlusCalc2(1000+n_B[0])+StPlusCard(1000+n_B[0])) /100);


			if(EquipNumSearch(626) && n_A_Arrow == 2)
				wBaiCI = wBaiCI * 125 /100;

			if(EquipNumSearch(627) && n_A_Arrow == 5)
				wBaiCI = wBaiCI * 125 /100;

			if(EquipNumSearch(628) && n_A_Arrow == 4)
				wBaiCI = wBaiCI * 125 /100;

			if(EquipNumSearch(629) && n_A_Arrow == 6)
				wBaiCI = wBaiCI * 125 /100;

			if(EquipNumSearch(630) && n_A_Arrow == 10)
				wBaiCI = wBaiCI * 150 /100;

			if(SkillSearch(258))
				wBaiCI = wBaiCI * 2;
			if(SkillSearch(266))
				wBaiCI = Math.floor(wBaiCI * (150 + 50 * SkillSearch(266)) /100);
			if(n_A_ActiveSkill==86 && (50 <= n_B[3] && n_B[3] < 60))
				wBaiCI = Math.floor(wBaiCI * (100 + 30 * n_A_ActiveSkillLV) /100);


			if(n_A_WeaponType == 11 && SkillSearch(262) && n_A_ActiveSkill != 263)
				wBaiCI = Math.floor(wBaiCI * (110 + 2 * SkillSearch(262))/100);

			w1 = 0;
			if(Taijin == 0){
				if(SkillSearch(354) && SkillSearch(365))
					w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch(354) *3);
				else if(SkillSearch(354) && n_B[4]==2 && n_B[6] >= 20000)
					w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch(354) *3);
				else if(SkillSearch(352) && n_B[4]==0)
					w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(352) *3);
				else if(SkillSearch(353) && n_B[4]==1 && n_B[6] >= 5000)
					w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(353) *3);
			}else{
				if(SkillSearch(354)){
					w1 += (n_A_BaseLV + n_A_STR + n_A_LUK + n_A_DEX) / (12 - SkillSearch(354) *3);
				}else{
					if(SkillSearch(352)){
						w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(352) *3);
					}else{
						if(SkillSearch(353))
							w1 += (n_A_BaseLV + n_A_LUK + n_A_DEX) / (12 - SkillSearch(353) *3);
					}
				}
			}
			wBaiCI = Math.floor(wBaiCI * (100+w1) /100);
		}

		wBaiCI = Math.floor(tPlusDamCut(wBaiCI));

		w1=0;

		if(n_A_ActiveSkill == 6)
			if(n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch(362))
				w1 += 10;

		if(TyouEnkakuSousa3dan == -1 && EquipNumSearch(639))
			w1 += 15;

		if((n_A_ActiveSkill==83 || n_A_ActiveSkill==388) && SkillSearch(381))
			w1 += 10;

		wBaiCI = wBaiCI * (100+StPlusCalc2(5000+n_A_ActiveSkill)+StPlusCard(5000+n_A_ActiveSkill) + w1) /100;

		return wBaiCI;
	}


	function BattleCalc3(w998)
	{
		wBC3_3dan = w998B * TyouEnkakuSousa3dan;
		wBC3_DA = w998E * w998 * 2;
		wBC3_Cri = w998G * n_A_CriATK[1];
		wBC3_Normal = w998I * w998;
		wBC3_Miss = w998L * BattleCalc2(0);

		wBC3_X = (wBC3_3dan +wBC3_DA +wBC3_Cri +wBC3_Normal +wBC3_Miss) /100;

		return tPlusLucky(wBC3_X);
	}


	function BattleCalc3left(w998)
	{

		wBC3L2 = 0;
		for(i=4;i<=7;i++){
			if(cardOBJ[n_A_card[i]][0]==106)
				wBC3L2 += 5;
		}

		wBC3_Normal = w998 * w_HIT /100;
		wBC3_Miss = wBC3L2 * (100-w_HIT) /100;

		wBC3_X = wBC3_Normal + wBC3_Miss;

		wBC3_X = tPlusDamCut(wBC3_X);

		return tPlusLucky(wBC3_X);
	}



	function SkillSearch(n)
	{
		for(k=0;k<=14;k++)
		{
			if(JobSkillPassOBJ[n_A_JOB][k] == n)
			{
				return n_A_PassSkill[k];
			}
		}
		return 0;
	}


	function BattleCalc4(wBC4,wBC4_2,wBC4_3){
		if(wBC4_3==0)
			wBC4_3=n_A_WeaponLV_upgradeBonusATK;
		else
			wBC4_3=n_A_Weapon2LV_upgradeBonusATK;
		if((StPlusCalc2(23) + StPlusCard(23)) == 0 || n_A_ActiveSkill==275)
		{
			if(n_A_ActiveSkill==432)
				return wBC4 + wBC4_3;
			if(StPlusCalc2(22) == n_B[2] && n_B[2] != 0)
				return wBC4 + wBC4_3;
			if(StPlusCalc2(22) == 99 && n_B[19] == 0)
				return wBC4 + wBC4_3;
			if(SkillSearch(364))
				return wBC4 + wBC4_3;
			if(CardNumSearch(255) && n_B[19] == 0)
				return wBC4 + wBC4_3;
			wBC4 = Math.floor(wBC4 * (100 - n_B[14]) /100) - n_B_DEF2[wBC4_2] + wBC4_3;
		}else{
			if(wBC4_2==0){
				wBC4 = Math.floor(wBC4 * (n_B_DEF2[2]+n_B[14])/100) +wBC4_3;
			}else if(wBC4_2==1){
				wBC4 = Math.floor(wBC4 * (n_B_DEF2[1]+n_B[14])/100) +wBC4_3;
			}else{
				wBC4 = Math.floor(wBC4 * (n_B_DEF2[0]+n_B[14])/100) +wBC4_3;
			}
		}
		return wBC4;
	}



	function BattleCalcEDP(wBCEDP,wBCEDP2){
		if(wBCEDP <= 0)
			return 0;
		if(element[n_B[3]][n_A_Weapon_element] <= 0 && BattleCalc2(0) == 0)
			return 0;

		if(n_A_ActiveSkill == 19 || n_A_ActiveSkill == 263 || n_A_ActiveSkill == 88 || n_A_ActiveSkill == 264 || n_A_ActiveSkill == 248)
			return 0;
		wBCEDPch=1;
		wBCEDPx=0;
		wBCEDPy=0;
		if(SkillSearch(266)){
			wBCEDPx = BattleCalc(wBCEDP,wBCEDP2);
			wBCEDPx = Math.floor((wBCEDPx * element[n_B[3]][5])/4);
		}
		if(n_A_PassSkill2[11]){
			wBCEDPy = BattleCalc(wBCEDP,wBCEDP2);
			wBCEDPy = Math.floor((wBCEDPy * element[n_B[3]][3]) /5);
		}
		wBCEDPch=0;
		return wBCEDPx + wBCEDPy;
	}


	function HitEDPplus(wBCEDPp){
		if(wBCEDPp <= 0)
			return 0;
		if(element[n_B[3]][n_A_Weapon_element] <= 0)
			return 0;
		wBCEDPpDA = 1;
		if(n_A_ActiveSkill==0)
			wBCEDPpDA = (100+w998E) /100;

		wBCEDPch=1;
		wBCEDPpHOSI = BattleCalc2(0);
		wBCEDPch=0;
		if(wBCEDPpHOSI >= 1){
			www = w_HIT;

			if(SkillSearch(266))
				wBCEDPpHOSI = Math.floor((wBCEDPpHOSI * element[n_B[3]][5])/4);
			if(n_A_PassSkill2[11])
				wBCEDPpHOSI = Math.floor((wBCEDPpHOSI * element[n_B[3]][3])/5);
		}
		else
			www = w998K * w_HIT /100;

		if(n_A_WeaponType == 11 && n_A_ActiveSkill==0)
			wBCEDPp = Math.floor(wBCEDPp * (1.01 + SkillSearch(13) * 0.02));


		if(n_A_ActiveSkill==0){
			wBCEDPp *= wBCEDPpDA;
			wBCEDPpHOSI *= wBCEDPpDA;
		}
		return (wBCEDPp * www /100) + (wBCEDPpHOSI * (100 -w_HIT) /100);
	}


	function CastAndDelay(){
		if(wCast!=0){
			wCAD = n_A_PassSkill3[2];
			if(wCAD != 0){
				wCAD = wCAD * 3 + n_A_PassSkill3[32] + Math.floor(n_A_PassSkill3[22] /10);
				wCast *= (100-wCAD)/100;
			}
			myInnerHtml("bSUBname",SubName[9],0);
			myInnerHtml("bSUB",Math.floor(wCast *100)/100 +SubName[1],0);
		}
		if(wDelay!=0){
			if(swDelay == 1){
				wCAD = n_A_PassSkill3[2];
				if(wDelay != "(Unknown)"){
					wDelay = Math.floor(wDelay * (100-(StPlusCard(74)+StPlusCalc2(74))))/100;
					if(wCAD != 0){
						if(wCAD==10)
							wCAD2=5;
						else
							wCAD2=3;
						wCAD = wCAD * wCAD2 + n_A_PassSkill3[32] *2 + Math.floor(n_A_PassSkill3[29] /5);
						wDelay *= (100-wCAD)/100;
					}
					wDelay = Math.floor(wDelay *100)/100;
					if(wCast + wDelay < eval(document.calcForm.Conf01.value) /100)
						wDelay = eval(document.calcForm.Conf01.value) /100 - wCast;
				}
				myInnerHtml("bSUB2name","Delay (Fixed Type)",0);
				myInnerHtml("bSUB2",Math.floor(wDelay *100)/100 +"s",0);
				return;
			}
			if(swDelay == 2){
				myInnerHtml("bSUB2name","Delay(Motion Type)",0);
				myInnerHtml("bSUB2",wDelay+"s",0);
			}else{
				if(n_SpSkill!=1){
					if(wDelay != "(�s��)")
						wDelay = Math.floor(wDelay *100)/100;
					myInnerHtml("bSUB2name","Delay(Attack Speed Type)",0);
					myInnerHtml("bSUB2",wDelay+"s",0);
				}
			}
		}
	}


}
/*
     FILE ARCHIVED ON 14:35:31 Nov 11, 2008 AND RETRIEVED FROM THE
     INTERNET ARCHIVE ON 07:19:09 Nov 20, 2023.
     JAVASCRIPT APPENDED BY WAYBACK MACHINE, COPYRIGHT INTERNET ARCHIVE.

     ALL OTHER CONTENT MAY ALSO BE PROTECTED BY COPYRIGHT (17 U.S.C.
     SECTION 108(a)(3)).
*/
/*
playback timings (ms):
  captures_list: 74.183
  exclusion.robots: 0.186
  exclusion.robots.policy: 0.17
  cdx.remote: 0.115
  esindex: 0.035
  LoadShardBlock: 46.539 (3)
  PetaboxLoader3.datanode: 60.389 (4)
  load_resource: 340.62
  PetaboxLoader3.resolve: 297.63
*/