function server() {
    SRV = eval(document.getElementById("server").value), calc()
}

function restrictEquipslot() {
    var c = document.calcForm;
    equip_restrict = eval(document.getElementById("restrict_equipslot").checked);
    var w = c.A_WeaponType.value;
    (3 == w || 5 == w || 7 == w || 10 == w || 11 == w || 16 == w || 17 == w || 18 == w || 19 == w || 20 == w || 21 == w || n_Nitou) && equip_restrict ? (c.A_LEFT_DEF_PLUS.disabled = !0, c.A_LEFT_DEF_PLUS.value = 0, c.A_left.disabled = !0, c.A_left.value = 305, c.A_left_card.disabled = !0, c.A_left_card.value = 0) : (c.A_LEFT_DEF_PLUS.disabled = !1, c.A_left.disabled = !1, card_restrict && 0 != ItemOBJ[c.A_left.value][5] && (c.A_left_card.disabled = !1))
}

function restrictCardslot(select_ID) {
    var c = document.calcForm;
    card_restrict = eval(document.getElementById("restrict_cardslot").checked), card_restrict ? (0 != ItemOBJ[c.A_weapon1.value][5] ? (c.A_weapon1_card1.disabled = !1, c.A_weapon1_card2.disabled = !1, c.A_weapon1_card3.disabled = !1, c.A_weapon1_card4.disabled = !1) : (c.A_weapon1_card1.disabled = !0, c.A_weapon1_card1.value = 0, c.A_weapon1_card2.disabled = !0, c.A_weapon1_card2.value = 0, c.A_weapon1_card3.disabled = !0, c.A_weapon1_card3.value = 0, c.A_weapon1_card4.disabled = !0, c.A_weapon1_card4.value = 0), n_Nitou && (0 != ItemOBJ[c.A_weapon2.value][5] ? (c.A_weapon2_card1.disabled = !1, c.A_weapon2_card2.disabled = !1, c.A_weapon2_card3.disabled = !1, c.A_weapon2_card4.disabled = !1) : (c.A_weapon2_card1.disabled = !0, c.A_weapon2_card1.value = 0, c.A_weapon2_card2.disabled = !0, c.A_weapon2_card2.value = 0, c.A_weapon2_card3.disabled = !0, c.A_weapon2_card3.value = 0, c.A_weapon2_card4.disabled = !0, c.A_weapon2_card4.value = 0)), 0 != ItemOBJ[c.A_head1.value][5] ? c.A_head1_card.disabled = !1 : (c.A_head1_card.disabled = !0, c.A_head1_card.value = 0), 0 != ItemOBJ[c.A_head2.value][5] ? c.A_head2_card.disabled = !1 : (c.A_head2_card.disabled = !0, c.A_head2_card.value = 0), 0 != ItemOBJ[c.A_left.value][5] ? c.A_left_card.disabled = !1 : (c.A_left_card.disabled = !0, c.A_left_card.value = 0), 0 != ItemOBJ[c.A_body.value][5] ? c.A_body_card.disabled = !1 : (c.A_body_card.disabled = !0, c.A_body_card.value = 0), 0 != ItemOBJ[c.A_shoulder.value][5] ? c.A_shoulder_card.disabled = !1 : (c.A_shoulder_card.disabled = !0, c.A_shoulder_card.value = 0), 0 != ItemOBJ[c.A_shoes.value][5] ? c.A_shoes_card.disabled = !1 : (c.A_shoes_card.disabled = !0, c.A_shoes_card.value = 0), 0 != ItemOBJ[c.A_acces1.value][5] ? c.A_acces1_card.disabled = !1 : (c.A_acces1_card.disabled = !0, c.A_acces1_card.value = 0), 0 != ItemOBJ[c.A_acces2.value][5] ? c.A_acces2_card.disabled = !1 : (c.A_acces2_card.disabled = !0, c.A_acces2_card.value = 0)) : (c.A_weapon1_card1.disabled = !1, c.A_weapon1_card2.disabled = !1, c.A_weapon1_card3.disabled = !1, c.A_weapon1_card4.disabled = !1, n_Nitou && (c.A_weapon2_card1.disabled = !1, c.A_weapon2_card2.disabled = !1, c.A_weapon2_card3.disabled = !1, c.A_weapon2_card4.disabled = !1), c.A_head1_card.disabled = !1, c.A_head2.disabled ? c.A_head2_card.disabled = !0 : c.A_head2_card.disabled = !1, c.A_left.disabled ? c.A_left_card.disabled = !0 : c.A_left_card.disabled = !1, c.A_body_card.disabled = !1, c.A_shoulder_card.disabled = !1, c.A_shoes_card.disabled = !1, c.A_acces1_card.disabled = !1, c.A_acces2_card.disabled = !1), calc()
}

function myInnerHtml(_, n, e) {
    if (0 == e) {
        for (wIHOB = document.getElementById(_); wIHOB.hasChildNodes();) wIHOB.removeChild(wIHOB.firstChild);
        wIHOB.innerHTML = n
    } else wIHOB = document.getElementById(_), wIHOB.insertAdjacentHTML("BeforeEnd", n)
}

