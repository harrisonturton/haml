package com.haml;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;

import java.io.*;

public class Compiler {
    private final ErrorReporter errorReporter;
    private final SyntaxErrorListener syntaxErrorListener;
    private final SymbolTablePass buildSymbolTablePass;

    public Compiler(
            ErrorReporter errorReporter,
            SyntaxErrorListener syntaxErrorListener,
            SymbolTablePass buildSymbolTablePass) {
        this.errorReporter = errorReporter;
        this.syntaxErrorListener = syntaxErrorListener;
        this.buildSymbolTablePass = buildSymbolTablePass;
    }

    public void run(String filepath) throws IOException {
        // Need to use buffered reader in order to read the stream multiple times.
        var file = new File(filepath);
        var fileReader = new FileInputStream(file);
        var inputStream = new BufferedInputStream(fileReader);

        inputStream.mark(0);
        var inputReader = new InputStreamReader(inputStream);
        var lines = new BufferedReader(inputReader).lines().toList();
        errorReporter.setCurrentFile(file.getName());
        errorReporter.addFile(file.getName(), lines);
        inputStream.reset();

        var charStream = CharStreams.fromStream(inputStream);
        var lexer = new HamlLexer(charStream);
        var tokenStream = new CommonTokenStream(lexer);
        var parser = new HamlParser(tokenStream);

        parser.removeErrorListeners();
        parser.addErrorListener(syntaxErrorListener);

        var tree = parser.program();
        buildSymbolTablePass.run(tree);

        if (errorReporter.hasErrors()) {
            var message = errorReporter.reportErrors();
            System.err.println(message);
        }
    }
}
