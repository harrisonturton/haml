package com.haml;

import com.haml.error.ErrorReporter;
import com.haml.error.SyntaxErrorListener;
import com.haml.passes.CheckSymbolReferencesPass;
import com.haml.passes.CollectSymbolsPass;
import com.haml.passes.ResolveImportsPass;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;

import java.io.*;
import java.nio.file.Paths;

public class Compiler {
    private final ErrorReporter errorReporter;
    private final SyntaxErrorListener syntaxErrorListener;

    private final CompilerState state = CompilerState.createEmpty();

    public Compiler(ErrorReporter errorReporter, SyntaxErrorListener syntaxErrorListener) {
        this.errorReporter = errorReporter;
        this.syntaxErrorListener = syntaxErrorListener;
    }

    public void run(String rootFilepath) throws IOException {
        state.queueFile(rootFilepath);

        while (state.hasRemainingFiles()) {
            runForFile(state.getNextFile());
        }

        if (errorReporter.hasErrors()) {
            var message = errorReporter.reportErrors();
            System.err.println(message);
        } else {
            System.out.printf("%s compiled successfully", Paths.get(rootFilepath).getFileName());
        }
    }

    private void runForFile(String filepath) throws IOException {
        var file = new File(filepath);
        var inputStream = ModuleReader.readModuleAbsolute(filepath);

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
        var resolveImportsPass = new ResolveImportsPass(
            state, filepath, syntaxErrorListener, errorReporter);

        var collectSymbolsPass = new CollectSymbolsPass(errorReporter, state);
        var checkSymbolReferencesPass = new CheckSymbolReferencesPass(errorReporter, state);

        if (resolveImportsPass.run(tree)) {
            state.queueFile(filepath);
            return;
        }

        if (!state.hasCollectedSymbols(filepath)) {
            collectSymbolsPass.run(tree);
            state.finishCollectingSymbols(filepath);
        }

        checkSymbolReferencesPass.run(tree);
        state.finishFile(filepath);
    }
}
