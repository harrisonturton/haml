package com.haml;

import com.haml.error.ErrorMessages;
import com.haml.error.ErrorReporter;

import java.io.IOException;

public class SemanticAnalysis extends HamlParserBaseListener {
    private final String filepath;
    private final State state;
    private final ErrorReporter errorReporter;

    public SemanticAnalysis(String filepath, State state, ErrorReporter errorReporter) {
        this.filepath = filepath;
        this.state = state;
        this.errorReporter = errorReporter;
    }

    @Override
    public void enterRuleDeclaration(HamlParser.RuleDeclarationContext ctx) {
        var name = ctx.IDENTIFIER().getText();
        if (state.getScope().resolve(name) != null) {
            errorReporter.reportFromToken(
                    ctx.getStart(), ErrorMessages.duplicateIdentifier(name));
            return;
        }
        state.getScope().define(name, new Symbol());
    }

    @Override
    public void enterExpression(HamlParser.ExpressionContext ctx) {
        if (ctx.IDENTIFIER() == null) {
            return;
        }
        var name = ctx.IDENTIFIER().getText();
        if (state.getScope().resolve(name) != null) {
            return;
        }
        errorReporter.reportFromToken(ctx.getStart(), ErrorMessages.unknownIdentifier(name));
    }

    @Override
    public void enterSelectiveImport(HamlParser.SelectiveImportContext ctx) {
        var importFilepath = ctx.STRING().getText().replace("\"", "");
        var absolutePath = Util.resolveImport(filepath, importFilepath);
        if (absolutePath.equals(filepath)) {
            errorReporter.reportFromToken(ctx.getStart(), ErrorMessages.fileImportedItself(absolutePath));
            return;
        }

        var importState = State.createEmpty(filepath);
        var compiler = Compiler.createDefaultWithState(absolutePath, importState);

        try {
            compiler.run();
        } catch (IOException e) {
            throw new RuntimeException(e);
        }

        state.getScope().addParent(importState.getScope());
    }
}
