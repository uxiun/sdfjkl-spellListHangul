#![allow(dead_code, unused)]
#![feature(iter_intersperse)]
use std::array::IntoIter;
use std::fs;
use std::hash::Hash;
use std::io::{BufReader, BufWriter, Write};
// use slab_tree::*;
// use std::convert::TryInto;
use lei::{fu_yubi_pre, rule2, Dan, Kagi, Rules2, Yubi};
use std::collections::HashMap;
use std::str::FromStr;
use trie_rs::{Trie, TrieBuilder};
pub mod util;
use std::thread::sleep;
use std::time::{Duration, Instant};
use util::TrieU8;

use crate::lei::{keymapping, GenerateSetting, Sayu, Table};
mod lei;

#[macro_export]
macro_rules! time_measure {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(
            "計測開始から{}.{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        result
    }};
}
fn main() {
    // let hangul_trie: Trie<u8> = time_measure!({ util::tried() });
    let setting = GenerateSetting {
        len: 3,
        lenonly: false,
        table: Table::Nv,
        rule: Rules2::WnLittleFluent,
    };
    // println!("{}", "$SPACE");
    exe_spell_list(true, &setting);
    // exe_ahk_hangul(&setting);
}

fn exe_spell_list(with_hangul: bool, setting: &GenerateSetting) {
    let hangul_trie: Trie<u8> = util::tried();
    let name = "spell".to_string()
        + &setting.len.to_string()
        + if setting.lenonly { "_only" } else { "" }
        + if with_hangul { "_hangul" } else { "" }
        + match &setting.rule {
            &Rules2::Fluent => "_fluent",
            &Rules2::WnLittleFluent => "_wn",
        }
        + ".txt";
    time_measure!({
        if with_hangul {
            spell_list_customizable(&name, setting, &hangul_trie);
            // spell_list_with_hangul(&name, setting, &hangul_trie);
        } else {
            spell_list(&name, setting);
        }
    });
}

fn exe_ahk_hangul(setting: &GenerateSetting) {
    let name = "spell_hangul".to_string()
        + if setting.lenonly { "_only" } else { "" }
        + match &setting.rule {
            &Rules2::Fluent => "_fluent",
            &Rules2::WnLittleFluent => "_wn",
        }
        + ".ahk";
    time_measure!({
        write_ahk_hangul(&name, setting);
    });
}

fn write_ahk_hangul(filename: &str, setting: &GenerateSetting) {
    let hangul_trie = util::tried();
    let con = gen_ahk_hangul(setting, &hangul_trie);
    let dir = String::from("ahk/");
    let name = dir + filename;
    bufwrite(&name, &con);
}

fn spell_list<'a>(filename: &str, setting: &GenerateSetting) {
    // let keymap = keymapping();

    let content = generate_flatlist(setting);

    let dir = String::from("table/");
    let name = dir + filename;
    bufwrite(&name, &content);
}
fn spell_list_with_hangul<'a>(
    filename: &str,
    setting: &GenerateSetting,
    hangul_trie: &'a Trie<u8>,
) {
    // let keymap = keymapping();
    let content = generate_flatlist_with_hangul(setting, hangul_trie);
    let dir = String::from("table/");
    let name = dir + filename;
    bufwrite(&name, &content);
}

fn spell_list_customizable<'a>(
    filename: &str,
    setting: &GenerateSetting,
    hangul_trie: &'a Trie<u8>,
) {
    let content = yong_def_affixed(
        spellvecvec(setting, hangul_trie),
        "a".to_string(),
        "$SPACE".to_string(),
    );
    let dir = String::from("table/");
    let name = dir + filename;
    bufwrite(&name, &content);
}

fn generate_flatlist<'a>(setting: &GenerateSetting) -> String {
    let infos = keymapping();
    let mut spellsbylen = Vec::new();
    let t = flatlist_go(setting, &infos, &mut spellsbylen);
    println!("length: {:?}", t.iter().len_vec());
    let s = t.iter().map(|f| f.iter().reduce_lines()).reduce_lines();
    // println!("{}", s);
    s
}
fn generate_flatlist_with_hangul<'a>(
    setting: &GenerateSetting,
    hangul_trie: &'a Trie<u8>,
) -> String {
    let infos = keymapping();
    let mut spellsbylen = Vec::new();
    let t = flatlist_go_with_hangul(hangul_trie, setting, &infos, &mut spellsbylen);
    println!("length: {:?}", t.iter().len_vec());
    let s = t
        .iter()
        .map(|f| f.iter().map(|(c, h)| c.to_owned() + " " + h).reduce_lines())
        .reduce_lines();
    // println!("{}", s);
    s
}

