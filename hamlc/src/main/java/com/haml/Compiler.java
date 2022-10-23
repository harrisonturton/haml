package com.haml;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;

import java.io.IOException;
import java.io.InputStream;

public class Compiler {
    private final ErrorReporter errorReporter;
    private final SymbolTablePass buildSymbolTablePass;

    public Compiler(ErrorReporter errorReporter, SymbolTablePass buildSymbolTablePass) {
        this.errorReporter = errorReporter;
        this.buildSymbolTablePass = buildSymbolTablePass;
    }

    public void run(InputStream inputStream) throws IOException {
        var charStream = CharStreams.fromStream(inputStream);
        var lexer = new HamlLexer(charStream);
        var tokenStream = new CommonTokenStream(lexer);
        var parser = new HamlParser(tokenStream);

        var tree = parser.program();
        buildSymbolTablePass.run(tree);

        if (errorReporter.hasErrors()) {
            var message = errorReporter.reportErrors();
            System.err.println(message);
        }
    }
}
