#include "jolt-rs/joltc/blobstore.h"

BlobstoreClient::BlobstoreClient() {}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}
