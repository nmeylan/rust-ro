use enum_macro::{WithNumberValue, WithStringValue};

use crate::enums::{EnumWithNumberValue, EnumWithStringValue};

#[derive(WithNumberValue, WithStringValue, Debug, Copy, Clone, PartialEq, Eq)]
pub enum ClientEffectIcon {
    #[value = 0]
    Provoke,
    #[value = 1]
    Endure,
    #[value = 2]
    Twohandquicken,
    #[value = 3]
    Concentration,
    #[value = 4]
    Hiding,
    #[value = 5]
    Cloaking,
    #[value = 6]
    Enchantpoison,
    #[value = 7]
    Poisonreact,
    #[value = 8]
    Quagmire,
    #[value = 9]
    Angelus,
    #[value = 10]
    Blessing,
    #[value = 11]
    Crucis,
    #[value = 12]
    IncAgi,
    #[value = 13]
    DecAgi,
    #[value = 14]
    Slowpoison,
    #[value = 15]
    Impositio,
    #[value = 16]
    Suffragium,
    #[value = 17]
    Aspersio,
    #[value = 18]
    Benedictio,
    #[value = 19]
    Kyrie,
    #[value = 20]
    Magnificat,
    #[value = 21]
    Gloria,
    #[value = 22]
    Lexaeterna,
    #[value = 23]
    Adrenaline,
    #[value = 24]
    Weaponperfect,
    #[value = 25]
    Overthrust,
    #[value = 26]
    Maximize,
    #[value = 29]
    Trickdead,
    #[value = 30]
    Shout,
    #[value = 31]
    Energycoat,
    #[value = 32]
    Brokenarmor,
    #[value = 33]
    Brokenweapon,
    #[value = 34]
    Illusion,
    #[value = 35]
    Weightover50,
    #[value = 36]
    Weightover90,
    #[value = 37]
    AtthastePotion1,
    #[value = 38]
    AtthastePotion2,
    #[value = 39]
    AtthastePotion3,
    #[value = 40]
    AtthasteInfinity,
    #[value = 41]
    MovhastePotion,
    #[value = 42]
    MovhasteInfinity,
    #[value = 43]
    Autocounter,
    #[value = 44]
    Splasher,
    #[value = 45]
    Anklesnare,
    #[value = 49]
    Barrier,
    #[value = 50]
    Noequipweapon,
    #[value = 51]
    Noequipshield,
    #[value = 52]
    Noequiparmor,
    #[value = 53]
    Noequiphelm,
    #[value = 54]
    Protectweapon,
    #[value = 55]
    Protectshield,
    #[value = 56]
    Protectarmor,
    #[value = 57]
    Protecthelm,
    #[value = 58]
    Autoguard,
    #[value = 59]
    Reflectshield,
    #[value = 60]
    Devotion,
    #[value = 61]
    Providence,
    #[value = 62]
    Defender,
    #[value = 63]
    Magicrod,
    #[value = 64]
    Weaponproperty,
    #[value = 65]
    Autospell,
    #[value = 68]
    Spearquicken,
    #[value = 69]
    Bdplaying,
    #[value = 70]
    Whistle,
    #[value = 71]
    Assassincross,
    #[value = 72]
    Poembragi,
    #[value = 73]
    Appleidun,
    #[value = 74]
    Humming,
    #[value = 75]
    Dontforgetme,
    #[value = 76]
    Fortunekiss,
    #[value = 77]
    Serviceforyou,
    #[value = 78]
    Richmankim,
    #[value = 79]
    Eternalchaos,
    #[value = 80]
    Drumbattlefield,
    #[value = 81]
    Ringnibelungen,
    #[value = 82]
    Rokisweil,
    #[value = 83]
    Intoabyss,
    #[value = 84]
    Siegfried,
    #[value = 85]
    Bladestop,
    #[value = 86]
    Explosionspirits,
    #[value = 87]
    Steelbody,
    #[value = 90]
    Propertyfire,
    #[value = 91]
    Propertywater,
    #[value = 92]
    Propertywind,
    #[value = 93]
    Propertyground,
    #[value = 95]
    Stop,
    #[value = 97]
    Propertyundead,
    #[value = 103]
    Aurablade,
    #[value = 104]
    Parrying,
    #[value = 105]
    Lkconcentration,
    #[value = 106]
    Tensionrelax,
    #[value = 107]
    Berserk,
    #[value = 109]
    Gospel,
    #[value = 110]
    Assumptio,
    #[value = 112]
    Groundmagic,
    #[value = 113]
    Magicpower,
    #[value = 114]
    Edp,
    #[value = 115]
    Truesight,
    #[value = 116]
    Windwalk,
    #[value = 117]
    Meltdown,
    #[value = 118]
    Cartboost,
    #[value = 119]
    Chasewalk,
    #[value = 120]
    Swordreject,
    #[value = 121]
    MarionetteMaster,
    #[value = 122]
    Marionette,
    #[value = 124]
    Blooding,
    #[value = 125]
    Jointbeat,
    #[value = 126]
    Mindbreaker,
    #[value = 127]
    Memorize,
    #[value = 128]
    Fogwall,
    #[value = 129]
    Spiderweb,
    #[value = 130]
    Protectexp,
    #[value = 132]
    Autoberserk,
    #[value = 133]
    Run,
    #[value = 135]
    StormkickOn,
    #[value = 137]
    DownkickOn,
    #[value = 139]
    TurnkickOn,
    #[value = 141]
    CounterOn,
    #[value = 143]
    DodgeOn,
    #[value = 145]
    Strup,
    #[value = 146]
    Propertydark,
    #[value = 147]
    Adrenaline2,
    #[value = 148]
    Propertytelekinesis,
    #[value = 149]
    Soullink,
    #[value = 150]
    Plusattackpower,
    #[value = 151]
    Plusmagicpower,
    #[value = 153]
    Kaite,
    #[value = 154]
    Swoo,
    #[value = 156]
    Kaizel,
    #[value = 157]
    Kaahi,
    #[value = 158]
    Kaupe,
    #[value = 159]
    SmaReady,
    #[value = 161]
    Onehandquicken,
    #[value = 165]
    SgSunWarm,
    #[value = 169]
    SunComfort,
    #[value = 170]
    MoonComfort,
    #[value = 171]
    StarComfort,
    #[value = 173]
    GdskillBattleorder,
    #[value = 174]
    GdskillRegeneration,
    #[value = 181]
    Preserve,
    #[value = 184]
    Clairvoyance,
    #[value = 186]
    Doublecasting,
    #[value = 187]
    Gravitation,
    #[value = 188]
    Overthrustmax,
    #[value = 189]
    Longing,
    #[value = 190]
    Hermode,
    #[value = 197]
    CrShrink,
    #[value = 198]
    WzSightblaster,
    #[value = 199]
    DcWinkcharm,
    #[value = 200]
    RgCconfineM,
    #[value = 201]
    RgCconfineS,
    #[value = 203]
    GsMadnesscancel,
    #[value = 204]
    GsGatlingfever,
    #[value = 205]
    Earthscroll,
    #[value = 206]
    NjUtsusemi,
    #[value = 207]
    NjBunsinjyutsu,
    #[value = 208]
    NjNen,
    #[value = 209]
    GsAdjustment,
    #[value = 210]
    GsAccuracy,
    #[value = 211]
    NjSuiton,
    #[value = 241]
    FoodStr,
    #[value = 242]
    FoodAgi,
    #[value = 243]
    FoodVit,
    #[value = 244]
    FoodDex,
    #[value = 245]
    FoodInt,
    #[value = 246]
    FoodLuk,
    #[value = 247]
    FoodBasicavoidance,
    #[value = 248]
    FoodBasichit,
    #[value = 250]
    CashPlusexp,
    #[value = 251]
    CashDeathpenalty,
    #[value = 252]
    CashReceiveitem,
    #[value = 253]
    CashBossAlarm,
    #[value = 271]
    FoodStrCash,
    #[value = 272]
    FoodAgiCash,
    #[value = 273]
    FoodVitCash,
    #[value = 274]
    FoodDexCash,
    #[value = 275]
    FoodIntCash,
    #[value = 276]
    FoodLukCash,
    #[value = 277]
    MerFlee,
    #[value = 278]
    MerAtk,
    #[value = 279]
    MerHp,
    #[value = 280]
    MerSp,
    #[value = 281]
    MerHit,
    #[value = 282]
    Slowcast,
    #[value = 286]
    Criticalwound,
    #[value = 290]
    ProtectDef,
    #[value = 291]
    ProtectMdef,
    #[value = 292]
    Healplus,
    #[value = 293]
    SLifepotion,
    #[value = 294]
    LLifepotion,
    #[value = 295]
    Criticalpercent,
    #[value = 296]
    Plusavoidvalue,
    #[value = 300]
    AtkerBlood,
    #[value = 301]
    TargetBlood,
    #[value = 302]
    ArmorProperty,
    #[value = 304]
    Hellpower,
    #[value = 311]
    Invincible,
    #[value = 312]
    CashPlusonlyjobexp,
    #[value = 316]
    Enchantblade,
    #[value = 317]
    Deathbound,
    #[value = 318]
    Refresh,
    #[value = 319]
    Giantgrowth,
    #[value = 320]
    Stonehardskin,
    #[value = 321]
    Vitalityactivation,
    #[value = 322]
    Fightingspirit,
    #[value = 323]
    Abundance,
    #[value = 324]
    ReuseMillenniumshield,
    #[value = 326]
    ReuseRefresh,
    #[value = 327]
    ReuseStormblast,
    #[value = 328]
    Venomimpress,
    #[value = 329]
    Epiclesis,
    #[value = 330]
    Oratio,
    #[value = 331]
    Laudaagnus,
    #[value = 332]
    Laudaramus,
    #[value = 333]
    Cloakingexceed,
    #[value = 334]
    Hallucinationwalk,
    #[value = 335]
    HallucinationwalkPostdelay,
    #[value = 336]
    Renovatio,
    #[value = 337]
    Weaponblocking,
    #[value = 339]
    Rollingcutter,
    #[value = 340]
    Expiatio,
    #[value = 341]
    Poisoningweapon,
    #[value = 342]
    Toxin,
    #[value = 343]
    Paralyse,
    #[value = 344]
    Venombleed,
    #[value = 345]
    Magicmushroom,
    #[value = 346]
    Deathhurt,
    #[value = 347]
    Pyrexia,
    #[value = 348]
    Oblivioncurse,
    #[value = 349]
    Leechesend,
    #[value = 350]
    Duplelight,
    #[value = 351]
    Frostmisty,
    #[value = 352]
    Fearbreeze,
    #[value = 353]
    Electricshocker,
    #[value = 354]
    Marshofabyss,
    #[value = 355]
    Recognizedspell,
    #[value = 356]
    Stasis,
    #[value = 358]
    Wugdash,
    #[value = 359]
    Wugbite,
    #[value = 360]
    Camouflage,
    #[value = 361]
    Acceleration,
    #[value = 362]
    Hovering,
    #[value = 363]
    Summon1,
    #[value = 364]
    Summon2,
    #[value = 365]
    Summon3,
    #[value = 366]
    Summon4,
    #[value = 367]
    Summon5,
    #[value = 372]
    OverheatLimitpoint,
    #[value = 373]
    Overheat,
    #[value = 374]
    Shapeshift,
    #[value = 375]
    Infraredscan,
    #[value = 376]
    Magneticfield,
    #[value = 377]
    Neutralbarrier,
    #[value = 378]
    NeutralbarrierMaster,
    #[value = 379]
    Stealthfield,
    #[value = 380]
    StealthfieldMaster,
    #[value = 381]
    ManuAtk,
    #[value = 382]
    ManuDef,
    #[value = 383]
    SplAtk,
    #[value = 384]
    SplDef,
    #[value = 386]
    ManuMatk,
    #[value = 387]
    SplMatk,
    #[value = 390]
    LgReflectdamage,
    #[value = 391]
    Forceofvanguard,
    #[value = 396]
    ShieldspellDef,
    #[value = 397]
    ShieldspellMdef,
    #[value = 398]
    ShieldspellRef,
    #[value = 400]
    Exeedbreak,
    #[value = 401]
    Adoramus,
    #[value = 402]
    Prestige,
    #[value = 405]
    Banding,
    #[value = 406]
    Earthdrive,
    #[value = 407]
    Inspiration,
    #[value = 413]
    Lightningwalk,
    #[value = 416]
    CursedcircleAtker,
    #[value = 417]
    CursedcircleTarget,
    #[value = 419]
    Crescentelbow,
    #[value = 429]
    Swing,
    #[value = 430]
    SymphonyLove,
    #[value = 431]
    Propertywalk,
    #[value = 432]
    Spellfist,
    #[value = 433]
    Netherworld,
    #[value = 434]
    Siren,
    #[value = 436]
    Sircleofnature,
    #[value = 437]
    Cold,
    #[value = 438]
    Gloomyday,
    #[value = 439]
    SongOfMana,
    #[value = 441]
    DanceWithWug,
    #[value = 442]
    RushWindmill,
    #[value = 443]
    Echosong,
    #[value = 444]
    Harmonize,
    #[value = 445]
    Striking,
    #[value = 446]
    Warmer,
    #[value = 447]
    MoonlitSerenade,
    #[value = 448]
    SaturdayNightFever,
    #[value = 450]
    Analyze,
    #[value = 451]
    LeradsDew,
    #[value = 452]
    Melodyofsink,
    #[value = 453]
    BeyondOfWarcry,
    #[value = 454]
    UnlimitedHummingVoice,
    #[value = 458]
    FreezeSp,
    #[value = 472]
    AbSecrament,
    #[value = 490]
    VacuumExtreme,
    #[value = 505]
    BandingDefence,
    #[value = 620]
    Crushstrike,
    #[value = 737]
    Burnt,
    #[value = 908]
    ResistPropertyWater,
    #[value = 182]
    Chasewalk2,
    #[value = 249]
    FoodCriticalsuccessvalue,
    #[value = 297]
    AtkerAspd,
    #[value = 303]
    ReuseLimitA,
    #[value = 306]
    ReuseLimitB,
    #[value = 307]
    ReuseLimitC,
    #[value = 308]
    ReuseLimitD,
    #[value = 309]
    ReuseLimitE,
    #[value = 310]
    ReuseLimitF,
    #[value = 313]
    Partyflee,
    #[value = 314]
    AngelProtect,
    #[value = 325]
    ReuseCrushstrike,
    #[value = 385]
    Reproduce,
    #[value = 393]
    Autoshadowspell,
    #[value = 394]
    Shadowform,
    #[value = 399]
    Bodypaint,
    #[value = 403]
    Invisibility,
    #[value = 404]
    Deadlyinfect,
    #[value = 408]
    Enervation,
    #[value = 409]
    Groomy,
    #[value = 410]
    Raisingdragon,
    #[value = 411]
    Ignorance,
    #[value = 412]
    Laziness,
    #[value = 415]
    Unlucky,
    #[value = 418]
    Weakness,
    #[value = 421]
    Stripaccessary,
    #[value = 422]
    Manhole,
    #[value = 425]
    GentletouchEnergygain,
    #[value = 426]
    GentletouchChange,
    #[value = 427]
    GentletouchRevitalize,
    #[value = 428]
    Bloodylust,
    #[value = 455]
    Spellbook1,
    #[value = 456]
    Spellbook2,
    #[value = 457]
    Spellbook3,
    #[value = 461]
    GnCartboost,
    #[value = 463]
    ThornsTrap,
    #[value = 464]
    BloodSucker,
    #[value = 467]
    FireExpansionSmokePowder,
    #[value = 468]
    FireExpansionTearGas,
    #[value = 470]
    Mandragora,
    #[value = 476]
    Stomachache,
    #[value = 477]
    MysteriousPowder,
    #[value = 478]
    MelonBomb,
    #[value = 479]
    BananaBombSitdownPostdelay,
    #[value = 480]
    PromoteHealthReserch,
    #[value = 481]
    EnergyDrinkReserch,
    #[value = 482]
    ExtractWhitePotionZ,
    #[value = 483]
    Vitata500,
    #[value = 484]
    ExtractSalamineJuice,
    #[value = 485]
    Boost500,
    #[value = 486]
    FullSwingK,
    #[value = 487]
    ManaPlus,
    #[value = 488]
    MustleM,
    #[value = 489]
    LifeForceF,
    #[value = 491]
    SavageSteak,
    #[value = 492]
    CocktailWargBlood,
    #[value = 493]
    MinorBbq,
    #[value = 494]
    SiromaIceTea,
    #[value = 495]
    DroceraHerbSteamed,
    #[value = 496]
    PuttiTailsNoodles,
    #[value = 497]
    BananaBomb,
    #[value = 499]
    Spellbook4,
    #[value = 500]
    Spellbook5,
    #[value = 501]
    Spellbook6,
    #[value = 502]
    Spellbook7,
    #[value = 515]
    CircleOfFire,
    #[value = 516]
    CircleOfFireOption,
    #[value = 517]
    FireCloak,
    #[value = 518]
    FireCloakOption,
    #[value = 519]
    WaterScreen,
    #[value = 520]
    WaterScreenOption,
    #[value = 521]
    WaterDrop,
    #[value = 522]
    WaterDropOption,
    #[value = 523]
    WindStep,
    #[value = 524]
    WindStepOption,
    #[value = 525]
    WindCurtain,
    #[value = 526]
    WindCurtainOption,
    #[value = 527]
    WaterBarrier,
    #[value = 528]
    Zephyr,
    #[value = 529]
    SolidSkin,
    #[value = 530]
    SolidSkinOption,
    #[value = 531]
    StoneShield,
    #[value = 532]
    StoneShieldOption,
    #[value = 533]
    PowerOfGaia,
    #[value = 539]
    Pyrotechnic,
    #[value = 540]
    PyrotechnicOption,
    #[value = 541]
    Heater,
    #[value = 542]
    HeaterOption,
    #[value = 543]
    Tropic,
    #[value = 544]
    TropicOption,
    #[value = 545]
    Aquaplay,
    #[value = 546]
    AquaplayOption,
    #[value = 547]
    Cooler,
    #[value = 548]
    CoolerOption,
    #[value = 549]
    ChillyAir,
    #[value = 550]
    ChillyAirOption,
    #[value = 551]
    Gust,
    #[value = 552]
    GustOption,
    #[value = 553]
    Blast,
    #[value = 554]
    BlastOption,
    #[value = 555]
    WildStorm,
    #[value = 556]
    WildStormOption,
    #[value = 557]
    Petrology,
    #[value = 558]
    PetrologyOption,
    #[value = 559]
    CursedSoil,
    #[value = 560]
    CursedSoilOption,
    #[value = 561]
    Upheaval,
    #[value = 562]
    UpheavalOption,
    #[value = 563]
    TidalWeapon,
    #[value = 564]
    TidalWeaponOption,
    #[value = 565]
    RockCrusher,
    #[value = 566]
    RockCrusherAtk,
    #[value = 567]
    FireInsignia,
    #[value = 568]
    WaterInsignia,
    #[value = 569]
    WindInsignia,
    #[value = 570]
    EarthInsignia,
    #[value = 574]
    ReuseLimitG,
    #[value = 575]
    ReuseLimitH,
    #[value = 576]
    NeedleOfParalyze,
    #[value = 577]
    PainKiller,
    #[value = 580]
    LightOfRegene,
    #[value = 581]
    OveredBoost,
    #[value = 583]
    OdinsPower,
    #[value = 598]
    GoldeneFerse,
    #[value = 599]
    AngriffsModus,
    #[value = 600]
    TinderBreaker,
    #[value = 601]
    TinderBreakerPostdelay,
    #[value = 602]
    Cbc,
    #[value = 604]
    Eqc,
    #[value = 605]
    MagmaFlow,
    #[value = 606]
    GraniticArmor,
    #[value = 607]
    Pyroclastic,
    #[value = 608]
    VolcanicAsh,
    #[value = 613]
    AllRiding,
    #[value = 614]
    AllRidingReuseLimit,
    #[value = 621]
    MonsterTransform,
    #[value = 624]
    MtfAspd,
    #[value = 625]
    MtfRangeatk,
    #[value = 626]
    MtfMatk,
    #[value = 627]
    MtfMleatked,
    #[value = 628]
    MtfCridamage,
    #[value = 629]
    ReuseLimitMtf,
    #[value = 622]
    Sit,
    #[value = 638]
    SetNumDef,
    #[value = 639]
    SetNumMdef,
    #[value = 645]
    KoJyumonjikiri,
    #[value = 646]
    Meikyousisui,
    #[value = 647]
    AtthasteCash,
    #[value = 651]
    Kyougaku,
    #[value = 652]
    Izayoi,
    #[value = 653]
    Zenkai,
    #[value = 654]
    KgKagehumi,
    #[value = 655]
    Kyomu,
    #[value = 656]
    Kagemusya,
    #[value = 657]
    Zangetsu,
    #[value = 659]
    Gensou,
    #[value = 660]
    Akaitsuki,
    #[value = 669]
    ReuseLimitEcl,
    #[value = 673]
    OnPushCart,
    #[value = 700]
    JpEvent04,
    #[value = 705]
    QuestBuff1,
    #[value = 706]
    QuestBuff2,
    #[value = 707]
    QuestBuff3,
    #[value = 708]
    ReuseLimitRecall,
    #[value = 712]
    ReuseLimitAspdPotion,
    #[value = 719]
    Strangelights,
    #[value = 779]
    DecorationOfMusic,
    #[value = 793]
    MtfMhp,
    #[value = 794]
    MtfMsp,
    #[value = 795]
    MtfPumpkin,
    #[value = 796]
    MtfHitflee,
    #[value = 818]
    MtfRangeatk2,
    #[value = 819]
    MtfAspd2,
    #[value = 820]
    MtfMatk2,
    #[value = 191]
    Tarotcard,
    #[value = 299]
    AtkerMovespeed,
    #[value = 465]
    SporeExplosion,
    #[value = 637]
    HandicapstateNorecover,
    #[value = 665]
    ActiveMonsterTransform,
    #[value = 668]
    EntryQueueApplyDelay,
    #[value = 671]
    EntryQueueNotifyAdmissionTimeOut,
    #[value = 674]
    HatEffect,
    #[value = 677]
    GlastheimAtk,
    #[value = 678]
    GlastheimDef,
    #[value = 679]
    GlastheimHeal,
    #[value = 680]
    GlastheimHidden,
    #[value = 681]
    GlastheimState,
    #[value = 682]
    GlastheimItemdef,
    #[value = 683]
    GlastheimHpsp,
    #[value = 686]
    GvgGiant,
    #[value = 687]
    GvgGolem,
    #[value = 688]
    GvgStun,
    #[value = 689]
    GvgStone,
    #[value = 690]
    GvgFreez,
    #[value = 691]
    GvgSleep,
    #[value = 692]
    GvgCurse,
    #[value = 693]
    GvgSilence,
    #[value = 694]
    GvgBlind,
    #[value = 696]
    ClanInfo,
    #[value = 702]
    GeffenMagic1,
    #[value = 703]
    GeffenMagic2,
    #[value = 704]
    GeffenMagic3,
    #[value = 713]
    Maxpain,
    #[value = 715]
    FriggSong,
    #[value = 716]
    Offertorium,
    #[value = 717]
    TelekinesisIntense,
    #[value = 718]
    Moonstar,
    #[value = 720]
    FullThrottle,
    #[value = 721]
    Rebound,
    #[value = 722]
    Unlimit,
    #[value = 723]
    KingsGrace,
    #[value = 728]
    SuperStar,
    #[value = 730]
    Darkcrow,
    #[value = 734]
    Illusiondoping,
    #[value = 740]
    Flashcombo,
    #[value = 752]
    BTrap,
    #[value = 753]
    EChain,
    #[value = 754]
    EQdShotReady,
    #[value = 755]
    CMarker,
    #[value = 756]
    HMine,
    #[value = 758]
    PAlter,
    #[value = 759]
    HeatBarrel,
    #[value = 760]
    AntiMBlast,
    #[value = 762]
    Swordclan,
    #[value = 763]
    Arcwandclan,
    #[value = 764]
    Goldenmaceclan,
    #[value = 765]
    Crossbowclan,
    #[value = 815]
    Jumpingclan,
    #[value = 822]
    Flowersmoke,
    #[value = 823]
    Fstone,
    #[value = 824]
    Dailysendmailcnt,
    #[value = 826]
    Ljosalfar,
    #[value = 849]
    HappinessStar,
    #[value = 856]
    DressUp,
    #[value = 857]
    MapleFalls,
    #[value = 863]
    MermaidLonging,
    #[value = 864]
    MagicalFeather,
    #[value = 872]
    TimeAccessory,
    #[value = 893]
    SuStoop,
    #[value = 894]
    Catnippowder,
    #[value = 896]
    SvRoottwist,
    #[value = 909]
    ResistPropertyGround,
    #[value = 910]
    ResistPropertyFire,
    #[value = 911]
    ResistPropertyWind,
    #[value = 917]
    Bitescar,
    #[value = 918]
    Arclousedash,
    #[value = 919]
    Tunaparty,
    #[value = 920]
    Shrimp,
    #[value = 921]
    Freshshrimp,
    #[value = 933]
    Suhide,
    #[value = 935]
    DoramBuf01,
    #[value = 936]
    DoramBuf02,
    #[value = 937]
    Spritemable,
    #[value = 950]
    Hiss,
    #[value = 952]
    Nyanggrass,
    #[value = 953]
    Chattering,
    #[value = 961]
    Grooming,
    #[value = 962]
    Protectionofshrimp,
    #[value = 963]
    Ep162BuffSs,
    #[value = 964]
    Ep162BuffSc,
    #[value = 965]
    Ep162BuffAc,
    #[value = 992]
    Cheerup,
    #[value = 1001]
    LhzDunN1,
    #[value = 1002]
    LhzDunN2,
    #[value = 1003]
    LhzDunN3,
    #[value = 1004]
    LhzDunN4,
    #[value = 1035]
    Lightofmoon,
    #[value = 1036]
    Lightofsun,
    #[value = 1037]
    Lightofstar,
    #[value = 1038]
    Lunarstance,
    #[value = 1039]
    Universestance,
    #[value = 1040]
    Sunstance,
    #[value = 1041]
    Flashkick,
    #[value = 1042]
    Newmoon,
    #[value = 1043]
    Starstance,
    #[value = 1044]
    Dimension,
    #[value = 1047]
    Creatingstar,
    #[value = 1048]
    Fallingstar,
    #[value = 1049]
    Novaexplosing,
    #[value = 1050]
    Gravitycontrol,
    #[value = 1053]
    Soulcollect,
    #[value = 1054]
    Soulreaper,
    #[value = 1055]
    Soulunity,
    #[value = 1056]
    Soulshadow,
    #[value = 1057]
    Soulfairy,
    #[value = 1058]
    Soulfalcon,
    #[value = 1059]
    Soulgolem,
    #[value = 1060]
    Souldivision,
    #[value = 1061]
    Soulenergy,
    #[value = 1062]
    UseSkillSpSpa,
    #[value = 1063]
    UseSkillSpSha,
    #[value = 1064]
    SpSha,
    #[value = 1088]
    Ensemblefatigue,
    #[value = 1095]
    Ancilla,
    #[value = 1107]
    WeaponblockOn,
    #[value = 1125]
    Soulcurse,
    #[value = 1126]
    SoundOfDestruction,
    #[value = 1141]
    MistyFrost,
    #[value = 1142]
    MagicPoison,
    #[value = 1154]
    Luxanima,
    #[value = 1165]
    HellsPlantArmor,
    #[value = 1169]
    RefTPotion,
    #[value = 1170]
    AddAtkDamage,
    #[value = 1171]
    AddMatkDamage,
}
