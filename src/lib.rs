mod check;

use std::ops::Index;
use quote::quote;
use syn::{ItemFn, parse_macro_input, AttributeArgs, NestedMeta};
use syn::Meta::Path;

#[proc_macro_attribute]
pub fn log_handler(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {

    let mut item = item;

    let args:AttributeArgs = parse_macro_input!(attr as AttributeArgs);

    if args.iter().all(|arg|{
        let string1 = if let NestedMeta::Meta(Path(s)) = arg {
            s.segments[0].ident.to_string()
        } else  {
            return true;
        };

        if string1.eq("allow_not_main") {
            return false;
        }
        true
    }) {
        //check is whether main that function name .
        item = check::function_name_check::check(item);
    }

    let input_fn:ItemFn = parse_macro_input!(item as ItemFn);

    let block = input_fn.block;
    let sig = input_fn.sig;
    let generate = quote! {
        #sig {
            //预执行代码 块
         {

             use log_lib::log::LevelFilter;
             use log_lib::log4rs::append::console::{ConsoleAppender, Target};
             use log_lib::log4rs::append::file::FileAppender;
             use log_lib::log4rs::append::rolling_file::policy::compound::CompoundPolicy;
             use log_lib::log4rs::append::rolling_file::RollingFileAppender;
             use log_lib::log4rs::config::{Appender, Logger, Root};
             use log_lib::log4rs::encode::pattern::PatternEncoder;
             use log_lib::log4rs::filter::threshold::ThresholdFilter;
             use log_lib::log4rs::Config;
             use log_lib::log::error;
             use std::panic;
             let now_time = log_lib::chrono::Local::now();
             let log_file_name =  now_time.format("%Y-%m-%d").to_string();
             let file_path = format!("log/{}.log",log_file_name);
             let stdout = ConsoleAppender::builder().target(Target::Stdout).build();
             let window_size = 3; // log0, log1, log2
             let fixed_window_roller =log_lib::log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller::builder().build("log/log{}", window_size).unwrap();
             let size_limit = 10*1024 * 1024*1024; // 10*1GB as max log file size to roll
             let size_trigger = log_lib::log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger::new(size_limit);
             let compound_policy = CompoundPolicy::new(Box::new(size_trigger),Box::new(fixed_window_roller));
             let rolling_file_appender = RollingFileAppender::builder()
             .append(true)
            .encoder(Box::new(PatternEncoder::new("{d} [{t}]:{L} -> {l} - {m} {n} ")))
            .build(file_path.clone(), Box::new(compound_policy)).unwrap();
             let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{t}]:{L} -> {l} - {m} {n} ")))
            .append(true)
            .build(file_path)
            .unwrap();
             let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("rolling_file_appender", Box::new(rolling_file_appender)))
            .build(
                Root::builder()
                    // .appender("logfile")
                    .appender("rolling_file_appender")
                    // .appender("stdout")
                    .build(LevelFilter::Trace),
            )
            .unwrap();
        panic::set_hook(Box::new(|panic_info| {
            let x = match panic_info.payload() {
                ref payload if payload.is::<&str>() => {
                    payload.downcast_ref::<&str>().unwrap().clone().to_string()
                }
                ref payload if payload.is::<String>() => {
                    payload.downcast_ref::<String>().unwrap().clone().to_string()
                }
                ref payload if payload.is::<&'static str>() => {
                    payload.downcast_ref::<&'static str>().unwrap().clone().to_string()
                }
                _ => "not exist payload value , is prue panic ! .".to_string(),
            };
            println!("panic occurred: location:{:?},reason:{}", panic_info.location(),x);
            error!("panic occurred: location:{:?},reason:{}", panic_info.location(),x);
        }));
        let _handle = log_lib::log4rs::init_config(config).unwrap();
             println!("日志上下文初始化成功!");
        }

            #block
            }
    };
    generate.into()
}