fn gen_ahk_hangul_maps<'a>(setting: &GenerateSetting, hangul_trie: &'a Trie<u8>) -> String {
    let infos = keymapping();
    let mut spellsbylen = Vec::new();
    let t = flatlist_go_with_hangul(hangul_trie, setting, &infos, &mut spellsbylen);
    let lenlist = t.iter().len_vec();
    println!("length: {:?}", lenlist);
    let s = t
        .iter()
        .enumerate()
        .map(|(i, f)| {
            format!(
                // "hangulOut{}(spell) {{
                //   switch spell {{{}
                //     default:{}
                //   }}
                // }}
                // "
                "  Map({}
  )", // , i+1
                f.iter()
                    .map(|(c, h)|
        // format!("case \"{}\": Send \"{}\""
        format!("\"{}\", \"{}\","
        ,c ,h))
                    .reduce_delemeter("\n    ") // , ahk_default_clause(&(i as u32))
            )
        })
        .fold("".to_string(), |s, m| s + &m + ",\n");
    // println!("{}", s);
    format!("hangulMaps := [\n{}\n] ", s)
}

fn gen_ahk_hangul<'a>(setting: &GenerateSetting, hangul_trie: &'a Trie<u8>) -> String {
    let infos = keymapping();
    let mut spellsbylen = Vec::new();
    let t = flatlist_go_with_hangul(hangul_trie, setting, &infos, &mut spellsbylen);
    let lenlist = t.iter().len_vec();
    println!("length: {:?}", lenlist);
    let s = t
        .iter()
        .enumerate()
        .map(|(i, f)| {
            format!(
                "hangulOut{}(spell) {{
  switch spell {{{}
    default:{}
  }}
}}
",
                i + 1,
                f.iter()
                    .map(|(c, h)| format!("case \"{}\": Send \"{}\"", c, h))
                    .reduce_delemeter("\n    "),
                ahk_default_clause(&(i as u32))
            )
        })
        .reduce_lines();
    s
}

fn spellvecvec<'a>(
    setting: &GenerateSetting,
    hangul_trie: &'a Trie<u8>,
) -> Vec<Vec<(String, String)>> {
    let infos = keymapping();
    let mut spellsbylen = Vec::new();
    let mut t = flatlist_go_with_hangul(hangul_trie, setting, &infos, &mut spellsbylen);
    let lenlist = t.iter().len_vec();
    println!("length: {:?}", lenlist);
    t
}

fn yong_def_affixed(
    pairsbylen: Vec<Vec<(String, String)>>,
    prefix: String,
    postfix: String,
) -> String {
    pairsbylen
        .iter()
        .enumerate()
        .map(|(i, pairs)| {
            pairs
                .iter()
                .map(|(spell, hangul)| "".to_string() + &prefix + spell + " " + hangul + &postfix)
                .reduce_lines()
        })
        .reduce_lines()
}

fn ahk_default_clause(lenminus1: &u32) -> String {
    match lenminus1 {
        0 => {
            "
      Send spell"
        }
        1 => {
            "
      hangulOut1(SubStr(spell, 1, 1))
      hangulOut1(SubStr(spell, -1))"
        }
        2 => {
            "
      hangulOut2(SubStr(spell, 1, 2))
      hangulOut1(SubStr(spell, -1))"
        }
        _ => "",
    }
    .to_string()
}

fn add_hangul(spells: Vec<Vec<String>>) -> Vec<Vec<String>> {
    vec![vec!["kari".to_string()]]
}
fn traslate_in_hangul(spell: &str) -> String {
    "lsdf".to_string()
}
fn hangul_cmap() -> HashMap<char, char> {
    HashMap::from([
        ('w', 'ㅂ'),
        ('e', 'ㅋ'),
        ('r', 'ㅌ'),
        ('u', 'ㅈ'),
        ('i', 'ㄹ'),
        ('o', 'ㅊ'),
        ('s', 'ㅁ'),
        ('d', 'ㄱ'),
        ('f', 'ㄷ'),
        ('j', 'ㅇ'),
        ('k', 'ㄴ'),
        ('l', 'ㅅ'),
        ('z', 'ㅍ'),
        ('x', 'ㄲ'),
        ('c', 'ㄸ'),
        ('m', 'ㅉ'),
        ('n', 'ㅎ'),
        ('v', 'ㅆ'),
    ])
}

