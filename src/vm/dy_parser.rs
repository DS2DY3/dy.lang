use std::rc::Weak;
use std::rc::Rc;
use std::cell::RefCell;

//pub enum BlockState {
//	None,
//	CommentBlock,
//	StringBlock,
//}
//
//pub enum RegionKind {
//	None,
//	Region,
//	If,
//	Elif,
//	Else,
//	LastActive,
//	InactiveRegion,
//	InactiveIf,
//	InactiveElif,
//	InactiveElse,
//}
//
//pub struct RegionTree {
//	kind: RegionKind,
//	line: FormatedLine,
//    parent: Option<Weak<RefCell<Box<RegionTree>>>>,
//	children: Option<Vec<Rc<RefCell<Box<RegionTree>>>>>,
//}
//
//
//pub struct FormatedLine {
//	block_state: BlockState,
//    region_tree: Option<Rc<RegionTree>>,
//    index: i32,
//    tokens: Option<Vec<SyntaxToken>>,
//}
//
//pub enum TokenKind {
//	Missing,
//	Whitespace,
//	Comment,
//	Preprocessor,
//	PreprocessorArguments,
//	PreprocessorSymbol,
//	PreprocessorDirectiveExpected,
//	PreprocessorCommentExpected,
//	PreprocessorUnexpectedDirective,
//	VerbatimStringLiteral,
//
//	LastWSToken, // Marker only
//
//	VerbatimStringBegin,
//	BuiltInLiteral,
//	CharLiteral,
//	StringLiteral,
//	IntegerLiteral,
//	RealLiteral,
//	Punctuator,
//	Keyword,
//	Identifier,
//	ContextualKeyword,
//	EOF,
//}
//
//pub struct SyntaxToken {
//	pub kind: TokenKind,
//    pub text: str,
//
//}
//
//fn scan_char_literal(line: str, start: & mut i32) -> SyntaxToken {
//
//}
//
//fn scan_whitespace(line: str, start: & mut i32) -> SyntaxToken {
//
//}
//
const KEY_WORDS: [&'static str; 58] = ["abstract", "as", "base", "break", "case", "catch", "checked", "class", "const", "continue",
		"default", "delegate", "do", "else", "enum", "event", "explicit", "extern", "finally",
		"fixed", "for", "foreach", "goto", "if", "implicit", "in", "interface", "internal", "is",
		"lock", "namespace", "new", "operator", "out", "override", "params", "private",
		"protected", "public", "readonly", "ref", "return", "sealed", "sizeof", "stackalloc", "static",
		"struct", "switch", "this", "throw", "try", "typeof", "unchecked", "unsafe", "using", "virtual",
		"volatile", "while"];

const OPERATORS: [&'static str; 44] = ["++", "--", "->", "+", "-", "!", "~", "++", "--", "&", "*", "/", "%", "+", "-", "<<", ">>", "<", ">",
		"<=", ">=", "==", "!=", "&", "^", "|", "&&", "||", "??", "?", "::", ":",
		"=", "+=", "-=", "*=", "/=", "%=", "&=", "|=", "^=", "<<=", ">>=", "=>"];

const BUILT_TYPES: [&'static str; 16] = ["bool", "byte", "char", "decimal", "double", "float", "int", "long", "object", "sbyte", "short",
		"string", "uint", "ulong", "ushort", "void"];

const PREPROCESSOR_KEY_WORLDS: [&'static str; 12] = ["define", "elif", "else", "endif", "endregion", "error", "if", "line", "pragma", "region", "undef", "warning"];
//
//pub struct DyParser<'a> {
//	pub lines: Vec<&'a str>,
////	pub root_region: RegionTree<'a>,
//}
//
//impl<'a> DyParser<'a> {
//	pub fn parser(&self, code_content : & String) {
//		self.lines = code_content.replace("\r\n", "\n").replace("\r", "\n").split("\n").collect();
//	}
//
//	pub fn parse_line() {
//
//	}
//
//	pub fn lex_line() {
//
//	}
//
//	pub fn tokenize() {
//
//	}
//}

#[derive(Debug)]
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
    VerbatimStringLiteral,

    LastWSToken, // Marker only

    VerbatimStringBegin,
    BuiltInLiteral,
    CharLiteral,
    StringLiteral,
    IntegerLiteral,
    RealLiteral,
    Punctuator,
    Keyword,
    Identifier,
    ContextualKeyword,
    EOF,

}


