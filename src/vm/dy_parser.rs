
pub enum BlockState {
	None,
	CommentBlock,
	StringBlock,
}

pub enum RegionKind {
	None,
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

pub struct RegionTree {
	kind: RegionKind,
	line: FormatedLine,
    parent: Option<Weak<RefCell<Box<RegionTree>>>>,
	children: Option<Vec<Rc<RefCell<Box<RegionTree>>>>>,
}


pub struct FormatedLine {
	block_state: BlockState,
    region_tree: Option<Rc<RegionTree>>,
    index: i32,
    tokens: Option<Vec<SyntaxToken>>,
}

pub enum TokenKind {
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

pub struct SyntaxToken {
	pub kind: TokenKind,
    pub text: str,

}

fn scan_char_literal(line: str, start: & mut i32) -> SyntaxToken {

}

fn scan_whitespace(line: str, start: & mut i32) -> SyntaxToken {

}

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

pub struct DyParser<'a> {
	pub lines: Vec<&'a str>,
	pub root_region: RegionTree<'a>,
}

impl<'a> DyParser<'a> {
	pub fn parser(&self, code_content : & String) {
		self.lines = code_content.replace("\r\n", "\n").replace("\r", "\n").split("\n").collect();
	}

	pub fn parse_line() {

	}

	pub fn lex_line() {

	}

	pub fn tokenize() {

	}
    pub fn
}
