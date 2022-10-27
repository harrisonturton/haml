package com.haml;

import com.haml.error.ErrorMessages;
import com.haml.error.ErrorReporter;

import java.io.IOException;
import java.util.HashMap;

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

        var properties = new HashMap<String, Symbol.Type>();
        for (var property : ctx.propertyDeclaration()) {
            var propertyName = property.IDENTIFIER().getText();
            if (property.expression().type() != null) {
                var rawType = property.expression().type().getText();
                var type = parseType(rawType);
                properties.put(propertyName, type);
                continue;
            }
            if (property.expression().STRING() != null) {
                var literal = property.expression().STRING().getText().replace("\"", "");
                var type = new Symbol.StringLiteralType(literal);
                properties.put(propertyName, type);
                continue;
            }
            if (property.expression().IDENTIFIER() != null) {
                var identifier = property.expression().IDENTIFIER().getText();
                var type = new Symbol.IdentifierType(identifier);
                properties.put(propertyName, type);
                continue;
            }
            errorReporter.reportFromToken(
                    property.getStart(),
                    ErrorMessages.unexpectedFailure("Unknown type %s for property \"%s\"",
                            property.expression().getText(), propertyName));
        }
        var type = new Symbol.StructType(properties);
        state.getScope().define(name, new Symbol(type, name));
    }

    private Symbol.Type parseType(String type) {
        return switch (type) {
            case "number" -> new Symbol.NumberType();
            case "string" -> new Symbol.StringType();
            default -> new Symbol.IdentifierType(type);
        };
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
