use crate::compile::CompileError;
use std::iter::Peekable;
use std::str::Chars;

use super::position::{CompileErrorWithPos, ParsePosition, PosCompileError};

pub fn nummeric_checker(s: char) -> bool {
    matches!(s, '0'..='9' | '+' | '-')
}

pub fn letter_checker(s: char) -> bool {
    matches!(s, 'a'..='z' | 'A'..='Z' | '$')
}

pub fn end_checker(s: char) -> bool {
    matches!(s, ';' | '#' | '\n' | '(')
}

pub struct ParseReader<'a> {
    pub inp: Peekable<&'a mut dyn Iterator<Item = char>>,
    pub pos: ParsePosition,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum BlockType {
    String,
    CurlyBracket,
    Parentheses,
}

impl BlockType {
    fn from_start_char(c: char) -> Option<BlockType> {
        match c {
            '\"' => Some(BlockType::String),
            '(' => Some(BlockType::Parentheses),
            '{' => Some(BlockType::CurlyBracket),
            _ => None,
        }
    }

    fn from_end_char(c: char) -> Option<BlockType> {
        match c {
            '\"' => Some(BlockType::String),
            ')' => Some(BlockType::Parentheses),
            '}' => Some(BlockType::CurlyBracket),
            _ => None,
        }
    }
}

impl<'a> ParseReader<'a> {
    pub fn is_empty(&mut self) -> bool {
        self.inp.peek().is_none()
    }

    pub fn peek(&mut self) -> Result<char, PosCompileError> {
        match self.inp.peek() {
            Some(c) => Ok(*c),
            None => Err(CompileError::UnexpectedEndOfFile.with_pos(self.pos)),
        }
    }

    pub fn take(&mut self) -> Result<char, PosCompileError> {
        match self.inp.next() {
            Some(c) => {
                self.pos.next(c);
                Ok(c)
            }
            None => Err(CompileError::UnexpectedEndOfFile.with_pos(self.pos)),
        }
    }

    pub fn read_until(
        &mut self,
        until: fn(char, char) -> bool,
    ) -> Result<Vec<char>, PosCompileError> {
        let mut collected: Vec<char> = Vec::new();
        let mut prev: char = '\0';

        while !self.is_empty() {
            let cur = self.peek()?;

            if until(cur, prev) {
                break;
            }

            prev = cur;

            collected.push(cur);
            self.take()?;
        }

        Ok(collected)
    }

    pub fn take_block(&mut self) -> Result<Vec<char>, PosCompileError> {
        let mut res = Vec::new();
        let mut block_stack: Vec<BlockType> = Vec::new();
        let first_char = self.take()?;
        let mut top_token = BlockType::from_start_char(first_char)
            .ok_or(CompileError::UnknownToken(first_char).with_pos(self.pos))?;
        block_stack.push(top_token);

        while !block_stack.is_empty() {
            let cur_char = self.take()?;
            res.push(cur_char);

            if top_token == BlockType::String {
                if cur_char == '\\' {
                    res.push(self.take()?);
                } else if cur_char == '\"' {
                    block_stack.pop();
                    top_token = *block_stack.last().unwrap_or(&BlockType::Parentheses);
                }
            } else if let Some(new_block) = BlockType::from_start_char(cur_char) {
                top_token = new_block;
                block_stack.push(new_block)
            } else if let Some(close_block) = BlockType::from_end_char(cur_char) {
                if top_token != close_block {
                    return Err(CompileError::UnexpectedToken(cur_char).with_pos(self.pos));
                }
                block_stack.pop();
                top_token = *block_stack.last().unwrap_or(&BlockType::Parentheses);
            }
        }

        res.pop();
        Ok(res)
    }
}

impl<'a> From<&'a mut Chars<'a>> for ParseReader<'a> {
    fn from(c: &'a mut Chars<'a>) -> Self {
        let iter_ref: &mut dyn std::iter::Iterator<Item = char> = c;
        // dbg!(iter_ref.)
        let p: ParseReader = ParseReader {
            inp: iter_ref.peekable(),
            pos: ParsePosition::default(),
        };
        
        p
    }
}

impl<'a> From<&'a mut std::vec::IntoIter<char>> for ParseReader<'a> {
    fn from(c: &'a mut std::vec::IntoIter<char>) -> Self {
        let iter_ref: &mut dyn std::iter::Iterator<Item = char> = c;

        let p: ParseReader = ParseReader {
            inp: iter_ref.peekable(),
            pos: ParsePosition::default(),
        };

        p
    }
}
