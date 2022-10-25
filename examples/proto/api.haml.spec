/**
 * This compiles files with a syntax that resembles proto specification files.
 * Useful for defining an RPC system for people who are already accustomed to
 * the proto syntax with the ability to slowly introduce more options.
 */

rule "message" {
    fields: MessageField[]
}

field MessageField {
    types: MessageTypes
}

enum MessageTypes {
    string
    number
    boolean
    timestamp
}

rule "service" {
    methods: repeatable {
        /**
         * GET request. The request body must not be specified.
         */
        get?: ServiceMethod,

        /**
         * POST request.
         */
        post?: ServiceMethod,

        /**
         * UPDATE request.
         */
        update?: ServiceMethod,

        /**
         * DELETE request.
         */
        delete?: ServiceMethod,
    }
}

method ServiceMethod {
    input: string;
    body: {
        /**
         * Schema of the JSON body in the request.
         */
        request?: object;

        /**
         * Schema of the JSON body in the response.
         */
        response?: object;
    };
}