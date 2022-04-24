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

pub const VIRIDIS_COLOURMAP: [Spectrum; 256] = [
    Spectrum {
        red: 0.2670040,
        green: 0.0048743,
        blue: 0.3294152,
    },
    Spectrum {
        red: 0.2685105,
        green: 0.0096048,
        blue: 0.3354265,
    },
    Spectrum {
        red: 0.2699438,
        green: 0.0146249,
        blue: 0.3413790,
    },
    Spectrum {
        red: 0.2713049,
        green: 0.0199419,
        blue: 0.3472686,
    },
    Spectrum {
        red: 0.2725938,
        green: 0.0255631,
        blue: 0.3530930,
    },
    Spectrum {
        red: 0.2738093,
        green: 0.0314975,
        blue: 0.3588526,
    },
    Spectrum {
        red: 0.2749524,
        green: 0.0377518,
        blue: 0.3645432,
    },
    Spectrum {
        red: 0.2760224,
        green: 0.0441672,
        blue: 0.3701642,
    },
    Spectrum {
        red: 0.2770184,
        green: 0.0503444,
        blue: 0.3757145,
    },
    Spectrum {
        red: 0.2779414,
        green: 0.0563244,
        blue: 0.3811907,
    },
    Spectrum {
        red: 0.2787907,
        green: 0.0621454,
        blue: 0.3865920,
    },
    Spectrum {
        red: 0.2795655,
        green: 0.0678359,
        blue: 0.3919172,
    },
    Spectrum {
        red: 0.2802666,
        green: 0.0734172,
        blue: 0.3971635,
    },
    Spectrum {
        red: 0.2808936,
        green: 0.0789070,
        blue: 0.4023294,
    },
    Spectrum {
        red: 0.2814458,
        green: 0.0843197,
        blue: 0.4074140,
    },
    Spectrum {
        red: 0.2819236,
        green: 0.0896662,
        blue: 0.4124152,
    },
    Spectrum {
        red: 0.2823274,
        green: 0.0949555,
        blue: 0.4173309,
    },
    Spectrum {
        red: 0.2826563,
        green: 0.1001958,
        blue: 0.4221603,
    },
    Spectrum {
        red: 0.2829105,
        green: 0.1053935,
        blue: 0.4269020,
    },
    Spectrum {
        red: 0.2830910,
        green: 0.1105531,
        blue: 0.4315538,
    },
    Spectrum {
        red: 0.2831970,
        green: 0.1156797,
        blue: 0.4361148,
    },
    Spectrum {
        red: 0.2832288,
        green: 0.1207770,
        blue: 0.4405840,
    },
    Spectrum {
        red: 0.2831868,
        green: 0.1258480,
        blue: 0.4449600,
    },
    Spectrum {
        red: 0.2830720,
        green: 0.1308948,
        blue: 0.4492413,
    },
    Spectrum {
        red: 0.2828839,
        green: 0.1359201,
        blue: 0.4534273,
    },
    Spectrum {
        red: 0.2826230,
        green: 0.1409256,
        blue: 0.4575173,
    },
    Spectrum {
        red: 0.2822904,
        green: 0.1459123,
        blue: 0.4615100,
    },
    Spectrum {
        red: 0.2818868,
        green: 0.1508815,
        blue: 0.4654047,
    },
    Spectrum {
        red: 0.2814123,
        green: 0.1558343,
        blue: 0.4692013,
    },
    Spectrum {
        red: 0.2808677,
        green: 0.1607713,
        blue: 0.4728991,
    },
    Spectrum {
        red: 0.2802547,
        green: 0.1656927,
        blue: 0.4764976,
    },
    Spectrum {
        red: 0.2795740,
        green: 0.1705988,
        blue: 0.4799968,
    },
    Spectrum {
        red: 0.2788262,
        green: 0.1754902,
        blue: 0.4833965,
    },
    Spectrum {
        red: 0.2780124,
        green: 0.1803668,
        blue: 0.4866970,
    },
    Spectrum {
        red: 0.2771344,
        green: 0.1852284,
        blue: 0.4898983,
    },
    Spectrum {
        red: 0.2761938,
        green: 0.1900745,
        blue: 0.4930007,
    },
    Spectrum {
        red: 0.2751912,
        green: 0.1949054,
        blue: 0.4960049,
    },
    Spectrum {
        red: 0.2741280,
        green: 0.1997209,
        blue: 0.4989113,
    },
    Spectrum {
        red: 0.2730060,
        green: 0.2045205,
        blue: 0.5017208,
    },
    Spectrum {
        red: 0.2718281,
        green: 0.2093031,
        blue: 0.5044341,
    },
    Spectrum {
        red: 0.2705947,
        green: 0.2140690,
        blue: 0.5070524,
    },
    Spectrum {
        red: 0.2693076,
        green: 0.2188178,
        blue: 0.5095768,
    },
    Spectrum {
        red: 0.2679685,
        green: 0.2235491,
        blue: 0.5120084,
    },
    Spectrum {
        red: 0.2665798,
        green: 0.2282621,
        blue: 0.5143487,
    },
    Spectrum {
        red: 0.2651445,
        green: 0.2329559,
        blue: 0.5165993,
    },
    Spectrum {
        red: 0.2636632,
        green: 0.2376308,
        blue: 0.5187616,
    },
    Spectrum {
        red: 0.2621380,
        green: 0.2422862,
        blue: 0.5208374,
    },
    Spectrum {
        red: 0.2605710,
        green: 0.2469217,
        blue: 0.5228282,
    },
    Spectrum {
        red: 0.2589645,
        green: 0.2515369,
        blue: 0.5247361,
    },
    Spectrum {
        red: 0.2573224,
        green: 0.2561304,
        blue: 0.5265633,
    },
    Spectrum {
        red: 0.2556452,
        green: 0.2607028,
        blue: 0.5283115,
    },
    Spectrum {
        red: 0.2539350,
        green: 0.2652538,
        blue: 0.5299827,
    },
    Spectrum {
        red: 0.2521940,
        green: 0.2697831,
        blue: 0.5315791,
    },
    Spectrum {
        red: 0.2504246,
        green: 0.2742902,
        blue: 0.5331026,
    },
    Spectrum {
        red: 0.2486290,
        green: 0.2787751,
        blue: 0.5345556,
    },
    Spectrum {
        red: 0.2468114,
        green: 0.2832366,
        blue: 0.5359409,
    },
    Spectrum {
        red: 0.2449721,
        green: 0.2876755,
        blue: 0.5372602,
    },
    Spectrum {
        red: 0.2431132,
        green: 0.2920915,
        blue: 0.5385156,
    },
    Spectrum {
        red: 0.2412371,
        green: 0.2964847,
        blue: 0.5397095,
    },
    Spectrum {
        red: 0.2393458,
        green: 0.3008549,
        blue: 0.5408440,
    },
    Spectrum {
        red: 0.2374414,
        green: 0.3052022,
        blue: 0.5419214,
    },
    Spectrum {
        red: 0.2355261,
        green: 0.3095266,
        blue: 0.5429440,
    },
    Spectrum {
        red: 0.2336028,
        green: 0.3138277,
        blue: 0.5439142,
    },
    Spectrum {
        red: 0.2316735,
        green: 0.3181058,
        blue: 0.5448344,
    },
    Spectrum {
        red: 0.2297393,
        green: 0.3223613,
        blue: 0.5457063,
    },
    Spectrum {
        red: 0.2278019,
        green: 0.3265943,
        blue: 0.5465320,
    },
    Spectrum {
        red: 0.2258633,
        green: 0.3308052,
        blue: 0.5473135,
    },
    Spectrum {
        red: 0.2239252,
        green: 0.3349940,
        blue: 0.5480529,
    },
    Spectrum {
        red: 0.2219892,
        green: 0.3391611,
        blue: 0.5487521,
    },
    Spectrum {
        red: 0.2200569,
        green: 0.3433069,
        blue: 0.5494130,
    },
    Spectrum {
        red: 0.2181300,
        green: 0.3474315,
        blue: 0.5500376,
    },
    Spectrum {
        red: 0.2162097,
        green: 0.3515355,
        blue: 0.5506274,
    },
    Spectrum {
        red: 0.2142976,
        green: 0.3556191,
        blue: 0.5511844,
    },
    Spectrum {
        red: 0.2123948,
        green: 0.3596827,
        blue: 0.5517101,
    },
    Spectrum {
        red: 0.2105031,
        green: 0.3637267,
        blue: 0.5522065,
    },
    Spectrum {
        red: 0.2086234,
        green: 0.3677515,
        blue: 0.5526749,
    },
    Spectrum {
        red: 0.2067563,
        green: 0.3717578,
        blue: 0.5531165,
    },
    Spectrum {
        red: 0.2049026,
        green: 0.3757459,
        blue: 0.5535328,
    },
    Spectrum {
        red: 0.2030631,
        green: 0.3797164,
        blue: 0.5539251,
    },
    Spectrum {
        red: 0.2012385,
        green: 0.3836699,
        blue: 0.5542944,
    },
    Spectrum {
        red: 0.1994295,
        green: 0.3876068,
        blue: 0.5546421,
    },
    Spectrum {
        red: 0.1976365,
        green: 0.3915276,
        blue: 0.5549691,
    },
    Spectrum {
        red: 0.1958599,
        green: 0.3954330,
        blue: 0.5552764,
    },
    Spectrum {
        red: 0.1941001,
        green: 0.3993234,
        blue: 0.5555649,
    },
    Spectrum {
        red: 0.1923572,
        green: 0.4031993,
        blue: 0.5558356,
    },
    Spectrum {
        red: 0.1906314,
        green: 0.4070615,
        blue: 0.5560891,
    },
    Spectrum {
        red: 0.1889226,
        green: 0.4109103,
        blue: 0.5563261,
    },
    Spectrum {
        red: 0.1872308,
        green: 0.4147465,
        blue: 0.5565472,
    },
    Spectrum {
        red: 0.1855559,
        green: 0.4185704,
        blue: 0.5567529,
    },
    Spectrum {
        red: 0.1838976,
        green: 0.4223828,
        blue: 0.5569438,
    },
    Spectrum {
        red: 0.1822556,
        green: 0.4261841,
        blue: 0.5571201,
    },
    Spectrum {
        red: 0.1806295,
        green: 0.4299749,
        blue: 0.5572822,
    },
    Spectrum {
        red: 0.1790188,
        green: 0.4337557,
        blue: 0.5574304,
    },
    Spectrum {
        red: 0.1774230,
        green: 0.4375272,
        blue: 0.5575647,
    },
    Spectrum {
        red: 0.1758415,
        green: 0.4412898,
        blue: 0.5576853,
    },
    Spectrum {
        red: 0.1742736,
        green: 0.4450441,
        blue: 0.5577922,
    },
    Spectrum {
        red: 0.1727188,
        green: 0.4487906,
        blue: 0.5578853,
    },
    Spectrum {
        red: 0.1711762,
        green: 0.4525298,
        blue: 0.5579646,
    },
    Spectrum {
        red: 0.1696457,
        green: 0.4562621,
        blue: 0.5580303,
    },
    Spectrum {
        red: 0.1681264,
        green: 0.4599880,
        blue: 0.5580820,
    },
    Spectrum {
        red: 0.1666171,
        green: 0.4637081,
        blue: 0.5581191,
    },
    Spectrum {
        red: 0.1651170,
        green: 0.4674229,
        blue: 0.5581414,
    },
    Spectrum {
        red: 0.1636254,
        green: 0.4711328,
        blue: 0.5581484,
    },
    Spectrum {
        red: 0.1621416,
        green: 0.4748382,
        blue: 0.5581397,
    },
    Spectrum {
        red: 0.1606647,
        green: 0.4785396,
        blue: 0.5581147,
    },
    Spectrum {
        red: 0.1591941,
        green: 0.4822374,
        blue: 0.5580728,
    },
    Spectrum {
        red: 0.1577293,
        green: 0.4859320,
        blue: 0.5580135,
    },
    Spectrum {
        red: 0.1562697,
        green: 0.4896237,
        blue: 0.5579360,
    },
    Spectrum {
        red: 0.1548149,
        green: 0.4933129,
        blue: 0.5578397,
    },
    Spectrum {
        red: 0.1533645,
        green: 0.4970000,
        blue: 0.5577237,
    },
    Spectrum {
        red: 0.1519182,
        green: 0.5006853,
        blue: 0.5575873,
    },
    Spectrum {
        red: 0.1504761,
        green: 0.5043690,
        blue: 0.5574297,
    },
    Spectrum {
        red: 0.1490392,
        green: 0.5080514,
        blue: 0.5572505,
    },
    Spectrum {
        red: 0.1476073,
        green: 0.5117326,
        blue: 0.5570486,
    },
    Spectrum {
        red: 0.1461803,
        green: 0.5154132,
        blue: 0.5568227,
    },
    Spectrum {
        red: 0.1447586,
        green: 0.5190932,
        blue: 0.5565718,
    },
    Spectrum {
        red: 0.1433433,
        green: 0.5227729,
        blue: 0.5562949,
    },
    Spectrum {
        red: 0.1419353,
        green: 0.5264525,
        blue: 0.5559910,
    },
    Spectrum {
        red: 0.1405360,
        green: 0.5301322,
        blue: 0.5556589,
    },
    Spectrum {
        red: 0.1391471,
        green: 0.5338120,
        blue: 0.5552977,
    },
    Spectrum {
        red: 0.1377705,
        green: 0.5374921,
        blue: 0.5549063,
    },
    Spectrum {
        red: 0.1364085,
        green: 0.5411726,
        blue: 0.5544834,
    },
    Spectrum {
        red: 0.1350656,
        green: 0.5448534,
        blue: 0.5540291,
    },
    Spectrum {
        red: 0.1337430,
        green: 0.5485346,
        blue: 0.5535411,
    },
    Spectrum {
        red: 0.1324440,
        green: 0.5522164,
        blue: 0.5530183,
    },
    Spectrum {
        red: 0.1311725,
        green: 0.5558987,
        blue: 0.5524595,
    },
    Spectrum {
        red: 0.1299327,
        green: 0.5595816,
        blue: 0.5518635,
    },
    Spectrum {
        red: 0.1287294,
        green: 0.5632650,
        blue: 0.5512293,
    },
    Spectrum {
        red: 0.1275677,
        green: 0.5669489,
        blue: 0.5505555,
    },
    Spectrum {
        red: 0.1264534,
        green: 0.5706332,
        blue: 0.5498411,
    },
    Spectrum {
        red: 0.1253938,
        green: 0.5743175,
        blue: 0.5490856,
    },
    Spectrum {
        red: 0.1243947,
        green: 0.5780021,
        blue: 0.5482874,
    },
    Spectrum {
        red: 0.1234628,
        green: 0.5816866,
        blue: 0.5474450,
    },
    Spectrum {
        red: 0.1226056,
        green: 0.5853711,
        blue: 0.5465572,
    },
    Spectrum {
        red: 0.1218312,
        green: 0.5890552,
        blue: 0.5456230,
    },
    Spectrum {
        red: 0.1211481,
        green: 0.5927389,
        blue: 0.5446411,
    },
    Spectrum {
        red: 0.1205650,
        green: 0.5964219,
        blue: 0.5436106,
    },
    Spectrum {
        red: 0.1200915,
        green: 0.6001039,
        blue: 0.5425304,
    },
    Spectrum {
        red: 0.1197376,
        green: 0.6037846,
        blue: 0.5414000,
    },
    Spectrum {
        red: 0.1195116,
        green: 0.6074639,
        blue: 0.5402175,
    },
    Spectrum {
        red: 0.1194234,
        green: 0.6111415,
        blue: 0.5389819,
    },
    Spectrum {
        red: 0.1194826,
        green: 0.6148170,
        blue: 0.5376922,
    },
    Spectrum {
        red: 0.1196986,
        green: 0.6184903,
        blue: 0.5363473,
    },
    Spectrum {
        red: 0.1200808,
        green: 0.6221608,
        blue: 0.5349463,
    },
    Spectrum {
        red: 0.1206382,
        green: 0.6258283,
        blue: 0.5334883,
    },
    Spectrum {
        red: 0.1213797,
        green: 0.6294924,
        blue: 0.5319728,
    },
    Spectrum {
        red: 0.1223124,
        green: 0.6331528,
        blue: 0.5303981,
    },
    Spectrum {
        red: 0.1234436,
        green: 0.6368090,
        blue: 0.5287634,
    },
    Spectrum {
        red: 0.1247795,
        green: 0.6404607,
        blue: 0.5270679,
    },
    Spectrum {
        red: 0.1263258,
        green: 0.6441074,
        blue: 0.5253107,
    },
    Spectrum {
        red: 0.1280870,
        green: 0.6477488,
        blue: 0.5234909,
    },
    Spectrum {
        red: 0.1300669,
        green: 0.6513844,
        blue: 0.5216079,
    },
    Spectrum {
        red: 0.1322680,
        green: 0.6550136,
        blue: 0.5196609,
    },
    Spectrum {
        red: 0.1346918,
        green: 0.6586362,
        blue: 0.5176488,
    },
    Spectrum {
        red: 0.1373392,
        green: 0.6622516,
        blue: 0.5155710,
    },
    Spectrum {
        red: 0.1402099,
        green: 0.6658593,
        blue: 0.5134268,
    },
    Spectrum {
        red: 0.1433029,
        green: 0.6694588,
        blue: 0.5112155,
    },
    Spectrum {
        red: 0.1466164,
        green: 0.6730497,
        blue: 0.5089364,
    },
    Spectrum {
        red: 0.1501478,
        green: 0.6766314,
        blue: 0.5065889,
    },
    Spectrum {
        red: 0.1538941,
        green: 0.6802034,
        blue: 0.5041722,
    },
    Spectrum {
        red: 0.1578515,
        green: 0.6837653,
        blue: 0.5016857,
    },
    Spectrum {
        red: 0.1620160,
        green: 0.6873163,
        blue: 0.4991291,
    },
    Spectrum {
        red: 0.1663832,
        green: 0.6908561,
        blue: 0.4965016,
    },
    Spectrum {
        red: 0.1709484,
        green: 0.6943841,
        blue: 0.4938029,
    },
    Spectrum {
        red: 0.1757067,
        green: 0.6978996,
        blue: 0.4910325,
    },
    Spectrum {
        red: 0.1806531,
        green: 0.7014022,
        blue: 0.4881894,
    },
    Spectrum {
        red: 0.1857827,
        green: 0.7048913,
        blue: 0.4852733,
    },
    Spectrum {
        red: 0.1910902,
        green: 0.7083664,
        blue: 0.4822840,
    },
    Spectrum {
        red: 0.1965706,
        green: 0.7118267,
        blue: 0.4792211,
    },
    Spectrum {
        red: 0.2022190,
        green: 0.7152718,
        blue: 0.4760843,
    },
    Spectrum {
        red: 0.2080305,
        green: 0.7187010,
        blue: 0.4728733,
    },
    Spectrum {
        red: 0.2140002,
        green: 0.7221137,
        blue: 0.4695877,
    },
    Spectrum {
        red: 0.2201238,
        green: 0.7255095,
        blue: 0.4662264,
    },
    Spectrum {
        red: 0.2263969,
        green: 0.7288875,
        blue: 0.4627893,
    },
    Spectrum {
        red: 0.2328150,
        green: 0.7322474,
        blue: 0.4592768,
    },
    Spectrum {
        red: 0.2393739,
        green: 0.7355883,
        blue: 0.4556884,
    },
    Spectrum {
        red: 0.2460697,
        green: 0.7389097,
        blue: 0.4520241,
    },
    Spectrum {
        red: 0.2528985,
        green: 0.7422110,
        blue: 0.4482836,
    },
    Spectrum {
        red: 0.2598568,
        green: 0.7454916,
        blue: 0.4444667,
    },
    Spectrum {
        red: 0.2669413,
        green: 0.7487508,
        blue: 0.4405728,
    },
    Spectrum {
        red: 0.2741492,
        green: 0.7519881,
        blue: 0.4366009,
    },
    Spectrum {
        red: 0.2814768,
        green: 0.7552027,
        blue: 0.4325521,
    },
    Spectrum {
        red: 0.2889210,
        green: 0.7583940,
        blue: 0.4284263,
    },
    Spectrum {
        red: 0.2964790,
        green: 0.7615614,
        blue: 0.4242234,
    },
    Spectrum {
        red: 0.3041480,
        green: 0.7647043,
        blue: 0.4199435,
    },
    Spectrum {
        red: 0.3119253,
        green: 0.7678221,
        blue: 0.4155864,
    },
    Spectrum {
        red: 0.3198086,
        green: 0.7709140,
        blue: 0.4111522,
    },
    Spectrum {
        red: 0.3277958,
        green: 0.7739795,
        blue: 0.4066401,
    },
    Spectrum {
        red: 0.3358854,
        green: 0.7770179,
        blue: 0.4020492,
    },
    Spectrum {
        red: 0.3440741,
        green: 0.7800286,
        blue: 0.3973810,
    },
    Spectrum {
        red: 0.3523599,
        green: 0.7830109,
        blue: 0.3926358,
    },
    Spectrum {
        red: 0.3607405,
        green: 0.7859642,
        blue: 0.3878135,
    },
    Spectrum {
        red: 0.3692142,
        green: 0.7888879,
        blue: 0.3829144,
    },
    Spectrum {
        red: 0.3777789,
        green: 0.7917815,
        blue: 0.3779385,
    },
    Spectrum {
        red: 0.3864328,
        green: 0.7946442,
        blue: 0.3728861,
    },
    Spectrum {
        red: 0.3951741,
        green: 0.7974754,
        blue: 0.3677573,
    },
    Spectrum {
        red: 0.4040010,
        green: 0.8002746,
        blue: 0.3625522,
    },
    Spectrum {
        red: 0.4129135,
        green: 0.8030410,
        blue: 0.3572689,
    },
    Spectrum {
        red: 0.4219081,
        green: 0.8057741,
        blue: 0.3519101,
    },
    Spectrum {
        red: 0.4309832,
        green: 0.8084734,
        blue: 0.3464761,
    },
    Spectrum {
        red: 0.4401369,
        green: 0.8111384,
        blue: 0.3409673,
    },
    Spectrum {
        red: 0.4493676,
        green: 0.8137684,
        blue: 0.3353843,
    },
    Spectrum {
        red: 0.4586736,
        green: 0.8163629,
        blue: 0.3297275,
    },
    Spectrum {
        red: 0.4680531,
        green: 0.8189214,
        blue: 0.3239976,
    },
    Spectrum {
        red: 0.4775045,
        green: 0.8214435,
        blue: 0.3181953,
    },
    Spectrum {
        red: 0.4870258,
        green: 0.8239286,
        blue: 0.3123213,
    },
    Spectrum {
        red: 0.4966154,
        green: 0.8263763,
        blue: 0.3063766,
    },
    Spectrum {
        red: 0.5062713,
        green: 0.8287862,
        blue: 0.3003621,
    },
    Spectrum {
        red: 0.5159918,
        green: 0.8311578,
        blue: 0.2942789,
    },
    Spectrum {
        red: 0.5257762,
        green: 0.8334906,
        blue: 0.2881265,
    },
    Spectrum {
        red: 0.5356211,
        green: 0.8357845,
        blue: 0.2819083,
    },
    Spectrum {
        red: 0.5455244,
        green: 0.8380392,
        blue: 0.2756260,
    },
    Spectrum {
        red: 0.5554840,
        green: 0.8402544,
        blue: 0.2692815,
    },
    Spectrum {
        red: 0.5654976,
        green: 0.8424299,
        blue: 0.2628768,
    },
    Spectrum {
        red: 0.5755630,
        green: 0.8445656,
        blue: 0.2564146,
    },
    Spectrum {
        red: 0.5856777,
        green: 0.8466614,
        blue: 0.2498975,
    },
    Spectrum {
        red: 0.5958393,
        green: 0.8487172,
        blue: 0.2433288,
    },
    Spectrum {
        red: 0.6060453,
        green: 0.8507331,
        blue: 0.2367121,
    },
    Spectrum {
        red: 0.6162928,
        green: 0.8527091,
        blue: 0.2300518,
    },
    Spectrum {
        red: 0.6265792,
        green: 0.8546454,
        blue: 0.2233526,
    },
    Spectrum {
        red: 0.6369016,
        green: 0.8565423,
        blue: 0.2166201,
    },
    Spectrum {
        red: 0.6472569,
        green: 0.8583999,
        blue: 0.2098609,
    },
    Spectrum {
        red: 0.6576420,
        green: 0.8602188,
        blue: 0.2030823,
    },
    Spectrum {
        red: 0.6680537,
        green: 0.8619993,
        blue: 0.1962931,
    },
    Spectrum {
        red: 0.6784887,
        green: 0.8637421,
        blue: 0.1895033,
    },
    Spectrum {
        red: 0.6889435,
        green: 0.8654478,
        blue: 0.1827246,
    },
    Spectrum {
        red: 0.6994146,
        green: 0.8671171,
        blue: 0.1759706,
    },
    Spectrum {
        red: 0.7098984,
        green: 0.8687509,
        blue: 0.1692571,
    },
    Spectrum {
        red: 0.7203912,
        green: 0.8703502,
        blue: 0.1626027,
    },
    Spectrum {
        red: 0.7308890,
        green: 0.8719158,
        blue: 0.1560289,
    },
    Spectrum {
        red: 0.7413880,
        green: 0.8734492,
        blue: 0.1495610,
    },
    Spectrum {
        red: 0.7518841,
        green: 0.8749514,
        blue: 0.1432283,
    },
    Spectrum {
        red: 0.7623734,
        green: 0.8764239,
        blue: 0.1370645,
    },
    Spectrum {
        red: 0.7728518,
        green: 0.8778681,
        blue: 0.1311086,
    },
    Spectrum {
        red: 0.7833154,
        green: 0.8792855,
        blue: 0.1254054,
    },
    Spectrum {
        red: 0.7937599,
        green: 0.8806776,
        blue: 0.1200053,
    },
    Spectrum {
        red: 0.8041816,
        green: 0.8820463,
        blue: 0.1149651,
    },
    Spectrum {
        red: 0.8145763,
        green: 0.8833933,
        blue: 0.1103468,
    },
    Spectrum {
        red: 0.8249403,
        green: 0.8847204,
        blue: 0.1062172,
    },
    Spectrum {
        red: 0.8352696,
        green: 0.8860294,
        blue: 0.1026459,
    },
    Spectrum {
        red: 0.8455606,
        green: 0.8873224,
        blue: 0.0997022,
    },
    Spectrum {
        red: 0.8558096,
        green: 0.8886013,
        blue: 0.0974519,
    },
    Spectrum {
        red: 0.8660133,
        green: 0.8898682,
        blue: 0.0959528,
    },
    Spectrum {
        red: 0.8761682,
        green: 0.8911249,
        blue: 0.0952505,
    },
    Spectrum {
        red: 0.8862715,
        green: 0.8923735,
        blue: 0.0953744,
    },
    Spectrum {
        red: 0.8963200,
        green: 0.8936161,
        blue: 0.0963354,
    },
    Spectrum {
        red: 0.9063112,
        green: 0.8948547,
        blue: 0.0981250,
    },
    Spectrum {
        red: 0.9162421,
        green: 0.8960913,
        blue: 0.1007168,
    },
    Spectrum {
        red: 0.9261058,
        green: 0.8973298,
        blue: 0.1040707,
    },
    Spectrum {
        red: 0.9359044,
        green: 0.8985704,
        blue: 0.1081309,
    },
    Spectrum {
        red: 0.9456363,
        green: 0.8998150,
        blue: 0.1128377,
    },
    Spectrum {
        red: 0.9552997,
        green: 0.9010653,
        blue: 0.1181283,
    },
    Spectrum {
        red: 0.9648935,
        green: 0.9023231,
        blue: 0.1239405,
    },
    Spectrum {
        red: 0.9744167,
        green: 0.9035899,
        blue: 0.1302149,
    },
    Spectrum {
        red: 0.9838683,
        green: 0.9048673,
        blue: 0.1368967,
    },
    Spectrum {
        red: 0.9932479,
        green: 0.9061566,
        blue: 0.1439362,
    },
];
