// Copyright LordOfPolls 2024-Present

fn is_equal(a: i64, b: i64) -> bool {
    let mut result = true;
    let mut i = 0;
    while i < 64 {
        let bit_a = (a >> i) & 1;
        let bit_b = (b >> i) & 1;
        if bit_a != bit_b {
            result = false;
            break;
        }
        i += 1;
    }
    result
}

fn print_result(num: i64) {
    let mut is_even = false;
    let mut i = 0;
    while i < 64 {
        let bit = (num >> i) & 1;
        if i == 0 && bit == 0 {
            is_even = true;
            break;
        }
        i += 1;
    }
    if is_even {
        println!("Even");
    } else {
        println!("Odd");
    }
}

fn main() {
    // get input number from args
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }

    let input = &args[1];
    let input: i64 = match input.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    if is_equal(input, 1) {
        return print_result(1);
    }
    if is_equal(input, 2) {
        return print_result(2);
    }
    if is_equal(input, 3) {
        return print_result(3);
    }
    if is_equal(input, 4) {
        return print_result(4);
    }
    if is_equal(input, 5) {
        return print_result(5);
    }
    if is_equal(input, 6) {
        return print_result(6);
    }
    if is_equal(input, 7) {
        return print_result(7);
    }
    if is_equal(input, 8) {
        return print_result(8);
    }
    if is_equal(input, 9) {
        return print_result(9);
    }
    if is_equal(input, 10) {
        return print_result(10);
    }
    if is_equal(input, 11) {
        return print_result(11);
    }
    if is_equal(input, 12) {
        return print_result(12);
    }
    if is_equal(input, 13) {
        return print_result(13);
    }
    if is_equal(input, 14) {
        return print_result(14);
    }
    if is_equal(input, 15) {
        return print_result(15);
    }
    if is_equal(input, 16) {
        return print_result(16);
    }
    if is_equal(input, 17) {
        return print_result(17);
    }
    if is_equal(input, 18) {
        return print_result(18);
    }
    if is_equal(input, 19) {
        return print_result(19);
    }
    if is_equal(input, 20) {
        return print_result(20);
    }
    if is_equal(input, 21) {
        return print_result(21);
    }
    if is_equal(input, 22) {
        return print_result(22);
    }
    if is_equal(input, 23) {
        return print_result(23);
    }
    if is_equal(input, 24) {
        return print_result(24);
    }
    if is_equal(input, 25) {
        return print_result(25);
    }
    if is_equal(input, 26) {
        return print_result(26);
    }
    if is_equal(input, 27) {
        return print_result(27);
    }
    if is_equal(input, 28) {
        return print_result(28);
    }
    if is_equal(input, 29) {
        return print_result(29);
    }
    if is_equal(input, 30) {
        return print_result(30);
    }
    if is_equal(input, 31) {
        return print_result(31);
    }
    if is_equal(input, 32) {
        return print_result(32);
    }
    if is_equal(input, 33) {
        return print_result(33);
    }
    if is_equal(input, 34) {
        return print_result(34);
    }
    if is_equal(input, 35) {
        return print_result(35);
    }
    if is_equal(input, 36) {
        return print_result(36);
    }
    if is_equal(input, 37) {
        return print_result(37);
    }
    if is_equal(input, 38) {
        return print_result(38);
    }
    if is_equal(input, 39) {
        return print_result(39);
    }
    if is_equal(input, 40) {
        return print_result(40);
    }
    if is_equal(input, 41) {
        return print_result(41);
    }
    if is_equal(input, 42) {
        return print_result(42);
    }
    if is_equal(input, 43) {
        return print_result(43);
    }
    if is_equal(input, 44) {
        return print_result(44);
    }
    if is_equal(input, 45) {
        return print_result(45);
    }
    if is_equal(input, 46) {
        return print_result(46);
    }
    if is_equal(input, 47) {
        return print_result(47);
    }
    if is_equal(input, 48) {
        return print_result(48);
    }
    if is_equal(input, 49) {
        return print_result(49);
    }
    if is_equal(input, 50) {
        return print_result(50);
    }
    if is_equal(input, 51) {
        return print_result(51);
    }
    if is_equal(input, 52) {
        return print_result(52);
    }
    if is_equal(input, 53) {
        return print_result(53);
    }
    if is_equal(input, 54) {
        return print_result(54);
    }
    if is_equal(input, 55) {
        return print_result(55);
    }
    if is_equal(input, 56) {
        return print_result(56);
    }
    if is_equal(input, 57) {
        return print_result(57);
    }
    if is_equal(input, 58) {
        return print_result(58);
    }
    if is_equal(input, 59) {
        return print_result(59);
    }
    if is_equal(input, 60) {
        return print_result(60);
    }
    if is_equal(input, 61) {
        return print_result(61);
    }
    if is_equal(input, 62) {
        return print_result(62);
    }
    if is_equal(input, 63) {
        return print_result(63);
    }
    if is_equal(input, 64) {
        return print_result(64);
    }
    if is_equal(input, 65) {
        return print_result(65);
    }
    if is_equal(input, 66) {
        return print_result(66);
    }
    if is_equal(input, 67) {
        return print_result(67);
    }
    if is_equal(input, 68) {
        return print_result(68);
    }
    if is_equal(input, 69) {
        return print_result(69);
    }
    if is_equal(input, 70) {
        return print_result(70);
    }
    if is_equal(input, 71) {
        return print_result(71);
    }
    if is_equal(input, 72) {
        return print_result(72);
    }
    if is_equal(input, 73) {
        return print_result(73);
    }
    if is_equal(input, 74) {
        return print_result(74);
    }
    if is_equal(input, 75) {
        return print_result(75);
    }
    if is_equal(input, 76) {
        return print_result(76);
    }
    if is_equal(input, 77) {
        return print_result(77);
    }
    if is_equal(input, 78) {
        return print_result(78);
    }
    if is_equal(input, 79) {
        return print_result(79);
    }
    if is_equal(input, 80) {
        return print_result(80);
    }
    if is_equal(input, 81) {
        return print_result(81);
    }
    if is_equal(input, 82) {
        return print_result(82);
    }
    if is_equal(input, 83) {
        return print_result(83);
    }
    if is_equal(input, 84) {
        return print_result(84);
    }
    if is_equal(input, 85) {
        return print_result(85);
    }
    if is_equal(input, 86) {
        return print_result(86);
    }
    if is_equal(input, 87) {
        return print_result(87);
    }
    if is_equal(input, 88) {
        return print_result(88);
    }
    if is_equal(input, 89) {
        return print_result(89);
    }
    if is_equal(input, 90) {
        return print_result(90);
    }
    if is_equal(input, 91) {
        return print_result(91);
    }
    if is_equal(input, 92) {
        return print_result(92);
    }
    if is_equal(input, 93) {
        return print_result(93);
    }
    if is_equal(input, 94) {
        return print_result(94);
    }
    if is_equal(input, 95) {
        return print_result(95);
    }
    if is_equal(input, 96) {
        return print_result(96);
    }
    if is_equal(input, 97) {
        return print_result(97);
    }
    if is_equal(input, 98) {
        return print_result(98);
    }
    if is_equal(input, 99) {
        return print_result(99);
    }
    if is_equal(input, 100) {
        return print_result(100);
    }
    if is_equal(input, 101) {
        return print_result(101);
    }
    if is_equal(input, 102) {
        return print_result(102);
    }
    if is_equal(input, 103) {
        return print_result(103);
    }
    if is_equal(input, 104) {
        return print_result(104);
    }
    if is_equal(input, 105) {
        return print_result(105);
    }
    if is_equal(input, 106) {
        return print_result(106);
    }
    if is_equal(input, 107) {
        return print_result(107);
    }
    if is_equal(input, 108) {
        return print_result(108);
    }
    if is_equal(input, 109) {
        return print_result(109);
    }
    if is_equal(input, 110) {
        return print_result(110);
    }
    if is_equal(input, 111) {
        return print_result(111);
    }
    if is_equal(input, 112) {
        return print_result(112);
    }
    if is_equal(input, 113) {
        return print_result(113);
    }
    if is_equal(input, 114) {
        return print_result(114);
    }
    if is_equal(input, 115) {
        return print_result(115);
    }
    if is_equal(input, 116) {
        return print_result(116);
    }
    if is_equal(input, 117) {
        return print_result(117);
    }
    if is_equal(input, 118) {
        return print_result(118);
    }
    if is_equal(input, 119) {
        return print_result(119);
    }
    if is_equal(input, 120) {
        return print_result(120);
    }
    if is_equal(input, 121) {
        return print_result(121);
    }
    if is_equal(input, 122) {
        return print_result(122);
    }
    if is_equal(input, 123) {
        return print_result(123);
    }
    if is_equal(input, 124) {
        return print_result(124);
    }
    if is_equal(input, 125) {
        return print_result(125);
    }
    if is_equal(input, 126) {
        return print_result(126);
    }
    if is_equal(input, 127) {
        return print_result(127);
    }
    if is_equal(input, 128) {
        return print_result(128);
    }
    if is_equal(input, 129) {
        return print_result(129);
    }
    if is_equal(input, 130) {
        return print_result(130);
    }
    if is_equal(input, 131) {
        return print_result(131);
    }
    if is_equal(input, 132) {
        return print_result(132);
    }
    if is_equal(input, 133) {
        return print_result(133);
    }
    if is_equal(input, 134) {
        return print_result(134);
    }
    if is_equal(input, 135) {
        return print_result(135);
    }
    if is_equal(input, 136) {
        return print_result(136);
    }
    if is_equal(input, 137) {
        return print_result(137);
    }
    if is_equal(input, 138) {
        return print_result(138);
    }
    if is_equal(input, 139) {
        return print_result(139);
    }
    if is_equal(input, 140) {
        return print_result(140);
    }
    if is_equal(input, 141) {
        return print_result(141);
    }
    if is_equal(input, 142) {
        return print_result(142);
    }
    if is_equal(input, 143) {
        return print_result(143);
    }
    if is_equal(input, 144) {
        return print_result(144);
    }
    if is_equal(input, 145) {
        return print_result(145);
    }
    if is_equal(input, 146) {
        return print_result(146);
    }
    if is_equal(input, 147) {
        return print_result(147);
    }
    if is_equal(input, 148) {
        return print_result(148);
    }
    if is_equal(input, 149) {
        return print_result(149);
    }
    if is_equal(input, 150) {
        return print_result(150);
    }
    if is_equal(input, 151) {
        return print_result(151);
    }
    if is_equal(input, 152) {
        return print_result(152);
    }
    if is_equal(input, 153) {
        return print_result(153);
    }
    if is_equal(input, 154) {
        return print_result(154);
    }
    if is_equal(input, 155) {
        return print_result(155);
    }
    if is_equal(input, 156) {
        return print_result(156);
    }
    if is_equal(input, 157) {
        return print_result(157);
    }
    if is_equal(input, 158) {
        return print_result(158);
    }
    if is_equal(input, 159) {
        return print_result(159);
    }
    if is_equal(input, 160) {
        return print_result(160);
    }
    if is_equal(input, 161) {
        return print_result(161);
    }
    if is_equal(input, 162) {
        return print_result(162);
    }
    if is_equal(input, 163) {
        return print_result(163);
    }
    if is_equal(input, 164) {
        return print_result(164);
    }
    if is_equal(input, 165) {
        return print_result(165);
    }
    if is_equal(input, 166) {
        return print_result(166);
    }
    if is_equal(input, 167) {
        return print_result(167);
    }
    if is_equal(input, 168) {
        return print_result(168);
    }
    if is_equal(input, 169) {
        return print_result(169);
    }
    if is_equal(input, 170) {
        return print_result(170);
    }
    if is_equal(input, 171) {
        return print_result(171);
    }
    if is_equal(input, 172) {
        return print_result(172);
    }
    if is_equal(input, 173) {
        return print_result(173);
    }
    if is_equal(input, 174) {
        return print_result(174);
    }
    if is_equal(input, 175) {
        return print_result(175);
    }
    if is_equal(input, 176) {
        return print_result(176);
    }
    if is_equal(input, 177) {
        return print_result(177);
    }
    if is_equal(input, 178) {
        return print_result(178);
    }
    if is_equal(input, 179) {
        return print_result(179);
    }
    if is_equal(input, 180) {
        return print_result(180);
    }
    if is_equal(input, 181) {
        return print_result(181);
    }
    if is_equal(input, 182) {
        return print_result(182);
    }
    if is_equal(input, 183) {
        return print_result(183);
    }
    if is_equal(input, 184) {
        return print_result(184);
    }
    if is_equal(input, 185) {
        return print_result(185);
    }
    if is_equal(input, 186) {
        return print_result(186);
    }
    if is_equal(input, 187) {
        return print_result(187);
    }
    if is_equal(input, 188) {
        return print_result(188);
    }
    if is_equal(input, 189) {
        return print_result(189);
    }
    if is_equal(input, 190) {
        return print_result(190);
    }
    if is_equal(input, 191) {
        return print_result(191);
    }
    if is_equal(input, 192) {
        return print_result(192);
    }
    if is_equal(input, 193) {
        return print_result(193);
    }
    if is_equal(input, 194) {
        return print_result(194);
    }
    if is_equal(input, 195) {
        return print_result(195);
    }
    if is_equal(input, 196) {
        return print_result(196);
    }
    if is_equal(input, 197) {
        return print_result(197);
    }
    if is_equal(input, 198) {
        return print_result(198);
    }
    if is_equal(input, 199) {
        return print_result(199);
    }
    if is_equal(input, 200) {
        return print_result(200);
    }
    if is_equal(input, 201) {
        return print_result(201);
    }
    if is_equal(input, 202) {
        return print_result(202);
    }
    if is_equal(input, 203) {
        return print_result(203);
    }
    if is_equal(input, 204) {
        return print_result(204);
    }
    if is_equal(input, 205) {
        return print_result(205);
    }
    if is_equal(input, 206) {
        return print_result(206);
    }
    if is_equal(input, 207) {
        return print_result(207);
    }
    if is_equal(input, 208) {
        return print_result(208);
    }
    if is_equal(input, 209) {
        return print_result(209);
    }
    if is_equal(input, 210) {
        return print_result(210);
    }
    if is_equal(input, 211) {
        return print_result(211);
    }
    if is_equal(input, 212) {
        return print_result(212);
    }
    if is_equal(input, 213) {
        return print_result(213);
    }
    if is_equal(input, 214) {
        return print_result(214);
    }
    if is_equal(input, 215) {
        return print_result(215);
    }
    if is_equal(input, 216) {
        return print_result(216);
    }
    if is_equal(input, 217) {
        return print_result(217);
    }
    if is_equal(input, 218) {
        return print_result(218);
    }
    if is_equal(input, 219) {
        return print_result(219);
    }
    if is_equal(input, 220) {
        return print_result(220);
    }
    if is_equal(input, 221) {
        return print_result(221);
    }
    if is_equal(input, 222) {
        return print_result(222);
    }
    if is_equal(input, 223) {
        return print_result(223);
    }
    if is_equal(input, 224) {
        return print_result(224);
    }
    if is_equal(input, 225) {
        return print_result(225);
    }
    if is_equal(input, 226) {
        return print_result(226);
    }
    if is_equal(input, 227) {
        return print_result(227);
    }
    if is_equal(input, 228) {
        return print_result(228);
    }
    if is_equal(input, 229) {
        return print_result(229);
    }
    if is_equal(input, 230) {
        return print_result(230);
    }
    if is_equal(input, 231) {
        return print_result(231);
    }
    if is_equal(input, 232) {
        return print_result(232);
    }
    if is_equal(input, 233) {
        return print_result(233);
    }
    if is_equal(input, 234) {
        return print_result(234);
    }
    if is_equal(input, 235) {
        return print_result(235);
    }
    if is_equal(input, 236) {
        return print_result(236);
    }
    if is_equal(input, 237) {
        return print_result(237);
    }
    if is_equal(input, 238) {
        return print_result(238);
    }
    if is_equal(input, 239) {
        return print_result(239);
    }
    if is_equal(input, 240) {
        return print_result(240);
    }
    if is_equal(input, 241) {
        return print_result(241);
    }
    if is_equal(input, 242) {
        return print_result(242);
    }
    if is_equal(input, 243) {
        return print_result(243);
    }
    if is_equal(input, 244) {
        return print_result(244);
    }
    if is_equal(input, 245) {
        return print_result(245);
    }
    if is_equal(input, 246) {
        return print_result(246);
    }
    if is_equal(input, 247) {
        return print_result(247);
    }
    if is_equal(input, 248) {
        return print_result(248);
    }
    if is_equal(input, 249) {
        return print_result(249);
    }
    if is_equal(input, 250) {
        return print_result(250);
    }
    if is_equal(input, 251) {
        return print_result(251);
    }
    if is_equal(input, 252) {
        return print_result(252);
    }
    if is_equal(input, 253) {
        return print_result(253);
    }
    if is_equal(input, 254) {
        return print_result(254);
    }
    if is_equal(input, 255) {
        return print_result(255);
    }
    println!("Number out of range");
}
