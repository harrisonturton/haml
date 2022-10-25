package com.haml.error;

import org.antlr.v4.runtime.*;

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
        var token = ((Token) offendingSymbol).getText();
        var error = new Error(
            errorReporter.getCurrentFile(),
            line,
            charPositionInLine,
            ErrorMessages.unknownSymbol(token));
        errorReporter.report(error);
    }

}
