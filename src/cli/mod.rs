use clap::App;

use self::input_params::parsers::parse_input_params;
pub use self::input_params::parsers::{FileType, InputParams};

mod input_params;

pub fn read_input_params() -> InputParams {
    let yaml = load_yaml!("cli.yaml");
    let args = App::from_yaml(yaml).get_matches();
    let input_params_result = parse_input_params(args);
    match input_params_result {
        Ok(input_params) => input_params,
        Err(parse_error) => {
            panic!(
                "Error while parsing the input arguments: {}",
                parse_error.description()
            );
        }
    }
}