fn bufwrite(filename: &str, con: &str) {
    let j = fs::File::create(filename);
    println!("file: {:?}", &j);
    let mut f = BufWriter::new(j.unwrap());
    // let b = [u8::from_str(con).unwrap()];
    let b = con.as_bytes();
    let r = f.write(b);
    println!("write result: {:?}", r);
}

fn writefile(filename: &str, content: &str) {
    match fs::write(filename, content) {
        Ok(_) => println!("ok"),
        Err(e) => println!("{}", e),
    }
}

trait Doublist {
    fn len_vec(self) -> Vec<u64>;
}
impl<U> Doublist for U
where
    U: Iterator,
    <U as Iterator>::Item: IntoIterator,
{
    fn len_vec(self) -> Vec<u64> {
        self.map(|d| d.into_iter().count() as u64)
            .collect::<Vec<_>>()
    }
}

trait Lines {
    fn reduce_lines(self) -> String;
    fn reduce_delemeter(self, delemeter: &str) -> String;
}
// impl<T> Lines for T where
// T: Lines
// {
//   fn reduce_lines_rec(self) -> String {
//     self.try
//   }
// }
impl<T> Lines for T
where
    T: Iterator,
    <T as Iterator>::Item: ToString,
{
    fn reduce_lines(self) -> String {
        self.fold(String::new(), |d, f| d + &f.to_string() + "\n")
    }
    fn reduce_delemeter(self, delemeter: &str) -> String {
        self.fold(String::new(), |d, f| d + delemeter + &f.to_string())
    }
}
// impl<T,U> Lines for U where
// U: Iterator,
// <<U as Iterator>::Item as Iterator>::Item: ToString,
// {

//   fn reduce_liness(self) -> String {
//     self.fold(String::new(), |d,f| d + &f.reduce_lines())
//   }
// }

fn flatlist_go(
    setting: &GenerateSetting,
    infos: &HashMap<char, Kagi>,
    spellsbylen: &mut Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    let parents = spellsbylen.last();
    if spellsbylen.len() < setting.len as usize {
        let mut childs = match parents {
            None => {
                let spells = next_chars(None, None, &setting.rule, infos);
                vec![spells.chars().map(|d| d.to_string()).collect::<Vec<_>>()]
            }
            Some(parents) => {
                parents
                    .into_iter()
                    .map(|spell| {
                        let mut cs = spell.chars();
                        let current = cs.nth_back(0);
                        let before = cs.nth_back(0);
                        // if current == Some('d') {
                        //   println!("spell:{}, before:{:?}, current:{:?}", spell, before, current);
                        // }
                        return next_chars(before, current, &setting.rule, infos)
                            .chars()
                            .map(|c| spell.to_string() + &c.to_string())
                            .collect::<Vec<String>>();
                    })
                    .collect::<Vec<Vec<String>>>()
            }
        };
        let flat = vec_flatten(&mut childs);
        spellsbylen.push(flat);
        flatlist_go(setting, infos, spellsbylen)
    } else {
        if setting.lenonly {
            vec![parents.unwrap().to_vec()]
        } else {
            spellsbylen.to_vec()
        }
    }
}

fn flatlist_go_with_hangul<'a>(
    hangul_trie: &'a Trie<u8>,
    setting: &GenerateSetting,
    infos: &HashMap<char, Kagi>,
    spellsbylen: &mut Vec<Vec<(String, String)>>,
) -> Vec<Vec<(String, String)>> {
    let parents = spellsbylen.last();
    let nth_next = spellsbylen.len() as u32;
    if nth_next < setting.len as u32 {
        let mut childs: Vec<Vec<(String, String)>> = match parents {
            None => {
                let spells =
                    next_chars_with_hangul(hangul_trie, nth_next, None, None, &setting.rule, infos);
                vec![spells
                    .iter()
                    .map(|(c, hangul)| (c.to_string(), hangul.to_string()))
                    .collect::<Vec<_>>()]
            }
            Some(parents) => {
                parents
                    .into_iter()
                    .map(|(spell, par_hangul)| {
                        let mut cs = spell.chars();
                        let current = cs.nth_back(0);
                        let before = cs.nth_back(0);
                        // if current == Some('d') {
                        //   println!("spell:{}, before:{:?}, current:{:?}", spell, before, current);
                        // }
                        next_chars_with_hangul(
                            hangul_trie,
                            nth_next,
                            before,
                            current,
                            &setting.rule,
                            infos,
                        )
                        .iter()
                        .map(|(c, h)| (spell.to_string() + &c.to_string(), h.to_string()))
                        .collect::<Vec<(String, String)>>()
                    })
                    .collect::<Vec<Vec<(String, String)>>>()
            }
        };
        let flat = vec_flatten(&mut childs);
        spellsbylen.push(flat);
        flatlist_go_with_hangul(hangul_trie, setting, infos, spellsbylen)
    } else {
        if setting.lenonly {
            vec![parents.unwrap().to_owned()]
        } else {
            spellsbylen.to_vec()
        }
    }
}

