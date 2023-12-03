var _____WB$wombat$assign$function_____ = function(name) {return (self._wb_wombat && self._wb_wombat.local_init && self._wb_wombat.local_init(name)) || self[name]; };
if (!self.__WB_pmw) { self.__WB_pmw = function(obj) { this.__WB_source = obj; return this; } }
{
    let window = _____WB$wombat$assign$function_____("window");
    let self = _____WB$wombat$assign$function_____("self");
    let document = _____WB$wombat$assign$function_____("document");
    let location = _____WB$wombat$assign$function_____("location");
    let top = _____WB$wombat$assign$function_____("top");
    let parent = _____WB$wombat$assign$function_____("parent");
    let frames = _____WB$wombat$assign$function_____("frames");
    let opener = _____WB$wombat$assign$function_____("opener");

    function tPlusDamCut(wPDC){
        if(wBTw1==0){
            if(TargetStatusFlags[6] && wLAch==0)//LA
                wPDC *= 2;
            if(TargetStatusFlags[17] && n_A_Weapon_element == 3)//ƒXƒpƒCƒ_ƒEƒFƒu
                wPDC *= 2;
            baizok = [110,114,117,119,120];
            if(n_A_PassSkill6[0] == 0 && n_A_PassSkill6[1] >= 1 && n_A_Weapon_element == 3)//‰Î
                wPDC = Math.floor(wPDC * baizok[n_A_PassSkill6[1]-1] /100);
            if(n_A_PassSkill6[0] == 1 && n_A_PassSkill6[1] >= 1 && n_A_Weapon_element == 1)//…
                wPDC = Math.floor(wPDC * baizok[n_A_PassSkill6[1]-1] /100);
            if(n_A_PassSkill6[0] == 2 && n_A_PassSkill6[1] >= 1 && n_A_Weapon_element == 4)//•—
                wPDC = Math.floor(wPDC * baizok[n_A_PassSkill6[1]-1] /100);
        }

        return wPDC;
    }

    function tPlusEnemyClick(){
        if(InWarOfEmperium){
            n_B = new Array();
            for(i=0;i<=26;i++)
                n_B[i] = MonsterOBJ[document.calcForm.B_Enemy.value][i];

            document.calcForm.B_LV.value = n_B[5];
            document.calcForm.B_AGI.value = n_B[8];
            document.calcForm.B_VIT.value = n_B[7];
            document.calcForm.B_INT.value = n_B[9];
            document.calcForm.B_LUK.value = n_B[11];
            document.calcForm.B_DEF.value = n_B[14];
            document.calcForm.B_MDEF.value = n_B[15];
        }}

    function tPlusTaiseiSyokia(){
    }

    function tPlusLucky(wPL){
        if(InWarOfEmperium){
            w = eval(document.calcForm.B_TAISEI6.value);
            w += (targetStatsArray[TARGET_STAT_LUK] / 10);

            w = wPL * (100-w) / 100;
            return w;
        }
        else{
            return wPL;
        }
    }

    function tPlusAG(){
        if(InWarOfEmperium){
            if(n_Enekyori!=2){
                wPAG = w_AG[eval(document.calcForm.B_TAISEI10.value)];
                w_Maxatk *= (wPAG /100);
                w_Minatk *= (wPAG /100);
                w_Aveatk *= (wPAG /100);
            }
        }}


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