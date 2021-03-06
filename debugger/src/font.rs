pub const FONT_WIDTH: usize = 4;
pub const FONT_HEIGHT: usize = 8;
pub const FONT: [[u8; FONT_WIDTH]; 256] = [
    //0x
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],

    //1x
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],

    //2x
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0, 0b00111100, 0b01000010, 0b00000000], //(
    [0, 0b01000010, 0b00111100, 0b00000000], //)
    [0; 4],
    [0; 4],
    [0, 0b00000000, 0b01100000, 0b00000000], //,
    [0; 4],
    [0; 4],
    [0; 4],

    //0-9
    [0, 0b00111110, 0b01000001, 0b00111110],//0
    [0, 0b01000010, 0b01111111, 0b01000000],//1
    [0, 0b01100010, 0b01010001, 0b01001110],//2
    [0, 0b00100010, 0b01001001, 0b00110110],//3
    [0, 0b00001111, 0b00001000, 0b01111111],//4
    [0, 0b00101111, 0b01001001, 0b00110001],//5
    [0, 0b00111110, 0b01001001, 0b00110010],//6
    [0, 0b00000001, 0b01111001, 0b00000111],//7
    [0, 0b00110110, 0b01001001, 0b00110110],//8
    [0, 0b00100110, 0b01001001, 0b00111110],//9
    [0, 0b00000000, 0b00101000, 0b00000000],//:
    [0, 0b00000000, 0b01101000, 0b00000000],//;
    [0, 0b00001000, 0b00010100, 0b00100010],//<
    [0, 0b00010100, 0b00010100, 0b00010100],//=
    [0, 0b00100010, 0b00010100, 0b00001000],//>
    [0; 4],//?
    [0; 4],//@
    //A-Z
    [0, 0b01111110, 0b00010001, 0b01111110],//A
    [0, 0b01111111, 0b01001001, 0b00110110],//B
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4], //Z
    [0, 0b01111111, 0b01000001, 0b00000000],//[
    [0; 4],
    [0, 0b01000001, 0b01111111, 0b00000000],//]
    [0; 4],
    [0; 4],
    [0; 4],
    //a-z
    [0, 0b00110000, 0b01001000, 0b01110000],//a
    [0, 0b01111110, 0b01001000, 0b00110000],//b
    [0, 0b00110000, 0b01001000, 0b00000000],//c
    [0, 0b00110000, 0b01001000, 0b01111110],//d
    [0, 0b00110000, 0b01011000, 0b00010000],//e
    [0, 0b01111110, 0b00001001, 0b00001010],//f
    [0, 0b00110000, 0b01001000, 0b11100000],//g
    [0, 0b01111110, 0b00001000, 0b01110000],//h
    [0, 0b00000000, 0b01110100, 0b00000000],//i
    [0, 0b10000000, 0b01110100, 0b00000000],//j
    [0, 0b01111110, 0b00100000, 0b01010000],//k
    [0, 0b00111110, 0b01000000, 0b00000000],//l
    [0, 0b01111000, 0b00011000, 0b01110000],//m
    [0, 0b01111000, 0b00001000, 0b01110000],//n
    [0, 0b00110000, 0b01001000, 0b00110000],//o
    [0, 0b01111000, 0b00101000, 0b00010000],//p
    [0, 0b00110000, 0b01001000, 0b11110000],//q
    [0, 0b01110000, 0b00001000, 0b00001000],//r
    [0, 0b01010000, 0b01011000, 0b00101000],//s
    [0, 0b00111100, 0b01001000, 0b01001000],//t
    [0, 0b00111000, 0b01000000, 0b01111000],//u
    [0, 0b00111000, 0b01000000, 0b00111000],//v
    [0, 0b01111000, 0b00100000, 0b01111000],//w
    [0, 0b01010000, 0b00100000, 0b01010000],//x
    [0, 0b00111000, 0b11000000, 0b01111000],//y
    [0, 0b01001000, 0b01101000, 0b01011000],//z
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],

    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4], //
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4], //
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
    [0; 4],
];
