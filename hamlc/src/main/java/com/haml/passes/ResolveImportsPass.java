package com.haml.passes;

import com.haml.Compiler;
import com.haml.CompilerState;
import com.haml.HamlParser;
import com.haml.HamlParserBaseListener;
import com.haml.ModuleReader;
import com.haml.error.ErrorReporter;
import com.haml.error.SyntaxErrorListener;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

public class ResolveImportsPass extends HamlParserBaseListener {
    public final String currentAbsoluteFilepath;
    public final CompilerState compilerState;
    public final SyntaxErrorListener syntaxErrorListener;
    public final ErrorReporter errorReporter;
    private boolean didImportFiles = false;

    public ResolveImportsPass(
            CompilerState compilerState,
            String currentAbsoluteFilepath,
            SyntaxErrorListener syntaxErrorListener,
            ErrorReporter errorReporter) {
        this.compilerState = compilerState;
        this.syntaxErrorListener = syntaxErrorListener;
        this.errorReporter = errorReporter;
        this.currentAbsoluteFilepath = currentAbsoluteFilepath;
    }

    public boolean run(ParseTree tree) {
        var walker = new ParseTreeWalker();
        walker.walk(this, tree);
        return didImportFiles;
    }

    @Override
    public void enterCompleteImport(HamlParser.CompleteImportContext ctx) {
        var importPath = ctx.STRING().getText().replace("\"", "");
        var absoluteImport = ModuleReader.resolveImport(currentAbsoluteFilepath, importPath);
        if (absoluteImport.equals(currentAbsoluteFilepath)) {
            return;
        }
        if (compilerState.hasProcessedFile(absoluteImport)) {
            return;
        }
        compilerState.queueFile(absoluteImport);
        didImportFiles = true;
    }
}
