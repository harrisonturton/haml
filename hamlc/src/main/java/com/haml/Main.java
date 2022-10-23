package com.haml;

import java.io.IOException;
import java.util.Deque;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Main {
//    public static String path = "/test/string.haml";
    public static String path = "/Users/harryturton/Documents/projects/haml/hamlc/src/main/resources/test/api.haml";

    public static void main(String[] args) {
        var errorReporter = createErrorReporter();
        var syntaxErrorListener = createSyntaxErrorListener(errorReporter);
        var compiler = createCompiler(
            errorReporter,
            syntaxErrorListener);

        try {
            compiler.run(path);
        } catch (IOException e) {
            System.err.println("Uncaught exception " + e);
            e.printStackTrace();
            System.exit(1);
        }
    }

    private static Compiler createCompiler(
        ErrorReporter errorReporter,
        SyntaxErrorListener syntaxErrorListener) {
        return new Compiler(
            errorReporter,
            syntaxErrorListener);
    }

    private static SyntaxErrorListener createSyntaxErrorListener(ErrorReporter errorReporter) {
        return new SyntaxErrorListener(errorReporter);
    }

    private static ErrorReporter createErrorReporter() {
        var errors = (Deque<Error>) new LinkedList<Error>();
        var files = new HashMap<String, List<String>>();
        return new ErrorReporter(errors, files);
    }
}