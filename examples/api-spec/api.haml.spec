
/**
 * Documentation
 */
rule "api" {
    input: string,

    methods: repeatable {
        /**
         * GET request
         */
        get?: ApiMethod,

        /**
         * POST request
         */
        post?: ApiMethod,

        /**
         * UPDATE request
         */
        update?: ApiMethod,

        /**
         * DELETE request
         */
        delete?: ApiMethod,
    },

    test: Something,
}

/**
 * Specification of an public API endpoint.
 * 
 * Long documentation description.
 */
method ApiMethod {
    input: string,
    body: {
        /**
         * Headers
         */
        headers?: object,

        /**
         * Request
         */
        request?: object,

        /**
         * Response
         */
        response?: object,
    }
}