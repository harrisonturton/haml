package com.haml;

import org.antlr.v4.runtime.ANTLRFileStream;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.CharStreams;
import org.antlr.v4.runtime.CommonTokenStream;

import java.io.IOException;
import java.io.InputStream;

public class Main {
    public static String path = "/test/string.haml";

    public static void main(String[] args) {
        var inputStream = Main.class.getResourceAsStream(path);
        if (inputStream == null) {
            System.err.println("Input stream is null");
            System.exit(1);
        }

        var parser = createParser(inputStream);
        var tree = parser.program();
        System.out.println(tree.toStringTree(parser));
    }

    private static HamlParser createParser(InputStream inputStream) {
        CharStream charStream;
        try {
            charStream = CharStreams.fromStream(inputStream);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
        var lexer = new HamlLexer(charStream);
        var tokenStream = new CommonTokenStream(lexer);
        return new HamlParser(tokenStream);
    }
}