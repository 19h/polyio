// Copyright (C) 2019 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

mod client;
mod env;
mod error;
mod events;
mod stock;
mod stream;

use std::borrow::Cow;

pub use client::Client;
pub use error::Error;
pub use events::Event;
pub use events::EventError;
pub use events::Stock;
pub use events::Subscription;

type Str = Cow<'static, str>;
