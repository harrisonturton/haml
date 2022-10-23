package com.haml;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;

import java.io.*;
import java.nio.file.Paths;
import java.util.*;

public class Compiler {
    private final ErrorReporter errorReporter;
    private final SyntaxErrorListener syntaxErrorListener;

    private final Deque<String> remainingFiles = new LinkedList<>();
    private final Map<String, SymbolTablePass.Symbol> symbolTable = new HashMap<>();
    private final Set<String> alreadyProcessed = new HashSet<>();

    public Compiler(ErrorReporter errorReporter, SyntaxErrorListener syntaxErrorListener) {
        this.errorReporter = errorReporter;
        this.syntaxErrorListener = syntaxErrorListener;
    }

    public void run(String rootFilepath) throws IOException {
        var rootFile = new File(rootFilepath);
        var importResolutionRoot = rootFile.getParent();
        remainingFiles.push(rootFilepath);

        while (remainingFiles.size() > 0) {
            var filepath = remainingFiles.pop();

            var path = Paths.get(filepath);
            if (path.isAbsolute()) {
                runForFile(importResolutionRoot, path.toString());
            } else {
                var rootPath = Paths.get(importResolutionRoot);
                var filePath = rootPath.resolve(filepath).normalize();
                runForFile(importResolutionRoot, filePath.toString());
            }
        }

        if (errorReporter.hasErrors()) {
            var message = errorReporter.reportErrors();
            System.err.println(message);
        } else {
            System.out.printf("%s compiled successfully", Paths.get(rootFilepath).getFileName());
        }
    }

    private void runForFile(String importResolutionRoot, String filepath) throws IOException {
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
        var resolveImportsPass = new ResolveImportsPass(importResolutionRoot, remainingFiles, alreadyProcessed);
        var buildSymbolTablePass = new SymbolTablePass(errorReporter, symbolTable);

        var remainingFilesBefore = remainingFiles.size();
        resolveImportsPass.run(tree);
        if (remainingFilesBefore != remainingFiles.size()) {
            remainingFiles.addLast(filepath);
            return;
        }
        buildSymbolTablePass.run(tree);
        alreadyProcessed.add(filepath);
    }
}