function BattleCalc999() {
    wbairitu = 1, wCast = 0, wHITsuu = 1, n_Enekyori = 0, wLAch = 0, w_DMG = [0, 0, 0], not_use_card = 0, cast_kotei = 0, str_PerHIT_DMG = 0, SG_Special_ch = 0;
    for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = 0, Last_DMG_B[i] = 0;
    if (str_bSUBname = "", str_bSUB = "", 0 == n_A_ActiveSkill || 272 == n_A_ActiveSkill || 401 == n_A_ActiveSkill || 86 == n_A_ActiveSkill && 50 <= n_B[3] && n_B[3] < 60 || (myInnerHtml("CRIATK", "", 0), myInnerHtml("CRInum", "", 0), myInnerHtml("CRIATKname", "", 0), myInnerHtml("CRInumname", "", 0)), 10 != n_A_WeaponType && 17 != n_A_WeaponType && 18 != n_A_WeaponType && 19 != n_A_WeaponType && 20 != n_A_WeaponType && 21 != n_A_WeaponType || 0 != n_A_ActiveSkill || (n_Enekyori = 1), 0 == n_A_ActiveSkill || 86 == n_A_ActiveSkill && 50 <= n_B[3] && n_B[3] < 60)
        if (myInnerHtml("CRIATKname", SubName[3], 0), myInnerHtml("CRInumname", SubName[4], 0), 86 == n_A_ActiveSkill && (n_Delay[0] = 1), n_Nitou) {
            if (TyouEnkakuSousa3dan = 0, n_A_workDEX = Math.floor(n_A_DEX * (1 + .2 * (n_A_Weapon2LV - 1))), n_A_workDEX >= n_A_Weapon2_ATK ? w_left_Maxatk = n_A_ATK + n_A_Weapon2LV_Maxplus + Math.floor((n_A_Weapon2_ATK + wImp) * wCSize) : w_left_Maxatk = n_A_ATK + n_A_Weapon2LV_Maxplus + Math.floor((n_A_Weapon2_ATK - 1 + wImp) * wCSize), w_left_Maxatk = BattleCalc4(w_left_Maxatk * wbairitu, 2, 1), w_left_Maxatk < 1 && (w_left_Maxatk = 1), w_left_Maxatk = Math.floor(w_left_Maxatk * zokusei[n_B[3]][n_A_Weapon2_zokusei]), w_left_star = 0, 106 == n_A_card[4] && 106 == n_A_card[5] && 106 == n_A_card[6]) w_left_star += 40;
            else
                for (i = 4; 6 >= i; i++) 106 == cardOBJ[n_A_card[i]][0] && (w_left_star += 5);
            106 == n_A_card[7] && (w_left_star += 10), w_left_Maxatk += w_left_star, w_left_Maxatk = w_left_Maxatk * (3 + SkillSearch(80)) / 10, w_left_Maxatk = Math.floor(w_left_Maxatk), n_A_workDEX > n_A_Weapon2_ATK && (n_A_workDEX = n_A_Weapon2_ATK), w_left_Minatk = n_A_ATK + n_A_Weapon2LV_Minplus + Math.floor((n_A_workDEX + wImp) * wCSize), w_left_Minatk = BattleCalc4(w_left_Minatk * wbairitu, 0, 1), w_left_Minatk < 1 && (w_left_Minatk = 1), w_left_Minatk = Math.floor(w_left_Minatk * zokusei[n_B[3]][n_A_Weapon2_zokusei]), w_left_Minatk += w_left_star, w_left_Minatk *= .3 + SkillSearch(80) / 10, w_left_Minatk = Math.floor(w_left_Minatk), w_left_Aveatk = (w_left_Maxatk + w_left_Minatk) / 2, w_left_Maxatk = tPlusDamCut(w_left_Maxatk), w_left_Minatk = tPlusDamCut(w_left_Minatk), w_left_Aveatk = tPlusDamCut(w_left_Aveatk), ATKbai02(wbairitu, 0), n_Min_DMG += w_left_Minatk, n_Max_DMG += w_left_Maxatk, w_DMG[0] = BattleCalc(n_A_DMG[0], 0);
            var wX = w_DMG[0] + EDP_DMG(0);
            Last_DMG_A[0] = Last_DMG_B[0] = wX + w_left_Minatk, InnStr[0] += wX + " (" + w_left_Minatk + ")", w998D && (str_bSUBname += "Double Attack chance<BR>", str_bSUB += 2 * wX + w_left_Minatk + "~"), SRV ? wX + w_left_Minatk < n_Min_DMG && w998G < 100 && (n_Min_DMG = wX + w_left_Minatk) : Last_DMG_A[0] < n_Min_DMG && (w998H > 0 ? n_Min_DMG = Last_DMG_A[0] : w998D > 0 && 2 * wX + w_left_Minatk < n_Min_DMG && (n_Min_DMG = 2 * wX + w_left_Minatk)), w_DMG[0] = n_Min_DMG, w_DMG[2] = BattleCalc(n_A_DMG[2], 2);
            var wX = w_DMG[2] + EDP_DMG(2) + w_left_Maxatk;
            Last_DMG_A[2] = Last_DMG_B[2] = wX + w_left_Maxatk, InnStr[2] += w_DMG[2] + EDP_DMG(2) + " (" + w_left_Maxatk + ")", w998D && (wX = 2 * (w_DMG[2] + EDP_DMG(2)) + w_left_Maxatk, str_bSUB += wX + " (" + w998D + "%)<BR>"), wX > n_Max_DMG && w998G < 100 && (n_Max_DMG = wX), w_DMG[2] = n_Max_DMG, w_DMG[1] = BattleCalc(n_A_DMG[1], 1);
            var wX = w_DMG[1] + EDP_DMG(1);
            Last_DMG_A[1] = Last_DMG_B[1] = wX + w_left_Aveatk, InnStr[1] += wX + " (" + w_left_Aveatk + ")", w_DMG[1] = BattleCalc3(w_DMG[1]), w_DMG[1] += BattleCalc3left(w_left_Aveatk), w_DMG[1] += EDP_DMG(1), EDPhyouzi(1);
            var wX = BattleCalc2(0),
                wX2 = Math.floor(w_left_star * (.3 + SkillSearch(80) / 10));
            n_PerHIT_DMG = wX + wX2, str_PerHIT_DMG = wX + "+" + wX2, CastAndDelay(), BattleCalc998()
        } else {
            if (n_TAKA_DMG = 0, wTAKA = BattleTAKA(), TyouEnkakuSousa3dan = 0, SkillSearch(187)) {
                TyouEnkakuSousa3dan = -1, wBC3_3danAtkBairitu = .2 * SkillSearch(187);
                for (var san = [0, 0, 0], i = 0; 2 >= i; i++) san[i] = BattleCalc(n_A_DMG[i] * (wbairitu + wBC3_3danAtkBairitu), i) + EDP_DMG(i), san[i] = 3 * Math.floor(san[i] / 3), 5 == n_B[19] && (san[i] = 3);
                str_bSUBname += "Raging Trifecta Blow Damage<BR>", str_bSUB += san[0] + "~" + san[2] + " (" + (30 - SkillSearch(187)) + "% Chance)<BR>", TyouEnkakuSousa3dan = 0, n_Min_DMG > san[0] && (n_Min_DMG = san[0]), n_Max_DMG < san[2] && (n_Max_DMG = san[2])
            }
            ATKbai02(wbairitu, 0);
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BattleCalc(n_A_DMG[i], i);
            var w_KATARU = [0, 0, 0],
                w_Ave_KATARU = 0;
            if (11 == n_A_WeaponType) {
                for (i = 0; 2 >= i; i++) w_KATARU[i] = Math.floor((w_DMG[i] + EDP_DMG(i)) * (.01 + .02 * SkillSearch(13)));
                w_Ave_KATARU = Math.floor(w_DMG[1] * (.01 + .02 * SkillSearch(13)))
            }
            Last_DMG_B[0] = w_DMG[0] + EDP_DMG(0), Last_DMG_A[0] = Last_DMG_B[0] + w_KATARU[0], InnStr[0] += Last_DMG_A[0], 11 == n_A_WeaponType && (InnStr[0] = Last_DMG_A[0] + " (" + Last_DMG_B[0] + "+" + w_KATARU[0] + ")"), SRV ? Last_DMG_A[0] < n_Min_DMG && w998G < 100 && (n_Min_DMG = Last_DMG_A[0]) : Last_DMG_A[0] < n_Min_DMG && (w998H > 0 ? n_Min_DMG = Last_DMG_A[0] : w998D > 0 && 2 * Last_DMG_A[0] < n_Min_DMG && (n_Min_DMG = 2 * Last_DMG_A[0])), w998D && (17 == n_A_WeaponType && SkillSearch(427) ? CardNumSearch(43) || 570 == n_A_Equip[2] || 1442 == n_A_Equip[2] || 1443 == n_A_Equip[2] || 1321 == n_A_Equip[2] || EquipNumSearch(1578) && n_A_SHOULDER_DEF_PLUS >= 5 ? str_bSUBname += "Double attack chance<BR>" : str_bSUBname += "Chain action chance<BR>" : str_bSUBname += "Double attack chance<BR>", str_bSUB += 2 * Last_DMG_A[0] + "~"), w_DMG[0] = n_Min_DMG, Last_DMG_B[2] = w_DMG[2] + EDP_DMG(2), Last_DMG_A[2] = Last_DMG_B[2] + w_KATARU[2], InnStr[2] += Last_DMG_A[2], 11 == n_A_WeaponType && (InnStr[2] = Last_DMG_A[2] + " (" + Last_DMG_B[2] + "+" + w_KATARU[2] + ")"), n_Max_DMG += n_TAKA_DMG;
            var wX = Last_DMG_A[2];
            if (wX += n_TAKA_DMG, n_Max_DMG < wX && w998G < 100 && (n_Max_DMG = wX), w998D) {
                var wX = 2 * (w_DMG[2] + EDP_DMG(2) + w_KATARU[2]);
                str_bSUB += wX + " (" + w998D + "%)<BR>", wX += n_TAKA_DMG, n_Max_DMG < wX && (n_Max_DMG = wX)
            }
            w_DMG[2] = n_Max_DMG, Last_DMG_B[1] = w_DMG[1] + EDP_DMG(1), Last_DMG_A[1] = Last_DMG_B[1] + w_KATARU[1], InnStr[1] += Last_DMG_A[1], 11 == n_A_WeaponType && (InnStr[1] = Last_DMG_A[1] + " (" + Last_DMG_B[1] + "+" + w_KATARU[1] + ")"), SkillSearch(187) && (TyouEnkakuSousa3dan = san[1]), w_DMG[1] += w_Ave_KATARU, w_DMG[1] = BattleCalc3(w_DMG[1]), w_DMG[1] += wTAKA, w_DMG[1] += EDP_DMG(1), EDPhyouzi(1), CastAndDelay(), BattleCalc998()
        } else {
        if (272 == n_A_ActiveSkill || 401 == n_A_ActiveSkill) {
            for (myInnerHtml("CRIATKname", "Critical Hit", 0), myInnerHtml("CRInumname", "Critical Attack chance", 0), 272 == n_A_ActiveSkill ? (n_Enekyori = 1, wbairitu += 1 + .5 * n_A_ActiveSkillLV, wCast = 2 * n_A_CAST, n_Delay[2] = 1.5) : (n_Delay[0] = 1, n_Enekyori = 0, wbairitu += n_A_ActiveSkillLV - 1), i = 0; 2 >= i; i++) n_A_CriATK[i] = n_A_DMG[i];
            ATKbai02(wbairitu, 1), wCriTyuu = 1;
            for (var i = 0; 2 >= i; i++) n_A_CriATK[i] = BattleCalc(n_A_CriATK[i], 10);
            wCriTyuu = 0;
            for (var i = 0; 2 >= i; i++) n_A_CriATK[i] += EDP_DMG(i);
            w998G >= 100 && (n_Min_DMG = n_A_CriATK[0]), w998G > 0 && (n_Max_DMG = n_A_CriATK[2]), myInnerHtml("CRIATK", n_A_CriATK[0] + "~" + n_A_CriATK[2], 0), ATKbai02(wbairitu, 0);
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BattleCalc(n_A_DMG[i], i), Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + EDP_DMG(i), InnStr[i] += Last_DMG_A[i];
            return w998G >= 100 && (w_DMG[0] = n_Min_DMG), w998G > 0 && (w_DMG[2] = n_Max_DMG), w_DMG[1] = BattleCalc3(w_DMG[1]), EDPplus(1), CastAndDelay(), void BattleCalc998()
        }
        for (w_ActS = [6, 7, 19, 41, 44, 65, 71, 72, 73, 83, 84, 111, 158, 161, 169, 171, 176, 188, 189, 199, 207, 248, 260, 261, 264, 288, 289, 290, 292, 302, 303, 305, 306, 307, 308, 326, 317, 318, 331, 333, 335, 337, 339, 382, 388, 398, 400, 419, 423, 428, 430, 431, 432, 434, 435, 436, 437, "NULL"], iw = 0; w_ActS[iw] != n_A_ActiveSkill && "NULL" != w_ActS[iw]; iw++);
        if (n_A_ActiveSkill == w_ActS[iw]) {
            if (wActiveHitNum = 1, 6 == n_A_ActiveSkill) wbairitu += .3 * n_A_ActiveSkillLV;
            else if (7 == n_A_ActiveSkill) wbairitu += .2 * n_A_ActiveSkillLV, n_A_Weapon_zokusei = 3, n_Delay[2] = 2;
            else if (19 == n_A_ActiveSkill) not_use_card = 1, wbairitu += .3, n_A_Weapon_zokusei = 2;
            else if (41 == n_A_ActiveSkill) n_Enekyori = 1, wbairitu += .05 * n_A_ActiveSkillLV - .25, n_Delay[2] = 1;
            else if (44 == n_A_ActiveSkill) n_Enekyori = 1, wCast = 1.5, wbairitu += .5;
            else if (65 == n_A_ActiveSkill) wbairitu += .5 * n_A_ActiveSkillLV;
            else if (71 == n_A_ActiveSkill) wbairitu += .2 * n_A_ActiveSkillLV, n_Enekyori = 1;
            else if (84 == n_A_ActiveSkill) n_A_ActiveSkillLV >= 3 && (n_Enekyori = 1), wbairitu += .2 * n_A_ActiveSkillLV;
            else if (158 == n_A_ActiveSkill) wbairitu += .2 * n_A_ActiveSkillLV, 305 == ItemOBJ[n_A_Equip[5]][0] && (wbairitu = 0);
            else if (161 == n_A_ActiveSkill) wbairitu += .35 * n_A_ActiveSkillLV, n_A_Weapon_zokusei = 6;
            else if (171 == n_A_ActiveSkill) wbairitu += .4 * n_A_ActiveSkillLV;
            else if (72 == n_A_ActiveSkill) wbairitu += .5 * n_A_ActiveSkillLV, n_Delay[2] = 1, n_Enekyori = 1;
            else if (73 == n_A_ActiveSkill) w = 1 + .2 * n_A_ActiveSkillLV, 10 == n_A_ActiveSkillLV ? wbairitu += 4.625 : n_A_ActiveSkillLV >= 7 ? wbairitu += w + w / 2 + w / 4 - 1 : n_A_ActiveSkillLV >= 4 ? wbairitu += w + w / 2 - 1 : wbairitu += w - 1, wCast = .7;
            else if (83 == n_A_ActiveSkill || 388 == n_A_ActiveSkill) wActiveHitNum = 8, wbairitu += .5 * n_A_ActiveSkillLV + 2, 388 == n_A_ActiveSkill && 0 == Taijin && (wbairitu *= 2), 388 == n_A_ActiveSkill && 1 == Taijin && (n_Ses ? wbairitu *= 1.25 : wbairitu *= 2), n_Delay[3] = 2;
            else if (111 == n_A_ActiveSkill) n_Delay[0] = 1, not_use_card = 1, n_A_Weapon_zokusei = 1;
            else if (169 == n_A_ActiveSkill) wbairitu += .4 * n_A_ActiveSkillLV + 2, n_Delay[2] = .5, w_HIT = 100, w_HIT_HYOUJI = 100;
            else if (176 == n_A_ActiveSkill) wbairitu += .3 * n_A_ActiveSkillLV, n_Delay[2] = 1;
            else if (188 == n_A_ActiveSkill) wActiveHitNum = 4, wbairitu += .5 + .5 * n_A_ActiveSkillLV, n_Delay[0] = 1, n_Delay[1] = .1, n_Delay[3] = 1 - .004 * n_A_AGI - .002 * n_A_DEX;
            else if (189 == n_A_ActiveSkill) wbairitu += 1.4 + .6 * n_A_ActiveSkillLV, n_Delay[0] = 1, n_Delay[1] = .1, n_Delay[3] = .7 - .004 * n_A_AGI - .002 * n_A_DEX;
            else if (199 == n_A_ActiveSkill || 207 == n_A_ActiveSkill) wCast = 1.5, wbairitu += .4 * n_A_ActiveSkillLV - .4, n_A_Weapon_zokusei = ArrowOBJ[n_A_Arrow][1], 0 != eval(document.calcForm.A_Weapon_zokusei.value) && (n_A_Weapon_zokusei = eval(document.calcForm.A_Weapon_zokusei.value)), n_Enekyori = 1;
            else if (248 == n_A_ActiveSkill) not_use_card = 1, n_A_Weapon_zokusei = 3, n_Delay[0] = 1, wCast = 1, wbairitu += .2 * n_A_ActiveSkillLV, w_HIT = 100, w_HIT_HYOUJI = 100;
            else if (260 == n_A_ActiveSkill) n_Enekyori = 1, wbairitu += .4 * n_A_ActiveSkillLV, n_Delay[2] = .5;
            else if (261 == n_A_ActiveSkill) n_Enekyori = 1, wbairitu += .1 * n_A_ActiveSkillLV - .5, n_A_ActiveSkillLV > 5 ? n_Delay[2] = 1 : n_Delay[2] = .8;
            else if (264 == n_A_ActiveSkill) not_use_card = 1, wbairitu += .4 * n_A_ActiveSkillLV - .6, wCast = .5, n_Delay[2] = .5;
            else if (288 == n_A_ActiveSkill) wbairitu += 1 + n_A_ActiveSkillLV, n_Delay[2] = .3;
            else if (289 == n_A_ActiveSkill) n_Delay[0] = 1, wbairitu += n_A_ActiveSkillLV - .6, n_Delay[1] = .1, n_Delay[3] = .7 - .004 * n_A_AGI - .002 * n_A_DEX;
            else if (290 == n_A_ActiveSkill) n_Delay[0] = 1, wbairitu += 3 + n_A_ActiveSkillLV, n_A_ActiveSkillLV > 6 ? n_Delay[2] = 1 : n_Delay[2] = .8;
            else if (292 == n_A_ActiveSkill) wActiveHitNum = 9, wbairitu += 1 + n_A_ActiveSkillLV, n_A_Weapon_zokusei = ArrowOBJ[n_A_Arrow][1], 0 != eval(document.calcForm.A_Weapon_zokusei.value) && (n_A_Weapon_zokusei = eval(document.calcForm.A_Weapon_zokusei.value)), n_Enekyori = 1, wCast = 1.8 + .2 * n_A_ActiveSkillLV, n_A_ActiveSkillLV >= 6 ? n_Delay[2] = 1 : n_Delay[2] = .8, n_Delay[3] = 3;
            else if (302 == n_A_ActiveSkill) n_Enekyori = 1, not_use_card = 1, n_A_Weapon_zokusei = 4;
            else if (303 == n_A_ActiveSkill) wbairitu += 1 * (n_A_ActiveSkillLV - 1);
            else if (306 == n_A_ActiveSkill) n_Enekyori = 1, not_use_card = 1, n_A_DMG[1] += Math.floor(14.5 * wCSize), n_A_DMG[2] += Math.floor(29 * wCSize);
            else if (307 == n_A_ActiveSkill) n_Enekyori = 1, not_use_card = 1, wbairitu += .5;
            else if (308 == n_A_ActiveSkill) {
                var w;
                w = eval(document.calcForm.SkillSubNum.value), wbairitu += w, wCast = .5 * (w + 1), wCast > 1.5 && (wCast = 1.5)
            } else 317 == n_A_ActiveSkill ? (n_Delay[0] = 1, n_Delay[5] = .05, 1 == n_B[19] && (n_Delay[5] = .1), 1 == Taijin && (str_bSUBname += "SP damage<BR>", str_bSUB += "15<BR>")) : 318 == n_A_ActiveSkill ? (n_Delay[5] = .05, 1 == n_B[19] && (n_Delay[5] = .1), 1 == Taijin && (n_Delay[0] = 1, str_bSUBname += "SP damage<BR>", str_bSUB += "15<BR>")) : 326 == n_A_ActiveSkill ? (not_use_card = 1, SRV ? (eval(document.calcForm.SkillSubNum.value) > 8e3 ? CT_WEIGHT = 8e3 : CT_WEIGHT = eval(document.calcForm.SkillSubNum.value), 0 == CT_WEIGHT ? (SkillSearch(154) && (wbairitu += .146 + Math.floor((n_tok[22] + 5 * SkillSearch(154)) / 100 * 100 / 100)), SkillSearch(327) && (wbairitu += .215 + Math.floor((n_tok[22] + 10 * SkillSearch(327)) / 100 * 100 / 100))) : wbairitu += .02 + Math.floor((CT_WEIGHT / (16 - n_A_ActiveSkillLV) + n_tok[22] + 5 * SkillSearch(154) + 20 * SkillSearch(327)) / 100 * 100 / 100)) : wbairitu += Math.floor(eval(document.calcForm.SkillSubNum.value) / (16 - n_A_ActiveSkillLV) / 100 * 100) / 100) : 382 == n_A_ActiveSkill ? (not_use_card = 1, wbairitu += 2) : 331 == n_A_ActiveSkill || 333 == n_A_ActiveSkill ? (n_Delay[0] = 1, wbairitu += .6 + .2 * n_A_ActiveSkillLV) : 335 == n_A_ActiveSkill || 337 == n_A_ActiveSkill ? (n_Delay[0] = 1, wbairitu += .9 + .3 * n_A_ActiveSkillLV, 337 == n_A_ActiveSkill && (wActiveHitNum = 3)) : 339 == n_A_ActiveSkill ? (n_Delay[0] = 1, wbairitu += -.7 + .1 * n_A_ActiveSkillLV) : 305 == n_A_ActiveSkill ? (n_Delay[0] = 1, SkillSearch(379) && 0 == n_A_WeaponType ? wbairitu += .08 * n_A_BaseLV - 1 : wbairitu += .04 * n_A_BaseLV - 1) : 398 == n_A_ActiveSkill ? (wbairitu += .1 * n_A_ActiveSkillLV, n_Delay[2] = 3) : 400 == n_A_ActiveSkill ? (n_Delay[0] = 1, wbairitu += .1 * n_A_ActiveSkillLV, n_Delay[2] = 1) : 419 == n_A_ActiveSkill ? (not_use_card = 1, wCast = .5, n_Delay[2] = 1, n_Enekyori = 1, wActiveHitNum = 5, (2 == n_B[2] || 7 == n_B[2]) && (wbairitu += 4)) : 423 == n_A_ActiveSkill ? (n_Enekyori = 1, n_Delay[2] = .5, n_A_Weapon_zokusei = 8, not_use_card = 1) : 428 == n_A_ActiveSkill ? (n_Enekyori = 1, wActiveHitNum = 5, wbairitu += .5 * n_A_ActiveSkillLV + 4, n_Delay[2] = 1.7) : 430 == n_A_ActiveSkill ? (SRV ? (n_A_Weapon_ATKplus > 8 && EquipNumSearch(1100) ? TCcast = 1.25 : EquipNumSearch(926) ? TCcast = .75 : TCcast = 1, wCast = (1 + .2 * n_A_ActiveSkillLV) * TCcast) : (wCast = 1 + .2 * n_A_ActiveSkillLV, cast_kotei = 1), n_Enekyori = 1, wbairitu += 1 * n_A_ActiveSkillLV + 1, n_Delay[2] = 1, w_HIT = 5 * w_HIT + 5, w_HIT > 100 && (w_HIT = 100), w_HIT_HYOUJI = w_HIT) : 431 == n_A_ActiveSkill ? (wCast = 2, n_Delay[2] = 1, n_Enekyori = 1) : 432 == n_A_ActiveSkill ? (wCast = 1.5, n_Enekyori = 1, wbairitu += .2 * n_A_ActiveSkillLV, n_Delay[2] = .5, w_HIT = 100, w_HIT_HYOUJI = 100) : 434 == n_A_ActiveSkill ? (cast_kotei = 1, wCast = 1, n_Enekyori = 0, wbairitu += .5 * n_A_ActiveSkillLV, n_Delay[3] = 1) : 435 == n_A_ActiveSkill ? (n_Enekyori = 1, wbairitu += 1 * n_A_ActiveSkillLV + 2, n_Delay[2] = 1 + .2 * n_A_ActiveSkillLV) : 436 == n_A_ActiveSkill ? (n_Enekyori = 1, wbairitu += .2 * n_A_ActiveSkillLV - .2, wCast = 1, n_Delay[2] = 1) : 437 == n_A_ActiveSkill && (n_Enekyori = 1, not_use_card = 1, wCast = 1, n_Delay[2] = 1);
            ATKbai02(wbairitu, 0), 0 == cast_kotei && (SRV && 430 == n_A_ActiveSkill ? wCast = wCast : wCast *= n_A_CAST);
            for (var i = 0; 2 >= i; i++) w_MagiclBulet = i, w_DMG[i] = BattleCalc(n_A_DMG[i], i), wActiveHitNum > 1 && (w_DMG[i] = Math.floor(w_DMG[i] / wActiveHitNum) * wActiveHitNum), Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + EDP_DMG(i), InnStr[i] += Last_DMG_A[i], wActiveHitNum > 1 && (InnStr[i] += " (" + w_DMG[i] / wActiveHitNum + " x " + wActiveHitNum + "Hit)");
            w_MagiclBulet = 1, w_DMG[1] = (w_DMG[1] * w_HIT + BattleCalc2(0) * (100 - w_HIT)) / 100, EDPplus(1), 0 == cast_kotei && CastAndDelay(), BattleCalc998()
        } else if (275 == n_A_ActiveSkill) {
            n_Enekyori = 1, wCast = .3, n_Delay[2] = .3, wCast *= n_A_CAST;
            for (var i = 0; 2 >= i; i++) SRV ? (w_DMG[i] = BattleCalc(BK_n_A_MATK[i] * (1 + .05 * n_A_Buf2[8]), i) - 1, 0 == n_A_WeaponType && (w_DMG[1] = w_DMG[0], w_DMG[2] = w_DMG[0])) : w_DMG[i] = BattleCalc(BK_n_A_MATK[i], i), Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + EDP_DMG(i), InnStr[i] += Last_DMG_A[i];
            n_PerHIT_DMG = BattleCalc2(0) + n_A_WeaponLV_seirenATK, w_DMG[1] = (w_DMG[1] * w_HIT + n_PerHIT_DMG * (100 - w_HIT)) / 100, EDPplus(1), CastAndDelay(), BattleCalc998()
        } else if (40 == n_A_ActiveSkill || 70 == n_A_ActiveSkill || 111 == n_A_ActiveSkill || 192 == n_A_ActiveSkill || 76 == n_A_ActiveSkill || 418 == n_A_ActiveSkill || 391 == n_A_ActiveSkill || 429 == n_A_ActiveSkill) {
            if (40 == n_A_ActiveSkill) n_Enekyori = 1, wbairitu += .1 * n_A_ActiveSkillLV - .1, wHITsuu = 2;
            else if (70 == n_A_ActiveSkill) wbairitu += .1 * n_A_ActiveSkillLV, wHITsuu = n_B[4] + 1;
            else if (76 == n_A_ActiveSkill) wbairitu += .4 * n_A_ActiveSkillLV, wCast = .7 * n_A_CAST, wHITsuu = 2, 1 == n_A_ActiveSkillLV && (wHITsuu = 1), wLAch = 1, 1 == n_B_IJYOU[6] && (wHITsuu = 3, 1 == n_A_ActiveSkillLV && (wHITsuu = 2));
            else if (192 == n_A_ActiveSkill) wbairitu += .5 * n_A_ActiveSkillLV, w = n_A_Buf2[12], w > n_A_ActiveSkillLV && (w = n_A_ActiveSkillLV), wHITsuu = w, wCast = (1 + w) * n_A_CAST, n_Delay[2] = .5, n_Enekyori = 1;
            else if (418 == n_A_ActiveSkill) n_Enekyori = 1, n_Delay[2] = 1, wbairitu += .5, wHITsuu = 3;
            else if (391 == n_A_ActiveSkill) n_Delay[0] = 1, n_Enekyori = 1, wbairitu += .08 * n_A_STR - .5, wHITsuu = 2;
            else if (429 == n_A_ActiveSkill) {
                n_Enekyori = 0, wbairitu += .5 * n_A_ActiveSkillLV - .5, n_Delay[2] = 1;
                var DEATH = [1, 1.2, 1.6, 2, 2.4, 3, 3.6, 4, 5, 6, 7, 8, 9, 10];
                wHITsuu = DEATH[eval(document.calcForm.SkillSubNum.value)]
            }
            ATKbai02(wbairitu, 0);
            for (var i = 0; 2 >= i; i++) w_DMG[i] = Math.floor((BattleCalc((n_A_DMG[i] * wHITsuu), i)) / wHITsuu), 391 == n_A_ActiveSkill && 2 != n_B[2] && 4 != n_B[2] && (w_DMG[i] = 0), w_DMG[i] += EDP_DMG(i), Last_DMG_A[i] = w_DMG[i] * wHITsuu, Last_DMG_B[i] = w_DMG[i], 0 == n_B_IJYOU[6] || 0 == wLAch ? InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8] + wHITsuu + "hit)" : (InnStr[i] += 3 * w_DMG[i] + "(" + 2 * w_DMG[i] + "+" + w_DMG[i] + ")", Last_DMG_B[i] = 3 * w_DMG[i]), w_DMG[i] -= EDP_DMG(i), w_DMG[i] *= wHITsuu;
            var wX = BattleCalc2(0);
            w_DMG[1] = (w_DMG[1] * w_HIT + wX * wHITsuu * (100 - w_HIT)) / 100, 0 == wHITsuu && 192 == n_A_ActiveSkill && (InnStr[0] = "<B style='color:red'># of Spirit Spheres must be higher than 0<BR>Please change it at [Supportive/Party Skills]</B>"), EDPplus(wHITsuu), n_PerHIT_DMG = wX * wHITsuu, str_PerHIT_DMG = wX * wHITsuu + " (" + wHITsuu + SubName[8] + wX + " Damage)", CastAndDelay(), BattleCalc998()
        } else if (118 == n_A_ActiveSkill || 271 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 0, n_Enekyori = 1, wBT = 80 + 2 * Math.floor(n_A_DEX / 10) + 2 * Math.floor(n_A_INT / 2) + 6 * SkillSearch(119), 271 == n_A_ActiveSkill ? (wBT = Math.floor(wBT * (150 + 70 * n_A_ActiveSkillLV) / 100), wBT = Math.floor(wBT * zokusei[n_B[3]][0]), wBT = tPlusDamCut(wBT), wBT *= 5, 5 == n_B[19] && (wBT = 1), wCast = 1 * n_A_CAST, n_Delay[2] = 3) : (wBT = Math.floor(wBT * zokusei[n_B[3]][0]), wBT = tPlusDamCut(wBT), wBT *= n_A_ActiveSkillLV, wCast = 1.5 * n_A_CAST, n_Delay[2] = 1);
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = wBT, InnStr[i] += Last_DMG_A[i], 118 == n_A_ActiveSkill && (Last_DMG_B[i] = wBT / n_A_ActiveSkillLV, InnStr[i] += " (" + Last_DMG_B[i] + " x " + n_A_ActiveSkillLV + "Hit)"), w_DMG[i] = wBT;
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (17 == n_A_ActiveSkill || 86 == n_A_ActiveSkill && (n_B[3] < 50 || 60 <= n_B[3])) {
            ATKbai02(wbairitu, 0), n_A_Weapon_zokusei = 5, wINV = Math.floor(BattleCalc2(0) * zokusei[n_B[3]][5]), n_PerHIT_DMG = wINV;
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BattleCalc(n_A_DMG[i], i), w_DMG[i] = Math.floor(w_DMG[i] * zokusei[n_B[3]][5]), Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + EDP_DMG(i), InnStr[i] += Last_DMG_A[i];
            w_DMG[1] = (w_DMG[1] * w_HIT + wINV * (100 - w_HIT)) / 100, EDPplus(1), CastAndDelay(), BattleCalc998()
        } else if (159 == n_A_ActiveSkill || 384 == n_A_ActiveSkill) {
            if(n_PerHIT_DMG=0,n_Enekyori=1,n_A_Weapon_zokusei=0,n_Delay[2]=.7,wbairitu2=1+.3*n_A_ActiveSkillLV,384==n_A_ActiveSkill&&(n_Delay[2]=.35),SRV){wSBr=10*n_A_LEFT_DEF_PLUS,EquipNumSearch(620)||EquipNumSearch(409)||CardNumSearch(255)||EquipNumSearch(43)?(M_DEF1=n_B[14],M_DEF2=n_B_DEF2[0]):(EquipNumSearch(393)||EquipNumSearch(904))&&7==n_B[2]?(M_DEF1=n_B[14],M_DEF2=n_B_DEF2[0]):(EquipNumSearch(392)||EquipNumSearch(401))&&3==n_B[2]?(M_DEF1=n_B[14],M_DEF2=n_B_DEF2[0]):(EquipNumSearch(467)||EquipNumSearch(405)||EquipNumSearch(471))&&9==n_B[2]?(M_DEF1=n_B[14],M_DEF2=n_B_DEF2[0]):EquipNumSearch(394)&&6==n_B[2]?(M_DEF1=n_B[14],M_DEF2=n_B_DEF2[0]):(M_DEF1=0,M_DEF2=0);var SB_ATK=n_A_ATK+.05*n_A_ATK*n_A_Buf2[8];SkillSearch(12)&&(SB_ATK+=.32*SB_ATK),n_A_Buf6[5]&&(SB_ATK+=Math.floor((.02+.03*n_A_Buf6[5])*SB_ATK)),n_A_Buf6[5]&&n_A_Buf7[31]?SB_ATK+=0:n_A_Buf7[31]&&(SB_ATK+=Math.floor(.05*SB_ATK));for(var i=0;i<=2;i++)0<n_tok[23]?(n_A_ATK_IP=Math.round((SB_ATK+ItemOBJ[n_A_Equip[5]][6])*(n_B_DEF2[2-i]+n_B[14])/100),w_DMG[i]=n_A_ATK_IP*wbairitu):w_DMG[i]=(SB_ATK+ItemOBJ[n_A_Equip[5]][6])*wbairitu,w_DMG[i]=Math.floor(w_DMG[i]*wbairitu2),384==n_A_ActiveSkill&&(w_DMG[i]=Math.floor(2*w_DMG[i])),w_DMG[i]=w_DMG[i]*(100+StPlusCalc2(5384)+StPlusCard(5384))/100,w_DMG[i]=Math.floor(w_DMG[i]*(100-(n_B[14]-M_DEF1))/100-(n_B_DEF2[i]-M_DEF2)),w_DMG[i]=BaiCI(w_DMG[i])+wSBr,0!=M_DEF1&&(w_DMG[2]=w_DMG[1]=w_DMG[0]),w_DMG[i]<1&&(w_DMG[i]=1),305==ItemOBJ[n_A_Equip[5]][0]&&(w_DMG[i]=0),w_DMG[i]=Math.floor(w_DMG[i]*zokusei[n_B[3]][0]),Last_DMG_A[i]=Last_DMG_B[i]=w_DMG[i],InnStr[i]+=Last_DMG_A[i]
            } else {
                wSBr = 4 * n_A_LEFT_DEF_PLUS, n_A_ATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10)), n_A_ATK = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5);
                for (var i = 0; 2 >= i; i++) w_DMG[i] = n_A_ATK * wbairitu + ItemOBJ[n_A_Equip[5]][6] + wSBr, w_DMG[i] = Math.floor(Math.floor(w_DMG[i] * (100 - n_B[14]) / 100 - n_B_DEF2[i]) * wbairitu2), w_DMG[i] = BaiCI(w_DMG[i]), w_DMG[i] < 1 && (w_DMG[i] = 1), w_DMG[i] = Math.floor(w_DMG[i] * zokusei[n_B[3]][0]), Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i]
            }
            w_DMG[1] = w_DMG[1] * w_HIT / 100, CastAndDelay(), BattleCalc998()
        } else if (324 == n_A_ActiveSkill) {
            if (SRV) {
                n_PerHIT_DMG = 0, n_Enekyori = 1, n_A_Weapon_zokusei = 0, wCast = 1 * n_A_CAST, n_Delay[2] = 1, wSC = ItemOBJ[n_A_Equip[5]][6], wbairitu2 = 1 + .3 * n_A_ActiveSkillLV;
                var n_A_ATK_WC = 20 * CardNumSearch(11) + 5 * CardNumSearch(28) + 5 * CardNumSearch(29) + 5 * CardNumSearch(30) + 5 * CardNumSearch(33) + 5 * CardNumSearch(34) + 5 * CardNumSearch(35) + 5 * CardNumSearch(36) + 5 * CardNumSearch(37) + 10 * CardNumSearch(38) + 5 * CardNumSearch(39) + 5 * CardNumSearch(110) + 5 * CardNumSearch(163) + 5 * CardNumSearch(165) + 5 * CardNumSearch(254) + 5 * CardNumSearch(259) + 5 * CardNumSearch(326) + 5 * CardNumSearch(356) + 5 * CardNumSearch(366) + 5 * CardNumSearch(380) + 5 * CardNumSearch(463) + 5 * CardNumSearch(498) + 5 * CardNumSearch(525),
                    SC_ATK = n_A_ATK - n_A_ATK_WC;
                SkillSearch(12) && (SC_ATK += .32 * SC_ATK), n_A_Buf6[5] && (SC_ATK += Math.floor((.02 + .03 * n_A_Buf6[5]) * SC_ATK)), n_A_Buf6[5] && n_A_Buf7[31] ? SC_ATK += 0 : n_A_Buf7[31] && (SC_ATK += Math.floor(.05 * SC_ATK));
                for (var i = 0; 2 >= i; i++) w_DMG[i] = SC_ATK * wbairitu + ItemOBJ[n_A_Equip[5]][6], w_DMG[i] = Math.floor(Math.floor(5 * w_DMG[i] * (100 - n_B[14]) / 100 - n_B_DEF2[i]) * wbairitu2) + 5 * n_A_LEFT_DEF_PLUS * 2, w_DMG[i] = BaiCI(w_DMG[i]), w_DMG[i] < 1 && (w_DMG[i] = 1), 305 == ItemOBJ[n_A_Equip[5]][0] ? (w_DMG[i] = 0, InnStr[i] += w_DMG[i] + " (no shield equiped)") : (w_DMG[i] = Math.floor(w_DMG[i] * zokusei[n_B[3]][0]), Last_DMG_B[i] = Math.floor(w_DMG[i] / 5) + Math.floor(n_B_DEF2[i] / 3 - 1), Last_DMG_A[i] = 5 * Last_DMG_B[i], InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8] + "5hit)", w_DMG[i] = Last_DMG_A[i]);
                w_DMG[1] = w_DMG[1] * w_HIT / 100
            } else {
                n_PerHIT_DMG = 0, n_Enekyori = 1, n_A_Weapon_zokusei = 0, wCast = 1 * n_A_CAST, n_Delay[2] = 1, wSBr = n_A_LEFT_DEF_PLUS, wSC = ItemOBJ[n_A_Equip[5]][6], wbairitu2 = 1 + .3 * n_A_ActiveSkillLV, n_A_ATK_w = Math.round(Math.floor(n_A_STR / 10) * Math.floor(n_A_STR / 10)), n_A_ATK = n_A_STR + n_A_ATK_w + Math.floor(n_A_DEX / 5) + Math.floor(n_A_LUK / 5), n_A_ATK = n_A_ATK * wbairitu + wSC + 4 * wSBr, wSC -= 100, wSC < 0 && (wSC = 0), wSC2 = [0, 0, 0], wSC2[2] = 100 + wSC + 2 * wSBr * wSBr, wSC2[1] = 100 + (wSC + 2 * wSBr * wSBr) / 2, wSC2[0] = 100;
                for (var i = 0; 2 >= i; i++) w_DMG[i] = (n_A_ATK * (100 - n_B[14]) / 100 - n_B_DEF2[i]) * wbairitu2, w_DMG[i] += wSC2[i], w_DMG[i] = BaiCI(w_DMG[i]), w_DMG[i] < 1 && (w_DMG[i] = 1), 305 == ItemOBJ[n_A_Equip[5]][0] && (w_DMG[i] = 0), w_DMG[i] = Math.floor(w_DMG[i] * zokusei[n_B[3]][0]), Last_DMG_A[i] = 5 * w_DMG[i], Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8] + "5hit)", w_DMG[i] = Last_DMG_A[i];
                w_DMG[1] = w_DMG[1] * w_HIT / 100
            }
            CastAndDelay(), BattleCalc998()
        } else if (259 == n_A_ActiveSkill) {
            n_Enekyori = 1, SRV ? wSPP2 = n_A_Weapon_ATKplus * zokusei[n_B[3]][n_A_Weapon_zokusei] : wSPP2 = n_A_WeaponLV_seirenATK * zokusei[n_B[3]][n_A_Weapon_zokusei], wSPP2 = BaiCI(wSPP2), wSPP2 = tPlusDamCut(wSPP2), n_PerHIT_DMG = 5 * wSPP2, 5 == n_A_ActiveSkillLV ? wCast = 1 * n_A_CAST : wCast = (.1 + .2 * n_A_ActiveSkillLV) * n_A_CAST, n_Delay[2] = 1 + .2 * n_A_ActiveSkillLV, wSPP = Math.floor(n_A_STR / 10), w_DMG[2] = wSPP * wSPP + .8 * ItemOBJ[n_A_Equip[0]][6] * (1 + .5 * n_A_ActiveSkillLV), wSPP = 1.25 - .25 * n_B[4], SRV ? w_DMG[2] = Math.floor(w_DMG[2] * wSPP) + 10 + 1.5 * n_A_Weapon_ATKplus : w_DMG[2] = Math.floor(w_DMG[2] * wSPP + n_A_WeaponLV_seirenATK), w_DMG[2] = w_DMG[2] * zokusei[n_B[3]][n_A_Weapon_zokusei], w_DMG[2] = BaiCI(w_DMG[2]), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_B[i] = w_DMG[i] + EDP_DMG(i), Last_DMG_A[i] = 5 * Last_DMG_B[i], InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8] + "5hit)", w_DMG[i] = Last_DMG_A[i];
            w_DMG[1] = w_DMG[1] * w_HIT / 100 + n_PerHIT_DMG * (100 - w_HIT) / 100, EDPplus(5), CastAndDelay(), BattleCalc998()
        } else if (88 == n_A_ActiveSkill) {
            if (n_PerHIT_DMG = 0, not_use_card = 1, n_Delay[0] = 1, wCast = 1 * n_A_CAST, 0 == n_B[19]) {
                wbairitu += (400 + 50 * n_A_ActiveSkillLV + 20 * eval(document.calcForm.SkillSubNum.value)) / 100, ATKbai02(wbairitu, 0);
                for (var i = 0; 2 >= i; i++) w_DMG[i] = BattleCalc(n_A_DMG[i], i), w_DMG[i] = Math.floor(w_DMG[i])
            } else 5 == n_B[19] ? w_DMG[0] = w_DMG[1] = w_DMG[2] = 1 : w_DMG[0] = w_DMG[1] = w_DMG[2] = 0;
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (263 == n_A_ActiveSkill) {
            not_use_card = 1, n_Enekyori = 1, wCast = .5 * n_A_CAST, n_Delay[2] = .8 + .2 * n_A_ActiveSkillLV, w_SBr = new Array, w = 5 * n_A_INT * n_A_ActiveSkillLV, w_SBr[2] = w + 1e3 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[2]) / 2), w_SBr[1] = w + 750 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[1]) / 2), w_SBr[0] = w + 500 - Math.floor((n_B[14] + n_B[15] + n_B_MDEF2 + n_B_DEF2[0]) / 2);
            for (var i = 0; 2 >= i; i++) w_SBr[i] = tPlusDamCut(w_SBr[i]);
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BattleCalc(n_A_DMG[i], i), w_DMG[i] *= n_A_ActiveSkillLV, Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] + w_SBr[i], InnStr[i] += Last_DMG_A[i] + " (" + w_DMG[i] + " + " + w_SBr[i] + ")", w_DMG[i] = Last_DMG_A[i];
            var wX = BattleCalc2(0) * n_A_ActiveSkillLV;
            if (n_PerHIT_DMG = wX + w_SBr[1], str_PerHIT_DMG = wX + w_SBr[0] + "~" + (wX + w_SBr[2]), 5 == n_B[19])
                for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] = 1, InnStr[i] += Last_DMG_A[i];
            w_DMG[1] = (w_DMG[1] * w_HIT + n_PerHIT_DMG * (100 - w_HIT)) / 100, CastAndDelay(), BattleCalc998()
        } else if (162 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, myInnerHtml("CRIATKname", '<Font color="#FF0000">Health Drain</Font>', 0), myInnerHtml("CRIATK", '<Font color="#FF0000">' + Math.floor(n_A_MaxHP / 5) + "</Font>", 0), myInnerHtml("CRInumname", '<Font color="#FF0000">Damage Backlash</Font>', 0), work_A_VITDEF = [0, 0, 0], work_A_VITDEF[0] = n_A_VITDEF[2], work_A_VITDEF[1] = n_A_VITDEF[1], work_A_VITDEF[2] = n_A_VITDEF[0], n_A_softMDEF = n_A_INT + Math.floor(n_A_VIT / 2);
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BK_n_A_DMG[i] * (100 - n_A_DEF) / 100 - work_A_VITDEF[i] + n_A_WeaponLV_seirenATK, w_DMG[i] = Math.floor(w_DMG[i] * (wbairitu + .4 * n_A_ActiveSkillLV)), w = BK_n_A_MATK[i] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w = Math.floor(w * (.4 * n_A_ActiveSkillLV + 1)), w_DMG[i] += w, w_DMG[i] = Math.floor(w_DMG[i] * (100 - n_tok[57]) / 100), w_DMG[i] = Math.floor(w_DMG[i] * (100 - n_tok[66]) / 100), w_DMG[i] = Math.floor(w_DMG[i] * (100 - n_tok[78]) / 100), eval(document.calcForm.A_youshi.checked) ? w_DMG[i] = Math.floor(w_DMG[i] * (100 - n_tok[190]) / 100) : w_DMG[i] = Math.floor(w_DMG[i] * (100 - n_tok[191]) / 100), w_DMG[i] = Math.floor(w_DMG[i] * zokusei[10 * n_A_BodyZokusei + 1][6]), w_DMG[i] = Math.floor(w_DMG[i] / 2), EquipNumSearch(1433) && (w_DMG[i] = Math.floor(1.172 * w_DMG[i]));
            myInnerHtml("CRInum", '<Font color="#FF0000">3' + SubName[8] + w_DMG[0] + "~" + w_DMG[2] + " Damage</Font>", 0), n_Enekyori = 2, n_A_Weapon_zokusei = 6, wCast = 3 * n_A_CAST, n_Delay[2] = 1.5, wLAch = 1;
            for (var i = 0; 2 >= i; i++) w_DMG[i] = BK_n_A_DMG[i] * (100 - n_B[14]) / 100 - n_B_DEF2[i] + n_A_WeaponLV_seirenATK, w_DMG[i] *= wbairitu + .4 * n_A_ActiveSkillLV, w_DMG[i] = Math.floor(w_DMG[i] * zokusei[n_B[3]][6]), w = BK_n_A_MATK[i] * (100 - n_B[15]) / 100 - n_B_MDEF2, w *= .4 * n_A_ActiveSkillLV + 1, w = Math.floor(w * zokusei[n_B[3]][6]), w_DMG[i] = tPlusDamCut(Math.floor((w + w_DMG[i]) * zokusei[n_B[3]][6])), EquipNumSearch(1433) && (w_DMG[i] = Math.floor(1.1 * w_DMG[i])), w_DMG[i] < 1 && (w_DMG[i] = 1), 60 <= n_B[3] && n_B[3] <= 69 && (w_DMG[i] = 0);
            if (0 == n_B_IJYOU[6])
                for (var b = 0; 2 >= b; b++) Last_DMG_A[b] = Last_DMG_B[b] = 3 * w_DMG[b], InnStr[b] += Last_DMG_A[b] + " (" + w_DMG[b] + SubName[8] + "3hit)", w_DMG[b] = Last_DMG_A[b];
            else
                for (var b = 0; 2 >= b; b++) Last_DMG_A[b] = Last_DMG_B[b] = 4 * w_DMG[b], InnStr[b] += Last_DMG_A[b] + " (" + 2 * w_DMG[b] + " + " + w_DMG[b] + SubName[8] + "2hit)", w_DMG[b] = Last_DMG_A[b];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (66 == n_A_ActiveSkill) {
            for (wCR = 100, n_PerHIT_DMG = Math.floor(2 * BattleCalc2(0) * zokusei[n_B[3]][0]), SkillSearch(327) ? wCR += 20 * SkillSearch(327) : (SkillSearch(154) && (wCR += 5 * SkillSearch(154)), 0 == SkillSearch(154) && n_A_Buf2[8] && (wCR += 5 * n_A_Buf2[8])), CR_n_A_DMG = [0, 0, 0], CRbai = eval(document.calcForm.SkillSubNum.value) / 8e3, b = 0; 2 >= b; b++) CR_n_A_DMG[b] = Math.floor(n_A_DMG[b] * wCR / 100);
            wbairitu += .5, ATKbai02(wbairitu, 0);
            for (var b = 0; 2 >= b; b++) w_DMG[b] = BattleCalc(n_A_DMG[b], b), w_DMG[b] += Math.floor(BattleCalc(CR_n_A_DMG[b], b) * CRbai), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), SRV && (!CardNumSearch(523) || 6 != n_A_JOB && 12 != n_A_JOB && 19 != n_A_JOB && 26 != n_A_JOB && 33 != n_A_JOB && 40 != n_A_JOB || (w_DMG[0] *= 1.5, w_DMG[1] *= 1.5, w_DMG[2] *= 1.5)), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b] + EDP_DMG(b), InnStr[b] += Last_DMG_A[b];
            w_DMG[1] = (w_DMG[1] * w_HIT + 2 * BattleCalc2(0) * (100 - w_HIT)) / 100, w_DMG[1] = Math.floor(w_DMG[1] * zokusei[n_B[3]][0]), EDPplus(1), CastAndDelay(), BattleCalc998()
        } else if (283 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, w_DMG[2] = 500 + 300 * n_A_ActiveSkillLV, 5 == n_B[19] && (w_DMG[2] = 1), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            wCast = (1.5 + .5 * n_A_ActiveSkillLV) * n_A_CAST, n_Delay[2] = 1.5 + .5 * n_A_ActiveSkillLV, SRV && (n_Delay[2] = (1.5 + .5 * n_A_ActiveSkillLV) / 2), w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (284 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 0, w_DMG[2] = Math.floor(.09 * n_A_MaxHP * (.9 + .1 * n_A_ActiveSkillLV)), w_DMG[2] = BaiCI(w_DMG[2]), w_DMG[2] = Math.floor(w_DMG[2] * zokusei[n_B[3]][0]), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (193 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, w_HIT_HYOUJI = 100, n_A_Weapon_zokusei = 0, ATKbai02(wbairitu, 0), wbairitu += .75 * n_A_ActiveSkillLV, work_B_DEF2 = [0, 0, 0], work_B_DEF2[0] = n_B_DEF2[2], work_B_DEF2[1] = n_B_DEF2[1], work_B_DEF2[2] = n_B_DEF2[0];
            for (var b = 0; 2 >= b; b++) w_DMG[b] = Math.floor(Math.floor(BK_n_A_DMG[b] * wbairitu) * (work_B_DEF2[b] + n_B[14]) / 50), w_DMG[b] = BaiCI(w_DMG[b]), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), SRV ? Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b] + EDP_DMG(b) + 3 * eval(document.calcForm.SkillSubNum.value) : Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b] + EDP_DMG(b), InnStr[b] += Last_DMG_A[b];
            EDPplus(1), wCast = 1 * n_A_CAST, n_Delay[2] = .5,
                CastAndDelay(), BattleCalc998()
        } else if (197 == n_A_ActiveSkill || 321 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, w_HIT_HYOUJI = 100, n_A_Weapon_zokusei = 0, ATKbai02(wbairitu, 0), SRV ? 197 == n_A_ActiveSkill ? wbairitu += 8 + eval(document.calcForm.SkillSubNum.value) / 10 : wbairitu += 8 + (n_A_MaxSP - 1) / 10 : 197 == n_A_ActiveSkill ? wbairitu += 7 + eval(document.calcForm.SkillSubNum.value) / 10 : wbairitu += 7 + (n_A_MaxSP - 1) / 10, wASYU = 250 + 150 * n_A_ActiveSkillLV;
            for (var b = 0; 2 >= b; b++) w_DMG[b] = Math.floor(BK_n_A_DMG[b] * wbairitu) + wASYU, w_DMG[b] = BaiCI(w_DMG[b]), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), SRV && (n_A_Buf6[5] && (w_DMG[b] += Math.floor((.02 + .03 * n_A_Buf6[5]) * w_DMG[b])), n_A_Buf6[5] && n_A_Buf7[31] ? w_DMG[b] += 0 : n_A_Buf7[31] && (w_DMG[b] += Math.floor(.05 * w_DMG[b])), 1 == n_A_Buf2[19] && (w_DMG[b] = 2 * w_DMG[b])), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b] + EDP_DMG(b), InnStr[b] += Last_DMG_A[b];
            EDPplus(1), wCast = (4.5 - .5 * n_A_ActiveSkillLV) * n_A_CAST, n_Delay[2] = 3.5 - .5 * n_A_ActiveSkillLV, CastAndDelay(), BattleCalc998()
        } else if (394 == n_A_ActiveSkill) {
            n_Enekyori = 1, not_use_card = 1, ATKbai02(wbairitu, 0);
            for (var b = 0; 2 >= b; b++) w_DMG[b] = BattleCalc(n_A_DMG[b], b), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b], InnStr[b] += Last_DMG_A[b];
            w_DMG[1] = (w_DMG[1] * w_HIT + BattleCalc2(0) * zokusei[n_B[3]][0] * (100 - w_HIT)) / 100, n_PerHIT_DMG = BattleCalc2(0) * zokusei[n_B[3]][0], CastAndDelay(), BattleCalc998()
        } else if (395 == n_A_ActiveSkill) {
            n_Enekyori = 1, n_Delay[2] = 1, not_use_card = 1, ATKbai02(wbairitu, 0), n_A_Weapon_zokusei = KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][1];
            for (var b = 0; 2 >= b; b++) w_DMG[b] = BattleCalc(n_A_DMG[b], b), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), Last_DMG_B[b] = w_DMG[b], Last_DMG_A[b] = 3 * w_DMG[b], InnStr[b] += Last_DMG_A[b] + " (" + Last_DMG_B[b] + SubName[8] + "3hit)", w_DMG[b] = Last_DMG_A[b];
            var wX = Math.floor(BattleCalc2(0) * zokusei[n_B[3]][0]);
            w_DMG[1] = (w_DMG[1] * w_HIT + 3 * wX * (100 - w_HIT)) / 100, n_PerHIT_DMG = 3 * wX, str_PerHIT_DMG = 3 * wX + " (3" + SubName[8] + wX + " Damage)", CastAndDelay(), BattleCalc998()
        } else if (396 == n_A_ActiveSkill) {
            wbairitu += 1.5 * n_A_ActiveSkillLV + .5, n_Enekyori = 1, ATKbai02(wbairitu, 0), wCast = 3 * n_A_CAST, n_Delay[2] = 3, wActiveHitNum = 2 + Math.round(n_A_ActiveSkillLV / 2);
            for (var b = 0; 2 >= b; b++) w_DMG[b] = BattleCalc(n_A_DMG[b], b), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), wActiveHitNum > 1 && (w_DMG[b] = Math.floor(w_DMG[b] / wActiveHitNum) * wActiveHitNum), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b], InnStr[b] += Last_DMG_A[b], InnStr[b] += " (" + Last_DMG_A[b] / wActiveHitNum + " x " + wActiveHitNum + "Hit)";
            w_DMG[1] = (w_DMG[1] * w_HIT + BattleCalc2(0) * zokusei[n_B[3]][0] * (100 - w_HIT)) / 100, n_PerHIT_DMG = BattleCalc2(0) * zokusei[n_B[3]][0], CastAndDelay(), BattleCalc998()
        } else if (405 == n_A_ActiveSkill || 438 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 0, n_Enekyori = 1, ATKbai02(wbairitu, 0), 405 == n_A_ActiveSkill ? w_1senHP = eval(document.calcForm.SkillSubNum.value) : w_1senHP = n_A_MaxHP - 1, SRV ? w_DMG[0] = 40 * (n_A_STR - SkillSearch(404)) + n_A_ActiveSkillLV * (w_1senHP / 10 + 35) : w_DMG[0] = 40 * (n_A_STR + n_A_ActiveSkillLV) + w_1senHP * (n_A_BaseLV / 100) * n_A_ActiveSkillLV / 10, w_DMG[0] = w_DMG[0] * (100 - n_B[14]) / 100, w_DMG[0] = BaiCI(w_DMG[0]), w_DMG[0] = Math.floor(w_DMG[0] * zokusei[n_B[3]][0]), w_DMG[2] = w_DMG[1] = w_DMG[0];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            CastAndDelay(), w_HIT_HYOUJI = 100, BattleCalc998()
        } else if (244 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, not_use_card = 1, n_Enekyori = 1, n_A_Weapon_zokusei = 0, wbairitu = (50 + 50 * n_A_ActiveSkillLV) / 100;
            for (var b = 0; 2 >= b; b++) w_DMG[b] = Math.floor((BK_n_A_DMG[b] - n_B_DEF2[b]) * wbairitu), w_DMG[b] = Math.floor(w_DMG[b] * zokusei[n_B[3]][0]), w_DMG[b] = Math.floor(BaiCI(w_DMG[b])), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b], InnStr[b] += Last_DMG_A[b];
            wCast = 1 * n_A_CAST, w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (328 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_Enekyori = 1, n_A_Weapon_zokusei = 0, wHITsuu = n_A_ActiveSkillLV, wAD = .7 * n_A_INT * n_A_INT * n_B[7] / (n_A_INT + n_B[7]), w_DMG[2] = Math.floor(wAD), w_DMG[2] = tPlusDamCut(Math.floor(w_DMG[2] * zokusei[n_B[3]][0])), 1 == Taijin && (w_DMG[2] = Math.floor(w_DMG[2] / 2)), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_B[i] = w_DMG[i], Last_DMG_A[i] = w_DMG[i] * wHITsuu, InnStr[i] += Last_DMG_A[i] + " (" + Last_DMG_B[i] + SubName[8] + wHITsuu + "hit)", w_DMG[i] = Last_DMG_A[i];
            wCast = 1 * n_A_CAST, n_Delay[2] = 1, w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (106 == n_A_ActiveSkill || 112 == n_A_ActiveSkill || 113 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_Delay[0] = 1, 106 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 2, w_DMG[2] = Math.floor((75 + n_A_DEX) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * zokusei[n_B[3]][2])) : 112 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, w_DMG[2] = Math.floor((50 + n_A_DEX / 2) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * zokusei[n_B[3]][4])) : 113 == n_A_ActiveSkill && (n_A_Weapon_zokusei = 3, w_DMG[2] = Math.floor((75 + n_A_DEX / 2) * (1 + n_A_INT / 100) * n_A_ActiveSkillLV * zokusei[n_B[3]][3])), w_DMG[2] = tPlusDamCut(w_DMG[2]), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (25 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 6, n_Delay[2] = 1, n_Enekyori = 2, w_DMG[2] = HealCalc(n_A_ActiveSkillLV, 0), w_DMG[2] = Math.floor(Math.floor(w_DMG[2] / 2) * zokusei[n_B[3]][6]), n_B[3] < 90 && (w_DMG[2] = 0);
            var wX = n_tok[170 + n_B[2]];
            w_DMG[2] = Math.floor(w_DMG[2] * (100 + wX) / 100), wHealBAI = 100 + n_tok[93], w_DMG[2] = Math.floor(w_DMG[2] * wHealBAI / 100), w_DMG[2] = tPlusDamCut(w_DMG[2]), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (94 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 6, wCast = 5 * n_A_CAST, n_Delay[0] = 1, n_Enekyori = 2, n_A_ActiveSkillLV <= 6 ? w_DMG[2] = 100 * n_A_ActiveSkillLV : w_DMG[2] = 777, w_HEAL_BAI = 100 + n_tok[94], w_DMG[2] = Math.floor(w_DMG[2] * w_HEAL_BAI / 100), w_DMG[2] = Math.floor(Math.floor(w_DMG[2] / 2) * zokusei[n_B[3]][6]), n_B[3] < 90 && 6 != n_B[2] && (w_DMG[2] = 0);
            var wX = n_tok[170 + n_B[2]];
            w_DMG[2] = Math.floor(w_DMG[2] * (100 + wX) / 100), w_HEAL_BAI = 100 + n_tok[96], w_DMG[2] = Math.floor(w_DMG[2] * w_HEAL_BAI / 100), w_DMG[2] = tPlusDamCut(w_DMG[2]), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i], InnStr[i] += Last_DMG_A[i];
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (102 == n_A_ActiveSkill || 97 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, 102 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 6, wCast = 1 * n_A_CAST) : (n_A_Weapon_zokusei = 0, wCast = 8 - 2 * n_A_ActiveSkillLV, wCast *= n_A_CAST), n_Enekyori = 2, n_B[3] < 90 ? (w = 0, w_DMG[2] = 0, w_DMG[0] = 0, w_DMG[1] = 0) : (1 != n_B[19] ? (w = (20 * n_A_ActiveSkillLV + n_A_BaseLV + n_A_INT + n_A_LUK) / 1e3, w_DMG[2] = n_B[6]) : (w = 0, w_DMG[2] = 0), w_DMG[0] = n_A_BaseLV + n_A_INT + 10 * n_A_ActiveSkillLV, w_DMG[0] = Math.floor(w_DMG[0] * zokusei[n_B[3]][n_A_Weapon_zokusei]), w_DMG[1] = Math.round(n_B[6] * w + w_DMG[0] * (100 - w) / 100));
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i];
            InnStr[0] += w_DMG[0] + " (Damage on Failure)", InnStr[1] += w_DMG[1] + "", InnStr[2] += Math.floor(w_DMG[2] * zokusei[n_B[3]][n_A_Weapon_zokusei]) + " (" + Math.floor(1e4 * w) / 100 + "% Success Chance)", n_Delay[2] = 3, w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else if (325 == n_A_ActiveSkill) {
            n_PerHIT_DMG = 0, n_A_Weapon_zokusei = 0, n_Delay[6] = 9, n_Enekyori = 2, wHITsuu = 4 + n_A_ActiveSkillLV, w_DMG[2] = 200 + 200 * n_A_ActiveSkillLV, w_DMG[2] = Math.floor(w_DMG[2]), 5 == n_B[19] && (w_DMG[2] = 1), 44 == n_B[0] && (w_DMG[2] = 400), w_DMG[0] = w_DMG[1] = w_DMG[2];
            for (var i = 0; 2 >= i; i++) Last_DMG_A[i] = Last_DMG_B[i] = w_DMG[i] * wHITsuu, w_DMG[i] = Last_DMG_A[i];
            var wStrG = Last_DMG_A[0] + " (" + w_DMG[0] / wHITsuu + " x " + wHITsuu + "hit)";
            for (i = 0; 2 >= i; i++) InnStr[i] += wStrG;
            wCast = 5 * n_A_CAST, n_Delay[2] = 2, w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        } else {
            if (n_PerHIT_DMG = 0, n_Enekyori = 2, wbairitu = 1, n_bunkatuHIT = 0, 51 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wHITsuu = n_A_ActiveSkillLV, wCast = .7 * n_A_ActiveSkillLV, n_Delay[2] = .8 + .2 * n_A_ActiveSkillLV) : 54 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, wHITsuu = n_A_ActiveSkillLV, wCast = .7 * n_A_ActiveSkillLV, n_Delay[2] = .8 + .2 * n_A_ActiveSkillLV) : 56 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wHITsuu = n_A_ActiveSkillLV, wCast = .7 * n_A_ActiveSkillLV, n_Delay[2] = .8 + .2 * n_A_ActiveSkillLV) : 52 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, n_A_ActiveSkillLV <= 5 ? (wCast = 1.5, n_Delay[2] = 1.5) : (wCast = 1, n_Delay[2] = 1), wbairitu = .7 + .1 * n_A_ActiveSkillLV) : 53 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wHITsuu = 4 + n_A_ActiveSkillLV, wCast = 2.15 - .15 * n_A_ActiveSkillLV, n_Delay[2] = .1, wbairitu = .5) : 55 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, wCast = .8, n_Delay[2] = 1.5, wbairitu = 1 + .1 * n_A_ActiveSkillLV) : 57 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wHITsuu = n_A_ActiveSkillLV, wCast = 1 * n_A_ActiveSkillLV, n_Delay[2] = 2, wbairitu = .8) : 46 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 8, wCast = .5, 10 == n_A_ActiveSkillLV ? n_Delay[2] = .5 : 9 == n_A_ActiveSkillLV ? n_Delay[2] = .6 : 8 == n_A_ActiveSkillLV ? n_Delay[2] = .7 : n_A_ActiveSkillLV >= 6 ? n_Delay[2] = .8 : n_A_ActiveSkillLV >= 4 ? n_Delay[2] = .9 : n_Delay[2] = 1, wbairitu = .7 + .1 * n_A_ActiveSkillLV) : 47 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 8, wHITsuu = Math.round(n_A_ActiveSkillLV / 2), wCast = .5, n_A_ActiveSkillLV % 2 == 0 ? n_Delay[2] = .8 + n_A_ActiveSkillLV / 2 * .2 : n_Delay[2] = 1 + (n_A_ActiveSkillLV + 1) / 2 * .2) : 122 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wHITsuu = n_A_ActiveSkillLV + 2, wCast = 3.3 - .3 * n_A_ActiveSkillLV, n_Delay[2] = 1, wbairitu = .2) : 124 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wCast = .7, n_Delay[2] = 2, wbairitu = 1 + .2 * n_A_ActiveSkillLV) : 125 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wHITsuu = Math.round(n_A_ActiveSkillLV / 2) * (Math.floor(n_A_ActiveSkillLV / 2) + 2), wCast = 15, n_Delay[2] = 1 * Math.floor(n_A_ActiveSkillLV / 2) + 2) : 126 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wHITsuu = n_A_ActiveSkillLV + 2, wCast = 2 + .5 * n_A_ActiveSkillLV) : 127 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wHITsuu = 4, wCast = 15.5 - .5 * n_A_ActiveSkillLV, n_Delay[2] = 5, n_Delay[6] = 4, wbairitu = .8 + .2 * n_A_ActiveSkillLV) : 128 == n_A_ActiveSkill || 320 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, n_A_ActiveSkillLV >= 4 ? wHITsuu = 25 : n_A_ActiveSkillLV >= 2 && (wHITsuu = 9), SG_Special_HITnum = wHITsuu, wCast = n_A_ActiveSkillLV, wbairitu = 1 + .3 * n_A_ActiveSkillLV, n_Delay[3] = .1 * wHITsuu) : 130 == n_A_ActiveSkill ? (wbairitu = .66 + .066 * n_A_ActiveSkillLV, n_A_Weapon_zokusei = 1, wCast = 6 - .5 * Math.floor((n_A_ActiveSkillLV - 1) / 2), n_Delay[2] = 1) : 131 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, wHITsuu = eval(document.calcForm.SkillSubNum.value), SG_Special_HITnum = wHITsuu, SRV ? (10 == n_A_Weapon_ATKplus && EquipNumSearch(1169) ? SGcast = .92 : SGcast = 1, wCast = (5 + n_A_ActiveSkillLV) * SGcast) : wCast = 5 + n_A_ActiveSkillLV, n_Delay[2] = 5, n_Delay[6] = 4.5, wbairitu = 1 + .4 * n_A_ActiveSkillLV) : 132 == n_A_ActiveSkill || 133 == n_A_ActiveSkill || 319 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 2, wHITsuu = n_A_ActiveSkillLV, 132 == n_A_ActiveSkill ? (wCast = .7 * n_A_ActiveSkillLV, n_Delay[2] = .8 + .2 * n_A_ActiveSkillLV) : (wCast = n_A_ActiveSkillLV, n_Delay[2] = 1)) : 277 == n_A_ActiveSkill ? (wHITsuu = n_A_ActiveSkillLV, n_A_Weapon_zokusei = 8, wCast = 1, n_Delay[2] = 1, wbairitu = .7 + .1 * n_A_ActiveSkillLV) : 37 == n_A_ActiveSkill || 387 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 6, wCast = 2, wbairitu = 1.25, 387 == n_A_ActiveSkill && (wbairitu *= 5)) : 104 == n_A_ActiveSkill ? (n_Delay[0] = 1, n_A_Weapon_zokusei = 6, wHITsuu = n_A_ActiveSkillLV, wCast = 15, n_Delay[2] = 4, 6 != n_B[2] && n_B[3] < 90 && (n_A_MATK[2] = 0, n_A_MATK[0] = 0, n_A_MATK[1] = 0)) : 312 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 7, wHITsuu = Math.round(n_A_ActiveSkillLV / 2), wCast = .5, n_A_ActiveSkillLV % 2 == 0 ? n_Delay[2] = .8 + n_A_ActiveSkillLV / 2 * .2 : n_Delay[2] = 1 + (n_A_ActiveSkillLV + 1) / 2 * .2) : 373 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = eval(document.calcForm.A_Weapon_zokusei.value), wCast = .1, n_Delay[2] = .5, 0 == n_B[4] ? wbairitu = .1 * n_A_ActiveSkillLV : wbairitu = .01, 1 == Taijin && (wbairitu = 0)) : 374 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = eval(document.calcForm.A_Weapon_zokusei.value), wCast = .1, n_Delay[2] = .5, wbairitu = .05 * n_A_ActiveSkillLV, 1 == Taijin && (wbairitu = 0)) : 375 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = eval(document.calcForm.A_Weapon_zokusei.value), n_Delay[0] = 1, wHITsuu = n_A_ActiveSkillLV, wCast = 2, n_Delay[2] = .5, wbairitu = .4 + n_A_BaseLV / 100, 1 == Taijin && (wbairitu = 0)) : 407 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wbairitu = .9, wHITsuu = n_A_ActiveSkillLV, wCast = .7 * n_A_ActiveSkillLV) : 408 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 3, wbairitu = .5, wHITsuu = Math.round(n_A_ActiveSkillLV / 2) + 4, wCast = 6.5 - .5 * n_A_ActiveSkillLV, n_Delay[2] = 1, n_Delay[0] = 1) : 409 == n_A_ActiveSkill ? (n_bunkatuHIT = 1, n_A_Weapon_zokusei = 3, wbairitu = 1.5 + 1.5 * n_A_ActiveSkillLV, wHITsuu = 3, wCast = 3, n_Delay[2] = 3) : 410 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, wbairitu = 1, wHITsuu = n_A_ActiveSkillLV + 2, wCast = .7 * n_A_ActiveSkillLV) : 412 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 1, wbairitu = 1 + .5 * n_A_ActiveSkillLV, wHITsuu = 1, wCast = 3, n_Delay[2] = 3) : 413 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wbairitu = 1, wHITsuu = Math.floor(n_A_ActiveSkillLV / 2) + 1, wCast = Math.floor(n_A_ActiveSkillLV / 2) + 1, n_Delay[2] = 1) : 414 == n_A_ActiveSkill ? (n_A_Weapon_zokusei = 4, wbairitu = 1.6 + .4 * n_A_ActiveSkillLV, wHITsuu = 1, wCast = 4) : 415 == n_A_ActiveSkill && (n_A_Weapon_zokusei = 4, wbairitu = 1 + 1 * n_A_ActiveSkillLV, wHITsuu = 1, wCast = 4), wCast *= n_A_CAST, 0 == n_bunkatuHIT)
                for (var b = 0; 2 >= b; b++) w_DMG[b] = BattleMagicCalc(n_A_MATK[b] * wbairitu), 0 != SG_Special_HITnum && (SG_Special_DMG[b] = w_DMG[b]), Last_DMG_B[b] = w_DMG[b], Last_DMG_A[b] = w_DMG[b] * wHITsuu, InnStr[b] += Last_DMG_A[b] + " (" + Last_DMG_B[b] + SubName[8] + wHITsuu + "hit)", w_DMG[b] = Last_DMG_A[b];
            else
                for (var b = 0; 2 >= b; b++) w_DMG[b] = Math.floor(BattleMagicCalc(n_A_MATK[b] * wbairitu) / wHITsuu), Last_DMG_A[b] = Last_DMG_B[b] = w_DMG[b] * wHITsuu, InnStr[b] += Last_DMG_A[b] + " (" + w_DMG[b] + SubName[8] + wHITsuu + "hit)", w_DMG[b] *= wHITsuu;
            w_HIT_HYOUJI = 100, CastAndDelay(), BattleCalc998()
        }
    }
}

