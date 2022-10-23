package com.haml;

import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.util.*;

public class SymbolTablePass extends HamlParserBaseListener {
    public static class Symbol {
        public final String name;
        public final String type;

        public Symbol(String name, String type) {
            this.name = name;
            this.type = type;
        }
    }

    private final ErrorReporter errorReporter;

    private final Map<String, Symbol> symbols;
    private boolean resolveForwardReferences = false;

    public SymbolTablePass(ErrorReporter errorReporter, Map<String, Symbol> symbols) {
        this.errorReporter = errorReporter;
        this.symbols = symbols;
    }

    public void run(ParseTree tree) {
        var walker = new ParseTreeWalker();
        walker.walk(this, tree);
        // Must do two passes in order to resolve forward references
        resolveForwardReferences = true;
        walker.walk(this, tree);
    }

    @Override
    public void enterRuleDeclaration(HamlParser.RuleDeclarationContext ctx) {
        var name = ctx.IDENTIFIER().getSymbol().getText();
        // Symbol has already been defined
        if (!resolveForwardReferences) {
            if (symbols.containsKey(name)) {
                errorReporter.reportFromToken(ctx.getStart(), "duplicate identifier \"%s\"", name);
                return;
            }
            var symbol = new Symbol(name, "ruleDeclaration");
            symbols.put(name, symbol);
        }
    }

    @Override
    public void enterExpression(HamlParser.ExpressionContext ctx) {
        var identifier = ctx.IDENTIFIER();
        if (identifier == null) {
            return;
        }
        var name = identifier.getSymbol().getText();
        if (symbols.containsKey(name)) {
            return;
        }
        if (resolveForwardReferences) {
            errorReporter.reportFromToken(ctx.getStart(), "unknown identifier \"%s\"", name);
        }
    }
}
