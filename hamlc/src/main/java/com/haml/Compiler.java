package com.haml;

import com.haml.error.ErrorReporter;
import com.haml.error.SyntaxErrorListener;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.HashMap;
import java.util.LinkedList;

public class Compiler {
    private final ErrorReporter errorReporter;
    private final SyntaxErrorListener syntaxErrorListener;
    private final State state;

    private Compiler(
            ErrorReporter errorReporter,
            SyntaxErrorListener syntaxErrorListener,
            State state) {
        this.errorReporter = errorReporter;
        this.syntaxErrorListener = syntaxErrorListener;
        this.state = state;
    }

    public static Compiler createDefault(String entryPath) {
        var errorReporter = new ErrorReporter(new LinkedList<>(), new HashMap<>());
        var syntaxErrorListener = new SyntaxErrorListener(errorReporter);
        var state = State.createEmpty(entryPath);
        state.pushFileFirst(entryPath);
        return new Compiler(errorReporter, syntaxErrorListener, state);
    }

    public static Compiler createDefaultWithState(String entryPath, State state) {
        var errorReporter = new ErrorReporter(new LinkedList<>(), new HashMap<>());
        var syntaxErrorListener = new SyntaxErrorListener(errorReporter);
        state.pushFileFirst(entryPath);
        return new Compiler(errorReporter, syntaxErrorListener, state);
    }

    public void run() throws IOException {
        while (state.hasRemainingFiles()) {
            runForFile(state.getNextFile());
        }
    }

    public ErrorReporter getErrorReporter() {
        return errorReporter;
    }

    private void runForFile(String filepath) throws IOException {
        var inputStream = Util.readModuleAbsolute(filepath);

        inputStream.mark(0);
        var inputReader = new InputStreamReader(inputStream);
        var lines = new BufferedReader(inputReader).lines().toList();
        errorReporter.setCurrentFile(filepath);
        errorReporter.addFile(filepath, lines);
        inputStream.reset();

        var charStream = CharStreams.fromStream(inputStream);
        var lexer = new HamlLexer(charStream);
        var tokenStream = new CommonTokenStream(lexer);
        var parser = new HamlParser(tokenStream);

        parser.removeErrorListeners();
        parser.addErrorListener(syntaxErrorListener);

        var tree = parser.program();
        var analysisPass = new SemanticAnalysis(filepath, state, errorReporter);
        var walker = new ParseTreeWalker();
        walker.walk(analysisPass, tree);
    }
}