function ATKbai01() {
    var _ = 100;
    193 != n_A_ActiveSkill && 197 != n_A_ActiveSkill && 321 != n_A_ActiveSkill && (SkillSearch(12) ? _ += 32 : n_A_Buf6[5] ? _ += 2 + 3 * n_A_Buf6[5] : n_A_Buf7[31] && (_ += 5), SkillSearch(256) && (_ += 5 * SkillSearch(256)), SkillSearch(270) && (_ += 2 * SkillSearch(270)), n_A_Buf2[19] && (_ += 100), n_A_Buf6[2] && (_ += 10), StPlusCalc2(87) && (_ += StPlusCalc2(87)), n_A_Buf6[22] && (_ -= 25));
    for (var n = 0; 2 >= n; n++) n_A_CriATK[n] = n_A_CriATK[n] * _ / 100, n_A_DMG[n] = n_A_DMG[n] * _ / 100
}

function ATKbai02(_, n) {
    wA02 = 100 * _, SkillSearch(327) ? wA02 += 20 * SkillSearch(327) : (SkillSearch(154) && (wA02 += 5 * SkillSearch(154)), 0 == SkillSearch(154) && n_A_Buf2[8] && (wA02 += 5 * n_A_Buf2[8])), SkillSearch(342) && (SRV && SkillSearch(380) <= 1 ? wA02 += 0 : wA02 += 2 * SkillSearch(342) * SkillSearch(380)), 0 == n ? (n_A_DMG[2] = Math.floor(n_A_DMG[2] * wA02 / 100), n_A_DMG[0] = Math.floor(n_A_DMG[0] * wA02 / 100), n_A_DMG[1] = Math.floor(n_A_DMG[1] * wA02 / 100)) : (n_A_CriATK[1] = Math.floor(n_A_CriATK[1] * wA02 / 100), n_A_CriATK[0] = Math.floor(n_A_CriATK[0] * wA02 / 100), n_A_CriATK[2] = Math.floor(n_A_CriATK[2] * wA02 / 100))
}

function BattleTAKA() {
    return 10 == n_A_WeaponType && SkillSearch(118) && 272 != n_A_ActiveSkill ? (wBTw1 = Math.floor((n_A_JobLV - 1) / 10 + 1), wBTw1 > 5 && (wBTw1 = 5), wBTw2 = SkillSearch(118), wBTw2 < wBTw1 && (wBTw1 = wBTw2), wBT = 80 + 2 * Math.floor(n_A_DEX / 10) + 2 * Math.floor(n_A_INT / 2) + 6 * SkillSearch(119), wBT = Math.floor(wBT * zokusei[n_B[3]][0]), wBT = tPlusDamCut(wBT), wBTw3 = Math.round(100 * (1 + .3 * n_A_LUK)) / 100, 44 == n_B[0] && (wBT = 0), str_bSUBname += "Falcon Damage<BR>", n_TAKA_DMG = wBT * wBTw1, str_bSUB += n_TAKA_DMG + " (" + wBT + " x " + wBTw1 + "Hit)", str_bSUB += "(" + wBTw3 + "% Chance)<BR>", wBT = n_TAKA_DMG * wBTw3 / 100, wBT = wBT * (w_HIT + (100 - w_HIT) * w_Cri / 100) / 100, wBTw1 = 0, Math.round(100 * wBT) / 100) : (n_TAKA_DMG = 0, 0)
}

function HealCalc(_, n) {
    wHeal = Math.floor((n_A_BaseLV + n_A_INT) / 8) * (8 * _ + 4);
    var e = 100 + 2 * SkillSearch(269);
    wHeal = Math.floor(wHeal * e / 100);
    var i = 100 + n_tok[91];
    return 1 == n && (i += n_tok[92]), EquipNumSearch(644) && (i += Math.floor(1.5 * n_A_Weapon_ATKplus)), EquipNumSearch(565) && n_A_HEAD_DEF_PLUS >= 7 && (i += 1 * EquipNumSearch(565)), EquipNumSearch(1277) && n_A_HEAD_DEF_PLUS >= 7 && (i += 3 * EquipNumSearch(1277)), EquipNumSearch(1813) && 3==n_A_JobClass() && (i += 5 * EquipNumSearch(1813)), wHeal = Math.floor(wHeal * (i / 100) * (1 - 20 * n_A_Buf6[18] / 100)), wHeal
}

function BattleCalc998() {
    if (n_PerHIT_DMG > 0 && w_HIT_HYOUJI < 100 && (str_bSUBname += "Damage When Missing", 0 == str_PerHIT_DMG ? str_bSUB += n_PerHIT_DMG : str_bSUB += str_PerHIT_DMG), 0 == n_A_ActiveSkill && (w_HIT_HYOUJI -= n_B_manual[38] * w_HIT_HYOUJI / 100), myInnerHtml("bSUBname", str_bSUBname, 0), myInnerHtml("bSUB", str_bSUB, 0), myInnerHtml("BattleHIT", w_HIT_HYOUJI, 0), 44 == n_B[0] && 0 != n_A_ActiveSkill && 325 != n_A_ActiveSkill)
        for (i = 0; 2 >= i; i++) w_DMG[i] = 0, myInnerHtml("ATK_0" + i, 0, 0);
    tPlusAG();
    var _;
    if (_ = Math.floor(n_B[6] / w_DMG[2]), n_B[6] % Math.floor(w_DMG[2]) != 0 && (_ += 1), 1e4 > _ ? myInnerHtml("MinATKnum", _, 0) : myInnerHtml("MinATKnum", SubName[5], 0), 0 != SG_Special_HITnum) {
        if (1 == _) {
            var n, e;
            if (n = SG_Special_HITnum, e = (SG_Special_DMG[2] * wHITsuu - n_B[6]) / (SG_Special_DMG[2] * wHITsuu - SG_Special_DMG[0] * wHITsuu), e > 1 && (e = 1), 0 > e && (e = 0), 2 == n && (e = .5 > e ? 2 * e * e : 1 - 2 * (1 - e) * (1 - e)), 3 == n && (1 / 3 > e ? e = 4.5 * Math.pow(e, 3) : e >= 1 / 3 && 2 / 3 > e ? e = 4.5 * (Math.pow(e, 3) - 3 * Math.pow(e - 1 / 3, 3)) : e >= 2 / 3 && (e = 1 - 4.5 * Math.pow(1 - e, 3))), n >= 4) {
                var l = Math.sqrt(Math.pow(SG_Special_DMG[2] - SG_Special_DMG[0], 2) / 12 * n);
                e = (SG_Special_DMG[1] * wHITsuu - n_B[6]) / l, e = e >= 0 ? .5 + .5 * Math.sqrt(1 - Math.exp(-2 * Math.pow(e, 2) / Math.PI)) : .5 - .5 * Math.sqrt(1 - Math.exp(-2 * Math.pow(e, 2) / Math.PI))
            }
            e = Math.floor(1e4 * e) / 100, myInnerHtml("MinATKnum", "1 (" + e + "% Chance)", 0)
        }
        SG_Special_HITnum = 0
    }
    if (w_HIT_HYOUJI < 100 && 0 == n_PerHIT_DMG) myInnerHtml("MaxATKnum", "Infinite (no 100% Hit)", 0);
    else {
        var t = w_DMG[0];
        w_HIT_HYOUJI < 100 && (t = n_PerHIT_DMG), _ = Math.floor(n_B[6] / t), n_B[6] % Math.floor(t) != 0 && (_ += 1), 1e4 > _ ? myInnerHtml("MaxATKnum", _, 0) : myInnerHtml("MaxATKnum", SubName[5], 0)
    }
    if (_ = Math.floor(n_B[6] / w_DMG[1]), n_B[6] % w_DMG[1] != 0 && (_ += 1), 0 == Taijin ? (myInnerHtml("nm063", "Base Exp Per Hit", 0), myInnerHtml("nm064", "Job Exp Per Hit", 0), 1e4 > _ ? (myInnerHtml("AtkBaseExp", Math.round(n_B[16] / _) + "Exp", 0), myInnerHtml("AtkJobExp", Math.round(n_B[17] / _) + "Exp", 0)) : (myInnerHtml("AtkBaseExp", SubName[7], 0), myInnerHtml("AtkJobExp", SubName[7], 0))) : (myInnerHtml("nm063", "", 0), myInnerHtml("AtkBaseExp", "", 0), myInnerHtml("nm064", "", 0), myInnerHtml("AtkJobExp", "", 0)), 1e4 > _) {
        myInnerHtml("AveATKnum", _, 0), n_AveATKnum = _;
        var a = (wCast + wDelay) * n_AveATKnum;
        a = Math.floor(100 * a) / 100, n_Delay[0] ? myInnerHtml("BattleTime", "Special", 0) : myInnerHtml("BattleTime", a + "s", 0)
    } else myInnerHtml("AveATKnum", SubName[5], 0), myInnerHtml("BattleTime", SubName[6], 0);
    var _ = 1 / (wCast + wDelay) * w_DMG[1];
    _ *= 100, _ = Math.round(_), _ /= 100, n_Delay[0] ? myInnerHtml("AveSecondATK", "Special", 0) : myInnerHtml("AveSecondATK", _, 0), _ = BattleHiDam(), _ = Math.round(_ * (100 - n_A_LUCKY)) / 100, _ = Math.round(_ * (100 - w_FLEE)) / 100, n_A_Buf2[13] && (_ = Math.round(_ * w_AG[n_A_Buf2[13]]) / 100), 3 == n_A_WeaponType && SkillSearch(255) && (_ = Math.round(_ * (80 - 3 * SkillSearch(255))) / 100), SkillSearch(287) && (_ = Math.round(_ * (100 - 7.5 * SkillSearch(287))) / 100), myInnerHtml("B_Ave2Atk", Math.round(100 * _ * BskillHitNum) / 100, 0)
}

