package com.haml.passes;

import com.haml.CompilerState;
import com.haml.error.ErrorMessages;
import com.haml.error.ErrorReporter;
import com.haml.HamlParser;
import com.haml.HamlParserBaseListener;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

public class CheckSymbolReferencesPass extends HamlParserBaseListener {
    private final ErrorReporter errorReporter;
    private final CompilerState state;

    public CheckSymbolReferencesPass(
        ErrorReporter errorReporter,
        CompilerState state) {
        this.errorReporter = errorReporter;
        this.state = state;
    }

    public void run(ParseTree tree) {
        var walker = new ParseTreeWalker();
        walker.walk(this, tree);
    }

    @Override
    public void enterExpression(HamlParser.ExpressionContext ctx) {
        var identifier = ctx.IDENTIFIER();
        if (identifier == null) {
            return;
        }
        var name = identifier.getSymbol().getText();
        if (state.hasSymbol(name)) {
            return;
        }
        errorReporter.reportFromToken(ctx.getStart(), ErrorMessages.unknownIdentifier(name));
    }
}
