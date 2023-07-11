use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::Arc,
};

use super::{CoveAPIConfig, Runtime};
use crate::utils::Error;

pub fn configure_nginx(config: &CoveAPIConfig) -> Result<(), Error> {
    configure_nginx_file(config, Path::new("/etc/nginx/nginx.conf"))
}

fn replace_url(base: &String, url: &str) -> String {
    base.replace("INSERT_URL_HERE", url)
}

fn replace_error_log(base: &String) -> String {
    base.replace("error_log  off;", "error_log  /var/log/nginx/error.log notice;")
}

fn replace_port_number(base: &String, port: u16) -> String {
    base.replace("INSERT_PORT_HERE", &port.to_string())
}

fn replace_runtime_configurations(base: &String, runtimes: &Vec<Arc<Runtime>>) -> String {
    let mut config_string = String::new();
    for runtime in runtimes {
        config_string.push_str(&build_runtime_config(runtime));
    }
    base.replace("INSERT_CONFIGURATIONS_HERE", &config_string)
}

fn build_runtime_config(runtime: &Runtime) -> String {
    const BASE_CONFIGURATION_STRUCTURE: &str = "
    server {
        listen INSERT_PORT_HERE;
        location /502 {
            return 502 'CoveAPI could not connect to your service, please double check that you specified the correct uri.';
        }
        location / {
            proxy_pass INSERT_URL_HERE;
        }
    }
    ";
    let config = &String::from(BASE_CONFIGURATION_STRUCTURE);
    let config = replace_port_number(&config, runtime.port);
    let config = replace_url(&config, runtime.app_base_url.as_str());
    config
}

fn open_config_file(path: &Path, for_writing: bool) -> Result<File, Error> {
    match OpenOptions::new()
        .write(for_writing)
        .read(true)
        .truncate(for_writing)
        .open(path)
    {
        Ok(file) => Ok(file),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue opening file {:?} due to: {}",
                path, why
            )))
        }
    }
}

fn configure_nginx_file(config: &CoveAPIConfig, path: &Path) -> Result<(), Error> {
    let mut file = open_config_file(path, false)?;

    let mut config_string = String::new();
    match file.read_to_string(&mut config_string) {
        Ok(_) => (),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue reading file {:?} due to: {}",
                path, why
            )))
        }
    }

    if config.debug {
        config_string = replace_error_log(&config_string);
    }
    config_string = replace_runtime_configurations(&config_string, &config.runtimes);

    let mut file = open_config_file(path, true)?;
    match file.write_all(config_string.as_bytes()) {
        Ok(_) => (),
        Err(why) => {
            return Err(Error::UnexpectedIOIssue(format!(
                "issue writing file {:?} due to: {}",
                path, why
            )))
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{Read, Write},
        path::Path,
        str::FromStr,
        sync::Arc,
    };

    use url::Url;

    use crate::{
        config::{
            nginx::{
                configure_nginx_file, replace_error_log, replace_port_number, replace_runtime_configurations,
                replace_url,
            },
            OpenapiSource, Runtime,
        },
        utils::test::create_mock_config,
    };

    use super::open_config_file;

    #[test]
    fn changes_marker_from_string() {
        let test_string = String::from("proxy_pass INSERT_URL_HERE");
        assert_eq!(
            replace_url(&test_string, "https://example.com"),
            "proxy_pass https://example.com"
        );
    }

    #[test]
    fn replaces_file_correctly() {
        write_default_config();

        let nginx_path = Path::new("./dump/nginx.conf");
        let config = create_mock_config();
        configure_nginx_file(&config, &nginx_path).unwrap();
        let mut conf_string = String::from("");
        File::open(&nginx_path)
            .unwrap()
            .read_to_string(&mut conf_string)
            .unwrap();

        assert!(conf_string.contains("http://example.com"));
        assert!(conf_string.contains("13750"));

        write_default_config();
    }

    #[test]
    fn generates_multiple_configurations() {
        let mut config = create_mock_config();
        config.runtimes.push(Arc::from(Runtime {
            openapi_source: OpenapiSource::Url(Url::from_str("http://example.com").unwrap()),
            app_base_url: Url::from_str("http://example.com").unwrap(),
            port: 123,
        }));
        config.runtimes.push(Arc::from(Runtime {
            openapi_source: OpenapiSource::Url(Url::from_str("http://example.com").unwrap()),
            app_base_url: Url::from_str("http://example.com").unwrap(),
            port: 456,
        }));
        let config_string = replace_runtime_configurations(&"INSERT_CONFIGURATIONS_HERE".to_string(), &config.runtimes);
        assert!(config_string.contains("123"));
        assert!(config_string.contains("456"));
    }

    fn write_default_config() {
        let mut file = open_config_file(Path::new("./dump/nginx.conf"), true).unwrap();
        file.write_all("...some other conf \nINSERT_CONFIGURATIONS_HERE\n...some more conf\n".as_bytes())
            .unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn replaces_log_when_debug_on() {
        let test_string = String::from("... stuff ... error_log  off; ... stuff ...");
        assert_eq!(
            replace_error_log(&test_string),
            "... stuff ... error_log  /var/log/nginx/error.log notice; ... stuff ..."
        );
    }

    #[test]
    fn repaces_port_number() {
        let test_string = String::from("... stuff ... INSERT_PORT_HERE ... stuff ...");
        assert_eq!(
            replace_port_number(&test_string, 13567),
            "... stuff ... 13567 ... stuff ..."
        );
    }
}
