# hamlc

`haml` is an experimental **configuration metalanguage**. It is used to create
specification files that can be used to parse and typecheck other files. In
other words, it lets you design the language you use to abstract over your
problem domain, and evolve this language over time.

### Example API specification

This schema could be used to parse and typecheck the following file:

```
@singleton
constructor api {
  name: string,
  endpoints: map<string, EndpointSet>
}

struct EndpointSet {
  get?: endpoint,
  post?: endpoint,
  delete?: endpoint,
  patch?: endpoint,
  put?: endpoint,
}

constructor endpoint {
  name: string,
  path: string,
  request: struct,
  response: struct,
}

annotation deprecated {
  expiry?: string,
  replaced_by?: Endpoint,
}

annotation name {}
annotation summary {}
annotation description {}
```

```
use spec "common/api";

api FolderApi {
  name: "Folders",
  endpoints: {
    "/folders": {
      get: FindFolders,
      post: CreateFolder,
    },
    "/folders/{folder_id}": {
      get: GetFolder,
      post: UpdateFolder,
      delete: DeleteFolder,
    },
    "/folders/{folder_id}/trash": {
      post: TrashFolder,
    },
  }
}


/**
 * @name
 * Get folder
 * 
 * @summary
 * Get a folder by it's ID.
 * 
 * @description
 * Get a folder by it's ID. This will return status 404 when the folder cannot
 * be found. If there are any missing scopes, this will be reported through a
 * response in our standard error format with the relevant error codes.
 */
@deprecated(expires = "2024-06-03")
endpoint GetFolder {
  name: "get_folder",
  path: "/folder/{folder_id}",
  request: GetFolderRequest,
  response: GetFolderResponse,
}

struct GetFolderRequest {
  folder_id: string,
}

struct GetFolderResponse {
  folder: Folder,
}

struct Folder {
  id: string,
  name: string,
  owner_id: string,
  parent_folder_id?: string,
  created: u64,
  updated: u64,
}
```
