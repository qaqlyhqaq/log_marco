use toml::Value;

pub(crate) fn check( _item:& proc_macro::TokenStream)-> proc_macro::TokenStream{

    let x = env!("CARGO_MANIFEST_DIR");
    let string = format!("{}/Cargo.toml", x);
    let path = std::path::Path::new(string.as_str());

    let cargo_toml_str = std::fs::read(path.to_str().unwrap()).unwrap();
    let config:Value = toml::from_str(String::from_utf8(cargo_toml_str).unwrap().as_str()).unwrap();

    if config["dev-dependencies"].get("log").is_none()
        && config["dependencies"].get("log").is_none(){
        panic!("无log 依赖");
    }else{
        println!("找到 log crate !");
    }

    if config["dev-dependencies"].get("log4rs").is_none()
        && config["dependencies"].get("log4rs").is_none(){
        panic!("无log4rs 依赖");
    }else{
        println!("找到 log4rs crate !");
    }

    if config["dev-dependencies"].get("chrono").is_none()
        && config["dependencies"].get("chrono").is_none(){
        panic!("无chrono 依赖");
    }else{
        println!("找到 chrono crate !");
    }


    proc_macro::TokenStream::new()

}