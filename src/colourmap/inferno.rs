/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

// From https://github.com/bids/colormap
use crate::colour::Spectrum;

pub const INFERNO_COLOURMAP: [Spectrum; 256] = [
    Spectrum {
        red: 0.0014616,
        green: 0.0004661,
        blue: 0.0138655,
    },
    Spectrum {
        red: 0.0022673,
        green: 0.0012699,
        blue: 0.0185704,
    },
    Spectrum {
        red: 0.0032990,
        green: 0.0022493,
        blue: 0.0242391,
    },
    Spectrum {
        red: 0.0045469,
        green: 0.0033918,
        blue: 0.0309092,
    },
    Spectrum {
        red: 0.0060055,
        green: 0.0046919,
        blue: 0.0385579,
    },
    Spectrum {
        red: 0.0076758,
        green: 0.0061361,
        blue: 0.0468360,
    },
    Spectrum {
        red: 0.0095605,
        green: 0.0077134,
        blue: 0.0551431,
    },
    Spectrum {
        red: 0.0116635,
        green: 0.0094168,
        blue: 0.0634598,
    },
    Spectrum {
        red: 0.0139950,
        green: 0.0112247,
        blue: 0.0718617,
    },
    Spectrum {
        red: 0.0165606,
        green: 0.0131362,
        blue: 0.0802818,
    },
    Spectrum {
        red: 0.0193732,
        green: 0.0151326,
        blue: 0.0887668,
    },
    Spectrum {
        red: 0.0224469,
        green: 0.0171991,
        blue: 0.0973274,
    },
    Spectrum {
        red: 0.0257927,
        green: 0.0193306,
        blue: 0.1059298,
    },
    Spectrum {
        red: 0.0294324,
        green: 0.0215031,
        blue: 0.1146213,
    },
    Spectrum {
        red: 0.0333852,
        green: 0.0237024,
        blue: 0.1233973,
    },
    Spectrum {
        red: 0.0376684,
        green: 0.0259208,
        blue: 0.1322321,
    },
    Spectrum {
        red: 0.0422526,
        green: 0.0281385,
        blue: 0.1411405,
    },
    Spectrum {
        red: 0.0469146,
        green: 0.0303236,
        blue: 0.1501639,
    },
    Spectrum {
        red: 0.0516438,
        green: 0.0324736,
        blue: 0.1592543,
    },
    Spectrum {
        red: 0.0564491,
        green: 0.0345692,
        blue: 0.1684135,
    },
    Spectrum {
        red: 0.0613397,
        green: 0.0365900,
        blue: 0.1776422,
    },
    Spectrum {
        red: 0.0663313,
        green: 0.0385036,
        blue: 0.1869616,
    },
    Spectrum {
        red: 0.0714289,
        green: 0.0402939,
        blue: 0.1963536,
    },
    Spectrum {
        red: 0.0766368,
        green: 0.0419053,
        blue: 0.2057988,
    },
    Spectrum {
        red: 0.0819621,
        green: 0.0433279,
        blue: 0.2152891,
    },
    Spectrum {
        red: 0.0874114,
        green: 0.0445562,
        blue: 0.2248135,
    },
    Spectrum {
        red: 0.0929902,
        green: 0.0455830,
        blue: 0.2343576,
    },
    Spectrum {
        red: 0.0987025,
        green: 0.0464019,
        blue: 0.2439037,
    },
    Spectrum {
        red: 0.1045509,
        green: 0.0470081,
        blue: 0.2534303,
    },
    Spectrum {
        red: 0.1105361,
        green: 0.0473987,
        blue: 0.2629122,
    },
    Spectrum {
        red: 0.1166564,
        green: 0.0475736,
        blue: 0.2723208,
    },
    Spectrum {
        red: 0.1229081,
        green: 0.0475360,
        blue: 0.2816242,
    },
    Spectrum {
        red: 0.1292850,
        green: 0.0472931,
        blue: 0.2907880,
    },
    Spectrum {
        red: 0.1357785,
        green: 0.0468564,
        blue: 0.2997764,
    },
    Spectrum {
        red: 0.1423778,
        green: 0.0462423,
        blue: 0.3085529,
    },
    Spectrum {
        red: 0.1490730,
        green: 0.0454676,
        blue: 0.3170851,
    },
    Spectrum {
        red: 0.1558497,
        green: 0.0445588,
        blue: 0.3253384,
    },
    Spectrum {
        red: 0.1626889,
        green: 0.0435543,
        blue: 0.3332767,
    },
    Spectrum {
        red: 0.1695751,
        green: 0.0424893,
        blue: 0.3408742,
    },
    Spectrum {
        red: 0.1764932,
        green: 0.0414017,
        blue: 0.3481106,
    },
    Spectrum {
        red: 0.1834288,
        green: 0.0403289,
        blue: 0.3549714,
    },
    Spectrum {
        red: 0.1903675,
        green: 0.0393089,
        blue: 0.3614469,
    },
    Spectrum {
        red: 0.1972974,
        green: 0.0384002,
        blue: 0.3675346,
    },
    Spectrum {
        red: 0.2042093,
        green: 0.0376323,
        blue: 0.3732376,
    },
    Spectrum {
        red: 0.2110955,
        green: 0.0370296,
        blue: 0.3785633,
    },
    Spectrum {
        red: 0.2179486,
        green: 0.0366146,
        blue: 0.3835224,
    },
    Spectrum {
        red: 0.2247629,
        green: 0.0364050,
        blue: 0.3881289,
    },
    Spectrum {
        red: 0.2315381,
        green: 0.0364053,
        blue: 0.3924002,
    },
    Spectrum {
        red: 0.2382730,
        green: 0.0366210,
        blue: 0.3963534,
    },
    Spectrum {
        red: 0.2449669,
        green: 0.0370545,
        blue: 0.4000066,
    },
    Spectrum {
        red: 0.2516204,
        green: 0.0377053,
        blue: 0.4033779,
    },
    Spectrum {
        red: 0.2582343,
        green: 0.0385706,
        blue: 0.4064850,
    },
    Spectrum {
        red: 0.2648096,
        green: 0.0396469,
        blue: 0.4093454,
    },
    Spectrum {
        red: 0.2713467,
        green: 0.0409216,
        blue: 0.4119761,
    },
    Spectrum {
        red: 0.2778498,
        green: 0.0423529,
        blue: 0.4143921,
    },
    Spectrum {
        red: 0.2843213,
        green: 0.0439326,
        blue: 0.4166079,
    },
    Spectrum {
        red: 0.2907634,
        green: 0.0456438,
        blue: 0.4186368,
    },
    Spectrum {
        red: 0.2971783,
        green: 0.0474700,
        blue: 0.4204912,
    },
    Spectrum {
        red: 0.3035682,
        green: 0.0493959,
        blue: 0.4221824,
    },
    Spectrum {
        red: 0.3099353,
        green: 0.0514070,
        blue: 0.4237210,
    },
    Spectrum {
        red: 0.3162818,
        green: 0.0534901,
        blue: 0.4251163,
    },
    Spectrum {
        red: 0.3226097,
        green: 0.0556335,
        blue: 0.4263769,
    },
    Spectrum {
        red: 0.3289208,
        green: 0.0578266,
        blue: 0.4275105,
    },
    Spectrum {
        red: 0.3352169,
        green: 0.0600599,
        blue: 0.4285243,
    },
    Spectrum {
        red: 0.3414998,
        green: 0.0623253,
        blue: 0.4294245,
    },
    Spectrum {
        red: 0.3477711,
        green: 0.0646156,
        blue: 0.4302168,
    },
    Spectrum {
        red: 0.3540322,
        green: 0.0669247,
        blue: 0.4309062,
    },
    Spectrum {
        red: 0.3602844,
        green: 0.0692472,
        blue: 0.4314973,
    },
    Spectrum {
        red: 0.3665292,
        green: 0.0715785,
        blue: 0.4319942,
    },
    Spectrum {
        red: 0.3727676,
        green: 0.0739149,
        blue: 0.4324004,
    },
    Spectrum {
        red: 0.3790007,
        green: 0.0762531,
        blue: 0.4327192,
    },
    Spectrum {
        red: 0.3852284,
        green: 0.0785915,
        blue: 0.4329550,
    },
    Spectrum {
        red: 0.3914527,
        green: 0.0809267,
        blue: 0.4331088,
    },
    Spectrum {
        red: 0.3976744,
        green: 0.0832568,
        blue: 0.4331826,
    },
    Spectrum {
        red: 0.4038943,
        green: 0.0855803,
        blue: 0.4331785,
    },
    Spectrum {
        red: 0.4101130,
        green: 0.0878962,
        blue: 0.4330981,
    },
    Spectrum {
        red: 0.4163312,
        green: 0.0902034,
        blue: 0.4329427,
    },
    Spectrum {
        red: 0.4225492,
        green: 0.0925015,
        blue: 0.4327136,
    },
    Spectrum {
        red: 0.4287677,
        green: 0.0947899,
        blue: 0.4324120,
    },
    Spectrum {
        red: 0.4349869,
        green: 0.0970686,
        blue: 0.4320387,
    },
    Spectrum {
        red: 0.4412071,
        green: 0.0993376,
        blue: 0.4315944,
    },
    Spectrum {
        red: 0.4474284,
        green: 0.1015971,
        blue: 0.4310805,
    },
    Spectrum {
        red: 0.4536506,
        green: 0.1038477,
        blue: 0.4304979,
    },
    Spectrum {
        red: 0.4598746,
        green: 0.1060892,
        blue: 0.4298458,
    },
    Spectrum {
        red: 0.4661005,
        green: 0.1083219,
        blue: 0.4291245,
    },
    Spectrum {
        red: 0.4723283,
        green: 0.1105466,
        blue: 0.4283343,
    },
    Spectrum {
        red: 0.4785579,
        green: 0.1127638,
        blue: 0.4274754,
    },
    Spectrum {
        red: 0.4847893,
        green: 0.1149744,
        blue: 0.4265480,
    },
    Spectrum {
        red: 0.4910224,
        green: 0.1171792,
        blue: 0.4255521,
    },
    Spectrum {
        red: 0.4972571,
        green: 0.1193791,
        blue: 0.4244879,
    },
    Spectrum {
        red: 0.5034927,
        green: 0.1215754,
        blue: 0.4233561,
    },
    Spectrum {
        red: 0.5097295,
        green: 0.1237687,
        blue: 0.4221557,
    },
    Spectrum {
        red: 0.5159673,
        green: 0.1259599,
        blue: 0.4208866,
    },
    Spectrum {
        red: 0.5222056,
        green: 0.1281504,
        blue: 0.4195488,
    },
    Spectrum {
        red: 0.5284442,
        green: 0.1303413,
        blue: 0.4181424,
    },
    Spectrum {
        red: 0.5346825,
        green: 0.1325338,
        blue: 0.4166673,
    },
    Spectrum {
        red: 0.5409202,
        green: 0.1347293,
        blue: 0.4151234,
    },
    Spectrum {
        red: 0.5471567,
        green: 0.1369290,
        blue: 0.4135107,
    },
    Spectrum {
        red: 0.5533916,
        green: 0.1391341,
        blue: 0.4118289,
    },
    Spectrum {
        red: 0.5596244,
        green: 0.1413463,
        blue: 0.4100780,
    },
    Spectrum {
        red: 0.5658545,
        green: 0.1435668,
        blue: 0.4082581,
    },
    Spectrum {
        red: 0.5720811,
        green: 0.1457972,
        blue: 0.4063692,
    },
    Spectrum {
        red: 0.5783037,
        green: 0.1480389,
        blue: 0.4044114,
    },
    Spectrum {
        red: 0.5845214,
        green: 0.1502937,
        blue: 0.4023848,
    },
    Spectrum {
        red: 0.5907336,
        green: 0.1525630,
        blue: 0.4002895,
    },
    Spectrum {
        red: 0.5969398,
        green: 0.1548482,
        blue: 0.3981249,
    },
    Spectrum {
        red: 0.6031389,
        green: 0.1571512,
        blue: 0.3958913,
    },
    Spectrum {
        red: 0.6093302,
        green: 0.1594735,
        blue: 0.3935893,
    },
    Spectrum {
        red: 0.6155126,
        green: 0.1618171,
        blue: 0.3912193,
    },
    Spectrum {
        red: 0.6216853,
        green: 0.1641836,
        blue: 0.3887815,
    },
    Spectrum {
        red: 0.6278474,
        green: 0.1665747,
        blue: 0.3862762,
    },
    Spectrum {
        red: 0.6339977,
        green: 0.1689923,
        blue: 0.3837039,
    },
    Spectrum {
        red: 0.6401354,
        green: 0.1714382,
        blue: 0.3810649,
    },
    Spectrum {
        red: 0.6462596,
        green: 0.1739139,
        blue: 0.3783590,
    },
    Spectrum {
        red: 0.6523693,
        green: 0.1764213,
        blue: 0.3755862,
    },
    Spectrum {
        red: 0.6584632,
        green: 0.1789624,
        blue: 0.3727482,
    },
    Spectrum {
        red: 0.6645400,
        green: 0.1815391,
        blue: 0.3698456,
    },
    Spectrum {
        red: 0.6705986,
        green: 0.1841533,
        blue: 0.3668790,
    },
    Spectrum {
        red: 0.6766378,
        green: 0.1868067,
        blue: 0.3638492,
    },
    Spectrum {
        red: 0.6826564,
        green: 0.1895014,
        blue: 0.3607569,
    },
    Spectrum {
        red: 0.6886532,
        green: 0.1922390,
        blue: 0.3576028,
    },
    Spectrum {
        red: 0.6946268,
        green: 0.1950215,
        blue: 0.3543879,
    },
    Spectrum {
        red: 0.7005759,
        green: 0.1978507,
        blue: 0.3511129,
    },
    Spectrum {
        red: 0.7064997,
        green: 0.2007282,
        blue: 0.3477769,
    },
    Spectrum {
        red: 0.7123963,
        green: 0.2036560,
        blue: 0.3443826,
    },
    Spectrum {
        red: 0.7182644,
        green: 0.2066360,
        blue: 0.3409312,
    },
    Spectrum {
        red: 0.7241026,
        green: 0.2096698,
        blue: 0.3374238,
    },
    Spectrum {
        red: 0.7299094,
        green: 0.2127593,
        blue: 0.3338614,
    },
    Spectrum {
        red: 0.7356834,
        green: 0.2159060,
        blue: 0.3302451,
    },
    Spectrum {
        red: 0.7414232,
        green: 0.2191116,
        blue: 0.3265763,
    },
    Spectrum {
        red: 0.7471272,
        green: 0.2223777,
        blue: 0.3228560,
    },
    Spectrum {
        red: 0.7527940,
        green: 0.2257058,
        blue: 0.3190854,
    },
    Spectrum {
        red: 0.7584221,
        green: 0.2290975,
        blue: 0.3152659,
    },
    Spectrum {
        red: 0.7640099,
        green: 0.2325541,
        blue: 0.3113987,
    },
    Spectrum {
        red: 0.7695560,
        green: 0.2360770,
        blue: 0.3074852,
    },
    Spectrum {
        red: 0.7750589,
        green: 0.2396674,
        blue: 0.3035263,
    },
    Spectrum {
        red: 0.7805170,
        green: 0.2433267,
        blue: 0.2995227,
    },
    Spectrum {
        red: 0.7859288,
        green: 0.2470560,
        blue: 0.2954768,
    },
    Spectrum {
        red: 0.7912927,
        green: 0.2508562,
        blue: 0.2913899,
    },
    Spectrum {
        red: 0.7966071,
        green: 0.2547285,
        blue: 0.2872636,
    },
    Spectrum {
        red: 0.8018707,
        green: 0.2586736,
        blue: 0.2830990,
    },
    Spectrum {
        red: 0.8070818,
        green: 0.2626924,
        blue: 0.2788976,
    },
    Spectrum {
        red: 0.8122390,
        green: 0.2667856,
        blue: 0.2746607,
    },
    Spectrum {
        red: 0.8173408,
        green: 0.2709537,
        blue: 0.2703895,
    },
    Spectrum {
        red: 0.8223858,
        green: 0.2751973,
        blue: 0.2660854,
    },
    Spectrum {
        red: 0.8273725,
        green: 0.2795168,
        blue: 0.2617496,
    },
    Spectrum {
        red: 0.8322995,
        green: 0.2839125,
        blue: 0.2573833,
    },
    Spectrum {
        red: 0.8371654,
        green: 0.2883846,
        blue: 0.2529877,
    },
    Spectrum {
        red: 0.8419690,
        green: 0.2929333,
        blue: 0.2485638,
    },
    Spectrum {
        red: 0.8467088,
        green: 0.2975585,
        blue: 0.2441128,
    },
    Spectrum {
        red: 0.8513836,
        green: 0.3022602,
        blue: 0.2396355,
    },
    Spectrum {
        red: 0.8559921,
        green: 0.3070382,
        blue: 0.2351330,
    },
    Spectrum {
        red: 0.8605332,
        green: 0.3118922,
        blue: 0.2306060,
    },
    Spectrum {
        red: 0.8650057,
        green: 0.3168218,
        blue: 0.2260554,
    },
    Spectrum {
        red: 0.8694085,
        green: 0.3218267,
        blue: 0.2214817,
    },
    Spectrum {
        red: 0.8737405,
        green: 0.3269062,
        blue: 0.2168857,
    },
    Spectrum {
        red: 0.8780007,
        green: 0.3320598,
        blue: 0.2122678,
    },
    Spectrum {
        red: 0.8821881,
        green: 0.3372867,
        blue: 0.2076283,
    },
    Spectrum {
        red: 0.8863018,
        green: 0.3425861,
        blue: 0.2029677,
    },
    Spectrum {
        red: 0.8903409,
        green: 0.3479573,
        blue: 0.1982861,
    },
    Spectrum {
        red: 0.8943046,
        green: 0.3533994,
        blue: 0.1935836,
    },
    Spectrum {
        red: 0.8981920,
        green: 0.3589112,
        blue: 0.1888602,
    },
    Spectrum {
        red: 0.9020025,
        green: 0.3644919,
        blue: 0.1841159,
    },
    Spectrum {
        red: 0.9057354,
        green: 0.3701404,
        blue: 0.1793504,
    },
    Spectrum {
        red: 0.9093901,
        green: 0.3758555,
        blue: 0.1745635,
    },
    Spectrum {
        red: 0.9129659,
        green: 0.3816361,
        blue: 0.1697548,
    },
    Spectrum {
        red: 0.9164623,
        green: 0.3874810,
        blue: 0.1649238,
    },
    Spectrum {
        red: 0.9198787,
        green: 0.3933890,
        blue: 0.1600702,
    },
    Spectrum {
        red: 0.9232148,
        green: 0.3993589,
        blue: 0.1551932,
    },
    Spectrum {
        red: 0.9264700,
        green: 0.4053893,
        blue: 0.1502923,
    },
    Spectrum {
        red: 0.9296441,
        green: 0.4114790,
        blue: 0.1453670,
    },
    Spectrum {
        red: 0.9327366,
        green: 0.4176268,
        blue: 0.1404165,
    },
    Spectrum {
        red: 0.9357471,
        green: 0.4238312,
        blue: 0.1354404,
    },
    Spectrum {
        red: 0.9386755,
        green: 0.4300912,
        blue: 0.1304382,
    },
    Spectrum {
        red: 0.9415214,
        green: 0.4364052,
        blue: 0.1254094,
    },
    Spectrum {
        red: 0.9442845,
        green: 0.4427722,
        blue: 0.1203540,
    },
    Spectrum {
        red: 0.9469647,
        green: 0.4491908,
        blue: 0.1152721,
    },
    Spectrum {
        red: 0.9495618,
        green: 0.4556597,
        blue: 0.1101639,
    },
    Spectrum {
        red: 0.9520754,
        green: 0.4621777,
        blue: 0.1050306,
    },
    Spectrum {
        red: 0.9545055,
        green: 0.4687435,
        blue: 0.0998736,
    },
    Spectrum {
        red: 0.9568519,
        green: 0.4753560,
        blue: 0.0946952,
    },
    Spectrum {
        red: 0.9591144,
        green: 0.4820140,
        blue: 0.0894989,
    },
    Spectrum {
        red: 0.9612929,
        green: 0.4887163,
        blue: 0.0842894,
    },
    Spectrum {
        red: 0.9633871,
        green: 0.4954618,
        blue: 0.0790732,
    },
    Spectrum {
        red: 0.9653970,
        green: 0.5022493,
        blue: 0.0738591,
    },
    Spectrum {
        red: 0.9673225,
        green: 0.5090778,
        blue: 0.0686589,
    },
    Spectrum {
        red: 0.9691633,
        green: 0.5159461,
        blue: 0.0634882,
    },
    Spectrum {
        red: 0.9709193,
        green: 0.5228533,
        blue: 0.0583675,
    },
    Spectrum {
        red: 0.9725904,
        green: 0.5297982,
        blue: 0.0533237,
    },
    Spectrum {
        red: 0.9741763,
        green: 0.5367801,
        blue: 0.0483920,
    },
    Spectrum {
        red: 0.9756770,
        green: 0.5437977,
        blue: 0.0436178,
    },
    Spectrum {
        red: 0.9770923,
        green: 0.5508503,
        blue: 0.0390500,
    },
    Spectrum {
        red: 0.9784220,
        green: 0.5579369,
        blue: 0.0349306,
    },
    Spectrum {
        red: 0.9796658,
        green: 0.5650566,
        blue: 0.0314092,
    },
    Spectrum {
        red: 0.9808237,
        green: 0.5722085,
        blue: 0.0285076,
    },
    Spectrum {
        red: 0.9818953,
        green: 0.5793918,
        blue: 0.0262497,
    },
    Spectrum {
        red: 0.9828805,
        green: 0.5866056,
        blue: 0.0246613,
    },
    Spectrum {
        red: 0.9837791,
        green: 0.5938492,
        blue: 0.0237702,
    },
    Spectrum {
        red: 0.9845908,
        green: 0.6011216,
        blue: 0.0236064,
    },
    Spectrum {
        red: 0.9853153,
        green: 0.6084222,
        blue: 0.0242021,
    },
    Spectrum {
        red: 0.9859525,
        green: 0.6157501,
        blue: 0.0255922,
    },
    Spectrum {
        red: 0.9865020,
        green: 0.6231047,
        blue: 0.0278139,
    },
    Spectrum {
        red: 0.9869637,
        green: 0.6304850,
        blue: 0.0309075,
    },
    Spectrum {
        red: 0.9873372,
        green: 0.6378904,
        blue: 0.0349161,
    },
    Spectrum {
        red: 0.9876223,
        green: 0.6453202,
        blue: 0.0398857,
    },
    Spectrum {
        red: 0.9878188,
        green: 0.6527734,
        blue: 0.0455808,
    },
    Spectrum {
        red: 0.9879263,
        green: 0.6602495,
        blue: 0.0517504,
    },
    Spectrum {
        red: 0.9879448,
        green: 0.6677476,
        blue: 0.0583287,
    },
    Spectrum {
        red: 0.9878739,
        green: 0.6752670,
        blue: 0.0652570,
    },
    Spectrum {
        red: 0.9877135,
        green: 0.6828068,
        blue: 0.0724892,
    },
    Spectrum {
        red: 0.9874635,
        green: 0.6903662,
        blue: 0.0799897,
    },
    Spectrum {
        red: 0.9871238,
        green: 0.6979444,
        blue: 0.0877314,
    },
    Spectrum {
        red: 0.9866942,
        green: 0.7055404,
        blue: 0.0956942,
    },
    Spectrum {
        red: 0.9861750,
        green: 0.7131534,
        blue: 0.1038633,
    },
    Spectrum {
        red: 0.9855657,
        green: 0.7207825,
        blue: 0.1122288,
    },
    Spectrum {
        red: 0.9848652,
        green: 0.7284275,
        blue: 0.1207847,
    },
    Spectrum {
        red: 0.9840751,
        green: 0.7360865,
        blue: 0.1295266,
    },
    Spectrum {
        red: 0.9831960,
        green: 0.7437583,
        blue: 0.1384531,
    },
    Spectrum {
        red: 0.9822285,
        green: 0.7514416,
        blue: 0.1475646,
    },
    Spectrum {
        red: 0.9811735,
        green: 0.7591349,
        blue: 0.1568632,
    },
    Spectrum {
        red: 0.9800322,
        green: 0.7668366,
        blue: 0.1663525,
    },
    Spectrum {
        red: 0.9788062,
        green: 0.7745450,
        blue: 0.1760373,
    },
    Spectrum {
        red: 0.9774975,
        green: 0.7822581,
        blue: 0.1859234,
    },
    Spectrum {
        red: 0.9761085,
        green: 0.7899738,
        blue: 0.1960176,
    },
    Spectrum {
        red: 0.9746378,
        green: 0.7976916,
        blue: 0.2063319,
    },
    Spectrum {
        red: 0.9730879,
        green: 0.8054093,
        blue: 0.2168768,
    },
    Spectrum {
        red: 0.9714678,
        green: 0.8131217,
        blue: 0.2276580,
    },
    Spectrum {
        red: 0.9697831,
        green: 0.8208251,
        blue: 0.2386859,
    },
    Spectrum {
        red: 0.9680408,
        green: 0.8285155,
        blue: 0.2499716,
    },
    Spectrum {
        red: 0.9662426,
        green: 0.8361910,
        blue: 0.2615339,
    },
    Spectrum {
        red: 0.9643939,
        green: 0.8438481,
        blue: 0.2733911,
    },
    Spectrum {
        red: 0.9625167,
        green: 0.8514763,
        blue: 0.2855457,
    },
    Spectrum {
        red: 0.9606255,
        green: 0.8590687,
        blue: 0.2980102,
    },
    Spectrum {
        red: 0.9587201,
        green: 0.8666244,
        blue: 0.3108205,
    },
    Spectrum {
        red: 0.9568341,
        green: 0.8741286,
        blue: 0.3239739,
    },
    Spectrum {
        red: 0.9549972,
        green: 0.8815689,
        blue: 0.3374755,
    },
    Spectrum {
        red: 0.9532151,
        green: 0.8889423,
        blue: 0.3513687,
    },
    Spectrum {
        red: 0.9515462,
        green: 0.8962259,
        blue: 0.3656270,
    },
    Spectrum {
        red: 0.9500185,
        green: 0.9034091,
        blue: 0.3802712,
    },
    Spectrum {
        red: 0.9486834,
        green: 0.9104730,
        blue: 0.3952892,
    },
    Spectrum {
        red: 0.9475944,
        green: 0.9173991,
        blue: 0.4106652,
    },
    Spectrum {
        red: 0.9468092,
        green: 0.9241682,
        blue: 0.4263732,
    },
    Spectrum {
        red: 0.9463915,
        green: 0.9307608,
        blue: 0.4423675,
    },
    Spectrum {
        red: 0.9464030,
        green: 0.9371590,
        blue: 0.4585915,
    },
    Spectrum {
        red: 0.9469026,
        green: 0.9433478,
        blue: 0.4749698,
    },
    Spectrum {
        red: 0.9479368,
        green: 0.9493175,
        blue: 0.4914261,
    },
    Spectrum {
        red: 0.9495448,
        green: 0.9550629,
        blue: 0.5078596,
    },
    Spectrum {
        red: 0.9517403,
        green: 0.9605867,
        blue: 0.5242030,
    },
    Spectrum {
        red: 0.9545293,
        green: 0.9658959,
        blue: 0.5403608,
    },
    Spectrum {
        red: 0.9578961,
        green: 0.9710033,
        blue: 0.5562751,
    },
    Spectrum {
        red: 0.9618120,
        green: 0.9759242,
        blue: 0.5719254,
    },
    Spectrum {
        red: 0.9662488,
        green: 0.9806782,
        blue: 0.5872058,
    },
    Spectrum {
        red: 0.9711616,
        green: 0.9852822,
        blue: 0.6021543,
    },
    Spectrum {
        red: 0.9765110,
        green: 0.9897534,
        blue: 0.6167604,
    },
    Spectrum {
        red: 0.9822573,
        green: 0.9941088,
        blue: 0.6310170,
    },
    Spectrum {
        red: 0.9883621,
        green: 0.9983641,
        blue: 0.6449240,
    },
];