#[derive(Debug)]
enum BlockState {
    None,
    CommentBlock,
    StringBlock
}

#[derive(Debug)]
struct SyntaxToken {
    kind: TokenKind,
    begin_at: usize,
    end_at: usize,

}

impl SyntaxToken {
    fn new(kind: TokenKind, begin_at: usize, end_at: usize) -> SyntaxToken {
        SyntaxToken{kind, begin_at, end_at}
    }
}

#[derive(Debug)]
pub struct DyParser {
    source: Vec<char>,
    formated_lines: Vec<FormatedLine>,

}

impl DyParser {

    pub fn new(code : String) -> DyParser  {
        let source = code.chars().collect();
        DyParser {
            source,
            formated_lines: Vec::new(),
        }
    }

    pub fn format(&mut self) {
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
                    self.formated_lines.push(formated_line);
                    formated_line_count += 1;
                    pre_line_begin = index;
                }
                has_line_char = false;
            }
        }
    }

    fn scan_whitespace(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
        if *start_at < 0 || *start_at > end_at {
            return None;
        }
        let mut begin = *start_at;
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

    fn scan_indentifier_or_keyword(&self, start_at: &mut usize, end_at: usize) -> Option<SyntaxToken> {
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
        let chs = text.chars();
        let len = chs.count();
        // let end_at = self.source.len() - 1;
        if end_at < start_at + len {
            return false;
        }
        for (i, &ch) in chs.enumerate() {
            if ch != self.source[start_at+i] {
                return false;
            }
        }
        return true;
    }

    fn push_whitespace(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
        let ws = self.scan_whitespace(start_at, formated_line.end_at);
        if let some(x) = ws {
            formated_line.tokens.push(x);
            return true;
        }
        return false;
    }
    // ------------------------------ help function end --------------------------------------------

    fn parse_pp_or_experssion(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
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
            let rhs = self.parse_pp_or_experssion(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return rhs || lhs
        }
        return lhs
    }

    fn parse_pp_and_experssion(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
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
            let rhs = self.parse_pp_and_experssion(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return lhs && rhs;
        }
        return lhs
    }

    fn parse_pp_equal_experssion(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
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
            let rhs = self.parse_pp_equal_experssion(formated_line, start_at);
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

    fn parse_pp_unary_experssion(&self, formated_line: &mut FormatedLine, start_at: &mut usize) -> bool {
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
            *start_at += 2;
            self.push_whitespace(formated_line, start_at);
            let result = self.parse_pp_unary_experssion(formated_line, start_at);
            self.push_whitespace(formated_line, start_at);
            return !result;
        }
        return self.parse_pp_primary_expression(formated_line, start_at);
    }



    fn tokenize(&self, formated_line: &mut FormatedLine) {
        let mut start_at = formated_line.begin_at;
        let end_at = formated_line.end_at;
        let ws = self.scan_whitespace(&mut start_at, end_at);
        if let Some(x) = ws {
            formated_line.tokens.push(x);
        }

    }
}

#[derive(Debug)]
struct FormatedLine {
    index: i32,
    begin_at: usize,
    end_at: usize,
    tokens: Vec<SyntaxToken>,
    block: BlockState,
}

impl FormatedLine {
    fn new(index: i32, begin_at: usize, end_at: usize) -> FormatedLine {
        FormatedLine{
            index,
            begin_at,
            end_at,
            tokens: Vec::new(),
            block: BlockState::None,
        }
    }
}
