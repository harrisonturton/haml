package com.haml;

import java.util.Map;

public class Symbol {
    public interface Type {
        String getName();
    }

    public static class StringType implements Type {
        @Override
        public String getName() {
            return "string";
        }
    }

    public static class NumberType implements Type {
        @Override
        public String getName() {
            return "number";
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
    }
}
