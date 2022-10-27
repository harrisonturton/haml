package com.haml;

import java.util.*;

public class State {

    private final Deque<String> remainingFiles;
    private final FileScope scope;

    private State(
            Deque<String> remainingFiles,
            FileScope scope) {
        this.remainingFiles = remainingFiles;
        this.scope = scope;
    }

    public static State createEmpty(String currentFile) {
        var scope = FileScope.createEmpty(currentFile);
        return new State(new LinkedList<>(), scope);
    }

    public boolean hasRemainingFiles() {
        return !remainingFiles.isEmpty();
    }

    public void pushFileFirst(String filepath) {
        remainingFiles.addFirst(filepath);
    }

    public String getNextFile() {
        return remainingFiles.pollFirst();
    }

    public FileScope getScope() {
        return scope;
    }
}