fn chars_spacing(cs: &[char]) -> String {
    cs.iter()
        .map(|&d| d.to_string())
        .intersperse(" ".to_string())
        .collect::<String>()
}

enum CharComposition {
    Last,
    Complete,
}

fn find_hangul<'a>(
    hangul_trie: &'a Trie<u8>,
    hangul_cmap: &HashMap<char, char>,
    infos: &HashMap<char, Kagi>,
    nth_next: u32,
    before: Option<char>,
    current_info: &Kagi,
    next_info: &Kagi,
    composi: CharComposition,
) -> Option<char> {
    match before {
        Some(before) => {
            let h0 = hangul_cmap.get(&before).unwrap();
            let before_info = infos.get(&before).unwrap();
            let h1 = find_hangul(
                hangul_trie,
                hangul_cmap,
                infos,
                1,
                None,
                before_info,
                current_info,
                CharComposition::Last,
            );
            let h2 = match next_info.ch {
                'm' => 'ㄵ',
                'c' => 'ㄶ',
                _ => *hangul_cmap.get(&next_info.ch).unwrap(),
            };
            if let Some(h1) = h1 {
                let cs = [*h0, h1, h2];
                let q = chars_spacing(&cs);
                // println!("query:{}", &q);
                let found_hanguls = hangul_trie.prefix_search_trie(&q);
                // println!("founds:{:#?}", &found_hanguls);
                match findbest_from_queried(&found_hanguls) {
                    Ok(one) => {
                        let postfix = str_chushutu_unique(&q, &one);
                        // println!("postfix:{} sub:{} from:{}", &postfix, &q, &one);
                        postfix.chars().last()
                    }
                    Err(kouho) => {
                        println!("coundn't find best. kouho:{:#?}", &kouho);
                        None
                    }
                }
            } else {
                println!("h1: None");
                None
            }
        }
        None => {
            let vowel = if current_info.sayu == next_info.sayu {
                match next_info.dan {
                    Dan::Chu => match current_info.yubi {
                        Yubi::Hitosashi => match next_info.yubi {
                            Yubi::Naka => Some('ㅡ'),
                            Yubi::Kusuri => Some('ㅢ'),
                            _ => None,
                        },
                        Yubi::Naka => match next_info.yubi {
                            Yubi::Kusuri => Some('ㅢ'),
                            Yubi::Hitosashi => Some('ㅡ'),
                            _ => None,
                        },
                        Yubi::Kusuri => match next_info.yubi {
                            Yubi::Naka => Some('ㅡ'),
                            Yubi::Hitosashi => Some('ㅢ'),
                            _ => None,
                        },
                        _ => None,
                    },
                    Dan::Jo => match current_info.yubi {
                        Yubi::Hitosashi => match next_info.yubi {
                            Yubi::Naka => Some('ㅜ'),
                            Yubi::Kusuri => Some('ㅠ'),
                            _ => None,
                        },
                        Yubi::Naka => match next_info.yubi {
                            Yubi::Kusuri => Some('ㅠ'),
                            Yubi::Hitosashi => Some('ㅜ'),
                            _ => None,
                        },
                        Yubi::Kusuri => match next_info.yubi {
                            Yubi::Naka => Some('ㅜ'),
                            Yubi::Hitosashi => Some('ㅠ'),
                            _ => None,
                        },
                        _ => None,
                    },
                    Dan::Ge => match current_info.yubi {
                        Yubi::Hitosashi => match next_info.yubi {
                            Yubi::Naka => Some('ㅗ'),
                            Yubi::Kusuri => Some('ㅛ'),
                            _ => None,
                        },
                        Yubi::Naka => match next_info.yubi {
                            Yubi::Kusuri => Some('ㅛ'),
                            Yubi::Hitosashi => Some('ㅗ'),
                            _ => None,
                        },
                        Yubi::Kusuri => match next_info.yubi {
                            Yubi::Naka => Some('ㅗ'),
                            Yubi::Hitosashi => Some('ㅛ'),
                            _ => None,
                        },
                        _ => None,
                    },
                }
            } else {
                match next_info.dan {
                    Dan::Chu => match next_info.yubi {
                        Yubi::Naka => Some('ㅣ'),
                        Yubi::Kusuri => Some('ㅏ'),
                        Yubi::Hitosashi => Some('ㅓ'),
                        _ => None,
                    },
                    Dan::Jo => match next_info.yubi {
                        Yubi::Kusuri => Some('ㅑ'),
                        Yubi::Naka => Some('ㅐ'),
                        Yubi::Hitosashi => Some('ㅕ'),
                        _ => None,
                    },
                    Dan::Ge => match next_info.yubi {
                        Yubi::Hitosashi => Some('ㅔ'),
                        Yubi::Naka => Some('ㅒ'),
                        Yubi::Kusuri => Some('ㅖ'),
                        _ => None,
                    },
                }
            };
            match composi {
                CharComposition::Last => vowel,
                CharComposition::Complete => match vowel {
                    None => None,
                    Some(vowel) => {
                        let &conso = hangul_cmap.get(&current_info.ch).unwrap();
                        let cs = [conso, vowel];
                        let query = chars_spacing(&cs);
                        let found_hanguls = hangul_trie.prefix_search_trie(&query);
                        match findbest_from_queried(&found_hanguls) {
                            Ok(one) => {
                                let postfix = str_chushutu_unique(&query, &one);
                                postfix.chars().last()
                            }
                            Err(kouho) => {
                                println!("coundn't find best. kouho:{:#?}", &kouho);
                                None
                            }
                        }
                    }
                },
            }
        }
    }
}
fn transpose<T>(ll: &[T]) -> Vec<Vec<<T as IntoIterator>::Item>>
where
    T: IntoIterator + Copy,
    <T as IntoIterator>::Item: Clone,
{
    let mut store: Vec<Vec<_>> = vec![];
    for n in ll.iter() {
        for (i, d) in n.into_iter().enumerate() {
            match store.get_mut(i) {
                None => {
                    store.push(vec![d]);
                }
                Some(list) => {
                    list.push(d);
                }
            }
        }
    }
    store
}
fn str_chushutu_unique(sub: &str, from: &str) -> String {
    // let ls = transpose(&[
    //   sub.chars(), from.chars()
    // ]);
    let (left, right) = from.split_at(sub.len());
    right.to_string()
}

