# rust-demo-study
learn rust by example


## main导入文件夹下的模块
1. 约定在文件夹下新建 `mod.rs`
    ```shell 
    src/
    ├── main.rs
    └── my_module/
        ├── mod.rs
        └── sub_module.rs
    
    ```
2. 在`mod.rs`中声明模块
    ```rust
    pub mod sub_module;
    ```
3. 在`main.rs`中引入
    ```rust
    mod my_module;
    use my_module::sub_module::funciton;
    
    ```