macro_rules! publish {
    ($id:ident) => {
        mod $id;
        pub use $id::*;
    };
    ($id:ident, $($ids:ident),+) => {
        publish!($id);
       $( publish!($ids);)+
    };
}

publish!(report_repair, sonar_sweep);
