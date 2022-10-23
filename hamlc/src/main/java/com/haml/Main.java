package com.haml;

import java.io.IOException;
import java.util.Deque;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Main {
    public static String path = "/test/string.haml";

    public static void main(String[] args) {
        var inputStream = Main.class.getResourceAsStream(path);
        if (inputStream == null) {
            System.err.println("Input stream is null");
            System.exit(1);
        }

        var errorReporter = createErrorReporter();
        var symbolTablePass = createSymbolTablePass(errorReporter);
        var compiler = createCompiler(errorReporter, symbolTablePass);

        try {
            compiler.run(inputStream);
        } catch (IOException e) {
            System.err.println("Uncaught exception " + e);
            System.exit(1);
        }
    }

    public static Compiler createCompiler(
        ErrorReporter errorReporter,
        SymbolTablePass symbolTablePass) {
        return new Compiler(errorReporter, symbolTablePass);
    }

    public static SymbolTablePass createSymbolTablePass(ErrorReporter errorReporter) {
        return new SymbolTablePass(errorReporter);
    }

    public static ErrorReporter createErrorReporter() {
        var errors = (Deque<Error>) new LinkedList<Error>();
        var files = new HashMap<String, List<String>>();
        return new ErrorReporter(errors, files);
    }
}