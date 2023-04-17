## axum - another rust webframework

#### an exercise

* to run backend  `cargo watch -q -c -w src/ -x run` (assumes you have cargo watch crate installed)  
>* q - ^ quiet
>* x - ^ clear
>* q - ^ watch only `/src` folder
>* x - ^ execute run

* to run client tests `cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"`