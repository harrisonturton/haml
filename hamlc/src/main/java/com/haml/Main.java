package com.haml;

import java.io.IOException;

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
            var message = errorReporter.reportErrors();
            System.err.println(message);
        } else {
            System.out.println("Compiled successfully.");
        }
    }
}