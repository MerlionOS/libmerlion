//! Prelude — commonly used types, matching std::prelude::v1.

pub use alloc::string::String;
pub use alloc::vec::Vec;
pub use alloc::boxed::Box;
pub use alloc::format;
pub use alloc::vec;

pub use crate::io::{Read, Write};
pub use crate::io::{stdout, stderr};
pub use crate::fs::File;
pub use crate::net::{TcpListener, TcpStream, SocketAddr, IpAddr, Ipv4Addr};
pub use crate::thread;
pub use crate::sync::{Arc, Mutex, RwLock, Condvar, Once};
pub use crate::time::{Duration, Instant, SystemTime};
pub use crate::process;
pub use crate::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque};

pub use crate::{println, print, eprintln};
