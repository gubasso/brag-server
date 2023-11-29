use serde::Deserialize;

use super::git_hosts::Host;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub hosts: Vec<Host>,
}
