package com.haml;

import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;
import org.antlr.v4.runtime.tree.ParseTreeWalker;

import java.io.*;
import java.util.List;
import java.util.Map;
import java.util.Set;

public class Module {
    private String absolutePath;
    private List<String> lines;
    private Map<String, String> symbols;
    private Set<String> imports;

    public Module(
            String absolutePath,
            List<String> lines,
            Map<String, String> symbols,
            Set<String> imports) {
        this.absolutePath = absolutePath;
        this.lines = lines;
        this.symbols = symbols;
        this.imports = imports;
    }

    public static Module readFromFile(String absolutePath) throws IOException {
        var file = new File(absolutePath);
        var fileReader = new FileInputStream(file);
        var inputStream = new BufferedInputStream(fileReader);

        inputStream.mark(0);
        var inputReader = new InputStreamReader(inputStream);
        var lines = new BufferedReader(inputReader).lines().toList();
        inputStream.reset();

        var charStream = CharStreams.fromStream(inputStream);
        var lexer = new HamlLexer(charStream);
        var tokenStream = new CommonTokenStream(lexer);
        var parser = new HamlParser(tokenStream);

        var parseTree = parser.program();
        var walker = new ParseTreeWalker();
    }
}
