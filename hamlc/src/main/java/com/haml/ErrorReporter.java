package com.haml;

import org.antlr.v4.runtime.Token;

import java.util.Deque;
import java.util.List;
import java.util.Map;

public class ErrorReporter {
    private final Deque<Error> errors;
    private final Map<String, List<String>> files;

    public ErrorReporter(Deque<Error> errors, Map<String, List<String>> files) {
        this.errors = errors;
        this.files = files;
    }

    public void addFile(String name, List<String> lines) {
        files.put(name, lines);
    }

    public void report(Error error) {
        errors.addFirst(error);
    }

    public void reportFromToken(Token token, String message, Object... args) {
        var error = new Error(
            token.getTokenSource().getSourceName(),
            token.getLine() - 1,
            token.getCharPositionInLine(),
            String.format(message, args));
        report(error);
    }

    public boolean hasErrors() {
        return errors.size() > 0;
    }

    public String reportErrors() {
        var builder = new StringBuilder();
        builder.append(String.format("Failed to compile due to %d errors:\n\n", errors.size()));
        for (var error : errors) {
            builder.append(error.userError(files.get(error.file)));
        }
        return builder.toString();
    }
}
