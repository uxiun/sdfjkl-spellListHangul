use std::collections::HashMap;

pub enum Rules2 {
  Fluent,
  WnLittleFluent
}


pub fn rule2(r: &Rules2, next: &Kagi, pre: &Kagi, prre: &Option<char>, infos: &HashMap<char, Kagi>) -> bool {
    match r {
      Rules2::Fluent => match prre {
          None => rule_pre(next, pre)
          ,Some(prre) =>
            rule_fluent(next, pre, infos.get(prre).unwrap())
      }
      ,Rules2::WnLittleFluent => match prre {
          None => rule_pre(next, pre)
          ,Some(prre)=>
            rule_wn_little_fluent(next, pre, infos.get(prre).unwrap())
      }
    }
}

pub fn rule_pre(next: &Kagi, pre: &Kagi)-> bool {
  !fu_yubi_pre(next, pre)
    && !yue_dan_pre(next, pre)
}

pub fn rule_wn_little_fluent(next: &Kagi, pre: &Kagi, prre: &Kagi) -> bool {
  !fu_yubi_prre(next, pre, prre)
  && !yue_dan_pre(next, pre)
}

pub fn rule_fluent(next: &Kagi, pre: &Kagi, prre: &Kagi) -> bool {
  !fu_yubi_prre(next, pre, prre)
  && !yue_dan_prre(next, pre, prre)
}

pub fn fu_yubi_prre(next: &Kagi, pre: &Kagi, prre: &Kagi) -> bool {
  fu_yubi_pre(next, pre)
  || fu_yubi_pre(next, prre)
  || fu_yubi_pre(pre, prre)
}

pub fn fu_yubi_pre(next: &Kagi, pre: &Kagi) -> bool {
  next.sayu == pre.sayu && next.yubi == pre.yubi
}

pub fn yue_dan_prre(next: &Kagi, pre: &Kagi, prre: &Kagi) -> bool {
  yue_dan_pre(next, pre)
  || yue_dan_pre(next, prre)
  || yue_dan_pre(pre, prre)
}

pub fn yue_dan_pre(next: &Kagi, pre: &Kagi) -> bool {
  next.sayu == pre.sayu &&
  dan_to_num(&next.dan).abs_diff(dan_to_num(&pre.dan)) > 1
}

fn dan_to_num(d: &Dan) -> u8 {
  match &d {
    Dan::Chu => 1,
    Dan::Ge => 0,
    Dan::Jo => 2
  }
}

pub fn keymapping() -> HashMap<char,Kagi> {
  HashMap::from([
    ('w', Kagi {
      ch: 'w',
      yubi: Yubi::Kusuri,
      dan: Dan::Jo,
      sayu: Sayu::Left
    }),
    ('e', Kagi {
      ch: 'e',
      yubi: Yubi::Naka,
      dan: Dan::Jo,
      sayu: Sayu::Left,
    }),
    ('r', Kagi {
      ch: 'r',
      yubi: Yubi::Hitosashi,
      dan: Dan::Jo,
      sayu: Sayu::Left,
    }),
    ('s', Kagi {
      ch: 's',
      yubi: Yubi::Kusuri,
      dan: Dan::Chu,
      sayu: Sayu::Left,
    }),
    ('d', Kagi {
      ch: 'd',
      yubi: Yubi::Naka,
      dan: Dan::Chu,
      sayu: Sayu::Left,
    }),
    ('f', Kagi {
      ch: 'f',
      yubi: Yubi::Hitosashi,
      dan: Dan::Chu,
      sayu: Sayu::Left,
    }),
    ('z', Kagi {
      ch: 'z',
      yubi: Yubi::Kusuri,
      dan: Dan::Ge,
      sayu: Sayu::Left,
    }),
    ('x', Kagi {
      ch: 'x',
      yubi: Yubi::Naka,
      dan: Dan::Ge,
      sayu: Sayu::Left,
    }),
    ('c', Kagi {
      ch: 'c',
      yubi: Yubi::Hitosashi,
      dan: Dan::Ge,
      sayu: Sayu::Left,
    }),
    ('u', Kagi {
      ch: 'u',
      yubi: Yubi::Hitosashi,
      dan: Dan::Jo,
      sayu: Sayu::Right,
    }),
    ('i', Kagi {
      ch: 'i',
      yubi: Yubi::Naka,
      dan: Dan::Jo,
      sayu: Sayu::Right,
    }),
    ('o', Kagi {
      ch: 'o',
      yubi: Yubi::Kusuri,
      dan: Dan::Jo,
      sayu: Sayu::Right,
    }),
    ('j', Kagi {
      ch: 'j',
      yubi: Yubi::Hitosashi,
      dan: Dan::Chu,
      sayu: Sayu::Right,
    }),
    ('k', Kagi {
      ch: 'k',
      yubi: Yubi::Naka,
      dan: Dan::Chu,
      sayu: Sayu::Right,
    }),
    ('l', Kagi {
      ch: 'l',
      yubi: Yubi::Kusuri,
      dan: Dan::Chu,
      sayu: Sayu::Right,
    }),
    ('m', Kagi {
      ch: 'm',
      yubi: Yubi::Hitosashi,
      dan: Dan::Ge,
      sayu: Sayu::Right,
    }),
    ('n', Kagi {
      ch: 'n',
      yubi: Yubi::Naka,
      dan: Dan::Ge,
      sayu: Sayu::Right,
    }),
    ('v', Kagi {
      ch: 'v',
      yubi: Yubi::Kusuri,
      dan: Dan::Ge,
      sayu: Sayu::Right,
    }),
  ])
}

pub struct Kagi {
  pub ch: char,
  pub yubi: Yubi,
  pub dan: Dan,
  pub sayu: Sayu
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Sayu {
  Right,
  Left
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Yubi {
  Oya,
  Hitosashi,
  Naka,
  Kusuri,
  Ko
}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Dan {
  Jo,
  Chu,
  Ge
}

pub enum Table {
  Nv,
  Normal
}
pub struct GenerateSetting {
  pub len: u8,
  pub lenonly: bool,
  pub table: Table,
  pub rule: Rules2
}
