use std::{borrow::Cow, collections::hash_map::Entry};

use rand::seq::SliceRandom;
use tree_sitter::{InputEdit, Node, Parser, Point, QueryCursor, QueryMatch, Tree};

use crate::obfuscated::ObfuscatedError;

use super::{ObfuscatorLanguage, Result};

pub struct QuestionAnswerQueryName<'query> {
    pub raw_capture_name: &'query str,
    pub capture_name: &'query str,
    pub weight: Option<u32>,
}

impl<'query> QuestionAnswerQueryName<'query> {
    pub fn precompute(raw_capture_name: &'query str) -> Result<QuestionAnswerQueryName<'query>> {
        let mut question_iter = raw_capture_name.split('!');
        let capture_name = question_iter
            .next()
            .ok_or(ObfuscatedError::CaptureNameEmpty)?;

        let weight = question_iter
            .next()
            .map(|weight| weight.parse())
            .transpose()
            .map_err(ObfuscatedError::WeightParseFailed)?;

        if question_iter.next().is_some() {
            return Err(ObfuscatedError::CaptureNameTooManySperators);
        }

        Ok(Self {
            raw_capture_name,
            capture_name,
            weight,
        })
    }
}

pub struct ObfuscatorQueryName<'query> {
    pub raw_capture_name: &'query str,
    pub capture_name: &'query str,
    pub chance: Option<f64>,
    pub obfuscate: for<'word> fn(&ObfuscatorLanguage, &[&'word str]) -> Result<Cow<'word, [u8]>>,
}

pub fn randomize<'word>(
    _language: &ObfuscatorLanguage,
    wordlist: &[&'word str],
) -> Result<Cow<'word, [u8]>> {
    Ok(Cow::Borrowed(
        wordlist
            .choose(&mut rand::thread_rng())
            .ok_or(ObfuscatedError::WordlistEmpty)?
            .as_bytes(),
    ))
}

// TODO: support other language comments
pub fn randomize_line_comment<'word>(
    language: &ObfuscatorLanguage,
    wordlist: &[&'word str],
) -> Result<Cow<'word, [u8]>> {
    Ok(Cow::Owned(
        format!(
            "{}{}",
            language.get_line_comment_prefix(),
            wordlist
                .choose(&mut rand::thread_rng())
                .ok_or(ObfuscatedError::WordlistEmpty)?
        )
        .into_bytes(),
    ))
}

pub fn randomize_block_comment<'word>(
    language: &ObfuscatorLanguage,
    wordlist: &[&'word str],
) -> Result<Cow<'word, [u8]>> {
    Ok(Cow::Owned(
        format!(
            "{}{}{}",
            language.get_block_comment_prefix(),
            wordlist
                .choose(&mut rand::thread_rng())
                .ok_or(ObfuscatedError::WordlistEmpty)?,
            language.get_block_comment_suffix()
        )
        .into_bytes(),
    ))
}

impl<'query> ObfuscatorQueryName<'query> {
    pub fn precompute(raw_capture_name: &'query str) -> Result<Self> {
        let mut obfuscator_iter = raw_capture_name.split('!');
        let capture_name = obfuscator_iter
            .next()
            .ok_or(ObfuscatedError::CaptureNameEmpty)?;

        let chance = obfuscator_iter
            .next()
            .map(|weight| weight.parse())
            .transpose()
            .map_err(ObfuscatedError::ChanceParseFailed)?;

        if obfuscator_iter.next().is_some() {
            return Err(ObfuscatedError::CaptureNameTooManySperators);
        }

        let obfuscate = match capture_name {
            "randomize" => randomize,
            "randomize_line_comment" => randomize_line_comment,
            "randomize_block_comment" => randomize_block_comment,
            _ => return Err(ObfuscatedError::ObfuscatorNameNotFound(capture_name.to_string())),
        };

        Ok(Self {
            raw_capture_name,
            capture_name,
            chance,
            obfuscate,
        })
    }
}

pub struct QuestionMatch<'tree, 'query> {
    pub question: Node<'tree>,
    pub question_index: u32,
    pub question_capture_name: &'query QuestionAnswerQueryName<'query>,
    pub pattern_index: usize,
    pub weight: u32,
}

impl<'tree, 'query> QuestionMatch<'tree, 'query> {
    pub fn to_question(
        query_match: QueryMatch<'_, 'tree>,
        precomputed_question_names: &'query [QuestionAnswerQueryName],
    ) -> Result<Self> {
        let question = query_match
            .captures
            .get(0)
            .ok_or(ObfuscatedError::FirstCaptureDoesNotExist)?;

        if precomputed_question_names[question.index as usize].capture_name != "question" {
            return Err(ObfuscatedError::FirstCaptureNotAQuestion);
        }

        Ok(Self {
            question: question.node,
            question_index: question.index,
            question_capture_name: &precomputed_question_names[question.index as usize],
            pattern_index: query_match.pattern_index,
            weight: precomputed_question_names[question.index as usize]
                .weight
                .unwrap_or(1),
        })
    }
}

