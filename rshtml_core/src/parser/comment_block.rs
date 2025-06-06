﻿use crate::Node;
use crate::parser::{IParser, RsHtmlParser, Rule};
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub struct CommentBlockParser;

impl IParser for CommentBlockParser {
    fn parse(_: &mut RsHtmlParser, pair: Pair<Rule>) -> Result<Node, Box<Error<Rule>>> {
        let span = pair.as_span();

        Ok(Node::Comment(
            pair.into_inner()
                .find(|p| p.as_rule() == Rule::comment_content)
                .map(|p| p.as_str().to_string())
                .ok_or(Error::new_from_span(ErrorVariant::CustomError { message: "".into() }, span))?,
        ))
    }
}
