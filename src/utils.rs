use std::os::macos::fs::MetadataExt;

#[derive(Clone)]
pub struct MacAddr {
    addr: [u8; 6],
}

impl MacAddr {
    pub fn as_string(&self) -> String {
        format!(
            "{:X?}{:X?}{:X?}{:X?}{:X?}{:X?}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.addr[4], self.addr[5]
        )
    }

    pub fn new_zeroed() -> Self {
        Self { addr: [20; 6] }
    }

    pub fn increment(&mut self) {
        for i in 0..6 {
            if self.addr[i] == 255 {
                continue;
            }

            self.addr[i] += 1;
            break;
        }
    }
}
