package com.haml;

import java.util.Map;

public class Symbol {
    public final Type type;
    public final String name;

    public Symbol(Type type, String name) {
        this.type = type;
        this.name = name;
    }

    public interface Type {
        String getName();
    }

    public static class StringType implements Type {
        @Override
        public String getName() {
            return "string";
        }
    }

    public static class StringLiteralType implements Type {
        public final String literal;

        public StringLiteralType(String literal) {
            this.literal = literal;
        }

        @Override
        public String getName() {
            return "\"" + literal + "\"";
        }

        @Override
        public String toString() {
            return getName();
        }
    }

    public static class NumberType implements Type {
        @Override
        public String getName() {
            return "number";
        }

        @Override
        public String toString() {
            return getName();
        }
    }

    public static class StructType implements Type {
        private final Map<String, Type> fields;

        public StructType(Map<String, Type> fields) {
            this.fields = fields;
        }

        @Override
        public String getName() {
            StringBuilder name = new StringBuilder("struct{");
            for (var field : fields.entrySet()) {
                name.append(field.getKey());
                name.append(":");
                name.append(field.getValue().getName());
                name.append(", ");
            }
            name.deleteCharAt(name.length() - 1);
            name.deleteCharAt(name.length() - 1);
            name.append("}");
            return name.toString();
        }

        @Override
        public String toString() {
            return getName();
        }
    }

    public static class IdentifierType implements Type {
        private final String identifier;
        public IdentifierType(String identifier) {
            this.identifier = identifier;
        }

        @Override
        public String getName() {
            return identifier;
        }

        @Override
        public String toString() {
            return getName();
        }
    }
}
