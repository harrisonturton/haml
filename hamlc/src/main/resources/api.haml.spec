import "versions.haml.spec";

rule api {
  path: string;
  method: string;
}

rule ForeignIdentifier {
    test: version;
}