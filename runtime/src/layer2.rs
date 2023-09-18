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

use std::convert::Infallible;
use std::error;
use std::fmt::Debug;
use std::path::Path;

pub trait Layer2: Debug {
    type Descr: Layer2Descriptor<LoadError = Self::LoadError, StoreError = Self::StoreError>;
    type Data: Layer2Data<LoadError = Self::LoadError, StoreError = Self::StoreError>;
    type Cache: Layer2Cache<LoadError = Self::LoadError, StoreError = Self::StoreError>;
    type LoadError: error::Error;
    type StoreError: error::Error;

    fn load(path: &Path) -> Result<Self, Self::LoadError>
    where Self: Sized;
    fn store(&self, path: &Path) -> Result<(), Self::StoreError>;
}

pub trait Layer2Descriptor: Debug {
    type LoadError: error::Error;
    type StoreError: error::Error;

    fn load(path: &Path) -> Result<Self, Self::LoadError>
    where Self: Sized;
    fn store(&self, path: &Path) -> Result<(), Self::StoreError>;
}

pub trait Layer2Data: Debug + Default {
    type LoadError: error::Error;
    type StoreError: error::Error;

    fn load(path: &Path) -> Result<Self, Self::LoadError>
    where Self: Sized;
    fn store(&self, path: &Path) -> Result<(), Self::StoreError>;
}

pub trait Layer2Cache: Debug + Default {
    type LoadError: error::Error;
    type StoreError: error::Error;

    fn load(path: &Path) -> Result<Self, Self::LoadError>
    where Self: Sized;
    fn store(&self, path: &Path) -> Result<(), Self::StoreError>;
}

impl Layer2 for () {
    type Descr = ();
    type Data = ();
    type Cache = ();
    type LoadError = Infallible;
    type StoreError = Infallible;

    fn load(_: &Path) -> Result<Self, Self::LoadError> { Ok(()) }
    fn store(&self, _: &Path) -> Result<(), Self::StoreError> { Ok(()) }
}

impl Layer2Descriptor for () {
    type LoadError = Infallible;
    type StoreError = Infallible;

    fn load(_: &Path) -> Result<Self, Self::LoadError> { Ok(()) }
    fn store(&self, _: &Path) -> Result<(), Self::StoreError> { Ok(()) }
}

impl Layer2Data for () {
    type LoadError = Infallible;
    type StoreError = Infallible;

    fn load(_: &Path) -> Result<Self, Self::LoadError> { Ok(()) }
    fn store(&self, _: &Path) -> Result<(), Self::StoreError> { Ok(()) }
}

impl Layer2Cache for () {
    type LoadError = Infallible;
    type StoreError = Infallible;

    fn load(_: &Path) -> Result<Self, Self::LoadError> { Ok(()) }
    fn store(&self, _: &Path) -> Result<(), Self::StoreError> { Ok(()) }
}
