package com.haml.error;

public class ErrorMessages {
    public static String duplicateIdentifier(String identifier) {
        return String.format("duplicate identifier \"%s\"", identifier);
    }

    public static String unknownIdentifier(String identifier) {
        return String.format("unknown identifier \"%s\"", identifier);
    }

    public static String unknownSymbol(String symbol) {
        return String.format("unknown symbol \"%s\"", symbol);
    }

    public static String fileImportedItself(String filepath) {
        return String.format("%s cannot import itself", filepath);
    }

    public static String unexpectedFailure(String message, Object... args) {
        return String.format(message, args);
    }
}
