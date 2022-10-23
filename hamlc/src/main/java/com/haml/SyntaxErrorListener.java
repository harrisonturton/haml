package com.haml;

import org.antlr.v4.runtime.*;

import javax.tools.Diagnostic;

public class SyntaxErrorListener extends BaseErrorListener {
    private final ErrorReporter errorReporter;

    public SyntaxErrorListener(ErrorReporter errorReporter) {
        this.errorReporter = errorReporter;
    }

    @Override
    public void syntaxError(
        Recognizer<?, ?> recognizer,
        Object offendingSymbol,
        int line,
        int charPositionInLine,
        String msg,
        RecognitionException e
    ) {
        var token = (Token) offendingSymbol;
        var message = String.format("unknown token \"%s\"", token.getText());
        var error = new Error(errorReporter.getCurrentFile(), line, charPositionInLine, message);
        errorReporter.report(error);
    }

}
