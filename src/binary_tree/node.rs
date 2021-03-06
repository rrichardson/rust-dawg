// Copyright (c) 2015 Takeru Ohta <phjgt308@gmail.com>
//
// This software is released under the MIT License,
// see the LICENSE file at the top-level directory.

use std::rc::Rc;
use std::cmp::PartialEq;
use std::hash::Hash;
use std::hash::Hasher;
use WordId;
use Char;

#[derive(Eq,Clone)]
pub struct Node {
    pub ch: Char,
    pub is_terminal: bool,
    pub child: Option<Rc<Node>>,
    pub sibling: Option<Rc<Node>>,
    child_total: u32,
    sibling_total: u32,
}

pub type NodeAddr = *const Node;

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        (self.child.as_ref().map(|n| n.addr()) == other.child.as_ref().map(|n| n.addr()) &&
         self.sibling.as_ref().map(|n| n.addr()) == other.sibling.as_ref().map(|n| n.addr()) &&
         self.ch == other.ch && self.is_terminal == other.is_terminal)
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ch.hash(state);
        self.is_terminal.hash(state);
        self.child.as_ref().map(|n| n.addr()).hash(state);
        self.sibling.as_ref().map(|n| n.addr()).hash(state);
    }
}

impl Node {
    pub fn new(ch: Char) -> Self {
        Node {
            ch: ch,
            is_terminal: false,
            child: None,
            sibling: None,
            child_total: 0,
            sibling_total: 0,
        }
    }

    pub fn len(&self) -> usize {
        (self.is_terminal as u32 + self.child_total + self.sibling_total) as usize
    }

    pub fn id_offset(&self) -> WordId {
        self.sibling_total
    }

    pub fn fix(&mut self) {
        self.child_total = self.child.as_ref().map_or(0, |n| n.len() as u32);
        self.sibling_total = self.sibling.as_ref().map_or(0, |n| n.len() as u32);
    }

    pub fn children(&self) -> Children {
        Children { curr: self.child.clone() }
    }

    pub fn take_children(&mut self) -> Children {
        Children { curr: self.child.take() }
    }

    pub fn addr(&self) -> NodeAddr {
        self as NodeAddr
    }
}

#[derive(Clone)]
pub struct Children {
    curr: Option<Rc<Node>>,
}

impl Iterator for Children {
    type Item = Rc<Node>;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|mut child| {
            self.curr = Rc::get_mut(&mut child)
                            .map(|c| c.sibling.take())
                            .unwrap_or_else(|| child.sibling.clone());
            child
        })
    }
}
