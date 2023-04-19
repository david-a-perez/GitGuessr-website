use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
    num::{ParseFloatError, ParseIntError},
    str::{FromStr, Utf8Error},
    string::FromUtf8Error,
};

use rand::{distributions::WeightedError, seq::SliceRandom, Rng};
use tree_sitter::{Language, Parser, Query, QueryCursor};

use obfuscator::{
    create_edit_for_node, edit_tree, shift_cursor, AnswerMatch, ObfuscatorMatch,
    ObfuscatorQueryName, OrInsertFailable, QuestionAnswerQueryName, QuestionMatch,
};

use wordlist::WORDLIST;

mod obfuscator;
mod wordlist;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObfuscatedError {
    #[error("Language not supported for Obfuscated")]
    LanguageNotSupported,

    #[error("Parsing code failed")]
    ParsingFailed,

    #[error("Tree-Sitter Langauge Error: {0}")]
    TreeSitterLanguageError(#[from] tree_sitter::LanguageError),

    #[error("Tree-Sitter Query Error: {0}")]
    TreeSitterQueryError(#[from] tree_sitter::QueryError),

    #[error("Capture name cannot be empty")]
    CaptureNameEmpty,

    #[error("Capture name has too many seperators")]
    CaptureNameTooManySperators,

    #[error("Wordlist is empty")]
    WordlistEmpty,

    #[error("Cannot parse question weight: {0}")]
    WeightParseFailed(ParseIntError),

    #[error("Cannot find obfuscator name: {0}")]
    ObfuscatorNameNotFound(String),

    #[error("Cannot parse obfuscator chance: {0}")]
    ChanceParseFailed(ParseFloatError),

    #[error("First capture does not exist")]
    FirstCaptureDoesNotExist,

    #[error("First capture is not a question")]
    FirstCaptureNotAQuestion,

    #[error("First capture is not an answer")]
    FirstCaptureNotAnAnswer,

    #[error("Cannot re-edit to the same text")]
    CannotReeditToSameText,

    #[error("Weighted Error: {0}")]
    WeightedError(#[from] WeightedError),

    #[error("Answer not found")]
    AnswerNotFound,

    #[error("Utf8Error: {0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] FromUtf8Error),
}

type Result<T> = std::result::Result<T, ObfuscatedError>;

pub enum ObfuscatorLanguage {
    Python,
    Rust,
}

impl FromStr for ObfuscatorLanguage {
    type Err = ObfuscatedError;

    fn from_str(language: &str) -> std::result::Result<Self, Self::Err> {
        match language {
            "python" => Ok(ObfuscatorLanguage::Python),
            "rust" => Ok(ObfuscatorLanguage::Rust),
            _ => Err(ObfuscatedError::LanguageNotSupported),
        }
    }
}

impl ObfuscatorLanguage {
    pub fn get_language(&self) -> Language {
        match self {
            ObfuscatorLanguage::Python => tree_sitter_python::language(),
            ObfuscatorLanguage::Rust => tree_sitter_rust::language(),
        }
    }

    pub fn get_questions(&self) -> &'static str {
        match self {
            ObfuscatorLanguage::Python => include_str!("questions-python.scm"),
            ObfuscatorLanguage::Rust => include_str!("questions-rust.scm"),
        }
    }

    pub fn get_obfuscators(&self) -> &'static str {
        match self {
            ObfuscatorLanguage::Python => include_str!("obfuscators-python.scm"),
            ObfuscatorLanguage::Rust => include_str!("obfuscators-rust.scm"),
        }
    }

    pub fn get_line_comment_prefix(&self) -> &'static str {
        match self {
            ObfuscatorLanguage::Python => "# ",
            ObfuscatorLanguage::Rust => "// ",
        }
    }

    pub fn get_block_comment_prefix(&self) -> &'static str {
        match self {
            ObfuscatorLanguage::Python => "\"\"\"",
            ObfuscatorLanguage::Rust => "/* ",
        }
    }

    pub fn get_block_comment_suffix(&self) -> &'static str {
        match self {
            ObfuscatorLanguage::Python => "\"\"\"",
            ObfuscatorLanguage::Rust => " */",
        }
    }
}

pub struct ObfuscatedQuestionData {
    pub text: String,
    pub answer: String,
}

