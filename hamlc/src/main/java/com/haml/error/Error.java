package com.haml.error;

import java.util.List;

public class Error {
    /**
     * Path of the source file the error occurred in.
     */
    public final String file;

    /**
     * The line number where the error was encountered.
     */
    public final int line;

    /**
     * The index of the character in the lineText where the error starts.
     * This is used to show the user exactly where the error occurred, so
     * it is easier to find.
     */
    public final int charPositionInLine;

    /**
     * The message explaining what went wrong.
     */
    public final String message;

    public Error(String file, int line, int charPositionInLine, String message) {
        this.file = file;
        this.line = line;
        this.charPositionInLine = charPositionInLine;
        this.message = message;
    }

    public String prettyPrint(List<String> lines) {
        if (lines == null) {
            throw new AssertionError("lines is null for file " + file);
        }
        var lineLabel = "  " + (line + 1) + " | ";

        var builder = new StringBuilder();
        builder.append("error: ");
        builder.append(message);
        builder.append(String.format(" at line %d in %s", line+1, file));
        builder.append("\n");
        builder.append(lineLabel);
        builder.append(lines.get(line));
        builder.append("\n");
        builder.append(" ".repeat(charPositionInLine + lineLabel.length()));
        builder.append("^");
        return builder.toString();
    }
}
