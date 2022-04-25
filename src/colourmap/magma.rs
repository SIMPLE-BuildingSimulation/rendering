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

pub const MAGMA_COLOURMAP : [Spectrum; 256] = [
    Spectrum{red:0.0014616, green:0.0004661, blue:0.0138655},
    Spectrum{red:0.0022576, green:0.0012950, blue:0.0183311},
    Spectrum{red:0.0032794, green:0.0023045, blue:0.0237083},
    Spectrum{red:0.0045123, green:0.0034904, blue:0.0299647},
    Spectrum{red:0.0059498, green:0.0048429, blue:0.0371297},
    Spectrum{red:0.0075880, green:0.0063561, blue:0.0449731},
    Spectrum{red:0.0094260, green:0.0080219, blue:0.0528444},
    Spectrum{red:0.0114654, green:0.0098283, blue:0.0607496},
    Spectrum{red:0.0137076, green:0.0117706, blue:0.0686666},
    Spectrum{red:0.0161558, green:0.0138405, blue:0.0766027},
    Spectrum{red:0.0188154, green:0.0160263, blue:0.0845845},
    Spectrum{red:0.0216919, green:0.0183201, blue:0.0926101},
    Spectrum{red:0.0247918, green:0.0207148, blue:0.1006756},
    Spectrum{red:0.0281228, green:0.0232009, blue:0.1087870},
    Spectrum{red:0.0316955, green:0.0257651, blue:0.1169647},
    Spectrum{red:0.0355204, green:0.0283975, blue:0.1252094},
    Spectrum{red:0.0396085, green:0.0310896, blue:0.1335151},
    Spectrum{red:0.0438295, green:0.0338300, blue:0.1418862},
    Spectrum{red:0.0480616, green:0.0366066, blue:0.1503270},
    Spectrum{red:0.0523204, green:0.0394066, blue:0.1588410},
    Spectrum{red:0.0566149, green:0.0421599, blue:0.1674456},
    Spectrum{red:0.0609494, green:0.0447945, blue:0.1761288},
    Spectrum{red:0.0653302, green:0.0473178, blue:0.1848915},
    Spectrum{red:0.0697637, green:0.0497265, blue:0.1937351},
    Spectrum{red:0.0742565, green:0.0520168, blue:0.2026604},
    Spectrum{red:0.0788150, green:0.0541845, blue:0.2116674},
    Spectrum{red:0.0834456, green:0.0562249, blue:0.2207551},
    Spectrum{red:0.0881548, green:0.0581331, blue:0.2299216},
    Spectrum{red:0.0929487, green:0.0599038, blue:0.2391637},
    Spectrum{red:0.0978335, green:0.0615314, blue:0.2484767},
    Spectrum{red:0.1028150, green:0.0630104, blue:0.2578544},
    Spectrum{red:0.1078987, green:0.0643351, blue:0.2672889},
    Spectrum{red:0.1130945, green:0.0654920, blue:0.2767840},
    Spectrum{red:0.1184050, green:0.0664792, blue:0.2863207},
    Spectrum{red:0.1238327, green:0.0672946, blue:0.2958794},
    Spectrum{red:0.1293802, green:0.0679349, blue:0.3054429},
    Spectrum{red:0.1350533, green:0.0683913, blue:0.3149999},
    Spectrum{red:0.1408580, green:0.0686541, blue:0.3245376},
    Spectrum{red:0.1467852, green:0.0687382, blue:0.3340111},
    Spectrum{red:0.1528392, green:0.0686369, blue:0.3434045},
    Spectrum{red:0.1590175, green:0.0683540, blue:0.3526880},
    Spectrum{red:0.1653081, green:0.0679109, blue:0.3618164},
    Spectrum{red:0.1717130, green:0.0673053, blue:0.3707708},
    Spectrum{red:0.1782117, green:0.0665758, blue:0.3794972},
    Spectrum{red:0.1848009, green:0.0657324, blue:0.3879725},
    Spectrum{red:0.1914597, green:0.0648183, blue:0.3961520},
    Spectrum{red:0.1981769, green:0.0638624, blue:0.4040090},
    Spectrum{red:0.2049349, green:0.0629066, blue:0.4115143},
    Spectrum{red:0.2117181, green:0.0619918, blue:0.4186467},
    Spectrum{red:0.2185116, green:0.0611585, blue:0.4253918},
    Spectrum{red:0.2253020, green:0.0604452, blue:0.4317418},
    Spectrum{red:0.2320765, green:0.0598887, blue:0.4376947},
    Spectrum{red:0.2388260, green:0.0595170, blue:0.4432560},
    Spectrum{red:0.2455432, green:0.0593524, blue:0.4484359},
    Spectrum{red:0.2522203, green:0.0594147, blue:0.4532477},
    Spectrum{red:0.2588573, green:0.0597056, blue:0.4577099},
    Spectrum{red:0.2654467, green:0.0602369, blue:0.4618403},
    Spectrum{red:0.2719941, green:0.0609936, blue:0.4656604},
    Spectrum{red:0.2784933, green:0.0619778, blue:0.4691903},
    Spectrum{red:0.2849511, green:0.0631676, blue:0.4724509},
    Spectrum{red:0.2913658, green:0.0645534, blue:0.4754622},
    Spectrum{red:0.2977404, green:0.0661170, blue:0.4782435},
    Spectrum{red:0.3040809, green:0.0678353, blue:0.4808116},
    Spectrum{red:0.3103820, green:0.0697025, blue:0.4831863},
    Spectrum{red:0.3166542, green:0.0716895, blue:0.4853804},
    Spectrum{red:0.3228991, green:0.0737820, blue:0.4874084},
    Spectrum{red:0.3291140, green:0.0759715, blue:0.4892868},
    Spectrum{red:0.3353075, green:0.0782361, blue:0.4910241},
    Spectrum{red:0.3414817, green:0.0805635, blue:0.4926313},
    Spectrum{red:0.3476357, green:0.0829464, blue:0.4941209},
    Spectrum{red:0.3537732, green:0.0853726, blue:0.4955011},
    Spectrum{red:0.3598979, green:0.0878312, blue:0.4967783},
    Spectrum{red:0.3660119, green:0.0903143, blue:0.4979600},
    Spectrum{red:0.3721162, green:0.0928160, blue:0.4990533},
    Spectrum{red:0.3782105, green:0.0953323, blue:0.5000666},
    Spectrum{red:0.3842994, green:0.0978549, blue:0.5010020},
    Spectrum{red:0.3903844, green:0.1003795, blue:0.5018642},
    Spectrum{red:0.3964667, green:0.1029022, blue:0.5026576},
    Spectrum{red:0.4025477, green:0.1054199, blue:0.5033858},
    Spectrum{red:0.4086285, green:0.1079298, blue:0.5040521},
    Spectrum{red:0.4147087, green:0.1104312, blue:0.5046618},
    Spectrum{red:0.4207912, green:0.1129202, blue:0.5052149},
    Spectrum{red:0.4268770, green:0.1153953, blue:0.5057136},
    Spectrum{red:0.4329670, green:0.1178550, blue:0.5061598},
    Spectrum{red:0.4390621, green:0.1202983, blue:0.5065550},
    Spectrum{red:0.4451631, green:0.1227244, blue:0.5069008},
    Spectrum{red:0.4512707, green:0.1251325, blue:0.5071983},
    Spectrum{red:0.4573855, green:0.1275221, blue:0.5074483},
    Spectrum{red:0.4635083, green:0.1298930, blue:0.5076518},
    Spectrum{red:0.4696395, green:0.1322448, blue:0.5078093},
    Spectrum{red:0.4757797, green:0.1345775, blue:0.5079212},
    Spectrum{red:0.4819290, green:0.1368914, blue:0.5079885},
    Spectrum{red:0.4880882, green:0.1391862, blue:0.5080107},
    Spectrum{red:0.4942577, green:0.1414621, blue:0.5079878},
    Spectrum{red:0.5004378, green:0.1437193, blue:0.5079198},
    Spectrum{red:0.5066289, green:0.1459582, blue:0.5078064},
    Spectrum{red:0.5128312, green:0.1481791, blue:0.5076476},
    Spectrum{red:0.5190448, green:0.1503826, blue:0.5074429},
    Spectrum{red:0.5252700, green:0.1525691, blue:0.5071922},
    Spectrum{red:0.5315067, green:0.1547392, blue:0.5068949},
    Spectrum{red:0.5377552, green:0.1568936, blue:0.5065505},
    Spectrum{red:0.5440154, green:0.1590329, blue:0.5061587},
    Spectrum{red:0.5502873, green:0.1611578, blue:0.5057188},
    Spectrum{red:0.5565708, green:0.1632691, blue:0.5052302},
    Spectrum{red:0.5628659, green:0.1653677, blue:0.5046924},
    Spectrum{red:0.5691724, green:0.1674544, blue:0.5041046},
    Spectrum{red:0.5754901, green:0.1695301, blue:0.5034663},
    Spectrum{red:0.5818189, green:0.1715957, blue:0.5027767},
    Spectrum{red:0.5881584, green:0.1736524, blue:0.5020352},
    Spectrum{red:0.5945083, green:0.1757011, blue:0.5012410},
    Spectrum{red:0.6008684, green:0.1777430, blue:0.5003935},
    Spectrum{red:0.6072382, green:0.1797793, blue:0.4994920},
    Spectrum{red:0.6136172, green:0.1818112, blue:0.4985357},
    Spectrum{red:0.6200050, green:0.1838399, blue:0.4975241},
    Spectrum{red:0.6264011, green:0.1858669, blue:0.4964563},
    Spectrum{red:0.6328049, green:0.1878935, blue:0.4953318},
    Spectrum{red:0.6392156, green:0.1899212, blue:0.4941498},
    Spectrum{red:0.6456328, green:0.1919516, blue:0.4929098},
    Spectrum{red:0.6520555, green:0.1939862, blue:0.4916112},
    Spectrum{red:0.6584831, green:0.1960268, blue:0.4902533},
    Spectrum{red:0.6649147, green:0.1980752, blue:0.4888357},
    Spectrum{red:0.6713493, green:0.2001332, blue:0.4873578},
    Spectrum{red:0.6777860, green:0.2022027, blue:0.4858192},
    Spectrum{red:0.6842237, green:0.2042857, blue:0.4842193},
    Spectrum{red:0.6906614, green:0.2063845, blue:0.4825579},
    Spectrum{red:0.6970978, green:0.2085011, blue:0.4808347},
    Spectrum{red:0.7035317, green:0.2106380, blue:0.4790493},
    Spectrum{red:0.7099619, green:0.2127973, blue:0.4772011},
    Spectrum{red:0.7163870, green:0.2149817, blue:0.4752898},
    Spectrum{red:0.7228055, green:0.2171938, blue:0.4733157},
    Spectrum{red:0.7292155, green:0.2194365, blue:0.4712789},
    Spectrum{red:0.7356155, green:0.2217126, blue:0.4691795},
    Spectrum{red:0.7420037, green:0.2240252, blue:0.4670178},
    Spectrum{red:0.7483781, green:0.2263773, blue:0.4647940},
    Spectrum{red:0.7547367, green:0.2287724, blue:0.4625085},
    Spectrum{red:0.7610773, green:0.2312136, blue:0.4601621},
    Spectrum{red:0.7673977, green:0.2337047, blue:0.4577554},
    Spectrum{red:0.7736954, green:0.2362493, blue:0.4552894},
    Spectrum{red:0.7799678, green:0.2388512, blue:0.4527650},
    Spectrum{red:0.7862124, green:0.2415143, blue:0.4501837},
    Spectrum{red:0.7924270, green:0.2442423, blue:0.4475432},
    Spectrum{red:0.7986078, green:0.2470398, blue:0.4448484},
    Spectrum{red:0.8047515, green:0.2499114, blue:0.4421016},
    Spectrum{red:0.8108548, green:0.2528614, blue:0.4393050},
    Spectrum{red:0.8169142, green:0.2558946, blue:0.4364611},
    Spectrum{red:0.8229258, green:0.2590155, blue:0.4335729},
    Spectrum{red:0.8288857, green:0.2622290, blue:0.4306436},
    Spectrum{red:0.8347908, green:0.2655397, blue:0.4276714},
    Spectrum{red:0.8406357, green:0.2689529, blue:0.4246656},
    Spectrum{red:0.8464158, green:0.2724735, blue:0.4216311},
    Spectrum{red:0.8521265, green:0.2761065, blue:0.4185728},
    Spectrum{red:0.8577629, green:0.2798567, blue:0.4154963},
    Spectrum{red:0.8633204, green:0.2837290, blue:0.4124029},
    Spectrum{red:0.8687934, green:0.2877282, blue:0.4093030},
    Spectrum{red:0.8741763, green:0.2918587, blue:0.4062054},
    Spectrum{red:0.8794639, green:0.2961246, blue:0.4031180},
    Spectrum{red:0.8846508, green:0.3005301, blue:0.4000471},
    Spectrum{red:0.8897314, green:0.3050788, blue:0.3970016},
    Spectrum{red:0.8947002, green:0.3097734, blue:0.3939946},
    Spectrum{red:0.8995519, green:0.3146164, blue:0.3910367},
    Spectrum{red:0.9042813, green:0.3196100, blue:0.3881369},
    Spectrum{red:0.9088835, green:0.3247551, blue:0.3853080},
    Spectrum{red:0.9133541, green:0.3300519, blue:0.3825634},
    Spectrum{red:0.9176889, green:0.3355001, blue:0.3799151},
    Spectrum{red:0.9218842, green:0.3410981, blue:0.3773760},
    Spectrum{red:0.9259371, green:0.3468437, blue:0.3749591},
    Spectrum{red:0.9298451, green:0.3527338, blue:0.3726765},
    Spectrum{red:0.9336065, green:0.3587644, blue:0.3705409},
    Spectrum{red:0.9372209, green:0.3649293, blue:0.3685665},
    Spectrum{red:0.9406874, green:0.3712242, blue:0.3667617},
    Spectrum{red:0.9440064, green:0.3776429, blue:0.3651363},
    Spectrum{red:0.9471795, green:0.3841779, blue:0.3637011},
    Spectrum{red:0.9502102, green:0.3908195, blue:0.3624677},
    Spectrum{red:0.9530991, green:0.3975629, blue:0.3614384},
    Spectrum{red:0.9558492, green:0.4044002, blue:0.3606191},
    Spectrum{red:0.9584641, green:0.4113237, blue:0.3600142},
    Spectrum{red:0.9609492, green:0.4183232, blue:0.3596298},
    Spectrum{red:0.9633103, green:0.4253897, blue:0.3594690},
    Spectrum{red:0.9655494, green:0.4325187, blue:0.3595292},
    Spectrum{red:0.9676711, green:0.4397030, blue:0.3598102},
    Spectrum{red:0.9696804, green:0.4469356, blue:0.3603111},
    Spectrum{red:0.9715822, green:0.4542102, blue:0.3610302},
    Spectrum{red:0.9733812, green:0.4615205, blue:0.3619647},
    Spectrum{red:0.9750824, green:0.4688609, blue:0.3631113},
    Spectrum{red:0.9766905, green:0.4762264, blue:0.3644662},
    Spectrum{red:0.9782100, green:0.4836120, blue:0.3660249},
    Spectrum{red:0.9796452, green:0.4910138, blue:0.3677826},
    Spectrum{red:0.9810003, green:0.4984278, blue:0.3697342},
    Spectrum{red:0.9822792, green:0.5058508, blue:0.3718743},
    Spectrum{red:0.9834854, green:0.5132801, blue:0.3741975},
    Spectrum{red:0.9846223, green:0.5207130, blue:0.3766982},
    Spectrum{red:0.9856929, green:0.5281475, blue:0.3793708},
    Spectrum{red:0.9867000, green:0.5355821, blue:0.3822097},
    Spectrum{red:0.9876460, green:0.5430152, blue:0.3852096},
    Spectrum{red:0.9885332, green:0.5504458, blue:0.3883650},
    Spectrum{red:0.9893633, green:0.5578731, blue:0.3916708},
    Spectrum{red:0.9901382, green:0.5652965, blue:0.3951221},
    Spectrum{red:0.9908712, green:0.5727063, blue:0.3987140},
    Spectrum{red:0.9915582, green:0.5801068, blue:0.4024411},
    Spectrum{red:0.9921957, green:0.5875017, blue:0.4062988},
    Spectrum{red:0.9927847, green:0.5948911, blue:0.4102830},
    Spectrum{red:0.9933256, green:0.6022753, blue:0.4143897},
    Spectrum{red:0.9938344, green:0.6096435, blue:0.4186132},
    Spectrum{red:0.9943085, green:0.6169990, blue:0.4229497},
    Spectrum{red:0.9947377, green:0.6243497, blue:0.4273968},
    Spectrum{red:0.9951219, green:0.6316964, blue:0.4319515},
    Spectrum{red:0.9954805, green:0.6390266, blue:0.4366072},
    Spectrum{red:0.9958099, green:0.6463439, blue:0.4413610},
    Spectrum{red:0.9960957, green:0.6536588, blue:0.4462130},
    Spectrum{red:0.9963414, green:0.6609694, blue:0.4511602},
    Spectrum{red:0.9965798, green:0.6682556, blue:0.4561918},
    Spectrum{red:0.9967748, green:0.6755415, blue:0.4613142},
    Spectrum{red:0.9969254, green:0.6828280, blue:0.4665257},
    Spectrum{red:0.9970772, green:0.6900879, blue:0.4718115},
    Spectrum{red:0.9971863, green:0.6973490, blue:0.4771817},
    Spectrum{red:0.9972540, green:0.7046108, blue:0.4826347},
    Spectrum{red:0.9973252, green:0.7118477, blue:0.4881544},
    Spectrum{red:0.9973510, green:0.7190891, blue:0.4937547},
    Spectrum{red:0.9973506, green:0.7263244, blue:0.4994280},
    Spectrum{red:0.9973413, green:0.7335447, blue:0.5051668},
    Spectrum{red:0.9972847, green:0.7407719, blue:0.5109833},
    Spectrum{red:0.9972284, green:0.7479806, blue:0.5168594},
    Spectrum{red:0.9971385, green:0.7551899, blue:0.5228060},
    Spectrum{red:0.9970193, green:0.7623979, blue:0.5288208},
    Spectrum{red:0.9968983, green:0.7695910, blue:0.5348923},
    Spectrum{red:0.9967269, green:0.7767949, blue:0.5410386},
    Spectrum{red:0.9965706, green:0.7839765, blue:0.5472330},
    Spectrum{red:0.9963691, green:0.7911673, blue:0.5534989},
    Spectrum{red:0.9961623, green:0.7983477, blue:0.5598196},
    Spectrum{red:0.9959324, green:0.8055271, blue:0.5662018},
    Spectrum{red:0.9956801, green:0.8127058, blue:0.5726448},
    Spectrum{red:0.9954240, green:0.8198753, blue:0.5791401},
    Spectrum{red:0.9951313, green:0.8270518, blue:0.5857015},
    Spectrum{red:0.9948511, green:0.8342128, blue:0.5923071},
    Spectrum{red:0.9945237, green:0.8413866, blue:0.5989828},
    Spectrum{red:0.9942219, green:0.8485405, blue:0.6056959},
    Spectrum{red:0.9938658, green:0.8557110, blue:0.6124818},
    Spectrum{red:0.9935453, green:0.8628588, blue:0.6192993},
    Spectrum{red:0.9931696, green:0.8700245, blue:0.6261895},
    Spectrum{red:0.9928310, green:0.8771684, blue:0.6331091},
    Spectrum{red:0.9924399, green:0.8843297, blue:0.6400995},
    Spectrum{red:0.9920895, green:0.8914695, blue:0.6471160},
    Spectrum{red:0.9916877, green:0.8986271, blue:0.6542015},
    Spectrum{red:0.9913319, green:0.9057627, blue:0.6613088},
    Spectrum{red:0.9909297, green:0.9129150, blue:0.6684812},
    Spectrum{red:0.9905699, green:0.9200487, blue:0.6756746},
    Spectrum{red:0.9901746, green:0.9271956, blue:0.6829256},
    Spectrum{red:0.9898148, green:0.9343285, blue:0.6901982},
    Spectrum{red:0.9894337, green:0.9414704, blue:0.6975186},
    Spectrum{red:0.9890774, green:0.9486041, blue:0.7048625},
    Spectrum{red:0.9887171, green:0.9557415, blue:0.7122422},
    Spectrum{red:0.9883670, green:0.9628780, blue:0.7196486},
    Spectrum{red:0.9880329, green:0.9700124, blue:0.7270768},
    Spectrum{red:0.9876907, green:0.9771542, blue:0.7345362},
    Spectrum{red:0.9873868, green:0.9842876, blue:0.7420015},
    Spectrum{red:0.9870525, green:0.9914379, blue:0.7495042},
];