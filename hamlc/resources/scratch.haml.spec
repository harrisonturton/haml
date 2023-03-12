package http.request;

import "testing";
import "http";

// "constructor" creates a type "api" that can be subclassed in the config file.
@singleton
constructor api {
    map<string, Endpoint>
}

struct Endpoint {
    repeatable {
        value: map<uint64, test>;
        get?: Endpoint;
    }
}

struct GetUserRequest {
    @pack(index = 1)
    // This part needs to be different. ID is fulfilling `FieldDescriptor` but
    // the type of id isn't written anywhere.
    id: {
        type: string,
        pack_index: 1,
    };

    @pack(index = 1)
    name: string;
}

// schema FieldDescriptor is type constructor (or generic type). The field `type` is generic over
// any type, and the field `pack_index` is a literal type that is generic over
// the set of uint32 literal types.
//
// Use "schema" as the keyword for specifying generic types versus normal
// types.
schema FieldDescriptor {
    type: type,
    pack_index: uint32,
}

struct GetUserRequest: FieldDescriptor {
    type: Test,
    pack_index: 8,
}

annotation pack {
    index: uint32,
}

string annotation {

}

// Really need to embrace types-as-values in the spec files