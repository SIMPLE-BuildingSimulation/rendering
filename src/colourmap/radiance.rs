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

// From Radiance's source code src/common/falsecolor.c

use crate::colour::Spectrum;

/// Radiance's standard falsecolour colourmap
pub const RADIANCE_COLOURMAP: [Spectrum; 256] = [
    Spectrum {
        red: 0.4335938,
        green: 0.0312500,
        blue: 0.5156250,
    },
    Spectrum {
        red: 0.4218750,
        green: 0.0273438,
        blue: 0.5195313,
    },
    Spectrum {
        red: 0.4101563,
        green: 0.0273438,
        blue: 0.5234375,
    },
    Spectrum {
        red: 0.3984375,
        green: 0.0234375,
        blue: 0.5312500,
    },
    Spectrum {
        red: 0.3828125,
        green: 0.0234375,
        blue: 0.5351563,
    },
    Spectrum {
        red: 0.3632813,
        green: 0.0195313,
        blue: 0.5429688,
    },
    Spectrum {
        red: 0.3476563,
        green: 0.0156250,
        blue: 0.5507813,
    },
    Spectrum {
        red: 0.3281250,
        green: 0.0117188,
        blue: 0.5585938,
    },
    Spectrum {
        red: 0.3085938,
        green: 0.0078125,
        blue: 0.5664063,
    },
    Spectrum {
        red: 0.2890625,
        green: 0.0039063,
        blue: 0.5781250,
    },
    Spectrum {
        red: 0.2656250,
        green: 0.0000000,
        blue: 0.5859375,
    },
    Spectrum {
        red: 0.2460938,
        green: 0.0000000,
        blue: 0.5976563,
    },
    Spectrum {
        red: 0.2226563,
        green: 0.0000000,
        blue: 0.6054688,
    },
    Spectrum {
        red: 0.2031250,
        green: 0.0000000,
        blue: 0.6132813,
    },
    Spectrum {
        red: 0.1796875,
        green: 0.0000000,
        blue: 0.6250000,
    },
    Spectrum {
        red: 0.1601563,
        green: 0.0000000,
        blue: 0.6328125,
    },
    Spectrum {
        red: 0.1406250,
        green: 0.0000000,
        blue: 0.6406250,
    },
    Spectrum {
        red: 0.1210938,
        green: 0.0000000,
        blue: 0.6484375,
    },
    Spectrum {
        red: 0.1015625,
        green: 0.0000000,
        blue: 0.6562500,
    },
    Spectrum {
        red: 0.0859375,
        green: 0.0000000,
        blue: 0.6640625,
    },
    Spectrum {
        red: 0.0703125,
        green: 0.0000000,
        blue: 0.6718750,
    },
    Spectrum {
        red: 0.0546875,
        green: 0.0078125,
        blue: 0.6796875,
    },
    Spectrum {
        red: 0.0429688,
        green: 0.0156250,
        blue: 0.6835938,
    },
    Spectrum {
        red: 0.0312500,
        green: 0.0273438,
        blue: 0.6875000,
    },
    Spectrum {
        red: 0.0273438,
        green: 0.0351563,
        blue: 0.6914063,
    },
    Spectrum {
        red: 0.0234375,
        green: 0.0429688,
        blue: 0.6914063,
    },
    Spectrum {
        red: 0.0195313,
        green: 0.0507813,
        blue: 0.6953125,
    },
    Spectrum {
        red: 0.0156250,
        green: 0.0625000,
        blue: 0.6953125,
    },
    Spectrum {
        red: 0.0117188,
        green: 0.0703125,
        blue: 0.6992188,
    },
    Spectrum {
        red: 0.0078125,
        green: 0.0820313,
        blue: 0.7031250,
    },
    Spectrum {
        red: 0.0039063,
        green: 0.0937500,
        blue: 0.7031250,
    },
    Spectrum {
        red: 0.0039063,
        green: 0.1093750,
        blue: 0.7070313,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1210938,
        blue: 0.7070313,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1367188,
        blue: 0.7109375,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1484375,
        blue: 0.7109375,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1640625,
        blue: 0.7148438,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1796875,
        blue: 0.7187500,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.1953125,
        blue: 0.7187500,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2109375,
        blue: 0.7187500,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2265625,
        blue: 0.7226563,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2460938,
        blue: 0.7226563,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2617188,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2773438,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.2968750,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.3125000,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.3281250,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.3476563,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0000000,
        green: 0.3632813,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0039063,
        green: 0.3789063,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0039063,
        green: 0.3984375,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0039063,
        green: 0.4140625,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0078125,
        green: 0.4296875,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0078125,
        green: 0.4453125,
        blue: 0.7304688,
    },
    Spectrum {
        red: 0.0117188,
        green: 0.4609375,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0117188,
        green: 0.4765625,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0156250,
        green: 0.4921875,
        blue: 0.7265625,
    },
    Spectrum {
        red: 0.0156250,
        green: 0.5078125,
        blue: 0.7226563,
    },
    Spectrum {
        red: 0.0156250,
        green: 0.5195313,
        blue: 0.7226563,
    },
    Spectrum {
        red: 0.0195313,
        green: 0.5351563,
        blue: 0.7187500,
    },
    Spectrum {
        red: 0.0195313,
        green: 0.5468750,
        blue: 0.7148438,
    },
    Spectrum {
        red: 0.0234375,
        green: 0.5585938,
        blue: 0.7109375,
    },
    Spectrum {
        red: 0.0234375,
        green: 0.5703125,
        blue: 0.7070313,
    },
    Spectrum {
        red: 0.0234375,
        green: 0.5820313,
        blue: 0.7031250,
    },
    Spectrum {
        red: 0.0273438,
        green: 0.5898438,
        blue: 0.6992188,
    },
    Spectrum {
        red: 0.0273438,
        green: 0.6015625,
        blue: 0.6953125,
    },
    Spectrum {
        red: 0.0273438,
        green: 0.6093750,
        blue: 0.6914063,
    },
    Spectrum {
        red: 0.0312500,
        green: 0.6171875,
        blue: 0.6835938,
    },
    Spectrum {
        red: 0.0312500,
        green: 0.6289063,
        blue: 0.6718750,
    },
    Spectrum {
        red: 0.0351563,
        green: 0.6367188,
        blue: 0.6601563,
    },
    Spectrum {
        red: 0.0351563,
        green: 0.6445313,
        blue: 0.6445313,
    },
    Spectrum {
        red: 0.0351563,
        green: 0.6523438,
        blue: 0.6289063,
    },
    Spectrum {
        red: 0.0351563,
        green: 0.6601563,
        blue: 0.6132813,
    },
    Spectrum {
        red: 0.0390625,
        green: 0.6640625,
        blue: 0.5976563,
    },
    Spectrum {
        red: 0.0390625,
        green: 0.6718750,
        blue: 0.5781250,
    },
    Spectrum {
        red: 0.0390625,
        green: 0.6757813,
        blue: 0.5585938,
    },
    Spectrum {
        red: 0.0429688,
        green: 0.6796875,
        blue: 0.5390625,
    },
    Spectrum {
        red: 0.0429688,
        green: 0.6796875,
        blue: 0.5195313,
    },
    Spectrum {
        red: 0.0429688,
        green: 0.6835938,
        blue: 0.4960938,
    },
    Spectrum {
        red: 0.0468750,
        green: 0.6835938,
        blue: 0.4765625,
    },
    Spectrum {
        red: 0.0468750,
        green: 0.6875000,
        blue: 0.4570313,
    },
    Spectrum {
        red: 0.0507813,
        green: 0.6875000,
        blue: 0.4335938,
    },
    Spectrum {
        red: 0.0546875,
        green: 0.6875000,
        blue: 0.4140625,
    },
    Spectrum {
        red: 0.0546875,
        green: 0.6875000,
        blue: 0.3945313,
    },
    Spectrum {
        red: 0.0585938,
        green: 0.6835938,
        blue: 0.3710938,
    },
    Spectrum {
        red: 0.0625000,
        green: 0.6835938,
        blue: 0.3515625,
    },
    Spectrum {
        red: 0.0664063,
        green: 0.6835938,
        blue: 0.3359375,
    },
    Spectrum {
        red: 0.0703125,
        green: 0.6796875,
        blue: 0.3164063,
    },
    Spectrum {
        red: 0.0781250,
        green: 0.6796875,
        blue: 0.3007813,
    },
    Spectrum {
        red: 0.0820313,
        green: 0.6757813,
        blue: 0.2851563,
    },
    Spectrum {
        red: 0.0859375,
        green: 0.6718750,
        blue: 0.2695313,
    },
    Spectrum {
        red: 0.0937500,
        green: 0.6718750,
        blue: 0.2578125,
    },
    Spectrum {
        red: 0.1015625,
        green: 0.6679688,
        blue: 0.2460938,
    },
    Spectrum {
        red: 0.1093750,
        green: 0.6640625,
        blue: 0.2343750,
    },
    Spectrum {
        red: 0.1171875,
        green: 0.6601563,
        blue: 0.2265625,
    },
    Spectrum {
        red: 0.1250000,
        green: 0.6562500,
        blue: 0.2226563,
    },
    Spectrum {
        red: 0.1328125,
        green: 0.6523438,
        blue: 0.2187500,
    },
    Spectrum {
        red: 0.1445313,
        green: 0.6484375,
        blue: 0.2148438,
    },
    Spectrum {
        red: 0.1562500,
        green: 0.6445313,
        blue: 0.2109375,
    },
    Spectrum {
        red: 0.1640625,
        green: 0.6406250,
        blue: 0.2109375,
    },
    Spectrum {
        red: 0.1757813,
        green: 0.6367188,
        blue: 0.2109375,
    },
    Spectrum {
        red: 0.1875000,
        green: 0.6328125,
        blue: 0.2148438,
    },
    Spectrum {
        red: 0.2031250,
        green: 0.6250000,
        blue: 0.2148438,
    },
    Spectrum {
        red: 0.2148438,
        green: 0.6171875,
        blue: 0.2187500,
    },
    Spectrum {
        red: 0.2265625,
        green: 0.6132813,
        blue: 0.2226563,
    },
    Spectrum {
        red: 0.2421875,
        green: 0.6054688,
        blue: 0.2226563,
    },
    Spectrum {
        red: 0.2578125,
        green: 0.5976563,
        blue: 0.2304688,
    },
    Spectrum {
        red: 0.2695313,
        green: 0.5937500,
        blue: 0.2343750,
    },
    Spectrum {
        red: 0.2851563,
        green: 0.5859375,
        blue: 0.2382813,
    },
    Spectrum {
        red: 0.3007813,
        green: 0.5781250,
        blue: 0.2460938,
    },
    Spectrum {
        red: 0.3164063,
        green: 0.5703125,
        blue: 0.2500000,
    },
    Spectrum {
        red: 0.3281250,
        green: 0.5625000,
        blue: 0.2578125,
    },
    Spectrum {
        red: 0.3437500,
        green: 0.5546875,
        blue: 0.2617188,
    },
    Spectrum {
        red: 0.3593750,
        green: 0.5429688,
        blue: 0.2695313,
    },
    Spectrum {
        red: 0.3750000,
        green: 0.5351563,
        blue: 0.2734375,
    },
    Spectrum {
        red: 0.3867188,
        green: 0.5273438,
        blue: 0.2812500,
    },
    Spectrum {
        red: 0.4023438,
        green: 0.5195313,
        blue: 0.2851563,
    },
    Spectrum {
        red: 0.4179688,
        green: 0.5117188,
        blue: 0.2929688,
    },
    Spectrum {
        red: 0.4296875,
        green: 0.5000000,
        blue: 0.2968750,
    },
    Spectrum {
        red: 0.4414063,
        green: 0.4921875,
        blue: 0.3007813,
    },
    Spectrum {
        red: 0.4570313,
        green: 0.4843750,
        blue: 0.3046875,
    },
    Spectrum {
        red: 0.4687500,
        green: 0.4726563,
        blue: 0.3085938,
    },
    Spectrum {
        red: 0.4804688,
        green: 0.4648438,
        blue: 0.3125000,
    },
    Spectrum {
        red: 0.4921875,
        green: 0.4570313,
        blue: 0.3125000,
    },
    Spectrum {
        red: 0.5000000,
        green: 0.4453125,
        blue: 0.3164063,
    },
    Spectrum {
        red: 0.5117188,
        green: 0.4375000,
        blue: 0.3164063,
    },
    Spectrum {
        red: 0.5195313,
        green: 0.4296875,
        blue: 0.3164063,
    },
    Spectrum {
        red: 0.5273438,
        green: 0.4218750,
        blue: 0.3125000,
    },
    Spectrum {
        red: 0.5312500,
        green: 0.4140625,
        blue: 0.3125000,
    },
    Spectrum {
        red: 0.5351563,
        green: 0.4101563,
        blue: 0.3125000,
    },
    Spectrum {
        red: 0.5390625,
        green: 0.4062500,
        blue: 0.3085938,
    },
    Spectrum {
        red: 0.5429688,
        green: 0.3984375,
        blue: 0.3085938,
    },
    Spectrum {
        red: 0.5468750,
        green: 0.3945313,
        blue: 0.3085938,
    },
    Spectrum {
        red: 0.5507813,
        green: 0.3906250,
        blue: 0.3046875,
    },
    Spectrum {
        red: 0.5546875,
        green: 0.3828125,
        blue: 0.3046875,
    },
    Spectrum {
        red: 0.5585938,
        green: 0.3750000,
        blue: 0.3007813,
    },
    Spectrum {
        red: 0.5625000,
        green: 0.3710938,
        blue: 0.2968750,
    },
    Spectrum {
        red: 0.5625000,
        green: 0.3632813,
        blue: 0.2968750,
    },
    Spectrum {
        red: 0.5664063,
        green: 0.3593750,
        blue: 0.2929688,
    },
    Spectrum {
        red: 0.5703125,
        green: 0.3515625,
        blue: 0.2890625,
    },
    Spectrum {
        red: 0.5703125,
        green: 0.3476563,
        blue: 0.2851563,
    },
    Spectrum {
        red: 0.5742188,
        green: 0.3398438,
        blue: 0.2851563,
    },
    Spectrum {
        red: 0.5781250,
        green: 0.3320313,
        blue: 0.2812500,
    },
    Spectrum {
        red: 0.5781250,
        green: 0.3281250,
        blue: 0.2773438,
    },
    Spectrum {
        red: 0.5820313,
        green: 0.3203125,
        blue: 0.2734375,
    },
    Spectrum {
        red: 0.5820313,
        green: 0.3125000,
        blue: 0.2695313,
    },
    Spectrum {
        red: 0.5859375,
        green: 0.3085938,
        blue: 0.2656250,
    },
    Spectrum {
        red: 0.5859375,
        green: 0.3007813,
        blue: 0.2617188,
    },
    Spectrum {
        red: 0.5898438,
        green: 0.2929688,
        blue: 0.2578125,
    },
    Spectrum {
        red: 0.5898438,
        green: 0.2851563,
        blue: 0.2539063,
    },
    Spectrum {
        red: 0.5898438,
        green: 0.2812500,
        blue: 0.2500000,
    },
    Spectrum {
        red: 0.5937500,
        green: 0.2734375,
        blue: 0.2460938,
    },
    Spectrum {
        red: 0.5937500,
        green: 0.2656250,
        blue: 0.2421875,
    },
    Spectrum {
        red: 0.5976563,
        green: 0.2578125,
        blue: 0.2382813,
    },
    Spectrum {
        red: 0.5976563,
        green: 0.2539063,
        blue: 0.2343750,
    },
    Spectrum {
        red: 0.5976563,
        green: 0.2460938,
        blue: 0.2304688,
    },
    Spectrum {
        red: 0.6015625,
        green: 0.2382813,
        blue: 0.2265625,
    },
    Spectrum {
        red: 0.6015625,
        green: 0.2343750,
        blue: 0.2226563,
    },
    Spectrum {
        red: 0.6015625,
        green: 0.2265625,
        blue: 0.2187500,
    },
    Spectrum {
        red: 0.6015625,
        green: 0.2187500,
        blue: 0.2148438,
    },
    Spectrum {
        red: 0.6054688,
        green: 0.2148438,
        blue: 0.2109375,
    },
    Spectrum {
        red: 0.6054688,
        green: 0.2070313,
        blue: 0.2070313,
    },
    Spectrum {
        red: 0.6054688,
        green: 0.1992188,
        blue: 0.1992188,
    },
    Spectrum {
        red: 0.6093750,
        green: 0.1953125,
        blue: 0.1953125,
    },
    Spectrum {
        red: 0.6093750,
        green: 0.1875000,
        blue: 0.1914063,
    },
    Spectrum {
        red: 0.6093750,
        green: 0.1796875,
        blue: 0.1875000,
    },
    Spectrum {
        red: 0.6132813,
        green: 0.1757813,
        blue: 0.1835938,
    },
    Spectrum {
        red: 0.6132813,
        green: 0.1679688,
        blue: 0.1796875,
    },
    Spectrum {
        red: 0.6132813,
        green: 0.1640625,
        blue: 0.1757813,
    },
    Spectrum {
        red: 0.6171875,
        green: 0.1562500,
        blue: 0.1718750,
    },
    Spectrum {
        red: 0.6171875,
        green: 0.1523438,
        blue: 0.1679688,
    },
    Spectrum {
        red: 0.6171875,
        green: 0.1445313,
        blue: 0.1640625,
    },
    Spectrum {
        red: 0.6210938,
        green: 0.1406250,
        blue: 0.1601563,
    },
    Spectrum {
        red: 0.6210938,
        green: 0.1328125,
        blue: 0.1562500,
    },
    Spectrum {
        red: 0.6210938,
        green: 0.1289063,
        blue: 0.1523438,
    },
    Spectrum {
        red: 0.6250000,
        green: 0.1250000,
        blue: 0.1484375,
    },
    Spectrum {
        red: 0.6250000,
        green: 0.1210938,
        blue: 0.1445313,
    },
    Spectrum {
        red: 0.6289063,
        green: 0.1132813,
        blue: 0.1445313,
    },
    Spectrum {
        red: 0.6289063,
        green: 0.1093750,
        blue: 0.1406250,
    },
    Spectrum {
        red: 0.6328125,
        green: 0.1054688,
        blue: 0.1367188,
    },
    Spectrum {
        red: 0.6328125,
        green: 0.1015625,
        blue: 0.1328125,
    },
    Spectrum {
        red: 0.6367188,
        green: 0.0976563,
        blue: 0.1289063,
    },
    Spectrum {
        red: 0.6367188,
        green: 0.0937500,
        blue: 0.1289063,
    },
    Spectrum {
        red: 0.6406250,
        green: 0.0898438,
        blue: 0.1250000,
    },
    Spectrum {
        red: 0.6445313,
        green: 0.0859375,
        blue: 0.1210938,
    },
    Spectrum {
        red: 0.6445313,
        green: 0.0820313,
        blue: 0.1210938,
    },
    Spectrum {
        red: 0.6562500,
        green: 0.0703125,
        blue: 0.1132813,
    },
    Spectrum {
        red: 0.6640625,
        green: 0.0625000,
        blue: 0.1093750,
    },
    Spectrum {
        red: 0.6718750,
        green: 0.0507813,
        blue: 0.1015625,
    },
    Spectrum {
        red: 0.6835938,
        green: 0.0429688,
        blue: 0.0976563,
    },
    Spectrum {
        red: 0.6914063,
        green: 0.0351563,
        blue: 0.0937500,
    },
    Spectrum {
        red: 0.7031250,
        green: 0.0273438,
        blue: 0.0898438,
    },
    Spectrum {
        red: 0.7148438,
        green: 0.0195313,
        blue: 0.0859375,
    },
    Spectrum {
        red: 0.7226563,
        green: 0.0117188,
        blue: 0.0820313,
    },
    Spectrum {
        red: 0.7343750,
        green: 0.0078125,
        blue: 0.0820313,
    },
    Spectrum {
        red: 0.7460938,
        green: 0.0039063,
        blue: 0.0781250,
    },
    Spectrum {
        red: 0.7578125,
        green: 0.0000000,
        blue: 0.0742188,
    },
    Spectrum {
        red: 0.7695313,
        green: 0.0000000,
        blue: 0.0742188,
    },
    Spectrum {
        red: 0.7773438,
        green: 0.0000000,
        blue: 0.0703125,
    },
    Spectrum {
        red: 0.7890625,
        green: 0.0000000,
        blue: 0.0664063,
    },
    Spectrum {
        red: 0.8007813,
        green: 0.0000000,
        blue: 0.0664063,
    },
    Spectrum {
        red: 0.8085938,
        green: 0.0000000,
        blue: 0.0625000,
    },
    Spectrum {
        red: 0.8203125,
        green: 0.0078125,
        blue: 0.0625000,
    },
    Spectrum {
        red: 0.8320313,
        green: 0.0117188,
        blue: 0.0585938,
    },
    Spectrum {
        red: 0.8398438,
        green: 0.0234375,
        blue: 0.0546875,
    },
    Spectrum {
        red: 0.8476563,
        green: 0.0312500,
        blue: 0.0507813,
    },
    Spectrum {
        red: 0.8554688,
        green: 0.0429688,
        blue: 0.0507813,
    },
    Spectrum {
        red: 0.8593750,
        green: 0.0507813,
        blue: 0.0468750,
    },
    Spectrum {
        red: 0.8671875,
        green: 0.0664063,
        blue: 0.0429688,
    },
    Spectrum {
        red: 0.8750000,
        green: 0.0781250,
        blue: 0.0429688,
    },
    Spectrum {
        red: 0.8828125,
        green: 0.0937500,
        blue: 0.0390625,
    },
    Spectrum {
        red: 0.8867188,
        green: 0.1093750,
        blue: 0.0351563,
    },
    Spectrum {
        red: 0.8945313,
        green: 0.1250000,
        blue: 0.0312500,
    },
    Spectrum {
        red: 0.9023438,
        green: 0.1445313,
        blue: 0.0273438,
    },
    Spectrum {
        red: 0.9062500,
        green: 0.1640625,
        blue: 0.0234375,
    },
    Spectrum {
        red: 0.9140625,
        green: 0.1835938,
        blue: 0.0195313,
    },
    Spectrum {
        red: 0.9218750,
        green: 0.2031250,
        blue: 0.0195313,
    },
    Spectrum {
        red: 0.9257813,
        green: 0.2226563,
        blue: 0.0156250,
    },
    Spectrum {
        red: 0.9335938,
        green: 0.2460938,
        blue: 0.0117188,
    },
    Spectrum {
        red: 0.9375000,
        green: 0.2656250,
        blue: 0.0078125,
    },
    Spectrum {
        red: 0.9453125,
        green: 0.2890625,
        blue: 0.0078125,
    },
    Spectrum {
        red: 0.9492188,
        green: 0.3085938,
        blue: 0.0039063,
    },
    Spectrum {
        red: 0.9570313,
        green: 0.3320313,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9609375,
        green: 0.3554688,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9648438,
        green: 0.3750000,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9687500,
        green: 0.3984375,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9765625,
        green: 0.4218750,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9804688,
        green: 0.4414063,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9843750,
        green: 0.4609375,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9882813,
        green: 0.4804688,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.5000000,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.5195313,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.5390625,
        blue: 0.0000000,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.5585938,
        blue: 0.0039063,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.5781250,
        blue: 0.0078125,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.6015625,
        blue: 0.0117188,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.6210938,
        blue: 0.0156250,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.6445313,
        blue: 0.0234375,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.6640625,
        blue: 0.0273438,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.6875000,
        blue: 0.0351563,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.7070313,
        blue: 0.0429688,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.7304688,
        blue: 0.0507813,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.7500000,
        blue: 0.0585938,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.7734375,
        blue: 0.0664063,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.7929688,
        blue: 0.0781250,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.8125000,
        blue: 0.0859375,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.8320313,
        blue: 0.0937500,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.8515625,
        blue: 0.1015625,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.8710938,
        blue: 0.1093750,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.8867188,
        blue: 0.1171875,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.9062500,
        blue: 0.1250000,
    },
    Spectrum {
        red: 0.9960938,
        green: 0.9218750,
        blue: 0.1328125,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.9375000,
        blue: 0.1367188,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.9492188,
        blue: 0.1445313,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.9609375,
        blue: 0.1484375,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.9726563,
        blue: 0.1523438,
    },
    Spectrum {
        red: 0.9921875,
        green: 0.9843750,
        blue: 0.1562500,
    },
];

