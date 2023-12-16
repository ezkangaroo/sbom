pub mod cli {
    pub mod opts;
    pub mod scan;
}

pub mod io {
    pub mod cmd;
}

pub mod analyzers {
    pub mod analyze;
    pub mod spec;
    pub mod python {
        pub mod setuptools;
        pub mod utils {
            pub mod pip_list;
        }
    }
}
