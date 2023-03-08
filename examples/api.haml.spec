// ------------------------------
// Schema specification
// ------------------------------

package api.spec;

use "http";

@singleton
constructor api {
  name: string,
  endpoints: map<string, Endpoint>
}

constructor endpoint {
    name: string,
    path: string,
    request: struct,
    response: struct,
}

/**
 * Mark endpoints that are deprecated, and optionally describe when they will be
 * deleted, and what they are superceded by.
 */
annotation deprecated {
  expiry?: string,
  replaced_by?: Endpoint,
}

/**
 * Name of an endpoint
 */
annotation summary { }

/**
 * Short description (one sentence) of an endpoint
 */
annotation summary { }

/**
 * Long description (multiple paragraphs) of an endpoint
 */
annotation description { }

// ------------------------------
// Type-checked config file
// ------------------------------

package api.folder;

use "common/api/spec";

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