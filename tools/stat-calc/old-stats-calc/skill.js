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

	SkillOBJ = [
		[ 0, 1, 'Basic Attack', 0 ] ,
		[ 1, 1, 'First Aid', 142 ] ,
		[ 2, 1, 'Play Dead', 143 ] ,
		[ 3, 10, ' Sword Mastery', 2 ] ,
		[ 4, 10, 'Two-Hand Sword Mastery', 0 ] ,
		[ 5, 10, 'Increased HP Recovery', 0 ] ,
		[ 6, 10, 'Bash', 5 ] ,
		[ 7, 10, 'Magnum Break', 7 ] ,
		[ 8, 10, 'Provoke', 6 ] ,
		[ 9, 10, 'Endure', 8 ] ,
		[ 10, 1, 'HP Recovery While Moving', 0 ] ,
		[ 11, 1, 'Fatal Blow', 145 ] ,
		[ 12, 1, 'Auto Berserk', 146 ] ,
		[ 13, 10, 'Double Attack', 48 ] ,
		[ 14, 10, 'Improve Dodge', 49 ] ,
		[ 15, 10, 'Steal', 50 ] ,
		[ 16, 10, 'Hiding', 51 ] ,
		[ 17, 10, 'Envenom', 52 ] ,
		[ 18, 1, 'Detoxify', 53 ] ,
		[ 19, 1, 'Sand Attack', 149 ] ,
		[ 20, 1, 'Back Slide', 150 ] ,
		[ 21, 1, 'Find Stone', 151 ] ,
		[ 22, 1, 'Stone Fling', 152 ] ,
		[ 23, 10, 'Divine Protection', 22 ] ,
		[ 24, 10, 'Demon Bane', 23 ] ,
		[ 25, 10, 'Heal', 28 ] ,
		[ 26, 1, 'Cure', 35 ] ,
		[ 27, 10, 'Increase AGI', 29 ] ,
		[ 28, 10, 'Decrease AGI', 30 ] ,
		[ 29, 10, 'Signum Crusis', 0 ] ,
		[ 30, 10, 'Angelus', 33 ] ,
		[ 31, 10, 'Blessing', 34 ] ,
		[ 32, 1, 'Pneuma', 25 ] ,
		[ 33, 1, 'Aqua Benedicta', 31 ] ,
		[ 34, 1, 'Ruwach', 24 ] ,
		[ 35, 2, 'Teleport', 26 ] ,
		[ 36, 4, 'Warp Portal', 27 ] ,
		[ 37, 1, 'Holy Light', 156 ] ,
		[ 38, 10, "Owl's Eye", 43 ] ,
		[ 39, 10, "Vulture's Eye", 44 ] ,
		[ 40, 10, 'Double Strafe', 46 ] ,
		[ 41, 10, 'Arrow Shower', 47 ] ,
		[ 42, 10, 'Improve Concentration', 45 ] ,
		[ 43, 1, 'Arrow Crafting', 147 ] ,
		[ 44, 1, 'Arrow Repel', 148 ] ,
		[ 45, 10, 'Increased SP Recovery', 0 ] ,
		[ 46, 10, 'Napalm Beat', 11 ] ,
		[ 47, 10, 'Soul Strike', 13 ] ,
		[ 48, 10, 'Safety Wall', 12 ] ,
		[ 49, 10, 'Stone Curse', 16 ] ,
		[ 50, 1, 'Sight', 10 ] ,
		[ 51, 10, 'Fire Bolt', 19 ] ,
		[ 52, 10, 'Fire Ball', 17 ] ,
		[ 53, 10, 'Fire Wall', 18 ] ,
		[ 54, 10, 'Cold Bolt', 14 ] ,
		[ 55, 10, 'Frost Diver', 15 ] ,
		[ 56, 10, 'Lightning Bolt', 20 ] ,
		[ 57, 10, 'Thunder Storm', 0 ] ,
		[ 58, 1, 'Energy Coat', 157 ] ,
		[ 59, 10, 'Enlarge Weight Limit', 36 ] ,
		[ 60, 10, 'Discount', 37 ] ,
		[ 61, 10, 'Overcharge', 38 ] ,
		[ 62, 10, 'Pushcart', 39 ] ,
		[ 63, 1, 'Item Appraisal', 40 ] ,
		[ 64, 10, 'Vending', 41 ] ,
		[ 65, 10, 'Mammonite', 42 ] ,
		[ 66, 1, 'Cart Revolution', 153 ] ,
		[ 67, 1, 'Change Cart', 154 ] ,
		[ 68, 1, 'Crazy Uproar', 155 ] ,
		[ 69, 10, 'Spear Mastery', 55 ] ,
		[ 70, 10, 'Pierce', 56 ] ,
		[ 71, 10, 'Spear Stab', 58 ] ,
		[ 72, 5, 'Spear Boomerang', 59 ] ,
		[ 73, 10, 'Brandish Spear', 57 ] ,
		[ 74, 10, 'Twohand Quicken', 60 ] ,
		[ 75, 5, 'Auto Counter', 0 ] ,
		[ 76, 10, 'Bowling Bash', 62 ] ,
		[ 77, 1, 'Peco Peco Ride', 0 ] ,
		[ 78, 6, 'Cavalier Mastery', 64 ] ,
		[ 79, 5, 'Righthand Mastery', 132 ] ,
		[ 80, 5, 'Lefthand Mastery', 133 ] ,
		[ 81, 10, 'Katar Mastery', 134 ] ,
		[ 82, 10, 'Cloaking', 135 ] ,
		[ 83, 10, 'Sonic Blow', 136 ] ,
		[ 84, 5, 'Grimtooth', 137 ] ,
		[ 85, 10, 'Enchant Poison', 138 ] ,
		[ 86, 10, 'Poison React (Counter)', 0 ] ,
		[ 87, 10, 'Venom Dust', 140 ] ,
		[ 88, 10, 'Venom Splasher', 141 ] ,
		[ 89, 10, 'Mace Mastery', 65 ] ,
		[ 90, 5, 'Impositio Manus', 66 ] ,
		[ 91, 3, 'Suffragium', 67 ] ,
		[ 92, 5, 'Aspersio', 68 ] ,
		[ 93, 5, 'BS Sacrimenti', 0 ] ,
		[ 94, 10, 'Sanctuary', 70 ] ,
		[ 95, 1, 'Status Recovery', 72 ] ,
		[ 96, 4, 'Slow Poison', 71 ] ,
		[ 97, 4, 'Ressurection', 0 ] ,
		[ 98, 10, 'Kyrie Eleison', 73 ] ,
		[ 99, 5, 'Magnificat', 74 ] ,
		[ 100, 5, 'Gloria', 75 ] ,
		[ 101, 10, 'Lex Divina', 76 ] ,
		[ 102, 10, 'Turn Undead', 77 ] ,
		[ 103, 1, 'Lex Aeterna', 78 ] ,
		[ 104, 10, 'Magnus Exorcismus', 79 ] ,
		[ 105, 5, 'Skid Trap', 115 ] ,
		[ 106, 5, 'Land Mine', 116 ] ,
		[ 107, 5, 'Ankle Snare', 117 ] ,
		[ 108, 5, 'Flasher', 120 ] ,
		[ 109, 5, 'Shockwave Trap', 118 ] ,
		[ 110, 5, 'Sandman', 119 ] ,
		[ 111, 5, 'Freezing Trap', 121 ] ,
		[ 112, 5, 'Blast Mine', 122 ] ,
		[ 113, 5, 'Claymore Trap', 123 ] ,
		[ 114, 1, 'Remove Trap', 124 ] ,
		[ 115, 1, 'Talkie Box', 125 ] ,
		[ 116, 10, 'Beast Bane', 126 ] ,
		[ 117, 1, 'Falconry Mastery', 127 ] ,
		[ 118, 5, 'Blitz Beat', 129 ] ,
		[ 119, 10, 'Steel Crow', 128 ] ,
		[ 120, 4, 'Detect', 130 ] ,
		[ 121, 5, 'Spring Trap', 131 ] ,
		[ 122, 10, 'Fire Pillar', 80 ] ,
		[ 123, 1, 'Sense', 93 ] ,
		[ 124, 10, 'Sightrasher', 81 ] ,
		[ 125, 10, 'Meteor Storm', 83 ] ,
		[ 126, 10, 'Jupitel Thunder', 84 ] ,
		[ 127, 10, 'Lord of Vermillion', 0 ] ,
		[ 128, 5, 'Water Ball', 86 ] ,
		[ 129, 10, 'Ice Wall', 87 ] ,
		[ 130, 10, 'Frost Nova', 88 ] ,
		[ 131, 10, 'Storm Gust', 89 ] ,
		[ 132, 5, 'Earth Spike', 90 ] ,
		[ 133, 5, "Heaven's Drive", 91 ] ,
		[ 134, 5, 'Quagmire', 92 ] ,
		[ 135, 5, 'Iron Tempering', 94 ] ,
		[ 136, 5, 'Steel Tempering', 95 ] ,
		[ 137, 5, 'Enchantedstone Craft', 0 ] ,
		[ 138, 5, 'Oridecon Research', 97 ] ,
		[ 139, 3, 'Smith Dagger', 98 ] ,
		[ 140, 3, 'Smith Sword', 99 ] ,
		[ 141, 3, 'Smith Two-handed Sword', 100 ] ,
		[ 142, 3, 'Smith Axe', 101 ] ,
		[ 143, 3, 'Smith Mace', 102 ] ,
		[ 144, 3, 'Smith Knucklebrace', 103 ] ,
		[ 145, 3, 'Smith Spear', 104 ] ,
		[ 146, 1, 'Hilt Binding', 105 ] ,
		[ 147, 1, 'Ore Discovery', 106 ] ,
		[ 148, 10, 'Weaponry Research', 107 ] ,
		[ 149, 1, 'Weapon Repair', 108 ] ,
		[ 150, 5, 'Skin Tempering', 109 ] ,
		[ 151, 5, 'Hammer Fall', 110 ] ,
		[ 152, 5, 'Andrenaline Rush', 0 ] ,
		[ 153, 5, 'Weapon Perfection', 112 ] ,
		[ 154, 5, 'Power-Thrust', 113 ] ,
		[ 155, 5, 'Power Maximize', 0 ] ,
		[ 156, 10, 'Faith', 248 ] ,
		[ 157, 10, 'Guard', 249 ] ,
		[ 158, 5, 'Smite', 250 ] ,
		[ 159, 5, 'Shield Boomerang', 251 ] ,
		[ 160, 10, 'Shield Reflect', 252 ] ,
		[ 161, 10, 'Holy Cross', 253 ] ,
		[ 162, 10, 'Grand Cross', 254 ] ,
		[ 163, 5, 'Sacrifice', 255 ] ,
		[ 164, 5, 'Resistant Souls', 256 ] ,
		[ 165, 5, 'Defending Aura', 257 ] ,
		[ 166, 10, 'Spear Quicken', 258 ] ,
		[ 167, 10, 'Snatch', 219 ] ,
		[ 168, 10, 'Mug', 211 ] ,
		[ 169, 10, 'Back Stab', 212 ] ,
		[ 170, 5, 'Stalk', 213 ] ,
		[ 171, 5, 'Sightless Mind', 214 ] ,
		[ 172, 5, 'Divest Weapon', 215 ] ,
		[ 173, 5, 'Divest Shield', 216 ] ,
		[ 174, 5, 'Divest Armor', 217 ] ,
		[ 175, 5, 'Divest Helm', 218 ] ,
		[ 176, 5, 'Gank', 210 ] ,
		[ 177, 1, 'Scribble', 220 ] ,
		[ 178, 5, 'Piece', 221 ] ,
		[ 179, 1, 'Remover', 222 ] ,
		[ 180, 1, 'Slyness', 223 ] ,
		[ 181, 5, 'Haggle', 224 ] ,
		[ 182, 10, 'Intimidate', 225 ] ,
		[ 183, 10, 'Iron Fist', 0 ] ,
		[ 184, 5, 'Spiritual Cadence', 260 ] ,
		[ 185, 5, 'Summon Spirit Sphere', 261 ] ,
		[ 186, 1, 'Spiritual Sphere Absorption', 0 ] ,
		[ 187, 10, 'Raging Trifecta Blow', 263 ] ,
		[ 188, 5, 'Raging Quadruple Blow', 272 ] ,
		[ 189, 5, 'Raging Thrust', 273 ] ,
		[ 190, 1, 'Snap', 264 ] ,
		[ 191, 10, 'Flee', 0 ] ,
		[ 192, 5, 'Throw Spirit Spheres (# Hits = # of Spirit Spheres)', 267 ] ,
		[ 193, 5, 'Occult Impaction', 266 ] ,
		[ 194, 5, 'Root', 269 ] ,
		[ 195, 5, 'Fury', 270 ] ,
		[ 196, 5, 'Mental Strength', 268 ] ,
		[ 197, 5, 'Guillotine Fist', 0 ] ,
		[ 198, 10, 'Music Lessons', 315 ] ,
		[ 199, 5, 'Melody Strike', 316 ] ,
		[ 200, 5, 'Unchained Serenade', 317 ] ,
		[ 201, 5, 'Unbarring Octave', 318 ] ,
		[ 202, 10, 'Perfect Tablature', 319 ] ,
		[ 203, 10, 'Impressive Rift', 0 ] ,
		[ 204, 10, 'Magic Strings', 321 ] ,
		[ 205, 10, 'Song of Lutie', 322 ] ,
		[ 206, 10, 'Dance Lessons', 323 ] ,
		[ 207, 5, 'Slinging Arrow', 324 ] ,
		[ 208, 5, 'Hip Shaker', 325 ] ,
		[ 209, 5, 'Dazzler', 326 ] ,
		[ 210, 10, 'Focus Ballet', 327 ] ,
		[ 211, 10, 'Slow Grace', 328 ] ,
		[ 212, 10, 'Lady Luck', 329 ] ,
		[ 213, 10, "Gypsy's Kiss", 330 ] ,
		[ 214, 1, 'Amp', 304 ] ,
		[ 215, 1, 'Encore', 305 ] ,
		[ 216, 1, 'Lullaby', 306 ] ,
		[ 217, 5, 'Mental Sensing', 307 ] ,
		[ 218, 1, 'Down Tempo', 308 ] ,
		[ 219, 5, 'Battle Theme', 309 ] ,
		[ 220, 5, 'Harmonic Lick', 310 ] ,
		[ 221, 1, 'Classical Pluck', 311 ] ,
		[ 222, 1, 'Power Cord', 0 ] ,
		[ 223, 5, 'Acoustic Rhythm', 313 ] ,
		[ 224, 10, 'Study', 274 ] ,
		[ 225, 5, 'Cast Cancel', 275 ] ,
		[ 226, 5, 'Magic Rod', 276 ] ,
		[ 227, 5, 'Spell Breaker', 277 ] ,
		[ 228, 10, 'Free Cast (N/A)', 0 ] ,
		[ 229, 10, 'Hindsight (N/A)', 0 ] ,
		[ 230, 5, 'Endow Blaze', 280 ] ,
		[ 231, 5, 'Endow Tsunami', 281 ] ,
		[ 232, 5, 'Endow Tornado', 282 ] ,
		[ 233, 5, 'Endow Quake', 283 ] ,
		[ 234, 5, 'Dragonology', 284 ] ,
		[ 235, 5, 'Volcano', 285 ] ,
		[ 236, 5, 'Deluge', 286 ] ,
		[ 237, 5, 'Whirlwind', 287 ] ,
		[ 238, 5, 'Magnetic Earth', 288 ] ,
		[ 239, 5, 'Dispell', 289 ] ,
		[ 240, 10, 'Hocus Pocus', 0 ] ,
		[ 241, 10, 'Axe Mastery', 226 ] ,
		[ 242, 10, 'Potion Research', 227 ] ,
		[ 243, 10, 'Prepare Potion', 228 ] ,
		[ 244, 5, 'Acid Terror', 230 ] ,
		[ 245, 5, 'Potion Pitcher', 0 ] ,
		[ 246, 5, 'Summon Flora', 232 ] ,
		[ 247, 5, 'Summon Marine Sphere', 233 ] ,
		[ 248, 5, 'Bomb', 229 ] ,
		[ 249, 5, 'Alchemical Weapon', 234 ] ,
		[ 250, 5, 'Synthesized Shield', 235 ] ,
		[ 251, 5, 'Synthetic Armor', 236 ] ,
		[ 252, 5, 'Biochemical Helm', 237 ] ,
		[ 253, 1, 'Fury (SuperNovice)', 0 ] ,
		[ 254, 5, 'Aura Blade', 355 ] ,
		[ 255, 10, 'Parrying', 356 ] ,
		[ 256, 5, 'Spear Dynamo', 0 ] ,
		[ 257, 1, 'Tension Relax', 0 ] ,
		[ 258, 1, 'Frenzy', 359 ] ,
		[ 259, 5, 'Clashing Spiral', 0 ] ,
		[ 260, 5, 'Traumatic Blow', 398 ] ,
		[ 261, 10, 'Vital Strike', 399 ] ,
		[ 262, 5, 'Advanced Katar Mastery', 376 ] ,
		[ 263, 10, 'Soul Destroyer', 379 ] ,
		[ 264, 10, 'Meteor Assault', 406 ] ,
		[ 265, 1, 'Create Deadly Poison', 407 ] ,
		[ 266, 5, 'Enchant Deadly Poison', 378 ] ,
		[ 267, 5, 'Assumptio', 361 ] ,
		[ 268, 5, 'Basilica', 362 ] ,
		[ 269, 10, 'Mediatio', 0 ] ,
		[ 270, 10, 'True Sight', 0 ] ,
		[ 271, 5, 'Falcon Eyes', 380 ] ,
		[ 272, 5, 'Sharp Shooting (Temp)', 0 ] ,
		[ 273, 10, 'Wind Walk', 0 ] ,
		[ 274, 10, 'Soul Drain', 364 ] ,
		[ 275, 1, 'Stave Crasher', 365 ] ,
		[ 276, 10, 'Mystical Amplification', 366 ] ,
		[ 277, 5, 'Napalm Vulcan', 400 ] ,
		[ 278, 10, 'Shattering Strike', 384 ] ,
		[ 279, 3, 'Coin Craft', 0 ] ,
		[ 280, 3, 'Nugget Craft', 0 ] ,
		[ 281, 1, 'Cart Boost', 387 ] ,
		[ 282, 1, 'Battle Machine Craft', 0 ] ,
		[ 283, 5, 'Gloria Domini', 367 ] ,
		[ 284, 5, "Martyr's Reconing", 0 ] ,
		[ 285, 10, 'Battle Chant', 369 ] ,
		[ 286, 5, 'Chase Walk', 389 ] ,
		[ 287, 5, 'Counter Instinct', 390 ] ,
		[ 288, 5, 'Raging Palm Strike', 370 ] ,
		[ 289, 5, 'Glacier Fist', 371 ] ,
		[ 290, 10, 'Chain Crush Combo', 372 ] ,
		[ 291, 1, 'Zen', 401 ] ,
		[ 292, 10, 'Arrow Vulcan', 394 ] ,
		[ 293, 5, 'Sheltering Bliss', 395 ] ,
		[ 294, 1, 'Marionette Control', 396 ] ,
		[ 295, 5, 'Indulge', 373 ] ,
		[ 296, 1, 'Soul Exhale', 374 ] ,
		[ 297, 5, 'Soul Siphon', 375 ] ,
		[ 298, 5, 'Mind Breaker', 402 ] ,
		[ 299, 5, 'Alchemy', 392 ] ,
		[ 300, 5, 'Potion Synthesis', 393 ] ,
		[
			301,
			1,
			'<Font size=2>Add the delay time when attacking for triple attack</Font>',
			0
		] ,
		[ 302, 1, 'Tomahawk Throwing', 0 ] ,
		[ 303, 1, 'Pulse Strike (Temp)', 0 ] ,
		[ 304, 1, '', 0 ] ,
		[ 305, 1, '', 0 ] ,
		[ 306, 1, '', 0 ] ,
		[ 307, 1, '', 0 ] ,
		[ 308, 1, '', 0 ] ,
		[ 309, 1, '', 0 ] ,
		[ 310, 1, '', 0 ] ,
		[ 311, 1, '', 0 ] ,
		[ 312, 1, '', 0 ] ,
		[ 313, 1, '', 0 ] ,
		[ 314, 1, '', 0 ] ,
		[ 315, 1, '', 0 ] ,
		[ 316, 1, '', 0 ] ,
		[ 317, 1, '', 0 ] ,
		[ 318, 1, '', 0 ] ,
		[ 319, 1, '', 0 ] ,
		[ 320, 10, 'Water Ball', 86 ] ,
		[ 321, 5, 'Guillotine Fist (MaxSP-1)', 0 ] ,
		[ 322, 1, 'Forsight', 0 ] ,
		[ 323, 1, 'Deadly Poison (Consumed)', 0 ] ,
		[ 324, 5, 'Rapid Smiting', 0 ] ,
		[ 325, 5, 'Gravity Field', 0 ] ,
		[ 326, 10, 'High Speed Cart Ram', 0 ] ,
		[ 327, 5, 'Maximum Power-Thust', 0 ] ,
		[ 328, 10, 'Acid Demonstration', 490 ] ,
		[ 329, 10, 'Sprint (Unarmed Mastery)', 0 ] ,
		[ 330, 1, 'Tornado Stance', 412 ] ,
		[ 331, 7, 'Tornado Kick', 413 ] ,
		[ 332, 1, 'Heel Drop Stance', 414 ] ,
		[ 333, 7, 'Heel Drop', 415 ] ,
		[ 334, 1, 'Roundhouse Stance', 416 ] ,
		[ 335, 7, 'Roundouse', 0 ] ,
		[ 336, 1, 'Counter Kick Stance', 418 ] ,
		[ 337, 7, 'Counter Kick', 419 ] ,
		[ 338, 2, 'Tumbling (Untested))', 0 ] ,
		[ 339, 7, 'Flying Kick (Normal)', 0 ] ,
		[ 340, 10, 'Peaceful Break', 422 ] ,
		[ 341, 10, 'Happy Break', 423 ] ,
		[ 342, 5, 'Kihop', 424 ] ,
		[ 343, 5, 'Leap', 0 ] ,
		[ 344, 1, 'Taekwon Mission', 493 ] ,
		[ 345, 1, 'Taekwon Ranker', 0 ] ,
		[ 346, 1, 'Mild Wind', 425 ] ,
		[ 347, 3, 'Solar, Lunar, and Stellar Perception', 0 ] ,
		[ 348, 3, 'Solar Heat', 0 ] ,
		[ 349, 3, 'Lunar Heat', 0 ] ,
		[ 350, 3, 'Stellar Heat', 0 ] ,
		[ 351, 3, 'Solar, Lunar, and Stellar Opposition', 0 ] ,
		[ 352, 3, 'Solar Wrath', 0 ] ,
		[ 353, 3, 'Lunar Wrath', 0 ] ,
		[ 354, 3, 'Stellar Wrath', 0 ] ,
		[ 355, 4, 'Solar Protection', 0 ] ,
		[ 356, 4, 'Lunar Protection', 0 ] ,
		[ 357, 4, 'Stellar Protection', 0 ] ,
		[ 358, 5, 'Solar Blessings', 0 ] ,
		[ 359, 5, 'Lunar Blessings', 0 ] ,
		[ 360, 5, 'Stellar Blessings', 0 ] ,
		[ 361, 10, 'Solar, Lunar, and Stellar Shadow', 0 ] ,
		[ 362, 3, 'Solar, Lunar, and Stellar Team-Up', 0 ] ,
		[ 363, 10, 'Solar, Lunar, and Stellar Courier', 0 ] ,
		[ 364, 1, 'Solar, Lunar, and Stellar Union', 0 ] ,
		[ 365, 1, 'Solar, Lunar, and Stellar Miracle', 0 ] ,
		[ 366, 1, 'Solar, Lunar, and Stellar Angel', 0 ] ,
		[ 367, 5, '(Solar/Lunar/Stellar) Blessings (Untested)', 0 ] ,
		[ 368, 7, 'Kaizel', 462 ] ,
		[ 369, 7, 'Kaahi', 463 ] ,
		[ 370, 3, 'Kaupe', 464 ] ,
		[ 371, 7, 'Kaite', 465 ] ,
		[ 372, 7, 'Kaina', 466 ] ,
		[ 373, 7, 'Estin', 467 ] ,
		[ 374, 7, 'Estun', 468 ] ,
		[ 375, 10, 'Esma', 469 ] ,
		[ 376, 7, 'Eswoo', 470 ] ,
		[ 377, 3, 'Eska', 472 ] ,
		[ 378, 3, 'Eske', 471 ] ,
		[ 379, 1, 'Sprint (STR + State)', 0 ] ,
		[ 380, 11, 'Party Members (Kihop Bonus', 0 ] ,
		[ 381, 1, 'Sonic Acceleration', 1003 ] ,
		[ 382, 1, 'Excruciating Palm', 0 ] ,
		[ 383, 1, 'Close Confine', 1005 ] ,
		[ 384, 5, 'Shield Boomerang (SoulLinked)', 0 ] ,
		[ 385, 1, 'Super Novice Spirit (Soul Link)', 0 ] ,
		[ 386, 1, 'One Hand Quicken (Soul Linked)', 0 ] ,
		[ 387, 1, 'Holy Light (Soul Linked)', 0 ] ,
		[ 388, 10, 'Sonic Blow (Soul Linked)', 0 ] ,
		[ 389, 1, 'Full Andrenaline Rush', 0 ] ,
		[ 390, 1, 'Hunter Spirit (Soul Link)', 0 ] ,
		[ 391, 1, 'Beast Strafing', 499 ] ,
		[ 392, 1, '1st Transcendent Spirit', 0 ] ,
		[ 393, 10, 'Dagger Throwing Practice', 0 ] ,
		[ 394, 10, 'Throw Dagger', 0 ] ,
		[ 395, 5, 'Throw Kunai', 524 ] ,
		[ 396, 5, 'Throw Huuma Shuriken', 525 ] ,
		[ 397, 10, 'Throw Coins', 0 ] ,
		[ 398, 5, 'Flip Tatami', 0 ] ,
		[ 399, 5, 'Shadow Leap', 529 ] ,
		[ 400, 10, 'Haze Slasher', 0 ] ,
		[ 401, 5, 'Shadow Slash', 530 ] ,
		[ 402, 5, 'Cicada Skin Shed', 0 ] ,
		[ 403, 10, 'Mirror Image', 532 ] ,
		[ 404, 5, 'Ninja Aura', 0 ] ,
		[ 405, 10, 'Final Strike', 544 ] ,
		[ 406, 10, 'Ninja Mastery', 0 ] ,
		[ 407, 10, 'Flaming Petals', 0 ] ,
		[ 408, 10, 'Blaze Shield', 0 ] ,
		[ 409, 5, 'Exploding Dragon', 0 ] ,
		[ 410, 10, 'Freezing Spear', 0 ] ,
		[ 411, 10, 'Watery Evasion', 0 ] ,
		[ 412, 5, 'Snow Flake Draft', 0 ] ,
		[ 413, 10, 'Wind Blade', 540 ] ,
		[ 414, 5, 'Lightning Jolt', 0 ] ,
		[ 415, 5, 'First Wind', 0 ] ,
		[ 416, 10, 'Coin Flip', 0 ] ,
		[ 417, 1, 'Coing Fling', 0 ] ,
		[ 418, 1, 'Triple Action', 502 ] ,
		[ 419, 1, "Bull's Eye", 0 ] ,
		[ 420, 1, 'Last Stand', 0 ] ,
		[ 421, 1, "Gunslinger's Panic", 0 ] ,
		[ 422, 1, 'Increase Accuracy', 0 ] ,
		[ 423, 1, 'Magical Bullet', 507 ] ,
		[ 424, 1, 'Cracker', 508 ] ,
		[ 425, 10, 'Single Action', 509 ] ,
		[ 426, 10, 'Snake Eyes', 0 ] ,
		[ 427, 10, 'Chain Action', 511 ] ,
		[ 428, 10, 'Trigger Happy Shot', 0 ] ,
		[ 429, 10, 'Desperado (Single Hit)', 0 ] ,
		[ 430, 10, 'Tracking', 512 ] ,
		[ 431, 5, 'Disarm', 513 ] ,
		[ 432, 5, 'Wounding Shot', 0 ] ,
		[ 433, 10, 'Gatling Fever', 517 ] ,
		[ 434, 10, 'Crowd Control Shot', 0 ] ,
		[ 435, 10, 'Full Blast', 0 ] ,
		[ 436, 10, 'Spread Shot', 0 ] ,
		[ 437, 10, 'Gunslinger Mine', 0 ] ,
		[ 438, 10, 'Final Strike (MaxHP-1)', 0 ]
	];

	JobSkillPassOBJ = [
		[999,999,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[  3,  4, 12,  9,999,999,999,999,999,999,999,999,999,999,999],
		[ 13, 14,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 23, 24,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 38, 39, 42,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 58,999,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 68,999,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[  3,  4, 12, 69, 74, 78,386,  9,999,999,999,999,999,999,999],
		[ 13, 14, 79, 80, 81,381,999,999,999,999,999,999,999,999,999],
		[ 23, 24, 89,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 38, 39, 42,116,118,119,390,999,999,999,999,999,999,999,999],
		[ 58,999,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 68,146,148,150,152,153,154,155,389,999,999,999,999,999,999],
		[  3,  4, 12,156, 69,166, 24, 23,157, 78,165,  9,999,999,999],
		[ 13, 14,  3, 39,187,383,999,999,999,999,999,999,999,999,999],
		[ 23, 24,183,185,187,191,195,196,301,999,999,999,999,999,999],
		[ 38, 39, 42,198,999,999,999,999,999,999,999,999,999,999,999],
		[ 38, 39, 42,206,999,999,999,999,999,999,999,999,999,999,999],
		[ 58,224,228,229,234,999,999,999,999,999,999,999,999,999,999],
		[ 68,241,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[  3, 13, 14, 23, 24, 38, 39, 42,253,385,  9,999,999,999,999],
		[  3,  4, 12, 69, 74, 78,254,256,258,255,386,  9,999,999,999],
		[ 13, 14, 79, 80, 81,262,266,323,381,999,999,999,999,999,999],
		[ 23, 24, 89,269,999,999,999,999,999,999,999,999,999,999,999],
		[ 38, 39, 42,116,118,119,270,273,390,999,999,999,999,999,999],
		[ 58,274,276,999,999,999,999,999,999,999,999,999,999,999,999],
		[ 68,146,148,150,152,153,154,155,327,389,999,999,999,999,999],
		[  3,  4, 12,156, 69,166, 24, 23,157, 78,165,  9,999,999,999],
		[ 13, 14,  3, 39,286,287,187,383,999,999,999,999,999,999,999],
		[ 23, 24,183,185,187,191,195,196,301,999,999,999,999,999,999],
		[ 38, 39, 42,198,999,999,999,999,999,999,999,999,999,999,999],
		[ 38, 39, 42,206,999,999,999,999,999,999,999,999,999,999,999],
		[ 58,224,228,229,234,322,999,999,999,999,999,999,999,999,999],
		[ 68,241,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[329,379,338,342,380,345,999,999,999,999,999,999,999,999,999],
		[329,379,338,342,380,352,353,354,355,356,357,367,361,364,365,999],
		[329,338,342,380,372,379,999,999,999,999,999,999,999,999,999],
		[393,404,999,999,999,999,999,999,999,999,999,999,999,999,999],
		[425,426,427,416,420,421,422,433,999,999,999,999,999,999,999],
	];

	JobSkillActiveOBJ = [
		[  0,999],
		[  0,  6,  7,999],
		[  0, 17, 19,999],
		[  0, 25, 37,999],
		[  0, 40, 41, 44,999],
		[  0, 51, 54, 56, 52, 53, 55, 57, 46, 47,999],
		[  0, 65, 66,999],
		[  0,  6,  7, 70, 71, 72, 73, 76,999],
		[  0, 17, 83, 84, 88, 86, 19,388,999],
		[  0, 25, 37,102,104, 94,387,999],
		[  0, 40, 41, 44,118,106,112,113,391,999],
		[  0, 51, 54, 56, 52, 53, 55, 57, 46, 47,122,124,125,126,127,128,130,131,132,133,999],
		[  0, 65, 66,999],
		[  0,  6,  7,158,159,161,162, 25,384,999],
		[  0,169,171, 40, 17,6,7,25,76,65,161,162,158,159,46,47,51,52,53,54,55,56,57,122,124,125,126,127,320,131,132,133,37,41,44,102,104,106,112,113,244,248,19,193,192,382,407,408,409,410,412,413,414,415,418,419,423,302,999],
		[  0,188,189,192,193,197,321, 25, 37,382,999],
		[  0, 40,199, 41, 44,999],
		[  0, 40,207, 41, 44,999],
		[  0, 51, 54, 56, 52, 53, 55, 57, 46, 47,132,133,999],
		[  0, 65, 66,244,248,999],
		[  0,  6,  7, 17, 65, 25, 51, 54, 56, 52, 53, 55, 57, 46, 47,999],
		[  0,  6,  7, 70, 71, 72, 73, 76,259,260,261,999],
		[  0, 17, 83, 84, 88,263,264, 86, 19,388,999],
		[  0, 25, 37,102,104, 94,387,999],
		[  0, 40, 41, 44,118,106,112,113,271,272,391,999],
		[  0, 51, 54, 56, 52, 53, 55, 57, 46, 47,122,124,125,126,127,128,130,131,132,133,275,277,325,999],
		[  0, 65, 66,326,999],
		[  0,  6,  7,158,159,161,162,283,284,324, 25,384,999],
		[  0,169,171, 40, 17,6,7,25,76,65,161,162,158,159,46,47,51,52,53,54,55,56,57,122,124,125,126,127,320,131,132,133,37,41,44,102,104,106,112,113,244,248, 19,193,192,382,407,408,409,410,412,413,414,415,418,419,423,302,999],
		[  0,188,189,192,193,197,321, 25, 37,288,289,290,382,999],
		[  0, 40,199, 41, 44,292,999],
		[  0, 40,207, 41, 44,292,999],
		[  0, 51, 54, 56, 52, 53, 55, 57, 46, 47,132,133,999],
		[  0, 65, 66,244,248,328,999],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[0],
		[  0,331,333,335,337,339,132,999],
		[  0,331,333,335,337,339,348,349,350,132,999],
		[  0,373,374,375,132,999],
		[  0,407,408,409,410,412,413,414,415,394,395,396,405,438,999],
		[  0,418,419,423,428,429,430,431,432,434,435,436,437,999],
	];

}
/*
     FILE ARCHIVED ON 14:38:08 Nov 11, 2008 AND RETRIEVED FROM THE
     INTERNET ARCHIVE ON 06:58:30 Nov 21, 2023.
     JAVASCRIPT APPENDED BY WAYBACK MACHINE, COPYRIGHT INTERNET ARCHIVE.

     ALL OTHER CONTENT MAY ALSO BE PROTECTED BY COPYRIGHT (17 U.S.C.
     SECTION 108(a)(3)).
*/
/*
playback timings (ms):
  captures_list: 67.595
  exclusion.robots: 0.162
  exclusion.robots.policy: 0.145
  cdx.remote: 0.104
  esindex: 0.018
  LoadShardBlock: 33.115 (3)
  PetaboxLoader3.datanode: 87.126 (4)
  load_resource: 115.583
  PetaboxLoader3.resolve: 34.062
*/