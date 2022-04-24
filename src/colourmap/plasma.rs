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

pub const PLASMA_COLOURMAP: [Spectrum; 256] = [
    Spectrum {
        red: 0.0503832,
        green: 0.0298029,
        blue: 0.5279749,
    },
    Spectrum {
        red: 0.0635364,
        green: 0.0284260,
        blue: 0.5331237,
    },
    Spectrum {
        red: 0.0753531,
        green: 0.0272064,
        blue: 0.5380070,
    },
    Spectrum {
        red: 0.0862218,
        green: 0.0261253,
        blue: 0.5426577,
    },
    Spectrum {
        red: 0.0963786,
        green: 0.0251651,
        blue: 0.5471035,
    },
    Spectrum {
        red: 0.1059797,
        green: 0.0243092,
        blue: 0.5513679,
    },
    Spectrum {
        red: 0.1151236,
        green: 0.0235563,
        blue: 0.5554677,
    },
    Spectrum {
        red: 0.1239029,
        green: 0.0228781,
        blue: 0.5594235,
    },
    Spectrum {
        red: 0.1323807,
        green: 0.0222584,
        blue: 0.5632501,
    },
    Spectrum {
        red: 0.1406031,
        green: 0.0216867,
        blue: 0.5669595,
    },
    Spectrum {
        red: 0.1486065,
        green: 0.0211536,
        blue: 0.5705617,
    },
    Spectrum {
        red: 0.1564206,
        green: 0.0206507,
        blue: 0.5740654,
    },
    Spectrum {
        red: 0.1640697,
        green: 0.0201705,
        blue: 0.5774781,
    },
    Spectrum {
        red: 0.1715739,
        green: 0.0197063,
        blue: 0.5808059,
    },
    Spectrum {
        red: 0.1789502,
        green: 0.0192522,
        blue: 0.5840542,
    },
    Spectrum {
        red: 0.1862130,
        green: 0.0188030,
        blue: 0.5872277,
    },
    Spectrum {
        red: 0.1933744,
        green: 0.0183541,
        blue: 0.5903300,
    },
    Spectrum {
        red: 0.2004453,
        green: 0.0179016,
        blue: 0.5933643,
    },
    Spectrum {
        red: 0.2074346,
        green: 0.0174421,
        blue: 0.5963333,
    },
    Spectrum {
        red: 0.2143503,
        green: 0.0169729,
        blue: 0.5992392,
    },
    Spectrum {
        red: 0.2211968,
        green: 0.0164970,
        blue: 0.6020833,
    },
    Spectrum {
        red: 0.2279830,
        green: 0.0160072,
        blue: 0.6048674,
    },
    Spectrum {
        red: 0.2347145,
        green: 0.0155015,
        blue: 0.6075924,
    },
    Spectrum {
        red: 0.2413963,
        green: 0.0149791,
        blue: 0.6102591,
    },
    Spectrum {
        red: 0.2480324,
        green: 0.0144394,
        blue: 0.6128677,
    },
    Spectrum {
        red: 0.2546267,
        green: 0.0138821,
        blue: 0.6154185,
    },
    Spectrum {
        red: 0.2611826,
        green: 0.0133075,
        blue: 0.6179114,
    },
    Spectrum {
        red: 0.2677030,
        green: 0.0127162,
        blue: 0.6203460,
    },
    Spectrum {
        red: 0.2741907,
        green: 0.0121091,
        blue: 0.6227219,
    },
    Spectrum {
        red: 0.2806480,
        green: 0.0114876,
        blue: 0.6250385,
    },
    Spectrum {
        red: 0.2870761,
        green: 0.0108555,
        blue: 0.6272950,
    },
    Spectrum {
        red: 0.2934777,
        green: 0.0102129,
        blue: 0.6294905,
    },
    Spectrum {
        red: 0.2998551,
        green: 0.0095608,
        blue: 0.6316239,
    },
    Spectrum {
        red: 0.3062098,
        green: 0.0089019,
        blue: 0.6336941,
    },
    Spectrum {
        red: 0.3125431,
        green: 0.0082390,
        blue: 0.6356998,
    },
    Spectrum {
        red: 0.3188562,
        green: 0.0075755,
        blue: 0.6376395,
    },
    Spectrum {
        red: 0.3251500,
        green: 0.0069149,
        blue: 0.6395120,
    },
    Spectrum {
        red: 0.3314255,
        green: 0.0062611,
        blue: 0.6413156,
    },
    Spectrum {
        red: 0.3376834,
        green: 0.0056183,
        blue: 0.6430489,
    },
    Spectrum {
        red: 0.3439246,
        green: 0.0049905,
        blue: 0.6447102,
    },
    Spectrum {
        red: 0.3501497,
        green: 0.0043820,
        blue: 0.6462977,
    },
    Spectrum {
        red: 0.3563592,
        green: 0.0037978,
        blue: 0.6478098,
    },
    Spectrum {
        red: 0.3625535,
        green: 0.0032432,
        blue: 0.6492446,
    },
    Spectrum {
        red: 0.3687328,
        green: 0.0027237,
        blue: 0.6506006,
    },
    Spectrum {
        red: 0.3748973,
        green: 0.0022451,
        blue: 0.6518758,
    },
    Spectrum {
        red: 0.3810471,
        green: 0.0018136,
        blue: 0.6530685,
    },
    Spectrum {
        red: 0.3871826,
        green: 0.0014345,
        blue: 0.6541768,
    },
    Spectrum {
        red: 0.3933040,
        green: 0.0011139,
        blue: 0.6551988,
    },
    Spectrum {
        red: 0.3994108,
        green: 0.0008594,
        blue: 0.6561328,
    },
    Spectrum {
        red: 0.4055029,
        green: 0.0006781,
        blue: 0.6569773,
    },
    Spectrum {
        red: 0.4115801,
        green: 0.0005771,
        blue: 0.6577304,
    },
    Spectrum {
        red: 0.4176421,
        green: 0.0005638,
        blue: 0.6583905,
    },
    Spectrum {
        red: 0.4236885,
        green: 0.0006459,
        blue: 0.6589560,
    },
    Spectrum {
        red: 0.4297192,
        green: 0.0008310,
        blue: 0.6594254,
    },
    Spectrum {
        red: 0.4357336,
        green: 0.0011271,
        blue: 0.6597971,
    },
    Spectrum {
        red: 0.4417321,
        green: 0.0015398,
        blue: 0.6600690,
    },
    Spectrum {
        red: 0.4477136,
        green: 0.0020795,
        blue: 0.6602404,
    },
    Spectrum {
        red: 0.4536774,
        green: 0.0027547,
        blue: 0.6603100,
    },
    Spectrum {
        red: 0.4596229,
        green: 0.0035737,
        blue: 0.6602767,
    },
    Spectrum {
        red: 0.4655496,
        green: 0.0045452,
        blue: 0.6601394,
    },
    Spectrum {
        red: 0.4714568,
        green: 0.0056776,
        blue: 0.6598972,
    },
    Spectrum {
        red: 0.4773439,
        green: 0.0069796,
        blue: 0.6595493,
    },
    Spectrum {
        red: 0.4832102,
        green: 0.0084598,
        blue: 0.6590950,
    },
    Spectrum {
        red: 0.4890550,
        green: 0.0101270,
        blue: 0.6585337,
    },
    Spectrum {
        red: 0.4948775,
        green: 0.0119897,
        blue: 0.6578649,
    },
    Spectrum {
        red: 0.5006777,
        green: 0.0140551,
        blue: 0.6570876,
    },
    Spectrum {
        red: 0.5064541,
        green: 0.0163333,
        blue: 0.6562023,
    },
    Spectrum {
        red: 0.5122060,
        green: 0.0188332,
        blue: 0.6552092,
    },
    Spectrum {
        red: 0.5179326,
        green: 0.0215632,
        blue: 0.6541085,
    },
    Spectrum {
        red: 0.5236330,
        green: 0.0245316,
        blue: 0.6529006,
    },
    Spectrum {
        red: 0.5293065,
        green: 0.0277469,
        blue: 0.6515860,
    },
    Spectrum {
        red: 0.5349522,
        green: 0.0312170,
        blue: 0.6501654,
    },
    Spectrum {
        red: 0.5405695,
        green: 0.0349501,
        blue: 0.6486397,
    },
    Spectrum {
        red: 0.5461575,
        green: 0.0389540,
        blue: 0.6470099,
    },
    Spectrum {
        red: 0.5517154,
        green: 0.0431365,
        blue: 0.6452773,
    },
    Spectrum {
        red: 0.5572425,
        green: 0.0473308,
        blue: 0.6434433,
    },
    Spectrum {
        red: 0.5627381,
        green: 0.0515448,
        blue: 0.6415094,
    },
    Spectrum {
        red: 0.5682014,
        green: 0.0557777,
        blue: 0.6394774,
    },
    Spectrum {
        red: 0.5736319,
        green: 0.0600281,
        blue: 0.6373488,
    },
    Spectrum {
        red: 0.5790287,
        green: 0.0642956,
        blue: 0.6351261,
    },
    Spectrum {
        red: 0.5843911,
        green: 0.0685790,
        blue: 0.6328116,
    },
    Spectrum {
        red: 0.5897186,
        green: 0.0728776,
        blue: 0.6304077,
    },
    Spectrum {
        red: 0.5950105,
        green: 0.0771903,
        blue: 0.6279170,
    },
    Spectrum {
        red: 0.6002663,
        green: 0.0815162,
        blue: 0.6253421,
    },
    Spectrum {
        red: 0.6054854,
        green: 0.0858544,
        blue: 0.6226857,
    },
    Spectrum {
        red: 0.6106675,
        green: 0.0902039,
        blue: 0.6199508,
    },
    Spectrum {
        red: 0.6158120,
        green: 0.0945640,
        blue: 0.6171404,
    },
    Spectrum {
        red: 0.6209186,
        green: 0.0989337,
        blue: 0.6142574,
    },
    Spectrum {
        red: 0.6259869,
        green: 0.1033122,
        blue: 0.6113052,
    },
    Spectrum {
        red: 0.6310166,
        green: 0.1076986,
        blue: 0.6082868,
    },
    Spectrum {
        red: 0.6360075,
        green: 0.1120923,
        blue: 0.6052055,
    },
    Spectrum {
        red: 0.6409594,
        green: 0.1164925,
        blue: 0.6020646,
    },
    Spectrum {
        red: 0.6458722,
        green: 0.1208984,
        blue: 0.5988674,
    },
    Spectrum {
        red: 0.6507456,
        green: 0.1253094,
        blue: 0.5956173,
    },
    Spectrum {
        red: 0.6555796,
        green: 0.1297248,
        blue: 0.5923175,
    },
    Spectrum {
        red: 0.6603743,
        green: 0.1341440,
        blue: 0.5889713,
    },
    Spectrum {
        red: 0.6651295,
        green: 0.1385664,
        blue: 0.5855823,
    },
    Spectrum {
        red: 0.6698454,
        green: 0.1429915,
        blue: 0.5821536,
    },
    Spectrum {
        red: 0.6745221,
        green: 0.1474188,
        blue: 0.5786882,
    },
    Spectrum {
        red: 0.6791597,
        green: 0.1518479,
        blue: 0.5751894,
    },
    Spectrum {
        red: 0.6837584,
        green: 0.1562782,
        blue: 0.5716602,
    },
    Spectrum {
        red: 0.6883184,
        green: 0.1607094,
        blue: 0.5681034,
    },
    Spectrum {
        red: 0.6928401,
        green: 0.1651412,
        blue: 0.5645220,
    },
    Spectrum {
        red: 0.6973236,
        green: 0.1695732,
        blue: 0.5609187,
    },
    Spectrum {
        red: 0.7017693,
        green: 0.1740052,
        blue: 0.5572961,
    },
    Spectrum {
        red: 0.7061776,
        green: 0.1784370,
        blue: 0.5536570,
    },
    Spectrum {
        red: 0.7105487,
        green: 0.1828683,
        blue: 0.5500036,
    },
    Spectrum {
        red: 0.7148832,
        green: 0.1872990,
        blue: 0.5463383,
    },
    Spectrum {
        red: 0.7191813,
        green: 0.1917289,
        blue: 0.5426633,
    },
    Spectrum {
        red: 0.7234436,
        green: 0.1961580,
        blue: 0.5389808,
    },
    Spectrum {
        red: 0.7276704,
        green: 0.2005861,
        blue: 0.5352926,
    },
    Spectrum {
        red: 0.7318622,
        green: 0.2050132,
        blue: 0.5316010,
    },
    Spectrum {
        red: 0.7360194,
        green: 0.2094391,
        blue: 0.5279084,
    },
    Spectrum {
        red: 0.7401426,
        green: 0.2138640,
        blue: 0.5242155,
    },
    Spectrum {
        red: 0.7442321,
        green: 0.2182879,
        blue: 0.5205238,
    },
    Spectrum {
        red: 0.7482885,
        green: 0.2227109,
        blue: 0.5168345,
    },
    Spectrum {
        red: 0.7523123,
        green: 0.2271332,
        blue: 0.5131490,
    },
    Spectrum {
        red: 0.7563039,
        green: 0.2315547,
        blue: 0.5094683,
    },
    Spectrum {
        red: 0.7602638,
        green: 0.2359758,
        blue: 0.5057935,
    },
    Spectrum {
        red: 0.7641925,
        green: 0.2403964,
        blue: 0.5021256,
    },
    Spectrum {
        red: 0.7680904,
        green: 0.2448168,
        blue: 0.4984653,
    },
    Spectrum {
        red: 0.7719579,
        green: 0.2492372,
        blue: 0.4948133,
    },
    Spectrum {
        red: 0.7757955,
        green: 0.2536578,
        blue: 0.4911705,
    },
    Spectrum {
        red: 0.7796036,
        green: 0.2580784,
        blue: 0.4875391,
    },
    Spectrum {
        red: 0.7833826,
        green: 0.2624997,
        blue: 0.4839177,
    },
    Spectrum {
        red: 0.7871330,
        green: 0.2669219,
        blue: 0.4803067,
    },
    Spectrum {
        red: 0.7908550,
        green: 0.2713453,
        blue: 0.4767063,
    },
    Spectrum {
        red: 0.7945491,
        green: 0.2757702,
        blue: 0.4731168,
    },
    Spectrum {
        red: 0.7982156,
        green: 0.2801969,
        blue: 0.4695383,
    },
    Spectrum {
        red: 0.8018548,
        green: 0.2846258,
        blue: 0.4659709,
    },
    Spectrum {
        red: 0.8054669,
        green: 0.2890571,
        blue: 0.4624146,
    },
    Spectrum {
        red: 0.8090524,
        green: 0.2934911,
        blue: 0.4588696,
    },
    Spectrum {
        red: 0.8126115,
        green: 0.2979279,
        blue: 0.4553376,
    },
    Spectrum {
        red: 0.8161444,
        green: 0.3023681,
        blue: 0.4518164,
    },
    Spectrum {
        red: 0.8196513,
        green: 0.3068123,
        blue: 0.4483059,
    },
    Spectrum {
        red: 0.8231323,
        green: 0.3112607,
        blue: 0.4448058,
    },
    Spectrum {
        red: 0.8265877,
        green: 0.3157138,
        blue: 0.4413159,
    },
    Spectrum {
        red: 0.8300176,
        green: 0.3201719,
        blue: 0.4378359,
    },
    Spectrum {
        red: 0.8334221,
        green: 0.3246355,
        blue: 0.4343656,
    },
    Spectrum {
        red: 0.8368012,
        green: 0.3291048,
        blue: 0.4309051,
    },
    Spectrum {
        red: 0.8401553,
        green: 0.3335801,
        blue: 0.4274548,
    },
    Spectrum {
        red: 0.8434841,
        green: 0.3380621,
        blue: 0.4240131,
    },
    Spectrum {
        red: 0.8467877,
        green: 0.3425513,
        blue: 0.4205793,
    },
    Spectrum {
        red: 0.8500661,
        green: 0.3470480,
        blue: 0.4171533,
    },
    Spectrum {
        red: 0.8533193,
        green: 0.3515528,
        blue: 0.4137344,
    },
    Spectrum {
        red: 0.8565471,
        green: 0.3560661,
        blue: 0.4103225,
    },
    Spectrum {
        red: 0.8597495,
        green: 0.3605882,
        blue: 0.4069170,
    },
    Spectrum {
        red: 0.8629266,
        green: 0.3651194,
        blue: 0.4035188,
    },
    Spectrum {
        red: 0.8660779,
        green: 0.3696604,
        blue: 0.4001260,
    },
    Spectrum {
        red: 0.8692034,
        green: 0.3742118,
        blue: 0.3967382,
    },
    Spectrum {
        red: 0.8723029,
        green: 0.3787739,
        blue: 0.3933549,
    },
    Spectrum {
        red: 0.8753761,
        green: 0.3833472,
        blue: 0.3899758,
    },
    Spectrum {
        red: 0.8784229,
        green: 0.3879322,
        blue: 0.3866005,
    },
    Spectrum {
        red: 0.8814429,
        green: 0.3925293,
        blue: 0.3832286,
    },
    Spectrum {
        red: 0.8844360,
        green: 0.3971389,
        blue: 0.3798602,
    },
    Spectrum {
        red: 0.8874017,
        green: 0.4017615,
        blue: 0.3764942,
    },
    Spectrum {
        red: 0.8903397,
        green: 0.4063977,
        blue: 0.3731302,
    },
    Spectrum {
        red: 0.8932496,
        green: 0.4110479,
        blue: 0.3697679,
    },
    Spectrum {
        red: 0.8961312,
        green: 0.4157125,
        blue: 0.3664069,
    },
    Spectrum {
        red: 0.8989839,
        green: 0.4203920,
        blue: 0.3630470,
    },
    Spectrum {
        red: 0.9018075,
        green: 0.4250868,
        blue: 0.3596878,
    },
    Spectrum {
        red: 0.9046013,
        green: 0.4297974,
        blue: 0.3563288,
    },
    Spectrum {
        red: 0.9073650,
        green: 0.4345243,
        blue: 0.3529698,
    },
    Spectrum {
        red: 0.9100981,
        green: 0.4392679,
        blue: 0.3496105,
    },
    Spectrum {
        red: 0.9128001,
        green: 0.4440286,
        blue: 0.3462507,
    },
    Spectrum {
        red: 0.9154705,
        green: 0.4488067,
        blue: 0.3428901,
    },
    Spectrum {
        red: 0.9181088,
        green: 0.4536028,
        blue: 0.3395288,
    },
    Spectrum {
        red: 0.9207144,
        green: 0.4584174,
        blue: 0.3361656,
    },
    Spectrum {
        red: 0.9232867,
        green: 0.4632508,
        blue: 0.3328008,
    },
    Spectrum {
        red: 0.9258251,
        green: 0.4681034,
        blue: 0.3294345,
    },
    Spectrum {
        red: 0.9283293,
        green: 0.4729755,
        blue: 0.3260666,
    },
    Spectrum {
        red: 0.9307985,
        green: 0.4778674,
        blue: 0.3226969,
    },
    Spectrum {
        red: 0.9332321,
        green: 0.4827796,
        blue: 0.3193254,
    },
    Spectrum {
        red: 0.9356297,
        green: 0.4877124,
        blue: 0.3159522,
    },
    Spectrum {
        red: 0.9379900,
        green: 0.4926665,
        blue: 0.3125754,
    },
    Spectrum {
        red: 0.9403129,
        green: 0.4976420,
        blue: 0.3091966,
    },
    Spectrum {
        red: 0.9425978,
        green: 0.5026391,
        blue: 0.3058158,
    },
    Spectrum {
        red: 0.9448439,
        green: 0.5076582,
        blue: 0.3024331,
    },
    Spectrum {
        red: 0.9470507,
        green: 0.5126994,
        blue: 0.2990486,
    },
    Spectrum {
        red: 0.9492174,
        green: 0.5177631,
        blue: 0.2956623,
    },
    Spectrum {
        red: 0.9513435,
        green: 0.5228495,
        blue: 0.2922745,
    },
    Spectrum {
        red: 0.9534277,
        green: 0.5279596,
        blue: 0.2888834,
    },
    Spectrum {
        red: 0.9554696,
        green: 0.5330931,
        blue: 0.2854904,
    },
    Spectrum {
        red: 0.9574688,
        green: 0.5382502,
        blue: 0.2820961,
    },
    Spectrum {
        red: 0.9594244,
        green: 0.5434310,
        blue: 0.2787010,
    },
    Spectrum {
        red: 0.9613359,
        green: 0.5486359,
        blue: 0.2753052,
    },
    Spectrum {
        red: 0.9632026,
        green: 0.5538649,
        blue: 0.2719092,
    },
    Spectrum {
        red: 0.9650237,
        green: 0.5591183,
        blue: 0.2685132,
    },
    Spectrum {
        red: 0.9667985,
        green: 0.5643963,
        blue: 0.2651178,
    },
    Spectrum {
        red: 0.9685256,
        green: 0.5696996,
        blue: 0.2617215,
    },
    Spectrum {
        red: 0.9702046,
        green: 0.5750283,
        blue: 0.2583254,
    },
    Spectrum {
        red: 0.9718350,
        green: 0.5803820,
        blue: 0.2549313,
    },
    Spectrum {
        red: 0.9734161,
        green: 0.5857610,
        blue: 0.2515396,
    },
    Spectrum {
        red: 0.9749473,
        green: 0.5911654,
        blue: 0.2481512,
    },
    Spectrum {
        red: 0.9764276,
        green: 0.5965953,
        blue: 0.2447668,
    },
    Spectrum {
        red: 0.9778564,
        green: 0.6020508,
        blue: 0.2413872,
    },
    Spectrum {
        red: 0.9792329,
        green: 0.6075321,
        blue: 0.2380134,
    },
    Spectrum {
        red: 0.9805563,
        green: 0.6130392,
        blue: 0.2346463,
    },
    Spectrum {
        red: 0.9818259,
        green: 0.6185723,
        blue: 0.2312872,
    },
    Spectrum {
        red: 0.9830407,
        green: 0.6241314,
        blue: 0.2279371,
    },
    Spectrum {
        red: 0.9841989,
        green: 0.6297175,
        blue: 0.2245950,
    },
    Spectrum {
        red: 0.9853008,
        green: 0.6353299,
        blue: 0.2212649,
    },
    Spectrum {
        red: 0.9863454,
        green: 0.6409685,
        blue: 0.2179485,
    },
    Spectrum {
        red: 0.9873321,
        green: 0.6466335,
        blue: 0.2146475,
    },
    Spectrum {
        red: 0.9882598,
        green: 0.6523248,
        blue: 0.2113641,
    },
    Spectrum {
        red: 0.9891279,
        green: 0.6580426,
        blue: 0.2081004,
    },
    Spectrum {
        red: 0.9899353,
        green: 0.6637869,
        blue: 0.2048589,
    },
    Spectrum {
        red: 0.9906813,
        green: 0.6695577,
        blue: 0.2016420,
    },
    Spectrum {
        red: 0.9913648,
        green: 0.6753551,
        blue: 0.1984529,
    },
    Spectrum {
        red: 0.9919850,
        green: 0.6811790,
        blue: 0.1952946,
    },
    Spectrum {
        red: 0.9925409,
        green: 0.6870296,
        blue: 0.1921705,
    },
    Spectrum {
        red: 0.9930317,
        green: 0.6929067,
        blue: 0.1890845,
    },
    Spectrum {
        red: 0.9934563,
        green: 0.6988105,
        blue: 0.1860405,
    },
    Spectrum {
        red: 0.9938138,
        green: 0.7047409,
        blue: 0.1830432,
    },
    Spectrum {
        red: 0.9941032,
        green: 0.7106978,
        blue: 0.1800972,
    },
    Spectrum {
        red: 0.9943236,
        green: 0.7166813,
        blue: 0.1772078,
    },
    Spectrum {
        red: 0.9944739,
        green: 0.7226914,
        blue: 0.1743807,
    },
    Spectrum {
        red: 0.9945533,
        green: 0.7287279,
        blue: 0.1716217,
    },
    Spectrum {
        red: 0.9945606,
        green: 0.7347908,
        blue: 0.1689375,
    },
    Spectrum {
        red: 0.9944950,
        green: 0.7408800,
        blue: 0.1663349,
    },
    Spectrum {
        red: 0.9943554,
        green: 0.7469954,
        blue: 0.1638212,
    },
    Spectrum {
        red: 0.9941410,
        green: 0.7531370,
        blue: 0.1614042,
    },
    Spectrum {
        red: 0.9938508,
        green: 0.7593044,
        blue: 0.1590920,
    },
    Spectrum {
        red: 0.9934822,
        green: 0.7654986,
        blue: 0.1568906,
    },
    Spectrum {
        red: 0.9930333,
        green: 0.7717198,
        blue: 0.1548076,
    },
    Spectrum {
        red: 0.9925052,
        green: 0.7779668,
        blue: 0.1528549,
    },
    Spectrum {
        red: 0.9918973,
        green: 0.7842391,
        blue: 0.1510416,
    },
    Spectrum {
        red: 0.9912087,
        green: 0.7905366,
        blue: 0.1493769,
    },
    Spectrum {
        red: 0.9904388,
        green: 0.7968588,
        blue: 0.1478698,
    },
    Spectrum {
        red: 0.9895871,
        green: 0.8032053,
        blue: 0.1465291,
    },
    Spectrum {
        red: 0.9886477,
        green: 0.8095786,
        blue: 0.1453573,
    },
    Spectrum {
        red: 0.9876206,
        green: 0.8159779,
        blue: 0.1443626,
    },
    Spectrum {
        red: 0.9865094,
        green: 0.8224006,
        blue: 0.1435567,
    },
    Spectrum {
        red: 0.9853142,
        green: 0.8288460,
        blue: 0.1429451,
    },
    Spectrum {
        red: 0.9840311,
        green: 0.8353154,
        blue: 0.1425284,
    },
    Spectrum {
        red: 0.9826528,
        green: 0.8418117,
        blue: 0.1423027,
    },
    Spectrum {
        red: 0.9811904,
        green: 0.8483289,
        blue: 0.1422786,
    },
    Spectrum {
        red: 0.9796436,
        green: 0.8548665,
        blue: 0.1424534,
    },
    Spectrum {
        red: 0.9779949,
        green: 0.8614323,
        blue: 0.1428082,
    },
    Spectrum {
        red: 0.9762650,
        green: 0.8680160,
        blue: 0.1433509,
    },
    Spectrum {
        red: 0.9744430,
        green: 0.8746222,
        blue: 0.1440612,
    },
    Spectrum {
        red: 0.9725300,
        green: 0.8812501,
        blue: 0.1449229,
    },
    Spectrum {
        red: 0.9705329,
        green: 0.8878961,
        blue: 0.1459187,
    },
    Spectrum {
        red: 0.9684435,
        green: 0.8945640,
        blue: 0.1470144,
    },
    Spectrum {
        red: 0.9662712,
        green: 0.9012494,
        blue: 0.1481796,
    },
    Spectrum {
        red: 0.9640211,
        green: 0.9079504,
        blue: 0.1493704,
    },
    Spectrum {
        red: 0.9616815,
        green: 0.9146725,
        blue: 0.1505203,
    },
    Spectrum {
        red: 0.9592756,
        green: 0.9214065,
        blue: 0.1515660,
    },
    Spectrum {
        red: 0.9568081,
        green: 0.9281521,
        blue: 0.1524095,
    },
    Spectrum {
        red: 0.9542868,
        green: 0.9349077,
        blue: 0.1529212,
    },
    Spectrum {
        red: 0.9517261,
        green: 0.9416706,
        blue: 0.1529254,
    },
    Spectrum {
        red: 0.9491505,
        green: 0.9484349,
        blue: 0.1521776,
    },
    Spectrum {
        red: 0.9466023,
        green: 0.9551899,
        blue: 0.1503279,
    },
    Spectrum {
        red: 0.9441517,
        green: 0.9619165,
        blue: 0.1468608,
    },
    Spectrum {
        red: 0.9418961,
        green: 0.9685898,
        blue: 0.1409556,
    },
    Spectrum {
        red: 0.9400151,
        green: 0.9751584,
        blue: 0.1313255,
    },
];
