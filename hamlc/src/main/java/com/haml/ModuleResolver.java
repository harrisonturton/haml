package com.haml;

import java.nio.file.Path;
import java.nio.file.Paths;

public class ModuleResolver {
    public final Path importResolutionRoot;

    public ModuleResolver(String importResolutionRoot) {
        this.importResolutionRoot = Paths.get(importResolutionRoot);
    }

    public String resolveImport(String test) {
        return importResolutionRoot.resolve(test).toString();
    }
}