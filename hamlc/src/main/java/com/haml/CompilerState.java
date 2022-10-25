package com.haml;

import java.util.*;

public class CompilerState {
    public static class Symbol {
        public final String sourceFilepath;
        public final String name;

        public Symbol(String sourceFilepath, String name) {
            this.sourceFilepath = sourceFilepath;
            this.name = name;
        }
    }

    private final Queue<String> remainingFiles;
    private final Set<String> finishedFiles;
    private final Set<String> finishedCollectingSymbolsFiles;
    private final Map<String, Symbol> symbolTable;

    public CompilerState(
            Queue<String> remainingFiles,
            Set<String> finishedFiles,
            Set<String> finishedCollectingSymbolsFiles,
            Map<String, Symbol> symbolTable) {
        this.remainingFiles = remainingFiles;
        this.finishedFiles = finishedFiles;
        this.finishedCollectingSymbolsFiles = finishedCollectingSymbolsFiles;
        this.symbolTable = symbolTable;
    }

    public static CompilerState createEmpty() {
        return new CompilerState(new LinkedList<>(), new HashSet<>(), new HashSet<>(), new Hashtable<>());
    }

    public String getNextFile() {
        return remainingFiles.poll();
    }

    public void queueFile(String file) {
        remainingFiles.add(file);
    }

    public boolean hasRemainingFiles() {
        return remainingFiles.size() > 0;
    }

    public void finishFile(String filepath) {
        finishedFiles.add(filepath);
    }

    public boolean hasProcessedFile(String filepath) {
        return finishedFiles.contains(filepath);
    }

    public boolean hasCollectedSymbols(String filepath) {
        return finishedCollectingSymbolsFiles.contains(filepath);
    }

    public void finishCollectingSymbols(String filepath) {
        finishedCollectingSymbolsFiles.add(filepath);
    }

    public void addSymbol(String symbol) {
        symbolTable.put(symbol, new Symbol(symbol, symbol));
    }

    public boolean hasSymbol(String symbol) {
        return symbolTable.containsKey(symbol);
    }

    @Override
    public String toString() {
        return String.format("Remaining=%s\nFinished=%s\n", remainingFiles, finishedFiles);
    }
}
