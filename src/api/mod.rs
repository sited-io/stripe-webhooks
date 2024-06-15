pub mod sited_io {
    pub mod media {
        pub mod v1 {
            include!("sited_io.media.v1.rs");
        }
    }

    pub mod pagination {
        pub mod v1 {
            include!("sited_io.pagination.v1.rs");
        }
    }
}
