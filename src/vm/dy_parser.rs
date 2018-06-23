
enum BlockState {
	None,
	CommentBlock,
	StringBlock,
}

enum RegionKind {
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

struct RegionTree {
	kind: RegionKind,
	line: FormatedLline,
	parent: Box<RegionTree>,
	children: Vec<RegionTree>,
}


struct FormatedLline {
	block_state: BlockState,
	// region_tree: RegionTree,
}

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

struct SyntaxToken {
	token_kind: TokenKind,

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

struct DyParser {

}