pub fn obfuscate(language: &str, text: &[u8], num: usize) -> Result<Vec<ObfuscatedQuestionData>> {
    let language = language.parse::<ObfuscatorLanguage>()?;

    let mut parser = Parser::new();
    parser.set_language(language.get_language())?;

    let tree = parser
        .parse(&text, None)
        .ok_or(ObfuscatedError::ParsingFailed)?;

    let root_node = tree.root_node();

    let mut question_query = Query::new(language.get_language(), language.get_questions())?;
    question_query.disable_capture("answer");

    let obfuscators_query = Query::new(language.get_language(), language.get_obfuscators())?;

    let mut cursor = QueryCursor::new();

    let precomputed_question_answer_names = question_query
        .capture_names()
        .iter()
        .map(|question_name| QuestionAnswerQueryName::precompute(question_name))
        .collect::<Result<Vec<_>>>()?;

    let precomputed_obfuscator_names = obfuscators_query
        .capture_names()
        .iter()
        .map(|obfuscator_name| ObfuscatorQueryName::precompute(obfuscator_name))
        .collect::<Result<Vec<_>>>()?;

    let questions = cursor
        .matches(&question_query, root_node, text)
        .map(|query_match| {
            QuestionMatch::to_question(query_match, &precomputed_question_answer_names)
        })
        .collect::<Result<Vec<_>>>()?;

    let selected_questions: Vec<_> = questions
        .choose_multiple_weighted(&mut rand::thread_rng(), num * 2, |question_match| {
            question_match.weight
        })?
        .collect();

    let mut used_answers = HashSet::new();

    Ok(selected_questions
        .into_iter()
        .map(|query_match| -> Result<ObfuscatedQuestionData> {
            let node = query_match.question;

            let mut tree_text = Vec::new();

            if node.end_position().row > node.start_position().row {
                tree_text.resize(node.start_position().column, b' ');
            }
            tree_text.extend_from_slice(&text[node.byte_range()]);

            let mut tree = parser
                .parse(tree_text.as_slice(), None)
                .ok_or(ObfuscatedError::ParsingFailed)?;

            let mut hash_map = HashMap::new();

            let mut cursor = QueryCursor::new();

            let mut temp_query = Query::new(language.get_language(), language.get_questions())?;

            for index in 0..temp_query.pattern_count() {
                if index != query_match.pattern_index {
                    temp_query.disable_pattern(index);
                }
            }
            temp_query.disable_capture(query_match.question_capture_name.raw_capture_name);

            let answers = cursor
                .matches(&temp_query, tree.root_node(), tree_text.as_slice())
                .map(|query_match| {
                    AnswerMatch::to_answer(query_match, &precomputed_question_answer_names)
                })
                .collect::<Result<Vec<_>>>()?;

            let answer = answers
                .choose(&mut rand::thread_rng())
                .ok_or(ObfuscatedError::AnswerNotFound)?;

            let answer = {
                let node = answer.answer;
                let node_byte_range = node.byte_range();

                let answer = &tree_text[node_byte_range.clone()];
                let answer_string = std::str::from_utf8(answer)?.to_string();

                let new_text = "ANSWER".as_bytes();
                hash_map.insert(answer.to_vec(), Some(Cow::Borrowed(new_text)));
                hash_map.insert(new_text.to_vec(), None);

                let edit = create_edit_for_node(node, &tree_text, new_text)?;
                edit_tree(
                    &mut parser,
                    &mut tree,
                    &mut tree_text,
                    node_byte_range.clone(),
                    &edit,
                    new_text,
                )?;
                answer_string
            };

            'matches: loop {
                let mut captures =
                    cursor.captures(&obfuscators_query, tree.root_node(), tree_text.as_slice());
                while let Some((query_match, capture_index)) = captures.next() {
                    let obfuscator_match = ObfuscatorMatch::to_obfuscator(
                        query_match,
                        capture_index,
                        &precomputed_obfuscator_names,
                    )?;

                    let new_text = hash_map
                        .entry(tree_text[obfuscator_match.node.byte_range()].to_vec())
                        .or_insert_with_failable(|| {
                            if rand::thread_rng().gen_bool(obfuscator_match.chance) {
                                Ok(Some((obfuscator_match.node_capture_name.obfuscate)(
                                    &language, WORDLIST,
                                )?))
                            } else {
                                Ok(None)
                            }
                        })?;

                    if let Some(new_text) = new_text {
                        let node = obfuscator_match.node;
                        let node_byte_range = node.byte_range();

                        let edit = create_edit_for_node(node, &tree_text, new_text)?;
                        edit_tree(
                            &mut parser,
                            &mut tree,
                            &mut tree_text,
                            node_byte_range,
                            &edit,
                            new_text,
                        )?;
                        shift_cursor(&tree, &mut cursor, &edit);
                        continue 'matches;
                    }
                }
                break 'matches;
            }

            Ok(ObfuscatedQuestionData {
                text: String::from_utf8(tree_text)?,
                answer,
            })
        })
        .filter(|question_data| {
            question_data.as_ref().map_or(true, |question_data| {
                used_answers.insert(question_data.answer.clone())
            })
        })
        .take(num)
        .collect::<Result<Vec<_>>>()?)
}
