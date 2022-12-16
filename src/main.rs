use std::fs;
use slab_tree::*;
use std::convert::TryInto;
use std::collections::HashMap;

fn main() {
  let setting = GenrateSetting { len: 3, lenonly: false, table: Table::Nv };
  spell_list(setting, "spelltable.txt")
}

fn tree() -> Tree<Y> {
    let mut tree = TreeBuilder::new().with_root(newY()).build();
    let mut root = tree.root_mut().unwrap();
    root.append(newY())
            .append(newY());
    let mut j = root.append(newY());
    j.append(newY()).append(newY());
    j.append(newY());
    return tree;

}

#[derive(Debug, PartialEq)]
struct Y {
    spell: String,
    depth: u32,
    chlen: u32,
    desclen: u32,
}

fn newY() -> Y {
    Y {
        spell: String::new(),
        depth: 0,
        chlen: 0,
        desclen: 0,
    }
}

fn descendant(n: NodeMut<Y>, tree: Tree<Y>) -> u32 {
    let ch = n.as_ref().children();
    match ch.next() {
        None => 0,
        Some(x) => {
            let mut sum = 0;
            for child in ch {
                sum += descendant(tree.get_mut(child.node_id()).unwrap(), tree);
            }
            n.data().chlen = ch.count().try_into().unwrap();
            n.data().desclen = sum;
            return sum+1;
        }
    }
}

fn open_file(filename: &str) -> String {
    let data = fs::read_to_string(filename);
    let data = match data {
        Ok(content) => content,
        Err(error) => {panic!("Could not open or find file: {}", error);}
    };
    return data;
}

fn keymapping() -> HashMap<char,Kagi> {
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

struct Kagi {
  ch: char,
  yubi: Yubi,
  dan: Dan,
  sayu: Sayu
}
enum Sayu {
  Right,
  Left
}
enum Yubi {
  Oya,
  Hitosashi,
  Naka,
  Kusuri,
  Ko
}
enum Dan {
  Jo,
  Chu,
  Ge
}

enum Table {
  Nv,
  Normal
}
struct GenrateSetting {
  len: u8,
  lenonly: bool,
  table: Table
}
fn spell_list(setting: GenrateSetting, filename: &str) {
  let keymap = keymapping();
  writefile(filename, "Hello, World!");
}

fn writefile(filename: &str, content: &str) -> std::io::Result<()> {
  fs::write(filename, content)?;
  Ok(())
}





