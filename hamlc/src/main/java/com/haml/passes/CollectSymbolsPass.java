package com.haml.passes;

import com.haml.CompilerState;
import com.haml.HamlParser;
import com.haml.HamlParserBaseListener;
import com.haml.error.ErrorMessages;
import com.haml.error.ErrorReporter;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

public class CollectSymbolsPass extends HamlParserBaseListener {
    private final ErrorReporter errorReporter;

    private final CompilerState state;

    public CollectSymbolsPass(ErrorReporter errorReporter, CompilerState state) {
        this.errorReporter = errorReporter;
        this.state = state;
    }

    public void run(ParseTree tree) {
        var walker = new ParseTreeWalker();
        walker.walk(this, tree);
    }

    @Override
    public void enterRuleDeclaration(HamlParser.RuleDeclarationContext ctx) {
        var name = ctx.IDENTIFIER().getSymbol().getText();
        if (state.hasSymbol(name)) {
            errorReporter.reportFromToken(ctx.getStart(), ErrorMessages.duplicateIdentifier(name));
            return;
        }
        state.addSymbol(name);
    }
}
