package com.haml;

import com.haml.error.Error;
import com.haml.error.ErrorReporter;
import com.haml.error.SyntaxErrorListener;

import java.io.IOException;
import java.util.Deque;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;

public class Main {
    public static String path = "/Users/harryturton/Documents/projects/haml/hamlc/src/main/resources/test/user.haml";

    public static void main(String[] args) {
        var compiler = Compiler.createDefault(path);

        try {
            compiler.run();
        } catch (IOException e) {
            System.err.println("Uncaught exception " + e);
            e.printStackTrace();
            System.exit(1);
        }

        var errorReporter = compiler.getErrorReporter();
        if (errorReporter.hasErrors()) {
            errorReporter.reportErrors();
        } else {
            System.out.println("Compiled successfully.");
        }
    }
}