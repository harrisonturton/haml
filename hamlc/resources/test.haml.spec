package http.request;

import "testing";
import "http";

/**
 * @short
 * Test
 *
 * @long
 * Test
 */
 // Unchecked comment
constructor api {
    map<string, Endpoint>
}

api FolderApi {
    documentation: {
        name: "Folder API",
        short: "For interacting with Canva folders",
        long: "For interacting with Canva folders"
    },
    endpoints: {
        "/folder": {
            get: GetFolder,
        }
    }
}

endpoint GetFolder {
    documentation: {
        short: "Testing",
        long: "testing",
    },
    request: GetFolderRequest
}