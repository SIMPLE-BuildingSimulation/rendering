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

// From Radiance's source code somewhere... I don't remember.

use crate::colour::Spectrum;

/// Radiance's standard falsecolour colourmap
pub const RADIANCE_COLOURMAP: [Spectrum; 23] = [
    Spectrum {
        red: 0.1884800,
        green: 0.0009766,
        blue: 0.2666000,
    },
    Spectrum {
        red: 0.0546817,
        green: 0.0000236,
        blue: 0.3638662,
    },
    Spectrum {
        red: 0.0010355,
        green: 0.0008966,
        blue: 0.4770437,
    },
    Spectrum {
        red: 0.0000001,
        green: 0.0264977,
        blue: 0.5131397,
    },
    Spectrum {
        red: 0.0000074,
        green: 0.1256843,
        blue: 0.5363797,
    },
    Spectrum {
        red: 0.0004391,
        green: 0.2865799,
        blue: 0.5193677,
    },
    Spectrum {
        red: 0.0013673,
        green: 0.4247083,
        blue: 0.4085123,
    },
    Spectrum {
        red: 0.0030760,
        green: 0.4739468,
        blue: 0.1702815,
    },
    Spectrum {
        red: 0.0137638,
        green: 0.4402732,
        blue: 0.0531424,
    },
    Spectrum {
        red: 0.0617077,
        green: 0.3671876,
        blue: 0.0519406,
    },
    Spectrum {
        red: 0.1739422,
        green: 0.2629843,
        blue: 0.0856408,
    },
    Spectrum {
        red: 0.2881156,
        green: 0.1725325,
        blue: 0.0988140,
    },
    Spectrum {
        red: 0.3299725,
        green: 0.1206819,
        blue: 0.0832437,
    },
    Spectrum {
        red: 0.3552663,
        green: 0.0731664,
        blue: 0.0607290,
    },
    Spectrum {
        red: 0.3725520,
        green: 0.0376103,
        blue: 0.0391076,
    },
    Spectrum {
        red: 0.3921184,
        green: 0.0161236,
        blue: 0.0231535,
    },
    Spectrum {
        red: 0.4363976,
        green: 0.0047737,
        blue: 0.0128446,
    },
    Spectrum {
        red: 0.6102754,
        green: 0.0000068,
        blue: 0.0051847,
    },
    Spectrum {
        red: 0.7757267,
        green: 0.0080361,
        blue: 0.0016918,
    },
    Spectrum {
        red: 0.9087369,
        green: 0.1008085,
        blue: 0.0000243,
    },
    Spectrum {
        red: 1.0000000,
        green: 0.3106831,
        blue: 0.0000121,
    },
    Spectrum {
        red: 1.0000000,
        green: 0.6447838,
        blue: 0.0066594,
    },
    Spectrum {
        red: 0.9863000,
        green: 0.9707000,
        blue: 0.0253900,
    },
];
