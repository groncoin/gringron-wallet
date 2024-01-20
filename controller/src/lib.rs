// Copyright 2021 The GrinGron Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Library module for the main wallet functionalities provided by GrinGron.

#[macro_use]
extern crate prettytable;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
use gringron_api as api;
use gringron_core as core;
use gringron_keychain as keychain;
use gringron_util as util;
use gringron_wallet_api as apiwallet;
use gringron_wallet_config as config;
use gringron_wallet_impls as impls;
use gringron_wallet_libwallet as libwallet;

pub mod command;
pub mod controller;
pub mod display;
mod error;

pub use crate::error::Error;