fn findbest_from_queried(res: &Vec<String>) -> Result<String, Vec<String>> {
    let (bests, _) = res.iter().fold((vec![], 100), |d, f| {
        let (store, saitan) = d;
        let flen = f.len() as i32;
        if saitan > flen {
            (vec![f.to_string()], flen)
        } else if saitan == flen {
            let mut store_mut = store.clone();
            store_mut.push(f.to_string());
            (store_mut, saitan)
        } else {
            (store, saitan)
        }
    });
    let last = bests.as_slice().last();
    match last {
        Some(best) => {
            if bests.len() > 1 {
                Err(bests)
            } else {
                Ok(best.to_string())
            }
        }
        _ => Err(bests),
    }
}

fn next_chars_with_hangul<'a>(
    hangul_trie: &'a Trie<u8>,
    nth_next: u32,
    before: Option<char>,
    current: Option<char>,
    rule: &Rules2,
    infos: &HashMap<char, Kagi>,
) -> Vec<(char, char)> {
    let hangul_cmap = hangul_cmap();
    let firstchars = "kdjflsierumcownxvz";
    match current {
        None => {
            firstchars
                .chars()
                .filter_map(|c| {
                    let kv = hangul_cmap.get_key_value(&c);
                    match kv {
                        Some((&j, &k)) => Some((j, k)),
                        _ => None,
                    }
                })
                // .map(|(&j, &k)| (j,k))
                .collect::<Vec<(char, char)>>()
        }
        Some(current) => {
            let mut scored: Vec<((char, Option<char>), u32)> = infos
                .iter()
                .filter_map(|(k, v)| {
                    let info = infos.get(&current).unwrap();
                    let isok = rule2(rule, v, info, &before, infos);
                    if isok {
                        fn yubipoint(d: &Yubi) -> u32 {
                            match d {
                                Yubi::Naka => 0,
                                Yubi::Hitosashi => 1,
                                Yubi::Kusuri => 2,
                                _ => 3,
                            }
                        }
                        let mut basescore = match (&v.dan, &v.yubi) {
                            (Dan::Chu, k) => yubipoint(k) * 10,
                            (Dan::Jo, Yubi::Naka) => 100,
                            (Dan::Jo, Yubi::Hitosashi) => {
                                if v.sayu == Sayu::Left {
                                    300
                                } else {
                                    450
                                }
                            }
                            (Dan::Jo, _) => 400,
                            (Dan::Ge, k) => match k {
                                Yubi::Hitosashi => 200,
                                Yubi::Naka => 500,
                                Yubi::Kusuri => 600,
                                _ => 1000,
                            },
                        };
                        basescore += if v.sayu == info.sayu {
                            match (&v.dan, &v.yubi) {
                                (Dan::Jo, Yubi::Kusuri) => 11,
                                _ => 1,
                            }
                        } else {
                            0
                        };

                        let hangul = find_hangul(
                            &hangul_trie,
                            &hangul_cmap,
                            infos,
                            nth_next,
                            before,
                            info,
                            v,
                            CharComposition::Complete,
                        );

                        if let None = hangul {
                            println!("hangul:None, spell:{:?}{}{}", before, current, k);
                        }
                        Some(((*k, hangul), basescore))
                    } else {
                        None
                    }
                })
                .collect::<Vec<((char, Option<char>), u32)>>();

            scored.sort_by(|(_, sj), (_, sk)| sj.partial_cmp(sk).unwrap());
            let s: Vec<(char, char)> = scored
                .iter()
                .filter_map(|(c, _)| {
                    if let Some(hangul) = c.1 {
                        Some((c.0, hangul))
                    } else {
                        None
                    }
                })
                .collect();
            return s;
        }
    }
}

