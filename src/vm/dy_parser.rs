use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter::FromIterator;
use vm::dy_util::VecExtend;
use vm::dy_common::DyRef;

// https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/keywords/index
const KEYWORDS: [&'static str; 58] = ["abstract", "as", "base", "break", "case", "catch", "checked", "class", "const", "continue",
		"default", "delegate", "do", "else", "enum", "event", "explicit", "extern", "finally",
		"fixed", "for", "foreach", "goto", "if", "implicit", "in", "interface", "internal", "is",
		"lock", "namespace", "new", "operator", "out", "override", "params", "private",
		"protected", "public", "readonly", "ref", "return", "sealed", "sizeof", "stackalloc", "static",
		"struct", "switch", "this", "throw", "try", "typeof", "unchecked", "unsafe", "using", "virtual",
		"volatile", "while"];



const OPERATORS: [&'static str; 44] = ["++", "--", "->", "+", "-", "!", "~", "++", "--", "&", "*", "/", "%", "+", "-", "<<", ">>", "<", ">",
		"<=", ">=", "==", "!=", "&", "^", "|", "&&", "||", "??", "?", "::", ":",
		"=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "=>"];

const BUILTIN_TYPES: [&'static str; 16] = ["bool", "byte", "char", "decimal", "double", "float", "int", "long", "object", "sbyte", "short",
		"string", "uint", "ulong", "ushort", "void"];

const PREPROCESSOR: [&'static str; 12] = ["define", "elif", "else", "endif", "endregion", "error", "if", "line", "pragma", "region", "undef", "warning"];


const PUNCTUATORS: [&'static str; 10] = [";", ":", ",", ".", "(", ")", "[", "]", "{", "}"];

// https://docs.microsoft.com/en-us/dotnet/csharp/language-reference/language-specification/lexical-structure

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    fn new(start: usize, end: usize) -> Span {
        Span { start, end, }
    }
}


trait Scanner {
    type Item;
    fn scan(dy_parser: &DyParser) -> Option<Self::Item>;
    fn get_span(&self) -> &Span;
}


#[derive(Debug)]
pub struct Whitespace {
    pub span: Span,
}

impl Whitespace {
    fn new (start: usize, end: usize) -> Whitespace {
        Whitespace { span: Span::new(start, end),}
    }
}

impl Scanner for Whitespace {
    type Item = Whitespace;
    fn scan(dy_parser: &DyParser) -> Option<Self::Item> {
        let (mut start_at, end_at) = dy_parser.cursor_span();
        let begin = start_at;
        while start_at <= end_at {
            let ch = dy_parser.source[start_at];
            if ch == '\t' || ch == ' ' || ch == '\r' || ch == '\n' {
                start_at += 1;
            }
            else {
                break;
            }
        }
        if start_at == begin {
            return None;
        }
        return Some(Whitespace::new(begin, start_at-1 as usize));
    }

    fn get_span(&self) -> &Span {
        return &self.span;
    }
}


#[derive(Debug)]
pub enum CommentKind {
    Single,
    Multiple,
}

#[derive(Debug)]
pub struct Comment {
    pub kind: CommentKind,
    pub span: Span,
}

impl Comment {
    fn new(kind: CommentKind, start: usize, end: usize) -> Comment {
        Comment {
            kind,
            span: Span::new(start, end),
        }
    }
}

impl Scanner for Comment {
    type Item = Comment;
    fn scan(dy_parser: &DyParser) -> Option<Self::Item> {
        if dy_parser.look_ahead("//") {
            let begin = dy_parser.cursor;
            let line_ending = dy_parser.find_line_ending(dy_parser.cursor+2);
            // dy_parser.cursor = line_ending + 1;
            return Some(Comment::new(CommentKind::Single, begin, line_ending));
        }
        else if dy_parser.look_ahead("/*") {
            let begin = dy_parser.cursor;
            let comment_ending = dy_parser.find_next("*/", begin+2);
            if comment_ending != 0 {
                return Some(Comment::new(CommentKind::Multiple, begin, comment_ending));
            }
            // missing
        }
        return None
    }

    fn get_span(&self) -> &Span {
        return &self.span;
    }
}


#[derive(Debug)]
pub enum StringKind {
    Normal,
    Verbatim,
    Interpolated,
}

#[derive(Debug)]
pub struct StringLiteral {
    pub kind: StringKind,
    pub span: Span,
}


impl Scanner for StringLiteral {
    type Item = StringLiteral;
    fn scan(dy_parser: &DyParser) -> Option<Self::Item> {
        if dy_parser.look_ahead("@\"") {

        }
        else if dy_parser.look_ahead("$\"") {

        }
        else if dy_parser.look_ahead("\"") {

        }
        return None
    }

    fn get_span(&self) -> &Span {
        return &self.span;
    }
}

pub struct CharLiteral {
    pub span: Span,
}


#[derive(Debug)]
pub enum Token {
    Whitespace(Whitespace),
    Comment(Comment),

    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    CharLiteral,
    BoolLiteral,
    NullLiteral,

    Keyword,
    Identifier,
    Preprocessor,
    Punctuator,
    EoF,
}



#[derive(Debug)]
pub struct DyParser {
    source: Vec<char>,
    tokens: Vec<Token>,
    formated_lines: Vec<FormatedLine>,
    root_region: RegionRef,
    cursor: usize,
    line_offset: Vec<usize>,
}


impl DyParser {

    pub fn new(code : String) -> DyParser  {
        let source = code.chars().collect();
        DyParser {
            source,
            formated_lines: Vec::new(),
            root_region: RegionRef::default(),
            tokens: Vec::new(),
            cursor: 0,
            line_offset: Vec::new(),
        }
    }

    pub fn lex_line(&mut self) {
        let mut has_line_char = false;
        let mut pre_line_begin= 0;
        let mut formated_line_count = 0;
        for (index, ch) in self.source.iter().enumerate() {
            if *ch == '\r' || *ch == '\n' {
                has_line_char = true;
            }
                else {
                    if has_line_char {
                        let pre_line_end = index - 1;
                        let mut formated_line = FormatedLine::new(formated_line_count, pre_line_begin, pre_line_end);
                        self.tokenize(&mut formated_line);
                        if formated_line_count == 0 {
                            // formated_line.region = self.root_region.clone()
                            formated_line.block_state = BlockState::None;
                        }
                            else {
                                let pre_line = &self.formated_lines[self.formated_lines.len()-1];
                                // formated_line.region = pre_line.region.clone();
                                formated_line.block_state = pre_line.block_state;
                            }
                        self.formated_lines.push(formated_line);
                        formated_line_count += 1;
                        pre_line_begin = index;
                    }
                    has_line_char = false;
                }
        }
    }

    fn tokenize(&self, formated_line: &mut FormatedLine) {
        let mut start_at = formated_line.begin_at;
        let end_at = formated_line.end_at;
        let ws = self.scan_whitespace(&mut start_at, end_at);
        if let Some(x) = ws {
            formated_line.push_token(x);
        }
        if formated_line.block_state == BlockState::None && start_at <= end_at && self.source[start_at] == '#' {

        }
    }

    fn cursor_span(&self) -> (usize, usize) {
        return (self.cursor, self.source.len()-1)
    }


    fn look_ahead(&self, text: &str) -> bool {
        return self.find_next(text, self.cursor) != 0;
    }

    // todo: 优化?
    // return: end index
    fn find_next(&self, text: &str, start_at: usize) -> usize {
        let len = self.source.len();
        if start_at >= len {
            return 0;
        }
        let target: Vec<char> = text.chars().collect();
        let target_len = target.len();
        for i in start_at..len {
            if target_len + i > len{
                return 0;
            }
            let mut found = true;
            for j in 0..target_len {
                if target[j] != self.source[i] {
                    found = false;
                    break;
                }
            }
            if found {
                return i + target_len -1 as usize;
            }
        }
        return 0;
    }

    fn find_line_ending(&self, start_at: usize) -> usize {
        let len = self.source.len();
        for i in start_at..len {
            let ch = self.source[i];
            if ch == '\n' {
                return i;
            }
            else if ch == '\r' {
                if i < len -1 && self.source[i+1] == '\n' {
                    return i+1;
                }
                return i;
            }
        }
        return len-1;
    }



    fn get_string(&self, start_at: usize, end_at: usize) -> Option<String> {
        if start_at < end_at && end_at < self.source.len() {
            let it = &self.source[start_at..end_at+1];
            return Some(String::from_iter(it));
        }
        return None;
    }

    fn is_keyword(&self, start_at: usize, end_at: usize) -> bool {
        if let Some(x) = self.get_string(start_at, end_at) {
            return KEYWORDS.contains(&x.as_str());
        }
        return false;
    }

    fn is_keyword_or_built_type(&self, start_at: usize, end_at: usize) -> bool {
        if let Some(x) = self.get_string(start_at, end_at) {
            let word = x.as_str();
            return KEYWORDS.contains(&word) || BUILTIN_TYPES.contains(&word);
        }
        return false;
    }

    fn is_operator(&self, start_at: usize, end_at: usize) -> bool {
        if let Some(x) = self.get_string(start_at, end_at) {
            return OPERATORS.contains(&x.as_str());
        }
        return false;
    }

    fn scan_whitespace(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        if *start_at < 0 || *start_at > end_at {
            return None;
        }
        let begin = *start_at;
        while *start_at <= end_at && (self.source[*start_at] == ' ' || self.source[*start_at] == '\t') {
            *start_at += 1;
        }
        if *start_at == begin {
            return None;
        }
        return Some(SyntaxToken::new(TokenKind::Whitespace, begin, *start_at-1));

    }

    fn scan_word(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        let begin = *start_at;
        while *start_at <= end_at {
            let ch = self.source[*start_at];
            if !ch.is_digit(10) && !ch.is_alphabetic() && ch != '_' {
                break;
            }
            *start_at += 1;
        }
        return Some(SyntaxToken::new(TokenKind::Identifier, begin, *start_at-1));
    }

    // todo:需要检查异常情况？？
    fn scan_char_literal(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        if self.source[*start_at] != '\'' {
            return None;
        }
        let begin = *start_at;
        while *start_at <= end_at {
            let ch = self.source[*start_at];
            if ch == '\'' {
                *start_at += 1;
                break;
            }
            if ch == '\\' && *start_at < end_at -1 {
                *start_at += 1;
            }
            *start_at += 1;
        }
        return Some(SyntaxToken::new(TokenKind::CharLiteral, begin, *start_at-1));
    }

    fn scan_string_literal(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        if self.source[*start_at] != '"' {
            return None;
        }
        let begin = *start_at;
        while *start_at <= end_at {
            let ch = self.source[*start_at];
            if ch == '"' {
                *start_at += 1;
                break;
            }
            if ch == '\\' && *start_at < end_at {
                *start_at += 1;
            }
            *start_at += 1;
        }
        return Some(SyntaxToken::new(TokenKind::StringLiteral, begin, *start_at-1));
    }

    fn scan_number_literal(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        let mut hex = false;
        let mut point = false;
        let mut exponent = false;
        let begin = *start_at;
        if self.source[*start_at] == '0' &&  *start_at < end_at
            && (self.source[*start_at+1] == 'x' || self.source[*start_at+1] == 'X') {
            *start_at += 2;
            hex = true;
            while *start_at <= end_at {
                let ch = self.source[*start_at];
                if (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' || ch < 'F') {
                    *start_at += 1;
                }
                    else {
                        break;
                    }
            }
        }
            else {
                while *start_at <= end_at &&  '0' <= self.source[*start_at] && self.source[*start_at] <= '9'{
                    *start_at += 1;
                }
            }
        if hex {
            return Some(SyntaxToken::new(TokenKind::IntegerLiteral, begin, *start_at-1));
        }
        if *start_at > begin && *start_at <= end_at {
            let ch = self.source[*start_at];
            if ch == 'l' || ch == 'L' || ch == 'u' || ch == 'U' {
                *start_at += 1;
                if *start_at <= end_at {
                    let ch_next = self.source[*start_at];
                    if (ch == 'l' || ch == 'L') && (ch_next == 'u' || ch_next == 'U') {
                        *start_at += 1;
                    }
                        else if ch_next == 'l' || ch_next == 'L' {
                            *start_at += 1;
                        }
                }
                return Some(SyntaxToken::new(TokenKind::IntegerLiteral, begin, *start_at-1));
            }
        }
        while *start_at <= end_at {
            let ch = self.source[*start_at];
            if !point && !exponent && ch == '.' {
                if *start_at < end_at &&  '0' <= self.source[*start_at+1] && self.source[*start_at+1] <= '9'{
                    *start_at += 1;
                    point = true;
                    continue;
                }
                    else {
                        break;
                    }
            }
            if !exponent && *start_at > begin && (ch == 'e' || ch == 'E'){
                exponent = true;
                *start_at += 1;
                if *start_at <= end_at &&  ('0' == self.source[*start_at] || self.source[*start_at] <= '9') {
                    *start_at += 1;
                }
                continue;
            }
            if ch == 'f' || ch == 'F' || ch == 'd' || ch == 'D' || ch == 'm' || ch == 'M' {
                point = true;
                *start_at += 1;
                break;
            }
            if ch < '0' || ch > '9' {
                break;
            }
            *start_at += 1;

        }
        let kind = if point || exponent {
            TokenKind::RealLiteral
        }
            else {
                TokenKind::IntegerLiteral
            };
        return Some(SyntaxToken::new(kind, begin, *start_at-1));

    }

    fn scan_hex_digit(&self, start_at: &mut usize, end_at: usize) -> bool {
        if *start_at > end_at {
            return false;
        }
        let ch = self.source[*start_at];
        if (ch >= '0' && ch <= '9') || (ch >= 'a' && ch <= 'f') || (ch >= 'A' || ch < 'F') {
            *start_at += 1;
            return true;
        }
        return false;
    }

    fn scan_unicode_escape_char(&self, start_at: &mut usize, end_at: usize) -> bool {
        if *start_at > end_at - 5 {
            return false;
        }
        if self.source[*start_at] != '\\' {
            return false;
        }
        let mut begin = *start_at + 1;
        let mut n = 0;
        if self.source[begin] == 'u' {
            n = 4;
        }
            else if self.source[begin] == 'U' {
                n = 8;
            }
                else {
                    return false;
                }
        begin += 1;
        while n >= 0 {
            if !self.scan_hex_digit(&mut begin, end_at) {
                break;
            }
            n -= 1;
        }
        if n == 0 {
            *start_at = begin;
            return true;
        }
        return false;

    }

    fn scan_identifier_or_keyword(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        let mut token = self.scan_identifier_or_keyword_raw(start_at, end_at);
        if let Some(ref mut st) = token {
            if st.kind == TokenKind::Keyword && !self.is_keyword_or_built_type(st.begin_at, st.end_at) {
                st.kind = TokenKind::Identifier;
            }
        }
        return token;
    }

    fn scan_identifier_or_keyword_raw(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        let mut identifier = false;
        let begin = *start_at;
        if *start_at > end_at {
            return None;
        }
        let ch = self.source[*start_at];
        if ch == '@' {
            identifier = true;
            *start_at += 1;
        }
        if *start_at <= end_at {
            let ch = self.source[*start_at];
            if ch.is_alphabetic() || ch == '_' {
                *start_at += 1;
            }
                else if !self.scan_unicode_escape_char(start_at, end_at) {
                    if begin == *start_at {
                        return None;
                    }
                    return Some(SyntaxToken::new(TokenKind::Identifier, begin, *start_at-1));
                }
                    else {
                        identifier = true;
                    }
            while *start_at <= end_at {
                let ch = self.source[*start_at];
                if ch.is_alphabetic() || ch.is_digit(10) || ch == '_' {
                    *start_at += 1;
                }
                    else if !self.scan_unicode_escape_char(start_at, end_at) {
                        break;
                    }
                        else {
                            identifier = true;
                        }
            }
        }
        let kind = if identifier {
            TokenKind::Identifier
        }
            else {
                TokenKind::Keyword
            };
        return Some(SyntaxToken::new(kind, begin, *start_at-1));
    }


    // ------------------------------- help function -----------------------------------------------
    fn source_equal(&self, start_at: usize, end_at: usize, text: &str) -> bool {
        for (i, ch) in text.chars().enumerate() {
            if start_at + i > end_at {
                return false;
            }
            if ch != self.source[start_at+i] {
                return false;
            }
        }
        return true;
    }

    fn push_whitespace(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let ws = self.scan_whitespace(start_at, formated_line.end_at);
        if let Some(x) = ws {
            formated_line.tokens.push(x);
            return true;
        }
        return false;
    }
    // ------------------------------ help function end --------------------------------------------

    // ---------------------------------- pp expression --------------------------------------------
    fn parse_pp_or_expression(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let end_at = formated_line.end_at;
        if *start_at > end_at {
            // todo: insert missing token
            return true;
        }
        let lhs = self.parse_pp_and_expression(formated_line, start_at);
        let begin = *start_at;
        self.push_whitespace(formated_line, start_at);
        let end_at = formated_line.end_at;
        if self.source_equal(*start_at, end_at, "||")  {
            let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+2);
            formated_line.tokens.push(token);
            *start_at += 2;
            self.push_whitespace(formated_line, start_at);
            let rhs = self.parse_pp_or_expression(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return rhs || lhs
        }
        return lhs
    }

    fn parse_pp_and_expression(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let end_at = formated_line.end_at;
        if *start_at > end_at {
            // todo: insert missing token
            return true;
        }
        let lhs = self.parse_pp_equal_expression(formated_line, start_at);
        let begin = *start_at;
        self.push_whitespace(formated_line, start_at);
        let end_at = formated_line.end_at;
        if self.source_equal(*start_at, end_at, "&&") {
            let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+2);
            formated_line.tokens.push(token);
            *start_at += 2;
            self.push_whitespace(formated_line, start_at);
            let rhs = self.parse_pp_and_expression(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return lhs && rhs;
        }
        return lhs
    }

    fn parse_pp_equal_expression(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let end_at = formated_line.end_at;
        if *start_at > end_at {
            // todo: insert missing token
            return true;
        }
        let lhs = self.parse_pp_unary_expression(formated_line, start_at);
        let begin = *start_at;
        self.push_whitespace(formated_line, start_at);
        let end_at = formated_line.end_at;
        if self.source_equal(*start_at, end_at, "==") || self.source_equal(*start_at, end_at, "!=") {
            let is_equal = self.source_equal(*start_at, end_at, "==");
            let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+2);
            formated_line.tokens.push(token);
            *start_at += 2;
            self.push_whitespace(formated_line, start_at);
            let rhs = self.parse_pp_equal_expression(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            if is_equal {
                return lhs == rhs;
            }
                else {
                    return lhs != rhs;
                }
        }
        return lhs
    }

    fn parse_pp_unary_expression(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let end_at = formated_line.end_at;
        if *start_at > end_at {
            // todo: insert missing token
            return true;
        }
        let begin = *start_at;
        self.push_whitespace(formated_line, start_at);
        let end_at = formated_line.end_at;
        if self.source[*start_at] == '!' {
            let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+1);
            formated_line.tokens.push(token);
            *start_at += 1;
            self.push_whitespace(formated_line, start_at);
            let result = self.parse_pp_unary_expression(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return !result;
        }
        return self.parse_pp_primary_expression(formated_line, start_at);
    }

    fn parse_pp_primary_expression(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        if self.source[*start_at] == '(' {
            let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+1);
            formated_line.tokens.push(token);
            *start_at += 1;
            self.push_whitespace(formated_line, start_at);
            let result = self.parse_pp_or_expression(formated_line, start_at);
            if *start_at > formated_line.end_at {
                // todo: insert missing token
                return result;
            }

            if self.source[*start_at] == '）' {
                let token = SyntaxToken::new(TokenKind::PreprocessorArguments, *start_at, *start_at+1);
                formated_line.tokens.push(token);
                *start_at += 1;
                self.push_whitespace(formated_line, start_at);
            }
            return result;
        }
        let result = self.parse_pp_symbol(formated_line, start_at);
        self.push_whitespace(formated_line, start_at);
        return result;
    }

    fn parse_pp_symbol(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let word = self.scan_identifier_or_keyword(start_at, formated_line.end_at);
        if let Some(mut x) = word {
            x.kind = TokenKind::PreprocessorSymbol;
            let x = formated_line.tokens.put(x);
            // formated_line.tokens.push(x);
            if self.source_equal(x.begin_at, x.end_at, "true") {
                return true;
            }
                else if self.source_equal(x.begin_at, x.end_at, "false") {
                    return false;
                }
            // todo: vm compilation define
            return false;
        }
        return true;
    }

    // ----------------------------------- region --------------------------------------------------
//    fn open_region(&mut self, formated_line: &mut FormatedLine, region_kind: RegionKind) {
//        // todo: 同一行不能有两个region
//        let mut parent_region = Weak::clone(&formated_line.region);
//        if region_kind == RegionKind::InactiveElif ||
//            region_kind == RegionKind::Elif ||
//            region_kind == RegionKind::Else ||
//            region_kind == RegionKind::InactiveElse {
//            let mut region_rc_op = formated_line.region.upgrade();
//            if let Some(ref mut region_rc) = region_rc_op {
//                let region_op = Rc::get_mut(region_rc);
//                if let Some(region) = region_op {
//                    parent_region = Weak::clone(&region.parent)
//                }
//            }
//            // todo error
//        }
//        let mut parent_rc = parent_region.upgrade().unwrap();
//        let parent = Rc::get_mut(&mut parent_rc).unwrap();
//        let region = Region::new(region_kind, formated_line.index);
//        parent.children.push(region);
//    }
//
//    fn close_region(formated_line: &mut FormatedLine) {
//        let region_rc = formated_line.region.upgrade();
//        match region_rc {
//            None => formated_line.region = Weak::new(),
//            Some(region) => formated_line.region = Weak::clone(&region.parent),
//        }
//    }
}






#[derive(Debug)]
#[derive(PartialEq)]
enum TokenKind {
    Missing,
    Whitespace,
    Comment,
    Preprocessor,
    PreprocessorArguments,
    PreprocessorSymbol,
    PreprocessorDirectiveExpected,
    PreprocessorCommentExpected,
    PreprocessorUnexpectedDirective,


    VerbatimPrefix,    // @
    InterpolatedPrefix,   // $
    BoolLiteral,   // true, false
    NullLiteral,  // bool
    CharLiteral,
    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    PunctuatorOrOperator,  // 分隔符
    Keyword,
    Identifier,
    EOF,

}


#[derive(Debug)]
#[derive(PartialEq)]
#[repr(u8)]
#[derive(Copy, Clone)]
enum BlockState {
    None,
    Comment,
    String,
}

#[derive(Debug)]
struct SyntaxToken {
    kind: TokenKind,
    begin_at: usize,
    end_at: usize,
    block_state: BlockState,
    line_index: usize,
}

impl SyntaxToken {
    fn new(kind: TokenKind, begin_at: usize, end_at: usize) -> SyntaxToken {
        SyntaxToken{kind, begin_at, end_at, block_state: BlockState::None, line_index: 0}
    }
}

#[derive(Debug)]
struct FormatedLine {
    index: usize,
    begin_at: usize,
    end_at: usize,
    tokens: Vec<SyntaxToken>,
    block_state: BlockState,
    // region: Option<RegionRef>,
}

impl FormatedLine {
    fn new(index: usize, begin_at: usize, end_at: usize) -> FormatedLine {
        FormatedLine{
            index,
            begin_at,
            end_at,
            tokens: Vec::new(),
            block_state: BlockState::None,
            // region: None,
        }
    }

    fn push_token(&mut self, mut token: SyntaxToken) -> &SyntaxToken{
        token.line_index = self.index;
        self.tokens.put(token)
    }
}

#[derive(Debug)]
#[derive(PartialOrd, PartialEq)]
#[derive(Copy, Clone)]
pub enum RegionKind {
    Root,
    Region,
    If,
    Elif,
    Else,
    LastActive,
    InactiveRegion,
    InactiveIf,
    InactiveElif,
    InactiveElse,
}


#[derive(Debug)]
struct Region {
    kind: RegionKind,
    line_index: usize,
}

impl Default for Region {
    fn default() -> Region {
        Region::new(RegionKind::Root, 0)
    }
}

impl Region {
    fn new(kind: RegionKind, line_index: usize) -> Region {
        Region {
            kind,
            line_index,
        }
    }
}

type RegionRef = DyRef<Region>;


