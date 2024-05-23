use crate::{
    board_elements::{CastlingType, Piece},
    constants::BOARD_START,
};

const COLORS_IN_CHESS: usize = 2;
const KINDS_OF_CHESS_PIECES: usize = 6;
const SQUARES_ON_BOARD: usize = 64;
const CASTLING_RIGHTS: usize = 4;
const POSSIBLE_EN_PASSANT_FILES: usize = 8;

// Requries rand, so commented out to keep zero dependencies
/*pub fn _generate_zobrist_bitstrings() {
    dbg!(_generate_rand_array(BITSTRING_INDICATING_BLACK_TO_MOVE));
}

pub fn _generate_rand_array(size: usize) -> Vec<u64> {
    let mut rng = thread_rng();

    (0..size).map(|_| rng.gen()).collect()
}
*/

pub struct ZobristOracle;

impl ZobristOracle {
    pub fn black_to_move_bitstring() -> u64 {
        BLACK_TO_MOVE_BITSTRING
    }

    pub fn piece_bitstring(piece: Piece, board_index: usize) -> u64 {
        PIECE_BITSTRINGS[piece.index() * board_index]
    }

    pub fn castling_right_bitstring(castling_type: CastlingType) -> u64 {
        CASTLING_BITSTRINGS[castling_type.index()]
    }

    pub fn en_passant_bitstring(file: usize) -> u64 {
        let file_without_offset = file - BOARD_START;
        EN_PASSANT_FILE_BITSTRINGS[file_without_offset]
    }
}
const BLACK_TO_MOVE_BITSTRING: u64 = 8876382254404422227;

const CASTLING_BITSTRINGS: [u64; CASTLING_RIGHTS] = [
    9252946648730652173,
    5221904148399552195,
    10403483266606492017,
    7587135690951589750,
];

const EN_PASSANT_FILE_BITSTRINGS: [u64; POSSIBLE_EN_PASSANT_FILES] = [
    14681790476698478451,
    15064494372072190424,
    8192443764381849699,
    11492292133730190635,
    411872552194389956,
    8196095195359712773,
    8237092200270485498,
    12391617185027411447,
];

