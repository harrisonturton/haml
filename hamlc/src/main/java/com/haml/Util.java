package com.haml;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Paths;

public class Util {
    public static String qualifyName(String filepath, String symbolName) {
        return String.format("%s:%s", filepath, symbolName);
    }

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

}
