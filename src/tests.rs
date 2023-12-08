use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use temp_dir::TempDir;

pub struct VirtualHwmonBuilder {
    root: PathBuf,
    index: u16,
}

impl VirtualHwmonBuilder {
    pub fn create(
        root: impl AsRef<Path>,
        index: u16,
        name: impl AsRef<[u8]>,
    ) -> VirtualHwmonBuilder {
        let path = root.as_ref().join(format!("hwmon{}", index));

        fs::create_dir_all(&path).unwrap();

        File::create(path.join("name"))
            .unwrap()
            .write(name.as_ref())
            .unwrap();

        File::create(path.join("update_interval"))
            .unwrap()
            .write("1000".as_bytes())
            .unwrap();

        VirtualHwmonBuilder {
            root: root.as_ref().to_path_buf(),
            index,
        }
    }

    pub fn add_temp(self, index: u16, value: i32, label: impl AsRef<str>) -> VirtualHwmonBuilder {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(format!("temp{}_input", index)))
            .unwrap()
            .write(value.to_string().as_bytes())
            .unwrap();

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(format!("temp{}_enable", index)))
            .unwrap()
            .write(b"1\n")
            .unwrap();

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(format!("temp{}_label", index)))
            .unwrap()
            .write(label.as_ref().as_bytes())
            .unwrap();

        self
    }

    pub fn add_fan(self, index: u16, value: u32) -> VirtualHwmonBuilder {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(format!("fan{}_input", index)))
            .unwrap()
            .write(value.to_string().as_bytes())
            .unwrap();

        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(format!("fan{}_enable", index)))
            .unwrap()
            .write(b"1\n")
            .unwrap();

        self
    }

    pub fn add_pwm(
        self,
        index: u16,
        create_enable_file: bool,
        create_mode_file: bool,
    ) -> VirtualHwmonBuilder {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.path().join(&format!("pwm{}", index)))
            .unwrap()
            .write(b"0\n")
            .unwrap();
        if create_enable_file {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(&format!("pwm{}_enable", index)))
                .unwrap()
                .write(b"2\n")
                .unwrap();
        }
        if create_mode_file {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(self.path().join(&format!("pwm{}_mode", index)))
                .unwrap()
                .write(b"1\n")
                .unwrap();
        }

        self.add_fan(index, 1000)
    }

    pub fn path(&self) -> PathBuf {
        self.root.join(format!("hwmon{}", self.index))
    }
}

#[test]
fn test_parse() {
    let test_dir = TempDir::new().unwrap();

    VirtualHwmonBuilder::create(test_dir.path(), 0, "system")
        .add_pwm(1, true, true)
        .add_pwm(2, true, true)
        .add_temp(1, 40000, "temp1")
        .add_temp(2, 60000, "temp2");
    VirtualHwmonBuilder::create(test_dir.path(), 1, "other")
        .add_pwm(1, true, true)
        .add_temp(1, 40000, "temp1")
        .add_fan(2, 1000);

    let hwmons = crate::hwmon::sync_hwmon::Hwmons::parse_path(test_dir.path()).unwrap();
    let hwmon0 = hwmons.hwmons_by_name("system").next().unwrap();
    let hwmon1 = hwmons.hwmons_by_name("other").next().unwrap();

    assert_eq!(hwmon0.name(), hwmons.hwmon_by_index(0).unwrap().name());
    assert_eq!(hwmon1.name(), hwmons.hwmon_by_index(1).unwrap().name());

    assert_eq!(hwmons.hwmon_by_index(2).is_none(), true);
    assert_eq!(hwmons.hwmons_by_name("alias").next().is_none(), true);

    assert_eq!(hwmon0.temps().len(), 2);
    assert_eq!(hwmon1.temps().len(), 1);
    assert_eq!(hwmon0.pwms().len(), 2);
    assert_eq!(hwmon1.pwms().len(), 1);

    hwmon0.pwms().get(&1u16).unwrap();
    hwmon0.pwms().get(&2u16).unwrap();
    hwmon1.pwms().get(&1u16).unwrap();
    hwmon0.temps().get(&1u16).unwrap();
    hwmon0.temps().get(&2u16).unwrap();
    hwmon1.temps().get(&1u16).unwrap();
}
