package com.haml;

import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.nio.file.Paths;
import java.util.Deque;
import java.util.Set;

public class ResolveImportsPass extends HamlParserBaseListener {
    public final String importResolutionRoot;
    public final Deque<String> remainingFiles;
    public final Set<String> alreadyProcessed;

    public ResolveImportsPass(String importResolutionRoot, Deque<String> remainingFiles, Set<String> alreadyProcessed) {
        this.importResolutionRoot = importResolutionRoot;
        this.remainingFiles = remainingFiles;
        this.alreadyProcessed = alreadyProcessed;
    }

    public void run(ParseTree tree) {
        var walker = new ParseTreeWalker();
        walker.walk(this, tree);
    }

    @Override
    public void enterSingleImportDeclaration(HamlParser.SingleImportDeclarationContext ctx) {
        var path = ctx.STRING().getText().replace("\"", "");
        var rootPath = Paths.get(importResolutionRoot);
        var filePath = rootPath.resolve(path).normalize().toString();
        if (!alreadyProcessed.contains(filePath)) {
            remainingFiles.addLast(filePath);
        }
    }

    @Override
    public void enterMultipleImportDeclaration(HamlParser.MultipleImportDeclarationContext ctx) {
        for (var node : ctx.STRING()) {
            var path = node.getText().replace("\"", "");
            var rootPath = Paths.get(importResolutionRoot);
            var filePath = rootPath.resolve(path).normalize().toString();
            if (!alreadyProcessed.contains(filePath)) {
                remainingFiles.addLast(filePath);
            }
        }
    }
}
