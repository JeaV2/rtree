pub const LONG_VERSION: &str = concat!(
    "\n",
    "Version: ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    "Description: ",
    env!("CARGO_PKG_DESCRIPTION"),
    "\n",
    "License: ",
    env!("CARGO_PKG_LICENSE")
);