function BattleHiDam() {
    var atkmod = 100,
        matkmod = 100,
        n_B_MATK = [0, 0, 0],
        w = Math.floor(n_B[9] / 7);
    n_B_MATK[0] = n_B[9] + n_B_manual[42] + w * w, n_B_MATK[0] += n_B_manual[43] * n_B_MATK[0] / 100, w = Math.floor(n_B[9] / 5), n_B_MATK[2] = n_B[9] + n_B_manual[42] + w * w, n_B_MATK[2] += n_B_manual[43] * n_B_MATK[2] / 100, n_B_MATK[1] = (n_B_MATK[2] + n_B_MATK[0]) / 2;
    var n_B_Weapon_zokusei = 0;
    n_B_rangedAtk = 0, n_B_rangedMAtk = 0;
    var n_B_ignoreDef = 0,
        n_B_ignoreMDef = 0;
    n_B_AtkSkill = document.calcForm.B_AtkSkill.value;
    var n_B_AtkSkillLV = 0;
    if (n_B_AtkSkillLV = MonsterOBJ[n_B[0]][2 * document.calcForm.B_AtkSkill.selectedIndex + 22], BskillHitNum = 1, 444 == n_B_AtkSkill || 445 == n_B_AtkSkill)
        if (BskillHitNum = 3, n_B_rangedAtk = n_B_AtkSkill - 444, n_B_ignoreDef = 1, n_B_ignoreFlee = 1, CardNumSearch(126)) atkmod = 0;
        else switch (n_B_AtkSkillLV) {
                case 1:
                    atkmod += 200;
                    break;
                case 2:
                    atkmod += 400;
                    break;
                case 3:
                    atkmod += 500;
                    break;
                case 4:
                    atkmod += 700;
                    break;
                case 5:
                    atkmod += 900;
                    break;
                case 6:
                    atkmod += 1100;
                    break;
                case 7:
                    atkmod += 1200;
                    break;
                case 8:
                    atkmod += 1400;
                    break;
                case 9:
                    atkmod += 1500;
                    break;
                case 10:
                    atkmod += 1700
            } else if (446 == n_B_AtkSkill || 447 == n_B_AtkSkill) atkmod += 100 * n_B_AtkSkillLV, n_B_rangedAtk = n_B_AtkSkill - 446;
            else if (448 == n_B_AtkSkill || 449 == n_B_AtkSkill) atkmod += 100 * n_B_AtkSkillLV - 100, n_B_rangedAtk = n_B_AtkSkill - 448;
    else if (n_B_AtkSkill >= 450 && 459 >= n_B_AtkSkill) n_B_Weapon_zokusei = n_B_AtkSkill - 450, atkmod += 100 * n_B_AtkSkillLV - 100;
    else if (463 == n_B_AtkSkill) n_B_Weapon_zokusei = 7;
    else if (464 == n_B_AtkSkill) n_B_Weapon_zokusei = 7;
    else if (470 == n_B_AtkSkill) BskillHitNum = 1 + n_B_AtkSkillLV, atkmod += 100 * n_B_AtkSkillLV - 100;
    else if (471 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 7, BskillHitNum = Math.round(n_B_AtkSkillLV / 2);
    else if (472 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 7, BskillHitNum = 2 + n_B_AtkSkillLV;
    else if (473 == n_B_AtkSkill) n_B_Weapon_zokusei = 7, atkmod += 35 * n_B_AtkSkillLV;
    else if (474 == n_B_AtkSkill) n_B_Weapon_zokusei = 7, atkmod += 40 * n_B_AtkSkillLV;
    else if (475 == n_B_AtkSkill) atkmod += 0, n_B_ignoreFlee = 1;
    else if (476 == n_B_AtkSkill) {
        n_B_ignoreFlee = 1;
        var w = Math.floor(n_B[9] / 7);
        n_B_MATK[0] = n_B[9] + w * w, w = Math.floor(n_B[9] / 5), n_B_MATK[2] = n_B[9] + w * w, n_B_MATK[1] = (n_B_MATK[2] + n_B_MATK[0]) / 2, atkmod = n_B[9]
    } else if (477 == n_B_AtkSkill) n_B_HIT += 20;
    else if (480 == n_B_AtkSkill) n_B_Weapon_zokusei = 7;
    else if (481 == n_B_AtkSkill) n_B_Weapon_zokusei = 7, n_B_ignoreFlee = 1;
    else if (482 == n_B_AtkSkill) 6 > n_B_AtkSkillLV ? atkmod = 100 * n_B_AtkSkillLV - 100 : atkmod += 100 * (n_B_AtkSkillLV - 5) - 100;
    else if (483 == n_B_AtkSkill) n_B_ignoreDef = 1;
    else if (484 == n_B_AtkSkill) n_B_Weapon_zokusei = document.calcForm.BSkillSubNum.value, atkmod += 100 * n_B_AtkSkillLV - 100;
    else if (485 == n_B_AtkSkill) n_B_rangedAtk = 1;
    else if (487 == n_B_AtkSkill) n_B_ignoreDef = 1, n_B[12] = n_B[13];
    else if (n_B_AtkSkill >= 490 && 499 >= n_B_AtkSkill) n_B_rangedAtk = 0, n_B_Weapon_zokusei = n_B_AtkSkill - 490, CardNumSearch(n_B_AtkSkill - 362) && 490 != n_B_AtkSkill && 497 >= n_B_AtkSkill ? atkmod = 0 : atkmod += 100 * n_B_AtkSkillLV - 100;
    else if (n_B_AtkSkill >= 500 && 509 >= n_B_AtkSkill) n_B_rangedAtk = 1, n_B_Weapon_zokusei = n_B_AtkSkill - 500, CardNumSearch(n_B_AtkSkill - 372) && 500 != n_B_AtkSkill && 507 >= n_B_AtkSkill ? atkmod = 0 : atkmod += 100 * n_B_AtkSkillLV - 100;
    else if (6 == n_B_AtkSkill) atkmod += 30 * n_B_AtkSkillLV;
    else if (7 == n_B_AtkSkill) atkmod += 20 * n_B_AtkSkillLV, n_B_Weapon_zokusei = 3;
    else if (19 == n_B_AtkSkill) atkmod += 30, n_B_Weapon_zokusei = 2;
    else if (41 == n_B_AtkSkill) n_B_rangedAtk = 1, atkmod += 5 * n_B_AtkSkillLV - 25;
    else if (44 == n_B_AtkSkill) n_B_rangedAtk = 1, atkmod += 50;
    else if (46 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 8, atkmod += 10 * n_B_AtkSkillLV - 30;
    else if (47 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 8, BskillHitNum = Math.round(n_B_AtkSkillLV / 2), (91 == n_B[3] || 92 == n_B[3] || 93 == n_B[3] || 94 == n_B[3]) && (atkmod += 5 * n_B_AtkSkillLV);
    else if (51 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, BskillHitNum = n_B_AtkSkillLV;
    else if (52 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, atkmod += 10 * n_B_AtkSkillLV - 30;
    else if (53 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, BskillHitNum = 4 + n_B_AtkSkillLV, atkmod -= 50;
    else if (54 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 1, BskillHitNum = n_B_AtkSkillLV;
    else if (55 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 1, atkmod += 10 * n_B_AtkSkillLV;
    else if (56 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 4, BskillHitNum = n_B_AtkSkillLV;
    else if (57 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 4, BskillHitNum = n_B_AtkSkillLV, atkmod -= 20;
    else if (65 == n_B_AtkSkill) atkmod += 50 * n_B_AtkSkillLV;
    else if (66 == n_B_AtkSkill) n_B_Weapon_zokusei = 3, atkmod += 30 * n_B_AtkSkillLV;
    else if (71 == n_B_AtkSkill) atkmod += 20 * n_B_AtkSkillLV, n_B_rangedAtk = 1;
    else if (84 == n_B_AtkSkill) n_B_AtkSkillLV >= 3 && (n_B_rangedAtk = 1), atkmod += 20 * n_B_AtkSkillLV;
    else if (158 == n_B_AtkSkill) atkmod += 20 * n_B_AtkSkillLV;
    else if (161 == n_B_AtkSkill) atkmod += 35 * n_B_AtkSkillLV, n_B_Weapon_zokusei = 6;
    else if (171 == n_B_AtkSkill) atkmod += 40 * n_B_AtkSkillLV;
    else if (72 == n_B_AtkSkill) atkmod += 50 * n_B_AtkSkillLV, n_B_rangedAtk = 1;
    else if (73 == n_B_AtkSkill) w = 20 * n_B_AtkSkillLV + 100, atkmod += 10 == n_B_AtkSkillLV ? 462.5 : n_B_AtkSkillLV >= 7 ? w + w / 2 + w / 4 - 100 : n_B_AtkSkillLV >= 4 ? w + w / 2 - 100 : w - 100;
    else if (83 == n_B_AtkSkill || 388 == n_B_AtkSkill) atkmod += 50 * n_B_AtkSkillLV + 200, 388 == n_B_AtkSkill && 0 == Taijin && (atkmod *= 2), 388 == n_B_AtkSkill && 1 == Taijin && (atkmod *= n_Ses ? 1.25 : 2);
    else if (111 == n_B_AtkSkill) n_B_Weapon_zokusei = 1;
    else if (122 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, BskillHitNum = 2 + n_B_AtkSkillLV, atkmod -= 30;
    else if (124 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, atkmod += 20 * n_B_AtkSkillLV;
    else if (125 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 3, BskillHitNum = document.calcForm.BSkillSubNum.value * Math.round(n_B_AtkSkillLV / 2);
    else if (126 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 4, BskillHitNum = 2 + n_B_AtkSkillLV;
    else if (127 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 4, BskillHitNum = 4, atkmod += 20 * n_B_AtkSkillLV - 20;
    else if (128 == n_B_AtkSkill || 320 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 1, BskillHitNum = n_B_AtkSkillLV >= 4 ? 25 : n_B_AtkSkillLV >= 2 ? 9 : 1, atkmod += 30 * n_B_AtkSkillLV;
    else if (130 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 1, atkmod += 7 * n_B_AtkSkillLV - 34;
    else if (131 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 1, BskillHitNum = document.calcForm.BSkillSubNum.value, atkmod += 40 * n_B_AtkSkillLV;
    else if (132 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 2, BskillHitNum = n_B_AtkSkillLV;
    else if (133 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 2, BskillHitNum = n_B_AtkSkillLV;
    else if (169 == n_B_AtkSkill) atkmod += 40 * n_B_AtkSkillLV + 200;
    else if (176 == n_B_AtkSkill) atkmod += 30 * n_B_AtkSkillLV;
    else if (188 == n_B_AtkSkill) BskillHitNum = 4, atkmod += 50 * n_B_AtkSkillLV + 50;
    else if (189 == n_B_AtkSkill) atkmod += 60 * n_B_AtkSkillLV + 140;
    else if (199 == n_B_AtkSkill || 207 == n_B_AtkSkill) atkmod += 40 * n_B_AtkSkillLV - 40, n_B_rangedAtk = 1;
    else if (248 == n_B_AtkSkill) n_B_Weapon_zokusei = 3, atkmod += .2 * n_B_AtkSkillLV;
    else if (260 == n_B_AtkSkill) n_B_rangedAtk = 1, atkmod += 40 * n_B_AtkSkillLV;
    else if (261 == n_B_AtkSkill) n_B_rangedAtk = 1, atkmod += 10 * n_B_AtkSkillLV - 50;
    else if (264 == n_B_AtkSkill) atkmod += 40 * n_B_AtkSkillLV - 60;
    else if (277 == n_B_AtkSkill) n_B_rangedMAtk = 1, n_B_Weapon_zokusei = 8, BskillHitNum = n_B_AtkSkillLV;
    else if (288 == n_B_AtkSkill) atkmod += 100 * (1 + n_B_AtkSkillLV);
    else if (289 == n_B_AtkSkill) atkmod += 100 * n_B_AtkSkillLV - 60;
    else if (290 == n_B_AtkSkill) atkmod += 100 * (3 + n_B_AtkSkillLV);
    else if (292 == n_B_AtkSkill) BskillHitNum = 9, atkmod += 100 * (1 + n_B_AtkSkillLV), n_B_rangedAtk = 1;
    else if (302 == n_B_AtkSkill) n_B_rangedAtk = 1, n_B_Weapon_zokusei = 4;
    else if (303 == n_B_AtkSkill) atkmod += 100 * (n_B_AtkSkillLV - 1);
    else if (305 == n_B_AtkSkill) atkmod += 4 * n_A_BaseLV - 100;
    else if (306 == n_B_AtkSkill) n_B_rangedAtk = 1;
    else if (307 == n_B_AtkSkill) n_B_rangedAtk = 1, atkmod += 50;
    else if (308 == n_B_AtkSkill) {
        var w;
        w = eval(document.calcForm.SkillSubNum.value), atkmod += 100 * w
    } else 326 == n_B_AtkSkill ? atkmod += Math.floor(eval(document.calcForm.SkillSubNum.value) / (16 - n_B_AtkSkillLV) / 100 * 100) : 382 == n_B_AtkSkill ? atkmod += 200 : atkmod = 100;
    if (atkmod = 0 == SRV && 0 == n_B_AtkSkill && 138 == n_A_card[11] ? atkmod : Math.floor(atkmod * zokusei[10 * n_A_BodyZokusei + 1][n_B_Weapon_zokusei]), w_HiDam = new Array, 0 == n_B_rangedMAtk ? (wBHD = n_B[13], w_HiDam[0] = atkmod / 100 * n_B[12], w_HiDam[1] = atkmod / 100 * (5 * n_B[12] + wBHD) / 6, w_HiDam[2] = atkmod / 100 * (4 * n_B[12] + 2 * wBHD) / 6, w_HiDam[3] = atkmod / 100 * (n_B[12] + wBHD) / 2, w_HiDam[4] = atkmod / 100 * (2 * n_B[12] + 4 * wBHD) / 6, w_HiDam[5] = atkmod / 100 * (n_B[12] + 5 * wBHD) / 6, w_HiDam[6] = atkmod / 100 * wBHD, n_B[12] == n_B[13] && (w_HiDam[6] = atkmod / 100 * wBHD - 1), 0 == n_B_ignoreDef && (w_HiDam[0] = w_HiDam[0] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2], w_HiDam[1] = w_HiDam[1] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2], w_HiDam[2] = w_HiDam[2] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[2], w_HiDam[3] = w_HiDam[3] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[1], w_HiDam[4] = w_HiDam[4] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0], w_HiDam[5] = w_HiDam[5] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0], w_HiDam[6] = w_HiDam[6] * (100 - n_A_totalDEF) / 100 - n_A_VITDEF[0])) : (wBHD = n_B_MATK[2], w_HiDam[0] = atkmod / 100 * n_B_MATK[0], w_HiDam[1] = atkmod / 100 * (5 * n_B_MATK[0] + wBHD) / 6, w_HiDam[2] = atkmod / 100 * (4 * n_B_MATK[0] + 2 * wBHD) / 6, w_HiDam[3] = atkmod / 100 * (n_B_MATK[0] + wBHD) / 2, w_HiDam[4] = atkmod / 100 * (2 * n_B_MATK[0] + 4 * wBHD) / 6, w_HiDam[5] = atkmod / 100 * (n_B_MATK[0] + 5 * wBHD) / 6, w_HiDam[6] = atkmod / 100 * wBHD, 0 == n_B_ignoreMDef && (w_HiDam[0] = w_HiDam[0] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[1] = w_HiDam[1] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[2] = w_HiDam[2] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[3] = w_HiDam[3] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[4] = w_HiDam[4] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[5] = w_HiDam[5] * (100 - n_A_MDEF) / 100 - n_A_softMDEF, w_HiDam[6] = w_HiDam[6] * (100 - n_A_MDEF) / 100 - n_A_softMDEF)), SkillSearch(23) && (n_B[3] >= 90 || 6 == n_B[2]))
        for (wBHD = Math.floor((3 + .04 * n_A_BaseLV) * SkillSearch(23)), i = 0; 6 >= i; i++) w_HiDam[i] -= wBHD;
    if (SkillSearch(355))
        for (wBHD = Math.floor((n_A_BaseLV + n_A_LUK + n_A_DEX) / 2), i = 0; 6 >= i; i++) w_HiDam[i] -= wBHD;
    if (wBHD = n_tok[60 + n_B_Weapon_zokusei], 0 != wBHD)
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if (SkillSearch(58))
        for (wBHD = 6 * SkillSearch(58), i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if (wBHD = n_tok[50 + n_B[2]], 0 != wBHD)
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if (wBHD = n_tok[190 + n_B[4]], 0 != wBHD)
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if (0 == n_B[19])
        for (wBHD = n_tok[79], i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if ((n_B[20] || n_B_rangedAtk || n_B_rangedMAtk || 2 == document.calcForm.B_AtkRange.value) && 1 != document.calcForm.B_AtkRange.value) {
        for (wBHD = n_tok[78], i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
        if (SkillSearch(421))
            for (wBHD = 20, i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100)
    }
    if ((n_B[20] || 2 == document.calcForm.B_AtkRange.value) && 1 != document.calcForm.B_AtkRange.value) {
        if (n_A_Buf2[15])
            for (wBHD = 5 + 15 * n_A_Buf2[15], i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
        if (n_A_Buf6[3])
            for (wBHD = 75, i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100)
    }
    if (1 == n_B[19])
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * n_tok[77] / 100);
    if (TimeItemNumSearch(9))
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] / 5);
    if (wBHD = n_tok[330 + Math.floor(n_B[3] / 10)], 0 != wBHD)
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    for (wBHD = StPlusCard(3e3 + n_B[0]), wBHD += StPlusCalc2(3e3 + n_B[0]), i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] * wBHD / 100);
    if (n_A_Buf7[22] && MANUKU_MONSTER())
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] / 10);
    if (n_A_Buf7[25] && SUPURE_MONSTER())
        for (i = 0; 6 >= i; i++) w_HiDam[i] -= Math.floor(w_HiDam[i] / 10);
    for (i = 0; 6 >= i; i++) w_HiDam[i] < 1 && (w_HiDam[i] = 1);
    if (n_A_Buf2[5])
        if (document.calcForm.A8_Skill14.value > 0 || n_A_Buf6[2])
            for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(2 * w_HiDam[i] / 3);
        else
            for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(w_HiDam[i] / 2);
    if (n_A_Buf6[16])
        for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(2 * w_HiDam[i]);
    if (n_A_Buf2[21])
        for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(w_HiDam[i] / 2);
    if (w_HiDam[0] = Math.floor(w_HiDam[0]), w_HiDam[6] = Math.floor(w_HiDam[6]), 444 == n_B_AtkSkill || 445 == n_B_AtkSkill) {
        var nPlayers = document.calcForm.BSkillSubNum.value;
        for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(w_HiDam[i] / nPlayers)
    }
    if (488 == n_B_AtkSkill)
        for (i = 0; 6 >= i; i++) w_HiDam[i] = Math.floor(n_B_AtkSkillLV * n_A_MaxHP / 10);
    if (489 == n_B_AtkSkill)
        for (n_B_Weapon_zokusei = 3, wBHD = n_tok[60 + n_B_Weapon_zokusei], i = 0; 6 >= i; i++) w_HiDam[i] = 1 * document.calcForm.BSkillSubNum.value - Math.floor(document.calcForm.BSkillSubNum.value * wBHD / 100);
    for (myInnerHtml("B_WeaponElement", ZokuseiOBJ[n_B_Weapon_zokusei] + " (" + 100 * zokusei[10 * n_A_BodyZokusei + 1][n_B_Weapon_zokusei] + "% vs " + ZokuseiOBJ[n_A_BodyZokusei] + "1)", 0), wBHD = 0, i = 0; 6 >= i; i++) wBHD += w_HiDam[i];
    wBHD = Math.round(wBHD / 7);
    var name67 = "Minimum Damage Received",
        name65 = "Average Damage Received",
        name68 = "Maximum Damage Received",
        wRefStr1 = "",
        wRefStr0 = "",
        wRefStr2 = "",
        asm = 1;
    if (n_A_Buf2[14]) {
        var wRSnum = (10 + 3 * n_A_Buf2[14]) * asm,
            wRef1 = new Array;
        wRef1[0] = Math.floor(wBHD * wRSnum / 100), 0 == wRef1[0] && (wRef1[0] = 1), wRef1[1] = Math.floor(w_HiDam[0] * wRSnum / 100), 0 == wRef1[1] && (wRef1[1] = 1), wRef1[2] = Math.floor(w_HiDam[6] * wRSnum / 100), 0 == wRef1[2] && (wRef1[2] = 1), wRefStr1 += "<BR><B style='color:blue'>" + wRef1[1] + "</B>", wRefStr0 += "<BR><B style='color:blue'>" + wRef1[0] + "</B>", wRefStr2 += "<BR><B style='color:blue'>" + wRef1[2] + "</B>", name67 += "<BR><B style='color:blue'>Min Dmg Reflected (Shield Reflect)</B>", name65 += "<BR><B style='color:blue'>Avg Dmg Reflected (Shield Reflect)</B>", name68 += "<BR><B style='color:blue'>Max Dmg Reflected (Shield Reflect)</B>"
    }
    if (n_tok[71]) {
        var wRef2 = new Array,
            w = n_tok[71] * asm;
        wRef2[0] = Math.floor(wBHD * w / 100), 0 == wRef2[0] && (wRef2[0] = 1), wRef2[1] = Math.floor(w_HiDam[0] * w / 100), 0 == wRef2[1] && (wRef2[1] = 1), wRef2[2] = Math.floor(w_HiDam[6] * w / 100), 0 == wRef2[2] && (wRef2[2] = 1), wRefStr1 += "<BR><B style='color:blue'>" + wRef2[1] + "</B>", wRefStr0 += "<BR><B style='color:blue'>" + wRef2[0] + "</B>", wRefStr2 += "<BR><B style='color:blue'>" + wRef2[2] + "</B>", name67 += "<BR><B style='color:blue'>Min Dmg Reflected (Equip/Cards)</B>", name65 += "<BR><B style='color:blue'>Avg Dmg Reflected (Equip/Cards)</B>", name68 += "<BR><B style='color:blue'>Max Dmg Reflected (Equip/Cards)</B>"
    }
    return myInnerHtml("nm067", name67, 0), myInnerHtml("nm065", name65, 0), myInnerHtml("nm068", name68, 0), BskillHitNum > 1 ? (myInnerHtml("B_MinAtk", w_HiDam[0] * BskillHitNum + " (" + w_HiDam[0] + " x " + BskillHitNum + ")" + wRefStr1, 0), myInnerHtml("B_AveAtk", wBHD * BskillHitNum + " (" + wBHD + " x " + BskillHitNum + ")" + wRefStr0, 0),
        myInnerHtml("B_MaxAtk", w_HiDam[6] * BskillHitNum + " (" + w_HiDam[6] + " x " + BskillHitNum + ")" + wRefStr2, 0)) : (myInnerHtml("B_MinAtk", w_HiDam[0] + wRefStr1, 0), myInnerHtml("B_AveAtk", wBHD + wRefStr0, 0), myInnerHtml("B_MaxAtk", w_HiDam[6] + wRefStr2, 0)), wBHD
}

function BattleMagicCalc(_) {
    wBMC_MDEF = n_B[15];
    var n = 0;
    0 == n_B[19] && CardNumSearch(424) && (n = 1), 0 != n && (wBMC_MDEF = 0, n_B_MDEF2 = 0), 122 == n_A_ActiveSkill ? wBMC2 = Math.floor(_ + 50) : wBMC2 = Math.floor(_ * (100 - wBMC_MDEF) / 100 - n_B_MDEF2), wBMC2 < 1 && (wBMC2 = 1), 104 == n_A_ActiveSkill && 6 != n_B[2] && n_B[3] < 90 && (wBMC2 = 0), wBMC2 = Math.floor(wBMC2 * zokusei[n_B[3]][n_A_Weapon_zokusei]), SRV ? n_B[3] > 89 && n_B[3] < 95 && 47 == n_A_ActiveSkill && (wBMC2 = Math.floor(wBMC2 * (1 + .05 * n_A_ActiveSkillLV))) : 90 <= n_B[3] && 47 == n_A_ActiveSkill && (wBMC2 = Math.floor(wBMC2 * (1 + .05 * n_A_ActiveSkillLV)));
    var e = n_tok[170 + n_B[2]];
    9 == n_B[2] && SkillSearch(234) && (e += 2 * SkillSearch(234)), wBMC2 = wBMC2 * (100 + e) / 100, wBMC2 = tPlusDamCut(wBMC2);
    var e = n_tok[650 + Math.floor(n_B[3]/10)];
    e && (wBMC2 = wBMC2 * (100 + e) / 100);
    var e = StPlusCalc2(5e3 + n_A_ActiveSkill) + StPlusCard(5e3 + n_A_ActiveSkill);
    return (46 == n_A_ActiveSkill || 47 == n_A_ActiveSkill || 277 == n_A_ActiveSkill) && 5 == n_A_JobClass() && (e += 20 * CardNumSearch(474)), (132 == n_A_ActiveSkill || 133 == n_A_ActiveSkill) && EquipNumSearch(1146) && (e += n_A_HEAD_DEF_PLUS), 131 == n_A_ActiveSkill && EquipNumSearch(1169) && (e += n_A_Weapon_ATKplus), (37 == n_A_ActiveSkill || 387 == n_A_ActiveSkill) && 3 == n_A_JobClass() && EquipNumSearch(1247) && (e += 5, n_A_HEAD_DEF_PLUS >= 7 && (e += 5)), wBMC2 = wBMC2 * (100 + e) / 100, n_A_Buf7[21] && MANUKU_MONSTER() && (wBMC2 = 110 * wBMC2 / 100), n_A_Buf7[24] && SUPURE_MONSTER() && (wBMC2 = 110 * wBMC2 / 100), 131 == n_A_ActiveSkill && n_B_IJYOU[4] && 0 == n_B[19] && n_B[3] < 90 && (wBMC2 = 0), wBMC2 = Math.floor(wBMC2), wBMC2
}

function ClickJob(n) {
    with(document.calcForm) {
        myInnerHtml("A_KakutyouSel", "", 0), myInnerHtml("A_KakutyouData", "", 0), A_Kakutyou.value = 0, n_A_JobSet(), n = n_A_JOB;
        var len = A_JobLV.length;
        for (i = 0; len > i; i++) A_JobLV.options[0] = null;
        var w = 0;
        for (w = 0 == n ? 10 : 19 >= n || n >= 41 && 43 >= n ? 50 : 20 == n ? 99 : 70, i = 1; w >= i; i++) A_JobLV.options[i - 1] = new Option(i, i);
        if (n_SkillSW && (45 == n_A_JOB ? (myInnerHtml("AS12_1", "<S># of Spirit Spheres</S>", 0), A2_Skill12.disabled = !0, A2_Skill12.value = 0, n_A_Buf2[12] = 0) : (myInnerHtml("AS12_1", "# of Spirit Spheres", 0), A2_Skill12.disabled = !1), 13 == n_A_JOB || 27 == n_A_JOB ? (myInnerHtml("AS11_1", "<S>Providence</S>", 0), A2_Skill11.disabled = !0, A2_Skill11.value = 0, n_A_Buf2[11] = 0) : (myInnerHtml("AS11_1", "Providence", 0), A2_Skill11.disabled = !1)), n_Skill7SW) {
            for (var i = 2; 3 >= i; i++) A_SpeedPOT.options[2] = null;
            3 != n_A_JOB && 9 != n_A_JobClass2() && 16 != n_A_JobClass2() ? A_SpeedPOT.options[2] = new Option(SpeedPotName[2], 2) : A_SpeedPOT.options[2] = new Option("-", 0), 1 == n_A_JobClass() || 6 == n_A_JobClass() || 41 == n_A_JobClass() || 14 == n_A_JobClass2() || 11 == n_A_JobClass2() || 5 == n_A_JOB || 45 == n_A_JOB ? A_SpeedPOT.options[3] = new Option(SpeedPotName[3] + " (Lv85)", 3) : 22 == n_A_JOB ? A_SpeedPOT.options[3] = new Option("* Special (" + SkillOBJ[304][2] + " Lv85) / Poison Bottle", 3) : A_SpeedPOT.options[3] = new Option("* Special (" + SkillOBJ[304][2] + ") (Lv85)", 3)
        }
        for (20 != n_A_JOB && (SuperNoviceFullWeaponCHECK = 0), SuperNoviceFullWeaponCHECK ? JobASPD[20][7] = 120 : JobASPD[20][7] = 0, 0 == SRV && 8 == n_A_JobClass2() && 1 == n_Nitou && (n_Nitou = 0, ClickWeaponType2(0)), i = 21; i >= 0; i--) A_WeaponType.options[i] = null;
        for (j = 0, i = 0; 21 >= i; i++) 0 != JobASPD[n][i] && (A_WeaponType.options[j] = new Option(WeaponName[i], i), j++);
        for (ClickWeaponType(0), i = 0; 14 >= i; i++) 392 == JobSkillPassOBJ[n][i] ? (myInnerHtml("P_Skill" + i, SkillOBJ[JobSkillPassOBJ[n][i]][2], 0), myInnerHtml("P_Skill" + i + "s", "<select name=A_skill" + i + " id=A_skill" + i + ' onChange=StAllCalc() style="width:70px;"></select>', 0), 0 == n_Tensei && myInnerHtml("P_Skill" + i, "", 0)) : 999 != JobSkillPassOBJ[n][i] ? (myInnerHtml("P_Skill" + i, SkillOBJ[JobSkillPassOBJ[n][i]][2], 0), myInnerHtml("P_Skill" + i + "s", "<select name=A_skill" + i + " id=A_skill" + i + " onChange=StAllCalc()></select>", 0)) : (myInnerHtml("P_Skill" + i, "", 0), myInnerHtml("P_Skill" + i + "s", "", 0));
        for (var j = 0; 14 >= j; j++) {
            var w = JobSkillPassOBJ[n][j],
                w2 = [12, 68, 152, 253, 258, 301, 309, 310, 322, 345, 364, 365, 383, 385, 386, 390, 420, 421, 422];
            if (NumSearch(w, w2)) {
                var wOBJ = document.getElementById("A_skill" + j);
                wOBJ.options[0] = new Option("off", 0), wOBJ.options[1] = new Option("on", 1)
            } else if (999 != w) {
                for (var wOBJ = document.getElementById("A_skill" + j), i = 10; i >= 0; i--) wOBJ.options[i] = null;
                for (var i = 0; i <= SkillOBJ[JobSkillPassOBJ[n][j]][1]; i++) wOBJ.options[i] = new Option(i, i);
                if (392 == SkillOBJ[JobSkillPassOBJ[n][j]][0]) {
                    for (n_ECname = ["off", "eAthena (max stats)", "iRO", "Original"], i = 0; 3 >= i; i++) wOBJ.options[i] = new Option(n_ECname[i], i);
                    0 == n_Tensei && (wOBJ.style.visibility = "hidden")
                }
            }
        }
        if (58 == JobSkillPassOBJ[n][0]) {
            for (i = 10; i >= 0; i--) A_skill0.options[i] = null;
            for (n_ECname = ["0", "6% Reduction", "12% Reduction", "18% Reduction", "24% Reduction", "30% Reduction"], i = 0; 5 >= i; i++) A_skill0.options[i] = new Option(n_ECname[i], i)
        }
        if (78 == JobSkillPassOBJ[n][5]) {
            for (i = 10; i >= 0; i--) A_skill5.options[i] = null;
            for (n_ECname = ["(No Peco)", "0", "1", "2", "3", "4", "5"], i = 0; 6 >= i; i++) A_skill5.options[i] = new Option(n_ECname[i], i)
        }
        if (78 == JobSkillPassOBJ[n][8]) {
            for (i = 10; i >= 0; i--) A_skill8.options[i] = null;
            for (n_ECname = ["(No Peco)", "0", "1", "2", "3", "4", "5"], i = 0; 6 >= i; i++) A_skill8.options[i] = new Option(n_ECname[i], i)
        }
        if (367 == JobSkillPassOBJ[n][11]) {
            for (i = 10; i >= 0; i--) A_skill11.options[i] = null;
            for (n_ECname = [0, 1, 2, 3, 4, 5, 6, 8, 10], i = 0; 8 >= i; i++) A_skill11.options[i] = new Option(10 * n_ECname[i] + "%", n_ECname[i])
        }
        for (var len = A_ActiveSkill.length, i = 0; len > i; i++) A_ActiveSkill.options[0] = null;
        for (i = 0; 999 != JobSkillActiveOBJ[n][i]; i++) A_ActiveSkill.options[i] = new Option(SkillOBJ[JobSkillActiveOBJ[n][i]][2], JobSkillActiveOBJ[n][i]);
        for (i = 0; 20 > i; i++) w_ASSP0bk[i] = 999;
        ActiveSkillSetPlus(), ClickActiveSkill(), WeaponSet2()
    }
}

function Bskill() {
    with(document.calcForm) {
        var BskillLen = 0;
        for (B_AtkSkill.options.length = 0, B_AtkSkill.options[0] = new Option("Basic Attack", 0), i = 23; 0 != MonsterOBJ[B_Enemy.value][i]; i += 2) BskillLen++, B_AtkSkill.options[BskillLen] = new Option(SkillOBJ[MonsterOBJ[B_Enemy.value][i]][2] + " Lv " + MonsterOBJ[B_Enemy.value][i + 1], MonsterOBJ[B_Enemy.value][i]);
        myInnerHtml("BBSkill", "", 0), 957 == n_A_Equip[7] && calc()
    }
}

function ClickWeaponType(n) {
    with(document.calcForm) {
        n_A_JobSet(), A_Arrow.disabled = !1;
        for (var i = 0; 23 > i; i++) A_Arrow.options[0] = null;
        if (10 == n || 14 == n || 15 == n)
            for (j = 23, i = 0; 4 >= i; i++) ArrowOBJ[i] = ArrowOBJbackup[i];
        else if (17 == n || 18 == n || 19 == n || 20 == n)
            for (j = 2, i = 0; 2 >= i; i++) ArrowOBJ[i] = BulletOBJ[i];
        else if (21 == n)
            for (j = 4, i = 0; 4 >= i; i++) ArrowOBJ[i] = GrenadeOBJ[i];
        else j = 1, ArrowOBJ[0] = [0, 0, "(No Arrow)"], A_Arrow.value = 0, A_Arrow.disabled = !0;
        for (i = 0; i <= j; i++) A_Arrow.options[i] = new Option(ArrowOBJ[i][2], i);
        WeaponSet(), 0 == n ? (A_Weapon_ATKplus.disabled = !0, A_Weapon_ATKplus.value = 0) : A_Weapon_ATKplus.disabled = !1, n_A_JobSet(), 8 != n_A_JOB && 22 != n_A_JOB || 11 == n ? (myInnerHtml("A_SobWeaponName", "", 0), myInnerHtml("spanA_weapon2", "", 0), myInnerHtml("spanA_weapon2seiren", "", 0), myInnerHtml("spanA_weapon2_CardShort", "", 0), myInnerHtml("nA_weapon2_c1", "", 0), myInnerHtml("nA_weapon2_c2", "", 0), myInnerHtml("nA_weapon2_c3", "", 0), myInnerHtml("nA_weapon2_c4", "", 0), n_Nitou = 0) : 0 == n_Nitou && myInnerHtml("A_SobWeaponName", 'Left Hand: <select name="A_Weapon2Type" onChange = "ClickWeaponType2(this[this.selectedIndex].value) | StAllCalc() | restrictCardslot(1)">	<option value="0">Fist or Shield<option value="1">Dagger<option value="2">Sword<option value="6">Axe</select>', 0), n_A_Equip[0] = eval(A_weapon1.value), ActiveSkillSetPlus(), ClickB_Item(n_A_Equip[0])
    }
}

function ClickWeaponType2(n) {
    with(document.calcForm) {
        if (n_A_JobSet(), 0 != n) {
            if (0 == n_Nitou) {
                for (myInnerHtml("spanA_weapon2", '<select name="A_weapon2" onChange="StAllCalc()|ClickB_Item(this[this.selectedIndex].value) | restrictCardslot(1)"></select>', 0), myInnerHtml("spanA_weapon2seiren", 'Refine (Left): <select name="A_Weapon2_ATKplus" onChange = "StAllCalc()"></select>', 0), i = 0; 10 >= i; i++) A_Weapon2_ATKplus.options[i] = new Option("+" + i, i);
                for (myInnerHtml("nA_weapon2_c1", '<select name="A_weapon2_card1" onChange="StAllCalc()|Card(this[this.selectedIndex].value)"></select>', 0), myInnerHtml("nA_weapon2_c2", '<select name="A_weapon2_card2" onChange="StAllCalc()|Card(this[this.selectedIndex].value)"></select>', 0), myInnerHtml("nA_weapon2_c3", '<select name="A_weapon2_card3" onChange="StAllCalc()|Card(this[this.selectedIndex].value)"></select>', 0), myInnerHtml("nA_weapon2_c4", '<select name="A_weapon2_card4" onChange="StAllCalc()|Card(this[this.selectedIndex].value)"></select>', 0), i = 0;
                    "NULL" != CardSortOBJ[0][i]; i++) A_weapon2_card1.options[i] = new Option(cardOBJ[CardSortOBJ[0][i]][2], cardOBJ[CardSortOBJ[0][i]][0]);
                for (i = 0;
                    "NULL" != CardSortOBJ[1][i]; i++) A_weapon2_card2.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]), A_weapon2_card3.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]), A_weapon2_card4.options[i] = new Option(cardOBJ[CardSortOBJ[1][i]][2], cardOBJ[CardSortOBJ[1][i]][0]);
                A_weapon2_card4.options[4] = new Option("Top10", 106)
            }
            for (myInnerHtml("spanA_weapon2_CardShort", '<select name="A_cardshortLeft" onChange="SetCardShortLeft()|StAllCalc()|ActiveSkillSetPlus()"></select>', 0), A_cardshortLeft.options[0] = new Option("Card Shortcuts", 0), i = 1; 60 >= i; i++) A_cardshortLeft.options[i] = new Option(CardShort[i][0], i);
            n_Nitou = 1, WeaponSetLeft()
        } else myInnerHtml("spanA_weapon2", "", 0), myInnerHtml("spanA_weapon2seiren", "", 0), myInnerHtml("spanA_weapon2_CardShort", "", 0), myInnerHtml("nA_weapon2_c1", "", 0), myInnerHtml("nA_weapon2_c2", "", 0), myInnerHtml("nA_weapon2_c3", "", 0), myInnerHtml("nA_weapon2_c4", "", 0), n_Nitou = 0;
        n_Nitou && (n_A_Equip[1] = eval(A_weapon2.value), ActiveSkillSetPlus(), ClickB_Item(n_A_Equip[1]))
    }
}

function ClickActiveSkill() {
    with(document.calcForm) {
        n_A_ActiveSkill = eval(A_ActiveSkill.value), n_A_ActiveSkill >= 3e3 ? (n_A_ActiveSkillLV = InsertSkill[n_A_ActiveSkill - 3e3][3], n_A_ActiveSkill = InsertSkill[n_A_ActiveSkill - 3e3][2]) : n_A_ActiveSkill >= 2e3 ? (n_A_ActiveSkillLV = AutoSpellSkill[n_A_ActiveSkill - 2e3][3], n_A_ActiveSkill = AutoSpellSkill[n_A_ActiveSkill - 2e3][2]) : (n_A_ActiveSkillLV = SkillOBJ[n_A_ActiveSkill][1], 0 == SRV && (14 != n_A_JobClass2() || 128 != n_A_ActiveSkill && 133 != n_A_ActiveSkill || (n_A_ActiveSkillLV = 10)));
        var len = A_ActiveSkillLV.length;
        for (i = 0; len > i; i++) A_ActiveSkillLV.options[0] = null;
        if (n_A_ActiveSkill >= 0)
            for (i = 1; i <= n_A_ActiveSkillLV; i++) A_ActiveSkillLV.options[i - 1] = new Option("Lv " + i, i);
        1 == SkillOBJ[n_A_ActiveSkill][1] ? A_ActiveSkillLV.style.visibility = "hidden" : (A_ActiveSkillLV.style.visibility = "visible", A_ActiveSkillLV.value = n_A_ActiveSkillLV), ClickActiveSkill2()
    }
}

function BClickAtkSkill() {
    with(document.calcForm) {
        if (n_B_AtkSkill = eval(B_AtkSkill.value), 444 == n_B_AtkSkill || 445 == n_B_AtkSkill)
            for (myInnerHtml("BBSkill", 'Players in Range: <select name="BSkillSubNum" onChange="calc()"></select>', 0), i = 1; 99 >= i; i++) BSkillSubNum.options[i - 1] = new Option(i, i);
        if (489 == n_B_AtkSkill) myInnerHtml("BBSkill", 'Enemy Remaining HP: <input type="text" name="BSkillSubNum" onkeyup="calc()" value="' + n_B[6] + '" size="7" class="center">', 0);
        else if (125 == n_B_AtkSkill) {
            myInnerHtml("BBSkill", 'Meteors landing on Player: <select name="BSkillSubNum" onChange="calc()"></select>', 0);
            var n_B_AtkSkillLV = 0;
            n_B_AtkSkillLV = MonsterOBJ[n_B[0]][2 * document.calcForm.B_AtkSkill.selectedIndex + 22];
            var MeteorNum = 2 + Math.floor(n_B_AtkSkillLV / 2);
            for (i = 2; MeteorNum >= i; i++) BSkillSubNum.options[i - 1] = new Option(i, i);
            BSkillSubNum.value = 2
        } else if (131 == n_B_AtkSkill) {
            for (myInnerHtml("BBSkill", 'Hits: <select name="BSkillSubNum" onChange="calc()"></select>', 0), i = 1; 15 >= i; i++) BSkillSubNum.options[i - 1] = new Option(i, i);
            BSkillSubNum.value = 3
        } else if (484 == n_B_AtkSkill) {
            for (myInnerHtml("BBSkill", 'Element: <select name="BSkillSubNum" onChange="calc()"></select>', 0), i = 0; 9 >= i; i++) BSkillSubNum.options[i] = new Option(ZokuseiOBJ2[i], i);
            BSkillSubNum.value = 0
        } else myInnerHtml("BBSkill", "", 0);
        calc(), 957 == n_A_Equip[7] && calc()
    }
}

function ClickActiveSkill2() {
    with(document.calcForm) if (66 == n_A_ActiveSkill || 326 == n_A_ActiveSkill) myInnerHtml("AASkill", 'Cart Weight: <input type="text" name="SkillSubNum" value="8000" size=2 onkeyup="calc()">', 0);
        else
    if (131 == n_A_ActiveSkill) {
        for (myInnerHtml("AASkill", 'Hits: <select name="SkillSubNum" onChange="calc()"></select>', 0), i = 1; 15 >= i; i++) SkillSubNum.options[i - 1] = new Option(i, i);
        SkillSubNum.value = 3
    } else if (88 == n_A_ActiveSkill) {
        for (myInnerHtml("AASkill", 'Poison React Lv: <select name="SkillSubNum" onChange="calc()"></select>', 0), i = 0; 10 >= i; i++) SkillSubNum.options[i] = new Option(i, i);
        SkillSubNum.value = 0, 14 == n_A_JobClass2() && (SkillSubNum.value = 0)
    } else if (197 == n_A_ActiveSkill) myInnerHtml("AASkill", 'Remaining SP: <input type="text" name="SkillSubNum" size=2 onkeyup="calc()">', 0), SkillSubNum.value = n_A_MaxSP - 1;
    else if (394 == n_A_ActiveSkill) {
        for (myInnerHtml("AASkill", '<select name="SkillSubNum" onChange="calc()"></select>', 0), i = 0; 4 >= i; i++) SkillSubNum.options[i] = new Option(SyurikenOBJ[i][2], i);
        SkillSubNum.value = 0
    } else if (395 == n_A_ActiveSkill) {
        for (myInnerHtml("AASkill", '<select name="SkillSubNum" onChange="calc()"></select>', 0), i = 0; 4 >= i; i++) SkillSubNum.options[i] = new Option(KunaiOBJ[i][2], i);
        SkillSubNum.value = 0
    } else if (405 == n_A_ActiveSkill) myInnerHtml("AASkill", 'Remaining HP: <input type="text" name="SkillSubNum" size=3 onkeyup="calc()">', 0), SkillSubNum.value = n_A_MaxHP - 1;
    else if (429 == n_A_ActiveSkill) {
        var DEATH = ["1", "1.2", "1.6", "2", "2.4", "3", "3.6", "4", "5", "6", "7", "8", "9", "10"];
        for (myInnerHtml("AASkill", 'Hits (Considering the Success Chance): <select name="SkillSubNum" onChange="calc()"></select>', 0), i = 0; 13 >= i; i++) SkillSubNum.options[i] = new Option(DEATH[i] + " Hit", i);
        SkillSubNum.value = 6
    } else if (308 == n_A_ActiveSkill) {
        myInnerHtml("AASkill", 'Enemy Distance: <select name="SkillSubNum" onChange="calc()"></select>', 0);
        var CHATK_NAME = ["0~3 Cells", "4~6 Cells", "7~9 Cells", "10~12 Cells", "13+ Cells"];
        for (i = 0; 4 >= i; i++) SkillSubNum.options[i] = new Option(CHATK_NAME[i], i);
        SkillSubNum.value = 4
    } else if (193 == n_A_ActiveSkill && SRV) {
        for (myInnerHtml("AASkill", 'Spheres Left: <select name="SkillSubNum" onChange="calc()"></select>', 0), i = 1; 5 >= i; i++) SkillSubNum.options[i - 1] = new Option(i, i);
        SkillSubNum.value = 5
    } else myInnerHtml("AASkill", "", 0)
}

function LoadFeatures() {
    if (1 != n_FeatSW) {
        n_FeatSW = 1;
        var _;
        _ = "• Added ''RO server'' at the top of the calc, allowing to simulate special server behaviour (Ghostring Card, ShieldBoom+Icepick, etc).", _ += "<br>• Added PvP: select enemy ''[Custom Player]'' and modify it's stats using ''Manual Edits on Enemy'' field.", _ += "<br>• Added ''Enemy Attack Skills'' to all monsters in ''Monster Combat Simulator''.", _ += '<br>• Added "Enemy Attack Range" in "Monster Combat Simulator", allowing to change attack range regardless of anything else.', _ += "<br>• Added color themes.", _ += '<br>• "Full Save" now saves everything in the calc.', _ += '<br>• "Save as URL" now saves almost everything in the calc.', _ += '<br>• Use the "Load URL from another calc" button to load your characters from other calcs.', _ += '<br>• Added "Additional Enchants & Manual Edits on Player" under "Item Data" in "CHARACTER SIMULATOR" (enables Mora, Feral, and other enchants).', _ += '<br>• Added "Headgear Hidden Slot Enchant" in "Equipment & Cards".', _ += '<br>• Added "+% MATK based Attack Modifiers" in "Other Info".', _ += '<br>• Added "Long-range" and "Critical damage" in "+% ATK based Attack Modifiers" in "Other Info".', _ += "<br>• Added ''Capture Pet Success Rate'' in ''Other Info''.", _ += "<br>• Added ''NPC Refinement Cost &amp; Success Rate'' in ''Other Info''.", _ += "<br>• Added ''NPC Socket Enchant Cost &amp; Success Rate'' in ''Other Info''.", _ += "<br>• Fixed elemental bows: when equipping Neutral element arrows and an elemental bow, damage will have bow's element instead of Neutral.", _ += "<br>• Visual improvements and fixes", _ += "<br>• Rearranged buffs/items/effects", _ += "<br>• Added buffs/items/effects/pets", _ += "<br>• Added a huge amount of equipments and cards.", _ += "<br>• Fixed lots of equipment and cards.", _ += "<br>• Added aRO's Revalorize System", _ += "<br>• Added aRO's custom versions of Renewal mob.", document.getElementById("features").innerHTML = _
    } else 1 == n_FeatSW && (n_FeatSW = 0, document.getElementById("features").innerHTML = "")
}

function LoadChangelog() {
    if (1 != n_LogSW) {
        n_LogSW = 1;
        var _;
        _ = "<b>2015-07-01:</b>", _ += "<br>• Changed ASPD formula.", _ += '<br>• Added equip restrictions options: "Equip Job Restrictions", "Equip Level Restrictions", "Equip Slot Restrictions", and "Equip Card Slot Restrictions".', _ += '<br>• Added "Attack Element" and "Enemy Attack Element" information in "Monster Combat Simulator".', _ += "<br>", _ += "<br><b>2015-06-02:</b>", _ += "<br>• Fixed Frozen, Stone Curse, Undead Attribute Change, and Holy Armor [B.S.S.]. Now they modify elemental damage taken from enemies.", _ += "<br>", _ += "<br><b>2015-06-01:</b>", _ += "<br>• Added attack skills to all monsters.", _ += "<br>", _ += "<br><b>2015-04-20:</b>", _ += "<br>• Added ''RO server'' at the top of the calc, allowing to simulate special server behaviour (Ghostring Card, ShieldBoom+Icepick, etc).", _ += "<br>• Fixed ''Shield Chain'' damage on private servers (damage divergence should be lower than 100 damage points per hit).", _ += "<br>• Added self-Provoke effect to ''Shield Boomerang'' damage on private servers.", _ += "<br>", _ += "<br><b>2015-04-19:</b>", _ += "<br>• Fixed Ruber'' (now Whitesmith and Creators will be able to equip it).", _ += "<br>• Deleted ''Roubel'' (it was exactly the same as ''Ruber'', but with another name).", _ += "<br>• Fixed ''Chrome Metal Sword'' (it was in ''Two-Handed Swords'' list, instead of ''Swords'' list).", _ += "<br>", _ += "<br><b>2015-04-18:</b>", _ += "<br>• Changed ''Alchemy Glove'' auto-cast.", _ += "<br>", _ += "<br><b>2015-04-13:</b>", _ += "<br>• Fixed Perfect Dodge (it was increasing ''Player's Dodge Ratio'' against enemy skills, instead of just basic attacks).", _ += "<br>• Fixed ''Load URL from another calc'' (it wasn't loading data from shown fields).", _ += "<br>• Fixed ''Load URL from another calc'' (it wasn't showing [Active] fields).", _ += "<br>• Fixed ''Full Load'' (it wasn't loading data from ''Additional Enchants & Manual Edits on Player'' field when it was being shown).", _ += "<br>• Fixed ''Full Load'' (it wasn't loading data from ''Manual Edits on Enemy'' field when it was being shown).", _ += "<br>• Fixed ''Full Load'' (calc crashed when loading twice data from ''Music and Dance Skills'' field when it was being shown).", _ += "<br>• Fixed ''Item Data'' field when loading data from ''Full Load'' (it was showing data from the incorrect item).", _ += "<br>• Performance improvement to ''Full Load''.", _ += "<br>", _ += "<br><b>2015-04-08:</b>", _ += "<br>• Added ''Capture Pet Success Rate'' in ''Other Info''.", _ += "<br>• Added ''NPC Refinement Cost &amp; Success Rate'' in ''Other Info''.", _ += "<br>• Added ''NPC Socket Enchant Cost &amp; Success Rate'' in ''Other Info''.", _ += '<br>• Added Stat+20 foods ("Cocktail Warg Blood", "Minor Stew", "Siroma Iced Tea", "Drosera Herb Salad", "Petite Tail Noodles") in "Food / Speed Potions / other Items".', _ += "<br>• Added ''Tyr's Blessing'' in ''Food / Speed Potions / other Items''.", _ += "<br>• Added automatic calculation when modifying player's skill level in ''Player Attack Skills'' (before it required to push ''Calculate'' button).", _ += "<br>• Added automatic calculation when typing a value in text fields from ''Additional Enchants & Manual Edits on Player'' and ''Manual Edits on Enemy''<BR> (before it required to select another element after typing).", _ += '<br>• Added "Enemy Attack Range" in "Monster Combat Simulator", allowing to ignore <i>enemy basic attack range</i> and <i>enemy skill attack range</i>.', _ += '<br>• Fixed enemy attack skill "Dark Breath".', _ += '<br>• Fixed enemy attack skill "Suicide Bombing / Self-Destruction".', _ += "<br>• Fixed a visual issue when changing enemies related to multiple hit enemy attack skills.", _ += '<br>• Fixed "Manual Edits on Enemy" in "Monster Combat Simulator" (except text with a line-through).', _ += "<br>", _ += "<br><b>2015-03-11:</b>", _ += "<br>• Added PvP: select enemy ''[Custom Player]'' and modify it's stats using ''Manual Edits on Enemy'' field.", _ += '<br>• Partially fixed "Manual Edits on Enemy" in "Monster Combat Simulator".', _ += "<br><br>", document.getElementById("changelog").innerHTML = _
    } else 1 == n_LogSW && (n_LogSW = 0, document.getElementById("changelog").innerHTML = "")
}

function theme() {
    var _ = document.getElementById("theme").value;
    document.body.style.backgroundColor = bBGC[_];
    for (var n = document.querySelectorAll("h1"), e = 0; e < n.length; e++) n[e].style.backgroundImage = "linear-gradient(to bottom, " + hBGC1[_] + ", " + hBGC2[_] + ")";
    for (var i = document.querySelectorAll("h3"), e = 0; e < i.length; e++) i[e].style.backgroundImage = "linear-gradient(to bottom, " + hBGC1[_] + ", " + hBGC2[_] + ")";
    for (var l = document.querySelectorAll(".links"), e = 0; e < l.length; e++) l[e].style.backgroundImage = "linear-gradient(to bottom, " + hBGC1[_] + ", " + hBGC2[_] + ")";
    document.querySelectorAll("select");
    addCSSRule(document.styleSheets[1], "select", "background-color:" + selBGC[_] + ";");
    for (var t = document.querySelectorAll(".subheader select"), e = 0; e < t.length; e++) t[e].style.backgroundColor = ssBGC[_];
    for (var a = document.querySelectorAll(".subheader"), e = 0; e < a.length; e++) a[e].style.backgroundColor = sBGC[_];
    for (var o = document.querySelectorAll(".main"), e = 0; e < o.length; e++) o[e].style.backgroundColor = mBGC[_];
    for (var _ = document.querySelectorAll(".tborderA"), e = 0; e < _.length; e++) _[e].style.backgroundColor = tBGC[_]
}

function addCSSRule(_, n, e, i) {
    "insertRule" in _ ? _.insertRule(n + "{" + e + "}", i) : "addRule" in _ && _.addRule(n, e, i)
}

function BufSW(v) {
    with(document.calcForm) {
        if (n_SkillSW = v, n_SkillSW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A2TD" ColSpan="6" class="subheader point" onclick="BufSW(0)">Supportive / Party Skills <span id="A2used"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR class="center">', str += '<TD ColSpan="2" class="data"><B><U>Gospel Effects</U></B></TD>', str += '<TD ColSpan="2" class="data"><B><U>Acolyte Class Buffs</U></B></TD>', str += '<TD ColSpan="2"><B><U>Other Party Buffs</U></B></TD>', str += "</TR><TR>", str += '<TD id="EN50"></TD><TD id="" class="data"></TD>', str += '<TD id="AS0_1" class="center">Blessing</TD><TD id="AS0_2" class="data"></TD>', str += '<TD id="AS6_1" class="center">Andrenaline Rush</TD><TD id="AS6_2" class="data"></TD>', str += "</TR><TR>", str += '<TD id="EN51"></TD><TD id="" class="data"></TD>', str += '<TD id="AS1_1" class="center">Increase Agi</TD><TD id="AS1_2" class="data"></TD>', str += '<TD id="AS7_1" class="center">Weapon Perfection</TD><TD id="AS7_2"></TD>', str += "</TR><TR>", str += '<TD id="EN52"></TD><TD id="" class="data"></TD>', str += '<TD id="AS4_1" class="center">Angelus</TD><TD id="AS4_2" class="data"></TD>', str += '<TD id="AS8_1" class="center">Power Thrust</TD><TD id="AS8_2" class="data"></TD>', str += "</TR><TR>", str += '<TD id="EN53"></TD><TD id="" class="data"></TD>', str += '<TD id="AS2_1" class="center">Impositio Manus</TD><TD id="AS2_2" class="data"></TD>', str += '<TD id="AS9_1" class="center">Wind Walker</TD><TD id="AS9_2"></TD>', str += "</TR><TR>", str += '<TD id="EN55"></TD><TD id="" class="data"></TD>', str += '<TD id="AS3_1" class="center">Gloria</TD><TD id="AS3_2" class="data"></TD>', str += '<TD id="" class="center">Auto-Guard</TD><TD id="AS13_2" class="data"></TD>', str += "</TR><TR>", str += '<TD id="EN54"></TD><TD id="" class="data"></TD>', str += '<TD id="AS10_1" class="center">Suffragium</TD><TD id="AS10_2" class="data"></TD>', str += '<TD id="" class="center">Shield Reflect</TD><TD id="AS14_2" class="data"></TD>', str += "</TR><TR>", str += '<TD id=""></TD><TD id="" class="data"></TD>', str += '<TD id="AS5_1" class="center">Assumptio</TD><TD id="AS5_2" class="data"></TD>', str += '<TD id="" class="center">Defender</TD><TD id="AS15_2"></TD>', str += "</TR><TR>", str += '<TD id=""></TD><TD id="" class="data"></TD>', str += '<TD id="AS12_1" class="center"># of Spirit Spheres</TD><TD id="AS12_2" class="data"></TD>', str += '<TD id="AS11_1" class="center">Providence</TD><TD id="AS11_2"></TD>', str += "</TR></TABLE>", myInnerHtml("SIENSKILL", str, 0), myInnerHtml("AS0_2", '<select name="A2_Skill0" onChange="A2(1)"></select>', 0), myInnerHtml("AS1_2", '<select name="A2_Skill1" onChange="A2(1)"></select>', 0), myInnerHtml("AS2_2", '<select name="A2_Skill2" onChange="A2(1)"></select>', 0), myInnerHtml("AS3_2", '<input type="checkbox" name="A2_Skill3" onClick="A2(1)">', 0), myInnerHtml("AS4_2", '<select name="A2_Skill4" onChange="A2(1)"></select>', 0), myInnerHtml("AS5_2", '<input type="checkbox" name="A2_Skill5" onClick="A2(1)">', 0), myInnerHtml("AS6_2", '<select name="A2_Skill6" onChange="A2(1)"></select>', 0), myInnerHtml("AS7_2", '<input type="checkbox" name="A2_Skill7" onClick="A2(1)">', 0), myInnerHtml("AS8_2", '<input type="checkbox" name="A2_Skill8" onClick="A2(1)">', 0), myInnerHtml("AS9_2", '<select name="A2_Skill9" onChange="A2(1)"></select>', 0), myInnerHtml("AS10_2", '<select name="A2_Skill10" onChange="A2(1)"></select>', 0), myInnerHtml("AS11_2", '<select name="A2_Skill11" onChange="A2(1)"></select>', 0), myInnerHtml("AS12_2", '<select name="A2_Skill12" onChange="A2(1)"></select>', 0), myInnerHtml("AS13_2", '<select name="A2_Skill13" onChange="A2(1)"></select>', 0), myInnerHtml("AS14_2", '<select name="A2_Skill14" onChange="A2(1)"></select>', 0), myInnerHtml("AS15_2", '<select name="A2_Skill15" onChange="A2(1)"></select>', 0), myInnerHtml("EN50", '<input type="checkbox" name="A5_Skill0" onClick="A2(1)">All Stats +20', 0), myInnerHtml("EN51", '<input type="checkbox" name="A5_Skill1" onClick="A2(1)">HP +100%', 0), myInnerHtml("EN52", '<input type="checkbox" name="A5_Skill2" onClick="A2(1)">SP +100%', 0), myInnerHtml("EN53", '<input type="checkbox" name="A5_Skill3" onClick="A2(1)">ATK +100%', 0), myInnerHtml("EN54", '<input type="checkbox" name="A5_Skill4" onClick="A2(1)">HIT & FLEE +50', 0), myInnerHtml("EN55", '<input type="checkbox" name="A5_Skill5" onClick="A2(1)">DEF +25%', 0), i = 0; 10 >= i; i++) A2_Skill0.options[i] = new Option(i, i), A2_Skill1.options[i] = new Option(i, i), A2_Skill4.options[i] = new Option(i, i), A2_Skill9.options[i] = new Option(i, i), A2_Skill13.options[i] = new Option(i, i), A2_Skill14.options[i] = new Option(i, i);
            for (i = 0; 5 >= i; i++) A2_Skill2.options[i] = new Option(i, i), A2_Skill11.options[i] = new Option(i, i), A2_Skill12.options[i] = new Option(i, i), A2_Skill15.options[i] = new Option(i, i);
            for (45 == n_A_JOB ? (myInnerHtml("AS12_1", "<S># of Spirit Spheres</S>", 0), A2_Skill12.disabled = !0, A2_Skill12.value = 0, n_A_Buf2[12] = 0) : (myInnerHtml("AS12_1", "# of Spirit Spheres", 0), A2_Skill12.disabled = !1), 13 == n_A_JOB || 27 == n_A_JOB ? (myInnerHtml("AS11_1", "<S>Providence</S>", 0), A2_Skill11.disabled = !0, A2_Skill11.value = 0, n_A_Buf2[11] = 0) : (myInnerHtml("AS11_1", "Providence", 0), A2_Skill11.disabled = !1), i = 0; 3 >= i; i++) A2_Skill10.options[i] = new Option(i, i);
            A2_Skill6.options[0] = new Option("OFF", 0), A2_Skill6.options[1] = new Option("Regular AR", 1), A2_Skill6.options[2] = new Option("Full AR", 2), A2_Skill6.options[3] = new Option("AR Scroll", 3), A2_Skill0.value = n_A_Buf2[0], A2_Skill1.value = n_A_Buf2[1], A2_Skill2.value = n_A_Buf2[2], A2_Skill3.checked = n_A_Buf2[3], A2_Skill4.value = n_A_Buf2[4], A2_Skill5.checked = n_A_Buf2[5], A2_Skill6.value = n_A_Buf2[6], A2_Skill7.checked = n_A_Buf2[7], A2_Skill8.checked = n_A_Buf2[8], A2_Skill9.value = n_A_Buf2[9], A2_Skill10.value = n_A_Buf2[10], A2_Skill11.value = n_A_Buf2[11], A2_Skill12.value = n_A_Buf2[12], A2_Skill13.value = n_A_Buf2[13], A2_Skill14.value = n_A_Buf2[14], A2_Skill15.value = n_A_Buf2[15], A5_Skill0.checked = n_A_Buf2[16], A5_Skill1.checked = n_A_Buf2[17], A5_Skill2.checked = n_A_Buf2[18], A5_Skill3.checked = n_A_Buf2[19], A5_Skill4.checked = n_A_Buf2[20], A5_Skill5.checked = n_A_Buf2[21]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="A2TD" class="subheader point" onclick="BufSW(1)">Supportive / Party Skills <span id="A2used"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("SIENSKILL", str, 0)
        }
        A2(0)
    }
}

function A2(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; 21 >= e; e++)
        if (0 != n_A_Buf2[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A2TD").style.backgroundColor = sBGC[i], myInnerHtml("A2used", "", 0)) : (document.getElementById("A2TD").style.backgroundColor = saBGC[i], myInnerHtml("A2used", " <B>[Active]</B>", 0))
}

function Buf3SW(v) {
    with(document.calcForm) {
        if (n_Skill3SW = v, n_Skill3SW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A3TD" ColSpan="6" class="subheader point" onclick="Buf3SW(0)">Music and Dance Skills <span id="A3used"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD id="EN0_1"></TD><TD id="EN0_2" class="data"></TD><TD id="EN0_3"></TD><TD id="EN0_4" class="data"></TD><TD id="EN0_5"></TD><TD id="EN0_6"></TD></TR>', str += '<TR><TD id="EN1_1"></TD><TD id="EN1_2" class="data"></TD><TD id="EN1_3"></TD><TD id="EN1_4" class="data"></TD><TD id="EN1_5"></TD><TD id="EN1_6"></TD></TR>', str += '<TR><TD id="EN2_1"></TD><TD id="EN2_2" class="data"></TD><TD id="EN2_3" style="line-height:165%;"></TD><TD id="EN2_4" class="data"></TD><TD id="EN2_5"></TD><TD id="EN2_6"></TD></TR>', str += '<TR><TD id="EN3_1"></TD><TD id="EN3_2" class="data"></TD><TD id="EN3_3"></TD><TD id="EN3_4" class="data"></TD><TD id="EN3_5"></TD><TD id="EN3_6"></TD></TR>', str += '<TR><TD id="EN4_1"></TD><TD id="EN4_2" class="data"></TD><TD id="EN4_3"></TD><TD id="EN4_4" class="data"></TD><TD id="EN4_5"></TD><TD id="EN4_6"></TD></TR>', str += '<TR><TD id="EN5_1"></TD><TD id="EN5_2" class="data"></TD><TD id="EN5_3"></TD><TD id="EN5_4" class="data"></TD><TD id="EN5_5"></TD><TD id="EN5_6"></TD></TR>', str += '<TR class="dotB"><TD id="EN6_1"></TD><TD id="EN6_2" class="data"></TD><TD id="EN6_3"></TD><TD id="EN6_4" class="data"></TD><TD id="EN6_5"></TD><TD id="EN6_6"></TD></TR>', str += '<TR><TD id="EN7_1"></TD><TD id="EN7_2" class="data"></TD><TD id="EN8_1" ColSpan="2"></TD><TD id="EN8_2"></TD><TD></TD></TR>', str += '<TR class="dotB"><TD id="EN9_1"></TD><TD id="EN9_2" class="data"></TD><TD id="EN10_1" ColSpan="2"></TD><TD id="EN10_2"></TD><TD></TD></TR>', str += '<TR><TD colspan=6><span id="EN11_1"></span><span id="EN11_2"></span><span id="EN11_1a"></span></TD></TR></TABLE>', myInnerHtml("SP_SIEN01", str, 0), SRV ? name_CS3SW_SKILL = ["A Whistle", "Assassin Cross of Sunset", "Poem of Bragi", "The Apple of Idun", "Humming", "Fortune's Kiss", "Service for You", "Invulnerable Siegfried", "Mr. Kim A Rich Man", "A Drum on the Battlefield", "The Ring of Nibelungen"] : name_CS3SW_SKILL = ["Perfect Tabulature", "Impressive Rift", "Magic Strings", "Song of Lutie", "Focus Ballet", "Lady Luck", "Gypsie's Kiss", "Acoustic Rhythm", "Mental Sensing", "Battle Theme", "Harmonic Lick"], i = 0; 10 >= i; i++) myInnerHtml("EN" + i + "_1", name_CS3SW_SKILL[i], 0);
            for (myInnerHtml("EN0_2", '<select name="A3_Skill0_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN1_2", '<select name="A3_Skill1_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN2_2", '<select name="A3_Skill2_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN3_2", '<select name="A3_Skill3_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN4_2", '<select name="A3_Skill4_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN5_2", '<select name="A3_Skill5_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN6_2", '<select name="A3_Skill6_1" onChange="Skill3SW_2()|A3(1)"></select>', 0), myInnerHtml("EN7_2", '<select name="A3_Skill7" onChange="A3(1)"></select>', 0), myInnerHtml("EN8_2", '<select name="A3_Skill8" onChange="A3(1)"></select>', 0), myInnerHtml("EN9_2", '<select name="A3_Skill9" onChange="A3(1)"></select>', 0), myInnerHtml("EN10_2", '<select name="A3_Skill10" onChange="A3(1)"></select>', 0), myInnerHtml("EN11_1", '<input type="checkbox" name="A3_Skill11" onClick="Skill3SW_2()|calc()">Marionette Control', 0), i = 0; 10 >= i; i++) A3_Skill0_1.options[i] = new Option(i, i), A3_Skill1_1.options[i] = new Option(i, i), A3_Skill2_1.options[i] = new Option(i, i), A3_Skill3_1.options[i] = new Option(i, i), A3_Skill4_1.options[i] = new Option(i, i), A3_Skill5_1.options[i] = new Option(i, i), A3_Skill6_1.options[i] = new Option(i, i);
            for (i = 0; 5 >= i; i++) A3_Skill7.options[i] = new Option(i, i), A3_Skill8.options[i] = new Option(i, i), A3_Skill9.options[i] = new Option(i, i), A3_Skill10.options[i] = new Option(i, i);
            A3_Skill0_1.value = n_A_Buf3[0], A3_Skill1_1.value = n_A_Buf3[1], A3_Skill2_1.value = n_A_Buf3[2], A3_Skill3_1.value = n_A_Buf3[3], A3_Skill4_1.value = n_A_Buf3[4], A3_Skill5_1.value = n_A_Buf3[5], A3_Skill6_1.value = n_A_Buf3[6], A3_Skill7.value = n_A_Buf3[7], A3_Skill8.value = n_A_Buf3[8], A3_Skill9.value = n_A_Buf3[9], A3_Skill10.value = n_A_Buf3[10], A3_Skill11.checked = n_A_Buf3[11],
                Skill3SW_2()
        } else {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A3TD" class="subheader point" onclick="Buf3SW(1)">Music and Dance Skills <span id="A3used"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("SP_SIEN01", str, 0), i = 0; 11 >= i; i++) SWs3sw[i] = 0
        }
        A3(0)
    }
}

function Skill3SW_2() {
    with(document.calcForm) {
        if (n_A_Buf3[0] = eval(A3_Skill0_1.value), n_A_Buf3[1] = eval(A3_Skill1_1.value), n_A_Buf3[2] = eval(A3_Skill2_1.value), n_A_Buf3[3] = eval(A3_Skill3_1.value), n_A_Buf3[4] = eval(A3_Skill4_1.value), n_A_Buf3[5] = eval(A3_Skill5_1.value), n_A_Buf3[6] = eval(A3_Skill6_1.value), n_A_Buf3[11] = eval(A3_Skill11.checked), 0 != n_A_Buf3[0]) {
            if (0 == SWs3sw[0]) {
                for (0 == n_A_Buf3[20] && (n_A_Buf3[20] = 100, n_A_Buf3[30] = 10), myInnerHtml("EN0_3", "Bard's AGI", 0), myInnerHtml("EN0_4", '<select name="A3_Skill0_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN0_5", "Musical Lessons", 0), myInnerHtml("EN0_6", '<select name="A3_Skill0_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill0_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill0_3.options[i] = new Option(i, i);
                SWs3sw[0] = 1, A3_Skill0_2.value = n_A_Buf3[20], A3_Skill0_3.value = n_A_Buf3[30]
            }
        } else SWs3sw[0] = 0, myInnerHtml("EN0_3", "", 0), myInnerHtml("EN0_4", "", 0), myInnerHtml("EN0_5", "", 0), myInnerHtml("EN0_6", "", 0);
        if (0 != n_A_Buf3[1]) {
            if (0 == SWs3sw[1]) {
                for (0 == n_A_Buf3[21] && (n_A_Buf3[21] = 100, n_A_Buf3[31] = 10), myInnerHtml("EN1_3", "Bard's AGI", 0), myInnerHtml("EN1_4", '<select name="A3_Skill1_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN1_5", "Musical Lessons", 0), myInnerHtml("EN1_6", '<select name="A3_Skill1_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill1_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill1_3.options[i] = new Option(i, i);
                SWs3sw[1] = 1, A3_Skill1_2.value = n_A_Buf3[21], A3_Skill1_3.value = n_A_Buf3[31]
            }
        } else SWs3sw[1] = 0, myInnerHtml("EN1_3", "", 0), myInnerHtml("EN1_4", "", 0), myInnerHtml("EN1_5", "", 0), myInnerHtml("EN1_6", "", 0);
        if (0 != n_A_Buf3[2]) {
            if (0 == SWs3sw[2]) {
                for (0 == n_A_Buf3[22] && (n_A_Buf3[22] = 100, n_A_Buf3[29] = 100, n_A_Buf3[32] = 10), myInnerHtml("EN2_3", "Bard's DEX<br>Bard's INT", 0), myInnerHtml("EN2_4", '<select name="A3_Skill2_2" onChange="A3(1)"></select><br><select name="A3_Skill2_3" onChange="A3(1)"></select>', 0), myInnerHtml("EN2_5", "Musical Lessons", 0), myInnerHtml("EN2_6", '<select name="A3_Skill2_4" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill2_2.options[i - 1] = new Option(i, i);
                for (i = 1; 150 >= i; i++) A3_Skill2_3.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill2_4.options[i] = new Option(i, i);
                SWs3sw[2] = 1, A3_Skill2_2.value = n_A_Buf3[22], A3_Skill2_3.value = n_A_Buf3[29], A3_Skill2_4.value = n_A_Buf3[32]
            }
        } else SWs3sw[2] = 0, myInnerHtml("EN2_3", "", 0), myInnerHtml("EN2_4", "", 0), myInnerHtml("EN2_5", "", 0), myInnerHtml("EN2_6", "", 0);
        if (0 != n_A_Buf3[3]) {
            if (0 == SWs3sw[3]) {
                for (0 == n_A_Buf3[23] && (n_A_Buf3[23] = 100, n_A_Buf3[33] = 10), myInnerHtml("EN3_3", "Bard's VIT", 0), myInnerHtml("EN3_4", '<select name="A3_Skill3_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN3_5", "Musical Lessons", 0), myInnerHtml("EN3_6", '<select name="A3_Skill3_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill3_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill3_3.options[i] = new Option(i, i);
                SWs3sw[3] = 1, A3_Skill3_2.value = n_A_Buf3[23], A3_Skill3_3.value = n_A_Buf3[33]
            }
        } else SWs3sw[3] = 0, myInnerHtml("EN3_3", "", 0), myInnerHtml("EN3_4", "", 0), myInnerHtml("EN3_5", "", 0), myInnerHtml("EN3_6", "", 0);
        if (0 != n_A_Buf3[4]) {
            if (0 == SWs3sw[4]) {
                for (0 == n_A_Buf3[24] && (n_A_Buf3[24] = 130, n_A_Buf3[34] = 10), myInnerHtml("EN4_3", "Dancer's DEX", 0), myInnerHtml("EN4_4", '<select name="A3_Skill4_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN4_5", "Dancing Lessons", 0), myInnerHtml("EN4_6", '<select name="A3_Skill4_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill4_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill4_3.options[i] = new Option(i, i);
                SWs3sw[4] = 1, A3_Skill4_2.value = n_A_Buf3[24], A3_Skill4_3.value = n_A_Buf3[34]
            }
        } else SWs3sw[4] = 0, myInnerHtml("EN4_3", "", 0), myInnerHtml("EN4_4", "", 0), myInnerHtml("EN4_5", "", 0), myInnerHtml("EN4_6", "", 0);
        if (0 != n_A_Buf3[5]) {
            if (0 == SWs3sw[5]) {
                for (0 == n_A_Buf3[25] && (n_A_Buf3[25] = 50, n_A_Buf3[35] = 10), myInnerHtml("EN5_3", "Dancer's LUK", 0), myInnerHtml("EN5_4", '<select name="A3_Skill5_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN5_5", "Dancing Lessons", 0), myInnerHtml("EN5_6", '<select name="A3_Skill5_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill5_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill5_3.options[i] = new Option(i, i);
                SWs3sw[5] = 1, A3_Skill5_2.value = n_A_Buf3[25], A3_Skill5_3.value = n_A_Buf3[35]
            }
        } else SWs3sw[5] = 0, myInnerHtml("EN5_3", "", 0), myInnerHtml("EN5_4", "", 0), myInnerHtml("EN5_5", "", 0), myInnerHtml("EN5_6", "", 0);
        if (0 != n_A_Buf3[6]) {
            if (0 == SWs3sw[6]) {
                for (0 == n_A_Buf3[26] && (n_A_Buf3[26] = 50, n_A_Buf3[36] = 10), myInnerHtml("EN6_3", "Dancer's INT", 0), myInnerHtml("EN6_4", '<select name="A3_Skill6_2" onChange="A3(1)"></select>', 0), myInnerHtml("EN6_5", "Dancing Lessons", 0), myInnerHtml("EN6_6", '<select name="A3_Skill6_3" onChange="A3(1)"></select>', 0), i = 1; 200 >= i; i++) A3_Skill6_2.options[i - 1] = new Option(i, i);
                for (i = 0; 10 >= i; i++) A3_Skill6_3.options[i] = new Option(i, i);
                SWs3sw[6] = 1, A3_Skill6_2.value = n_A_Buf3[26], A3_Skill6_3.value = n_A_Buf3[36]
            }
        } else SWs3sw[6] = 0, myInnerHtml("EN6_3", "", 0), myInnerHtml("EN6_4", "", 0), myInnerHtml("EN6_5", "", 0), myInnerHtml("EN6_6", "", 0);
        if (0 != n_A_Buf3[11]) {
            if (0 == SWs3sw[11]) {
                for (myInnerHtml("EN11_2", '<br>Controller\'s Stats: <select name="A3_Skill11_STR" onChange="A3(1)"></select><select name="A3_Skill11_AGI" onChange="A3(1)"></select><select name="A3_Skill11_VIT" onChange="A3(1)"></select><select name="A3_Skill11_INT" onChange="A3(1)"></select><select name="A3_Skill11_DEX" onChange="A3(1)"></select><select name="A3_Skill11_LUK" onChange="A3(1)"></select><BR><input type="checkbox" name="A3_Skill11a" onClick="A3(1)">Status compensation (adjustment for equipment / human calculation)', 0), A3_Skill11_STR.options[0] = new Option("STR", 0), A3_Skill11_AGI.options[0] = new Option("AGI", 0), A3_Skill11_VIT.options[0] = new Option("VIT", 0), A3_Skill11_INT.options[0] = new Option("INT", 0), A3_Skill11_DEX.options[0] = new Option("DEX", 0), A3_Skill11_LUK.options[0] = new Option("LUK", 0), i = 1; 99 >= i; i++) A3_Skill11_STR.options[i] = new Option(i, i), A3_Skill11_AGI.options[i] = new Option(i, i), A3_Skill11_VIT.options[i] = new Option(i, i), A3_Skill11_INT.options[i] = new Option(i, i), A3_Skill11_DEX.options[i] = new Option(i, i), A3_Skill11_LUK.options[i] = new Option(i, i);
                SWs3sw[11] = 1, A3_Skill11_STR.value = n_A_Buf3[12], A3_Skill11_AGI.value = n_A_Buf3[13], A3_Skill11_VIT.value = n_A_Buf3[14], A3_Skill11_INT.value = n_A_Buf3[15], A3_Skill11_DEX.value = n_A_Buf3[16], A3_Skill11_LUK.value = n_A_Buf3[17], A3_Skill11a.checked = n_A_Buf3[18]
            }
        } else SWs3sw[11] = 0, myInnerHtml("EN11_2", "", 0)
    }
}

function A3(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; 17 >= e; e++)
        if (11 != e && 0 != n_A_Buf3[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A3TD").style.backgroundColor = sBGC[i], myInnerHtml("A3used", "", 0)) : (document.getElementById("A3TD").style.backgroundColor = saBGC[i], myInnerHtml("A3used", " <B>[Active]</B>", 0))
}

function Buf4SW(v) {
    with(document.calcForm) {
        if (n_Skill4SW = v, n_Skill4SW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A4TD" ColSpan="2" class="subheader point" onclick="Buf4SW(0)">Guild Skills <span id="A4used"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD id="EN40_1" class="center"></TD><TD id="EN40_2"></TD></TR>', str += '<TR><TD id="EN41_1" class="center"></TD><TD id="EN41_2"></TD></TR>', str += '<TR><TD id="EN42_1" class="center"></TD><TD id="EN42_2"></TD></TR>', str += '<TR><TD id="EN43_1" class="center"></TD><TD id="EN43_2"></TD></TR>', str += '<TR><TD id="EN44_1" class="center"></TD><TD id="EN44_2"></TD></TR></TABLE>', myInnerHtml("SP_SIEN02", str, 0), name_CS4SW_SKILL = ["Battle Orders", "Great Leadership", "Wounds of Glory", "Soul of Cold", "Sharp Hawk Eyes"], i = 0; 4 >= i; i++) myInnerHtml("EN4" + i + "_1", name_CS4SW_SKILL[i], 0);
            for (myInnerHtml("EN40_2", '<input type="checkbox" name="A3_Skill40" onClick="A4(1)">', 0), myInnerHtml("EN41_2", '<select name="A3_Skill41" onChange="A4(1)"></select>', 0), myInnerHtml("EN42_2", '<select name="A3_Skill42" onChange="A4(1)"></select>', 0), myInnerHtml("EN43_2", '<select name="A3_Skill43" onChange="A4(1)"></select>', 0), myInnerHtml("EN44_2", '<select name="A3_Skill44" onChange="A4(1)"></select>', 0), i = 0; 5 >= i; i++) A3_Skill41.options[i] = new Option(i, i), A3_Skill42.options[i] = new Option(i, i), A3_Skill43.options[i] = new Option(i, i), A3_Skill44.options[i] = new Option(i, i);
            A3_Skill40.checked = n_A_Buf3[40], A3_Skill41.value = n_A_Buf3[41], A3_Skill42.value = n_A_Buf3[42], A3_Skill43.value = n_A_Buf3[43], A3_Skill44.value = n_A_Buf3[44]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="A4TD" class="subheader point" onclick="Buf4SW(1)">Guild Skills <span id="A4used"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("SP_SIEN02", str, 0)
        }
        A4(0)
    }
}

function A4(_) {
    1 == _ && calc();
    for (var n = 0, e = 40; 44 >= e; e++)
        if (0 != n_A_Buf3[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A4TD").style.backgroundColor = sBGC[i], myInnerHtml("A4used", "", 0)) : (document.getElementById("A4TD").style.backgroundColor = saBGC[i], myInnerHtml("A4used", " <B>[Active]</B>", 0))
}

function Buf6SW(v){
  with(document.calcForm){
    if(n_Skill6SW=v,n_Skill6SW){
      var str;
      for(str='<TABLE class="tborder">',
          str+='<TR><TD id="A6TD" ColSpan="4" class="subheader point" onclick="Buf6SW(0)">Miscellaneous Effects on Player<span id="A6used"></span>',
          str+='<DIV class="right">(click to hide)</DIV></TD></TR>',
          str+='<TR><TD id="EN60_1" class="center"></TD><TD id="EN60_2" class="data"></TD><TD class="center">Poison</TD><TD id="EN62_2"></TD></TR>',
          str+='<TR><TD class="center">Provoke (self)</TD><TD id="EN63_2" class="data"></TD><TD class="center">Stun</TD><TD id="EN75_2"></TD></TR>',
          str+='<TR><TD class="center">Mind Breaker (self)</TD><TD id="EN61_2" class="data"></TD><TD class="center">Freeze</TD><TD id="EN78_2"></TD></TR>',
          str+='<TR><TD class="center">AGI Down</TD><TD id="EN66_2" class="data"></TD><TD class="center">Curse</TD><TD id="EN64_2"></TD></TR>',
          str+='<TR><TD class="center">Quagmire</TD><TD id="EN68_2" class="data"></TD><TD class="center">Blind</TD><TD id="EN74_2"></TD></TR>',
          str+='<TR><TD class="center">Wall of Fog</TD><TD id="EN70_2" class="data"></TD><TD class="center">Sleep</TD><TD id="EN77_2"></TD></TR>',
          str+='<TR><TD class="center">Undead Attribute Change</TD><TD id="EN65_2" class="data"></TD><TD class="center">Stone</TD><TD id="EN76_2"></TD></TR>',
          str+='<TR><TD class="center">Holy Armor [B.S.S.]</TD><TD id="EN67_2" class="data"></TD><TD class="center">Bleeding</TD><TD id="EN80_2"></TD></TR>',
          str+='<TR><TD class="center">Magnum Break Bonus</TD><TD id="EN69_2" class="data"></TD><TD class="center">Lex Aeterna</TD><TD id="EN79_2"></TD></TR>',
          str+='<TR><TD class="center">Set CRIT to 0%</TD><TD id="EN71_2" class="data"></TD><TD class="center">Critical Wounds</TD><TD id="EN81_2"></TD></TR>',
          str+='<TR><TD id="EN72_1" class="center" colspan="3">All Stats +1 [SuperNovice Wife Undying Love Bonus]</TD><TD id="EN72_2"></TD></TR>',
          str+='<TR><TD id="EN73_1" class="center" colspan="3">Quad Exp & Bonus to Stats</TD><TD id="EN73_2"></TD></TR>',
          str+="</TABLE>",
          myInnerHtml("SP_SIEN04",str,0),
          myInnerHtml("EN60_1",'<select name="A6_Skill0" onChange="StAllCalc()"></select>',0),
          myInnerHtml("EN60_2",'<select name="A6_Skill1" onChange="A6(1)"></select>',0),
          myInnerHtml("EN61_2",'<select name="A6_Skill4" onChange="A6(1)"></select>',0),
          myInnerHtml("EN62_2",'<input type="checkbox" name="A_IJYOU2" onClick="A6(1)">',0),
          myInnerHtml("EN63_2",'<select name="A6_Skill5" onChange="A6(1)"></select>',0),
          myInnerHtml("EN64_2",'<input type="checkbox" name="A_IJYOU3" onClick="A6(1)">',0),
          myInnerHtml("EN65_2",'<input type="checkbox" name="A6_Skill8" onClick="A6(1)">',0),
          myInnerHtml("EN66_2",'<select name="A_IJYOU1" onChange="A6(1)"></select>',0),
          myInnerHtml("EN67_2",'<input type="checkbox" name="A6_Skill6" onClick="A6(1)">',0),
          myInnerHtml("EN68_2",'<select name="A_IJYOU0" onChange="A6(1)"></select>',0),
          myInnerHtml("EN69_2",'<input type="checkbox" name="A6_Skill7" onClick="A6(1)">',0),
          myInnerHtml("EN70_2",'<input type="checkbox" name="A6_Skill3" onClick="A6(1)">',0),
          myInnerHtml("EN71_2",'<input type="checkbox" name="A6_Skill9" onClick="A6(1)">',0),
          myInnerHtml("EN72_2",'<input type="checkbox" name="A6_Skill10" onClick="A6(1)">',0),
          myInnerHtml("EN73_2",'<select name="A6_Skill2" onChange="A6(1)"></select>',0),
          myInnerHtml("EN74_2",'<input type="checkbox" name="A6_Skill11" onClick="A6(1)">',0),
          myInnerHtml("EN75_2",'<input type="checkbox" name="A6_Skill12" onClick="A6(1)">',0),
          myInnerHtml("EN76_2",'<input type="checkbox" name="A6_Skill13" onClick="A6(1)">',0),
          myInnerHtml("EN77_2",'<input type="checkbox" name="A6_Skill14" onClick="A6(1)">',0),
          myInnerHtml("EN78_2",'<input type="checkbox" name="A6_Skill15" onClick="A6(1)">',0),
          myInnerHtml("EN79_2",'<input type="checkbox" name="A6_Skill16" onClick="A6(1)">',0),
          myInnerHtml("EN80_2",'<input type="checkbox" name="A6_Skill17" onClick="A6(1)">',0),
          myInnerHtml("EN81_2",'<select name="A6_Skill18" onChange="A6(1)"></select>',0),
          A6_Skill0.options[0]=new Option("Volcano",0),
          A6_Skill0.options[1]=new Option("Deluge",1),
          A6_Skill0.options[2]=new Option("Violent Gale",2),
          i=0;5>=i;i++
         )A6_Skill1.options[i]=new Option(i, i);
      A6_Skill2.options[0]=new Option("None",0);
      A6_Skill2.options[1]=new Option("ALL+3",1);
      A6_Skill2.options[2]=new Option("ALL+5",2);
      for(i=0;5>=i;i++)A6_Skill4.options[i]=new Option(i,i),A6_Skill18.options[i]=new Option(i,i);
      for(i=0;10>=i;i++)A6_Skill5.options[i]=new Option(i,i);
      for(A_IJYOU0.options[0]=new Option("-",0),i=1;5>=i;i++) A_IJYOU0.options[i]=new Option("Lv"+i,i);
      for(A_IJYOU1.options[0]=new Option("-",0),i=1;10>=i;i++) A_IJYOU1.options[i]=new Option("Lv"+i,i);
      A_IJYOU1.options[11]=new Option("Lv46",46),
      A6_Skill0.value=n_A_Buf6[0],
      A6_Skill1.value=n_A_Buf6[1],
      A6_Skill2.value=n_A_Buf6[2],
      A6_Skill3.checked=n_A_Buf6[3],
      A6_Skill4.value=n_A_Buf6[4],
      A6_Skill5.value=n_A_Buf6[5],
      A6_Skill6.checked=n_A_Buf6[6],
      A6_Skill7.checked=n_A_Buf6[7],
      A6_Skill8.checked=n_A_Buf6[8],
      A6_Skill9.checked=n_A_Buf6[9],
      A6_Skill10.checked=n_A_Buf6[10],
      A6_Skill11.checked=n_A_Buf6[11],
      A6_Skill12.checked=n_A_Buf6[12],
      A6_Skill13.checked=n_A_Buf6[13],
      A6_Skill14.checked=n_A_Buf6[14],
      A6_Skill15.checked=n_A_Buf6[15],
      A6_Skill16.checked=n_A_Buf6[16],
      A6_Skill17.checked=n_A_Buf6[17],
      A6_Skill18.value=n_A_Buf6[18],
      A_IJYOU0.value=n_A_Buf6[19],
      A_IJYOU1.value=n_A_Buf6[20],
      A_IJYOU2.checked=n_A_Buf6[21],
      A_IJYOU3.checked=n_A_Buf6[22]
    }else{
      var str;
      str='<table class="tborder">',
      str+='<TR><TD id="A6TD" class="subheader point" onclick="Buf6SW(1)">Miscellaneous Effects on Player<span id="A6used"></span>',
      str+='<div class="right">(click to show)</div></TD></TR></TABLE>',
      myInnerHtml("SP_SIEN04",str,0)
    }
  A6(0)
  }
}

function A6(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; e < n_A_Buf6.length; e++)
        if (0 != e && 0 != n_A_Buf6[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A6TD").style.backgroundColor = sBGC[i], myInnerHtml("A6used", "", 0)) : (document.getElementById("A6TD").style.backgroundColor = saBGC[i], myInnerHtml("A6used", " <B>[Active]</B>", 0))
}

function Buf7SW(v) {
    with(document.calcForm) {
        if (n_Skill7SW = v, n_Skill7SW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A7TD" ColSpan="3" class="subheader point" onclick="Buf7SW(0)">Food / Speed Potions / other Items <span id="A7used"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD colspan="3" class="data dotB"><span id="EN73"></span><span id="EN74"></span><span id="EN75"></span><span id="EN76"></span><span id="EN77"></span><span id="EN78"></span></TD></TR>', str += '<TR><TD id="EN722" class="data"></TD><TD class="data"><Font size="2"><B>Battle Grounds Food</B></Font></TD><TD><Font size="2"><B>Elemental Resist Potions</B></Font></TD></TR>', str += '<TR><TD id="EN717" class="data"></TD><TD id="EN723" class="data"></TD><TD id="EN711"></TD></TR>', str += '<TR><TD id="EN720" class="data"></TD><TD id="EN724" class="data"></TD><TD id="EN712"></TD></TR>', str += '<TR><TD id="EN710" class="data"></TD><TD id="EN725" class="data"></TD><TD id="EN713"></TD></TR>', str += '<TR><TD id="EN79" class="data"></TD><TD id="EN726" class="dotB data"></TD><TD id="EN714" class="dotB"></TD></TR>', str += '<TR><TD id="EN70" class="data"></TD><TD colspan="2"><Font size="2"><B>New World Food</B></Font></TD></TR>', str += '<TR><TD id="EN71" class="data"></TD><TD colspan="2" id="EN727"></TD></TR>', str += '<TR><TD id="EN72" class="data"></TD><TD colspan="2" id="EN728"></TD></TR>', str += '<TR><TD id="EN715" class="data"></TD><TD colspan="2" id="EN729"></TD></TR>', str += '<TR><TD id="EN716" class="data"></TD><TD colspan="2" id="EN730"></TD></TR>', str += '<TR><TD id="EN718" class="data"></TD><TD colspan="2" id="EN731"></TD></TR>', str += '<TR><TD id="EN719" class="data"></TD><TD colspan="2" id="EN732"></TD></TR>', str += '<TR><TD id="EN721" class="data"></TD><TD colspan="2" id="EN733"></TD></TR>', str += '<TR><TD id="EN743" class="data"></TD><TD colspan="2" id="EN734"></TD></TR>', str += '<TR><TD id="EN738" class="data"></TD><TD colspan="2" id="EN735" class="dotB"></TD></TR>', str += '<TR><TD id="EN737" class="data"></TD><TD colspan="2"><Font size="2"><B>Stat+20 Food</B></Font></TD></TR>', str += '<TR><TD id="EN742" class="data"></TD><TD colspan="2" id="EN741"></TD></TR>', str += '<TR><TD id="EN746" class="data"></TD><TD colspan="2" id="EN747"></TD></TR>', str += '<TR><TD id="EN744" class="data"></TD><TD colspan="2" id="EN748"></TD></TR>', str += '<TR><TD id="EN745" class="data"></TD><TD colspan="2" id="EN749"></TD></TR>', str += '<TR><TD id="EN739" class="data"></TD><TD colspan="2" id="EN750"></TD></TR>', str += '<TR><TD id="EN740" class="data"></TD><TD colspan="2" id="EN751"></TD></TR>', str += "</TABLE>", myInnerHtml("SP_SIEN05", str, 0), myInnerHtml("EN70", '<input type="checkbox" name="A7_Skill0" onClick="A7(1)">Sesame Pastry [HIT +30]', 0), myInnerHtml("EN71", '<input type="checkbox" name="A7_Skill1" onClick="A7(1)">Honey Pastry [FLEE +30]', 0), myInnerHtml("EN72", '<input type="checkbox" name="A7_Skill2" onClick="A7(1)">Rainbow Cake [ATK/MATK +10]', 0), myInnerHtml("EN79", '<input type="checkbox" name="A7_Skill9" onClick="A7(1)">Box of Resentment [ATK +20]', 0), myInnerHtml("EN710", '<input type="checkbox" name="A7_Skill10" onClick="A7(1)">Box of Drowsiness [MATK +20]', 0), myInnerHtml("EN711", '<input type="checkbox" name="A7_Skill11" onClick="A7(1)">Coldproof Potion', 0), myInnerHtml("EN712", '<input type="checkbox" name="A7_Skill12" onClick="A7(1)">Earthproof Potion', 0), myInnerHtml("EN713", '<input type="checkbox" name="A7_Skill13" onClick="A7(1)">Fireproof Potion', 0), myInnerHtml("EN714", '<input type="checkbox" name="A7_Skill14" onClick="A7(1)">Thunderproof Potion', 0), myInnerHtml("EN715", '<input type="checkbox" name="A7_Skill26" onClick="A7(1)">Guarana Candy [ASPD +10%, HIT +5]', 0), myInnerHtml("EN716", '<input type="checkbox" name="A7_Skill16" onClick="A7(1)">Buche de Noel [HIT +3%, CRIT +7, MaxHP&SP +3%]', 0), myInnerHtml("EN717", '<input type="checkbox" name="A7_Skill31" onClick="A7(1)">Aloevera [Self Provoke Lv 1]', 0), myInnerHtml("EN718", '<input type="checkbox" name="A7_Skill32" onClick="A7(1)">Small/Big Defense Potion [DEF +3]', 0), myInnerHtml("EN719", '<input type="checkbox" name="A7_Skill33" onClick="A7(1)">Small/Big Magic Defense Potion [MDEF +3]', 0), myInnerHtml("EN720", '<input type="checkbox" name="A7_Skill34" onClick="A7(1)">Box of Gloom [Attention Concentrate Lv 1]', 0), myInnerHtml("EN721", '<input type="checkbox" name="A7_Skill36" onClick="A7(1)">Halo-Halo [All Stats +3]', 0), myInnerHtml("EN722", '<select name="A_SpeedPOT" onChange="A7(1)"><option value="0">(No Speed Potion)</option></select>', 0), myInnerHtml("EN723", '<input type="checkbox" name="A7_Skill27" onClick="A7(1)">Military Ration B [HIT +33]', 0), myInnerHtml("EN724", '<input type="checkbox" name="A7_Skill28" onClick="A7(1)">Military Ration C [FLEE +33]', 0), myInnerHtml("EN725", '<input type="checkbox" name="A7_Skill29" onClick="A7(1)">Tasty Pink Ration [ATK +15]', 0), myInnerHtml("EN726", '<input type="checkbox" name="A7_Skill30" onClick="A7(1)">Tasty White Ration [MATK +15]', 0), myInnerHtml("EN727", '<input type="checkbox" name="A7_Skill17" onClick="A7(1)">Rune Strawberry Cake [ATK, MATK +5%]', 0), myInnerHtml("EN728", '<input type="checkbox" name="A7_Skill18" onClick="A7(1)">Schwartzwald Pine Jubilee [HIT +10, FLEE +20]', 0), myInnerHtml("EN729", '<input type="checkbox" name="A7_Skill19" onClick="A7(1)">Arunafeltz Desert Sandwich [CRIT +7]', 0), myInnerHtml("EN730", '<input type="checkbox" name="A7_Skill20" onClick="A7(1)">Manuk\'s Sturdiness [ATK based damage on Manuk maps +10%]', 0), myInnerHtml("EN731", '<input type="checkbox" name="A7_Skill21" onClick="A7(1)">Manuk\'s Faith [MATK based damage on Manuk maps +10%]', 0), myInnerHtml("EN732", '<input type="checkbox" name="A7_Skill22" onClick="A7(1)">Manuk\'s Will [Received damage on Manuk maps -10%]', 0), myInnerHtml("EN733", '<input type="checkbox" name="A7_Skill23" onClick="A7(1)">Pinguicula\'s Fruit Jam [ATK based dmg on Splendide maps +10%]', 0), myInnerHtml("EN734", '<input type="checkbox" name="A7_Skill24" onClick="A7(1)">Cornus\' Tear [MATK based damage on Splendide maps +10%]', 0), myInnerHtml("EN735", '<input type="checkbox" name="A7_Skill25" onClick="A7(1)">Luciola\'s Honey Jam [Received damage on Splendide maps -10%]', 0), myInnerHtml("EN737", '<input type="checkbox" name="A7_Skill37" onClick="A7(1)">Lucky Rice Cake [LUK +21]', 0), myInnerHtml("EN738", '<input type="checkbox" name="A7_Skill38" onClick="A7(1)">Distilled Fighting Spirit [ATK +30]', 0), myInnerHtml("EN746", "<input type=checkbox name=A7_Skill46 onClick=A7(1)>Tyr's Blessing [FLEE +30, HIT +30, ATK +20, MATK +20]", 0), myInnerHtml("EN742", '<select name="A7_Skill42" onChange="A7(1)" style="width:175px;"><option value="0">(None)</option></select>', 0), myInnerHtml("EN743", '<input type="checkbox" name="A7_Skill43" onClick="A7(1)">Luxurious Western Food [All Stats +3]', 0), myInnerHtml("EN744", '<input type="checkbox" name="A7_Skill44" onClick="A7(1)">Ginger Bread [ASPD +% (same as Awakening Potion)]', 0), myInnerHtml("EN745", '<input type="checkbox" name="A7_Skill45" onClick="A7(1)">Regeneration Potion [Items/skills recover +20%]', 0), myInnerHtml("EN739", '<select name="A7_Skill39" onChange="A7(1)"><option value="0">(No HP Increase Potion)</option></select>', 0), myInnerHtml("EN740", '<select name="A7_Skill40" onChange="A7(1)"><option value="0">(No SP Increase Potion)</option></select>', 0), myInnerHtml("EN741", '<input type="checkbox" name="A7_Skill41" onClick="A7(1)">Savage BBQ [STR +20]', 0), myInnerHtml("EN747", '<input type="checkbox" name="A7_Skill47" onClick="A7(1)">Cocktail Warg Blood [AGI +20]', 0), myInnerHtml("EN748", '<input type="checkbox" name="A7_Skill48" onClick="A7(1)">Minor Stew [VIT +20]', 0), myInnerHtml("EN749", '<input type="checkbox" name="A7_Skill49" onClick="A7(1)">Siroma Iced Tea [INT +20]', 0), myInnerHtml("EN750", '<input type="checkbox" name="A7_Skill50" onClick="A7(1)">Drosera Herb Salad [DEX +20] (not saved via URL)', 0), myInnerHtml("EN751", '<input type="checkbox" name="A7_Skill51" onClick="A7(1)">Petite Tail Noodles [LUK +20] (not saved via URL)', 0), myInnerHtml("EN73", '<select name="A7_Skill3" onChange="A7(1)"></select> ', 0), myInnerHtml("EN74", '<select name="A7_Skill4" onChange="A7(1)"></select> ', 0), myInnerHtml("EN75", '<select name="A7_Skill5" onChange="A7(1)"></select> ', 0), myInnerHtml("EN76", '<select name="A7_Skill6" onChange="A7(1)"></select> ', 0), myInnerHtml("EN77", '<select name="A7_Skill7" onChange="A7(1)"></select> ', 0), myInnerHtml("EN78", '<select name="A7_Skill8" onChange="A7(1)"></select> ', 0), A7_Skill3.options[0] = new Option("STR+ food", 0), A7_Skill4.options[0] = new Option("AGI+ food", 0), A7_Skill5.options[0] = new Option("VIT+ food", 0), A7_Skill6.options[0] = new Option("INT+ food", 0), A7_Skill7.options[0] = new Option("DEX+ food", 0), A7_Skill8.options[0] = new Option("LUK+ food", 0), i = 1; 10 >= i; i++) A7_Skill3.options[i] = new Option("+" + i, i), A7_Skill4.options[i] = new Option("+" + i, i), A7_Skill5.options[i] = new Option("+" + i, i), A7_Skill6.options[i] = new Option("+" + i, i), A7_Skill7.options[i] = new Option("+" + i, i), A7_Skill8.options[i] = new Option("+" + i, i);
            A7_Skill39.options[0] = new Option("(No HP Increase Potion)", 0), A7_Skill39.options[1] = new Option("Small HP Increase Potion", 1), A7_Skill39.options[2] = new Option("Medium HP Increase Potion", 2), A7_Skill39.options[3] = new Option("Large HP Increase Potion", 3), A7_Skill40.options[0] = new Option("(No SP Increase Potion)", 0), A7_Skill40.options[1] = new Option("Small SP Increase Potion", 1), A7_Skill40.options[2] = new Option("Medium SP Increase Potion", 2), A7_Skill40.options[3] = new Option("Large SP Increase Potion", 3), A7_Skill42.options[0] = new Option("(No Mega Resist Potion)", 0), A7_Skill42.options[1] = new Option("Mega Resist Potion [+10% resistance to Blind, Bleeding, Confusion, Curse, Frozen, Poison, Silence, Sleep, Stun and Stone Curse status effects]", 1), A7_Skill42.options[2] = new Option("Mega Resist Potion (iRO) [+20% resistance to Blind, Bleeding, Confusion, Curse, Frozen, Poison, Silence, Sleep, Stun and Stone Curse status effects]", 2), SpeedPotName = ["(No Speed Potion)", "Concentration Potion", "Awakening Potion (Lv40)", "Berserk Potion"], A_SpeedPOT.options[0] = new Option(SpeedPotName[0], 0), A_SpeedPOT.options[1] = new Option(SpeedPotName[1], 1);
            for (var i = 2; 3 >= i; i++) A_SpeedPOT.options[2] = null;
            3 != n_A_JOB && 9 != n_A_JobClass2() && 16 != n_A_JobClass2() ? A_SpeedPOT.options[2] = new Option(SpeedPotName[2], 2) : A_SpeedPOT.options[2] = new Option("-", 0), 1 == n_A_JobClass() || 6 == n_A_JobClass() || 41 == n_A_JobClass() || 14 == n_A_JobClass2() || 11 == n_A_JobClass2() || 5 == n_A_JOB || 45 == n_A_JOB ? A_SpeedPOT.options[3] = new Option(SpeedPotName[3] + " (Lv85)", 3) : 22 == n_A_JOB ? A_SpeedPOT.options[3] = new Option("* Special (" + SkillOBJ[304][2] + " Lv85) / Poison Bottle", 3) : A_SpeedPOT.options[3] = new Option("* Special (" + SkillOBJ[304][2] + ") (Lv85)", 3), A7_Skill0.checked = n_A_Buf7[0], A7_Skill1.checked = n_A_Buf7[1], A7_Skill2.checked = n_A_Buf7[2], A7_Skill3.value = n_A_Buf7[3], A7_Skill4.value = n_A_Buf7[4], A7_Skill5.value = n_A_Buf7[5], A7_Skill6.value = n_A_Buf7[6], A7_Skill7.value = n_A_Buf7[7], A7_Skill8.value = n_A_Buf7[8], A7_Skill9.checked = n_A_Buf7[9], A7_Skill10.checked = n_A_Buf7[10], A7_Skill11.checked = n_A_Buf7[11], A7_Skill12.checked = n_A_Buf7[12], A7_Skill13.checked = n_A_Buf7[13], A7_Skill14.checked = n_A_Buf7[14], A7_Skill16.checked = n_A_Buf7[16], A7_Skill17.checked = n_A_Buf7[17], A7_Skill18.checked = n_A_Buf7[18], A7_Skill19.checked = n_A_Buf7[19], A7_Skill20.checked = n_A_Buf7[20], A7_Skill21.checked = n_A_Buf7[21], A7_Skill22.checked = n_A_Buf7[22], A7_Skill23.checked = n_A_Buf7[23], A7_Skill24.checked = n_A_Buf7[24], A7_Skill25.checked = n_A_Buf7[25], A7_Skill26.checked = n_A_Buf7[26], A7_Skill27.checked = n_A_Buf7[27], A7_Skill28.checked = n_A_Buf7[28], A7_Skill29.checked = n_A_Buf7[29], A7_Skill30.checked = n_A_Buf7[30], A7_Skill31.checked = n_A_Buf7[31], A7_Skill32.checked = n_A_Buf7[32], A7_Skill33.checked = n_A_Buf7[33], A7_Skill34.checked = n_A_Buf7[34], A_SpeedPOT.value = n_A_Buf7[35], A7_Skill36.checked = n_A_Buf7[36], A7_Skill37.checked = n_A_Buf7[37], A7_Skill38.checked = n_A_Buf7[38], A7_Skill39.value = n_A_Buf7[39], A7_Skill40.value = n_A_Buf7[40], A7_Skill41.checked = n_A_Buf7[41], A7_Skill42.value = n_A_Buf7[42], A7_Skill43.checked = n_A_Buf7[43], A7_Skill44.checked = n_A_Buf7[44], A7_Skill45.checked = n_A_Buf7[45], A7_Skill46.checked = n_A_Buf7[46], A7_Skill47.checked = n_A_Buf7[47], A7_Skill48.checked = n_A_Buf7[48], A7_Skill49.checked = n_A_Buf7[49], A7_Skill50.checked = n_A_Buf7[50], A7_Skill51.checked = n_A_Buf7[51]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="A7TD" class="subheader point" onclick="Buf7SW(1)">Food / Speed Potions / other Items <span id="A7used"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("SP_SIEN05", str, 0)
        }
        A7(0)
    }
}

function A7(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; e < n_A_Buf7.length; e++)
        if (0 != n_A_Buf7[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A7TD").style.backgroundColor = sBGC[i], myInnerHtml("A7used", "", 0)) : (document.getElementById("A7TD").style.backgroundColor = saBGC[i], myInnerHtml("A7used", " <B>[Active]</B>", 0))
}

function Buf8SW(v) {
    with(document.calcForm) {
        if (n_Skill8SW = v, n_Skill8SW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="A8TD" class="subheader point" onclick="Buf8SW(0)">Additional Effects <SPAN id="A8used"></SPAN>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD><Font size="2"><B>Pets</B></Font></TD></TR>', str += '<TR><TD id="EN800" class="dotB"></TD></TR>', str += '<TR><TD id="EN809" class="dotB"></TD></TR>', str += '<TR><TD><Font size="2"><B>EXP Items & Variables</B></Font></TD></TR>', str += '<TR><TD id="EN803" class="data"></TD></TR>', str += '<TR><TD id="EN804" class="data"></TD></TR>', str += '<TR><TD id="EN805" class="data"></TD></TR>', str += '<TR><TD id="EN806" class="data"></TD></TR>', str += '<TR><TD id="EN801" class="data"></TD></TR>', str += '<TR><TD id="EN802" class="data"></TD></TR>', str += "</TABLE>", myInnerHtml("ID_ETC", str, 0), myInnerHtml("EN800", '<select name="A8_Skill0" onChange="A8(1)"></select>', 0), i = 0; i < PET_OBJ_SORT.length; i++) {
                var n = PET_OBJ_SORT[i];
                A8_Skill0.options[i] = new Option(PET_OBJ[n][1], PET_OBJ[n][0])
            }
            for (myInnerHtml("EN801", 'Battle Manual <select name="A8_Skill1" onChange="A8(1)"></select>', 0), A8_Skill1.options[0] = new Option("None", 0), A8_Skill1.options[1] = new Option("25", 1), A8_Skill1.options[2] = new Option("50", 2), A8_Skill1.options[3] = new Option("100", 4), myInnerHtml("EN802", '<input type="checkbox" name="A8_Skill2" onClick="A8(1)">Job Manual 50', 0), myInnerHtml("EN803", 'Server Base Experience Rate <select name="A8_Skill3" onChange="A8(1)"></select>', 0), A8_Skill3.options[0] = new Option("1x", 0), i = 1; 199 >= i; i++) A8_Skill3.options[i] = new Option(1 + 1 * i + "x", i);
            for (myInnerHtml("EN804", 'Server Job Experience Rate <select name="A8_Skill7" onChange="A8(1)"></select>', 0), A8_Skill7.options[0] = new Option("1x", 0), i = 1; 199 >= i; i++) A8_Skill7.options[i] = new Option(1 + 1 * i + "x", i);
            for (myInnerHtml("EN805", 'Partymember Count <select name="A8_Skill5" onChange="A8(1)"></select>', 0), i = 0; 12 >= i; i++) A8_Skill5.options[i] = new Option(i, i);
            for (myInnerHtml("EN806", 'Exp Tap Bonus <select name="A8_Skill6" onChange="A8(1)"></select>', 0), A8_Skill6.options[0] = new Option("-", 0), i = 1; 20 >= i; i++) A8_Skill6.options[i] = new Option("+" + 25 * i + "%", i);
            for (str = '<Font size="2"><b>Temporary Card/Item active Effects </b></Font>', str += '<font color="red">(Duration and Chance are ignored)</font><BR>', str += '<select name="A8_Skill8" onChange="A8(1)" style="width:514px;"></select><BR>', str += '<select name="A8_Skill9" onChange="A8(1)" style="width:514px;"></select><BR>', str += '<select name="A8_Skill10" onChange="A8(1)" style="width:514px;"></select><BR>', str += '<select name="A8_Skill11" onChange="A8(1)" style="width:514px;"></select><BR>', myInnerHtml("EN809", str, 0), i = 0; i < ITEM_SP_TIME_OBJ_SORT.length; i++) {
                var n = ITEM_SP_TIME_OBJ_SORT[i];
                A8_Skill8.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] + " [" + ITEM_SP_TIME_OBJ[n][2] + "]", n), A8_Skill9.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] + " [" + ITEM_SP_TIME_OBJ[n][2] + "]", n), A8_Skill10.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] + " [" + ITEM_SP_TIME_OBJ[n][2] + "]", n), A8_Skill11.options[i] = new Option(ITEM_SP_TIME_OBJ[n][1] + " [" + ITEM_SP_TIME_OBJ[n][2] + "]", n)
            }
            A8_Skill0.value = n_A_Buf8[0], A8_Skill1.value = n_A_Buf8[1], A8_Skill2.checked = n_A_Buf8[2], A8_Skill3.value = n_A_Buf8[3], A8_Skill5.value = n_A_Buf8[5], A8_Skill6.value = n_A_Buf8[6], A8_Skill7.value = n_A_Buf8[7], A8_Skill8.value = n_A_Buf8[8], A8_Skill9.value = n_A_Buf8[9], A8_Skill10.value = n_A_Buf8[10], A8_Skill11.value = n_A_Buf8[11]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="A8TD" class="subheader point" onclick="Buf8SW(1)">Additional Effects <SPAN id="A8used"></SPAN>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("ID_ETC", str, 0)
        }
        A8(0)
    }
}

function A8(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; e < n_A_Buf8.length; e++)
        if (0 != n_A_Buf8[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A8TD").style.backgroundColor = sBGC[i], myInnerHtml("A8used", "", 0)) : (document.getElementById("A8TD").style.backgroundColor = saBGC[i], myInnerHtml("A8used", " <B>[Active]</B>", 0))
}

function Buf9SW(v) {
    with(document.calcForm) {
        if (n_Skill9SW = v, n_Skill9SW) {
            var str;
            for (str = '<table class="tborder manual">', str += '<TR><TD id="A9TD" Colspan="4" class="subheader point" onclick="Buf9SW(0)">Additional Enchants & Manual Edits on Player <SPAN id="A9used"></SPAN>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD colspan="4"><p class="center"><b>-- Anything here will ONLY be saved via Full Save --</b></p></TD></TR>', str += '<TR><TD colspan="4"><Font size="2"><B>Player Stats:</B></Font></TD></TR>', str += '<tr><TD id="EN965"></TD><TD id="EN967"></TD><TD id="EN969"></TD><td></td></tr>', str += '<tr><TD id="EN971"></TD><TD id="EN973"></TD><TD id="EN975"></TD><td></td></tr>', str += '<tr><TD id="EN931"></TD><TD id="EN933"></TD><TD id="EN935"></TD><TD id="EN937"></TD></tr>', str += '<tr><TD id="EN939"></TD><TD id="EN941"></TD><TD id="EN943"></TD><TD id="EN945"></TD></tr>', str += '<tr><TD id="EN951"></TD><TD id="EN953"></TD><TD id="EN947"></TD><TD id="EN949"></TD></tr>', str += '<tr><TD id="EN955"></TD><TD id="EN957"></TD><TD id="EN959"></TD><TD id="EN961"></TD></tr>', str += '<tr class="dotB"><TD id="EN963"></TD><td></td><td></td><td></td></tr>', str += '<TR><TD colspan="4"><Font size="2"><B>+% ATK based Damage:</B></Font></TD></TR>', str += '<tr><TD id="EN900" class="data"></TD><TD id="EN904" class="data"></TD><TD id="EN908" class="data"></TD><TD id="EN911"></TD></tr>', str += '<tr><TD id="EN901" class="data"></TD><TD id="EN905" class="data"></TD><TD id="EN909" class="data"></TD><TD id="EN912"></TD></tr>', str += '<tr><TD id="EN902" class="data"></TD><TD id="EN906" class="data"></TD><TD id="EN910" class="data"></TD><TD id="EN913"></TD></tr>', str += '<tr class="dotB"><TD id="EN903" class="data"></TD><TD id="EN907" class="data"></TD><td class="data"></td><TD id="EN914" class="data"></TD></tr>', str += '<TR><TD id="EN915" colspan="2"></TD><TD id="EN916" colspan="2"></TD></TR>', str += "</table>", myInnerHtml("ID_ARG", str, 0), i = 0; 10 > i; i++) myInnerHtml("EN90" + i, '+<input type="text" onkeyup="A9(1)" name="ARG_RC' + i + '" value="0" size="1" class="center">% vs<select name="A9_Skill' + i + '" onChange="A9(1)"></select>', 0);
            for (i = 10; 15 > i; i++) myInnerHtml("EN9" + i, '+<input type="text" onkeyup="A9(1)" name="ARG_RC' + i + '" value="0" size="1" class="center">% vs<select name="A9_Skill' + i + '" onChange="A9(1)"></select>', 0);
            for (i = 0; i < SyuzokuOBJ2.length; i++) document.calcForm.A9_Skill0.options[i] = new Option(SyuzokuOBJ2[i], i), document.calcForm.A9_Skill1.options[i] = new Option(SyuzokuOBJ2[i], i), document.calcForm.A9_Skill2.options[i] = new Option(SyuzokuOBJ2[i], i), document.calcForm.A9_Skill3.options[i] = new Option(SyuzokuOBJ2[i], i);
            for (i = 0; i < ZokuseiOBJ2.length; i++) document.calcForm.A9_Skill4.options[i] = new Option(ZokuseiOBJ2[i], i), document.calcForm.A9_Skill5.options[i] = new Option(ZokuseiOBJ2[i], i), document.calcForm.A9_Skill6.options[i] = new Option(ZokuseiOBJ2[i], i), document.calcForm.A9_Skill7.options[i] = new Option(ZokuseiOBJ2[i], i);
            for (i = 0; i < SizeOBJ.length; i++) document.calcForm.A9_Skill8.options[i] = new Option(SizeOBJ[i], i), document.calcForm.A9_Skill9.options[i] = new Option(SizeOBJ[i], i), document.calcForm.A9_Skill10.options[i] = new Option(SizeOBJ[i], i);
            for (i = 0; i < SpecialTypeOBJ.length; i++) document.calcForm.A9_Skill11.options[i] = new Option(SpecialTypeOBJ[i], i), document.calcForm.A9_Skill12.options[i] = new Option(SpecialTypeOBJ[i], i), document.calcForm.A9_Skill13.options[i] = new Option(SpecialTypeOBJ[i], i), document.calcForm.A9_Skill14.options[i] = new Option(SpecialTypeOBJ[i], i);
            myInnerHtml("EN915", '<Font size="2"><B>+<input type="text" onkeyup="A9(1)" name="ARG_RC26" value="0" size="1" class="center">% ATK based damage on any target.</B></Font>', 0), myInnerHtml("EN916", '<Font size="2"><B>+<input type="text" onkeyup="A9(1)" name="ARG_RC39" value="0" size="1" class="center">% MATK based damage on any target.</B></Font>', 0), myInnerHtml("EN931", '+<input type="text" onkeyup="A9(1)" name="ARG_RC15" value="0" size="1" class="center">MaxHP', 0), myInnerHtml("EN933", '+<input type="text" onkeyup="A9(1)" name="ARG_RC16" value="0" size="1" class="center">% MaxHP', 0), myInnerHtml("EN935", '+<input type="text" onkeyup="A9(1)" name="ARG_RC17" value="0" size="1" class="center">MaxSP', 0), myInnerHtml("EN937", '+<input type="text" onkeyup="A9(1)" name="ARG_RC18" value="0" size="1" class="center">% MaxSP', 0), myInnerHtml("EN939", '+<input type="text" onkeyup="A9(1)" name="ARG_RC19" value="0" size="1" class="center">DEF', 0), myInnerHtml("EN941", '+<input type="text" onkeyup="A9(1)" name="ARG_RC20" value="0" size="1" class="center">MDEF', 0), myInnerHtml("EN943", '+<input type="text" onkeyup="A9(1)" name="ARG_RC21" value="0" size="1" class="center">HIT', 0), myInnerHtml("EN945", '+<input type="text" onkeyup="A9(1)" name="ARG_RC22" value="0" size="1" class="center">FLEE', 0), myInnerHtml("EN947", '+<input type="text" onkeyup="A9(1)" name="ARG_RC23" value="0" size="1" class="center">Perfect Dodge', 0), myInnerHtml("EN949", '+<input type="text" onkeyup="A9(1)" name="ARG_RC24" value="0" size="1" class="center">Critical Rate', 0), myInnerHtml("EN951", '+<input type="text" onkeyup="A9(1)" name="ARG_RC25" value="0" size="1" class="center">ATK', 0), myInnerHtml("EN953", '+<input type="text" onkeyup="A9(1)" name="ARG_RC38" value="0" size="1" class="center">% ATK', 0), myInnerHtml("EN955", '+<input type="text" onkeyup="A9(1)" name="ARG_RC27" value="0" size="1" class="center">MATK', 0), myInnerHtml("EN957", '+<input type="text" onkeyup="A9(1)" name="ARG_RC28" value="0" size="1" class="center">% MATK', 0), myInnerHtml("EN959", '+<input type="text" onkeyup="A9(1)" name="ARG_RC29" value="0" size="1" class="center">% ASPD', 0), myInnerHtml("EN961", '+<input type="text" onkeyup="A9(1)" name="ARG_RC30" value="0" size="1" class="center">% HP Regen', 0), myInnerHtml("EN963", '+<input type="text" onkeyup="A9(1)" name="ARG_RC31" value="0" size="1" class="center">% SP Regen', 0), myInnerHtml("EN965", '+<input type="text" onkeyup="A9(1)" name="ARG_RC32" value="0" size="1" class="center">STR', 0), myInnerHtml("EN967", '+<input type="text" onkeyup="A9(1)" name="ARG_RC33" value="0" size="1" class="center">AGI', 0), myInnerHtml("EN969", '+<input type="text" onkeyup="A9(1)" name="ARG_RC34" value="0" size="1" class="center">VIT', 0), myInnerHtml("EN971", '+<input type="text" onkeyup="A9(1)" name="ARG_RC35" value="0" size="1" class="center">INT', 0), myInnerHtml("EN973", '+<input type="text" onkeyup="A9(1)" name="ARG_RC36" value="0" size="1" class="center">DEX', 0), myInnerHtml("EN975", '+<input type="text" onkeyup="A9(1)" name="ARG_RC37" value="0" size="1" class="center">LUK', 0), A9_Skill0.value = n_A_Buf9[0], ARG_RC0.value = n_A_Buf9[1], A9_Skill1.value = n_A_Buf9[2], ARG_RC1.value = n_A_Buf9[3], A9_Skill2.value = n_A_Buf9[4], ARG_RC2.value = n_A_Buf9[5], A9_Skill3.value = n_A_Buf9[6], ARG_RC3.value = n_A_Buf9[7], A9_Skill4.value = n_A_Buf9[8], ARG_RC4.value = n_A_Buf9[9], A9_Skill5.value = n_A_Buf9[10], ARG_RC5.value = n_A_Buf9[11], A9_Skill6.value = n_A_Buf9[12], ARG_RC6.value = n_A_Buf9[13], A9_Skill7.value = n_A_Buf9[14], ARG_RC7.value = n_A_Buf9[15], A9_Skill8.value = n_A_Buf9[16], ARG_RC8.value = n_A_Buf9[17], A9_Skill9.value = n_A_Buf9[18], ARG_RC9.value = n_A_Buf9[19], A9_Skill10.value = n_A_Buf9[20], ARG_RC10.value = n_A_Buf9[21], A9_Skill11.value = n_A_Buf9[22], ARG_RC11.value = n_A_Buf9[23], A9_Skill12.value = n_A_Buf9[24], ARG_RC12.value = n_A_Buf9[25], A9_Skill13.value = n_A_Buf9[26], ARG_RC13.value = n_A_Buf9[27], A9_Skill14.value = n_A_Buf9[28], ARG_RC14.value = n_A_Buf9[29], ARG_RC15.value = n_A_Buf9[30], ARG_RC16.value = n_A_Buf9[31], ARG_RC17.value = n_A_Buf9[32], ARG_RC18.value = n_A_Buf9[33], ARG_RC19.value = n_A_Buf9[34], ARG_RC20.value = n_A_Buf9[35], ARG_RC21.value = n_A_Buf9[36], ARG_RC22.value = n_A_Buf9[37], ARG_RC23.value = n_A_Buf9[38], ARG_RC24.value = n_A_Buf9[39], ARG_RC25.value = n_A_Buf9[40], ARG_RC26.value = n_A_Buf9[41], ARG_RC27.value = n_A_Buf9[42], ARG_RC28.value = n_A_Buf9[43], ARG_RC29.value = n_A_Buf9[44], ARG_RC30.value = n_A_Buf9[45], ARG_RC31.value = n_A_Buf9[46], ARG_RC32.value = n_A_Buf9[47], ARG_RC33.value = n_A_Buf9[48], ARG_RC34.value = n_A_Buf9[49], ARG_RC35.value = n_A_Buf9[50], ARG_RC36.value = n_A_Buf9[51], ARG_RC37.value = n_A_Buf9[52], ARG_RC38.value = n_A_Buf9[53], ARG_RC39.value = n_A_Buf9[54]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="A9TD" class="subheader point" onclick="Buf9SW(1)">Additional Enchants & Manual Edits on Player <SPAN id="A9used"></SPAN>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("ID_ARG", str, 0)
        }
        A9(0)
    }
}

function A9(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; e < n_A_Buf9.length; e++)
        if (0 != n_A_Buf9[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A9TD").style.backgroundColor = sBGC[i], myInnerHtml("A9used", "", 0)) : (document.getElementById("A9TD").style.backgroundColor = saBGC[i], myInnerHtml("A9used", " <B>[Active]</B>", 0))
}

function Buf10SW(v) {
    with(document.calcForm) {
        if (n_Skill10SW = v, n_Skill10SW) {
            var str;
            for (str = '<table class="tborderA manual">', str += '<TR><TD id="A10TD" Colspan="4" class="subheader point" onclick="Buf10SW(0)">Manual Edits on Enemy <SPAN id="A10used"></SPAN>', str += '<div class="right">(click to hide)</div></TD></TR>', str += "<TR><TD colspan='4'><Font size='2'><B>Enemy's Stats:</B></Font></TD></TR>", str += '<tr><TD id="EN1065"></TD><TD id="EN1067"></TD><TD id="EN1069"></TD><td></td></tr>', str += '<tr><TD id="EN1071"></TD><TD id="EN1073"></TD><TD id="EN1075"></TD><td></td></tr>', str += '<tr><TD id="EN1031"></TD><TD id="EN1033"></TD><TD id="EN1035"></TD><TD id="EN1037"></TD></tr>', str += '<tr><TD id="EN1039"></TD><TD id="EN1041"></TD><TD id="EN1043"></TD><TD id="EN1045"></TD></tr>', str += '<tr><TD id="EN1051"></TD><TD id="EN1053"></TD><TD id="EN1047"></TD><TD id="EN1049"></TD></tr>', str += '<tr><TD id="EN1055"></TD><TD id="EN1057"></TD><TD id="EN1059"></TD><TD id="EN1061"></TD></tr>', str += '<tr class="dotB"><TD id="EN1063"></TD><td></td><td></td><td></td></tr>', str += "<TR><TD colspan='4'><Font size='2'><B>Enemy's Damage Reductions:</B></Font></TD></TR>", str += '<tr><TD id="EN1001"></TD><TD id="EN1000"></TD><TD id="EN1004"></TD><TD id="EN1011"></TD></tr>', str += '<tr><TD id="EN1002"></TD><TD id="EN1003"></TD><TD id="EN1010"></TD><TD id="EN1012"></TD></tr>', str += '<tr><TD id="EN1008"></TD><TD id="EN1006"></TD><TD id="EN1009"></TD><TD id="EN1013"></TD></tr>', str += '<tr class="dotB"><TD id="EN1005"></TD><TD id="EN1007"></TD><td></td><TD id="EN1014"></TD></tr>', str += '<TR class="dotB"><TD id="EN1015" colspan="2"></TD><TD id="EN1016" colspan="2"></TD></TR>', str += "</table>", myInnerHtml("B_MANUAL", str, 0), myInnerHtml("EN1000", '+<input type="text" onkeyup="A10(1)" name="BRG_RC0" value="0" size="1" class="center">% Racial Resistance', 0), myInnerHtml("EN1001", '+<input type="text" onkeyup="A10(1)" name="BRG_RC1" value="0" size="1" class="center">vs<select name="Bman1" onChange="A10(1)"></select>', 0), myInnerHtml("EN1002", '+<input type="text" onkeyup="A10(1)" name="BRG_RC2" value="0" size="1" class="center">vs<select name="Bman2" onChange="A10(1)"></select>', 0), myInnerHtml("EN1003", '+<input type="text" onkeyup="A10(1)" name="BRG_RC3" value="0" size="1" class="center">% Size Resistance', 0), myInnerHtml("EN1004", '+<input type="text" onkeyup="A10(1)" name="BRG_RC4" value="0" size="1" class="center">% Long-range Resistance', 0), myInnerHtml("EN1010", '+<input type="text" onkeyup="A10(1)" name="BRG_RC10" value="0" size="1" class="center">% Additional Reflect (Equip/Card)', 0), myInnerHtml("EN1011", "", 0), myInnerHtml("EN1012", "", 0), myInnerHtml("EN1013", "", 0), myInnerHtml("EN1014", "", 0), i = 0; 10 >= i; i++) document.calcForm.Bman1.options[i] = new Option(ZokuseiOBJ2[i], i), document.calcForm.Bman2.options[i] = new Option(ZokuseiOBJ2[i], i);
            for (myInnerHtml("EN1015", '<Font size="2"><B>+<input type="text" onkeyup="A10(1)" name="BRG_RC26" value="0" size="1" class="center">% ATK based damage on any target.</B></Font>', 0), myInnerHtml("EN1016", '<Font size="2"><B>+<input type="text" onkeyup="A10(1)" name="BRG_RC39" value="0" size="1" class="center">% MATK based damage on any target.</B></Font>', 0), myInnerHtml("EN1031", '+<input type="text" onkeyup="A10(1)" name="BRG_RC15" value="0" size="1" class="center">MaxHP', 0), myInnerHtml("EN1033", '+<input type="text" onkeyup="A10(1)" name="BRG_RC16" value="0" size="1" class="center">% MaxHP', 0), myInnerHtml("EN1039", '+<input type="text" onkeyup="A10(1)" name="BRG_RC19" value="0" size="1" class="center">DEF', 0), myInnerHtml("EN1041", '+<input type="text" onkeyup="A10(1)" name="BRG_RC20" value="0" size="1" class="center">MDEF', 0), myInnerHtml("EN1043", '+<input type="text" onkeyup="A10(1)" name="BRG_RC21" value="0" size="1" class="center">HIT', 0), myInnerHtml("EN1045", '+<input type="text" onkeyup="A10(1)" name="BRG_RC22" value="0" size="1" class="center">FLEE', 0), myInnerHtml("EN1047", '+<input type="text" onkeyup="A10(1)" name="BRG_RC23" value="0" size="1" class="center" disabled><S>Perfect Dodge</S>', 0), myInnerHtml("EN1049", '+<input type="text" onkeyup="A10(1)" name="BRG_RC24" value="0" size="1" class="center">Critical Rate', 0), myInnerHtml("EN1051", '+<input type="text" onkeyup="A10(1)" name="BRG_RC25" value="0" size="1" class="center">ATK', 0), myInnerHtml("EN1053", '+<input type="text" onkeyup="A10(1)" name="BRG_RC38" value="0" size="1" class="center">% ATK', 0), myInnerHtml("EN1055", '+<input type="text" onkeyup="A10(1)" name="BRG_RC27" value="0" size="1" class="center">MATK', 0), myInnerHtml("EN1057", '+<input type="text" onkeyup="A10(1)" name="BRG_RC28" value="0" size="1" class="center">% MATK', 0), myInnerHtml("EN1059", '+<input type="text" onkeyup="A10(1)" name="BRG_RC29" value="0" size="1" class="center" disabled><S>% ASPD</S>', 0), myInnerHtml("EN1061", "", 0), myInnerHtml("EN1063", "", 0), myInnerHtml("EN1065", "", 0), myInnerHtml("EN1067", 'AGI +<select name="BRG_RC33" onChange="A10(1)"></select>', 0), myInnerHtml("EN1069", 'VIT +<select name="BRG_RC34" onChange="A10(1)"></select>', 0), myInnerHtml("EN1071", 'INT +<select name="BRG_RC35" onChange="A10(1)"></select>', 0), myInnerHtml("EN1073", 'DEX +<select name="BRG_RC36" onChange="A10(1)"></select>', 0), myInnerHtml("EN1075", 'LUK +<select name="BRG_RC37" onChange="A10(1)"></select>', 0), i = 0; 200 >= i; i++) BRG_RC33.options[i] = new Option(i, i), BRG_RC34.options[i] = new Option(i, i), BRG_RC35.options[i] = new Option(i, i), BRG_RC36.options[i] = new Option(i, i), BRG_RC37.options[i] = new Option(i, i);
            BRG_RC0.value = n_B_manual[1], Bman1.value = n_B_manual[2], BRG_RC1.value = n_B_manual[3], Bman2.value = n_B_manual[4], BRG_RC2.value = n_B_manual[5], BRG_RC3.value = n_B_manual[7], BRG_RC4.value = n_B_manual[9], BRG_RC10.value = n_B_manual[21], BRG_RC15.value = n_B_manual[30], BRG_RC16.value = n_B_manual[31], BRG_RC19.value = n_B_manual[34], BRG_RC20.value = n_B_manual[35], BRG_RC21.value = n_B_manual[36], BRG_RC22.value = n_B_manual[37], BRG_RC23.value = n_B_manual[38], BRG_RC24.value = n_B_manual[39], BRG_RC25.value = n_B_manual[40], BRG_RC26.value = n_B_manual[41], BRG_RC27.value = n_B_manual[42], BRG_RC28.value = n_B_manual[43], BRG_RC29.value = n_B_manual[44], BRG_RC33.value = n_B_manual[48], BRG_RC34.value = n_B_manual[49], BRG_RC35.value = n_B_manual[50], BRG_RC36.value = n_B_manual[51], BRG_RC37.value = n_B_manual[52], BRG_RC38.value = n_B_manual[53], BRG_RC39.value = n_B_manual[54]
        } else {
            var str;
            str = '<table class="tborderA">', str += '<TR><TD id="A10TD" class="subheader point" onclick="Buf10SW(1)">Manual Edits on Enemy <SPAN id="A10used"></SPAN>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("B_MANUAL", str, 0)
        }
        A10(0)
    }
}

function A10(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; e < n_B_manual.length; e++)
        if (0 != n_B_manual[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("A10TD").style.backgroundColor = sBGC[i], myInnerHtml("A10used", "", 0)) : (document.getElementById("A10TD").style.backgroundColor = saBGC[i], myInnerHtml("A10used", " <B>[Active]</B>", 0))
}

function IjyouSW(v) {
    with(document.calcForm) {
        if (n_IjyouSW = v, n_IjyouSW) {
            var str;
            if (str = '<table class="tborder">', str += '<TR><TD id="AITD" ColSpan="4" class="subheader point" onClick="IjyouSW(0)">Debuffs on Enemy <span id="AIused"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD id="BI0_1" class="center">Provoke (Non Undead)</TD><TD id="BI0_2" class="data"></TD><TD id="BI2_1" class="center">Poison</TD><TD id="BI2_2"></TD></TR>', str += '<TR><TD id="BI4_1" class="center">Frozen (Non Undead)</TD><TD id="BI4_2" class="data"></TD><TD id="BI3_1" class="center">Blind</TD><TD id="BI3_2"></TD></TR>', str += '<TR><TD id="BI5_1" class="center">Blessing (Demon/Undead)</TD><TD id="BI5_2" class="data"></TD><TD id="BI10_1" class="center">Curse</TD><TD id="BI10_2"></TD></TR>', str += '<TR><TD id="BI6_1" class="center">Lex Aeterna</TD><TD id="BI6_2" class="data"></TD><TD id="BI7_1" class="center">Stun</TD><TD id="BI7_2"></TD></TR>', str += '<TR><TD id="BI11_1" class="center">Agility Down</TD><TD id="BI11_2" class="data"></TD><TD id="BI9_1" class="center">Stone</TD><TD id="BI9_2"></TD></TR>', str += '<TR><TD id="BI1_1" class="center">Quagmire</TD><TD id="BI1_2" class="data"></TD><TD id="BI8_1" class="center">Sleep</TD><TD id="BI8_2"></TD></TR>', str += '<TR><TD id="BI12_1" class="center">Signum Crucis</TD><TD id="BI12_2" class="data"></TD><TD id="BI19_1" class="center">Slow Grace</TD><TD id="BI19_2"></TD></TR>', str += '<TR><TD id="BI24_1" class="center">Flying</TD><TD id="BI24_2" class="data"></TD><TD id="BI20_1" class="center">Down Tempo</TD><TD id="BI20_2"></TD></TR>', str += '<TR class="dotB"><TD id="BI18_1" class="center">Mind Breaker</TD><TD id="BI18_2" class="data"></TD><TD id="BI17_1" class="center">Spider Web</TD><TD id="BI17_2"></TD></TR>', 0 == Taijin && (str += '<TR><TD class="center" ColSpan="4"><b>Monster Exclusive Debuffs</b></TD></TR>', str += '<TR><TD id="BI13_1" class="center">Strip Weapon</TD><TD id="BI13_2" class="data"></TD><TD id="BI14_1" class="center">Strip Shield</TD><TD id="BI14_2"></TD></TR>', str += '<TR><TD id="BI15_1" class="center">Strip Armor</TD><TD id="BI15_2" class="data"></TD><TD id="BI16_1" class="center">Strip Helm</TD><TD id="BI16_2"></TD></TR>', str += '<TR><TD id="BI21_1" class="center">Eska</TD><TD id="BI21_2" class="data"></TD><TD id="BI22_1" class="center">Eske</TD><TD id="BI22_2"></TD></TR>', str += '<TR><TD id="BI23_1" class="center" colspan="2">Elemental Change (Sage Skill)</TD><TD id="BI23_2" colspan="2"></TD></TR>'), str += "</TABLE>", myInnerHtml("MONSTER_IJYOU", str, 0), myInnerHtml("BI0_2", '<select name="B_IJYOU0" onChange="AI(1)"></select>', 0), myInnerHtml("BI1_2", '<select name="B_IJYOU1" onChange="AI(1)"></select>', 0), myInnerHtml("BI2_2", '<input type="checkbox" name="B_IJYOU2" onClick="AI(1)">', 0), myInnerHtml("BI3_2", '<input type="checkbox" name="B_IJYOU3" onClick="AI(1)">', 0), myInnerHtml("BI4_2", '<input type="checkbox" name="B_IJYOU4" onClick="AI(1)">', 0), myInnerHtml("BI5_2", '<input type="checkbox" name="B_IJYOU5" onClick="AI(1)">', 0), myInnerHtml("BI6_2", '<input type="checkbox" name="B_IJYOU6" onClick="AI(1)">', 0), myInnerHtml("BI7_2", '<input type="checkbox" name="B_IJYOU7" onClick="AI(1)">', 0), myInnerHtml("BI8_2", '<input type="checkbox" name="B_IJYOU8" onClick="AI(1)">', 0), myInnerHtml("BI9_2", '<input type="checkbox" name="B_IJYOU9" onClick="AI(1)">', 0), myInnerHtml("BI10_2", '<input type="checkbox" name="B_IJYOU10" onClick="AI(1)">', 0), myInnerHtml("BI11_2", '<select name="B_IJYOU11" onChange="AI(1)"></select>', 0), myInnerHtml("BI12_2", '<select name="B_IJYOU12" onChange="AI(1)"></select>', 0), myInnerHtml("BI17_2", '<input type="checkbox" name="B_IJYOU17" onClick="AI(1)">', 0), myInnerHtml("BI18_2", '<select name="B_IJYOU18" onChange="AI(1)"></select>', 0), myInnerHtml("BI19_2", '<input type="checkbox" name="B_IJYOU19" onClick="AI(1)">', 0), myInnerHtml("BI20_2", '<input type="checkbox" name="B_IJYOU20" onClick="AI(1)">', 0), myInnerHtml("BI24_2", '<select name="B_IJYOU24" onChange="AI(1)"></select>', 0), 0 == Taijin) {
                myInnerHtml("BI13_2", '<input type="checkbox" name="B_IJYOU13" onClick="AI(1)">', 0), myInnerHtml("BI14_2", '<input type="checkbox" name="B_IJYOU14" onClick="AI(1)">', 0), myInnerHtml("BI15_2", '<input type="checkbox" name="B_IJYOU15" onClick="AI(1)">', 0), myInnerHtml("BI16_2", '<input type="checkbox" name="B_IJYOU16" onClick="AI(1)">', 0), myInnerHtml("BI21_2", '<input type="checkbox" name="B_IJYOU21" onClick="AI(1)">', 0), myInnerHtml("BI22_2", '<input type="checkbox" name="B_IJYOU22" onClick="AI(1)">', 0), myInnerHtml("BI23_2", '<select name="B_IJYOU23" onChange="AI(1)"></select>', 0);
                var ZoHe2 = ["None", "Water", "Earth", "Fire", "Wind"];
                for (i = 0; 4 >= i; i++) B_IJYOU23.options[i] = new Option(ZoHe2[i], i)
            }
            for (i = 0; 10 >= i; i++) B_IJYOU0.options[i] = new Option(i, i), B_IJYOU11.options[i] = new Option(i, i), B_IJYOU12.options[i] = new Option(i, i);
            for (i = 0; 5 >= i; i++) B_IJYOU1.options[i] = new Option(i, i), B_IJYOU18.options[i] = new Option(i, i), B_IJYOU24.options[i] = new Option(i, i);
            B_IJYOU0.value = n_B_IJYOU[0], B_IJYOU1.value = n_B_IJYOU[1], B_IJYOU2.checked = n_B_IJYOU[2], B_IJYOU3.checked = n_B_IJYOU[3], B_IJYOU4.checked = n_B_IJYOU[4], B_IJYOU5.checked = n_B_IJYOU[5], B_IJYOU6.checked = n_B_IJYOU[6], B_IJYOU7.checked = n_B_IJYOU[7], B_IJYOU8.checked = n_B_IJYOU[8], B_IJYOU9.checked = n_B_IJYOU[9], B_IJYOU10.checked = n_B_IJYOU[10], B_IJYOU11.value = n_B_IJYOU[11], B_IJYOU12.value = n_B_IJYOU[12], B_IJYOU17.checked = n_B_IJYOU[17], B_IJYOU18.value = n_B_IJYOU[18], B_IJYOU19.checked = n_B_IJYOU[19], B_IJYOU20.checked = n_B_IJYOU[20], 0 == Taijin && (B_IJYOU13.checked = n_B_IJYOU[13], B_IJYOU14.checked = n_B_IJYOU[14], B_IJYOU15.checked = n_B_IJYOU[15], B_IJYOU16.checked = n_B_IJYOU[16], B_IJYOU21.checked = n_B_IJYOU[21], B_IJYOU22.checked = n_B_IJYOU[22], B_IJYOU23.value = n_B_IJYOU[23]), B_IJYOU24.value = n_B_IJYOU[24]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="AITD" class="subheader point" onClick="IjyouSW(1)">Debuffs on Enemy <span id="AIused"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("MONSTER_IJYOU", str, 0)
        }
        AI(0)
    }
}

function AI(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; 24 >= e; e++)
        if (0 != n_B_IJYOU[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("AITD").style.backgroundColor = sBGC[i], myInnerHtml("AIused", "", 0)) : (document.getElementById("AITD").style.backgroundColor = saBGC[i], myInnerHtml("AIused", " <B>[Active]</B>", 0))
}

function EnemyKyoukaSW(v) {
    with(document.calcForm) {
        if (n_KyoukaSW = v, n_KyoukaSW) {
            var str;
            for (str = '<table class="tborder">', str += '<TR><TD id="AKTD" colspan="4" class="subheader point" onClick="EnemyKyoukaSW(0)">Buffs on Enemy <span id="AKused"></span>', str += '<div class="right">(click to hide)</div></TD></TR>', str += '<TR><TD class="center">Increase AGI</TD><TD id="ID_Kb0" class="data"></TD><TD class="center">Assumptio</TD><TD id="ID_Kb1"></TD></TR>', str += '<TR><TD class="center">Angelus</TD><TD id="ID_Kb10" class="data"></TD><TD class="center">Maximize Power</TD><TD id="ID_Kb3"></TD></TR>', str += '<TR><TD class="center">Auto Guard</TD><TD id="ID_Kb11" class="data"></TD><TD class="center">Adrenaline Rush</TD><TD id="ID_Kb2"></TD></TR>', str += '<TR><TD class="center">Shield Reflect</TD><TD id="ID_Kb12" class="data"></TD><TD class="center">Defending Aura</TD><TD id="ID_Kb13"></TD></TR>', str += '<TR><TD class="center dotB">Energy Coat</TD><TD id="ID_Kb14" class="data dotB" colspan="3"></TD></TR>', str += '<TR><TD class="center" colspan="4"><b>Monster Exclusive buffs</b></TD></TR>', str += '<TR><TD class="center">Attrib. Change</TD><TD id="ID_Kb6" class="data"></TD><TD class="center">Stone Skin</TD><TD id="ID_Kb7"></TD></TR>', str += '<TR><TD class="center">Keeping</TD><TD id="ID_Kb9" class="data"></TD><TD class="center">Magic Mirror</TD><TD id="ID_Kb8"></TD></TR>', str += '<TR><TD class="center"></TD><TD id="ID_Kb15" class="data"></TD><TD class="center">Agi Up (AGI x3)</TD><TD id="ID_Kb5"></TD></TR>', str += '<TR><TD class="center" colspan="2">POWER UP (ATK x3, HIT x2)</TD><TD id="ID_Kb4" colspan="2"></TD></TR>', str += "</TABLE>", myInnerHtml("MONSTER_KYOUKA", str, 0), myInnerHtml("ID_Kb0", '<select name="B_KYOUKA0" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb1", '<input type="checkbox" name="B_KYOUKA1" onClick="AK(1)">', 0), myInnerHtml("ID_Kb2", '<input type="checkbox" name="B_KYOUKA2" onClick="AK(1)">', 0), myInnerHtml("ID_Kb3", '<input type="checkbox" name="B_KYOUKA3" onClick="AK(1)">', 0), myInnerHtml("ID_Kb10", '<select name="B_buff10" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb11", '<select name="B_buff11" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb12", '<select name="B_buff12" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb13", '<select name="B_buff13" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb14", '<select name="B_buff14" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb4", '<input type="checkbox" name="B_KYOUKA4" onClick="AK(1)">', 0), myInnerHtml("ID_Kb5", '<select name="B_KYOUKA5" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb6", '<select name="B_KYOUKA6" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb7", '<select name="B_KYOUKA7" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb8", '<select name="B_KYOUKA8" onChange="AK(1)"></select>', 0), myInnerHtml("ID_Kb9", '<input type="checkbox" name="B_KYOUKA9" onClick="AK(1)">', 0), i = 0; 10 >= i; i++) B_KYOUKA0.options[i] = new Option(i, i), B_buff10.options[i] = new Option(i, i), B_buff11.options[i] = new Option(i, i), B_buff12.options[i] = new Option(i, i);
            for (i = 0; 5 >= i; i++) B_KYOUKA5.options[i] = new Option(i, i), B_KYOUKA7.options[i] = new Option(i, i), B_KYOUKA8.options[i] = new Option(i, i), B_buff13.options[i] = new Option(i, i), B_buff14.options[i] = new Option(EnergyCoatOBJ[i], i);
            var ZoHe = [
                ["None", "Neutral 1", "Neutral 2", "Neutral 3", "Neutral 4", "Water 1", "Water 2", "Water 3", "Water 4", "Earth 1", "Earth 2", "Earth 3", "Earth 4", "Fire 1", "Fire 2", "Fire 3", "Fire 4", "Wind 1", "Wind 2", "Wind 3", "Wind 4", "Poison 1", "Poison 2", "Poison 3", "Poison 4", "Holy 1", "Holy 2", "Holy 3", "Holy 4", "Shadow 1", "Shadow 2", "Shadow 3", "Shadow 4", "Ghost 1", "Ghost 2", "Ghost 3", "Ghost 4", "Undead 1", "Undead 2", "Undead 3", "Undead 4"],
                [0, 1, 2, 3, 4, 11, 12, 13, 14, 21, 22, 23, 24, 31, 32, 33, 34, 41, 42, 43, 44, 51, 52, 53, 54, 61, 62, 63, 64, 71, 72, 73, 74, 81, 82, 83, 84, 91, 92, 93, 94]
            ];
            for (i = 0; 40 >= i; i++) B_KYOUKA6.options[i] = new Option(ZoHe[0][i], ZoHe[1][i]);
            B_KYOUKA0.value = n_B_KYOUKA[0], B_KYOUKA1.checked = n_B_KYOUKA[1], B_KYOUKA2.checked = n_B_KYOUKA[2], B_KYOUKA3.checked = n_B_KYOUKA[3], B_KYOUKA4.checked = n_B_KYOUKA[4], B_KYOUKA5.value = n_B_KYOUKA[5], B_KYOUKA6.value = n_B_KYOUKA[6], B_KYOUKA7.value = n_B_KYOUKA[7], B_KYOUKA8.value = n_B_KYOUKA[8], B_KYOUKA9.checked = n_B_KYOUKA[9], B_buff10.value = n_B_KYOUKA[10], B_buff11.value = n_B_KYOUKA[11], B_buff12.value = n_B_KYOUKA[12], B_buff13.value = n_B_KYOUKA[13], B_buff14.value = n_B_KYOUKA[14]
        } else {
            var str;
            str = '<table class="tborder">', str += '<TR><TD id="AKTD" class="subheader point" onClick="EnemyKyoukaSW(1)">Buffs on Enemy <span id="AKused"></span>', str += '<div class="right">(click to show)</div></TD></TR></TABLE>', myInnerHtml("MONSTER_KYOUKA", str, 0)
        }
        AK(0)
    }
}

function AK(_) {
    1 == _ && calc();
    for (var n = 0, e = 0; 14 >= e; e++)
        if (0 != n_B_KYOUKA[e]) {
            n = 1;
            break
        }
    var i = document.getElementById("theme").value;
    0 == n ? (document.getElementById("AKTD").style.backgroundColor = sBGC[i], myInnerHtml("AKused", "", 0)) : (document.getElementById("AKTD").style.backgroundColor = saBGC[i], myInnerHtml("AKused", " <B>[Active]</B>", 0))
}

function ClickB_Enemy() {
    with(document.calcForm) {
        for (n_B = new Array, n_B2 = new Array, i = 0; 22 >= i; i++) n_B[i] = MonsterOBJ[B_Enemy.value][i], n_B2[i] = n_B[i];
        586 == n_B[0] ? Taijin = 1 : Taijin = 0, n_B[6] += n_B_manual[30], n_B[6] += Math.floor(n_B[6] * n_B_manual[31] / 100), n_B[7] += n_B_manual[49], n_B[8] += n_B_manual[48], n_B[9] += n_B_manual[50], n_B[10] += n_B_manual[51], n_B[11] += n_B_manual[52], n_B[12] += n_B_manual[40], n_B[12] += Math.floor(n_B[12] * n_B_manual[53] / 100), n_B[13] += n_B_manual[40], n_B[13] += Math.floor(n_B[13] * n_B_manual[53] / 100), n_B[14] += n_B_manual[34], n_B[15] += n_B_manual[35], 586 == n_B[0] ? (n_B[23] = Math.floor(.5 * n_B[7]) + Math.floor(.3 * n_B[7]), n_B[24] = Math.floor(.5 * n_B[7]) + Math.floor(n_B[7] * n_B[7] / 150) - 1, n_B[23] > n_B[24] && (n_B[24] = n_B[23])) : (n_B2[23] = n_B[7], n_B2[24] = n_B[7] + (Math.floor(n_B[7] / 20) * Math.floor(n_B[7] / 20) - 1), n_B2[23] > n_B2[24] && (n_B2[24] = n_B2[23])), myInnerHtml("B_6", n_B[6], 0), myInnerHtml("B_16", n_B[16], 0), myInnerHtml("B_12", n_B[12], 0), myInnerHtml("B_13", n_B[13], 0), myInnerHtml("B_17", n_B[17], 0), myInnerHtml("B_14", n_B[14], 0), myInnerHtml("B_23", n_B[23], 0), myInnerHtml("B_15", n_B[15], 0), myInnerHtml("B_vit", n_B[7], 0), myInnerHtml("B_agi", n_B[8], 0), myInnerHtml("B_int", n_B[9], 0), myInnerHtml("B_dex", n_B[10], 0), myInnerHtml("B_luk", n_B[11], 0), Taijin ? (n_B[23] = Math.floor(.5 * n_B[7]) + Math.floor(.3 * n_B[7]), n_B[24] = Math.floor(.5 * n_B[7]) + Math.floor(n_B[7] * n_B[7] / 150) - 1, n_B[23] > n_B[24] && (n_B[24] = n_B[23])) : (n_B2[23] = n_B[7], n_B2[24] = n_B[7] + (Math.floor(n_B[7] / 20) * Math.floor(n_B[7] / 20) - 1), n_B2[23] > n_B2[24] && (n_B2[24] = n_B2[23])), n_B2[25] = Math.floor(n_B[7] / 2) + n_B[9], n_B2[26] = n_B[5] + n_B[10], n_B2[27] = n_B[5] + n_B[8], n_IjyouSW && (n_B_IJYOU[0] = eval(B_IJYOU0.value), n_B_IJYOU[1] = eval(B_IJYOU1.value), n_B_IJYOU[2] = B_IJYOU2.checked, n_B_IJYOU[3] = B_IJYOU3.checked, n_B_IJYOU[4] = B_IJYOU4.checked, n_B_IJYOU[5] = B_IJYOU5.checked, n_B_IJYOU[6] = B_IJYOU6.checked, n_B_IJYOU[7] = B_IJYOU7.checked, n_B_IJYOU[8] = B_IJYOU8.checked, n_B_IJYOU[9] = B_IJYOU9.checked, n_B_IJYOU[10] = B_IJYOU10.checked, n_B_IJYOU[11] = eval(B_IJYOU11.value), n_B_IJYOU[12] = eval(B_IJYOU12.value), n_B_IJYOU[17] = B_IJYOU17.checked, n_B_IJYOU[18] = eval(B_IJYOU18.value), n_B_IJYOU[19] = B_IJYOU19.checked, n_B_IJYOU[20] = B_IJYOU20.checked, n_B_IJYOU[24] = eval(B_IJYOU24.value), 0 == Taijin && (n_B_IJYOU[13] = B_IJYOU13.checked, n_B_IJYOU[14] = B_IJYOU14.checked, n_B_IJYOU[15] = B_IJYOU15.checked, n_B_IJYOU[16] = B_IJYOU16.checked, n_B_IJYOU[21] = B_IJYOU21.checked, n_B_IJYOU[22] = B_IJYOU22.checked, n_B_IJYOU[23] = eval(B_IJYOU23.value))), n_KyoukaSW && (n_B_KYOUKA[0] = eval(B_KYOUKA0.value), n_B_KYOUKA[1] = B_KYOUKA1.checked, n_B_KYOUKA[2] = B_KYOUKA2.checked, n_B_KYOUKA[3] = B_KYOUKA3.checked, n_B_KYOUKA[4] = B_KYOUKA4.checked, n_B_KYOUKA[5] = eval(B_KYOUKA5.value), n_B_KYOUKA[6] = eval(B_KYOUKA6.value), n_B_KYOUKA[7] = eval(B_KYOUKA7.value), n_B_KYOUKA[8] = eval(B_KYOUKA8.value), n_B_KYOUKA[9] = B_KYOUKA9.checked, n_B_KYOUKA[10] = eval(B_buff10.value), n_B_KYOUKA[11] = eval(B_buff11.value), n_B_KYOUKA[12] = eval(B_buff12.value), n_B_KYOUKA[13] = eval(B_buff13.value), n_B_KYOUKA[14] = eval(B_buff14.value)), n_B_KYOUKA[6] && (n_B[3] = n_B_KYOUKA[6]), n_B_IJYOU[23] && (n_B[3] = 10 * n_B_IJYOU[23] + n_B[3] % 10), 0 == n_B[19] && n_B[3] < 90 && n_B_IJYOU[4] && (n_B[3] = 11), 0 == n_B[19] && n_B[3] < 90 && n_B_IJYOU[9] && (n_B[3] = 21), n_B_KYOUKA[3] && (n_B[12] = n_B[13]), 0 == n_B[19] && n_B_IJYOU[10] && (n_B[12] -= Math.floor(25 * n_B[12] / 100), n_B[13] -= Math.floor(25 * n_B[13] / 100));
        var wATK = 0;
        if (0 == n_B[19] && 0 != n_B_IJYOU[0] && n_B[3] < 90 && (wATK += 2 + 3 * n_B_IJYOU[0]), 0 == Taijin && n_B_IJYOU[22] && (wATK += 300), n_B_KYOUKA[4] && (wATK += 200), n_B[12] += Math.floor(n_B[12] * wATK / 100), n_B[13] += Math.floor(n_B[13] * wATK / 100), n_B_IJYOU[13] && 0 == Taijin && (n_B[12] -= Math.floor(25 * n_B[12] / 100), n_B[13] -= Math.floor(25 * n_B[13] / 100)), n_B_KYOUKA[0] && (n_B[8] += 2 + n_B_KYOUKA[0]), n_B_IJYOU[1]) {
            var w, w2;
            Taijin ? (w2 = 5 * n_B_IJYOU[1], w = Math.floor(n_B[8] / 4)) : (w2 = 10 * n_B_IJYOU[1], w = Math.floor(n_B[8] / 2)), w > w2 ? n_B[8] -= w2 : n_B[8] -= w
        }
        if (0 == n_B[19] && n_B_IJYOU[11] && (n_B[8] -= n_B_IJYOU[11] + 2, n_B[8] < 0 && (n_B[8] = 0)), n_B_IJYOU[1]) {
            var w, w2;
            Taijin ? (w2 = 5 * n_B_IJYOU[1], w = Math.floor(n_B[10] / 4)) : (w2 = 10 * n_B_IJYOU[1], w = Math.floor(n_B[10] / 2)), w > w2 ? n_B[10] -= w2 : n_B[10] -= w
        }
        0 == n_B[19] && n_B_IJYOU[5] && (6 == n_B[2] || n_B[3] >= 90) && (n_B[10] = n_B[10] - Math.floor(n_B[10] / 2)), n_B_IJYOU[15] && 0 == Taijin && (n_B[7] -= Math.floor(40 * n_B[7] / 100)), 0 == n_B[19] && n_B_IJYOU[5] && (6 == n_B[2] || n_B[3] >= 90) && (n_B[9] = n_B[9] - Math.floor(n_B[9] / 2)), n_B_IJYOU[16] && 0 == Taijin && (n_B[9] -= Math.floor(40 * n_B[9] / 100)), 0 == n_B[19] && n_B_IJYOU[10] && (n_B[11] = 0), 0 == Taijin && (n_B[23] = n_B[7], n_B[24] = n_B[7] + (Math.floor(n_B[7] / 20) * Math.floor(n_B[7] / 20) - 1), n_B[23] > n_B[24] && (n_B[24] = n_B[23])), w = n_B_KYOUKA[10], w && (n_B[23] = Math.floor(n_B[23] * (1 + .05 * w)), n_B[24] = Math.floor(n_B[24] * (1 + .05 * w))), n_B[25] = Math.floor(n_B[7] / 2) + n_B[9], n_B[26] = n_B[5] + n_B[10], n_B[27] = n_B[5] + n_B[8];
        var wDEF = 0;
        0 == n_B[19] && 0 != n_B_IJYOU[0] && n_B[3] < 90 && (wDEF += 5 + 5 * n_B_IJYOU[0]), 0 == Taijin && n_B_IJYOU[22] && (wDEF += 50), 0 == Taijin && n_B_IJYOU[24] && (wDEF += 5 * n_B_IJYOU[24]), wDEF > 100 && (wDEF = 100), 0 == Taijin && (n_B[14] -= Math.floor(n_B[14] * wDEF / 100)), 0 == n_B[19] && n_B_IJYOU[2] && (n_B[14] -= Math.floor(25 * n_B[14] / 100));
        var w = 0;
        w += n_tok[290], SRV ? (0 == n_B[19] && (w += n_tok[291]), 1 == n_B[19] && (w += n_tok[292]), w += n_tok[300 + n_B[2]], (324 == n_A_ActiveSkill || 159 == n_A_ActiveSkill || 384 == n_A_ActiveSkill || 162 == n_A_ActiveSkill || 193 == n_A_ActiveSkill || 405 == n_A_ActiveSkill || 438 == n_A_ActiveSkill) && (w = 0)) : (w += n_tok[300 + n_B[2]], (324 == n_A_ActiveSkill || 159 == n_A_ActiveSkill || 384 == n_A_ActiveSkill || 162 == n_A_ActiveSkill || 193 == n_A_ActiveSkill || 405 == n_A_ActiveSkill || 438 == n_A_ActiveSkill) && (w = 0)), w && (0 > w && (w = 0), n_B[14] -= Math.floor(n_B[14] * w / 100)), n_B_IJYOU[14] && 0 == Taijin && (n_B[14] -= Math.floor(15 * n_B[14] / 100)), 0 == n_B[19] && n_B_IJYOU[4] && n_B[3] < 90 && (n_B[14] -= Math.floor(50 * n_B[14] / 100)), 0 == n_B[19] && n_B_IJYOU[9] && n_B[3] < 90 && (n_B[14] -= Math.floor(50 * n_B[14] / 100)), n_B_KYOUKA[9] && (SRV ? n_B[14] = 90 : n_B[14] *= 2), n_B_IJYOU[12] && (6 == n_B[2] || n_B[3] >= 90) && (n_B[14] -= Math.floor(n_B[14] * (10 + 4 * n_B_IJYOU[12]) / 100)), n_B_IJYOU[20] && 0 == Taijin && (n_B[14] = 0), n_B[23] -= Math.floor(n_B[23] * wDEF / 100), n_B[24] -= Math.floor(n_B[24] * wDEF / 100), 0 == n_B[19] && n_B_IJYOU[2] && (n_B[23] -= Math.floor(25 * n_B[23] / 100), n_B[24] -= Math.floor(25 * n_B[24] / 100)), 0 == n_B[19] && n_B_IJYOU[4] && n_B[3] < 90 && (n_B[23] -= Math.floor(50 * n_B[23] / 100), n_B[24] -= Math.floor(50 * n_B[24] / 100)), 0 == n_B[19] && n_B_IJYOU[9] && n_B[3] < 90 && (n_B[23] -= Math.floor(50 * n_B[23] / 100), n_B[24] -= Math.floor(50 * n_B[24] / 100)), 0 == Taijin && n_B_KYOUKA[8] && (n_B[23] -= Math.floor(20 * n_B[23] * n_B_KYOUKA[8] / 100), n_B[24] -= Math.floor(20 * n_B[24] * n_B_KYOUKA[8] / 100)), 0 == Taijin && n_B_IJYOU[21] && (n_B[24] += 90), n_B_IJYOU[20] && (n_B[23] = 0, n_B[24] = 0);
        var w = 0;
        if (w += n_tok[295], w += n_tok[310 + n_B[2]], w && (0 > w && (w = 0), n_B[15] -= Math.floor(n_B[15] * w / 100)), 0 == n_B[19] && n_B_IJYOU[4] && n_B[3] < 90 && (n_B[15] += Math.floor(25 * n_B[15] / 100)), 0 == n_B[19] && n_B_IJYOU[9] && n_B[3] < 90 && (n_B[15] += Math.floor(25 * n_B[15] / 100)), 0 == n_B[19] && n_B_IJYOU[18] && n_B[3] < 90 && (n_B[25] -= Math.floor(12 * n_B[25] * n_B_IJYOU[18] / 100)), 0 == Taijin && n_B_KYOUKA[7] && (n_B[25] -= Math.floor(20 * n_B[25] * n_B_KYOUKA[7] / 100)), 0 == Taijin && n_B_IJYOU[21] && (n_B[25] = 90), n_B[26] += n_B_manual[36], 0 == n_B[19] && n_B_IJYOU[3] && (n_B[26] -= 25, n_B[26] < 1 && (n_B[26] = 1)), !n_B[20] && 2 != document.calcForm.B_AtkRange.value || 1 == document.calcForm.B_AtkRange.value || n_A_Buf6[3] && (n_B[26] -= 50, n_B[26] < 1 && (n_B[26] = 1)), n_B_KYOUKA[4] && (n_B[26] = 2 * n_B[26]), n_B[27] += n_B_manual[37], 0 == n_B[19] && n_B_IJYOU[3] && (n_B[27] -= Math.floor(25 * n_B[27] / 100)), 0 != n_B_KYOUKA[5] && (n_B[27] = Math.floor(n_B[27] * (1 + .2 * n_B_KYOUKA[5]))), n_B_IJYOU[17] && (n_B[27] -= 50, n_B[27] < 0 && (n_B[27] = 0)), 0 == n_B[19] && n_B_IJYOU[4] && n_B[3] < 90 && (n_B[27] = -19), 0 == n_B[19] && n_B_IJYOU[9] && n_B[3] < 90 && (n_B[27] = -19), 0 == n_B[19] && (n_B_IJYOU[7] || n_B_IJYOU[8]) && (n_B[27] = -19), 0 == Taijin) {
            var w1_Exp = 100;
            w1_Exp += StPlusCard(120 + n_B[2]), w1_Exp += StPlusCalc2(120 + n_B[2]);
            var w2_Exp = 0;
            EquipNumSearch(1030) && (w1_Exp += 5 * EquipNumSearch(1030)), 3 != n_A_JobClass() || !CardNumSearch(452) || 1 != n_B[2] && 6 != n_B[2] || (w1_Exp += 5), 2 == n_B[2] && 4 == n_A_JobClass() && CardNumSearch(453) && (w1_Exp += 5), n_A_Buf8[1] && (w1_Exp += 25 * n_A_Buf8[1]), n_A_Buf8[2] && (w2_Exp += 50), n_A_Buf6[2] && (w1_Exp += 100), (3 == document.calcForm.A8_Skill14.value || n_A_Buf6[2]) && (w1_Exp = 2 * w1_Exp, w2_Exp = 2 * w2_Exp), (0 != w1_Exp || 0 != w2_Exp) && (n_B[16] = Math.floor(n_B[16] * w1_Exp / 100), n_B[17] = Math.floor(n_B[17] * (w1_Exp + w2_Exp) / 100)), n_A_Buf8[5] && (n_B[16] = Math.floor(n_B[16] / (1 + n_A_Buf8[5]) + 1), n_B[17] = Math.floor(n_B[17] / (1 + n_A_Buf8[5]) + 1)), n_A_Buf8[6] && (n_B[16] = Math.floor(n_B[16] * (100 + 25 * n_A_Buf8[6]) / 100), n_B[17] = Math.floor(n_B[17] * (100 + 25 * n_A_Buf8[6]) / 100)), SkillSearch(367) && (n_B[16] = Math.floor(n_B[16] * (100 + 10 * SkillSearch(367)) / 100), n_B[17] = Math.floor(n_B[17] * (100 + 10 * SkillSearch(367)) / 100)), n_A_Buf8[7] && (n_B[17] = Math.floor(n_B[17] * (1 + n_A_Buf8[7]))), n_A_Buf8[3] && (n_B[16] = Math.floor(n_B[16] * (1 + n_A_Buf8[3]))), 0 == n_B[19] && n_A_Buf3[8] && (n_B[16] = Math.floor(n_B[16] * (125 + 11 * n_A_Buf3[8]) / 100), n_B[17] = Math.floor(n_B[17] * (125 + 11 * n_A_Buf3[8]) / 100))
        }
        n_B[21] = n_B[27] + 20, n_B[22] = n_B[26] + 75, myInnerHtml("B_AA", " + ", 0), myInnerHtml("B_AB", " + ", 0);
        var wIJ = [6, 12, 13, 21, 22, 14, 15, 23, 25],
            wIJ2 = [16, 17],
            wFront = "<B style='color:blue'>",
            wFront2 = "<B style='color:red'>",
            wBack = "</B>";
        for (i = 0; 8 >= i; i++) {
            var wIJstr = n_B[wIJ[i]];
            n_B[wIJ[i]] < n_B2[wIJ[i]] && (wIJstr = wFront + n_B[wIJ[i]] + wBack), n_B[wIJ[i]] > n_B2[wIJ[i]] && (wIJstr = wFront2 + n_B[wIJ[i]] + wBack), myInnerHtml("B_" + wIJ[i], wIJstr, 0)
        }
        if (0 == Taijin)
            for (i = 0; 1 >= i; i++) {
                var wIJstr = n_B[wIJ2[i]];
                n_B[wIJ2[i]] < n_B2[wIJ2[i]] && (wIJstr = wFront2 + n_B[wIJ2[i]] + wBack), n_B[wIJ2[i]] > n_B2[wIJ2[i]] && (wIJstr = wFront + n_B[wIJ2[i]] + wBack), myInnerHtml("B_" + wIJ2[i], wIJstr, 0)
            }
        myInnerHtml("B_2", SyuzokuOBJ[n_B[2]], 0), w = Math.floor(n_B[3] / 10), n_B[3] != n_B2[3] ? myInnerHtml("B_3", "<b>" + wFront2 + (ZokuseiOBJ[w] + n_B[3] % 10) + wBack + "</b>", 0) : myInnerHtml("B_3", "<b>" + (ZokuseiOBJ[w] + n_B[3] % 10) + "</b>", 0), myInnerHtml("B_4", SizeOBJ[n_B[4]], 0), n_B[27] += n_B_manual[37], 1 == document.calcForm.A8_Skill14.value && (n_Ses = 1), n_Ses && (n_B[27] = Math.floor(.8 * n_B[27])), n_B_DEF2 = [0, 0, 0], n_B_DEF2[2] = n_B[23], n_B_DEF2[0] = n_B[24], n_B_DEF2[1] = Math.floor((n_B_DEF2[2] + n_B_DEF2[0]) / 2), n_B_MDEF2 = n_B[25], n_B_HIT = n_B[26], n_B_FLEE = n_B[27]
    }
}

function calc() {
    for (var _ = 0; 2 >= _; _++) InnStr[_] = "";
    if (StAllCalc(), wCSize = weaponsize[n_A_WeaponType][n_B[4]], SkillSearch(78) && (4 != n_A_WeaponType && 5 != n_A_WeaponType || 1 != n_B[4] || (wCSize = 1)), (SkillSearch(153) || n_A_Buf2[7]) && (wCSize = 1), (32 == cardOBJ[n_A_card[0]][0] || 32 == cardOBJ[n_A_card[1]][0] || 32 == cardOBJ[n_A_card[2]][0] || 32 == cardOBJ[n_A_card[3]][0] || 32 == cardOBJ[n_A_card[4]][0] || 32 == cardOBJ[n_A_card[5]][0] || 32 == cardOBJ[n_A_card[6]][0] || 32 == cardOBJ[n_A_card[7]][0]) && (wCSize = 1), EquipNumSearch(1177) && (wCSize = 1), w_HIT = n_A_HIT + 80 - n_B_FLEE, w_HIT_EDP = w_HIT, w_HIT_EDP > 100 && (w_HIT_EDP = 100), w_HIT_EDP < 5 && (w_HIT_EDP = 5), SkillSearch(148) && (w_HIT = Math.floor(w_HIT * (100 + 2 * SkillSearch(148)) / 100)), (70 == n_A_ActiveSkill || 6 == n_A_ActiveSkill) && (w_HIT *= 1 + .05 * n_A_ActiveSkillLV), 83 != n_A_ActiveSkill && 388 != n_A_ActiveSkill || !SkillSearch(381) || (w_HIT *= 1.5), 7 == n_A_ActiveSkill && (w_HIT *= 1 + .1 * n_A_ActiveSkillLV), 272 == n_A_ActiveSkill && (w_HIT *= 1 + .1 * n_A_ActiveSkillLV), 337 == n_A_ActiveSkill && (w_HIT = 100), 0 == SRV && 324 == n_A_ActiveSkill && (w_HIT += 20), 384 == n_A_ActiveSkill && (w_HIT = 100), SkillSearch(364) && (w_HIT = 100), w_HIT > 100 ? w_HIT = 100 : w_HIT < 5 && (w_HIT = 5), StPlusCalc2(86) + StPlusCard(86) && (w_HIT += (100 - w_HIT) * (StPlusCalc2(86) + StPlusCard(86)) / 100), w_HIT = Math.floor(100 * w_HIT) / 100, w_HIT_HYOUJI = w_HIT, 272 == n_A_ActiveSkill && (n_A_CRI += 20), 401 == n_A_ActiveSkill && (n_A_CRI += 25 + 5 * n_A_ActiveSkillLV), w_Cri = n_A_CRI - .2 * n_B[11] + .1, n_B_IJYOU[8] && (w_Cri *= 2), w_Cri < 0 ? w_Cri = 0 : w_Cri > 100 && (w_Cri = 100), TyouEnkakuSousa3dan = 0, wBC3_3danHatudouRitu = 0, SkillSearch(187) && (wBC3_3danHatudouRitu = 30 - SkillSearch(187)), wDA = 5 * SkillSearch(13), 1 != n_A_WeaponType && (wDA = 0), CardNumSearch(43) && (SkillSearch(13) > 1 ? wDA = 5 * SkillSearch(13) : wDA = 5), 0 != n_A_WeaponType && ((EquipNumSearch(570) || EquipNumSearch(1442) || EquipNumSearch(1443)) && (SkillSearch(13) > 2 ? wDA = 5 * SkillSearch(13) : wDA = 10), EquipNumSearch(1578) && (wDA = 5 * SkillSearch(13), (5 == n_A_SHOULDER_DEF_PLUS || 6 == n_A_SHOULDER_DEF_PLUS) && SkillSearch(13) < 1 && (wDA = 5), n_A_SHOULDER_DEF_PLUS >= 7 && SkillSearch(13) < 5 && (wDA = 25)), EquipNumSearch(1321) && (SkillSearch(13) > 5 ? wDA = 5 * SkillSearch(13) : wDA = 25)), (EquipNumSearch(399) || EquipNumSearch(1571)) && (SkillSearch(13) > 5 ? wDA = 5 * SkillSearch(13) : wDA = 25), 17 == n_A_WeaponType && (wDA = 5 * SkillSearch(427), CardNumSearch(43) && (wDA = 5 * SkillSearch(427) + 5 * (100 - 5 * SkillSearch(427)) / 100), (EquipNumSearch(570) || EquipNumSearch(1442) || EquipNumSearch(1443)) && (wDA = 5 * SkillSearch(427) + 10 * (100 - 5 * SkillSearch(427)) / 100)), w_HIT_DA = w_HIT, 0 != wDA && 17 != n_A_WeaponType && (w_HIT_DA = w_HIT_DA * (100 + SkillSearch(13)) / 100, w_HIT_DA >= 100 && (w_HIT_DA = 100)), w998A = 100 - wBC3_3danHatudouRitu, w998B = wBC3_3danHatudouRitu * w_HIT / 100, w998C = wBC3_3danHatudouRitu - w998B, w998D = w998A * wDA / 100, w998E = w998D * w_HIT_DA / 100, w998F = w998D - w998E, w998G = (100 - wBC3_3danHatudouRitu - w998D) * w_Cri / 100, w998H = 100 - wBC3_3danHatudouRitu - w998D - w998G, w998I = w998H * w_HIT / 100, w998J = w998H - w998I, w998K = w998B + w998E + w998G + w998I, 0 == SRV && (w_HIT >= 100 && (w998K = 100), w_Cri >= 100 && (w998K = 100)), w998L = 100 - w998K, (0 == n_A_ActiveSkill || 272 == n_A_ActiveSkill || 401 == n_A_ActiveSkill || 86 == n_A_ActiveSkill && 50 <= n_B[3] && n_B[3] < 60) && (w_HIT_HYOUJI = Math.floor(100 * w998K) / 100, myInnerHtml("CRInum", Math.round(100 * w998G) / 100 + SubName[0], 0)), w_FLEE = n_A_FLEE + 20 - n_B_HIT, w_FLEE > 95 ? w_FLEE = 95 : w_FLEE < 5 && (w_FLEE = 5), (444 == n_B_AtkSkill || 445 == n_B_AtkSkill || 475 == n_B_AtkSkill || 476 == n_B_AtkSkill || 481 == n_B_AtkSkill) && (w_FLEE = 0, n_A_LUCKY = 0, n_A_Buf6[3] && 0 != n_B_rangedAtk && (w_FLEE = 75)), n_A_Buf6[3] && 0 != n_B_rangedAtk && (w_FLEE = 75), 0 != n_B_AtkSkill && (n_A_LUCKY = 0), myInnerHtml("BattleFLEE", Math.floor(100 * (w_FLEE + (100 - w_FLEE) * n_A_LUCKY / 100)) / 100, 0), n_A_workDEX = Math.floor(n_A_DEX * (1 + .2 * (n_A_WeaponLV - 1))), n_A_DMG = [0, 0, 0], n_A_workDEX >= n_A_Weapon_ATK || SkillSearch(155) ? n_A_DMG[2] = n_A_ATK + n_A_WeaponLV_Maxplus + Math.floor((n_A_Weapon_ATK + wImp) * wCSize) : n_A_DMG[2] = n_A_ATK + n_A_WeaponLV_Maxplus + Math.floor((n_A_Weapon_ATK - 1 + wImp) * wCSize), (10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType) && (n_A_DMG[2] += Math.floor((ArrowOBJ[n_A_Arrow][0] - 1) * wCSize)), (10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType) && (w1 = n_A_ATK + n_A_WeaponLV_Maxplus + Math.floor(n_A_Weapon_ATK * n_A_Weapon_ATK / 100 * wCSize) + Math.floor(wImp * wCSize), t = n_A_ATK + n_A_WeaponLV_Maxplus + Math.floor(n_A_Weapon_ATK * n_A_workDEX / 100 * wCSize) + Math.floor(wImp * wCSize), l = Math.floor((ArrowOBJ[n_A_Arrow][0] - 1) * wCSize), w1 += l, t += l, w1 > t && (w1 = t), n_A_DMG[2] < w1 && (n_A_DMG[2] = w1)), 10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType ? (n_A_DMG[0] = n_A_ATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_Weapon_ATK / 100 + wImp) * wCSize), l = n_A_ATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK * n_A_workDEX / 100 + wImp) * wCSize), n_A_DMG[0] > l && (n_A_DMG[0] = l)) : n_A_workDEX >= n_A_Weapon_ATK ? n_A_DMG[0] = n_A_ATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + wImp) * wCSize) : (SkillSearch(155) && (n_A_workDEX = n_A_Weapon_ATK), n_A_DMG[0] = n_A_ATK + n_A_WeaponLV_Minplus + Math.floor((n_A_workDEX + wImp) * wCSize)), n_A_DMG[1] = (n_A_DMG[0] + n_A_DMG[2]) / 2, n_Enekyori = 0, n_A_CriATK = [0, 0, 0], n_A_CriATK[1] = n_A_ATK + (n_A_WeaponLV_Minplus + n_A_WeaponLV_Maxplus) / 2 + Math.floor((n_A_Weapon_ATK + wImp) * wCSize), n_A_CriATK[0] = n_A_ATK + n_A_WeaponLV_Minplus + Math.floor((n_A_Weapon_ATK + wImp) * wCSize), n_A_CriATK[2] = n_A_ATK + n_A_WeaponLV_Maxplus + Math.floor((n_A_Weapon_ATK + wImp) * wCSize), (10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType) && (n_Enekyori = 1, 10 == n_A_WeaponType))
        for (_ = 0; 2 >= _; _++) n_A_CriATK[_] += Math.floor(ArrowOBJ[n_A_Arrow][0] * wCSize);
    BK_n_A_DMG = [0, 0, 0], BK_n_A_DMG[2] = n_A_DMG[2], BK_n_A_DMG[0] = n_A_DMG[0], BK_n_A_DMG[1] = n_A_DMG[1], ATKbai01(), ATKbai02(1, 1), n_PerHIT_DMG = BattleCalc2(0), wCriTyuu = 1, n_A_CriATK[1] = BattleCalc(n_A_CriATK[1], 10), n_A_CriATK[0] = BattleCalc(n_A_CriATK[0], 10), n_A_CriATK[2] = BattleCalc(n_A_CriATK[2], 10), wCriTyuu = 0, n_A_EDP_DMG = [0, 0, 0];
    for (var _ = 0; 2 >= _; _++) n_A_EDP_DMG[_] = BattleCalcEDP(n_A_DMG[_], _);
    for (var _ = 0; 2 >= _; _++) n_A_CriATK[_] += EDP_DMG(_);
    var n = [0, 0, 0];
    if (11 == n_A_WeaponType) {
        for (var _ = 0; 2 >= _; _++) n[_] = Math.floor(n_A_CriATK[_] * (.01 + .02 * SkillSearch(13))), n_A_CriATK[_] += n[_];
        n_A_CriATK[0] == n_A_CriATK[2] ? myInnerHtml("CRIATK", n_A_CriATK[0] + " (" + (n_A_CriATK[0] - n[0]) + " + " + n[0] + ")", 0) : myInnerHtml("CRIATK", n_A_CriATK[0] + "~" + n_A_CriATK[2] + " (" + (n_A_CriATK[0] - n[0]) + "~" + (n_A_CriATK[2] - n[2]) + " + " + n[0] + "~" + n[2] + ")", 0)
    } else n_A_CriATK[0] == n_A_CriATK[2] ? myInnerHtml("CRIATK", n_A_CriATK[1], 0) : myInnerHtml("CRIATK", n_A_CriATK[0] + "~" + n_A_CriATK[2], 0);
    if (n_Max_DMG = 0, n_Min_DMG = 9999999, (0 == n_A_ActiveSkill || 86 == n_A_ActiveSkill && 50 <= n_B[3] && n_B[3] < 60) && w998G > 0 && (n_Min_DMG = n_A_CriATK[0], n_Max_DMG = n_A_CriATK[2]), BattleCalc999(), myInnerHtml("A_WeaponElement", ZokuseiOBJ[n_A_Weapon_zokusei] + " (" + 100 * zokusei[n_B[3]][n_A_Weapon_zokusei] + "% vs " + ZokuseiOBJ[Math.floor(n_B[3] / 10)] + n_B[3] % 10 + ")", 0), 0 == n_Enekyori) {
        var e = n_B_KYOUKA[12],
            i = n_B_manual[21];
        if (e > 0) {
            e = 10 + 3 * e;
            var l, t = 1;
            1 == n_B_KYOUKA[1] && (t = 1.5);
            for (var _ = 0; 2 >= _; _++) l = Math.floor(Last_DMG_A[_] * t * e / 100), 0 == l && (l = 1), InnStr[_] += "<BR><B style='color:red'>" + l + " (Reflected)</B>"
        }
        if (i) {
            var l, t = 1,
                a = 1;
            1 == n_B_KYOUKA[1] && (t = 1.5), 1 == n_Ses && (a = 0 != n_A_ActiveSkill ? 100 / 60 : 1.25);
            for (var _ = 0; 2 >= _; _++) l = Math.floor(Last_DMG_B[_] * t * a * i / 100), 0 == l && (l = 1), InnStr[_] += "<BR><B style='color:red'>" + l + " (Reflected)</B>"
        }
    }
    for (var _ = 0; _ < InnStr.length; _++) myInnerHtml("strID_" + _, InnStr[_], 0)
}

function BattleCalc(_, n) {
    return 10 == n ? _ += n_A_WeaponLV_seirenATK : _ = BattleCalc4(_, n, 0), 1 > _ && (_ = 1), 1 == n_A_WeaponType || 2 == n_A_WeaponType ? _ += 4 * SkillSearch(3) : 3 == n_A_WeaponType ? _ += 4 * SkillSearch(4) : 4 == n_A_WeaponType || 5 == n_A_WeaponType ? _ += 0 == SkillSearch(78) ? 4 * SkillSearch(69) : 5 * SkillSearch(69) : 8 == n_A_WeaponType ? _ += 3 * SkillSearch(89) : 11 == n_A_WeaponType ? _ += 3 * SkillSearch(81) : 14 == n_A_WeaponType ? _ += 3 * SkillSearch(198) : 15 == n_A_WeaponType ? _ += 3 * SkillSearch(206) : 12 == n_A_WeaponType ? _ += 3 * SkillSearch(224) : 6 == n_A_WeaponType || 7 == n_A_WeaponType ? _ += 3 * SkillSearch(241) : (13 == n_A_WeaponType || 0 == n_A_WeaponType) && (_ += 3 * SkillSearch(183)), 0 == n_A_WeaponType && SkillSearch(329) && (_ += 10 * SkillSearch(329)), !n_A_Buf3[10] || 4 != n_A_WeaponLV && 4 != n_A_Weapon2LV || (_ += 50 + 25 * n_A_Buf3[10]), (6 == n_B[2] || 90 <= n_B[3] && n_B[3] <= 99) && SkillSearch(24) && (_ += Math.floor((3 + .05 * n_A_BaseLV) * SkillSearch(24))), (2 == n_B[2] || 4 == n_B[2]) && (_ += 4 * SkillSearch(116), SkillSearch(390) && (_ += n_A_STR)), _ = BattleCalc2(_), Math.floor(_)
}

function BattleCalc2(w999) {
    if (w999_AB = 0, w999 > 0 && (w999_AB = 1), w999 += 2 * SkillSearch(148), 0 == wBCEDPch && (w999 = Math.floor(w999 * zokusei[n_B[3]][n_A_Weapon_zokusei])), 0 == n_A_WeaponType && SkillSearch(329) && (331 == n_A_ActiveSkill || 333 == n_A_ActiveSkill || 335 == n_A_ActiveSkill || 337 == n_A_ActiveSkill) && (w999 += 10 * SkillSearch(329)), w999 += 3 * n_A_Buf2[12], w999 += 3 * SkillSearch(416), 0 != n_A_WeaponType && 1 == w999_AB && (w999 += 20 * SkillSearch(254)), 0 == wBCEDPch && ((17 == n_A_ActiveSkill || 307 == n_A_ActiveSkill) && (w999 += 15 * n_A_ActiveSkillLV), 86 == n_A_ActiveSkill && (n_B[3] < 50 || 60 <= n_B[3]) && (w999 += 75)), 423 == n_A_ActiveSkill && (w999 += Math.floor(n_A_MATK[w_MagiclBulet] * (100 - n_B[15]) / 100 - n_B_MDEF2)), 437 == n_A_ActiveSkill && (w999 += 50 * n_A_ActiveSkillLV), 106 == cardOBJ[n_A_card[0]][0] && 106 == cardOBJ[n_A_card[1]][0] && 106 == cardOBJ[n_A_card[2]][0]) w999 += 40;
    else
        for (i = 0; 2 >= i; i++) 106 == cardOBJ[n_A_card[i]][0] && (w999 += 5);
    return 106 == n_A_card[3] && (w999 += 10), 394 == n_A_ActiveSkill && (w999 += SyurikenOBJ[eval(document.calcForm.SkillSubNum.value)][0], w999 += 3 * SkillSearch(393), w999 += 4 * n_A_ActiveSkillLV), 395 == n_A_ActiveSkill && (w999 += 3 * KunaiOBJ[eval(document.calcForm.SkillSubNum.value)][0]), w999 = BaiCI(w999), 169 == n_A_ActiveSkill && 10 == n_A_WeaponType && (w999 = Math.floor(w999 / 2)), n_Nitou && 0 == n_A_ActiveSkill && 0 != n_A_WeaponType && (w999 = Math.floor(w999 * (50 + 10 * SkillSearch(79)) / 100)), 423 == n_A_ActiveSkill && (w999 *= zokusei[n_B[3]][8]), 437 == n_A_ActiveSkill && (w999 *= zokusei[n_B[3]][0]), 1 == Taijin && (317 == n_A_ActiveSkill || 318 == n_A_ActiveSkill) && (w999 = 0), w999
}

function BaiCI(_) {
    if (0 == wBCEDPch && 0 == not_use_card) {
        var n = 0;
        n = n_tok[30 + n_B[2]], _ = Math.floor(_ * (100 + n) / 100), n = n_tok[40 + Math.floor(n_B[3] / 10)], _ = Math.floor(_ * (100 + n) / 100), n = n_tok[27 + n_B[4]], _ = Math.floor(_ * (100 + n) / 100), 1 == n_Enekyori && -1 != TyouEnkakuSousa3dan && (n = n_tok[25], _ = Math.floor(_ * (100 + n) / 100)), n = 0, 1 == n_B[19] && (n += n_tok[26]), n += n_tok[80], _ = Math.floor(_ * (100 + n) / 100), 1 == wCriTyuu && 401 != n_A_ActiveSkill && (_ = Math.floor(_ * (100 + n_tok[70]) / 100)), (108 <= n_B[0] && n_B[0] <= 115 || 319 == n_B[0]) && (_ = Math.floor(_ * (100 + n_tok[81]) / 100)), 116 <= n_B[0] && n_B[0] <= 120 && (_ = Math.floor(_ * (100 + n_tok[82]) / 100)), (49 <= n_B[0] && n_B[0] <= 52 || 55 == n_B[0] || 221 == n_B[0]) && (_ = Math.floor(_ * (100 + n_tok[83]) / 100)), (106 == n_B[0] || 152 == n_B[0] || 308 == n_B[0] || 32 == n_B[0] || 541 == n_B[0]) && (_ = Math.floor(_ * (100 + n_tok[84]) / 100)), _ = Math.floor(_ * (100 + StPlusCalc2(1e3 + n_B[0]) + StPlusCard(1e3 + n_B[0])) / 100), SkillSearch(258) && (_ = 2 * _), SkillSearch(266) && (_ = Math.floor(_ * (150 + 50 * SkillSearch(266)) / 100)), 86 == n_A_ActiveSkill && 50 <= n_B[3] && n_B[3] < 60 && (_ = Math.floor(_ * (100 + 30 * n_A_ActiveSkillLV) / 100)), 11 == n_A_WeaponType && SkillSearch(262) && (_ = Math.floor(_ * (110 + 2 * SkillSearch(262)) / 100)), n = 0, 0==Taijin?SkillSearch(354)&&SkillSearch(365)?n+=(n_A_BaseLV+n_A_STR+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(354)):SkillSearch(354)&&2==n_B[4]&&17392<=n_B[6]?n+=(n_A_BaseLV+n_A_STR+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(354)):SkillSearch(352)&&0==n_B[4]?n+=(n_A_BaseLV+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(352)):SkillSearch(353)&&1==n_B[4]&&5218<=n_B[6]&&(n+=(n_A_BaseLV+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(353))):SkillSearch(354)?n+=(n_A_BaseLV+n_A_STR+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(354)):SkillSearch(352)?n+=(n_A_BaseLV+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(352)):SkillSearch(353)&&(n+=(n_A_BaseLV+n_A_LUK+n_A_DEX)/(12-3*SkillSearch(353))),_=Math.floor(_*(100+n)/100);
    }
    return _ = Math.floor(tPlusDamCut(_)), n = 0, 6 == n_A_ActiveSkill && n_A_SHOES_DEF_PLUS >= 9 && CardNumSearch(362) && (n += 10), 76 == n_A_ActiveSkill && (2 == n_A_WeaponType || 3 == n_A_WeaponType) && (n += 25 * CardNumSearch(464)), 41 == n_A_ActiveSkill && 10 == n_A_WeaponType && (n += 50 * CardNumSearch(465)), 40 == n_A_ActiveSkill && n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1089) && (n += 20), 428 == n_A_ActiveSkill && n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1099) && (n += 2 * n_A_Weapon_ATKplus), 430 == n_A_ActiveSkill && n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1100) && (n += 3 * n_A_Weapon_ATKplus), 436 == n_A_ActiveSkill && n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1102) && (n += 2 * n_A_Weapon_ATKplus), 437 == n_A_ActiveSkill && n_A_Weapon_ATKplus >= 9 && EquipNumSearch(1103) && (n += 2 * n_A_Weapon_ATKplus), (6 == n_A_ActiveSkill || 76 == n_A_ActiveSkill) && 10 == n_A_ActiveSkillLV && EquipNumSearch(1159) && (n += 50), 65 == n_A_ActiveSkill && SU_LUK >= 90 && SU_DEX >= 90 && EquipNumSearch(1164) && (n += 15), 264 == n_A_ActiveSkill && EquipNumSearch(1176) && 10 == SkillSearch(81) && (n += 20), -1 == TyouEnkakuSousa3dan && EquipNumSearch(639) && (n += 15), 83 != n_A_ActiveSkill && 388 != n_A_ActiveSkill || !SkillSearch(381) || 0 != wBCEDPch || (n += 10), 384!=n_A_ActiveSkill&&159!=n_A_ActiveSkill&&(_=_*(100+StPlusCalc2(5e3+n_A_ActiveSkill)+StPlusCard(5e3+n_A_ActiveSkill)+n)/100), n_A_Buf7[20] && MANUKU_MONSTER() && (_ = 110 * _ / 100), n_A_Buf7[23] && SUPURE_MONSTER() && (_ = 110 * _ / 100), _
}

function BattleCalc3(_) {
    return wBC3_3dan = w998B * TyouEnkakuSousa3dan, wBC3_DA = w998E * _ * 2, wBC3_Cri = w998G * n_A_CriATK[1], wBC3_Normal = w998I * _, wBC3_Miss = w998L * BattleCalc2(0), wBC3_X = (wBC3_3dan + wBC3_DA + wBC3_Cri + wBC3_Normal + wBC3_Miss) / 100, tPlusLucky(wBC3_X)
}

function BattleCalc3left(_) {
    for (wBC3L2 = 0, i = 4; 7 >= i; i++) 106 == cardOBJ[n_A_card[i]][0] && (wBC3L2 += 5);
    return wBC3_Normal = _ * w_HIT / 100, wBC3_Miss = wBC3L2 * (100 - w_HIT) / 100, wBC3_X = wBC3_Normal + wBC3_Miss, wBC3_X = tPlusDamCut(wBC3_X), tPlusLucky(wBC3_X)
}

function SkillSearch(_) {
    if (258 == _ && TimeItemNumSearch(35)) return 1;
    for (var n = 0; 14 >= n; n++)
        if (JobSkillPassOBJ[n_A_JOB][n] == _) return n_A_Buf[n];
    return 0
}

function BattleCalc4(_, n, e) {
    return e = 0 == e ? n_A_WeaponLV_seirenATK : n_A_Weapon2LV_seirenATK, 275 == n_A_ActiveSkill ? Math.floor(_ * (100 - n_B[14]) / 100) - n_B_DEF2[n] + e : 432 == n_A_ActiveSkill ? _ + e : n_tok[180 + n_B[2]] >= 1 ? _ + e : n_tok[22] >= 1 && 0 == n_B[19] ? _ + e : n_tok[22] >= 10 ? _ + e : SkillSearch(364) ? _ + e : _ = 0 == n_tok[23] ? Math.floor(_ * (100 - n_B[14]) / 100) - n_B_DEF2[n] + e : 0 == n ? Math.floor(_ * (n_B_DEF2[2] + n_B[14]) / 100) + e : 1 == n ? Math.floor(_ * (n_B_DEF2[1] + n_B[14]) / 100) + e : Math.floor(_ * (n_B_DEF2[0] + n_B[14]) / 100) + e
}

function BattleCalcEDP(_, n) {
    if (0 >= _) return 0;
    if (19 == n_A_ActiveSkill || 263 == n_A_ActiveSkill || 88 == n_A_ActiveSkill || 264 == n_A_ActiveSkill || 248 == n_A_ActiveSkill) return 0;
    wBCEDPch = 1;
    var e = 0,
        i = 0;
    return SkillSearch(266) && (e = BattleCalc(_, n), e = Math.floor(e * zokusei[n_B[3]][5] / 4)), n_A_Buf6[7] && (i = BattleCalc(_, n), i = Math.floor(i * zokusei[n_B[3]][3] / 5)), wBCEDPch = 0, e + i
}

function EDPplus(_) {
    (SkillSearch(266) || n_A_Buf6[7]) && (w_DMG[2] += EDP_DMG(2) * _, w_DMG[1] += EDP_DMG(1) * _, 100 == w_HIT_EDP && (w_DMG[0] += EDP_DMG(0) * _), EDPhyouzi(_))
}

function EDPhyouzi(_) {}

function EDP_DMG(_) {
    if (SkillSearch(266) || n_A_Buf6[7]) {
        if (17 == n_A_ActiveSkill && 52 <= n_B[3] && n_B[3] <= 59) return 0;
        if ((66 == n_A_ActiveSkill || 193 == n_A_ActiveSkill || 197 == n_A_ActiveSkill || 321 == n_A_ActiveSkill) && 83 <= n_B[3] && n_B[3] <= 89) return 0;
        if (zokusei[n_B[3]][n_A_Weapon_zokusei] <= 0 && 0 == n_PerHIT_DMG) return 0;
        if (0 == _) return 100 == w_HIT_EDP ? n_A_EDP_DMG[0] : 0;
        if (1 == _) {
            var n = 0;
            return 100 == w_HIT_HYOUJI && (n = 1), n_PerHIT_DMG && (n = 1), Math.floor(n ? n_A_EDP_DMG[1] * w_HIT_EDP / 100 : n_A_EDP_DMG[1] * w_HIT / 100 * w_HIT_EDP / 100)
        }
        if (2 == _) return n_A_EDP_DMG[2]
    }
    return 0
}

function CastAndDelay() {
    0 != wCast && (str_bSUBname += SubName[9] + "<BR>", str_bSUB += Math.floor(100 * wCast) / 100 + SubName[1] + "<BR>");
    var strSUB2name = "",
        strSUB2 = "";
    wDelay = 0;
    var w = 0;
    n_Delay[1] > wDelay && (wDelay = n_Delay[1], w = 1), n_Delay[2] = Math.floor(n_Delay[2] * (100 - AC_I - (100 - AC_I) * n_tok[74] / 100)) / 100, n_Delay[2] > wDelay && (wDelay = n_Delay[2], w = 2), n_Delay[3] > wDelay && (wDelay = n_Delay[3], w = 3), 0 != n_A_ActiveSkill && 284 != n_A_ActiveSkill && (n_Delay[4] = eval(document.calcForm.Conf01.value) / 100), n_Delay[4] > wDelay + wCast && (wDelay = n_Delay[4] - wCast, w = 4), 0 != n_Delay[5] && (wDelay = n_Delay[5], w = 5), n_Delay[6] > wDelay + wCast && (wDelay = n_Delay[6] - wCast, w = 6), 1 == w && (0 == n_A_ActiveSkill ? SkillSearch(187) ? (strSUB2name += "Attack interval (normal)<BR>Attack Interval (Raging Trifecta Blow)<BR>", strSUB2 += n_Delay[1] + "s<BR>" + sandanDelay + "s<BR>", wDelay = n_Delay[1] * w998A / 100 + sandanDelay * wBC3_3danHatudouRitu / 100) : (strSUB2name += "Time/Hit<BR>", strSUB2 += n_Delay[1] + "s<BR>") : (strSUB2name += "Motion Delay (ASPD Based)<BR>", strSUB2 += n_Delay[1] + "s<BR>")), 2 == w && (strSUB2name += "Delay (Fixed Skills)<BR>", strSUB2 += n_Delay[2] + "s<BR>"), 3 == w && (188 == n_A_ActiveSkill || 189 == n_A_ActiveSkill || 289 == n_A_ActiveSkill ? (strSUB2name += "Delay (+delay reception combo)<BR>", strSUB2 += n_Delay[3] + "~" + (n_Delay[3] + .3) + "s<BR>") : (strSUB2name += "Delay (Forced Motion)<BR>", strSUB2 += n_Delay[3] + "s<BR>")), 4 == w && (strSUB2name += "Delay (Input Limit)<BR>", strSUB2 += n_Delay[4] + "s<BR>"), 5 == w && (strSUB2name += "Damage Interval<BR>", strSUB2 += n_Delay[5] + "s<BR>"), 6 == w && (strSUB2name += "Limited Skill-Duration(?)<BR>", strSUB2 += Math.floor(100 * wDelay) / 100 + "s<BR>"), myInnerHtml("bSUB2name", strSUB2name, 0), myInnerHtml("bSUB2", strSUB2, 0)
}

function tPlusDamCut(wPDC) {
    return 0 == Taijin && 1 == document.calcForm.A8_Skill14.value && (wPDC = Math.floor(10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType ? .6 * wPDC : 0 != n_A_ActiveSkill ? .6 * wPDC : .8 * wPDC), document.calcForm.A8_Skill15.value > 0 && (wPDC = Math.floor(wPDC * (10 / (5 * document.calcForm.A8_Skill15.value))))), n_Ses && (wPDC = Math.floor(10 == n_A_WeaponType || 17 == n_A_WeaponType || 18 == n_A_WeaponType || 19 == n_A_WeaponType || 20 == n_A_WeaponType || 21 == n_A_WeaponType ? .6 * wPDC : 0 != n_A_ActiveSkill ? .6 * wPDC : .8 * wPDC)), w = n_B_manual[1], wPDC = Math.floor(wPDC * (100 - w) / 100), (n_Enekyori || 10 == n_A_WeaponType) && (w = n_B_manual[9], wPDC = Math.floor(wPDC * (100 - w) / 100)), 0 == eval(document.calcForm.A_youshi.checked) && (w = n_B_KYOUKA[14], wPDC = Math.floor(wPDC * (100 - w) / 100)), w = n_B_manual[2], n_A_Weapon_zokusei == w && (w = n_B_manual[3], wPDC = Math.floor(wPDC * (100 - w) / 100)), w = n_B_manual[4], n_A_Weapon_zokusei == w && (w = n_B_manual[5], wPDC = Math.floor(wPDC * (100 - w) / 100)), w = n_B_KYOUKA[13], 0 == w || 1 != n_Enekyori && 10 != n_A_WeaponType || (w = 95 - 15 * n_B_KYOUKA[13], wPDC = Math.floor(wPDC * w / 100)), w = n_B_KYOUKA[14], w > 0 && 2 != n_Enekyori && (wPDC -= Math.floor(wPDC * w * 6 / 100)), 0 == wBTw1 && (n_B_IJYOU[6] && 0 == wLAch && (wPDC *= 2), n_B_IJYOU[17] && 3 == n_A_Weapon_zokusei && (wPDC *= 2), baizok = [110, 114, 117, 119, 120], 0 == n_A_Buf6[0] && n_A_Buf6[1] >= 1 && 3 == n_A_Weapon_zokusei && (wPDC = Math.floor(wPDC * baizok[n_A_Buf6[1] - 1] / 100)), 1 == n_A_Buf6[0] && n_A_Buf6[1] >= 1 && 1 == n_A_Weapon_zokusei && (wPDC = Math.floor(wPDC * baizok[n_A_Buf6[1] - 1] / 100)), 2 == n_A_Buf6[0] && n_A_Buf6[1] >= 1 && 4 == n_A_Weapon_zokusei && (wPDC = Math.floor(wPDC * baizok[n_A_Buf6[1] - 1] / 100))), n_B_KYOUKA[1] && 0 == Taijin && (wPDC = Math.floor(wPDC / 2)), n_B_KYOUKA[1] && 1 == Taijin && (wPDC = Math.floor(2 * wPDC / 3)), n_B_KYOUKA[7] && 2 != n_Enekyori && (wPDC -= Math.floor(20 * wPDC * n_B_KYOUKA[7] / 100)), n_B_KYOUKA[8] && 2 == n_Enekyori && (wPDC -= Math.floor(20 * wPDC * n_B_KYOUKA[8] / 100)), 5 == n_B[19] && (wPDC = 1, 122 == n_A_ActiveSkill && (wPDC = 0)), wPDC
}

function tPlusEnemyClick() {
    if (Taijin) {
        for (n_B = new Array, i = 0; 26 >= i; i++) n_B[i] = MonsterOBJ2[document.calcForm.B_Enemy.value][i];
        document.calcForm.B_LV.value = n_B[5], document.calcForm.B_AGI.value = n_B[8], document.calcForm.B_VIT.value = n_B[7], document.calcForm.B_INT.value = n_B[9], document.calcForm.B_LUK.value = n_B[11], document.calcForm.B_DEF.value = n_B[14], document.calcForm.B_MDEF.value = n_B[15]
    }
}

function tPlusTaiseiSyokia() {
    if (Taijin) {
        for (i = 1; 150 >= i; i++) document.calcForm.B_AGI.options[i - 1] = new Option(i, i), document.calcForm.B_VIT.options[i - 1] = new Option(i, i), document.calcForm.B_INT.options[i - 1] = new Option(i, i), document.calcForm.B_LUK.options[i - 1] = new Option(i, i);
        for (i = 0; 100 >= i; i++) document.calcForm.B_DEF.options[i] = new Option(i, i), document.calcForm.B_MDEF.options[i] = new Option(i, i);
        for (i = 1; 99 >= i; i++) document.calcForm.B_LV.options[i - 1] = new Option(i, i);
        for (i = 0; 9 >= i; i++) document.calcForm.B_ZOKUSEI.options[i] = new Option(ZokuseiOBJ[i] + "1", 10 * i + 1);
        for (i = 0; 9 >= i; i++) document.calcForm.B_TAISEI2_1.options[i] = new Option(ZokuseiOBJ[i], i), document.calcForm.B_TAISEI3_1.options[i] = new Option(ZokuseiOBJ[i], i);
        for (i = 0; 10 >= i; i++) document.calcForm.B_TAISEI4.options[i] = new Option(i, i);
        for (i = 0; 5 >= i; i++) document.calcForm.B_TAISEI5.options[i] = new Option(i, i);
        for (i = 0; 10 >= i; i++) document.calcForm.B_TAISEI10.options[i] = new Option(i, i);
        for (i = 0; 5 >= i; i++) document.calcForm.B_TAISEI13.options[i] = new Option(EnergyCoatOBJ[i], i);
        for (i = 0; 10 >= i; i++) document.calcForm.B_TAISEI14.options[i] = new Option(i, i);
        for (n_B = new Array, i = 0; 26 >= i; i++) n_B[i] = MonsterOBJ2[document.calcForm.B_Enemy.value][i];
        i = eval(document.calcForm.B_Enemy.value), document.calcForm.B_LV.value = MonsterOBJ2[i][5], document.calcForm.B_VIT.value = MonsterOBJ2[i][7], document.calcForm.B_AGI.value = MonsterOBJ2[i][8], document.calcForm.B_INT.value = MonsterOBJ2[i][9], document.calcForm.B_LUK.value = MonsterOBJ2[i][11], document.calcForm.B_DEF.value = MonsterOBJ2[i][14], document.calcForm.B_MDEF.value = MonsterOBJ2[i][15]
    }
}

function tPlusLucky(_) {
    return Taijin ? (w = n_B_manual[38], w += n_B[11] / 10, w = _ * (100 - w) / 100, w) : _
}

function tPlusAG() {
    2 != n_Enekyori && n_B_KYOUKA[11] > 0 && (wPAG = w_AG[n_B_KYOUKA[11]], w_DMG[0] *= wPAG / 100, w_DMG[1] *= wPAG / 100, w_DMG[2] *= wPAG / 100)
}
n_A_WeaponLV = 0, n_A_Weapon2LV = 0, n_Nitou = 0, n_Tensei = 0, n_Ses = 0, n_Enekyori = 0, w_AG = [100, 95, 90, 86, 82, 79, 76, 74, 72, 71, 70], n_FeatSW = 0, n_LogSW = 0, n_itemSW = 0, n_SkillSW = 0, n_Skill3SW = 0, n_Skill4SW = 0, n_Skill5SW = 0, n_Skill6SW = 0, n_Skill7SW = 0, n_Skill8SW = 0, n_Skill9SW = 0, n_Skill10SW = 0, n_IjyouSW = 0, n_KyoukaSW = 0, wBCEDPch = 0, wLAch = 0, wCriTyuu = 0, wBTw1 = 0, n_TAKA_DMG = 0, TyouEnkakuSousa3dan = 0, not_use_card = 0;
var n_B_AtkSkill = 0,
    n_B_rangedAtk = 0,
    n_B_rangedMAtk = 0,
    BskillHitNum = 1,
    n_B_ignoreFlee = 0;
str_bSUBname = "", str_bSUB = "", SuperNoviceFullWeaponCHECK = 0, cast_kotei = 0, b = 0, n_PerHIT_DMG = 0, n_Delay = [0, 0, 0, 0, 0, 0, 0], wDelay = 0, n_tok = new Array;
for (var i = 0; 450 >= i; i++) n_tok[i] = 0;
var first_check = 0;
n_B = new Array, Last_DMG_A = [0, 0, 0], Last_DMG_B = [0, 0, 0], InnStr = new Array, SG_Special_HITnum = 0, SG_Special_DMG = [0, 0, 0], Item_or_Card = "Item", ItemCardNumberCheck = 142;
var SRV = 10,
    equip_restrict = 1,
    card_restrict = 0;
WeaponName = ["(Unarmed)", "Dagger", "Sword", "Two-handed Sword", "Spear", "Two-handed Spear", "Axe", "Two-handed Axe", "Mace", "Rod / Staff", "Bow", "Katar", "Book", "Knuckle", "Instrument", "Whip", "Huuma Shuriken", "Handgun", "Rifle", "Shotgun", "Gatling Gun", "Grenade Launcher"], ArrowOBJ = [
    [25, 0, "Arrow"],
    [30, 6, "Silver Arrow"],
    [30, 3, "Fire Arrow"],
    [30, 0, "Iron Arrow"],
    [30, 2, "Stone Arrow"],
    [30, 1, "Crystal Arrow"],
    [30, 4, "Arrow of Wind"],
    [30, 7, "Arrow of Shadow"],
    [30, 8, "Immaterial Arrow"],
    [30, 5, "Rusty Arrow"],
    [40, 0, "Steel Arrow"],
    [50, 0, "Oridecon Arrow"],
    [50, 6, "Arrow of Counter Evil"],
    [1, 1, "Frozen Arrow"],
    [1, 5, "Poison Arrow"],
    [10, 0, "Sharp Arrow"],
    [50, 6, "Holy Arrow"],
    [35, 0, "Hunting Arrow"],
    [45, 0, "Elven Arrow"],
    [1, 0, "Stun Arrow"],
    [1, 0, "Cursed Arrow"],
    [1, 0, "Flash Arrow"],
    [1, 0, "Sleep Arrow"],
    [1, 0, "Mute Arrow"]
], ArrowOBJbackup = [
    [25, 0, "Arrow"],
    [30, 6, "Silver Arrow"],
    [30, 3, "Fire Arrow"],
    [30, 0, "Iron Arrow"],
    [30, 2, "Stone Arrow"]
], BulletOBJ = [
    [10, 0, "Bullet"],
    [15, 6, "Silver Bullet"],
    [30, 0, "Blood Bullet"]
], GrenadeOBJ = [
    [50, 3, "Flare Sphere"],
    [50, 1, "Freezing Sphere"],
    [50, 4, "Lightning Sphere"],
    [50, 7, "Blind Sphere"],
    [50, 5, "Poison Sphere"]
], SyurikenOBJ = [
    [10, 0, "Shuriken"],
    [30, 0, "Numbus Shuriken"],
    [45, 0, "Flash Shuriken"],
    [70, 0, "Sharp Leaf Shuriken"],
    [100, 0, "Thorn Needle Shuriken"]
], KunaiOBJ = [
    [30, 3, "Heat Wave Kunai"],
    [30, 1, "Icicle Kunai"],
    [30, 4, "High Wind Kunai"],
    [30, 2, "Black Earth Kunai"],
    [30, 5, "Fell Poison Kunai"]
], JobEquipItemOBJ = [
    [0, 50, 90, 100, 999],
    [0, 1, 51, 101, 70, 71, 72, 74, 75, 78, 83, 84, 85, 86, 87, 90, 91, 999],
    [0, 1, 52, 102, 72, 74, 75, 78, 80, 83, 84, 85, 90, 91, 999],
    [0, 1, 53, 103, 71, 73, 74, 77, 78, 85, 89, 57, 999],
    [0, 1, 54, 104, 75, 76, 83, 89, 999],
    [0, 1, 55, 105, 71, 77, 89, 57, 999],
    [0, 1, 56, 106, 70, 71, 72, 73, 74, 75, 78, 83, 84, 85, 86, 90, 91, 999],
    [0, 1, 51, 61, 107, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 90, 91, 999],
    [0, 1, 52, 62, 108, 72, 74, 75, 78, 79, 81, 83, 84, 85, 90, 91, 999],
    [0, 1, 53, 63, 109, 71, 73, 74, 77, 78, 79, 81, 85, 89, 57, 999],
    [0, 1, 54, 64, 110, 75, 76, 79, 80, 83, 88, 89, 999],
    [0, 1, 55, 65, 111, 71, 77, 79, 89, 57, 999],
    [0, 1, 56, 66, 112, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 90, 91, 999],
    [0, 1, 51, 61, 113, 70, 71, 72, 74, 75, 78, 79, 83, 84, 85, 86, 87, 90, 91, 999],
    [0, 1, 52, 62, 114, 72, 74, 75, 76, 78, 79, 80, 83, 84, 85, 88, 91, 999],
    [0, 1, 53, 63, 115, 71, 73, 74, 77, 78, 79, 85, 89, 57, 999],
    [0, 1, 54, 64, 116, 74, 75, 76, 79, 83, 89, 999],
    [0, 1, 54, 64, 117, 74, 75, 76, 79, 83, 89, 999],
    [0, 1, 55, 65, 118, 71, 77, 79, 89, 57, 999],
    [0, 1, 56, 66, 119, 70, 71, 72, 73, 74, 75, 78, 79, 83, 84, 85, 86, 90, 91, 999],
    [0, 50, 90, 120, 999],
    [0, 1, 51, 61, 107, 121, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 999],
    [0, 1, 52, 62, 108, 122, 72, 74, 75, 78, 79, 81, 82, 83, 84, 85, 90, 91, 999],
    [0, 1, 53, 63, 109, 123, 71, 73, 74, 77, 78, 79, 81, 82, 85, 89, 57, 999],
    [0, 1, 54, 64, 110, 124, 75, 76, 79, 80, 82, 83, 88, 89, 999],
    [0, 1, 55, 65, 111, 125, 71, 77, 79, 82, 89, 57, 999],
    [0, 1, 56, 66, 112, 126, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91, 999],
    [0, 1, 51, 61, 113, 127, 70, 71, 72, 74, 75, 78, 79, 82, 83, 84, 85, 86, 87, 90, 91, 999],
    [0, 1, 52, 62, 114, 128, 72, 74, 75, 76, 78, 79, 80, 82, 83, 84, 85, 88, 91, 999],
    [0, 1, 53, 63, 115, 129, 71, 73, 74, 77, 78, 79, 82, 85, 89, 57, 123, 999],
    [0, 1, 54, 64, 116, 130, 74, 75, 76, 79, 82, 83, 89, 999],
    [0, 1, 54, 64, 117, 131, 74, 75, 76, 79, 82, 83, 89, 999],
    [0, 1, 55, 65, 118, 132, 71, 77, 79, 82, 89, 57, 999],
    [0, 1, 56, 66, 119, 133, 70, 71, 72, 73, 74, 75, 78, 79, 82, 83, 84, 85, 86, 90, 91, 999],
    [0],
    [0],
    [0],
    [0],
    [0],
    [0],
    [0],
    [0, 1, 141, 83, 84, 85, 86, 3001, 3051, 3070, 3072, 999],
    [0, 1, 142, 79, 83, 84, 85, 86, 87, 91, 2082, 3001, 3051, 3070, 3072, 3079, 999],
    [0, 1, 143, 55, 65, 71, 77, 79, 89, 111, 2082, 3001, 3079, 3089, 999],
    [0, 1, 144, 58, 52, 91, 3001, 3051, 3054, 3070, 3072, 3089, 999],
    [0, 1, 145, 59, 83, 3001, 3054, 3089, 999]
], SyuzokuOBJ = ["<b style='color:#9F9E9B'>Formless</b>", "<b style='color:#252520'>Undead</b>", "<b style='color:brown'>Brute</b>", "<b style='color:#88bd68'>Plant</b>", "<b style='color:green'>Insect</b>", "<b style='color:blue'>Fish</b>", "<b>Demon</b>", "<b style='color:orange'>Demi-Human</b>", "<b style='color:#CDCD40'>Angel</b>", "<b style='color:red'>Dragon</b>"], SyuzokuOBJ2 = ["Formless", "Undead", "Brute", "Plant", "Insect", "Fish", "Demon", "Demi-Human", "Angel", "Dragon"], ZokuseiOBJ = ["<b style='color:#A8A682'>Neutral</b>", "<b style='color:blue'>Water</b>", "<b style='color:brown'>Earth</b>", "<b style='color:red'>Fire</b>", "<b style='color:green'>Wind</b>", "<b style='color:#7B2488'>Poison</b>", "<b style='color:#CDCD40'>Holy</b>", "<b>Shadow</b>", "<b style='color:#9F9E9B'>Ghost</b>", "<b style='color:#252520'>Undead</b>"], ZokuseiOBJ2 = ["Neutral ", "Water ", "Earth ", "Fire ", "Wind ", "Poison ", "Holy ", "Shadow ", "Ghost ", "Undead "], SizeOBJ = ["Small", "Medium", "Large"], IjyouOBJ = ["Poison", "Stun", "Freeze", "Curse", "Blind", "Sleep", "Silence", "Chaos", "Bleeding", "Stone", "Weapon Break", "Armor Break"], EnergyCoatOBJ = ["0", "6% Reduction", "12% Reduction", "18% Reduction", "24% Reduction", "30% Reduction"], SpecialTypeOBJ = ["(None)", "Goblin", "Golem", "Guardian", "Kobold", "Orc", "Satan Morroc"], BossTypeOBJ = ["Normal", "Boss"], SubName = ["%", "s", "Damage", "Critical Damage", "Critical Rate", "Over 10000 Hits", "Too High to Calculate", "Immesurable", " x ", "Cast Time", "Off", "On"], Pets = [
    [800, 1e3, 200, 500, 1500, 800, 500, 800, 500, 1500, 200, 200, 200, 800, 800, 800, 800, 800, 800, 800, 50, 500, 2e3, 500, 50, 500, 200, 200, 1500, 500, 500, 200, 200, 500, 200, 500, 1e3, 500, 2e3, 1e3, 1e3, 2e3, 1500, 1500, 500, 1e3, 500, 1500, 1e3, 200, 200, 1e3, 800, 500, 2e3, 300],
    [13, 189, 229, 275, 184, 194, 193, 186, 205, 214, 329, 198, 466, 108, 109, 110, 111, 112, 113, 114, 115, 106, 398, 232, 23, 20, 345, 326, 311, 334, 283, 306, 299, 302, 216, 50, 262, 250, 238, 267, 271, 272, 320, 128, 73, 161, 167, 160, 153, 260, 123, 307, 129, 26, 399, 142]
], bBGC = ["#CDF", "#CCC", "#FDC", "#313", "#000", "#CDF"], hBGC1 = ["#355", "#57D", "#B44", "#622", "#444", "#0FB91E"], hBGC2 = ["#477", "#24A", "#A33", "#411", "#222", "#326B2D"], selBGC = ["#FC8", "#FC8", "#FC8", "#FC8", "#AAA", "#FC8"], ssBGC = ["#FFF", "#FFF", "#FFF", "#FC8", "#AAA", "#FFF"], sBGC = ["#466", "#36B", "#A33", "#626", "#000", "#466"], saBGC = ["#A52", "#811", "#3A3", "#A11", "#A11", "#A52"], mBGC = ["#FFF", "#FFF", "#FFF", "#C8F", "#444", "#FFF"], tBGC = ["#FFF", "#FFF", "#FFF", "#C8F", "#555", "#FFF"];
var sheet = function() {
    var _ = document.createElement("style");
    return _.appendChild(document.createTextNode("")), document.head.appendChild(_),
        _.sheet
}();
SWs3sw = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