fn next_chars(
    before: Option<char>,
    current: Option<char>,
    rule: &Rules2,
    infos: &HashMap<char, Kagi>,
) -> String {
    let firstchars = "kdjflsierumcownxvz".to_string();
    match current {
        None => firstchars,
        Some(current) => {
            let mut scored: Vec<(char, u32)> = infos
                .iter()
                .filter_map(|(k, v)| {
                    let info = infos.get(&current).unwrap();
                    let isok = rule2(rule, v, info, &before, infos);
                    let res: Option<(char, u32)> = if isok {
                        fn yubipoint(d: &Yubi) -> u32 {
                            match d {
                                Yubi::Naka => 0,
                                Yubi::Hitosashi => 1,
                                Yubi::Kusuri => 2,
                                _ => 3,
                            }
                        }
                        let mut basescore = match (&v.dan, &v.yubi) {
                            (Dan::Chu, k) => yubipoint(k) * 10,
                            (Dan::Jo, Yubi::Naka) => 100,
                            (Dan::Jo, Yubi::Hitosashi) => {
                                if v.sayu == Sayu::Left {
                                    300
                                } else {
                                    450
                                }
                            }
                            (Dan::Jo, _) => 400,
                            (Dan::Ge, k) => match k {
                                Yubi::Hitosashi => 200,
                                Yubi::Naka => 500,
                                Yubi::Kusuri => 600,
                                _ => 1000,
                            },
                        };
                        basescore += if v.sayu == info.sayu {
                            match (&v.dan, &v.yubi) {
                                (Dan::Jo, Yubi::Kusuri) => 11,
                                _ => 1,
                            }
                        } else {
                            0
                        };

                        // basescore += match before {
                        //   None => 0,
                        //   Some(c) => {
                        //     let ci = infos.get(&c).unwrap();
                        //     if ci.sayu == v.sayu {
                        //       1
                        //     } else {0}
                        //   }
                        // };

                        Some((*k, basescore))
                    } else {
                        None
                    };

                    return res;
                })
                .collect::<Vec<(char, u32)>>();

            scored.sort_by(|(_, sj), (_, sk)| sj.partial_cmp(sk).unwrap());
            let s = scored.iter().map(|(c, _)| *c).collect::<String>();
            return s;
        }
    }
}

fn open_file(filename: &str) -> String {
    let data = fs::read_to_string(filename);
    let data = match data {
        Ok(content) => content,
        Err(error) => {
            panic!("Could not open or find file: {}", error);
        }
    };
    return data;
}

fn vec_flatten<T>(v: &mut Vec<Vec<T>>) -> Vec<T>
where
    T: Clone,
{
    v.iter().fold(vec![], |d, f| {
        let mut m = vec![];
        m.extend_from_slice(d.as_slice());
        m.extend_from_slice(f.as_slice());
        m
    })
}
