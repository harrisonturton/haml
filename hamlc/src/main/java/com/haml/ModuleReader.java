package com.haml;

import com.haml.passes.CollectSymbolsPass;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.ParseTreeListener;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class ModuleReader {
    public static InputStream readModuleAbsolute(String absoluteImportPath) throws FileNotFoundException {
        var file = new File(absoluteImportPath);
        var fileReader = new FileInputStream(file);
        return new BufferedInputStream(fileReader);
    }

    public static String resolveImport(String sourceFilepath, String importFilepath) {
        var importPath = Paths.get(importFilepath);
        if (importPath.isAbsolute()) {
            return importPath.toString();
        }
        var resolutionRoot = Paths.get(sourceFilepath);
        if (Files.isRegularFile(resolutionRoot)) {
            resolutionRoot = resolutionRoot.getParent();
        }
        return resolutionRoot.resolve(importPath).normalize().toString();
    }

    public static class Module implements AutoCloseable {
        private final InputStream inputStream;

        public Module(InputStream inputStream) {
            this.inputStream = inputStream;
        }

        public HamlParser parse() throws IOException {
            inputStream.mark(0);
            var charStream = CharStreams.fromStream(inputStream);
            var lexer = new HamlLexer(charStream);
            var tokenStream = new CommonTokenStream(lexer);
            var parser = new HamlParser(tokenStream);
            inputStream.reset();
            return parser;
        }

        public void resolveSymbols(ParseTreeListener listener) throws IOException {
            var parser = parse();
            var tree = parser.program();
            var walker = new ParseTreeWalker();
            walker.walk(listener, tree);
        }

        public List<String> getLines() throws IOException {
            inputStream.mark(0);
            var inputReader = new InputStreamReader(inputStream);
            var lines = new BufferedReader(inputReader).lines().toList();
            inputStream.reset();
            return lines;
        }

        @Override
        public void close() throws Exception {
            inputStream.close();
        }
    }
}