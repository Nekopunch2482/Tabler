use std::collections::HashMap;

pub fn get_symbol_map() -> HashMap<&'static str, &'static str> {
    let mut symbol_map: HashMap<_, _> = HashMap::new();
    symbol_map.insert("F020", " ");
    symbol_map.insert("F021", "!");
    symbol_map.insert("F022", "∀");
    symbol_map.insert("F023", "#");
    symbol_map.insert("F024", "∃");
    symbol_map.insert("F025", "%");
    symbol_map.insert("F026", "&");
    symbol_map.insert("F027", "∋");
    symbol_map.insert("F028", "(");
    symbol_map.insert("F029", ")");
    symbol_map.insert("F02A", "∗");
    symbol_map.insert("F02B", "+");
    symbol_map.insert("F02C", ",");
    symbol_map.insert("F02D", "−");
    symbol_map.insert("F02E", ".");
    symbol_map.insert("F02F", "/");
    symbol_map.insert("F030", "0");
    symbol_map.insert("F031", "1");
    symbol_map.insert("F032", "2");
    symbol_map.insert("F033", "3");
    symbol_map.insert("F034", "4");
    symbol_map.insert("F035", "5");
    symbol_map.insert("F036", "6");
    symbol_map.insert("F037", "7");
    symbol_map.insert("F038", "8");
    symbol_map.insert("F039", "9");
    symbol_map.insert("F03A", ":");
    symbol_map.insert("F03B", ";");
    symbol_map.insert("F03C", "<");
    symbol_map.insert("F03D", "=");
    symbol_map.insert("F03E", ">");
    symbol_map.insert("F03F", "?");
    symbol_map.insert("F040", "≅");
    symbol_map.insert("F041", "A");
    symbol_map.insert("F042", "B");
    symbol_map.insert("F043", "C");
    symbol_map.insert("F044", "D");
    symbol_map.insert("F045", "E");
    symbol_map.insert("F046", "F");
    symbol_map.insert("F047", "G");
    symbol_map.insert("F048", "H");
    symbol_map.insert("F049", "I");
    symbol_map.insert("F04A", "J");
    symbol_map.insert("F04B", "K");
    symbol_map.insert("F04C", "L");
    symbol_map.insert("F04D", "M");
    symbol_map.insert("F04E", "N");
    symbol_map.insert("F04F", "O");
    symbol_map.insert("F050", "P");
    symbol_map.insert("F051", "Q");
    symbol_map.insert("F052", "R");
    symbol_map.insert("F053", "S");
    symbol_map.insert("F054", "T");
    symbol_map.insert("F055", "U");
    symbol_map.insert("F056", "V");
    symbol_map.insert("F057", "W");
    symbol_map.insert("F058", "X");
    symbol_map.insert("F059", "Y");
    symbol_map.insert("F05A", "Z");
    symbol_map.insert("F05B", "[");
    symbol_map.insert("F05C", "∖");
    symbol_map.insert("F05D", "]");
    symbol_map.insert("F05E", "⌃");
    symbol_map.insert("F05F", "_");
    symbol_map.insert("F060", "∼");
    symbol_map.insert("F061", "a");
    symbol_map.insert("F062", "b");
    symbol_map.insert("F063", "c");
    symbol_map.insert("F064", "d");
    symbol_map.insert("F065", "e");
    symbol_map.insert("F066", "f");
    symbol_map.insert("F067", "g");
    symbol_map.insert("F068", "h");
    symbol_map.insert("F069", "i");
    symbol_map.insert("F06A", "j");
    symbol_map.insert("F06B", "k");
    symbol_map.insert("F06C", "l");
    symbol_map.insert("F06D", "m");
    symbol_map.insert("F06E", "n");
    symbol_map.insert("F06F", "o");
    symbol_map.insert("F070", "p");
    symbol_map.insert("F071", "q");
    symbol_map.insert("F072", "r");
    symbol_map.insert("F073", "s");
    symbol_map.insert("F074", "t");
    symbol_map.insert("F075", "u");
    symbol_map.insert("F076", "v");
    symbol_map.insert("F077", "w");
    symbol_map.insert("F078", "x");
    symbol_map.insert("F079", "y");
    symbol_map.insert("F07A", "z");
    symbol_map.insert("F07B", "{");
    symbol_map.insert("F07C", "|");
    symbol_map.insert("F07D", "}");
    symbol_map.insert("F07E", "∽");
    symbol_map.insert("F0A0", "Δ");
    symbol_map.insert("F0A1", "Ψ");
    symbol_map.insert("F0A2", "Γ");
    symbol_map.insert("F0A3", "Θ");
    symbol_map.insert("F0A4", "Λ");
    symbol_map.insert("F0A5", "Ξ");
    symbol_map.insert("F0A6", "Π");
    symbol_map.insert("F0A7", "Σ");
    symbol_map.insert("F0A8", "Υ");
    symbol_map.insert("F0A9", "Φ");
    symbol_map.insert("F0AA", "Ω");
    symbol_map.insert("F0AB", "α");
    symbol_map.insert("F0AC", "β");
    symbol_map.insert("F0AD", "γ");
    symbol_map.insert("F0AE", "δ");
    symbol_map.insert("F0AF", "ε");
    symbol_map.insert("F0B0", "ζ");
    symbol_map.insert("F0B1", "±");
    symbol_map.insert("F0B2", "η");
    symbol_map.insert("F0B3", "θ");
    symbol_map.insert("F0B4", "ι");
    symbol_map.insert("F0B5", "κ");
    symbol_map.insert("F0B6", "λ");
    symbol_map.insert("F0B7", "•");
    symbol_map.insert("F0B8", "ν");
    symbol_map.insert("F0B9", "ξ");
    symbol_map.insert("F0BA", "ο");
    symbol_map.insert("F0BB", "π");
    symbol_map.insert("F0BC", "ρ");
    symbol_map.insert("F0BD", "σ");
    symbol_map.insert("F0BE", "τ");
    symbol_map.insert("F0BF", "υ");
    symbol_map.insert("F0C0", "φ");
    symbol_map.insert("F0C1", "χ");
    symbol_map.insert("F0C2", "ψ");
    symbol_map.insert("F0C3", "ω");
    symbol_map.insert("F0C4", "⊂");
    symbol_map.insert("F0C5", "⊃");
    symbol_map.insert("F0C6", "⊆");
    symbol_map.insert("F0C7", "⊇");
    symbol_map.insert("F0C8", "∪");
    symbol_map.insert("F0C9", "∩");
    symbol_map.insert("F0CA", "∈");
    symbol_map.insert("F0CB", "∉");
    symbol_map.insert("F0CC", "∠");
    symbol_map.insert("F0CD", "⊥");
    symbol_map.insert("F0CE", "∧");
    symbol_map.insert("F0CF", "∨");
    symbol_map.insert("F0D0", "⇒");
    symbol_map.insert("F0D1", "⇔");
    symbol_map.insert("F0D2", "≤");
    symbol_map.insert("F0D3", "≥");
    symbol_map.insert("F0D4", "≠");
    symbol_map.insert("F0D5", "≈");
    symbol_map.insert("F0D6", "∝");
    symbol_map.insert("F0D7", "×");
    symbol_map.insert("F0D8", "∞");
    symbol_map.insert("F0D9", "÷");
    symbol_map.insert("F0DA", "Α");
    symbol_map.insert("F0DB", "Β");
    symbol_map.insert("F0DC", "∇");
    symbol_map.insert("F0DD", "Ε");
    symbol_map.insert("F0DE", "Ζ");
    symbol_map.insert("F0DF", "Η");
    symbol_map.insert("F0E0", "Ι");
    symbol_map.insert("F0E1", "Κ");
    symbol_map.insert("F0E2", "Μ");
    symbol_map.insert("F0E3", "Ν");
    symbol_map.insert("F0E4", "Ο");
    symbol_map.insert("F0E5", "Ρ");
    symbol_map.insert("F0E6", "Τ");
    symbol_map.insert("F0E7", "Χ");
    symbol_map.insert("F0E8", "∫");
    symbol_map.insert("F0E9", "∑");
    symbol_map.insert("F0EA", "∏");
    symbol_map.insert("F0EB", "√");
    symbol_map.insert("F0EC", "¬");
    symbol_map.insert("F0ED", "⋅");
    symbol_map.insert("F0EE", "ˆ");
    symbol_map.insert("F0EF", "˜");
    symbol_map.insert("F0F0", "°");
    symbol_map.insert("F0F1", "˚");
    symbol_map.insert("F0F2", "′");
    symbol_map.insert("F0F3", "″");
    symbol_map.insert("F0F4", "≡");
    symbol_map.insert("F0F5", "⊄");
    symbol_map.insert("F0F6", "⊈");
    symbol_map.insert("F0F7", "⊉");
    symbol_map.insert("F0F8", "∂");
    symbol_map.insert("F0F9", "ƒ");
    symbol_map.insert("F0FA", "ϰ");
    symbol_map.insert("F0FB", "ς");
    symbol_map.insert("F0FC", "ϑ");
    symbol_map.insert("F0FD", "ϕ");
    symbol_map.insert("F0FE", "ϖ");
    symbol_map.insert("F0FF", "℘");
    symbol_map
}
