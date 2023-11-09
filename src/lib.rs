#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("jolt-rs/joltc/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_blobstore() {
        let client = ffi::new_blobstore_client();
    }
}
