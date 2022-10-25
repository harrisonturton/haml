package com.haml;

import java.util.Map;

public class SymbolTable {

    public static abstract class Symbol {
        public final String name;
        public final Type type;

        public Symbol(String name, Type type) {
            this.name = name;
            this.type = type;
        }
    }

    public interface Type {
        String getName();
    }

    public static class Rule implements Type {
        public final String name;

        public Rule(String name) {
            this.name = name;
        }

        @Override
        public String getName() {
            return name;
        }
    }

    public interface Scope {
        String getScopeName();
        Scope getEnclosingScope();
        void define(Symbol symbol);
        Symbol resolve(String name);
    }

    public static class LocalScope implements Scope {
        public final String name;
        public final Scope enclosingScope;
        public final Map<String, Symbol> symbols;

        public LocalScope(String name, Scope enclosingScope, Map<String, Symbol> symbolTable) {
            this.name = name;
            this.enclosingScope = enclosingScope;
            this.symbols = symbolTable;
        }

        @Override
        public String getScopeName() {
            return name;
        }

        @Override
        public Scope getEnclosingScope() {
            return enclosingScope;
        }

        @Override
        public void define(Symbol symbol) {
            symbols.put(symbol.name, symbol);
        }

        @Override
        public Symbol resolve(String name) {
            var symbol = symbols.get(name);
            if (symbol != null) {
                return symbol;
            }
            if (enclosingScope != null) {
                return enclosingScope.resolve(name);
            }
            return null;
        }
    }
}
