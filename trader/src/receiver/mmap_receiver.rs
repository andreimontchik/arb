use {
    super::Receiver,
    anyhow::Result,
    common::{message::Message, serializer::Serializer},
    log::info,
    memmap2::Mmap,
    serde::Deserialize,
    serde_json::Value,
    std::{fs::OpenOptions, path::Path},
};

#[derive(Deserialize)]
struct MmapReceiverConfig {
    file_name: String,
}

impl MmapReceiverConfig {
    pub fn new(json: &Value) -> Self {
        serde_json::from_value(json.clone()).unwrap()
    }
}

pub struct MmapReceiver<T: Serializer> {
    serializer: T,
    mmap: Mmap,
}

impl<T: Serializer> Receiver<T> for MmapReceiver<T> {
    fn new(config: &Value) -> Self {
        let conf = MmapReceiverConfig::new(config);
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&conf.file_name)
            .expect(&format!("Failed to open the file {}!", &conf.file_name));

        let mmap = unsafe {
            Mmap::map(&file).expect(&format!(
                "Failed to create memory map for the file {}!",
                conf.file_name
            ))
        };

        Self {
            serializer: T::new(),
            mmap,
        }
    }

    fn try_new(config: &Value) -> Option<Self>
    where
        Self: Sized,
    {
        info!("Creating Receiver from ({config}).");
        let conf = MmapReceiverConfig::new(config);
        if Path::new(&conf.file_name).exists() {
            Some(Self::new(config))
        } else {
            None
        }
    }

    fn receive(&mut self) -> Result<Option<Message>> {
        if self.mmap.len() > 0 && self.mmap[0] > 0 {
            Ok(Some(self.serializer.deserialize_message(&self.mmap[..])?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        chrono::Utc,
        common::serializer::BUFFER_SIZE,
        memmap2::{Mmap, MmapOptions},
        std::{
            fs::{remove_file, OpenOptions},
            str,
            time::Instant,
        },
    };

    #[test]
    fn test_mmap() {
        let file_name = &format!("/tmp/mmap_file_{}", Utc::now().format("%Y-%m-%dT%H%M%S%f"));
        let w_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_name)
            .unwrap();
        w_file.set_len(BUFFER_SIZE as u64).unwrap();
        let mut w_mmap = unsafe { MmapOptions::new().len(BUFFER_SIZE).map_mut(&w_file) }.unwrap();

        let r_file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(file_name)
            .unwrap();
        let r_mmap = unsafe { Mmap::map(&r_file) }.unwrap();

        // Remove the memory map file from dist to make sure that only it's memory representations is used
        // to transfer data between write w_mmap and r_mmap handles.
        remove_file(file_name).unwrap();

        let start = Instant::now();
        let total = 1_000_000;
        for i in 0..total {
            let src = format!("This is the test message #{i}");
            let src_buffer = src.as_bytes();

            w_mmap[..src_buffer.len()].copy_from_slice(src_buffer);
            let res = str::from_utf8(&r_mmap[..src_buffer.len()]).unwrap();
            assert_eq!(src, res);
        }
        println!(
            "Encoded and transferred {total} messages for {}us.",
            start.elapsed().as_micros()
        );
    }
}
