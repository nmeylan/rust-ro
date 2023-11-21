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



    zokusei = new Array();


    zokusei[1] = new Array(1,1,1,1,1,1,1,1,0.25,1);
    zokusei[2] = new Array(1,1,1,1,1,1,1,1,0,1);
    zokusei[3] = new Array(1,1,1,1,1,1,1,1,0,1);
    zokusei[4] = new Array(1,1,1,1,1,1,1,1,0,1);


    zokusei[11] = new Array(1,0.25,1,0.5,1.75,1,1,1,1,1);
    zokusei[12] = new Array(1,0,1,0.25,1.75,0.75,1,1,0.75,0.75);
    zokusei[13] = new Array(1,-0.25,1,0,2,0.5,1,1,0.5,0.5);
    zokusei[14] = new Array(1,-0.5,1,0,2,0.25,0.75,0.75,0.25,0.25);


    zokusei[21] = new Array(1,1,1,1.5,0.5,1.25,1,1,1,1);
    zokusei[22] = new Array(1,1,0.5,1.75,0.25,1.25,1,1,0.75,0.75);
    zokusei[23] = new Array(1,1,0,2,0,1,1,1,0.5,0.5);
    zokusei[24] = new Array(1,1,-0.25,2,0,0.75,0.75,0.75,0.25,0.25);


    zokusei[31] = new Array(1,1.5,0.5,0.25,1,1.25,1,1,1,1);
    zokusei[32] = new Array(1,1.75,0.25,0,1,1.25,1,1,0.75,0.75);
    zokusei[33] = new Array(1,2,0,-0.25,1,1,1,1,0.5,0.5);
    zokusei[34] = new Array(1,2,0,-0.5,1,0.75,0.75,0.75,0.25,0.25);



    zokusei[41] = new Array(1,0.5,1.5,1,0.25,1.25,1,1,1,1);
    zokusei[42] = new Array(1,0.25,1.75,1,0,1.25,1,1,0.75,0.75);
    zokusei[43] = new Array(1,0,2,1,-0.25,1,1,1,0.5,0.5);
    zokusei[44] = new Array(1,0,2,1,-0.5,0.75,0.75,0.75,0.25,0.25);



    zokusei[51] = new Array(1,1,1,1,1,0,1,0.5,1,0.5);
    zokusei[52] = new Array(1,1,1,1,1,0,1,0.25,0.75,0.25);
    zokusei[53] = new Array(1,1,1,1,1,0,1.25,0,0.5,0);
    zokusei[54] = new Array(1,0.75,0.75,0.75,0.75,0,1.25,-0.25,0.25,-0.25);


    zokusei[61] = new Array(1,0.75,0.75,0.75,0.75,0.75,0,1.25,0.75,1);
    zokusei[62] = new Array(1,0.5,0.5,0.5,0.5,0.5,-0.25,1.5,0.5,1.25);
    zokusei[63] = new Array(1,0.25,0.25,0.25,0.25,0.25,-0.5,1.75,0.25,1.5);
    zokusei[64] = new Array(1,0,0,0,0,0,-1,2,0,1.75);


    zokusei[71] = new Array(1,1,1,1,1,0.5,1.25,0,0.75,0);
    zokusei[72] = new Array(1,0.75,0.75,0.75,0.75,0.25,1.5,-0.25,0.5,0);
    zokusei[73] = new Array(1,0.5,0.5,0.5,0.5,0,1.75,-0.5,0.25,0);
    zokusei[74] = new Array(1,0.25,0.25,0.25,0.25,-0.25,2,-1,0,0);


    zokusei[81] = new Array(0.25,1,1,1,1,1,1,1,1.25,1);
    zokusei[82] = new Array(0.25,1,1,1,1,0.75,1,1,1.50,1);
    zokusei[83] = new Array(0,1,1,1,1,0.5,1,1,1.75,1);
    zokusei[84] = new Array(0,1,1,1,1,0.25,1,1,2,1);


    zokusei[91] = new Array(1,1,1,1.25,1,-0.25,1.5,-0.25,1,0);
    zokusei[92] = new Array(1,1,1,1.5,1,-0.5,1.75,-0.5,1.25,0);
    zokusei[93] = new Array(1,1.25,0.75,1.75,1,-0.75,2,-0.75,1.5,0);
    zokusei[94] = new Array(1,1.5,0.5,2,1,-1,2,-1,1.75,0);


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