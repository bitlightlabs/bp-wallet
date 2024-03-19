// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
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

#[macro_use]
extern crate amplify;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_crate as serde;

mod loglevel;
mod opts;
mod args;
mod config;
mod command;

pub use args::{Args, Exec};
pub use bpwallet::*;
pub use command::{BpCommand, Command};
pub use config::Config;
pub use loglevel::LogLevel;
pub use opts::{
    DescrStdOpts, DescriptorOpts, GeneralOpts, ResolverOpt, WalletOpts, DATA_DIR, DATA_DIR_ENV,
    DEFAULT_ELECTRUM, DEFAULT_ESPLORA,
};
