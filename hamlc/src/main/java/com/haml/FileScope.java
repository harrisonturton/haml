package com.haml;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class FileScope {
    public String filepath;
    private final List<FileScope> parentScopes;
    private final Map<String, Symbol> symbols;

    private FileScope(
            String filepath,
            List<FileScope> parentScopes,
            Map<String, Symbol> symbols) {
        this.filepath = filepath;
        this.parentScopes = parentScopes;
        this.symbols = symbols;
    }

    public static FileScope createEmpty(String filepath) {
        return new FileScope(filepath, new ArrayList<>(), new HashMap<>());
    }

    /**
     * Store a value identified by a symbol.
     *
     * @param symbol Symbol to define.
     */
    public void define(String name, Symbol symbol) {
        symbols.put(name, symbol);
    }

    /**
     * Resolve the value stored against a symbol. This attempts to resolve the symbol in this scope,
     * and then recursively resolves upwards through the parent scopes.
     *
     * @param name Name of the symbol.
     * @return Dummy value to indicate that the symbol is there, or null if it could not be found.
     */
    public Symbol resolve(String name) {
        if (symbols.containsKey(name)) {
            return symbols.get(name);
        }
        for (var scope : parentScopes) {
            var symbol = scope.resolve(name);
            if (symbol != null) {
                return symbol;
            }
        }
        return null;
    }

    public void addParent(FileScope scope) {
        parentScopes.add(scope);
    }

    @Override
    public String toString() {
        return String.format("Scope(%s, %s)", parentScopes, symbols);
    }
}