const PIECE_BITSTRINGS: [u64; COLORS_IN_CHESS * KINDS_OF_CHESS_PIECES * SQUARES_ON_BOARD] = [
    359208017975557256,
    12589318807296072540,
    14739478489658724102,
    17218677482575912697,
    14828215336600102188,
    7016297474027023372,
    11547386325395530792,
    5112418951819149571,
    654431932398939582,
    10718216586238536578,
    16936211610206609289,
    4757340148673060237,
    7293454881337360849,
    15196538204021648750,
    16563466103239336593,
    11172245761742118255,
    45143893534249365,
    10714638590613637377,
    17621896361694569021,
    9884738830893644631,
    17905050063511944602,
    13224579358937531079,
    335085745765365248,
    17289428295250129274,
    15762215526036249757,
    6596568248643723554,
    11089341041830133322,
    9935001944114327831,
    12211768823224886544,
    9823566779257343281,
    15203671652996059621,
    3713525470943291506,
    7516146522202934493,
    9164754344315649004,
    4940388864043108771,
    7292288135976095128,
    5549445107624406676,
    10659010366728487570,
    14062678839000328486,
    10086964004584947568,
    7226058430330290658,
    731668811566715705,
    178385547011703736,
    17618291250431626046,
    16442898251864790017,
    2571237501068896168,
    14211005472061702809,
    17476703671127085611,
    9734781915602223721,
    8317385618558451898,
    4815738512496519754,
    2034071417779917603,
    14481849658533260501,
    1746274206954841372,
    18078605852537525076,
    3869713597052156522,
    6011853038032488285,
    3075157195535735234,
    9966303395234771594,
    31791921377288875,
    12169638854206173067,
    15311348613232953266,
    6033691173726800833,
    96095718000750689,
    7041628657287840004,
    228286713272350921,
    8613964064211361359,
    14447075947344438360,
    16535358657707576292,
    1680529958959816209,
    9358666837546340873,
    11155959224021550451,
    3021384364914959869,
    16789495919097058218,
    4690845962742562239,
    12747549914828175689,
    16820253910617757830,
    7642641511738371613,
    18295689634377396607,
    13424828666271598828,
    14803874608897011099,
    186148773798885556,
    5780319606520535301,
    10616275992844014172,
    9674078921658023166,
    5069895294808207366,
    1888431647895495606,
    3248946330374403068,
    221263996414941152,
    13401657695790760697,
    472597227718903412,
    8914082404210886696,
    8267179660992913881,
    8131981459364447738,
    17673739008272919541,
    3083881993895471676,
    5267494361448686413,
    15181682273185946119,
    5062443450926492831,
    6806399281046032712,
    15484840499905362040,
    17133131211738549271,
    18393768056458025091,
    13610723078138059274,
    11312362612831603351,
    13434702768873820882,
    5218475738415795378,
    11430446818420551968,
    1299874866883863222,
    4528284899992758739,
    12108251360152431722,
    7005197934945071446,
    3483797465388784757,
    4987704277028942038,
    4229973264225698283,
    16853724200122670728,
    14416109075659157392,
    6119731976232015486,
    5249338962503528704,
    12346928278768222595,
    17310785630954731444,
    10156124160189121123,
    2228245035820384081,
    749464954198752322,
    3222666031508404054,
    2717288901883926112,
    10062917407341648350,
    12784693556103029730,
    14597130714078264523,
    13085367721131795219,
    934366376285269238,
    15388751214216644470,
    9364932300557699710,
    10894409912271423929,
    6326839835704249544,
    10178203160634014835,
    16034284493394963465,
    161349738208842211,
    11551903383337368563,
    6374399203299176682,
    9532630974267373216,
    1341282036656108401,
    2530573341430552153,
    11681666577766578495,
    4903200439383349110,
    9349095984909300233,
    17802558669588407965,
    11208325869588605191,
    5882199696826630444,
    13118190975036588315,
    9844063699970756305,
    9829132396567961274,
    7814159897312162489,
    2575773690264305899,
    1334175347850999237,
    10234588516036969861,
    2404852967594316322,
    5215955566257977475,
    16948201745795047149,
    2200632382504115983,
    17091640743867543131,
    6027146449873378925,
    6418959428026318544,
    3297918948321740754,
    12788941857218183431,
    9171958331850518776,
    14216240572022078367,
    17159590222711952229,
    637155644858875469,
    76176269441831676,
    10011505417824476286,
    230769661972350502,
    18331600204310670212,
    16075495912598310049,
    15836600953786751440,
    4915177515152932479,
    6578772564837892573,
    16128407910143153285,
    4769797449925392462,
    4224953216145139059,
    16104015478965792921,
    17813222211336729864,
    17790723506845133600,
    340627010481744702,
    8391020902030672101,
    13960657793951025157,
    10591054931178857705,
    5434119069531810012,
    18446230511765531769,
    446157456276080632,
    17833759666200390661,
    13174042748002940009,
    12526249090883586037,
    3686962993403163905,
    6041480024500720264,
    14794981652400551118,
    6038169644893220754,
    3499053016288282189,
    245396157586700369,
    13448808511848171266,
    10760117853471649408,
    11089528247840892693,
    11895561023586107589,
    9803723274578313294,
    3622861506659304115,
    290238384053743747,
    5368204443312836538,
    177821014129684029,
    11288992040712280562,
    6427035695830558710,
    14562823588262658253,
    13437460024098756470,
    14912017715680909262,
    6199666446600562927,
    16089375967600840257,
    9836445861185292609,
    1005863169667177417,
    10185629690025089827,
    15224357068063203943,
    7235868395109256906,
    7769290447468992421,
    9554516164086480893,
    5733153780247017457,
    17268679263365062432,
    2221580407224090705,
    5260838399602016112,
    4251388734339775748,
    9023074425698954335,
    3487150457070132057,
    3118329529730107945,
    2040807845420417450,
    13287297524627414482,
    17221005591521612826,
    15026402129331917398,
    16496056114264694634,
    3053555886931461532,
    4771823302336499783,
    3358270915619090716,
    13392702377066972082,
    14086529731992755362,
    14812012522240671122,
    13529010639488878542,
    1749049271395702791,
    10942878802987401977,
    1036859959825273779,
    2223617109701114736,
    9747066181753064648,
    7208068742519175458,
    14316279942617669616,
    6883334278698164086,
    15006060298204266139,
    12554002492879338162,
    9867818014937412631,
    2192487634994716591,
    16099319417058703152,
    7240005463468194965,
    8366937907561964904,
    5685683960393664601,
    15853777402182842767,
    16179384979194558485,
    13532444064303192485,
    1716571879834336240,
    4159385412439058444,
    15629943458247970872,
    14641455318898728745,
    9126748883753228548,
    4047063610903229260,
    10984099424895811675,
    15467292995984623276,
    16049932406025713076,
    2710724515189591358,
    17197393675947048935,
    11592311420183143692,
    1007437905986498559,
    8343539650981009714,
    11739341413064384427,
    13118212045107817234,
    13392290468981218354,
    2942736807593154235,
    11106765691065905412,
    7111526736903176573,
    1377847796144293196,
    11788054628959099714,
    6337091989233431578,
    9922536958416685870,
    3823597590092004526,
    17315926329743295631,
    11851589708473918820,
    11058564215556306286,
    14721174105619966586,
    6967700024258584052,
    1282586488134820995,
    12095178677426764787,
    6359449152011788135,
    4153579986927260940,
    14710551449181880233,
    11995440939769136365,
    14616218186476871332,
    15083357657435607196,
    16045191838047383312,
    2475682024689479157,
    10827407097779596576,
    13709682675277741918,
    7456922556754460042,
    15986620873481663947,
    8347878441218625631,
    4501889198173525735,
    8125198111451015027,
    1852260952093386162,
    9301052036477904925,
    10784786816027654766,
    9919470610934095952,
    3169858365495032037,
    18215621658268579029,
    15027213679441700660,
    9883207768445689887,
    13586575479444199758,
    8185349518673758205,
    10452777043491429862,
    8431354001368314067,
    13151331057269059579,
    1510101841484972088,
    4394460122111862667,
    1591307230869285107,
    7567519781431757586,
    3268947078355416421,
    14513001249875870148,
    11758117045444831204,
    2298919734169539359,
    9332606829709046648,
    7262188671091754223,
    13069555188058896754,
    16664561748933848636,
    12362265596259373552,
    15104398203600134244,
    548154931957622189,
    8373255669308609199,
    13140351989925087253,
    6941183032452901929,
    1299416716326411781,
    14039941649674063958,
    17856299148614283972,
    16821859661757111354,
    4399303345167663472,
    3935701796029297092,
    16761528179233142950,
    13025775732324321542,
    1013285199369312717,
    16331025457329466641,
    12002771779937659056,
    5484762570880954110,
    7131394226090026981,
    2987131421271736373,
    11745594597272648599,
    6888282236996503842,
    3502609956387352124,
    8451035374163443182,
    5907972092524769389,
    967126159487547735,
    9253397053814662167,
    18020379651206918030,
    1550686209484978576,
    11487099021281074312,
    15646370767279135660,
    9517087857607508575,
    8267258588902101611,
    1851034056553925017,
    7224478624148523379,
    14097368321739140047,
    2123994302580257745,
    17463220590666109497,
    584227950826958903,
    11171587177203896860,
    271036224480906719,
    10296550572934127963,
    801670276698054887,
    3527313234135879252,
    9366514530488307942,
    14205989811491031789,
    2988943304127808006,
    4895643138776034942,
    6888530976639537641,
    11082028058501535002,
    17219202578859521592,
    2735912958658035181,
    12320495319747615987,
    1546652409033467612,
    16064581694161762370,
    12687424775220636052,
    7686320739570380084,
    10904574790696518884,
    4354059571007029185,
    15252355221677033465,
    2098482981630673911,
    13696109004726023229,
    8828779845913833426,
    5102519509517296621,
    426745579428552711,
    12865166946907360938,
    11217091501732233607,
    6780780801415333872,
    8040166814797140052,
    13651966255841238807,
    3629453445463986857,
    16738220868917014046,
    10020296977373417276,
    8243359749681138661,
    3885470818320956201,
    1986665046807277785,
    768851855243470367,
    5860618588649492398,
    15444249738072564807,
    9857088090714895065,
    12190344925985739028,
    15634486980642261457,
    1836793476717800835,
    685906026288738765,
    2271325236150298030,
    16802587730283422205,
    12982210433985430899,
    7336683120241615680,
    212161559900263966,
    6973557609767749902,
    9008119284576206960,
    46765654381487198,
    18329616493374858310,
    8728255927708623724,
    7241011242573867249,
    6468607564178174381,
    5873210251049622833,
    12205503928616153709,
    5470224520453271027,
    13147858134241802022,
    6490138684278954353,
    9885490741412393907,
    11777710108017999646,
    17664751538370502129,
    18384925398360996346,
    9740809280403888368,
    2590917086394354599,
    3569513135529619637,
    10128685625860915913,
    12371887349623804460,
    13159110877482944576,
    980897162052686139,
    8206236545790128306,
    965031335613376629,
    4756665221965675088,
    13190709847053651173,
    13175100794204986234,
    14491213346380800208,
    7773734673328847103,
    8323242153287074291,
    17202739453883235000,
    6360204819416545310,
    17976421698888528204,
    972307611516193614,
    13674405126645481015,
    5834093646785126289,
    18313260723021776237,
    4487844876062953221,
    4176337107602029228,
    8953360916052731510,
    15457404786271490594,
    1255195326111113050,
    2519031349094434376,
    3232225778988184286,
    7318893926781137732,
    13398678253290712023,
    7676240253521781735,
    267711649375769764,
    5573962425846054561,
    8305359715064474722,
    1423452214089747286,
    13222705025169844913,
    8605043952944422710,
    16665839036206287522,
    3158475391147156999,
    8184492202167058740,
    12556696240473961792,
    10320721957294886949,
    8420340958541884217,
    4959097785148172536,
    13551962352924589005,
    12916214995254723388,
    11775617893762802781,
    11937478845928500467,
    9609115479559747958,
    12793595569499393023,
    4161040062968842455,
    13791045193560269381,
    893886753579750529,
    626071261020644909,
    4878401043520434884,
    6444683914076233925,
    9117047370680186868,
    9283304004129684249,
    6322587859036112943,
    12087243315909518420,
    2922798070379597260,
    7977348463875974215,
    1692863856573990545,
    10331008020128193946,
    8468810570306013123,
    15189175960793146158,
    7760768438036062230,
    4696988040051571858,
    6147113960902335132,
    12399705465316352152,
    8490926226781075646,
    16674275264158459382,
    3636749359004890650,
    8725547332433889222,
    3108398730107568054,
    14664481410116784110,
    2072093147257536265,
    12195910550331040593,
    6214559929354892086,
    10125352779076555018,
    2816162875253709994,
    9747332862643396214,
    1440610625993384519,
    17469388013028429979,
    7865273910161989261,
    2985211938989755526,
    9332156541645524277,
    4557799366004191516,
    7022618996016109529,
    13564463960115882369,
    17103730053777629320,
    13009997756910208828,
    16951854563982605372,
    1195294484118589963,
    11844122781940062452,
    17000920417102282076,
    1257886979562623529,
    7371646808168658735,
    17639576541414858769,
    10098412958001564633,
    13402454460801263013,
    15646148149641280472,
    5356186173638077782,
    15260460558113862232,
    17865015272748349527,
    18123211741009276989,
    9037583868606966126,
    15840037670343300333,
    11690320897933132743,
    2792631027497156275,
    6455910047581394981,
    3830960970130785492,
    16257386279897090680,
    692357458148858712,
    11326987222016242670,
    5117161435884054078,
    11058788182633247934,
    2015967286093484649,
    5140777509906798998,
    3281460736438350673,
    60263097492136669,
    1145699437353237077,
    10267824363821817321,
    1278593901035258162,
    2449156305529345626,
    11109375853276597286,
    14391713144745315581,
    11798122720337843558,
    309219633422079315,
    11200296102744724481,
    734191663192025875,
    7630934110605280379,
    9378858461161609899,
    17649653629582545145,
    18008819963393161596,
    7682569689964042808,
    8322441898865122670,
    15115986830426937304,
    14562698669728141711,
    16828959370205681799,
    16045910236817378863,
    3883882623631451330,
    2613306350329927915,
    13131942392185752437,
    4553824238108586779,
    16707599891377999808,
    9406912049271178991,
    6517807914622880295,
    4856490901786043360,
    211708013073618719,
    10922974727441239707,
    13325929284007248054,
    5505814969630798725,
    2933946688815938970,
    5829508286116404623,
    3328606837571097493,
    11502248637880480130,
    8682543691636317447,
    14493635252360259269,
    18050619161367337560,
    9969904257747190305,
    2697641201710357531,
    14292362181203883536,
    1899925719182728748,
    1640354663898882104,
    9356798969462834125,
    1108270675630601524,
    6664269920413846579,
    14959891379944673736,
    7494164252442838915,
    6468906092836141704,
    4805515017794467552,
    4728611899280358253,
    8648476338846405722,
    13212728507064482545,
    15906711158328689891,
    10523260933007073952,
    7355782512720354466,
    7443720473004581805,
    15411577045415552315,
    7817849836020125806,
    2816933547506397611,
    376762568693350736,
    11367272368973976050,
    10575814004713456281,
    4360356276744465202,
    12008352379119964996,
    15941612865049121063,
    8105252579480607236,
    10180899910746141986,
    7757903829301953187,
    8596919549728472446,
    7428739985106071688,
    10918562345318807223,
    17375265019854595805,
    3683172538397048523,
    5489068488723053001,
    16578358407057875014,
    15765134253443841557,
    581533367890694507,
    16795075766064431840,
    11517087790084312573,
    8205355071246440156,
    1249649571518365669,
    16254460096288787180,
    15600244256491552494,
    8627613613391793242,
    3401567211938266370,
    5561552675693711541,
    11548253570692473033,
    16326030110930252964,
    11346522522541452312,
    12787253809816592903,
    5985666113665126322,
    2095427297044964566,
    14702176729033769766,
    500167255068614631,
    6456764047996171153,
    9245885636347218295,
    9062138576837276341,
    8921955096092326351,
    173614376194033692,
    14323071974019915644,
    9962479435601616910,
    17167541760278124693,
    9395198708947266814,
    14419029800124137957,
    16364242049491753763,
    4117322344480422468,
    4337760794022393863,
    3727986084219909680,
    1637140304579824542,
    13514556567324617801,
    1803226083545638813,
    17112438224714871194,
    17555075737155262934,
    8826059014734754962,
    2393818816340940234,
    13177666504047683393,
    15822198379885832420,
    6796889616688150360,
    3814317746375444506,
    724310826664487208,
    8640181998886664941,
    15334740469660372346,
    4770738497587178404,
    8854162688765411088,
    6531167013962803227,
    8180060062451362405,
    1868184743286172055,
    5393825041994435830,
    5780100866172057340,
    7667260142779107245,
    16681266341399237379,
    3699383386220786071,
    11883893456633720388,
    13082940207345153899,
    16786507142560884292,
    404413687007778594,
    17465660914786372378,
    12250122239938917434,
    3659704017899295408,
    5845019930662647913,
    15702840164205174842,
    11837791317330324017,
    13597780697686730806,
    18446507807736119209,
    7593149069273956716,
    4890951497825475318,
    6296192802816907417,
    3494831524315619623,
    14087534805527735253,
    4845757742156641830,
    2311887594165461031,
    13150997071458369521,
    3840059697724676182,
    17595386148027769787,
    4739030970767065446,
    11590600855228215650,
    9376494946663061266,
    9288103048586192451,
    4385728794072407268,
    2002523373222882854,
    14806179819366078825,
    15765333992439663954,
    10560590253530256607,
    386132377029914013,
    6639080913966537703,
    10789253295242340971,
    15639413799912335054,
    12119988054345780682,
    7378924634383501647,
    421609083879245900,
    17718639157274315282,
    8849203895691475979,
    17875385081251537540,
    11589792144731400970,
    15389507883374049258,
    8011405049721308644,
    16152602284737477350,
    6778795557219330653,
    13435912407873149408,
    16905755530525020413,
    5679625942190212920,
    15106392419243095416,
    2347398823754242121,
    9218766266795555007,
    218250456603125462,
    10533143447539133688,
    6783204180328881422,
    1233823802103199593,
    7373774201254208119,
    18094687655959377895,
    12877974534840137628,
    8204266104412064365,
    6781680950496632220,
    14028473279175265441,
    9208030038963526107,
    9472925528523427033,
    6551964048061435198,
    18304245513246653156,
    14379576118378225939,
    7207225227280455212,
    101064407621050813,
    7383751675617534379,
    12220849876341864927,
    10311038057343245068,
    3750836412502181880,
    1554222515065551682,
];
