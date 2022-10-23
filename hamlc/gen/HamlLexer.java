// Generated from /Users/harryturton/Documents/projects/haml/parser/src/main/antlr4/com/haml/HamlLexer.g4 by ANTLR 4.10.1
import org.antlr.v4.runtime.Lexer;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenStream;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.*;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class HamlLexer extends Lexer {
	static { RuntimeMetaData.checkVersion("4.10.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		OPEN_BRACE=1, CLOSE_BRACE=2, OPEN_PAREN=3, CLOSE_PAREN=4, OPEN_BRACKET=5, 
		CLOSE_BRACKET=6, FORWARD_SLASH=7, STAR=8, EQ=9, QUESTION=10, SEMICOLON=11, 
		COLON=12, STRUCT=13, RULE=14, SPEC=15, STRING=16, ENUM=17, COMMA=18, TYPE=19, 
		INTERFACE=20, IDENTIFIER=21, KEYWORD=22, WS=23, BLOCK_COMMENT=24, START_SLASH_COMMENT=25, 
		SLASH_COMMENT=26;
	public static final int
		SlashComment=1;
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE", "SlashComment"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"OPEN_BRACE", "CLOSE_BRACE", "OPEN_PAREN", "CLOSE_PAREN", "OPEN_BRACKET", 
			"CLOSE_BRACKET", "FORWARD_SLASH", "STAR", "EQ", "QUESTION", "SEMICOLON", 
			"COLON", "STRUCT", "RULE", "SPEC", "STRING", "ENUM", "COMMA", "TYPE", 
			"INTERFACE", "IDENTIFIER", "KEYWORD", "WS", "BLOCK_COMMENT", "START_SLASH_COMMENT", 
			"SLASH_COMMENT"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'{'", "'}'", "'('", "')'", "'['", "']'", "'/'", "'*'", "'='", 
			"'?'", "';'", "':'", "'struct'", "'rule'", "'spec'", null, "'enum'", 
			"','", null, "'interface'", null, null, null, null, "'//'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "OPEN_BRACE", "CLOSE_BRACE", "OPEN_PAREN", "CLOSE_PAREN", "OPEN_BRACKET", 
			"CLOSE_BRACKET", "FORWARD_SLASH", "STAR", "EQ", "QUESTION", "SEMICOLON", 
			"COLON", "STRUCT", "RULE", "SPEC", "STRING", "ENUM", "COMMA", "TYPE", 
			"INTERFACE", "IDENTIFIER", "KEYWORD", "WS", "BLOCK_COMMENT", "START_SLASH_COMMENT", 
			"SLASH_COMMENT"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}


	public HamlLexer(CharStream input) {
		super(input);
		_interp = new LexerATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@Override
	public String getGrammarFileName() { return "HamlLexer.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public String[] getChannelNames() { return channelNames; }

	@Override
	public String[] getModeNames() { return modeNames; }

	@Override
	public ATN getATN() { return _ATN; }

	public static final String _serializedATN =
		"\u0004\u0000\u001a\u00b9\u0006\uffff\uffff\u0006\uffff\uffff\u0002\u0000"+
		"\u0007\u0000\u0002\u0001\u0007\u0001\u0002\u0002\u0007\u0002\u0002\u0003"+
		"\u0007\u0003\u0002\u0004\u0007\u0004\u0002\u0005\u0007\u0005\u0002\u0006"+
		"\u0007\u0006\u0002\u0007\u0007\u0007\u0002\b\u0007\b\u0002\t\u0007\t\u0002"+
		"\n\u0007\n\u0002\u000b\u0007\u000b\u0002\f\u0007\f\u0002\r\u0007\r\u0002"+
		"\u000e\u0007\u000e\u0002\u000f\u0007\u000f\u0002\u0010\u0007\u0010\u0002"+
		"\u0011\u0007\u0011\u0002\u0012\u0007\u0012\u0002\u0013\u0007\u0013\u0002"+
		"\u0014\u0007\u0014\u0002\u0015\u0007\u0015\u0002\u0016\u0007\u0016\u0002"+
		"\u0017\u0007\u0017\u0002\u0018\u0007\u0018\u0002\u0019\u0007\u0019\u0001"+
		"\u0000\u0001\u0000\u0001\u0001\u0001\u0001\u0001\u0002\u0001\u0002\u0001"+
		"\u0003\u0001\u0003\u0001\u0004\u0001\u0004\u0001\u0005\u0001\u0005\u0001"+
		"\u0006\u0001\u0006\u0001\u0007\u0001\u0007\u0001\b\u0001\b\u0001\t\u0001"+
		"\t\u0001\n\u0001\n\u0001\u000b\u0001\u000b\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\r\u0001\r\u0001\r\u0001\r\u0001\r\u0001"+
		"\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000e\u0001\u000f\u0001"+
		"\u000f\u0005\u000fb\b\u000f\n\u000f\f\u000fe\t\u000f\u0001\u000f\u0001"+
		"\u000f\u0001\u0010\u0001\u0010\u0001\u0010\u0001\u0010\u0001\u0010\u0001"+
		"\u0011\u0001\u0011\u0001\u0012\u0001\u0012\u0001\u0012\u0001\u0012\u0001"+
		"\u0012\u0001\u0012\u0001\u0012\u0001\u0012\u0001\u0012\u0001\u0012\u0001"+
		"\u0012\u0001\u0012\u0003\u0012|\b\u0012\u0001\u0013\u0001\u0013\u0001"+
		"\u0013\u0001\u0013\u0001\u0013\u0001\u0013\u0001\u0013\u0001\u0013\u0001"+
		"\u0013\u0001\u0013\u0001\u0014\u0004\u0014\u0089\b\u0014\u000b\u0014\f"+
		"\u0014\u008a\u0001\u0015\u0004\u0015\u008e\b\u0015\u000b\u0015\f\u0015"+
		"\u008f\u0001\u0015\u0005\u0015\u0093\b\u0015\n\u0015\f\u0015\u0096\t\u0015"+
		"\u0001\u0016\u0004\u0016\u0099\b\u0016\u000b\u0016\f\u0016\u009a\u0001"+
		"\u0016\u0001\u0016\u0001\u0017\u0001\u0017\u0001\u0017\u0001\u0017\u0005"+
		"\u0017\u00a3\b\u0017\n\u0017\f\u0017\u00a6\t\u0017\u0001\u0017\u0001\u0017"+
		"\u0001\u0017\u0001\u0018\u0001\u0018\u0001\u0018\u0001\u0018\u0001\u0018"+
		"\u0001\u0019\u0004\u0019\u00b1\b\u0019\u000b\u0019\f\u0019\u00b2\u0001"+
		"\u0019\u0003\u0019\u00b6\b\u0019\u0001\u0019\u0001\u0019\u0003c\u00a4"+
		"\u00b2\u0000\u001a\u0002\u0001\u0004\u0002\u0006\u0003\b\u0004\n\u0005"+
		"\f\u0006\u000e\u0007\u0010\b\u0012\t\u0014\n\u0016\u000b\u0018\f\u001a"+
		"\r\u001c\u000e\u001e\u000f \u0010\"\u0011$\u0012&\u0013(\u0014*\u0015"+
		",\u0016.\u00170\u00182\u00194\u001a\u0002\u0000\u0001\u0007\u0001\u0000"+
		"\"\"\u0002\u0000AZaz\u0001\u0000az\u0004\u000009AZ__az\u0003\u0000\t\n"+
		"\r\r  \u0001\u0000\n\n\u0001\u0001\n\n\u00bf\u0000\u0002\u0001\u0000\u0000"+
		"\u0000\u0000\u0004\u0001\u0000\u0000\u0000\u0000\u0006\u0001\u0000\u0000"+
		"\u0000\u0000\b\u0001\u0000\u0000\u0000\u0000\n\u0001\u0000\u0000\u0000"+
		"\u0000\f\u0001\u0000\u0000\u0000\u0000\u000e\u0001\u0000\u0000\u0000\u0000"+
		"\u0010\u0001\u0000\u0000\u0000\u0000\u0012\u0001\u0000\u0000\u0000\u0000"+
		"\u0014\u0001\u0000\u0000\u0000\u0000\u0016\u0001\u0000\u0000\u0000\u0000"+
		"\u0018\u0001\u0000\u0000\u0000\u0000\u001a\u0001\u0000\u0000\u0000\u0000"+
		"\u001c\u0001\u0000\u0000\u0000\u0000\u001e\u0001\u0000\u0000\u0000\u0000"+
		" \u0001\u0000\u0000\u0000\u0000\"\u0001\u0000\u0000\u0000\u0000$\u0001"+
		"\u0000\u0000\u0000\u0000&\u0001\u0000\u0000\u0000\u0000(\u0001\u0000\u0000"+
		"\u0000\u0000*\u0001\u0000\u0000\u0000\u0000,\u0001\u0000\u0000\u0000\u0000"+
		".\u0001\u0000\u0000\u0000\u00000\u0001\u0000\u0000\u0000\u00002\u0001"+
		"\u0000\u0000\u0000\u00014\u0001\u0000\u0000\u0000\u00026\u0001\u0000\u0000"+
		"\u0000\u00048\u0001\u0000\u0000\u0000\u0006:\u0001\u0000\u0000\u0000\b"+
		"<\u0001\u0000\u0000\u0000\n>\u0001\u0000\u0000\u0000\f@\u0001\u0000\u0000"+
		"\u0000\u000eB\u0001\u0000\u0000\u0000\u0010D\u0001\u0000\u0000\u0000\u0012"+
		"F\u0001\u0000\u0000\u0000\u0014H\u0001\u0000\u0000\u0000\u0016J\u0001"+
		"\u0000\u0000\u0000\u0018L\u0001\u0000\u0000\u0000\u001aN\u0001\u0000\u0000"+
		"\u0000\u001cU\u0001\u0000\u0000\u0000\u001eZ\u0001\u0000\u0000\u0000 "+
		"_\u0001\u0000\u0000\u0000\"h\u0001\u0000\u0000\u0000$m\u0001\u0000\u0000"+
		"\u0000&{\u0001\u0000\u0000\u0000(}\u0001\u0000\u0000\u0000*\u0088\u0001"+
		"\u0000\u0000\u0000,\u008d\u0001\u0000\u0000\u0000.\u0098\u0001\u0000\u0000"+
		"\u00000\u009e\u0001\u0000\u0000\u00002\u00aa\u0001\u0000\u0000\u00004"+
		"\u00b0\u0001\u0000\u0000\u000067\u0005{\u0000\u00007\u0003\u0001\u0000"+
		"\u0000\u000089\u0005}\u0000\u00009\u0005\u0001\u0000\u0000\u0000:;\u0005"+
		"(\u0000\u0000;\u0007\u0001\u0000\u0000\u0000<=\u0005)\u0000\u0000=\t\u0001"+
		"\u0000\u0000\u0000>?\u0005[\u0000\u0000?\u000b\u0001\u0000\u0000\u0000"+
		"@A\u0005]\u0000\u0000A\r\u0001\u0000\u0000\u0000BC\u0005/\u0000\u0000"+
		"C\u000f\u0001\u0000\u0000\u0000DE\u0005*\u0000\u0000E\u0011\u0001\u0000"+
		"\u0000\u0000FG\u0005=\u0000\u0000G\u0013\u0001\u0000\u0000\u0000HI\u0005"+
		"?\u0000\u0000I\u0015\u0001\u0000\u0000\u0000JK\u0005;\u0000\u0000K\u0017"+
		"\u0001\u0000\u0000\u0000LM\u0005:\u0000\u0000M\u0019\u0001\u0000\u0000"+
		"\u0000NO\u0005s\u0000\u0000OP\u0005t\u0000\u0000PQ\u0005r\u0000\u0000"+
		"QR\u0005u\u0000\u0000RS\u0005c\u0000\u0000ST\u0005t\u0000\u0000T\u001b"+
		"\u0001\u0000\u0000\u0000UV\u0005r\u0000\u0000VW\u0005u\u0000\u0000WX\u0005"+
		"l\u0000\u0000XY\u0005e\u0000\u0000Y\u001d\u0001\u0000\u0000\u0000Z[\u0005"+
		"s\u0000\u0000[\\\u0005p\u0000\u0000\\]\u0005e\u0000\u0000]^\u0005c\u0000"+
		"\u0000^\u001f\u0001\u0000\u0000\u0000_c\u0005\"\u0000\u0000`b\b\u0000"+
		"\u0000\u0000a`\u0001\u0000\u0000\u0000be\u0001\u0000\u0000\u0000cd\u0001"+
		"\u0000\u0000\u0000ca\u0001\u0000\u0000\u0000df\u0001\u0000\u0000\u0000"+
		"ec\u0001\u0000\u0000\u0000fg\u0005\"\u0000\u0000g!\u0001\u0000\u0000\u0000"+
		"hi\u0005e\u0000\u0000ij\u0005n\u0000\u0000jk\u0005u\u0000\u0000kl\u0005"+
		"m\u0000\u0000l#\u0001\u0000\u0000\u0000mn\u0005,\u0000\u0000n%\u0001\u0000"+
		"\u0000\u0000op\u0005s\u0000\u0000pq\u0005t\u0000\u0000qr\u0005r\u0000"+
		"\u0000rs\u0005i\u0000\u0000st\u0005n\u0000\u0000t|\u0005g\u0000\u0000"+
		"uv\u0005n\u0000\u0000vw\u0005u\u0000\u0000wx\u0005m\u0000\u0000xy\u0005"+
		"b\u0000\u0000yz\u0005e\u0000\u0000z|\u0005r\u0000\u0000{o\u0001\u0000"+
		"\u0000\u0000{u\u0001\u0000\u0000\u0000|\'\u0001\u0000\u0000\u0000}~\u0005"+
		"i\u0000\u0000~\u007f\u0005n\u0000\u0000\u007f\u0080\u0005t\u0000\u0000"+
		"\u0080\u0081\u0005e\u0000\u0000\u0081\u0082\u0005r\u0000\u0000\u0082\u0083"+
		"\u0005f\u0000\u0000\u0083\u0084\u0005a\u0000\u0000\u0084\u0085\u0005c"+
		"\u0000\u0000\u0085\u0086\u0005e\u0000\u0000\u0086)\u0001\u0000\u0000\u0000"+
		"\u0087\u0089\u0007\u0001\u0000\u0000\u0088\u0087\u0001\u0000\u0000\u0000"+
		"\u0089\u008a\u0001\u0000\u0000\u0000\u008a\u0088\u0001\u0000\u0000\u0000"+
		"\u008a\u008b\u0001\u0000\u0000\u0000\u008b+\u0001\u0000\u0000\u0000\u008c"+
		"\u008e\u0007\u0002\u0000\u0000\u008d\u008c\u0001\u0000\u0000\u0000\u008e"+
		"\u008f\u0001\u0000\u0000\u0000\u008f\u008d\u0001\u0000\u0000\u0000\u008f"+
		"\u0090\u0001\u0000\u0000\u0000\u0090\u0094\u0001\u0000\u0000\u0000\u0091"+
		"\u0093\u0007\u0003\u0000\u0000\u0092\u0091\u0001\u0000\u0000\u0000\u0093"+
		"\u0096\u0001\u0000\u0000\u0000\u0094\u0092\u0001\u0000\u0000\u0000\u0094"+
		"\u0095\u0001\u0000\u0000\u0000\u0095-\u0001\u0000\u0000\u0000\u0096\u0094"+
		"\u0001\u0000\u0000\u0000\u0097\u0099\u0007\u0004\u0000\u0000\u0098\u0097"+
		"\u0001\u0000\u0000\u0000\u0099\u009a\u0001\u0000\u0000\u0000\u009a\u0098"+
		"\u0001\u0000\u0000\u0000\u009a\u009b\u0001\u0000\u0000\u0000\u009b\u009c"+
		"\u0001\u0000\u0000\u0000\u009c\u009d\u0006\u0016\u0000\u0000\u009d/\u0001"+
		"\u0000\u0000\u0000\u009e\u009f\u0005/\u0000\u0000\u009f\u00a0\u0005*\u0000"+
		"\u0000\u00a0\u00a4\u0001\u0000\u0000\u0000\u00a1\u00a3\t\u0000\u0000\u0000"+
		"\u00a2\u00a1\u0001\u0000\u0000\u0000\u00a3\u00a6\u0001\u0000\u0000\u0000"+
		"\u00a4\u00a5\u0001\u0000\u0000\u0000\u00a4\u00a2\u0001\u0000\u0000\u0000"+
		"\u00a5\u00a7\u0001\u0000\u0000\u0000\u00a6\u00a4\u0001\u0000\u0000\u0000"+
		"\u00a7\u00a8\u0005*\u0000\u0000\u00a8\u00a9\u0005/\u0000\u0000\u00a91"+
		"\u0001\u0000\u0000\u0000\u00aa\u00ab\u0005/\u0000\u0000\u00ab\u00ac\u0005"+
		"/\u0000\u0000\u00ac\u00ad\u0001\u0000\u0000\u0000\u00ad\u00ae\u0006\u0018"+
		"\u0001\u0000\u00ae3\u0001\u0000\u0000\u0000\u00af\u00b1\b\u0005\u0000"+
		"\u0000\u00b0\u00af\u0001\u0000\u0000\u0000\u00b1\u00b2\u0001\u0000\u0000"+
		"\u0000\u00b2\u00b3\u0001\u0000\u0000\u0000\u00b2\u00b0\u0001\u0000\u0000"+
		"\u0000\u00b3\u00b5\u0001\u0000\u0000\u0000\u00b4\u00b6\u0007\u0006\u0000"+
		"\u0000\u00b5\u00b4\u0001\u0000\u0000\u0000\u00b6\u00b7\u0001\u0000\u0000"+
		"\u0000\u00b7\u00b8\u0006\u0019\u0002\u0000\u00b85\u0001\u0000\u0000\u0000"+
		"\u000b\u0000\u0001c{\u008a\u008f\u0094\u009a\u00a4\u00b2\u00b5\u0003\u0006"+
		"\u0000\u0000\u0005\u0001\u0000\u0004\u0000\u0000";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}