pub struct AnswerMatch<'tree, 'query> {
    pub answer: Node<'tree>,
    pub answer_index: u32,
    pub answer_capture_name: &'query QuestionAnswerQueryName<'query>,
    pub pattern_index: usize,
}

impl<'tree, 'query> AnswerMatch<'tree, 'query> {
    pub fn to_answer(
        query_match: QueryMatch<'_, 'tree>,
        precomputed_question_names: &'query [QuestionAnswerQueryName],
    ) -> Result<Self> {
        let answer = query_match
            .captures
            .get(0)
            .ok_or(ObfuscatedError::FirstCaptureDoesNotExist)?;

        if precomputed_question_names[answer.index as usize].capture_name != "answer" {
            return Err(ObfuscatedError::FirstCaptureNotAnAnswer)
        }

        Ok(Self {
            answer: answer.node,
            answer_index: answer.index,
            answer_capture_name: &precomputed_question_names[answer.index as usize],
            pattern_index: query_match.pattern_index,
        })
    }
}

pub struct ObfuscatorMatch<'tree, 'query> {
    pub node: Node<'tree>,
    pub node_index: u32,
    pub node_capture_name: &'query ObfuscatorQueryName<'query>,
    pub pattern_index: usize,
    pub chance: f64,
}

impl<'tree, 'query> ObfuscatorMatch<'tree, 'query> {
    pub fn to_obfuscator(
        query_match: QueryMatch<'_, 'tree>,
        capture_index: usize,
        precomputed_obfuscator_names: &'query [ObfuscatorQueryName],
    ) -> Result<ObfuscatorMatch<'tree, 'query>> {
        let obfuscator = query_match.captures[capture_index];

        Ok(Self {
            node: obfuscator.node,
            node_index: obfuscator.index,
            node_capture_name: &precomputed_obfuscator_names[obfuscator.index as usize],
            pattern_index: query_match.pattern_index,
            chance: precomputed_obfuscator_names[obfuscator.index as usize]
                .chance
                .unwrap_or(1.0),
        })
    }
}

pub fn create_edit_for_node(node: Node, tree_text: &[u8], new_text: &[u8]) -> Result<InputEdit> {
    // TODO: properly update position if new_text contains newlines and maybe if it contains non-ascii
    let new_end_position = Point::new(
        node.start_position().row,
        node.start_position().column + new_text.len(),
    );

    if new_text == &tree_text[node.byte_range()] {
        return Err(ObfuscatedError::CannotReeditToSameText)
    }

    Ok(InputEdit {
        start_byte: node.start_byte(),
        old_end_byte: node.end_byte(),
        new_end_byte: node.start_byte() + new_text.len(),
        start_position: node.start_position(),
        old_end_position: node.end_position(),
        new_end_position: new_end_position,
    })
}

pub fn edit_tree(
    parser: &mut Parser,
    tree: &mut Tree,
    tree_text: &mut Vec<u8>,
    node_byte_range: std::ops::Range<usize>,
    edit: &InputEdit,
    new_text: &[u8],
) -> Result<()> {
    tree_text.splice(node_byte_range, new_text.iter().cloned());

    tree.edit(&edit);

    *tree = parser
        .parse(&tree_text, Some(&tree))
        .ok_or(ObfuscatedError::ParsingFailed)?;

    Ok(())
}

pub fn shift_cursor(tree: &Tree, cursor: &mut QueryCursor, edit: &InputEdit) {
    cursor.set_byte_range(std::ops::Range {
        start: edit.new_end_byte,
        end: tree.root_node().end_byte(),
    });
}

pub trait OrInsertFailable<'a, K, V> {
    fn or_insert_with_failable<F: FnOnce() -> Result<V>>(self, default: F) -> Result<&'a mut V>;
}

impl<'a, K, V> OrInsertFailable<'a, K, V> for Entry<'a, K, V> {
    fn or_insert_with_failable<F: FnOnce() -> Result<V>>(self, default: F) -> Result<&'a mut V> {
        match self {
            std::collections::hash_map::Entry::Occupied(entry) => Ok(entry.into_mut()),
            std::collections::hash_map::Entry::Vacant(entry) => Ok(entry.insert(default()?)),
        }
    }
}