// /// Radiance's standard falsecolour colourmap
// pub const RADIANCE_COLOURMAP: [Spectrum; 23] = [
//     Spectrum {
//         red: 0.1884800,
//         green: 0.0009766,
//         blue: 0.2666000,
//     },
//     Spectrum {
//         red: 0.0546817,
//         green: 0.0000236,
//         blue: 0.3638662,
//     },
//     Spectrum {
//         red: 0.0010355,
//         green: 0.0008966,
//         blue: 0.4770437,
//     },
//     Spectrum {
//         red: 0.0000001,
//         green: 0.0264977,
//         blue: 0.5131397,
//     },
//     Spectrum {
//         red: 0.0000074,
//         green: 0.1256843,
//         blue: 0.5363797,
//     },
//     Spectrum {
//         red: 0.0004391,
//         green: 0.2865799,
//         blue: 0.5193677,
//     },
//     Spectrum {
//         red: 0.0013673,
//         green: 0.4247083,
//         blue: 0.4085123,
//     },
//     Spectrum {
//         red: 0.0030760,
//         green: 0.4739468,
//         blue: 0.1702815,
//     },
//     Spectrum {
//         red: 0.0137638,
//         green: 0.4402732,
//         blue: 0.0531424,
//     },
//     Spectrum {
//         red: 0.0617077,
//         green: 0.3671876,
//         blue: 0.0519406,
//     },
//     Spectrum {
//         red: 0.1739422,
//         green: 0.2629843,
//         blue: 0.0856408,
//     },
//     Spectrum {
//         red: 0.2881156,
//         green: 0.1725325,
//         blue: 0.0988140,
//     },
//     Spectrum {
//         red: 0.3299725,
//         green: 0.1206819,
//         blue: 0.0832437,
//     },
//     Spectrum {
//         red: 0.3552663,
//         green: 0.0731664,
//         blue: 0.0607290,
//     },
//     Spectrum {
//         red: 0.3725520,
//         green: 0.0376103,
//         blue: 0.0391076,
//     },
//     Spectrum {
//         red: 0.3921184,
//         green: 0.0161236,
//         blue: 0.0231535,
//     },
//     Spectrum {
//         red: 0.4363976,
//         green: 0.0047737,
//         blue: 0.0128446,
//     },
//     Spectrum {
//         red: 0.6102754,
//         green: 0.0000068,
//         blue: 0.0051847,
//     },
//     Spectrum {
//         red: 0.7757267,
//         green: 0.0080361,
//         blue: 0.0016918,
//     },
//     Spectrum {
//         red: 0.9087369,
//         green: 0.1008085,
//         blue: 0.0000243,
//     },
//     Spectrum {
//         red: 1.0000000,
//         green: 0.3106831,
//         blue: 0.0000121,
//     },
//     Spectrum {
//         red: 1.0000000,
//         green: 0.6447838,
//         blue: 0.0066594,
//     },
//     Spectrum {
//         red: 0.9863000,
//         green: 0.9707000,
//         blue: 0.0253900,
//     },
// ];

